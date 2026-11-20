#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_030b4747(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4747); }
 let mut pc: u32 = 0x30b4747;
 loop { match pc {
 0x30b4747 => {
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
  if cpu.zf { pc = 0x30b4764; continue; }
  pc = 0x30b475e; continue;
 }
 0x30b475e => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x30b4767; continue; }
  pc = 0x30b4764; continue;
 }
 0x30b4764 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x30b4767; continue;
 }
 0x30b4767 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x30b4773; continue; }
  pc = 0x30b476d; continue;
 }
 0x30b476d => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x30b4776; continue; }
  pc = 0x30b4773; continue;
 }
 0x30b4773 => {
  { let v: u32 = cpu.eax | 0x20u32; cpu.eax = v; }
  pc = 0x30b4776; continue;
 }
 0x30b4776 => {
  { cpu.sub32(cpu.r32(0x68u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x30b477f; continue; }
  pc = 0x30b477c; continue;
 }
 0x30b477c => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  pc = 0x30b477f; continue;
 }
 0x30b477f => {
  { let v: u32 = cpu.eax; cpu.w32(0x315cfe0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x20u32); }
  if !cpu.zf { pc = 0x30b47ad; continue; }
  pc = 0x30b4793; continue;
 }
 0x30b4793 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x30b47c6; continue; }
  pc = 0x30b47a3; continue;
 }
 0x30b47a3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x30b47c6; continue;
 }
 0x30b47ad => {
  { let v: u32 = cpu.r32(0x315cfe0u32.wrapping_add(cpu.bases[3])) | 0x8u32; cpu.w32(0x315cfe0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  pc = 0x30b47c6; continue;
 }
 0x30b47c6 => {
  { let v: u32 = cpu.r32(0x315cfe0u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  let t = cpu.r32(0x30b27b8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.flat_ds(0x30b47cb); cpu.push(0x30b47d3); cpu.call(t); if cpu.returned_to != 0x30b47d3 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x30b47d9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b47da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b47da); }
 let mut pc: u32 = 0x30b47da;
 loop { match pc {
 0x30b47da => {
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
  cpu.flat_ds(0x30b4805); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4806(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4806); }
 let mut pc: u32 = 0x30b4806;
 loop { match pc {
 0x30b4806 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0x315d000u32; cpu.push(v); }
  { let v = 0x315cfe0u32; cpu.push(v); }
  cpu.flat_ds(0x30b4815); cpu.push(0x30b481a); f_030a38a0(cpu); if cpu.returned_to != 0x30b481a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x30b4806u32; cpu.push(v); }
  { let v = 0x30b27b8u32; cpu.push(v); }
  cpu.flat_ds(0x30b4827); cpu.push(0x30b482c); f_030a38a0(cpu); if cpu.returned_to != 0x30b482c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b4834); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4835(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4835); }
 let mut pc: u32 = 0x30b4835;
 loop { match pc {
 0x30b4835 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x315b305u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x315b305u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x315b2c4u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x315b305u32, v); }
  pc = 0x30b489b; continue;
 }
 0x30b4859 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x30b4863); f_030ad84c(cpu); if cpu.returned_to != 0x30b4863 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b4888; continue; }
  pc = 0x30b4869; continue;
 }
 0x30b4869 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x30b4873); f_030ad84c(cpu); if cpu.returned_to != 0x30b4873 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b4888; continue; }
  pc = 0x30b4879; continue;
 }
 0x30b4879 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x314b764u32; cpu.eax = v; }
  cpu.push(0x30b4888); f_030adb62(cpu); if cpu.returned_to != 0x30b4888 { return; }
  pc = 0x30b4888; continue;
 }
 0x30b4888 => {
  { let v: u32 = cpu.r32(0x32d1ba0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32d1ba0u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30b489b; continue;
 }
 0x30b489b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x30b4859; continue; }
  pc = 0x30b48a2; continue;
 }
 0x30b48a2 => {
  { let v: u32 = cpu.edx; cpu.w32(0x32d0fdcu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b48ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b48ac); }
 let mut pc: u32 = 0x30b48ac;
 loop { match pc {
 0x30b48ac => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x30b48b3); f_030b48bd(cpu); if cpu.returned_to != 0x30b48b3 { return; }
  pc = 0x30b5bde; continue;
 }
 0x30b5bde => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x30b5bdf; continue;
 }
 0x30b5bdf => {
  { let v: u32 = cpu.r32(0x32d0fdcu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b5bfa; continue; }
  pc = 0x30b5be9; continue;
 }
 0x30b5be9 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x30b5bf2); f_030ad92f(cpu); if cpu.returned_to != 0x30b5bf2 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0fdcu32, v); }
  cpu.poll(); pc = 0x30b5bdf; continue;
 }
 0x30b5bfa => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b48bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b48bd); }
 let mut pc: u32 = 0x30b48bd;
 loop { match pc {
 0x30b48bd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x315b2c4u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d1ba0u32); cpu.eax = v; }
  pc = 0x30b490a; continue;
 }
 0x30b48df => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x30b4902; continue; }
  pc = 0x30b48ef; continue;
 }
 0x30b48ef => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x30b4902; continue; }
  pc = 0x30b48f5; continue;
 }
 0x30b48f5 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x30b4908; continue; }
  pc = 0x30b48f9; continue;
 }
 0x30b48f9 => {
  { cpu.sub32(cpu.eax, 0x315b346u32); }
  if !cpu.cf { pc = 0x30b4902; continue; }
  pc = 0x30b4900; continue;
 }
 0x30b4900 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x30b4902; continue;
 }
 0x30b4902 => {
  cpu.push(0x30b4907); f_030b5c29(cpu); if cpu.returned_to != 0x30b4907 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x30b4908; continue;
 }
 0x30b4908 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30b490a; continue;
 }
 0x30b490a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30b48df; continue; }
  pc = 0x30b490e; continue;
 }
 0x30b490e => {
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
pub fn f_030b4915(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4915); }
 let mut pc: u32 = 0x30b4915;
 loop { match pc {
 0x30b4915 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x30b4927); f_030b20ca(cpu); if cpu.returned_to != 0x30b4927 { return; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x30b495a; continue; }
  pc = 0x30b492b; continue;
 }
 0x30b492b => {
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
  if (cpu.sf == cpu.of) { pc = 0x30b495a; continue; }
  pc = 0x30b494d; continue;
 }
 0x30b494d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30b4958); f_030b2032(cpu); if cpu.returned_to != 0x30b4958 { return; }
  pc = 0x30b4981; continue;
 }
 0x30b495a => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30b494d; continue; }
  pc = 0x30b4971; continue;
 }
 0x30b4971 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x30b497f; continue; }
  pc = 0x30b4975; continue;
 }
 0x30b4975 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x30b497f); f_0309f3db(cpu); if cpu.returned_to != 0x30b497f { return; }
  pc = 0x30b497f; continue;
 }
 0x30b497f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b4981; continue;
 }
 0x30b4981 => {
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
pub fn f_030b4989(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4989); }
 let mut pc: u32 = 0x30b4989;
 loop { match pc {
 0x30b4989 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x8000u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d048u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b49aa; continue; }
  pc = 0x30b49a5; continue;
 }
 0x30b49a5 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b49b1; continue; }
  pc = 0x30b49aa; continue;
 }
 0x30b49aa => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30b49b3; continue;
 }
 0x30b49b1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30b49b3; continue;
 }
 0x30b49b3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b49ec; continue; }
  pc = 0x30b49b7; continue;
 }
 0x30b49b7 => {
  cpu.push(0x30b49bc); f_030b5e4c(cpu); if cpu.returned_to != 0x30b49bc { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b49ec; continue; }
  pc = 0x30b49c1; continue;
 }
 0x30b49c1 => {
  { let v: u32 = 0x309e343u32; cpu.edx = v; }
  { let v: u32 = 0x309e338u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x309e338u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b49ec; continue;
 }
 0x30b49ec => {
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
pub fn f_030b49f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b49f2); }
 let mut pc: u32 = 0x30b49f2;
 loop { match pc {
 0x30b49f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b49f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b49f3); }
 let mut pc: u32 = 0x30b49f3;
 loop { match pc {
 0x30b49f3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x315d000u32); cpu.push(0x30b49fc); cpu.call(t); if cpu.returned_to != 0x30b49fc { return; }
  let t = cpu.r32(0x315d004u32); cpu.push(0x30b4a02); cpu.call(t); if cpu.returned_to != 0x30b4a02 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30b4a09); f_030b4a0b(cpu); if cpu.returned_to != 0x30b4a09 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4a22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4a22); }
 let mut pc: u32 = 0x30b4a22;
 loop { match pc {
 0x30b4a22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x30b4a71; continue; }
  pc = 0x30b4a2c; continue;
 }
 0x30b4a2c => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x30b4a71; continue; }
  pc = 0x30b4a31; continue;
 }
 0x30b4a31 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4a71; continue; }
  pc = 0x30b4a46; continue;
 }
 0x30b4a46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4a55; continue; }
  pc = 0x30b4a50; continue;
 }
 0x30b4a50 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x30b4a55; continue;
 }
 0x30b4a55 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x30b4a75; continue; }
  pc = 0x30b4a5b; continue;
 }
 0x30b4a5b => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x30b4a55; continue; }
  pc = 0x30b4a6e; continue;
 }
 0x30b4a6e => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4a71; continue;
 }
 0x30b4a71 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x30b4ac0; continue;
 }
 0x30b4a75 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x30b4aa3; continue; }
  pc = 0x30b4a85; continue;
 }
 0x30b4a85 => {
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
  pc = 0x30b4ab8; continue;
 }
 0x30b4aa3 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x30b4ab8; continue;
 }
 0x30b4ab8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x30b4ac0; continue;
 }
 0x30b4ac0 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x30b4ac4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4aca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4aca); }
 let mut pc: u32 = 0x30b4aca;
 loop { match pc {
 0x30b4ac5 => {
  pc = 0x30b4bd0; continue;
 }
 0x30b4aca => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x30b4ac5; continue; }
  pc = 0x30b4ad4; continue;
 }
 0x30b4ad4 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x30b4ac5; continue; }
  pc = 0x30b4adf; continue;
 }
 0x30b4adf => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x30b4b0c; continue; }
  pc = 0x30b4aed; continue;
 }
 0x30b4aed => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x30b4af5; continue; }
  pc = 0x30b4af2; continue;
 }
 0x30b4af2 => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4af5; continue;
 }
 0x30b4af5 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4b88; continue;
 }
 0x30b4b0c => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x30b4b23; continue; }
  pc = 0x30b4b15; continue;
 }
 0x30b4b15 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4b88; continue; }
  pc = 0x30b4b1a; continue;
 }
 0x30b4b1a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x30b4b88; continue; }
  pc = 0x30b4b21; continue;
 }
 0x30b4b21 => {
  pc = 0x30b4b32; continue;
 }
 0x30b4b23 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x30b4b88; continue; }
  pc = 0x30b4b2a; continue;
 }
 0x30b4b2a => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4b88; continue; }
  pc = 0x30b4b32; continue;
 }
 0x30b4b32 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x30b4b67; continue; }
  pc = 0x30b4b42; continue;
 }
 0x30b4b42 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4b4f; continue; }
  pc = 0x30b4b4d; continue;
 }
 0x30b4b4d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30b4b4f; continue;
 }
 0x30b4b4f => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x30b4b53; continue;
 }
 0x30b4b53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x30b4b88; continue; }
  pc = 0x30b4b5a; continue;
 }
 0x30b4b5a => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x30b4b67; continue; }
  pc = 0x30b4b5f; continue;
 }
 0x30b4b5f => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b4b53; continue; }
  pc = 0x30b4b67; continue;
 }
 0x30b4b67 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x30b4b71; continue; }
  pc = 0x30b4b6e; continue;
 }
 0x30b4b6e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x30b4b71; continue;
 }
 0x30b4b71 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x30b4b88; continue; }
  pc = 0x30b4b75; continue;
 }
 0x30b4b75 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x30b4b88; continue; }
  pc = 0x30b4b7c; continue;
 }
 0x30b4b7c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x30b4b88; continue; }
  pc = 0x30b4b83; continue;
 }
 0x30b4b83 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x30b4b71; continue;
 }
 0x30b4b88 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x30b4ba7; continue; }
  pc = 0x30b4b97; continue;
 }
 0x30b4b97 => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x30b4ba3; continue; }
  pc = 0x30b4ba0; continue;
 }
 0x30b4ba0 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4ba3; continue;
 }
 0x30b4ba3 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x30b4bb8; continue;
 }
 0x30b4ba7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4bb8; continue;
 }
 0x30b4bb8 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x30b4bc8; continue; }
  pc = 0x30b4bc0; continue;
 }
 0x30b4bc0 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x30b4bc8; continue; }
  pc = 0x30b4bc5; continue;
 }
 0x30b4bc5 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4bc8; continue;
 }
 0x30b4bc8 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x30b4bd0; continue; }
  pc = 0x30b4bcd; continue;
 }
 0x30b4bcd => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b4bd0; continue;
 }
 0x30b4bd0 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x30b4bd4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4bd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4bd5); }
 let mut pc: u32 = 0x30b4bd5;
 loop { match pc {
 0x30b4bd5 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x315cd6cu32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x30b4beb; continue; }
  pc = 0x30b4be2; continue;
 }
 0x30b4be2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315cd6cu32, v); }
  pc = 0x30b4beb; continue;
 }
 0x30b4beb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32d1ba4u32)); }
  if !cpu.zf { pc = 0x30b4bfb; continue; }
  pc = 0x30b4bf3; continue;
 }
 0x30b4bf3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32d1ba4u32, v); }
  pc = 0x30b4bfb; continue;
 }
 0x30b4bfb => {
  { let v: u32 = cpu.r32(0x315cd68u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x30b4c0e; continue; }
  pc = 0x30b4c05; continue;
 }
 0x30b4c05 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315cd68u32, v); }
  pc = 0x30b4c0e; continue;
 }
 0x30b4c0e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b4c1b; continue; }
  pc = 0x30b4c18; continue;
 }
 0x30b4c18 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x30b4c1b; continue;
 }
 0x30b4c1b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b4c22; continue; }
  pc = 0x30b4c1f; continue;
 }
 0x30b4c1f => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b4c22; continue;
 }
 0x30b4c22 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4c26(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b4c26); } r_030b4c26(cpu, 0x30b4c26); }
pub fn f_030b4e45(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b4e45); } r_030b4c26(cpu, 0x30b4e45); }
pub fn f_030b4f16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b4f16); } r_030b4c26(cpu, 0x30b4f16); }
fn r_030b4c26(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b4c26 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x315cd68u32); cpu.edx = v; }
  pc = 0x30b4c31; continue;
 }
 0x30b4c31 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b4f10; continue; }
  pc = 0x30b4c39; continue;
 }
 0x30b4c39 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x30b4c7d; continue; }
  pc = 0x30b4c47; continue;
 }
 0x30b4c47 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x30b4c54); f_030b4bd5(cpu); if cpu.returned_to != 0x30b4c54 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x30b4c6d; continue; }
  pc = 0x30b4c5d; continue;
 }
 0x30b4c5d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x30b4c31; continue;
 }
 0x30b4c6d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x30b4c31; continue;
 }
 0x30b4c7d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30b4c31; continue;
 }
 0x30b4e45 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x30b4e59); f_030b4c26(cpu); if cpu.returned_to != 0x30b4e59 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x30b4e74; continue; }
  pc = 0x30b4e64; continue;
 }
 0x30b4e64 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b4e74; continue;
 }
 0x30b4e74 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b4e91; continue; }
  pc = 0x30b4e78; continue;
 }
 0x30b4e78 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b4f0d; continue;
 }
 0x30b4e91 => {
  { cpu.and16(cpu.r16(0x32d205eu32), 0xfff0u16); }
  if cpu.zf { pc = 0x30b4ea6; continue; }
  pc = 0x30b4e9c; continue;
 }
 0x30b4e9c => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x32d205cu32, v); }
  pc = 0x30b4ea6; continue;
 }
 0x30b4ea6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4f0b; continue; }
  pc = 0x30b4eaf; continue;
 }
 0x30b4eaf => {
  { let v: u32 = cpu.r32(0x32d205cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30b4f0b; continue; }
  pc = 0x30b4ec9; continue;
 }
 0x30b4ec9 => {
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
  if (cpu.sf != cpu.of) { pc = 0x30b4f0b; continue; }
  pc = 0x30b4eed; continue;
 }
 0x30b4eed => {
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
  pc = 0x30b4f0d; continue;
 }
 0x30b4f0b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30b4f0d; continue;
 }
 0x30b4f0d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x30b4f10; continue;
 }
 0x30b4f10 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b4f16 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315d068u32), 0x0u32); }
  if !cpu.zf { pc = 0x30b4f29; continue; }
  pc = 0x30b4f25; continue;
 }
 0x30b4f25 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x30b4f0d; continue;
 }
 0x30b4f29 => {
  { cpu.sub32(cpu.r32(0x315be28u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x30b4f25; continue; }
  pc = 0x30b4f32; continue;
 }
 0x30b4f32 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x30b4f39); f_030b5112(cpu); if cpu.returned_to != 0x30b4f39 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x30b4f0d; continue; }
  pc = 0x30b4f3d; continue;
 }
 0x30b4f3d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b4f5e; continue; }
  pc = 0x30b4f49; continue;
 }
 0x30b4f49 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x30b4f51); f_030b4e45(cpu); if cpu.returned_to != 0x30b4f51 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b4f79; continue; }
  pc = 0x30b4f57; continue;
 }
 0x30b4f57 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x30b4f79; continue;
 }
 0x30b4f5e => {
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
  pc = 0x30b4f79; continue;
 }
 0x30b4f79 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x30b4f25; continue; }
  pc = 0x30b4f7d; continue;
 }
 0x30b4f7d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x30b4f25; continue; }
  pc = 0x30b4f8a; continue;
 }
 0x30b4f8a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x30b4f25; continue; }
  pc = 0x30b4f92; continue;
 }
 0x30b4f92 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30b4f9b); f_030b4d81(cpu); if cpu.returned_to != 0x30b4f9b { return; }
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
  cpu.push(0x30b4fbc); f_030ad92f(cpu); if cpu.returned_to != 0x30b4fbc { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b4f0d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4c82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4c82); }
 let mut pc: u32 = 0x30b4c82;
 loop { match pc {
 0x30b4c82 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x30b4c90); f_030b4c26(cpu); if cpu.returned_to != 0x30b4c90 { return; }
  { let v: u32 = cpu.r32(0x315cd68u32); cpu.edx = v; }
  pc = 0x30b4d71; continue;
 }
 0x30b4c9b => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b4d79; continue; }
  pc = 0x30b4ca3; continue;
 }
 0x30b4ca3 => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x30b4d6e; continue; }
  pc = 0x30b4cae; continue;
 }
 0x30b4cae => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x30b4d6e; continue; }
  pc = 0x30b4cb8; continue;
 }
 0x30b4cb8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30b4cbf); f_030b4bd5(cpu); if cpu.returned_to != 0x30b4cbf { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x30b4d79; continue; }
  pc = 0x30b4ccc; continue;
 }
 0x30b4ccc => {
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
  if cpu.zf { pc = 0x30b4d10; continue; }
  pc = 0x30b4d00; continue;
 }
 0x30b4d00 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b4d10; continue;
 }
 0x30b4d10 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b4d7b; continue; }
  pc = 0x30b4d14; continue;
 }
 0x30b4d14 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30b4d2b); f_030b4d81(cpu); if cpu.returned_to != 0x30b4d2b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x30b4d67; continue; }
  pc = 0x30b4d40; continue;
 }
 0x30b4d40 => {
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
  cpu.push(0x30b4d65); f_030ad92f(cpu); if cpu.returned_to != 0x30b4d65 { return; }
  pc = 0x30b4d6a; continue;
 }
 0x30b4d67 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b4d6a; continue;
 }
 0x30b4d6a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30b4d7b; continue;
 }
 0x30b4d6e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30b4d71; continue;
 }
 0x30b4d71 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x30b4c9b; continue; }
  pc = 0x30b4d79; continue;
 }
 0x30b4d79 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x30b4d7b; continue;
 }
 0x30b4d7b => {
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
pub fn f_030b4d81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4d81); }
 let mut pc: u32 = 0x30b4d81;
 loop { match pc {
 0x30b4d81 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x315cd68u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x30b4d97; continue;
 }
 0x30b4d8e => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x30b4d9b; continue; }
  pc = 0x30b4d92; continue;
 }
 0x30b4d92 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30b4d97; continue;
 }
 0x30b4d97 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30b4d8e; continue; }
  pc = 0x30b4d9b; continue;
 }
 0x30b4d9b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30b4daa; continue; }
  pc = 0x30b4da5; continue;
 }
 0x30b4da5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30b4db0; continue;
 }
 0x30b4daa => {
  { let v: u32 = cpu.edx; cpu.w32(0x315cd68u32, v); }
  pc = 0x30b4db0; continue;
 }
 0x30b4db0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b4db7; continue; }
  pc = 0x30b4db4; continue;
 }
 0x30b4db4 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b4db7; continue;
 }
 0x30b4db7 => {
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
pub fn f_030b4df5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4df5); }
 let mut pc: u32 = 0x30b4df5;
 loop { match pc {
 0x30b4df5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x315cd68u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b4e40; continue; }
  pc = 0x30b4e01; continue;
 }
 0x30b4e01 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b4e33; continue; }
  pc = 0x30b4e17; continue;
 }
 0x30b4e17 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b4e33; continue; }
  pc = 0x30b4e1c; continue;
 }
 0x30b4e1c => {
  { cpu.sub8(cpu.r8(0x315be58u32), 0x0u8); }
  if !cpu.zf { pc = 0x30b4e33; continue; }
  pc = 0x30b4e25; continue;
 }
 0x30b4e25 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x30b4e2a, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315be28u32, v); }
  pc = 0x30b4e33; continue;
 }
 0x30b4e33 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315be28u32)); }
  if !cpu.zf { pc = 0x30b4e40; continue; }
  pc = 0x30b4e3b; continue;
 }
 0x30b4e3b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b4e40 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4fc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4fc6); }
 let mut pc: u32 = 0x30b4fc6;
 loop { match pc {
 0x30b4fc6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b4fe1; continue; }
  pc = 0x30b4fd8; continue;
 }
 0x30b4fd8 => {
  { cpu.sub8(cpu.r8(0x315be57u32), 0x0u8); }
  if cpu.zf { pc = 0x30b4fed; continue; }
  pc = 0x30b4fe1; continue;
 }
 0x30b4fe1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x30b4ffa; continue; }
  pc = 0x30b4fed; continue;
 }
 0x30b4fed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x30b4ff5); f_030b4f16(cpu); if cpu.returned_to != 0x30b4ff5 { return; }
  pc = 0x30b5109; continue;
 }
 0x30b4ffa => {
  { cpu.sub32(cpu.r32(0x315d068u32), 0x0u32); }
  if !cpu.zf { pc = 0x30b500a; continue; }
  pc = 0x30b5003; continue;
 }
 0x30b5003 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30b5109; continue;
 }
 0x30b500a => {
  { cpu.sub32(cpu.r32(0x315be28u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x30b5003; continue; }
  pc = 0x30b5013; continue;
 }
 0x30b5013 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x30b501a); f_030b5112(cpu); if cpu.returned_to != 0x30b501a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b5109; continue; }
  pc = 0x30b5022; continue;
 }
 0x30b5022 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b504a; continue; }
  pc = 0x30b502e; continue;
 }
 0x30b502e => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b504a; continue; }
  pc = 0x30b5033; continue;
 }
 0x30b5033 => {
  { cpu.sub8(cpu.r8(0x315be58u32), 0x0u8); }
  if !cpu.zf { pc = 0x30b504a; continue; }
  pc = 0x30b503c; continue;
 }
 0x30b503c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x30b5041, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315be28u32, v); }
  pc = 0x30b504a; continue;
 }
 0x30b504a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x315be28u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x30b505e; continue; }
  pc = 0x30b5059; continue;
 }
 0x30b5059 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x30b505e; continue;
 }
 0x30b505e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30b5065); f_030b63dc(cpu); if cpu.returned_to != 0x30b5065 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x30b5003; continue; }
  pc = 0x30b506a; continue;
 }
 0x30b506a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x30b5003; continue; }
  pc = 0x30b5071; continue;
 }
 0x30b5071 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x30b5003; continue; }
  pc = 0x30b5075; continue;
 }
 0x30b5075 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x30b5003; continue; }
  pc = 0x30b5081; continue;
 }
 0x30b5081 => {
  { let v: u32 = cpu.r32(0x315cd68u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x30b509e; continue;
 }
 0x30b508c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30b50a2; continue; }
  pc = 0x30b5093; continue;
 }
 0x30b5093 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b509b; continue; }
  pc = 0x30b5097; continue;
 }
 0x30b5097 => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x30b50a2; continue; }
  pc = 0x30b509b; continue;
 }
 0x30b509b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30b509e; continue;
 }
 0x30b509e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x30b508c; continue; }
  pc = 0x30b50a2; continue;
 }
 0x30b50a2 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b50cc; continue; }
  pc = 0x30b50a6; continue;
 }
 0x30b50a6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x30b50cc; continue; }
  pc = 0x30b50b1; continue;
 }
 0x30b50b1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x30b50eb; continue;
 }
 0x30b50cc => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x30b5003; continue; }
  pc = 0x30b50d6; continue;
 }
 0x30b50d6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x30b50e4); f_030b4d81(cpu); if cpu.returned_to != 0x30b50e4 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x30b50eb; continue;
 }
 0x30b50eb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x30b5104); f_030ad92f(cpu); if cpu.returned_to != 0x30b5104 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30b5109; continue;
 }
 0x30b5109 => {
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
pub fn f_030b5112(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5112); }
 let mut pc: u32 = 0x30b5112;
 loop { match pc {
 0x30b5112 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x30b5129; continue; }
  pc = 0x30b5123; continue;
 }
 0x30b5123 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5129 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b513e; continue; }
  pc = 0x30b5135; continue;
 }
 0x30b5135 => {
  { cpu.sub8(cpu.r8(0x315be57u32), 0x0u8); }
  if cpu.zf { pc = 0x30b514a; continue; }
  pc = 0x30b513e; continue;
 }
 0x30b513e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x30b514f; continue; }
  pc = 0x30b514a; continue;
 }
 0x30b514a => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x30b5156; continue;
 }
 0x30b514f => {
  cpu.push(0x30b5154); f_030b4df5(cpu); if cpu.returned_to != 0x30b5154 { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x30b5156; continue;
 }
 0x30b5156 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x30b5123; continue; }
  pc = 0x30b515f; continue;
 }
 0x30b515f => {
  { let v: u32 = cpu.r32(0x315d048u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x30b516e; continue; }
  pc = 0x30b5169; continue;
 }
 0x30b5169 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30b516e; continue;
 }
 0x30b516e => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x30b5123; continue; }
  pc = 0x30b517a; continue;
 }
 0x30b517a => {
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
pub fn f_030b518f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b518f); }
 let mut pc: u32 = 0x30b518f;
 loop { match pc {
 0x30b518f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5192(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5192); }
 let mut pc: u32 = 0x30b5192;
 loop { match pc {
 0x30b5192 => {
  { cpu.sub8(cpu.r8(0x315d014u32), 0x0u8); }
  if cpu.zf { pc = 0x30b51b5; continue; }
  pc = 0x30b519b; continue;
 }
 0x30b519b => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x30b51a3, "Int3");
  pc = 0x30b51ac; continue;
 }
 0x30b51ac => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b51b5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b51b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b51b8); }
 let mut pc: u32 = 0x30b51b8;
 loop { match pc {
 0x30b51b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x30b51b9; continue;
 }
 0x30b51b9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x30b51c5; continue; }
  pc = 0x30b51c1; continue;
 }
 0x30b51c1 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x30b51b9; continue;
 }
 0x30b51c5 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b51c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b51c7); }
 let mut pc: u32 = 0x30b51c7;
 loop { match pc {
 0x30b51c7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(8); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x30b51de; continue; }
  pc = 0x30b51d8; continue;
 }
 0x30b51d8 => {
  { let v: u32 = cpu.r32(0x315d00cu32); cpu.edi = v; }
  pc = 0x30b51de; continue;
 }
 0x30b51de => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30b51fa; continue; }
  pc = 0x30b51e4; continue;
 }
 0x30b51e4 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x30b51f5; continue;
 }
 0x30b51e8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x30b51ef); f_03094d94(cpu); if cpu.returned_to != 0x30b51ef { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x30b51f5; continue;
 }
 0x30b51f5 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b51e8; continue; }
  pc = 0x30b51fa; continue;
 }
 0x30b51fa => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x30b520d; continue; }
  pc = 0x30b5201; continue;
 }
 0x30b5201 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x30b5208); f_03094d94(cpu); if cpu.returned_to != 0x30b5208 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x30b520d; continue;
 }
 0x30b520d => {
  { let v: u32 = cpu.r32(0x315d048u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.w32(0x315d048u32, v); }
  cpu.push(0x30b5229); f_030ad84c(cpu); if cpu.returned_to != 0x30b5229 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b525e; continue; }
  pc = 0x30b522d; continue;
 }
 0x30b522d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30b5234); f_030ad84c(cpu); if cpu.returned_to != 0x30b5234 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b525e; continue; }
  pc = 0x30b5238; continue;
 }
 0x30b5238 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x30b5242); f_0309f3db(cpu); if cpu.returned_to != 0x30b5242 { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x30b524c); f_0309f3fe(cpu); if cpu.returned_to != 0x30b524c { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d048u32, v); }
  pc = 0x30b5254; continue;
 }
 0x30b5254 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30b52f9; continue;
 }
 0x30b525e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d048u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30b528f; continue; }
  pc = 0x30b527b; continue;
 }
 0x30b527b => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x30b528a; continue;
 }
 0x30b527f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.push(0x30b5286); f_030b51b8(cpu); if cpu.returned_to != 0x30b5286 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  pc = 0x30b528a; continue;
 }
 0x30b528a => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b527f; continue; }
  pc = 0x30b528f; continue;
 }
 0x30b528f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x30b52a2; continue; }
  pc = 0x30b5299; continue;
 }
 0x30b5299 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.push(0x30b52a2); f_03094d41(cpu); if cpu.returned_to != 0x30b52a2 { return; }
  pc = 0x30b52a2; continue;
 }
 0x30b52a2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x30b52c3; continue; }
  pc = 0x30b52a9; continue;
 }
 0x30b52a9 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x30b52bd; continue;
 }
 0x30b52ae => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b52b3; continue; }
  pc = 0x30b52b2; continue;
 }
 0x30b52b2 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x30b52b3; continue;
 }
 0x30b52b3 => {
  cpu.push(0x30b52b8); f_03094d94(cpu); if cpu.returned_to != 0x30b52b8 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x30b52bd; continue;
 }
 0x30b52bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30b52ae; continue; }
  pc = 0x30b52c3; continue;
 }
 0x30b52c3 => {
  { cpu.sub32(cpu.edx, 0x7eu32); }
  if (cpu.cf || cpu.zf) { pc = 0x30b52eb; continue; }
  pc = 0x30b52c8; continue;
 }
 0x30b52c8 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x30b52d2); f_0309f3db(cpu); if cpu.returned_to != 0x30b52d2 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x30b52dc); f_0309f3fe(cpu); if cpu.returned_to != 0x30b52dc { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x30b52e6); f_030ad92f(cpu); if cpu.returned_to != 0x30b52e6 { return; }
  cpu.poll(); pc = 0x30b5254; continue;
 }
 0x30b52eb => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x30b52f9; continue;
 }
 0x30b52f9 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b52ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b52ff); }
 let mut pc: u32 = 0x30b52ff;
 loop { match pc {
 0x30b52ff => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x30b530e; continue; }
  pc = 0x30b530d; continue;
 }
 0x30b530d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x30b530e; continue;
 }
 0x30b530e => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x30b5336; continue; }
  pc = 0x30b5313; continue;
 }
 0x30b5313 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x30b5336; continue; }
  pc = 0x30b531b; continue;
 }
 0x30b531b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.push(0x30b5327); f_030b51b8(cpu); if cpu.returned_to != 0x30b5327 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x30b5336; continue; }
  pc = 0x30b532e; continue;
 }
 0x30b532e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x30b531b; continue;
 }
 0x30b5336 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30b5340; continue; }
  pc = 0x30b533a; continue;
 }
 0x30b533a => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5340 => {
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
pub fn f_030b5352(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5352); }
 let mut pc: u32 = 0x30b5352;
 loop { match pc {
 0x30b5352 => {
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
  { let v: u32 = cpu.eax; cpu.w32(0x315d018u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x315d01cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d024u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x315d02au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x315d028u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x315d02eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x315d030u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x315d034u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d01eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x315d022u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x315d03cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x315d038u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x315d03eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x315d018u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x30b53cc; continue; }
  pc = 0x30b53c6; continue;
 }
 0x30b53c6 => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x30b53ce; continue;
 }
 0x30b53cc => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x30b53ce; continue;
 }
 0x30b53ce => {
  { cpu.cf = false; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x315be50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x4bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.r16(0x315d03cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x315d038u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v = cpu.r16(0x315d03eu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x0u32; cpu.w32(0x315be50u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x30b540d; continue; }
  pc = 0x30b5409; continue;
 }
 0x30b5409 => {
  { let v: u8 = 0x4du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x30b540d; continue;
 }
 0x30b540d => {
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  cpu.flat_ds(0x30b5412); cpu.push(0x30b5417); f_030b202e(cpu); if cpu.returned_to != 0x30b5417 { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x30b541f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5420); }
 let mut pc: u32 = 0x30b5420;
 loop { match pc {
 0x30b5420 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b542f; continue; }
  pc = 0x30b542a; continue;
 }
 0x30b542a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b542f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x93u32); }
  if cpu.cf { pc = 0x30b5442; continue; }
  pc = 0x30b543d; continue;
 }
 0x30b543d => {
  { let v: u32 = 0x92u32; cpu.edx = v; }
  pc = 0x30b5442; continue;
 }
 0x30b5442 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30b5449); f_030b657b(cpu); if cpu.returned_to != 0x30b5449 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b5456); f_030b6848(cpu); if cpu.returned_to != 0x30b5456 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b545f); f_030b65b9(cpu); if cpu.returned_to != 0x30b545f { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b546c); f_030b65b9(cpu); if cpu.returned_to != 0x30b546c { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5472(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5472); }
 let mut pc: u32 = 0x30b5472;
 loop { match pc {
 0x30b5472 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x30b54b3; continue; }
  pc = 0x30b5484; continue;
 }
 0x30b5484 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x30b54b3; continue; }
  pc = 0x30b548e; continue;
 }
 0x30b548e => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30b54a3; continue; }
  pc = 0x30b5492; continue;
 }
 0x30b5492 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x30b54a3; continue;
 }
 0x30b54a3 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b54af); f_030b65ea(cpu); if cpu.returned_to != 0x30b54af { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x30b54be; continue;
 }
 0x30b54b3 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30b54be; continue; }
  pc = 0x30b54b7; continue;
 }
 0x30b54b7 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30b54be; continue;
 }
 0x30b54be => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x30b54c4; continue;
 }
 0x30b54c4 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x30b54c6; continue;
 }
 0x30b54c6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b54cd); f_030b22ae(cpu); if cpu.returned_to != 0x30b54cd { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b550e; continue; }
  pc = 0x30b54d3; continue;
 }
 0x30b54d3 => {
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x30b54dd; continue; }
  pc = 0x30b54d8; continue;
 }
 0x30b54d8 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x30b54c6; continue;
 }
 0x30b54dd => {
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30b54f9; continue; }
  pc = 0x30b54e6; continue;
 }
 0x30b54e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x30b54fb; continue;
 }
 0x30b54f9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30b54fb; continue;
 }
 0x30b54fb => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x30b5508; continue; }
  pc = 0x30b5503; continue;
 }
 0x30b5503 => {
  { cpu.sub32(cpu.edx, 0x2fu32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b54c6; continue; }
  pc = 0x30b5508; continue;
 }
 0x30b5508 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x30b54c4; continue;
 }
 0x30b550e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x30b551c); f_030b5420(cpu); if cpu.returned_to != 0x30b551c { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x30b5524; continue; }
  pc = 0x30b5522; continue;
 }
 0x30b5522 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x30b5524; continue;
 }
 0x30b5524 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x30b5535); f_030b5420(cpu); if cpu.returned_to != 0x30b5535 { return; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x30b5544); f_030b5420(cpu); if cpu.returned_to != 0x30b5544 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b554d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b554d); }
 let mut pc: u32 = 0x30b554d;
 loop { match pc {
 0x30b554d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d00cu32); cpu.ecx = v; }
  cpu.push(0x30b5567); f_030adbf7(cpu); if cpu.returned_to != 0x30b5567 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b556b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b556b); }
 let mut pc: u32 = 0x30b556b;
 loop { match pc {
 0x30b556b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x315d00cu32); cpu.ecx = v; }
  cpu.push(0x30b5577); f_030b691b(cpu); if cpu.returned_to != 0x30b5577 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b557f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b557f); } r_030b557f(cpu, 0x30b557f); }
pub fn f_030b5622(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b5622); } r_030b557f(cpu, 0x30b5622); }
fn r_030b557f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b557f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30b5598; continue; }
  pc = 0x30b5592; continue;
 }
 0x30b5592 => {
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x30b559a; continue;
 }
 0x30b5598 => {
  { let v: u8 = 0x2bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b559a; continue;
 }
 0x30b559a => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x30b55bd; continue; }
  pc = 0x30b55a8; continue;
 }
 0x30b55a8 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x30b55bd; continue; }
  pc = 0x30b55b6; continue;
 }
 0x30b55b6 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x30b55bd; continue;
 }
 0x30b55bd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b55cd; continue; }
  pc = 0x30b55c1; continue;
 }
 0x30b55c1 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x30b55cd; continue; }
  pc = 0x30b55cb; continue;
 }
 0x30b55cb => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b55cd; continue;
 }
 0x30b55cd => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b5619; continue; }
  pc = 0x30b55d5; continue;
 }
 0x30b55d5 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b5616; continue; }
  pc = 0x30b55d9; continue;
 }
 0x30b55d9 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b55db; continue;
 }
 0x30b55db => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b55e6; continue; }
  pc = 0x30b55e0; continue;
 }
 0x30b55e0 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x30b55db; continue;
 }
 0x30b55e6 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x30b55e6; continue; }
  pc = 0x30b5614; continue;
 }
 0x30b5614 => {
  pc = 0x30b5619; continue;
 }
 0x30b5616 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  pc = 0x30b5619; continue;
 }
 0x30b5619 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x30b5620; continue;
 }
 0x30b5620 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5622 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x30b5637; continue; }
  pc = 0x30b5629; continue;
 }
 0x30b5629 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b5637; continue; }
  pc = 0x30b562d; continue;
 }
 0x30b562d => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5637 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b5648; continue; }
  pc = 0x30b563b; continue;
 }
 0x30b563b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x30b5648; continue; }
  pc = 0x30b5641; continue;
 }
 0x30b5641 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x30b5637; continue;
 }
 0x30b5648 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b5655; continue; }
  pc = 0x30b564c; continue;
 }
 0x30b564c => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b564c; continue; }
  pc = 0x30b5655; continue;
 }
 0x30b5655 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b5669; continue; }
  pc = 0x30b565d; continue;
 }
 0x30b565d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30b5669; continue; }
  pc = 0x30b5661; continue;
 }
 0x30b5661 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b565d; continue; }
  pc = 0x30b5669; continue;
 }
 0x30b5669 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30b5620; continue; }
  pc = 0x30b566d; continue;
 }
 0x30b566d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x30b567a; continue; }
  pc = 0x30b5673; continue;
 }
 0x30b5673 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b566d; continue; }
  pc = 0x30b567a; continue;
 }
 0x30b567a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x30b5620; continue; }
  pc = 0x30b567e; continue;
 }
 0x30b567e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x30b567a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5685(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5685); }
 let mut pc: u32 = 0x30b5685;
 loop { match pc {
 0x30b5685 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x30b56ee; continue; }
  pc = 0x30b5696; continue;
 }
 0x30b5696 => {
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x30b56a5; continue;
 }
 0x30b56a5 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x30b56a5; continue; }
  pc = 0x30b56b4; continue;
 }
 0x30b56b4 => {
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x30b56c4; continue; }
  pc = 0x30b56bd; continue;
 }
 0x30b56bd => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x30b56e2; continue;
 }
 0x30b56c4 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x30b56ce; continue; }
  pc = 0x30b56c9; continue;
 }
 0x30b56c9 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x30b56e2; continue; }
  pc = 0x30b56ce; continue;
 }
 0x30b56ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x30b56e2; continue; }
  pc = 0x30b56d8; continue;
 }
 0x30b56d8 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x30b56e2; continue;
 }
 0x30b56e2 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30b56ee); f_0309f0c0(cpu); if cpu.returned_to != 0x30b56ee { return; }
  pc = 0x30b56ee; continue;
 }
 0x30b56ee => {
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
pub fn f_030b56f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b56f6); }
 let mut pc: u32 = 0x30b56f6;
 loop { match pc {
 0x30b56f6 => {
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
  cpu.push(0x30b5716); f_030b6a6b(cpu); if cpu.returned_to != 0x30b5716 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b5736; continue; }
  pc = 0x30b571a; continue;
 }
 0x30b571a => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x30b5729; continue;
 }
 0x30b5721 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x30b5727; continue; }
  pc = 0x30b5725; continue;
 }
 0x30b5725 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b5727; continue;
 }
 0x30b5727 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x30b5729; continue;
 }
 0x30b5729 => {
  { let v: u8 = cpu.r8(0xffffffc0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x30b5721; continue; }
  pc = 0x30b5731; continue;
 }
 0x30b5731 => {
  pc = 0x30b5913; continue;
 }
 0x30b5736 => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x47u32); }
  if !cpu.zf { pc = 0x30b57ab; continue; }
  pc = 0x30b5744; continue;
 }
 0x30b5744 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.esi, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x30b575d; continue; }
  pc = 0x30b5759; continue;
 }
 0x30b5759 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b57a4; continue; }
  pc = 0x30b575d; continue;
 }
 0x30b575d => {
  { let v = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  cpu.push(0x30b5765); f_030b6b45(cpu); if cpu.returned_to != 0x30b5765 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x30b5775); f_030b6b6d(cpu); if cpu.returned_to != 0x30b5775 { return; }
  cpu.push(0x30b577a); f_0308d622(cpu); if cpu.returned_to != 0x30b577a { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if (cpu.sf != cpu.of) { pc = 0x30b5789; continue; }
  pc = 0x30b5785; continue;
 }
 0x30b5785 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b578d; continue; }
  pc = 0x30b5789; continue;
 }
 0x30b5789 => {
  { let v: u8 = 0x45u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b57ab; continue;
 }
 0x30b578d => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b579a; continue; }
  pc = 0x30b5797; continue;
 }
 0x30b5797 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b579a; continue;
 }
 0x30b579a => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30b57a4; continue; }
  pc = 0x30b57a3; continue;
 }
 0x30b57a3 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x30b57a4; continue;
 }
 0x30b57a4 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = 0x46u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b57ab; continue;
 }
 0x30b57ab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x30b57da; continue; }
  pc = 0x30b57b4; continue;
 }
 0x30b57b4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30b57c6; continue; }
  pc = 0x30b57bf; continue;
 }
 0x30b57bf => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b57da; continue; }
  pc = 0x30b57c6; continue;
 }
 0x30b57c6 => {
  { let v: u32 = 0x2au32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x30b57d5); f_0309f0c0(cpu); if cpu.returned_to != 0x30b57d5 { return; }
  pc = 0x30b5927; continue;
 }
 0x30b57da => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x30b57f7; continue; }
  pc = 0x30b57e6; continue;
 }
 0x30b57e6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b57f2; continue; }
  pc = 0x30b57ed; continue;
 }
 0x30b57ed => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30b57f7; continue;
 }
 0x30b57f2 => {
  if (cpu.sf == cpu.of) { pc = 0x30b57f7; continue; }
  pc = 0x30b57f4; continue;
 }
 0x30b57f4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x30b57f7; continue;
 }
 0x30b57f7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x30b5810; continue; }
  pc = 0x30b580c; continue;
 }
 0x30b580c => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30b5831; continue; }
  pc = 0x30b5810; continue;
 }
 0x30b5810 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if cpu.zf { pc = 0x30b5831; continue; }
  pc = 0x30b5819; continue;
 }
 0x30b5819 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b5831; continue; }
  pc = 0x30b5820; continue;
 }
 0x30b5820 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30b582e); f_0309f44b(cpu); if cpu.returned_to != 0x30b582e { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x30b5831; continue;
 }
 0x30b5831 => {
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
  cpu.push(0x30b5850); f_030b6ba9(cpu); if cpu.returned_to != 0x30b5850 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30b5869; continue; }
  pc = 0x30b585e; continue;
 }
 0x30b585e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x30b5876; continue;
 }
 0x30b5869 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x30b5876; continue; }
  pc = 0x30b586d; continue;
 }
 0x30b586d => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2bu8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x30b5876; continue;
 }
 0x30b5876 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x30b58e8; continue; }
  pc = 0x30b5882; continue;
 }
 0x30b5882 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30b5911; continue; }
  pc = 0x30b5891; continue;
 }
 0x30b5891 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x30b589e); f_030b5622(cpu); if cpu.returned_to != 0x30b589e { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x30b58ba; continue; }
  pc = 0x30b58af; continue;
 }
 0x30b58af => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x30b58ba; continue; }
  pc = 0x30b58b3; continue;
 }
 0x30b58b3 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b58ba; continue;
 }
 0x30b58ba => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x30b58cc; continue; }
  pc = 0x30b58c8; continue;
 }
 0x30b58c8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b58d2; continue; }
  pc = 0x30b58cc; continue;
 }
 0x30b58cc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b58d2; continue;
 }
 0x30b58d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x30b58e4); f_030b557f(cpu); if cpu.returned_to != 0x30b58e4 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x30b5913; continue;
 }
 0x30b58e8 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x30b5911; continue; }
  pc = 0x30b58fc; continue;
 }
 0x30b58fc => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x30b590f); f_030b5622(cpu); if cpu.returned_to != 0x30b590f { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x30b5911; continue;
 }
 0x30b5911 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x30b5913; continue;
 }
 0x30b5913 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b57c6; continue; }
  pc = 0x30b591b; continue;
 }
 0x30b591b => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x30b5927); f_030b5685(cpu); if cpu.returned_to != 0x30b5927 { return; }
  pc = 0x30b5927; continue;
 }
 0x30b5927 => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5930); }
 let mut pc: u32 = 0x30b5930;
 loop { match pc {
 0x30b5930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x30b595d; continue; }
  pc = 0x30b593b; continue;
 }
 0x30b593b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x30b5943); f_030b20a3(cpu); if cpu.returned_to != 0x30b5943 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b595d; continue; }
  pc = 0x30b5947; continue;
 }
 0x30b5947 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x30b595d; continue; }
  pc = 0x30b5955; continue;
 }
 0x30b5955 => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x30b595d; continue;
 }
 0x30b595d => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5961(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5961); }
 let mut pc: u32 = 0x30b5961;
 loop { match pc {
 0x30adf7d => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[5] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x30adf85); cpu.push(0x30adf8a); f_030adfca(cpu); if cpu.returned_to != 0x30adf8a { return; }
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
  cpu.flat_ds(0x30adfc9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5961 => {
  cpu.poll(); pc = 0x30adf7d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5966(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5966); }
 let mut pc: u32 = 0x30b5966;
 loop { match pc {
 0x30b5966 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x30b598e; continue; }
  pc = 0x30b5975; continue;
 }
 0x30b5975 => {
  { cpu.sub8(cpu.r8(0x315be58u32), 0x0u8); }
  if cpu.zf { pc = 0x30b598e; continue; }
  pc = 0x30b597e; continue;
 }
 0x30b597e => {
  { let v: u16 = cpu.r16(0x315b2a2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b5984; continue;
 }
 0x30b5984 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x32d1c58u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b598e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b599f; continue; }
  pc = 0x30b599a; continue;
 }
 0x30b599a => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b59a9; continue; }
  pc = 0x30b599f; continue;
 }
 0x30b599f => {
  { let v: u8 = cpu.r8(0x315be56u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x30b59b4; continue; }
  pc = 0x30b59a9; continue;
 }
 0x30b59a9 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x34u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x30b5984; continue;
 }
 0x30b59b4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b59c6; continue; }
  pc = 0x30b59bd; continue;
 }
 0x30b59bd => {
  { cpu.sub8(cpu.r8(0x315be57u32), 0x0u8); }
  if cpu.zf { pc = 0x30b59d2; continue; }
  pc = 0x30b59c6; continue;
 }
 0x30b59c6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x30b59d7; continue; }
  pc = 0x30b59d2; continue;
 }
 0x30b59d2 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x30b5984; continue;
 }
 0x30b59d7 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b5a4d; continue; }
  pc = 0x30b59dc; continue;
 }
 0x30b59dc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be57u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b5a4d; continue; }
  pc = 0x30b59e8; continue;
 }
 0x30b59e8 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30b5a14; continue; }
  pc = 0x30b5a05; continue;
 }
 0x30b5a05 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x314b794u32; cpu.eax = v; }
  cpu.push(0x30b5a14); f_030adb62(cpu); if cpu.returned_to != 0x30b5a14 { return; }
  pc = 0x30b5a14; continue;
 }
 0x30b5a14 => {
  { let v: u32 = 0xfffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x32d1c58u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30b5a56; continue; }
  pc = 0x30b5a3a; continue;
 }
 0x30b5a3a => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x314b7bau32; cpu.eax = v; }
  cpu.push(0x30b5a49); f_030adb62(cpu); if cpu.returned_to != 0x30b5a49 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5a4d => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x32d1c58u32, v); }
  pc = 0x30b5a56; continue;
 }
 0x30b5a56 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5a5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5a5a); }
 let mut pc: u32 = 0x30b5a5a;
 loop { match pc {
 0x30b5a5a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b5a80; continue; }
  pc = 0x30b5a67; continue;
 }
 0x30b5a67 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be57u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b5a80; continue; }
  pc = 0x30b5a73; continue;
 }
 0x30b5a73 => {
  { let v: u16 = cpu.r16(0x32d1c58u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x30b5a80; continue;
 }
 0x30b5a80 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5a88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5a88); }
 let mut pc: u32 = 0x30b5a88;
 loop { match pc {
 0x30b5a88 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x30b5a99); f_030b6d8b(cpu); if cpu.returned_to != 0x30b5a99 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x314ba9cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x30b5aab); f_030b6daa(cpu); if cpu.returned_to != 0x30b5aab { return; }
  { let v: u32 = 0x314b869u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x30b5ab7); f_030b6daa(cpu); if cpu.returned_to != 0x30b5ab7 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30b5ac0); f_030b6daa(cpu); if cpu.returned_to != 0x30b5ac0 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x30b5acc); f_030b2193(cpu); if cpu.returned_to != 0x30b5acc { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5ad0); }
 let mut pc: u32 = 0x30b5ad0;
 loop { match pc {
 0x30b5ad0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x315d044u32); cpu.push(0x30b5ada); cpu.call(t); if cpu.returned_to != 0x30b5ada { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b5af9; continue; }
  pc = 0x30b5ade; continue;
 }
 0x30b5ade => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x30b5ae8); f_030b5a88(cpu); if cpu.returned_to != 0x30b5ae8 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x30b5af4; continue; }
  pc = 0x30b5aed; continue;
 }
 0x30b5aed => {
  cpu.push(0x30b5af2); f_0309f3d6(cpu); if cpu.returned_to != 0x30b5af2 { return; }
  pc = 0x30b5af9; continue;
 }
 0x30b5af4 => {
  cpu.push(0x30b5af9); f_0309f3e7(cpu); if cpu.returned_to != 0x30b5af9 { return; }
  pc = 0x30b5af9; continue;
 }
 0x30b5af9 => {
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5aff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5aff); }
 let mut pc: u32 = 0x30b5aff;
 loop { match pc {
 0x30b5aff => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0fdcu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30b5b27; continue; }
  pc = 0x30b5b0e; continue;
 }
 0x30b5b0e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0x4003u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fdcu32, v); }
  { let v: u16 = (cpu.edi as u16) | 0x3u16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x30b5b6e; continue;
 }
 0x30b5b27 => {
  { let v: u32 = 0x315b2c4u32; cpu.ecx = v; }
  pc = 0x30b5b4e; continue;
 }
 0x30b5b2e => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x3u8); }
  if !cpu.zf { pc = 0x30b5b4b; continue; }
  pc = 0x30b5b34; continue;
 }
 0x30b5b34 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x30b5b3e); f_030ad84c(cpu); if cpu.returned_to != 0x30b5b3e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b5b93; continue; }
  pc = 0x30b5b44; continue;
 }
 0x30b5b44 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x30b5b6e; continue;
 }
 0x30b5b4b => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  pc = 0x30b5b4e; continue;
 }
 0x30b5b4e => {
  { cpu.sub32(cpu.ecx, 0x315b4ccu32); }
  if cpu.cf { cpu.poll(); pc = 0x30b5b2e; continue; }
  pc = 0x30b5b56; continue;
 }
 0x30b5b56 => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u32 = 0x4003u32; cpu.edi = v; }
  cpu.push(0x30b5b65); f_030ad84c(cpu); if cpu.returned_to != 0x30b5b65 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b5b93; continue; }
  pc = 0x30b5b6b; continue;
 }
 0x30b5b6b => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x30b5b6e; continue;
 }
 0x30b5b6e => {
  { let v: u32 = 0x1au32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x30b5b7c); f_0309f0c0(cpu); if cpu.returned_to != 0x30b5b7c { return; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x32d1ba0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x32d1ba0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30b5b9f; continue;
 }
 0x30b5b93 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x30b5b9d); f_0309f3db(cpu); if cpu.returned_to != 0x30b5b9d { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x30b5b9f; continue;
 }
 0x30b5b9f => {
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
pub fn f_030b5ba5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5ba5); }
 let mut pc: u32 = 0x30b5ba5;
 loop { match pc {
 0x30b5ba5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x32d1ba0u32; cpu.edx = v; }
  pc = 0x30b5baf; continue;
 }
 0x30b5baf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b5bda; continue; }
  pc = 0x30b5bb5; continue;
 }
 0x30b5bb5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x30b5bc0; continue; }
  pc = 0x30b5bbc; continue;
 }
 0x30b5bbc => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x30b5baf; continue;
 }
 0x30b5bc0 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x32d0fdcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fdcu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x30b5bda; continue;
 }
 0x30b5bda => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5bfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5bfc); }
 let mut pc: u32 = 0x30b5bfc;
 loop { match pc {
 0x30b5bfc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d1ba0u32); cpu.eax = v; }
  pc = 0x30b5c05; continue;
 }
 0x30b5c05 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b5c11; continue; }
  pc = 0x30b5c09; continue;
 }
 0x30b5c09 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b5c11 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x30b5c1a; continue; }
  pc = 0x30b5c16; continue;
 }
 0x30b5c16 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x30b5c05; continue;
 }
 0x30b5c1a => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30b5c26); f_030b5c29(cpu); if cpu.returned_to != 0x30b5c26 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5c29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5c29); }
 let mut pc: u32 = 0x30b5c29;
 loop { match pc {
 0x30b5c29 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x30b5c31); f_030b5caf(cpu); if cpu.returned_to != 0x30b5c31 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30b5c3a); f_030b5ba5(cpu); if cpu.returned_to != 0x30b5c3a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5c3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5c3e); }
 let mut pc: u32 = 0x30b5c3e;
 loop { match pc {
 0x30b5c3e => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b5c49; continue; }
  pc = 0x30b5c46; continue;
 }
 0x30b5c46 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x30b5c49; continue;
 }
 0x30b5c49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5c4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5c4a); }
 let mut pc: u32 = 0x30b5c4a;
 loop { match pc {
 0x30b5c4a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x30b5c57); f_030b6e25(cpu); if cpu.returned_to != 0x30b5c57 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x30b5c61; continue;
 }
 0x30b5c61 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x30b5c6b); f_030b5c3e(cpu); if cpu.returned_to != 0x30b5c6b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30b5c61; continue; }
  pc = 0x30b5c75; continue;
 }
 0x30b5c75 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x30b5c86); f_030b5c3e(cpu); if cpu.returned_to != 0x30b5c86 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x30b5c93); f_030b5c3e(cpu); if cpu.returned_to != 0x30b5c93 { return; }
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
pub fn f_030b5caf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5caf); }
 let mut pc: u32 = 0x30b5caf;
 loop { match pc {
 0x30b5caf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x30b5ccb; continue; }
  pc = 0x30b5cc1; continue;
 }
 0x30b5cc1 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30b5d36; continue;
 }
 0x30b5ccb => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x30b5cde; continue; }
  pc = 0x30b5cd5; continue;
 }
 0x30b5cd5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30b5cdc); f_030ad4ba(cpu); if cpu.returned_to != 0x30b5cdc { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x30b5cde; continue;
 }
 0x30b5cde => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30b5ce5); f_030b6e2b(cpu); if cpu.returned_to != 0x30b5ce5 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30b5cf8; continue; }
  pc = 0x30b5cea; continue;
 }
 0x30b5cea => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x30b5cf8); f_030a1dbd(cpu); if cpu.returned_to != 0x30b5cf8 { return; }
  pc = 0x30b5cf8; continue;
 }
 0x30b5cf8 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30b5d06; continue; }
  pc = 0x30b5cfc; continue;
 }
 0x30b5cfc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x30b5d04); f_030b2134(cpu); if cpu.returned_to != 0x30b5d04 { return; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x30b5d06; continue;
 }
 0x30b5d06 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x30b5d1b; continue; }
  pc = 0x30b5d0c; continue;
 }
 0x30b5d0c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x30b5d14); f_030ad92f(cpu); if cpu.returned_to != 0x30b5d14 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x30b5d1b; continue;
 }
 0x30b5d1b => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x30b5d34; continue; }
  pc = 0x30b5d21; continue;
 }
 0x30b5d21 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x30b5d2d); f_030b5c4a(cpu); if cpu.returned_to != 0x30b5d2d { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x30b5d34); f_030a1e00(cpu); if cpu.returned_to != 0x30b5d34 { return; }
  pc = 0x30b5d34; continue;
 }
 0x30b5d34 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x30b5d36; continue;
 }
 0x30b5d36 => {
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
pub fn f_030b5d3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b5d3f); } r_030b5d3f(cpu, 0x30b5d3f); }
pub fn f_030b5dd4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b5dd4); } r_030b5d3f(cpu, 0x30b5dd4); }
fn r_030b5d3f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b5d3f => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x30b5d49); cpu.push(0x30b5d4e); f_0309e338(cpu); if cpu.returned_to != 0x30b5d4e { return; }
  { let v: u32 = 0x32d2c4cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315be38u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315be50u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x32d1c5cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315be38u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x30b5da3; continue; }
  pc = 0x30b5d82; continue;
 }
 0x30b5d82 => {
  { let v: u32 = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30b49f2u32); }
  if cpu.zf { pc = 0x30b5da3; continue; }
  pc = 0x30b5d8e; continue;
 }
 0x30b5d8e => {
  { cpu.sub32(cpu.eax, 0x30b6096u32); }
  if cpu.zf { pc = 0x30b5da3; continue; }
  pc = 0x30b5d95; continue;
 }
 0x30b5d95 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x30b5d9c); cpu.push(0x30b5da1); f_030b6f61(cpu); if cpu.returned_to != 0x30b5da1 { return; }
  pc = 0x30b5db4; continue;
 }
 0x30b5da3 => {
  { let v: u16 = cpu.r16(0x315d050u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d04cu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x30b5daf; continue;
 }
 0x30b5daf => {
  cpu.flat_ds(0x30b5daf); cpu.push(0x30b5db4); f_030a0150(cpu); if cpu.returned_to != 0x30b5db4 { return; }
  pc = 0x30b5db4; continue;
 }
 0x30b5db4 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315be38u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x30b5dd3); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x30b5dd4 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x30b5dde); cpu.push(0x30b5de3); f_0309e338(cpu); if cpu.returned_to != 0x30b5de3 { return; }
  { let v: u32 = 0x32d2c4cu32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315be38u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315be50u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x32d1c5cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315be38u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x30b5e3b; continue; }
  pc = 0x30b5e17; continue;
 }
 0x30b5e17 => {
  { let v: u32 = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30b49f2u32); }
  if cpu.zf { pc = 0x30b5e3b; continue; }
  pc = 0x30b5e23; continue;
 }
 0x30b5e23 => {
  { cpu.sub32(cpu.eax, 0x30b5ee0u32); }
  if cpu.zf { pc = 0x30b5e3b; continue; }
  pc = 0x30b5e2a; continue;
 }
 0x30b5e2a => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x30b5e31); cpu.push(0x30b5e36); f_030b6f61(cpu); if cpu.returned_to != 0x30b5e36 { return; }
  cpu.poll(); pc = 0x30b5db4; continue;
 }
 0x30b5e3b => {
  { let v: u16 = cpu.r16(0x315d056u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d052u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.poll(); pc = 0x30b5daf; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5e4c); }
 let mut pc: u32 = 0x30b5e4c;
 loop { match pc {
 0x30b5e4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x315d064u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x30b5ed3; continue; }
  pc = 0x30b5e60; continue;
 }
 0x30b5e60 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x30b5e77; continue; }
  pc = 0x30b5e6c; continue;
 }
 0x30b5e6c => {
  { let v: u16 = 0x1686u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x30b5ec9; continue; }
  pc = 0x30b5e77; continue;
 }
 0x30b5e77 => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b5ec9; continue; }
  pc = 0x30b5ebd; continue;
 }
 0x30b5ebd => {
  { let v: u32 = 0x1u32; cpu.w32(0x315d064u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b5ed3; continue;
 }
 0x30b5ec9 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x315d064u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b5ed3; continue;
 }
 0x30b5ed3 => {
  { let v: u32 = cpu.r32(0x315d064u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b5edf); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b5ee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b5ee0); }
 let mut pc: u32 = 0x30b5ee0;
 loop { match pc {
 0x30b5ee0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315d04cu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x30b5efb; continue; }
  pc = 0x30b5eed; continue;
 }
 0x30b5eed => {
  { cpu.sub16(cpu.r16(0x315d050u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x30b5fce; continue; }
  pc = 0x30b5efb; continue;
 }
 0x30b5efb => {
  { let v: u32 = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x30b6091u32); }
  if !cpu.zf { pc = 0x30b5f15; continue; }
  pc = 0x30b5f09; continue;
 }
 0x30b5f09 => {
  { let v: u32 = 0x30b6096u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b5f27; continue;
 }
 0x30b5f15 => {
  { cpu.sub32(cpu.ebx, 0x30b5ee0u32); }
  if !cpu.zf { pc = 0x30b5f27; continue; }
  pc = 0x30b5f1d; continue;
 }
 0x30b5f1d => {
  { let v: u32 = 0x30b49f2u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b5f27; continue;
 }
 0x30b5f27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b5f70; continue; }
  pc = 0x30b5f33; continue;
 }
 0x30b5f33 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b5f70; continue; }
  pc = 0x30b5f38; continue;
 }
 0x30b5f38 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d050u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d04cu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d05cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d058u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x30b5fbd; continue;
 }
 0x30b5f70 => {
  cpu.flat_ds(0x30b5f70); cpu.push(0x30b5f75); f_030b5e4c(cpu); if cpu.returned_to != 0x30b5f75 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b5fa6; continue; }
  pc = 0x30b5f7a; continue;
 }
 0x30b5f7a => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d050u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d04cu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d05cu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d058u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x30b5fbd; continue;
 }
 0x30b5fa6 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x315d050u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d04cu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x30b5fb8); cpu.push(0x30b5fbd); f_0309f1be(cpu); if cpu.returned_to != 0x30b5fbd { return; }
  pc = 0x30b5fbd; continue;
 }
 0x30b5fbd => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x315d050u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x315d04cu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b5fce; continue;
 }
 0x30b5fce => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b5fd2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6091(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6091); }
 let mut pc: u32 = 0x30b6091;
 loop { match pc {
 0x30b6091 => {
  cpu.push(0x30b6096); f_030b5ee0(cpu); if cpu.returned_to != 0x30b6096 { return; }
  f_030b6096(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6096(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6096); }
 let mut pc: u32 = 0x30b6096;
 loop { match pc {
 0x30b5fd3 => {
  { cpu.sub32(cpu.ebx, 0x30b6096u32); }
  if !cpu.zf { pc = 0x30b5fe5; continue; }
  pc = 0x30b5fdb; continue;
 }
 0x30b5fdb => {
  { let v: u32 = 0x30b49f2u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b5fe5; continue;
 }
 0x30b5fe5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b602e; continue; }
  pc = 0x30b5ff1; continue;
 }
 0x30b5ff1 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b602e; continue; }
  pc = 0x30b5ff6; continue;
 }
 0x30b5ff6 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d056u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d052u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d062u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d05eu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x30b607b; continue;
 }
 0x30b602e => {
  cpu.flat_ds(0x30b602e); cpu.push(0x30b6033); f_030b5e4c(cpu); if cpu.returned_to != 0x30b6033 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b6064; continue; }
  pc = 0x30b6038; continue;
 }
 0x30b6038 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d056u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d052u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x315d062u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d05eu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x30b607b; continue;
 }
 0x30b6064 => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x315d056u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315d052u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x30b6076); cpu.push(0x30b607b); f_0309f1be(cpu); if cpu.returned_to != 0x30b607b { return; }
  pc = 0x30b607b; continue;
 }
 0x30b607b => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x315d056u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x315d052u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b608c; continue;
 }
 0x30b608c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b6090); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6096 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315d052u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x30b60ad; continue; }
  pc = 0x30b60a3; continue;
 }
 0x30b60a3 => {
  { cpu.sub16(cpu.r16(0x315d056u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x30b608c; continue; }
  pc = 0x30b60ad; continue;
 }
 0x30b60ad => {
  { let v: u32 = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x30b6091u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b5fd3; continue; }
  pc = 0x30b60bf; continue;
 }
 0x30b60bf => {
  { let v: u32 = 0x30b5ee0u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x30b5fe5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b60ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b60ce); }
 let mut pc: u32 = 0x30b60ce;
 loop { match pc {
 0x30b60ce => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315d04cu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x30b61e5; continue; }
  pc = 0x30b60de; continue;
 }
 0x30b60de => {
  { cpu.sub16(cpu.r16(0x315d050u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x30b61e5; continue; }
  pc = 0x30b60ec; continue;
 }
 0x30b60ec => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b614b; continue; }
  pc = 0x30b60f8; continue;
 }
 0x30b60f8 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b614b; continue; }
  pc = 0x30b60fd; continue;
 }
 0x30b60fd => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d050u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d04cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x30b5d3fu32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d05cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d058u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x30b61b7; continue;
 }
 0x30b614b => {
  cpu.flat_ds(0x30b614b); cpu.push(0x30b6150); f_030b5e4c(cpu); if cpu.returned_to != 0x30b6150 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b6190; continue; }
  pc = 0x30b6155; continue;
 }
 0x30b6155 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d050u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d04cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d05cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d058u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x30b5d3fu32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x30b61b7; continue;
 }
 0x30b6190 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x30b5d3fu32; cpu.ebx = v; }
  cpu.flat_ds(0x30b619a); cpu.push(0x30b619f); f_0309f18c(cpu); if cpu.returned_to != 0x30b619f { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d04cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x315d050u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x30b61b2); cpu.push(0x30b61b7); f_0309f1be(cpu); if cpu.returned_to != 0x30b61b7 { return; }
  pc = 0x30b61b7; continue;
 }
 0x30b61b7 => {
  { let v: u32 = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x30b49f2u32); }
  if !cpu.zf { pc = 0x30b61d3; continue; }
  pc = 0x30b61c5; continue;
 }
 0x30b61c5 => {
  { let v: u32 = 0x30b5ee0u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b61d2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b61d3 => {
  { cpu.sub32(cpu.ebx, 0x30b6096u32); }
  if !cpu.zf { pc = 0x30b61e5; continue; }
  pc = 0x30b61db; continue;
 }
 0x30b61db => {
  { let v: u32 = 0x30b6091u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b61e5; continue;
 }
 0x30b61e5 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b61e8); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b61e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b61e9); }
 let mut pc: u32 = 0x30b61e9;
 loop { match pc {
 0x30b61e9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315d052u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x30b6300; continue; }
  pc = 0x30b61f9; continue;
 }
 0x30b61f9 => {
  { cpu.sub16(cpu.r16(0x315d056u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x30b6300; continue; }
  pc = 0x30b6207; continue;
 }
 0x30b6207 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b6266; continue; }
  pc = 0x30b6213; continue;
 }
 0x30b6213 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b6266; continue; }
  pc = 0x30b6218; continue;
 }
 0x30b6218 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d056u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d052u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x30b5dd4u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1bu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d062u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d05eu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x30b62d2; continue;
 }
 0x30b6266 => {
  cpu.flat_ds(0x30b6266); cpu.push(0x30b626b); f_030b5e4c(cpu); if cpu.returned_to != 0x30b626b { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b62ab; continue; }
  pc = 0x30b6270; continue;
 }
 0x30b6270 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d056u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d052u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d062u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d05eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x30b5dd4u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x30b62d2; continue;
 }
 0x30b62ab => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u32 = 0x30b5dd4u32; cpu.ebx = v; }
  cpu.flat_ds(0x30b62b5); cpu.push(0x30b62ba); f_0309f18c(cpu); if cpu.returned_to != 0x30b62ba { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d052u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x315d056u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x30b62cd); cpu.push(0x30b62d2); f_0309f1be(cpu); if cpu.returned_to != 0x30b62d2 { return; }
  pc = 0x30b62d2; continue;
 }
 0x30b62d2 => {
  { let v: u32 = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x30b49f2u32); }
  if !cpu.zf { pc = 0x30b62ee; continue; }
  pc = 0x30b62e0; continue;
 }
 0x30b62e0 => {
  { let v: u32 = 0x30b6096u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b62ed); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b62ee => {
  { cpu.sub32(cpu.ebx, 0x30b5ee0u32); }
  if !cpu.zf { pc = 0x30b6300; continue; }
  pc = 0x30b62f6; continue;
 }
 0x30b62f6 => {
  { let v: u32 = 0x30b6091u32; cpu.w32(0x315d004u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b6300; continue;
 }
 0x30b6300 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x30b6303); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b63dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b63dc); }
 let mut pc: u32 = 0x30b63dc;
 loop { match pc {
 0x30b63dc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x315be3cu32)); }
  if cpu.cf { pc = 0x30b6447; continue; }
  pc = 0x30b63ec; continue;
 }
 0x30b63ec => {
  { cpu.sub8(cpu.r8(0x315be56u32), 0x0u8); }
  if !cpu.zf { pc = 0x30b6459; continue; }
  pc = 0x30b63f5; continue;
 }
 0x30b63f5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xfu32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x30b6407; continue; }
  pc = 0x30b6402; continue;
 }
 0x30b6402 => {
  { let v: u32 = 0xfffffffu32; cpu.ebp = v; }
  pc = 0x30b6407; continue;
 }
 0x30b6407 => {
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
  if (cpu.sf == cpu.of) { pc = 0x30b6434; continue; }
  pc = 0x30b6424; continue;
 }
 0x30b6424 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.edi; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30b6447; continue; }
  pc = 0x30b6434; continue;
 }
 0x30b6434 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x30b6439; continue;
 }
 0x30b6439 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30b648e; continue; }
  pc = 0x30b6447; continue;
 }
 0x30b6447 => {
  cpu.push(0x30b644c); f_030a1ac1(cpu); if cpu.returned_to != 0x30b644c { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30b649c; continue;
 }
 0x30b6459 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x30b646e; continue; }
  pc = 0x30b6469; continue;
 }
 0x30b6469 => {
  { let v: u32 = 0xfffffu32; cpu.ebx = v; }
  pc = 0x30b646e; continue;
 }
 0x30b646e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b6489; continue; }
  pc = 0x30b647a; continue;
 }
 0x30b647a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315be57u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30b6489; continue; }
  pc = 0x30b6486; continue;
 }
 0x30b6486 => {
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  pc = 0x30b6489; continue;
 }
 0x30b6489 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x30b6439; continue;
 }
 0x30b648e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315be28u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be28u32, v); }
  pc = 0x30b649c; continue;
 }
 0x30b649c => {
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
pub fn f_030b64a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b64a7); }
 let mut pc: u32 = 0x30b64a7;
 loop { match pc {
 0x30b64a7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.sub32(cpu.r32(0x315d00cu32), 0x0u32); }
  if !cpu.zf { pc = 0x30b6572; continue; }
  pc = 0x30b64bc; continue;
 }
 0x30b64bc => {
  { let v = cpu.r16(0x315be5du32); cpu.set_segment(4, v as u16); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315be59u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30b64d2; continue;
 }
 0x30b64d2 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]))); }
  if cpu.zf { pc = 0x30b64ea; continue; }
  pc = 0x30b64d7; continue;
 }
 0x30b64d7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x30b64e5; continue; }
  pc = 0x30b64e1; continue;
 }
 0x30b64e1 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x30b64d7; continue;
 }
 0x30b64e5 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x30b64d2; continue;
 }
 0x30b64ea => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x30b64f3; continue; }
  pc = 0x30b64ee; continue;
 }
 0x30b64ee => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30b64f3; continue;
 }
 0x30b64f3 => {
  cpu.push(0x30b64f8); f_030ad84c(cpu); if cpu.returned_to != 0x30b64f8 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b6572; continue; }
  pc = 0x30b6504; continue;
 }
 0x30b6504 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x30b6513); f_030ad84c(cpu); if cpu.returned_to != 0x30b6513 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b656b; continue; }
  pc = 0x30b6517; continue;
 }
 0x30b6517 => {
  { let v: u32 = cpu.eax; cpu.w32(0x315d00cu32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30b6526; continue;
 }
 0x30b6526 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u8); }
  if cpu.zf { pc = 0x30b6546; continue; }
  pc = 0x30b652c; continue;
 }
 0x30b652c => {
  { let v: u32 = cpu.r32(0x315d00cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x30b6535; continue;
 }
 0x30b6535 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x30b6535; continue; }
  pc = 0x30b6540; continue;
 }
 0x30b6540 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x30b6526; continue;
 }
 0x30b6546 => {
  { let v: u32 = cpu.r32(0x315d00cu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d010u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x30b6569); f_0309f0c0(cpu); if cpu.returned_to != 0x30b6569 { return; }
  pc = 0x30b6572; continue;
 }
 0x30b656b => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30b6572); f_030ad92f(cpu); if cpu.returned_to != 0x30b6572 { return; }
  pc = 0x30b6572; continue;
 }
 0x30b6572 => {
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
pub fn f_030b657b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b657b); }
 let mut pc: u32 = 0x30b657b;
 loop { match pc {
 0x30b657b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30b65af; continue; }
  pc = 0x30b658d; continue;
 }
 0x30b658d => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x30b6592; continue;
 }
 0x30b6592 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30b659b); f_030b2475(cpu); if cpu.returned_to != 0x30b659b { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30b65ab; continue; }
  pc = 0x30b65a4; continue;
 }
 0x30b65a4 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b65ab; continue; }
  pc = 0x30b65a8; continue;
 }
 0x30b65a8 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x30b6592; continue;
 }
 0x30b65ab => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30b65b1; continue;
 }
 0x30b65af => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b65b1; continue;
 }
 0x30b65b1 => {
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
pub fn f_030b65b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b65b9); }
 let mut pc: u32 = 0x30b65b9;
 loop { match pc {
 0x30b65b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30b65e2; continue; }
  pc = 0x30b65cb; continue;
 }
 0x30b65cb => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x30b65de; continue; }
  pc = 0x30b65d1; continue;
 }
 0x30b65d1 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  cpu.push(0x30b65d8); f_030b2475(cpu); if cpu.returned_to != 0x30b65d8 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b65cb; continue; }
  pc = 0x30b65de; continue;
 }
 0x30b65de => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x30b65e4; continue;
 }
 0x30b65e2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x30b65e4; continue;
 }
 0x30b65e4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b65ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b65ea); }
 let mut pc: u32 = 0x30b65ea;
 loop { match pc {
 0x30b65ea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30b65ff; continue; }
  pc = 0x30b65f6; continue;
 }
 0x30b65f6 => {
  cpu.push(0x30b65fb); f_030b65b9(cpu); if cpu.returned_to != 0x30b65fb { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b65ff => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6756(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6756); }
 let mut pc: u32 = 0x30b6756;
 loop { match pc {
 0x30b6756 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30b6835; continue; }
  pc = 0x30b6773; continue;
 }
 0x30b6773 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x30b67fa; continue; }
  pc = 0x30b677c; continue;
 }
 0x30b677c => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30b6780; continue;
 }
 0x30b6780 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x30b6798; continue; }
  pc = 0x30b6786; continue;
 }
 0x30b6786 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x30b6796; continue; }
  pc = 0x30b678a; continue;
 }
 0x30b678a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x30b6798; continue; }
  pc = 0x30b6791; continue;
 }
 0x30b6791 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30b6780; continue; }
  pc = 0x30b6796; continue;
 }
 0x30b6796 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x30b6798; continue;
 }
 0x30b6798 => {
  { let v: u32 = cpu.r32(0x315ce7cu32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315ce7cu32); cpu.esi = v; }
  pc = 0x30b67a9; continue;
 }
 0x30b67a9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x30b67f3; continue; }
  pc = 0x30b67ae; continue;
 }
 0x30b67ae => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x30b67d3; continue; }
  pc = 0x30b67b2; continue;
 }
 0x30b67b2 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30b67cb; continue; }
  pc = 0x30b67b6; continue;
 }
 0x30b67b6 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x30b67cd; continue;
 }
 0x30b67cb => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x30b67cd; continue;
 }
 0x30b67cd => {
  if cpu.zf { pc = 0x30b67d3; continue; }
  pc = 0x30b67cf; continue;
 }
 0x30b67cf => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x30b67f3; continue;
 }
 0x30b67d3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30b67ec; continue; }
  pc = 0x30b67d7; continue;
 }
 0x30b67d7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x30b67ee; continue;
 }
 0x30b67ec => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x30b67ee; continue;
 }
 0x30b67ee => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.poll(); pc = 0x30b67a9; continue;
 }
 0x30b67f3 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x30b67fa; continue; }
  pc = 0x30b67f7; continue;
 }
 0x30b67f7 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  pc = 0x30b67fa; continue;
 }
 0x30b67fa => {
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
  pc = 0x30b683e; continue;
 }
 0x30b6835 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x30b683e); f_030b7023(cpu); if cpu.returned_to != 0x30b683e { return; }
  pc = 0x30b683e; continue;
 }
 0x30b683e => {
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
pub fn f_030b6848(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6848); }
 let mut pc: u32 = 0x30b6848;
 loop { match pc {
 0x30b6848 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b6857); f_030b65b9(cpu); if cpu.returned_to != 0x30b6857 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30b6862); f_030b6867(cpu); if cpu.returned_to != 0x30b6862 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6867); }
 let mut pc: u32 = 0x30b6867;
 loop { match pc {
 0x30b674f => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6867 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30b6911; continue; }
  pc = 0x30b687a; continue;
 }
 0x30b687a => {
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
  { let v: u32 = cpu.r32(0x315ce7cu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30b68a5; continue; }
  pc = 0x30b6896; continue;
 }
 0x30b6896 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  pc = 0x30b68a5; continue;
 }
 0x30b68a5 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30b68ae; continue; }
  pc = 0x30b68a9; continue;
 }
 0x30b68a9 => {
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x30b68ae; continue; }
  pc = 0x30b68ad; continue;
 }
 0x30b68ad => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x30b68ae; continue;
 }
 0x30b68ae => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x30b68d6; continue; }
  pc = 0x30b68b5; continue;
 }
 0x30b68b5 => {
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
  cpu.poll(); pc = 0x30b674f; continue;
 }
 0x30b68d6 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x30b68db; continue;
 }
 0x30b68db => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x30b68f3; continue; }
  pc = 0x30b68e3; continue;
 }
 0x30b68e3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30b68db; continue; }
  pc = 0x30b68f3; continue;
 }
 0x30b68f3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x30b674f; continue; }
  pc = 0x30b68fc; continue;
 }
 0x30b68fc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.push(0x30b690c); f_0309f0c0(cpu); if cpu.returned_to != 0x30b690c { return; }
  cpu.poll(); pc = 0x30b674f; continue;
 }
 0x30b6911 => {
  cpu.push(0x30b6916); f_0308fb14(cpu); if cpu.returned_to != 0x30b6916 { return; }
  cpu.poll(); pc = 0x30b674f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b691b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b691b); }
 let mut pc: u32 = 0x30b691b;
 loop { match pc {
 0x30b691b => {
  { let v = 0xb8u32; cpu.push(v); }
  cpu.push(0x30b6925); f_0309e68a(cpu); if cpu.returned_to != 0x30b6925 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(164); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30b693f); f_030adbf7(cpu); if cpu.returned_to != 0x30b693f { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b694a; continue;
 }
 0x30b694a => {
  cpu.push(0x30b694f); f_030a1ac1(cpu); if cpu.returned_to != 0x30b694f { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x30b6962; continue; }
  pc = 0x30b6954; continue;
 }
 0x30b6954 => {
  cpu.push(0x30b6959); f_030a1ac1(cpu); if cpu.returned_to != 0x30b6959 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6962; continue;
 }
 0x30b6962 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b696e; continue;
 }
 0x30b696e => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6977; continue;
 }
 0x30b6977 => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6984; continue;
 }
 0x30b6984 => {
  { let v: u32 = 0x314b870u32; cpu.eax = v; }
  cpu.push(0x30b698e); f_030ad9b1(cpu); if cpu.returned_to != 0x30b698e { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6998; continue;
 }
 0x30b6998 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30b699f); f_03094d94(cpu); if cpu.returned_to != 0x30b699f { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b69ae; continue;
 }
 0x30b69ae => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b69b7; continue;
 }
 0x30b69b7 => {
  { let v: u32 = 0x3bu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30b69c3); f_03094dad(cpu); if cpu.returned_to != 0x30b69c3 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30b69d3; continue; }
  pc = 0x30b69c9; continue;
 }
 0x30b69c9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30b69d0); f_03094d94(cpu); if cpu.returned_to != 0x30b69d0 { return; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  pc = 0x30b69d3; continue;
 }
 0x30b69d3 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x6eu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x30b69fa; continue; }
  pc = 0x30b69df; continue;
 }
 0x30b69df => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x30b69e9); f_0309f3db(cpu); if cpu.returned_to != 0x30b69e9 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x30b69f3); f_0309f3fe(cpu); if cpu.returned_to != 0x30b69f3 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30b6a61; continue;
 }
 0x30b69fa => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x30b6a06); f_0309f167(cpu); if cpu.returned_to != 0x30b6a06 { return; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x7eu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x30b6a19; continue; }
  pc = 0x30b6a15; continue;
 }
 0x30b6a15 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30b6a19; continue;
 }
 0x30b6a19 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x30b6a26); f_0309f167(cpu); if cpu.returned_to != 0x30b6a26 { return; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x30b6a34); f_030adbf7(cpu); if cpu.returned_to != 0x30b6a34 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6a3b; continue;
 }
 0x30b6a3b => {
  cpu.push(0x30b6a40); f_030a1ac1(cpu); if cpu.returned_to != 0x30b6a40 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x30b6a4f; continue; }
  pc = 0x30b6a45; continue;
 }
 0x30b6a45 => {
  cpu.push(0x30b6a4a); f_030a1ac1(cpu); if cpu.returned_to != 0x30b6a4a { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6a4f; continue;
 }
 0x30b6a4f => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3bu32); }
  if !cpu.zf { pc = 0x30b6a5f; continue; }
  pc = 0x30b6a57; continue;
 }
 0x30b6a57 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.poll(); pc = 0x30b69ae; continue;
 }
 0x30b6a5f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b6a61; continue;
 }
 0x30b6a61 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6a6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6a6b); }
 let mut pc: u32 = 0x30b6a6b;
 loop { match pc {
 0x30b6a6b => {
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
  if !cpu.zf { pc = 0x30b6af3; continue; }
  pc = 0x30b6a98; continue;
 }
 0x30b6a98 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x30b6ab1; continue; }
  pc = 0x30b6aab; continue;
 }
 0x30b6aab => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30b6ab1; continue;
 }
 0x30b6ab1 => {
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
  if !cpu.zf { pc = 0x30b6ae4; continue; }
  pc = 0x30b6adc; continue;
 }
 0x30b6adc => {
  { let v: u32 = cpu.r32(0x314b878u32); cpu.edx = v; }
  pc = 0x30b6aea; continue;
 }
 0x30b6ae4 => {
  { let v: u32 = cpu.r32(0x314b87cu32); cpu.edx = v; }
  pc = 0x30b6aea; continue;
 }
 0x30b6aea => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30b6af5; continue;
 }
 0x30b6af3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30b6af5; continue;
 }
 0x30b6af5 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6afe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b6afe); } r_030b6afe(cpu, 0x30b6afe); }
pub fn f_030b6b45(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b6b45); } r_030b6afe(cpu, 0x30b6b45); }
fn r_030b6afe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b6afe => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x30b6b00; continue;
 }
 0x30b6b00 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b6b25; continue; }
  pc = 0x30b6b13; continue;
 }
 0x30b6b13 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30b6b21); f_030b7062(cpu); if cpu.returned_to != 0x30b6b21 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x30b6b3d; continue;
 }
 0x30b6b25 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if !cpu.zf { pc = 0x30b6b2d; continue; }
  pc = 0x30b6b29; continue;
 }
 0x30b6b29 => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  pc = 0x30b6b37; continue;
 }
 0x30b6b2d => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x30b6b35; continue; }
  pc = 0x30b6b31; continue;
 }
 0x30b6b31 => {
  { cpu.fpush(1.0); }
  pc = 0x30b6b37; continue;
 }
 0x30b6b35 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x30b6b37; continue;
 }
 0x30b6b37 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x30b6b3d; continue;
 }
 0x30b6b3d => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6b45 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x30b6b00; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6b49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6b49); }
 let mut pc: u32 = 0x30b6b49;
 loop { match pc {
 0x30b6b49 => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x30b6b52); f_030b6afe(cpu); if cpu.returned_to != 0x30b6b52 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6b6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6b6d); }
 let mut pc: u32 = 0x30b6b6d;
 loop { match pc {
 0x30b6b6d => {
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
  cpu.push(0x30b6b87); f_030b709e(cpu); if cpu.returned_to != 0x30b6b87 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x30b6b9c; continue; }
  pc = 0x30b6b90; continue;
 }
 0x30b6b90 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x314b880u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x30b6b9c; continue;
 }
 0x30b6b9c => {
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
pub fn f_030b6ba9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6ba9); }
 let mut pc: u32 = 0x30b6ba9;
 loop { match pc {
 0x30b6ba9 => {
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
  cpu.push(0x30b6bda); f_030b6a6b(cpu); if cpu.returned_to != 0x30b6bda { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b6be5; continue; }
  pc = 0x30b6bde; continue;
 }
 0x30b6bde => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b6d7a; continue;
 }
 0x30b6be5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x30b6c0f; continue; }
  pc = 0x30b6c07; continue;
 }
 0x30b6c07 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30b6d37; continue; }
  pc = 0x30b6c0f; continue;
 }
 0x30b6c0f => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x30b6c27; continue; }
  pc = 0x30b6c19; continue;
 }
 0x30b6c19 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x30b6c27; continue;
 }
 0x30b6c27 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x30b6c35); f_030b70b7(cpu); if cpu.returned_to != 0x30b6c35 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30b6c3f; continue; }
  pc = 0x30b6c3d; continue;
 }
 0x30b6c3d => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x30b6c3f; continue;
 }
 0x30b6c3f => {
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
  if (cpu.sf == cpu.of) { pc = 0x30b6c5f; continue; }
  pc = 0x30b6c5d; continue;
 }
 0x30b6c5d => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x30b6c5f; continue;
 }
 0x30b6c5f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if !cpu.zf { pc = 0x30b6c6b; continue; }
  pc = 0x30b6c69; continue;
 }
 0x30b6c69 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x30b6c6b; continue;
 }
 0x30b6c6b => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30b6d05; continue; }
  pc = 0x30b6c73; continue;
 }
 0x30b6c73 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b6c7d; continue; }
  pc = 0x30b6c78; continue;
 }
 0x30b6c78 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x30b6c7d; continue;
 }
 0x30b6c7d => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30b6c93); f_030b709e(cpu); if cpu.returned_to != 0x30b6c93 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x30b6ca7; continue; }
  pc = 0x30b6c9c; continue;
 }
 0x30b6c9c => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b6ca7; continue; }
  pc = 0x30b6ca0; continue;
 }
 0x30b6ca0 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b6ca7; continue;
 }
 0x30b6ca7 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x30b6cbb); f_0309f40a(cpu); if cpu.returned_to != 0x30b6cbb { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b6d05; continue; }
  pc = 0x30b6cc1; continue;
 }
 0x30b6cc1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.zf { pc = 0x30b6ccf; continue; }
  pc = 0x30b6cc9; continue;
 }
 0x30b6cc9 => {
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x30b6cd1; continue; }
  pc = 0x30b6ccf; continue;
 }
 0x30b6ccf => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  pc = 0x30b6cd1; continue;
 }
 0x30b6cd1 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x30b6cdd; continue; }
  pc = 0x30b6cd6; continue;
 }
 0x30b6cd6 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x30b6ce7; continue;
 }
 0x30b6cdd => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b6ce7; continue; }
  pc = 0x30b6ce2; continue;
 }
 0x30b6ce2 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x30b6ce7; continue;
 }
 0x30b6ce7 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x30b6cfb); f_0309f40a(cpu); if cpu.returned_to != 0x30b6cfb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b6d05; continue; }
  pc = 0x30b6cff; continue;
 }
 0x30b6cff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x30b6d05; continue;
 }
 0x30b6d05 => {
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
  if !cpu.zf { pc = 0x30b6d37; continue; }
  pc = 0x30b6d2b; continue;
 }
 0x30b6d2b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if !cpu.zf { pc = 0x30b6d37; continue; }
  pc = 0x30b6d35; continue;
 }
 0x30b6d35 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  pc = 0x30b6d37; continue;
 }
 0x30b6d37 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x30b6d43; continue; }
  pc = 0x30b6d3c; continue;
 }
 0x30b6d3c => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x30b6d4d; continue;
 }
 0x30b6d43 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b6d4d; continue; }
  pc = 0x30b6d48; continue;
 }
 0x30b6d48 => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x30b6d4d; continue;
 }
 0x30b6d4d => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x30b6d5e); f_030ae3a3(cpu); if cpu.returned_to != 0x30b6d5e { return; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x30b6d5f; continue;
 }
 0x30b6d5f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x30b6d77; continue; }
  pc = 0x30b6d67; continue;
 }
 0x30b6d67 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30b6d5f; continue; }
  pc = 0x30b6d77; continue;
 }
 0x30b6d77 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30b6d7a; continue;
 }
 0x30b6d7a => {
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
pub fn f_030b6d86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6d86); }
 let mut pc: u32 = 0x30b6d86;
 loop { match pc {
 0x30b6d86 => {
  pc = 0x30b7126; continue;
 }
 0x30b7126 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6d8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6d8b); }
 let mut pc: u32 = 0x30b6d8b;
 loop { match pc {
 0x30b6d8b => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (cpu.cf || cpu.zf) { pc = 0x30b6d95; continue; }
  pc = 0x30b6d91; continue;
 }
 0x30b6d91 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6d95 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x315b2c4u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6daa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6daa); }
 let mut pc: u32 = 0x30b6daa;
 loop { match pc {
 0x30b6daa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x30b6dc0; continue; }
  pc = 0x30b6db9; continue;
 }
 0x30b6db9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30b6dc0); f_030b2237(cpu); if cpu.returned_to != 0x30b6dc0 { return; }
  pc = 0x30b6dc0; continue;
 }
 0x30b6dc0 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x30b6ddc; continue; }
  pc = 0x30b6dca; continue;
 }
 0x30b6dca => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  pc = 0x30b6ddc; continue;
 }
 0x30b6ddc => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x30b6de0; continue;
 }
 0x30b6de0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b6df7; continue; }
  pc = 0x30b6de8; continue;
 }
 0x30b6de8 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x30b6df0); f_030b2193(cpu); if cpu.returned_to != 0x30b6df0 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b6de0; continue; }
  pc = 0x30b6df5; continue;
 }
 0x30b6df5 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x30b6df7; continue;
 }
 0x30b6df7 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30b6e15; continue; }
  pc = 0x30b6dfb; continue;
 }
 0x30b6dfb => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30b6e15; continue; }
  pc = 0x30b6e0c; continue;
 }
 0x30b6e0c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30b6e13); f_030ad4ba(cpu); if cpu.returned_to != 0x30b6e13 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x30b6e15; continue;
 }
 0x30b6e15 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30b6e1d; continue; }
  pc = 0x30b6e19; continue;
 }
 0x30b6e19 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x30b6e1d; continue;
 }
 0x30b6e1d => {
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
pub fn f_030b6e25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6e25); }
 let mut pc: u32 = 0x30b6e25;
 loop { match pc {
 0x30b6e25 => {
  { let v: u32 = cpu.r32(0x315be34u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6e2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6e2b); }
 let mut pc: u32 = 0x30b6e2b;
 loop { match pc {
 0x30b6e2b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x30b6e37); f_030a1edb(cpu); if cpu.returned_to != 0x30b6e37 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30b6e68; continue; }
  pc = 0x30b6e3e; continue;
 }
 0x30b6e3e => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x30b6e51; continue; }
  pc = 0x30b6e44; continue;
 }
 0x30b6e44 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x30b6e51; continue; }
  pc = 0x30b6e4a; continue;
 }
 0x30b6e4a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30b6e51); f_030b7129(cpu); if cpu.returned_to != 0x30b6e51 { return; }
  pc = 0x30b6e51; continue;
 }
 0x30b6e51 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x30b6e68; continue; }
  pc = 0x30b6e57; continue;
 }
 0x30b6e57 => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x30b6e65; continue; }
  pc = 0x30b6e5d; continue;
 }
 0x30b6e5d => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6e65 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x30b6e68; continue;
 }
 0x30b6e68 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6e6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6e6d); }
 let mut pc: u32 = 0x30b6e6d;
 loop { match pc {
 0x30b6e6d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_030b6f61(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6e77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6e77); }
 let mut pc: u32 = 0x30b6e77;
 loop { match pc {
 0x30b6e77 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x315d074u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30b6ea6; continue; }
  pc = 0x30b6e8a; continue;
 }
 0x30b6e8a => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x30b6ea6; continue; }
  pc = 0x30b6e8f; continue;
 }
 0x30b6e8f => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x30b6ea6; continue; }
  pc = 0x30b6e94; continue;
 }
 0x30b6e94 => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315d074u32, v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x30b6ea6); cpu.call(t); if cpu.returned_to != 0x30b6ea6 { return; }
  pc = 0x30b6ea6; continue;
 }
 0x30b6ea6 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6eac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b6eac); }
 let mut pc: u32 = 0x30b6eac;
 loop { match pc {
 0x30b6eac => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b6eb9; continue; }
  pc = 0x30b6eb4; continue;
 }
 0x30b6eb4 => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b6eca; continue; }
  pc = 0x30b6eb9; continue;
 }
 0x30b6eb9 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x30b6ec3); f_0309f3db(cpu); if cpu.returned_to != 0x30b6ec3 { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6eca => {
  { let v: u32 = 0x30b6e6du32; cpu.w32(0x315d0a0u32, v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x30b6ef1; continue; }
  pc = 0x30b6ed9; continue;
 }
 0x30b6ed9 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x30b6ee5; continue; }
  pc = 0x30b6ede; continue;
 }
 0x30b6ede => {
  cpu.push(0x30b6ee3); f_030b5ee0(cpu); if cpu.returned_to != 0x30b6ee3 { return; }
  pc = 0x30b6f21; continue;
 }
 0x30b6ee5 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x30b6f21; continue; }
  pc = 0x30b6eea; continue;
 }
 0x30b6eea => {
  cpu.push(0x30b6eef); f_030b60ce(cpu); if cpu.returned_to != 0x30b6eef { return; }
  pc = 0x30b6f21; continue;
 }
 0x30b6ef1 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if !cpu.zf { pc = 0x30b6f0e; continue; }
  pc = 0x30b6ef6; continue;
 }
 0x30b6ef6 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x30b6f02; continue; }
  pc = 0x30b6efb; continue;
 }
 0x30b6efb => {
  cpu.push(0x30b6f00); f_030b6096(cpu); if cpu.returned_to != 0x30b6f00 { return; }
  pc = 0x30b6f21; continue;
 }
 0x30b6f02 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x30b6f21; continue; }
  pc = 0x30b6f07; continue;
 }
 0x30b6f07 => {
  cpu.push(0x30b6f0c); f_030b61e9(cpu); if cpu.returned_to != 0x30b6f0c { return; }
  pc = 0x30b6f21; continue;
 }
 0x30b6f0e => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x30b6f21; continue; }
  pc = 0x30b6f13; continue;
 }
 0x30b6f13 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x30b6f21; continue; }
  pc = 0x30b6f17; continue;
 }
 0x30b6f17 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x30b6f21; continue; }
  pc = 0x30b6f1c; continue;
 }
 0x30b6f1c => {
  cpu.push(0x30b6f21); f_030b7151(cpu); if cpu.returned_to != 0x30b6f21 { return; }
  pc = 0x30b6f21; continue;
 }
 0x30b6f21 => {
  { let v: u32 = cpu.r32(0x315d06cu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d06cu32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b6f31(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b6f31); } r_030b6f31(cpu, 0x30b6f31); }
pub fn f_030b6f61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b6f61); } r_030b6f31(cpu, 0x30b6f61); }
fn r_030b6f31(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b6f31 => {
  { let v: u8 = cpu.r8(0x8b030b6fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x30b6f36, "Outsd");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0xbu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0xf7030b6fu32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  cpu.unimplemented(0x30b6f42, "Outsd");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0xbu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.unimplemented(0x30b6f4a, "Outsd");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0xbu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  cpu.unimplemented(0x30b6f52, "Outsd");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0xbu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  cpu.unimplemented(0x30b6f5a, "Outsd");
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0xbu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x51u32.wrapping_add(cpu.ebx))); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d06cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b7019; continue; }
  pc = 0x30b6f83; continue;
 }
 0x30b6f61 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d06cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b7019; continue; }
  pc = 0x30b6f83; continue;
 }
 0x30b6f83 => {
  cpu.poll(); pc = cpu.r32(0x30b6f31u32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x30b6f8b => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x30b6f95); f_030b6e77(cpu); if cpu.returned_to != 0x30b6f95 { return; }
  pc = 0x30b6f95; continue;
 }
 0x30b6f95 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30b6f97; continue;
 }
 0x30b6f97 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b6fa0 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x30b6fab; continue; }
  pc = 0x30b6fa6; continue;
 }
 0x30b6fa6 => {
  cpu.push(0x30b6fab); f_030b713f(cpu); if cpu.returned_to != 0x30b6fab { return; }
  pc = 0x30b6fab; continue;
 }
 0x30b6fab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fb3; continue;
 }
 0x30b6fb3 => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fb8; continue;
 }
 0x30b6fb8 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fbd; continue;
 }
 0x30b6fbd => {
  { let v: u32 = 0x2u32; cpu.w32(0x315d06cu32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.push(0x30b6fcd); f_030b5ee0(cpu); if cpu.returned_to != 0x30b6fcd { return; }
  pc = 0x30b6fcd; continue;
 }
 0x30b6fcd => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x30b6fcf; continue;
 }
 0x30b6fcf => {
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x30b6fd2); cpu.call(t); if cpu.returned_to != 0x30b6fd2 { return; }
  cpu.poll(); pc = 0x30b6f95; continue;
 }
 0x30b6fd4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fdc; continue;
 }
 0x30b6fdc => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fe1; continue;
 }
 0x30b6fe1 => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fe6; continue;
 }
 0x30b6fe6 => {
  { let v: u32 = 0x2u32; cpu.w32(0x315d06cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x30b6ff5); f_030b6096(cpu); if cpu.returned_to != 0x30b6ff5 { return; }
  cpu.poll(); pc = 0x30b6fcd; continue;
 }
 0x30b6ff7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b6fff; continue;
 }
 0x30b6fff => {
  { cpu.sub32(cpu.ebp, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b7004; continue;
 }
 0x30b7004 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b6f95; continue; }
  pc = 0x30b7009; continue;
 }
 0x30b7009 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315d06cu32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x30b6fcf; continue;
 }
 0x30b7019 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b6f97; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7023(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7023); }
 let mut pc: u32 = 0x30b7023;
 loop { match pc {
 0x30b7023 => {
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
  if cpu.ecx == 0 { pc = 0x30b7042; continue; }
  pc = 0x30b7037; continue;
 }
 0x30b7037 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x30b7042; continue; }
  pc = 0x30b703f; continue;
 }
 0x30b703f => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30b7047; continue;
 }
 0x30b7042 => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30b7047; continue;
 }
 0x30b7047 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30b7058; continue; }
  pc = 0x30b704b; continue;
 }
 0x30b704b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x30b7058; continue; }
  pc = 0x30b7053; continue;
 }
 0x30b7053 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x30b7047; continue;
 }
 0x30b7058 => {
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
pub fn f_030b7062(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7062); }
 let mut pc: u32 = 0x30b7062;
 loop { match pc {
 0x30b7062 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x30b708b; continue; }
  pc = 0x30b7073; continue;
 }
 0x30b7073 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x30b708b; continue; }
  pc = 0x30b7081; continue;
 }
 0x30b7081 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x30b708b; continue; }
  pc = 0x30b7085; continue;
 }
 0x30b7085 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1080u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b708f; continue;
 }
 0x30b708b => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1040u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b708f; continue;
 }
 0x30b708f => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x30b7097); f_030b2655(cpu); if cpu.returned_to != 0x30b7097 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b709e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b709e); }
 let mut pc: u32 = 0x30b709e;
 loop { match pc {
 0x30b709e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x30b70aa); f_0308d622(cpu); if cpu.returned_to != 0x30b70aa { return; }
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
pub fn f_030b70b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b70b7); }
 let mut pc: u32 = 0x30b70b7;
 loop { match pc {
 0x30b70b7 => {
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
  if !cpu.zf { pc = 0x30b70de; continue; }
  pc = 0x30b70da; continue;
 }
 0x30b70da => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30b7113; continue; }
  pc = 0x30b70de; continue;
 }
 0x30b70de => {
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
  pc = 0x30b7113; continue;
 }
 0x30b7113 => {
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
pub fn f_030b7129(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7129); }
 let mut pc: u32 = 0x30b7129;
 loop { match pc {
 0x30b7129 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_030ad4ba(cpu); return; }
  pc = 0x30b7131; continue;
 }
 0x30b7131 => {
  cpu.push(0x30b7136); f_0309df21(cpu); if cpu.returned_to != 0x30b7136 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b713f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b713f); }
 let mut pc: u32 = 0x30b713f;
 loop { match pc {
 0x30b713f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x314b888u32; cpu.eax = v; }
  cpu.push(0x30b714f); f_030adb62(cpu); if cpu.returned_to != 0x30b714f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7151); }
 let mut pc: u32 = 0x30b7151;
 loop { match pc {
 0x30b7151 => {
  pc = 0x30b7156; continue;
 }
 0x30b7156 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315d008u32), 0x30b71e5u32); }
  if cpu.zf { pc = 0x30b717f; continue; }
  pc = 0x30b7164; continue;
 }
 0x30b7164 => {
  { let v: u32 = 0x30b6e77u32; cpu.ebx = v; }
  { let v: u32 = 0x30b71e5u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be61u32, v); }
  cpu.push(0x30b7179); f_030b7188(cpu); if cpu.returned_to != 0x30b7179 { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315d008u32, v); }
  pc = 0x30b717f; continue;
 }
 0x30b717f => {
  cpu.push(0x30b7184); f_030b7242(cpu); if cpu.returned_to != 0x30b7184 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7188(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7188); }
 let mut pc: u32 = 0x30b7188;
 loop { match pc {
 0x30b7188 => {
  { cpu.sub16(cpu.r16(0x315d0a4u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x30b719d; continue; }
  pc = 0x30b7192; continue;
 }
 0x30b7192 => {
  { cpu.sub8(cpu.r8(0x315454cu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x30b719d; continue; }
  pc = 0x30b719b; continue;
 }
 0x30b719b => {
  pc = 0x30b719e; continue;
 }
 0x30b719d => {
  cpu.flat_ds(0x30b719d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b719e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x30b71b4; continue; }
  pc = 0x30b71ad; continue;
 }
 0x30b71ad => {
  cpu.flat_ds(0x30b71ad); cpu.push(0x30b71b2); f_030b75d8(cpu); if cpu.returned_to != 0x30b71b2 { return; }
  pc = 0x30b71d9; continue;
 }
 0x30b71b4 => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x30b71c4; continue; }
  pc = 0x30b71bd; continue;
 }
 0x30b71bd => {
  cpu.flat_ds(0x30b71bd); cpu.push(0x30b71c2); f_030b7592(cpu); if cpu.returned_to != 0x30b71c2 { return; }
  pc = 0x30b71d9; continue;
 }
 0x30b71c4 => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x30b71d4; continue; }
  pc = 0x30b71cd; continue;
 }
 0x30b71cd => {
  cpu.flat_ds(0x30b71cd); cpu.push(0x30b71d2); f_030b7630(cpu); if cpu.returned_to != 0x30b71d2 { return; }
  pc = 0x30b71d9; continue;
 }
 0x30b71d4 => {
  cpu.flat_ds(0x30b71d4); cpu.push(0x30b71d9); f_030b74a2(cpu); if cpu.returned_to != 0x30b71d9 { return; }
  pc = 0x30b71d9; continue;
 }
 0x30b71d9 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b71da); cpu.push(0x30b71df); f_030b7242(cpu); if cpu.returned_to != 0x30b71df { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x30b71e4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b71e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b71e5); }
 let mut pc: u32 = 0x30b71e5;
 loop { match pc {
 0x30b71e5 => {
  { cpu.sub16(cpu.r16(0x315d0a4u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x30b71f0; continue; }
  pc = 0x30b71ef; continue;
 }
 0x30b71ef => {
  cpu.flat_ds(0x30b71ef); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b71f0 => {
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
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x30b7217; continue; }
  pc = 0x30b7210; continue;
 }
 0x30b7210 => {
  cpu.flat_ds(0x30b7210); cpu.push(0x30b7215); f_030b7616(cpu); if cpu.returned_to != 0x30b7215 { return; }
  pc = 0x30b723c; continue;
 }
 0x30b7217 => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x30b7227; continue; }
  pc = 0x30b7220; continue;
 }
 0x30b7220 => {
  cpu.flat_ds(0x30b7220); cpu.push(0x30b7225); f_030b75bf(cpu); if cpu.returned_to != 0x30b7225 { return; }
  pc = 0x30b723c; continue;
 }
 0x30b7227 => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x30b7237; continue; }
  pc = 0x30b7230; continue;
 }
 0x30b7230 => {
  cpu.flat_ds(0x30b7230); cpu.push(0x30b7235); f_030b7688(cpu); if cpu.returned_to != 0x30b7235 { return; }
  pc = 0x30b723c; continue;
 }
 0x30b7237 => {
  cpu.flat_ds(0x30b7237); cpu.push(0x30b723c); f_030b7565(cpu); if cpu.returned_to != 0x30b723c { return; }
  pc = 0x30b723c; continue;
 }
 0x30b723c => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x30b7240); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7242(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7242); }
 let mut pc: u32 = 0x30b7242;
 loop { match pc {
 0x30b7242 => {
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
pub fn f_030b7257(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b7257); } r_030b7257(cpu, 0x30b7257); }
pub fn f_030b7264(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b7264); } r_030b7257(cpu, 0x30b7264); }
fn r_030b7257(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b7257 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { cpu.port_out(0xa0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out(0xf0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x30b7264; continue;
 }
 0x30b7264 => {
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
  cpu.unimplemented(0x30b7272, "Fnstenv");
  {  }
  {  }
  {  }
  {  }
  cpu.unimplemented(0x30b727a, "Fndisi");
  {  }
  cpu.flat_ds(0x30b727d); cpu.push(0x30b7282); f_0309e338(cpu); if cpu.returned_to != 0x30b7282 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x30b7292; continue;
 }
 0x30b7292 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if cpu.cf { cpu.poll(); pc = 0x30b7292; continue; }
  pc = 0x30b729c; continue;
 }
 0x30b729c => {
  { cpu.sub8((cpu.ebx as u8), 0xdfu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x30b7292; continue; }
  pc = 0x30b72a1; continue;
 }
 0x30b72a1 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let a = (cpu.ebx as u8); let b = ((cpu.ebx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x30b72b4; continue; }
  pc = 0x30b72b0; continue;
 }
 0x30b72b0 => {
  { let v: u8 = 0x86u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b7300; continue;
 }
 0x30b72b4 => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x30b72c8; continue; }
  pc = 0x30b72b9; continue;
 }
 0x30b72b9 => {
  { cpu.and16((cpu.edx as u16), 0x200u16); }
  if !cpu.zf { pc = 0x30b72c4; continue; }
  pc = 0x30b72c0; continue;
 }
 0x30b72c0 => {
  { let v: u8 = 0x8bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b72c6; continue;
 }
 0x30b72c4 => {
  { let v: u8 = 0x8au8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b72c6; continue;
 }
 0x30b72c6 => {
  pc = 0x30b7300; continue;
 }
 0x30b72c8 => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x30b72d4; continue; }
  pc = 0x30b72cd; continue;
 }
 0x30b72cd => {
  cpu.flat_ds(0x30b72cd); cpu.push(0x30b72d2); f_030b736a(cpu); if cpu.returned_to != 0x30b72d2 { return; }
  pc = 0x30b7300; continue;
 }
 0x30b72d4 => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x30b72dd; continue; }
  pc = 0x30b72d9; continue;
 }
 0x30b72d9 => {
  { let v: u8 = 0x82u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b7300; continue;
 }
 0x30b72dd => {
  { cpu.and8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x30b72eb; continue; }
  pc = 0x30b72e2; continue;
 }
 0x30b72e2 => {
  cpu.flat_ds(0x30b72e2); cpu.push(0x30b72e7); f_030b73f7(cpu); if cpu.returned_to != 0x30b72e7 { return; }
  { let v: u8 = 0x84u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b7300; continue;
 }
 0x30b72eb => {
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x30b72f4; continue; }
  pc = 0x30b72f0; continue;
 }
 0x30b72f0 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b7300; continue;
 }
 0x30b72f4 => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x30b7300; continue; }
  pc = 0x30b72f9; continue;
 }
 0x30b72f9 => {
  cpu.flat_ds(0x30b72f9); cpu.push(0x30b72fe); f_030b7449(cpu); if cpu.returned_to != 0x30b72fe { return; }
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b7300; continue;
 }
 0x30b7300 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x30b7350; continue; }
  pc = 0x30b7305; continue;
 }
 0x30b7305 => {
  { let v = cpu.r32(0x315be38u32.wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x315d2c6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x315d2c8u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x315d2c6u32; cpu.esp = v; }
  { let v: u32 = 0x315d0c6u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315be38u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x315be61u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x30b732e); cpu.push(0x30b7334); cpu.call(t); if cpu.returned_to != 0x30b7334 { return; }
  { let v = cpu.r16(0x315d2c6u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x315d2c8u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x315be38u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x315d2c6u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x30b7350; continue;
 }
 0x30b7350 => {
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
  cpu.flat_ds(0x30b7369); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b736a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b736a); }
 let mut pc: u32 = 0x30b736a;
 loop { match pc {
 0x30b736a => {
  { let v: u8 = 0x81u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xd9fau16); }
  if !cpu.zf { pc = 0x30b7376; continue; }
  pc = 0x30b7373; continue;
 }
 0x30b7373 => {
  { let v: u8 = 0x88u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b7376 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f1u16); }
  if !cpu.zf { pc = 0x30b7380; continue; }
  pc = 0x30b737d; continue;
 }
 0x30b737d => {
  { let v: u8 = 0x8eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b7380 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f8u16); }
  if cpu.zf { pc = 0x30b738c; continue; }
  pc = 0x30b7387; continue;
 }
 0x30b7387 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f5u16); }
  pc = 0x30b738c; continue;
 }
 0x30b738c => {
  if !cpu.zf { pc = 0x30b7392; continue; }
  pc = 0x30b738e; continue;
 }
 0x30b738e => {
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b73f6; continue;
 }
 0x30b7392 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x310u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x310u16); }
  if !cpu.zf { pc = 0x30b73a5; continue; }
  pc = 0x30b73a1; continue;
 }
 0x30b73a1 => {
  { let v: u8 = 0x8du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b73f6; continue;
 }
 0x30b73a5 => {
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x30b73cf; continue; }
  pc = 0x30b73b1; continue;
 }
 0x30b73b1 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x30b73c4; continue; }
  pc = 0x30b73bf; continue;
 }
 0x30b73bf => {
  cpu.push(0x30b73c4); f_030b7494(cpu); if cpu.returned_to != 0x30b73c4 { return; }
  pc = 0x30b73c4; continue;
 }
 0x30b73c4 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x30b73cb; continue; }
  pc = 0x30b73c9; continue;
 }
 0x30b73c9 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x30b73cb; continue;
 }
 0x30b73cb => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b73f6; continue;
 }
 0x30b73cf => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x130u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x30u16); }
  if !cpu.zf { pc = 0x30b73f6; continue; }
  pc = 0x30b73dd; continue;
 }
 0x30b73dd => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x38u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x30b73f6; continue; }
  pc = 0x30b73f4; continue;
 }
 0x30b73f4 => {
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30b73f6; continue;
 }
 0x30b73f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b73f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b73f7); }
 let mut pc: u32 = 0x30b73f7;
 loop { match pc {
 0x30b73f7 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9fdu16); }
  if !cpu.zf { pc = 0x30b7400; continue; }
  pc = 0x30b73fe; continue;
 }
 0x30b73fe => {
  pc = 0x30b7448; continue;
 }
 0x30b7400 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x30b742c; continue; }
  pc = 0x30b740f; continue;
 }
 0x30b740f => {
  cpu.push(0x30b7414); f_030b7449(cpu); if cpu.returned_to != 0x30b7414 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x30b7423; continue; }
  pc = 0x30b741e; continue;
 }
 0x30b741e => {
  cpu.push(0x30b7423); f_030b7494(cpu); if cpu.returned_to != 0x30b7423 { return; }
  pc = 0x30b7423; continue;
 }
 0x30b7423 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x30b742a; continue; }
  pc = 0x30b7428; continue;
 }
 0x30b7428 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x30b742a; continue;
 }
 0x30b742a => {
  pc = 0x30b7448; continue;
 }
 0x30b742c => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x30b7443; continue; }
  pc = 0x30b7436; continue;
 }
 0x30b7436 => {
  { let v: u32 = 0x315d0b0u32; cpu.esi = v; }
  cpu.push(0x30b7441); f_030b746f(cpu); if cpu.returned_to != 0x30b7441 { return; }
  pc = 0x30b7448; continue;
 }
 0x30b7443 => {
  cpu.push(0x30b7448); f_030b7449(cpu); if cpu.returned_to != 0x30b7448 { return; }
  pc = 0x30b7448; continue;
 }
 0x30b7448 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7449); }
 let mut pc: u32 = 0x30b7449;
 loop { match pc {
 0x30b7449 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x30b745e; continue; }
  pc = 0x30b7456; continue;
 }
 0x30b7456 => {
  { let v = cpu.rf64(0x315d0bau32); cpu.fpush(v); }
  pc = 0x30b7464; continue;
 }
 0x30b745e => {
  { let v = cpu.rf32(0x315d0c2u32); cpu.fpush(v); }
  pc = 0x30b7464; continue;
 }
 0x30b7464 => {
  { cpu.and16(cpu.r16(0x14u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if cpu.zf { pc = 0x30b746e; continue; }
  pc = 0x30b746c; continue;
 }
 0x30b746c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x30b746e; continue;
 }
 0x30b746e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b746f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b746f); }
 let mut pc: u32 = 0x30b746f;
 loop { match pc {
 0x30b746f => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if !cpu.zf { pc = 0x30b7478; continue; }
  pc = 0x30b7474; continue;
 }
 0x30b7474 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30b747d; continue;
 }
 0x30b7478 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.edx as u8), 0x7u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30b747d; continue;
 }
 0x30b747d => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x30b747e; continue;
 }
 0x30b747e => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x30b7486; continue; }
  pc = 0x30b7482; continue;
 }
 0x30b7482 => {
  cpu.unimplemented(0x30b7482, "Fincstp");
  cpu.poll(); pc = 0x30b747e; continue;
 }
 0x30b7486 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x30b748b; continue;
 }
 0x30b748b => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x30b7493; continue; }
  pc = 0x30b748f; continue;
 }
 0x30b748f => {
  cpu.unimplemented(0x30b748f, "Fdecstp");
  cpu.poll(); pc = 0x30b748b; continue;
 }
 0x30b7493 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7494(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7494); }
 let mut pc: u32 = 0x30b7494;
 loop { match pc {
 0x30b7494 => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x30b749e; continue; }
  pc = 0x30b7499; continue;
 }
 0x30b7499 => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30b74a1; continue;
 }
 0x30b749e => {
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30b74a1; continue;
 }
 0x30b74a1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b74a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b74a2); }
 let mut pc: u32 = 0x30b74a2;
 loop { match pc {
 0x30b74a2 => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x315d2cfu32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x30b74b4; continue; }
  pc = 0x30b74ac; continue;
 }
 0x30b74ac => {
  { let v: u32 = 0x30b7257u32; cpu.edx = v; }
  pc = 0x30b74ba; continue;
 }
 0x30b74b4 => {
  { let v: u32 = 0x30b7264u32; cpu.edx = v; }
  pc = 0x30b74ba; continue;
 }
 0x30b74ba => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x30b74e1; continue; }
  pc = 0x30b74c3; continue;
 }
 0x30b74c3 => {
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
  pc = 0x30b74e1; continue;
 }
 0x30b74e1 => {
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x315d2cdu32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d0a6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x315d0a4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d0aau32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x30b7558; continue; }
  pc = 0x30b751a; continue;
 }
 0x30b751a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x30b751d, "Sidt");
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
  pc = 0x30b7563; continue;
 }
 0x30b7558 => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2506u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  pc = 0x30b7563; continue;
 }
 0x30b7563 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b7564); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7565(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7565); }
 let mut pc: u32 = 0x30b7565;
 loop { match pc {
 0x30b7565 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x315d2cdu32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x315d0aau32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315d0a6u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x315d0a4u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b7591); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7592(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7592); }
 let mut pc: u32 = 0x30b7592;
 loop { match pc {
 0x30b7592 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x315d2ccu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d0a6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x315d0a4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x315d2ccu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x30b7264u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b75be); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b75bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b75bf); }
 let mut pc: u32 = 0x30b75bf;
 loop { match pc {
 0x30b75bf => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x315d2ccu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x315d0a6u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x315d0a4u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b75d7); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b75d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b75d8); }
 let mut pc: u32 = 0x30b75d8;
 loop { match pc {
 0x30b75d8 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x315d2ceu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d0a6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x315d0a4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x315d2ceu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x315d2cfu32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x30b7607; continue; }
  pc = 0x30b75ff; continue;
 }
 0x30b75ff => {
  { let v: u32 = 0x30b7257u32; cpu.edx = v; }
  pc = 0x30b760d; continue;
 }
 0x30b7607 => {
  { let v: u32 = 0x30b7264u32; cpu.edx = v; }
  pc = 0x30b760d; continue;
 }
 0x30b760d => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b7615); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7616(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7616); }
 let mut pc: u32 = 0x30b7616;
 loop { match pc {
 0x30b7616 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x315d2ceu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x315d0a6u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x315d0a4u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30b762e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7630); }
 let mut pc: u32 = 0x30b7630;
 loop { match pc {
 0x30b7630 => {
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x315d2ccu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d0a6u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315d0a4u32, v); }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x315d2ccu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315d0aau32, v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x315d2cfu32), 0x20u8); }
  if !cpu.zf { pc = 0x30b7675; continue; }
  pc = 0x30b766d; continue;
 }
 0x30b766d => {
  { let v: u32 = 0x30b7257u32; cpu.edx = v; }
  pc = 0x30b767b; continue;
 }
 0x30b7675 => {
  { let v: u32 = 0x30b7264u32; cpu.edx = v; }
  pc = 0x30b767b; continue;
 }
 0x30b767b => {
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x315d2ccu32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7688); }
 let mut pc: u32 = 0x30b7688;
 loop { match pc {
 0x30b7688 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7690); }
 let mut pc: u32 = 0x30b7690;
 loop { match pc {
 0x30b7690 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x30b7695; continue;
 }
 0x30b7695 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x30b76a3; continue; }
  pc = 0x30b769f; continue;
 }
 0x30b769f => {
  { let v: u16 = (cpu.eax as u16) | 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b76a3; continue;
 }
 0x30b76a3 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if cpu.zf { pc = 0x30b76af; continue; }
  pc = 0x30b76ab; continue;
 }
 0x30b76ab => {
  { let v: u16 = (cpu.eax as u16) | 0x7e0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b76af; continue;
 }
 0x30b76af => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if cpu.zf { pc = 0x30b76bb; continue; }
  pc = 0x30b76b7; continue;
 }
 0x30b76b7 => {
  { let v: u16 = (cpu.eax as u16) | 0xf800u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30b76bb; continue;
 }
 0x30b76bb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3223f90u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x30b76d2; continue; }
  pc = 0x30b76cf; continue;
 }
 0x30b76cf => {
  { let v: u32 = cpu.eax & 0xffffffe0u32; cpu.eax = v; }
  pc = 0x30b76d2; continue;
 }
 0x30b76d2 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if !cpu.zf { pc = 0x30b76df; continue; }
  pc = 0x30b76da; continue;
 }
 0x30b76da => {
  { let v: u32 = cpu.eax & 0xfffff81fu32; cpu.eax = v; }
  pc = 0x30b76df; continue;
 }
 0x30b76df => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if !cpu.zf { pc = 0x30b76ec; continue; }
  pc = 0x30b76e7; continue;
 }
 0x30b76e7 => {
  { let v: u32 = cpu.eax & 0xffff07ffu32; cpu.eax = v; }
  pc = 0x30b76ec; continue;
 }
 0x30b76ec => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x3225f90u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30b7695; continue; }
  pc = 0x30b76ff; continue;
 }
 0x30b76ff => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7701(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7701); }
 let mut pc: u32 = 0x30b7701;
 loop { match pc {
 0x30b7701 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x30b7721; continue; }
  pc = 0x30b7708; continue;
 }
 0x30b7708 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0x3223f90u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7721; continue;
 }
 0x30b7721 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7722(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7722); }
 let mut pc: u32 = 0x30b7722;
 loop { match pc {
 0x30b7722 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16) ^ (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x30b774b; continue; }
  pc = 0x30b7732; continue;
 }
 0x30b7732 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.eax as u16), cpu.r16(0x3225f90u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b774b; continue;
 }
 0x30b774b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b774c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b774c); }
 let mut pc: u32 = 0x30b774c;
 loop { match pc {
 0x30b774c => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x30b7767; continue; }
  pc = 0x30b7750; continue;
 }
 0x30b7750 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7bef7befu32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7767; continue;
 }
 0x30b7767 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7768(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7768); }
 let mut pc: u32 = 0x30b7768;
 loop { match pc {
 0x30b7768 => {
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
pub fn f_030b778d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b778d); }
 let mut pc: u32 = 0x30b778d;
 loop { match pc {
 0x30b778d => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x31537b8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31537b4u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x31537a0u32, v); }
  pc = 0x30b7c65; continue;
 }
 0x30b77be => {
  { let v: u32 = cpu.r32(0x31537b4u32); cpu.ecx = v; }
  pc = 0x30b79dc; continue;
 }
 0x30b77c9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b77e7; continue; }
  pc = 0x30b77d0; continue;
 }
 0x30b77d0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b77e7; continue;
 }
 0x30b77e7 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7808; continue; }
  pc = 0x30b77ef; continue;
 }
 0x30b77ef => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7808; continue;
 }
 0x30b7808 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7829; continue; }
  pc = 0x30b7810; continue;
 }
 0x30b7810 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7829; continue;
 }
 0x30b7829 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b784a; continue; }
  pc = 0x30b7831; continue;
 }
 0x30b7831 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b784a; continue;
 }
 0x30b784a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b786b; continue; }
  pc = 0x30b7852; continue;
 }
 0x30b7852 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b786b; continue;
 }
 0x30b786b => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b788c; continue; }
  pc = 0x30b7873; continue;
 }
 0x30b7873 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x30b788c; continue;
 }
 0x30b788c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b78ad; continue; }
  pc = 0x30b7894; continue;
 }
 0x30b7894 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30b78ad; continue;
 }
 0x30b78ad => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b78ce; continue; }
  pc = 0x30b78b5; continue;
 }
 0x30b78b5 => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30b78ce; continue;
 }
 0x30b78ce => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b78ef; continue; }
  pc = 0x30b78d6; continue;
 }
 0x30b78d6 => {
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b78ef; continue;
 }
 0x30b78ef => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7910; continue; }
  pc = 0x30b78f7; continue;
 }
 0x30b78f7 => {
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7910; continue;
 }
 0x30b7910 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7931; continue; }
  pc = 0x30b7918; continue;
 }
 0x30b7918 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7931; continue;
 }
 0x30b7931 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7952; continue; }
  pc = 0x30b7939; continue;
 }
 0x30b7939 => {
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7952; continue;
 }
 0x30b7952 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7973; continue; }
  pc = 0x30b795a; continue;
 }
 0x30b795a => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7973; continue;
 }
 0x30b7973 => {
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7994; continue; }
  pc = 0x30b797b; continue;
 }
 0x30b797b => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7994; continue;
 }
 0x30b7994 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b79b5; continue; }
  pc = 0x30b799c; continue;
 }
 0x30b799c => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x30b79b5; continue;
 }
 0x30b79b5 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b79d6; continue; }
  pc = 0x30b79bd; continue;
 }
 0x30b79bd => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x30b79d6; continue;
 }
 0x30b79d6 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x30b79dc; continue;
 }
 0x30b79dc => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b77c9; continue; }
  pc = 0x30b79e5; continue;
 }
 0x30b79e5 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b79f1; continue;
 }
 0x30b79f1 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7a0f; continue; }
  pc = 0x30b79f8; continue;
 }
 0x30b79f8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7a0f; continue;
 }
 0x30b7a0f => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7a2f; continue; }
  pc = 0x30b7a16; continue;
 }
 0x30b7a16 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7a2f; continue;
 }
 0x30b7a2f => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7a3b; continue;
 }
 0x30b7a3b => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7a59; continue; }
  pc = 0x30b7a42; continue;
 }
 0x30b7a42 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7a59; continue;
 }
 0x30b7a59 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7a79; continue; }
  pc = 0x30b7a60; continue;
 }
 0x30b7a60 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7a79; continue;
 }
 0x30b7a79 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7a85; continue;
 }
 0x30b7a85 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7aa3; continue; }
  pc = 0x30b7a8c; continue;
 }
 0x30b7a8c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7aa3; continue;
 }
 0x30b7aa3 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7ac3; continue; }
  pc = 0x30b7aaa; continue;
 }
 0x30b7aaa => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7ac3; continue;
 }
 0x30b7ac3 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7acf; continue;
 }
 0x30b7acf => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7aed; continue; }
  pc = 0x30b7ad6; continue;
 }
 0x30b7ad6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7aed; continue;
 }
 0x30b7aed => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7b0d; continue; }
  pc = 0x30b7af4; continue;
 }
 0x30b7af4 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7b0d; continue;
 }
 0x30b7b0d => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7b19; continue;
 }
 0x30b7b19 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7b37; continue; }
  pc = 0x30b7b20; continue;
 }
 0x30b7b20 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7b37; continue;
 }
 0x30b7b37 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7b57; continue; }
  pc = 0x30b7b3e; continue;
 }
 0x30b7b3e => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7b57; continue;
 }
 0x30b7b57 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7b63; continue;
 }
 0x30b7b63 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7b81; continue; }
  pc = 0x30b7b6a; continue;
 }
 0x30b7b6a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7b81; continue;
 }
 0x30b7b81 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7ba1; continue; }
  pc = 0x30b7b88; continue;
 }
 0x30b7b88 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7ba1; continue;
 }
 0x30b7ba1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7bad; continue;
 }
 0x30b7bad => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7bcb; continue; }
  pc = 0x30b7bb4; continue;
 }
 0x30b7bb4 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7bcb; continue;
 }
 0x30b7bcb => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7beb; continue; }
  pc = 0x30b7bd2; continue;
 }
 0x30b7bd2 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7beb; continue;
 }
 0x30b7beb => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b7c34; continue; }
  pc = 0x30b7bf3; continue;
 }
 0x30b7bf3 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7c11; continue; }
  pc = 0x30b7bfa; continue;
 }
 0x30b7bfa => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7c11; continue;
 }
 0x30b7c11 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7c31; continue; }
  pc = 0x30b7c18; continue;
 }
 0x30b7c18 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7c31; continue;
 }
 0x30b7c31 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b7c34; continue;
 }
 0x30b7c34 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x30b7c5d; continue; }
  pc = 0x30b7c3c; continue;
 }
 0x30b7c3c => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7c5a; continue; }
  pc = 0x30b7c43; continue;
 }
 0x30b7c43 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7c5a; continue;
 }
 0x30b7c5a => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30b7c5d; continue;
 }
 0x30b7c5d => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x31537a0u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x30b7c65; continue;
 }
 0x30b7c65 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x31537b8u32)); cpu.w32(0x31537b8u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x30b77be; continue; }
  pc = 0x30b7c71; continue;
 }
 0x30b7c71 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7c75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7c75); }
 let mut pc: u32 = 0x30b7c75;
 loop { match pc {
 0x30b7c75 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x31537b8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31537b4u32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x3153744u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x30b7cc7; continue;
 }
 0x30b7ca2 => {
  { let v: u32 = cpu.r32(0x31537b4u32); cpu.ecx = v; }
  pc = 0x30b7ca8; continue;
 }
 0x30b7ca8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x30b7cba; continue; }
  pc = 0x30b7cb0; continue;
 }
 0x30b7cb0 => {
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x3153744u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7cba; continue;
 }
 0x30b7cba => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b7ca8; continue; }
  pc = 0x30b7cc3; continue;
 }
 0x30b7cc3 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x30b7cc7; continue;
 }
 0x30b7cc7 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x31537b8u32)); cpu.w32(0x31537b8u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x30b7ca2; continue; }
  pc = 0x30b7ccf; continue;
 }
 0x30b7ccf => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7cd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b7cd3); }
 let mut pc: u32 = 0x30b7cd3;
 loop { match pc {
 0x30b7cd3 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x31537b8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31537b4u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x3153746u32, v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x31537a0u32, v); }
  pc = 0x30b7d42; continue;
 }
 0x30b7d02 => {
  { let v: u32 = cpu.r32(0x31537b4u32); cpu.ecx = v; }
  pc = 0x30b7d08; continue;
 }
 0x30b7d08 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x3153746u32)); }
  if cpu.zf { pc = 0x30b7d31; continue; }
  pc = 0x30b7d16; continue;
 }
 0x30b7d16 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x3223f90u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7d31; continue;
 }
 0x30b7d31 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b7d08; continue; }
  pc = 0x30b7d3a; continue;
 }
 0x30b7d3a => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x31537a0u32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x30b7d42; continue;
 }
 0x30b7d42 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x31537b8u32)); cpu.w32(0x31537b8u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x30b7d02; continue; }
  pc = 0x30b7d4a; continue;
 }
 0x30b7d4a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b7d4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b7d4e); } r_030b7d4e(cpu, 0x30b7d4e); }
pub fn f_030b812e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b812e); } r_030b7d4e(cpu, 0x30b812e); }
pub fn f_030b8187(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b8187); } r_030b7d4e(cpu, 0x30b8187); }
pub fn f_030b81e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b81e0); } r_030b7d4e(cpu, 0x30b81e0); }
pub fn f_030b8239(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b8239); } r_030b7d4e(cpu, 0x30b8239); }
pub fn f_030b8281(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b8281); } r_030b7d4e(cpu, 0x30b8281); }
pub fn f_030b82da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b82da); } r_030b7d4e(cpu, 0x30b82da); }
pub fn f_030b8333(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b8333); } r_030b7d4e(cpu, 0x30b8333); }
pub fn f_030b838c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b838c); } r_030b7d4e(cpu, 0x30b838c); }
fn r_030b7d4e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30b7d4e => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153750u32, v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153754u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315375cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3153764u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153760u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3153768u32, v); }
  { let v: u32 = cpu.r32(0x3153764u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315376cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3153770u32, v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3153788u32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x30b7de5; continue; }
  pc = 0x30b7dae; continue;
 }
 0x30b7dae => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3153750u32).wrapping_sub(cpu.eax); cpu.w32(0x3153750u32, v); }
  { cpu.and32(cpu.r32(0x3153750u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b83a7; continue; }
  pc = 0x30b7dc6; continue;
 }
 0x30b7dc6 => {
  { let v = cpu.r32(0x3153764u32); cpu.imul32(v); }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x315375cu32).wrapping_sub(cpu.eax); cpu.w32(0x315375cu32, v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315376cu32, v); }
  { let v: u32 = cpu.r32(0x3153788u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b7de5; continue;
 }
 0x30b7de5 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3153750u32)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3153790u32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b7e1c; continue; }
  pc = 0x30b7df7; continue;
 }
 0x30b7df7 => {
  { let v: u32 = cpu.r32(0x3153750u32).wrapping_sub(cpu.eax); cpu.w32(0x3153750u32, v); }
  { cpu.and32(cpu.r32(0x3153750u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b83a7; continue; }
  pc = 0x30b7e0d; continue;
 }
 0x30b7e0d => {
  { let v = cpu.r32(0x3153764u32); cpu.imul32(v); }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x315375cu32).wrapping_sub(cpu.eax); cpu.w32(0x315375cu32, v); }
  pc = 0x30b7e1c; continue;
 }
 0x30b7e1c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x315378cu32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x30b7e5e; continue; }
  pc = 0x30b7e27; continue;
 }
 0x30b7e27 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3153754u32).wrapping_sub(cpu.eax); cpu.w32(0x3153754u32, v); }
  { cpu.and32(cpu.r32(0x3153754u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b83a7; continue; }
  pc = 0x30b7e3f; continue;
 }
 0x30b7e3f => {
  { let v = cpu.r32(0x3153768u32); cpu.imul32(v); }
  { let v = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x3153760u32).wrapping_sub(cpu.eax); cpu.w32(0x3153760u32, v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153770u32, v); }
  { let v: u32 = cpu.r32(0x315378cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b7e5e; continue;
 }
 0x30b7e5e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3153754u32)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3153794u32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b7e95; continue; }
  pc = 0x30b7e70; continue;
 }
 0x30b7e70 => {
  { let v: u32 = cpu.r32(0x3153754u32).wrapping_sub(cpu.eax); cpu.w32(0x3153754u32, v); }
  { cpu.and32(cpu.r32(0x3153754u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b83a7; continue; }
  pc = 0x30b7e86; continue;
 }
 0x30b7e86 => {
  { let v = cpu.r32(0x3153768u32); cpu.imul32(v); }
  { let v = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x3153760u32).wrapping_sub(cpu.eax); cpu.w32(0x3153760u32, v); }
  pc = 0x30b7e95; continue;
 }
 0x30b7e95 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153758u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x315375cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3153750u32); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153774u32, v); }
  { let v: u32 = cpu.r32(0x3153760u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3153754u32); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153778u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153780u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3153750u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3153750u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153784u32, v); }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x30b83aa; continue; }
  pc = 0x30b7f01; continue;
 }
 0x30b7f01 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x30b8447; continue; }
  pc = 0x30b7f0b; continue;
 }
 0x30b7f0b => {
  { let v: u32 = cpu.r32(0x3153770u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3153754u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153798u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x3153798u32)); cpu.w32(0x3153798u32, v); }
  pc = 0x30b839b; continue;
 }
 0x30b7f26 => {
  { let v: u32 = cpu.r32(0x3153758u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x3153780u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315376cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3153750u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315379cu32, v); }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30b7f9b; continue; }
  pc = 0x30b7f58; continue;
 }
 0x30b7f58 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b7f98; continue; }
  pc = 0x30b7f6d; continue;
 }
 0x30b7f6d => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b7f98; continue;
 }
 0x30b7f98 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30b7f9b; continue;
 }
 0x30b7f9b => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315379cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b811d; continue; }
  pc = 0x30b7faa; continue;
 }
 0x30b7faa => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b8003; continue; }
  pc = 0x30b7fd7; continue;
 }
 0x30b7fd7 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8003; continue;
 }
 0x30b8003 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b805c; continue; }
  pc = 0x30b8030; continue;
 }
 0x30b8030 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b805c; continue;
 }
 0x30b805c => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b80b5; continue; }
  pc = 0x30b8089; continue;
 }
 0x30b8089 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b80b5; continue;
 }
 0x30b80b5 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b810e; continue; }
  pc = 0x30b80e2; continue;
 }
 0x30b80e2 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30b810e; continue;
 }
 0x30b810e => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315379cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30b7faa; continue; }
  pc = 0x30b811d; continue;
 }
 0x30b811d => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x315379cu32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x31536f0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30b812e => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b8184; continue; }
  pc = 0x30b815b; continue;
 }
 0x30b815b => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8184; continue;
 }
 0x30b8184 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b8187; continue;
 }
 0x30b8187 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b81dd; continue; }
  pc = 0x30b81b4; continue;
 }
 0x30b81b4 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b81dd; continue;
 }
 0x30b81dd => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b81e0; continue;
 }
 0x30b81e0 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b8236; continue; }
  pc = 0x30b820d; continue;
 }
 0x30b820d => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8236; continue;
 }
 0x30b8236 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b8239; continue;
 }
 0x30b8239 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b8279; continue; }
  pc = 0x30b824e; continue;
 }
 0x30b824e => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8279; continue;
 }
 0x30b8279 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30b838c; continue;
 }
 0x30b8281 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b82d7; continue; }
  pc = 0x30b82ae; continue;
 }
 0x30b82ae => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b82d7; continue;
 }
 0x30b82d7 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b82da; continue;
 }
 0x30b82da => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b8330; continue; }
  pc = 0x30b8307; continue;
 }
 0x30b8307 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8330; continue;
 }
 0x30b8330 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b8333; continue;
 }
 0x30b8333 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x30b8389; continue; }
  pc = 0x30b8360; continue;
 }
 0x30b8360 => {
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
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8389; continue;
 }
 0x30b8389 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30b838c; continue;
 }
 0x30b838c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x3153778u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3153784u32)); cpu.edi = v; }
  pc = 0x30b839b; continue;
 }
 0x30b839b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x3153798u32)); cpu.w32(0x3153798u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b7f26; continue; }
  pc = 0x30b83a7; continue;
 }
 0x30b83a7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b83aa => {
  { let v: u32 = cpu.r32(0x3153770u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3153754u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153798u32, v); }
  pc = 0x30b8438; continue;
 }
 0x30b83bc => {
  { let v: u32 = cpu.r32(0x3153758u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x3153780u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315376cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3153750u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315379cu32, v); }
  pc = 0x30b83e6; continue;
 }
 0x30b83e6 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16) ^ cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x30b841e; continue; }
  pc = 0x30b8405; continue;
 }
 0x30b8405 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x3225f90u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b841e; continue;
 }
 0x30b841e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x315379cu32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30b83e6; continue; }
  pc = 0x30b8429; continue;
 }
 0x30b8429 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x3153778u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3153784u32)); cpu.edi = v; }
  pc = 0x30b8438; continue;
 }
 0x30b8438 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x3153798u32)); cpu.w32(0x3153798u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b83bc; continue; }
  pc = 0x30b8444; continue;
 }
 0x30b8444 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b8447 => {
  { let v: u32 = cpu.r32(0x3153770u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3153754u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153798u32, v); }
  pc = 0x30b84b6; continue;
 }
 0x30b8459 => {
  { let v: u32 = cpu.r32(0x3153758u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x3153780u32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315376cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3153750u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315379cu32, v); }
  pc = 0x30b8483; continue;
 }
 0x30b8483 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x3153774u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x30b849c; continue; }
  pc = 0x30b8499; continue;
 }
 0x30b8499 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b849c; continue;
 }
 0x30b849c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x315379cu32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30b8483; continue; }
  pc = 0x30b84a7; continue;
 }
 0x30b84a7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x3153778u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3153784u32)); cpu.edi = v; }
  pc = 0x30b84b6; continue;
 }
 0x30b84b6 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x3153798u32)); cpu.w32(0x3153798u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b8459; continue; }
  pc = 0x30b84be; continue;
 }
 0x30b84be => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b84c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b84c1); }
 let mut pc: u32 = 0x30b84c1;
 loop { match pc {
 0x30b84c1 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b84d7; continue; }
  pc = 0x30b84cb; continue;
 }
 0x30b84cb => {
  if cpu.zf { pc = 0x30b84dd; continue; }
  pc = 0x30b84cd; continue;
 }
 0x30b84cd => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x30b84c1; continue; }
  pc = 0x30b84d3; continue;
 }
 0x30b84d3 => {
  { cpu.movs(2, false); }
  cpu.poll(); pc = 0x30b84c1; continue;
 }
 0x30b84d7 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x30b84c1; continue;
 }
 0x30b84dd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b84de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b84de); }
 let mut pc: u32 = 0x30b84de;
 loop { match pc {
 0x30b84de => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b8519; continue; }
  pc = 0x30b84e8; continue;
 }
 0x30b84e8 => {
  if cpu.zf { pc = 0x30b851f; continue; }
  pc = 0x30b84ea; continue;
 }
 0x30b84ea => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  pc = 0x30b84fd; continue;
 }
 0x30b84f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x30b84fd; continue;
 }
 0x30b84fd => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b84f0; continue; }
  pc = 0x30b8500; continue;
 }
 0x30b8500 => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x30b84de; continue; }
  pc = 0x30b8508; continue;
 }
 0x30b8508 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30b84de; continue;
 }
 0x30b8519 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x30b84de; continue;
 }
 0x30b851f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8520); }
 let mut pc: u32 = 0x30b8520;
 loop { match pc {
 0x30b8520 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b8556; continue; }
  pc = 0x30b852a; continue;
 }
 0x30b852a => {
  if cpu.zf { pc = 0x30b855c; continue; }
  pc = 0x30b852c; continue;
 }
 0x30b852c => {
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16) ^ cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x3225f90u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b852c; continue; }
  pc = 0x30b8554; continue;
 }
 0x30b8554 => {
  cpu.poll(); pc = 0x30b8520; continue;
 }
 0x30b8556 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x30b8520; continue;
 }
 0x30b855c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b855d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b855d); }
 let mut pc: u32 = 0x30b855d;
 loop { match pc {
 0x30b855d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x30b8561; continue;
 }
 0x30b8561 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b858e; continue; }
  pc = 0x30b856b; continue;
 }
 0x30b856b => {
  if cpu.zf { pc = 0x30b8594; continue; }
  pc = 0x30b856d; continue;
 }
 0x30b856d => {
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
  if !cpu.zf { cpu.poll(); pc = 0x30b856d; continue; }
  pc = 0x30b858c; continue;
 }
 0x30b858c => {
  cpu.poll(); pc = 0x30b8561; continue;
 }
 0x30b858e => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x30b8561; continue;
 }
 0x30b8594 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8595(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8595); }
 let mut pc: u32 = 0x30b8595;
 loop { match pc {
 0x30b8595 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3153740u32, v); }
  { let v = cpu.rf32(0x3153730u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x32a2e54u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x315373cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3153728u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3153720u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153734u32, v); }
  { let v: u32 = cpu.r32(0x3153724u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31537b8u32, v); }
  { let v: u32 = cpu.r32(0x315371cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31537bcu32, v); }
  pc = 0x30b85d4; continue;
 }
 0x30b85d4 => {
  { let v: u32 = cpu.r32(0x3153740u32); cpu.esi = v; }
  pc = 0x30b85da; continue;
 }
 0x30b85da => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x30b87bd; continue; }
  pc = 0x30b85e7; continue;
 }
 0x30b85e7 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x30b87af; continue; }
  pc = 0x30b85f0; continue;
 }
 0x30b85f0 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.rf32(0x315373cu32); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x31537bcu32); cpu.ebp = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x315372cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3153738u32, v); }
  { cpu.and32(cpu.ebp, 0x2u32); }
  if !cpu.zf { pc = 0x30b8696; continue; }
  pc = 0x30b8634; continue;
 }
 0x30b8634 => {
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3153730u32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x31537a8u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31537a8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b8693; continue; }
  pc = 0x30b864a; continue;
 }
 0x30b864a => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x315374cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31537a8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x31537a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x30b866a; continue; }
  pc = 0x30b8668; continue;
 }
 0x30b8668 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x30b866a; continue;
 }
 0x30b866a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x3227f90u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x3247f90u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b8693; continue;
 }
 0x30b8693 => {
  { let v: u32 = cpu.ebp.wrapping_sub(0x2u32); cpu.ebp = v; }
  pc = 0x30b8696; continue;
 }
 0x30b8696 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3153738u32)); cpu.eax = v; }
  if cpu.zf { pc = 0x30b8742; continue; }
  pc = 0x30b86a4; continue;
 }
 0x30b86a4 => {
  if cpu.sf { pc = 0x30b87a1; continue; }
  pc = 0x30b86aa; continue;
 }
 0x30b86aa => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  pc = 0x30b86ac; continue;
 }
 0x30b86ac => {
  { let v: u32 = cpu.r32(0x3153730u32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x31537a8u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31537a8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b872d; continue; }
  pc = 0x30b86c0; continue;
 }
 0x30b86c0 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x315374cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31537a8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x31537a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x30b86e0; continue; }
  pc = 0x30b86de; continue;
 }
 0x30b86de => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x30b86e0; continue;
 }
 0x30b86e0 => {
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffe00000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3227f8eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1f0000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3247f8eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x3227f90u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x3247f90u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffeu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b872d; continue;
 }
 0x30b872d => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x3153738u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b86ac; continue; }
  pc = 0x30b873e; continue;
 }
 0x30b873e => {
  if !cpu.zf { pc = 0x30b87a1; continue; }
  pc = 0x30b8740; continue;
 }
 0x30b8740 => {
  pc = 0x30b8744; continue;
 }
 0x30b8742 => {
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  pc = 0x30b8744; continue;
 }
 0x30b8744 => {
  { let v: u32 = cpu.r32(0x3153730u32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x31537a8u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31537a8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b87a1; continue; }
  pc = 0x30b8758; continue;
 }
 0x30b8758 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x315374cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31537a8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x31537a8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x30b8778; continue; }
  pc = 0x30b8776; continue;
 }
 0x30b8776 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x30b8778; continue;
 }
 0x30b8778 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x3227f90u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x3247f90u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30b87a1; continue;
 }
 0x30b87a1 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.poll(); pc = 0x30b85da; continue;
 }
 0x30b87af => {
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b85f0; continue; }
  pc = 0x30b87b8; continue;
 }
 0x30b87b8 => {
  cpu.poll(); pc = 0x30b85da; continue;
 }
 0x30b87bd => {
  { let v: u32 = cpu.r32(0x3153740u32).wrapping_add(0x1cu32); cpu.w32(0x3153740u32, v); }
  { let v: u32 = cpu.r32(0x3153734u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x31537bcu32), cpu.eax); cpu.w32(0x31537bcu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x31537b8u32)); cpu.w32(0x31537b8u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b85d4; continue; }
  pc = 0x30b87db; continue;
 }
 0x30b87db => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b87dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b87dd); }
 let mut pc: u32 = 0x30b87dd;
 loop { match pc {
 0x30b87dd => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3248790u32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x30b87f0; continue;
 }
 0x30b87f0 => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b884c; continue; }
  pc = 0x30b87f4; continue;
 }
 0x30b87f4 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x3248798u32, v); }
  { let v: u32 = cpu.r32(0x3248790u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3248798u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x3248af4u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x3248fa8u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x3248790u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x3248794u32, v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3248798u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x3248af4u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x3248794u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x3248fa8u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x30b8852; continue;
 }
 0x30b884c => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x30b8852; continue;
 }
 0x30b8852 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b87f0; continue; }
  pc = 0x30b8856; continue;
 }
 0x30b8856 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8858(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8858); }
 let mut pc: u32 = 0x30b8858;
 loop { match pc {
 0x30b8858 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.fpu_control; cpu.w16(0x324879cu32, v); }
  { let v: u16 = cpu.r16(0x324879cu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp | 0xc00u32; cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x324879eu32, v); }
  { cpu.fpu_control = cpu.r16(0x324879eu32); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x3248ae0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x3248ae4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3248ad4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3248ad8u32, v); }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3248adcu32, v); }
  { let v = (cpu.r32(0x3248ae4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3248ae8u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x3153718u32), cpu.ebx); }
  if cpu.zf { pc = 0x30b88d4; continue; }
  pc = 0x30b88ba; continue;
 }
 0x30b88ba => {
  { cpu.sub32(cpu.ebx, 0x96u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8b49; continue; }
  pc = 0x30b88c6; continue;
 }
 0x30b88c6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153718u32, v); }
  cpu.push(0x30b88d3); f_030b87dd(cpu); if cpu.returned_to != 0x30b88d3 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x30b88d4; continue;
 }
 0x30b88d4 => {
  { let v: u32 = 0x32487acu32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32487a0u32, v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x30b88e4; continue;
 }
 0x30b88e4 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x30b88e7; continue;
 }
 0x30b88e7 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b88f7; continue; }
  pc = 0x30b88f5; continue;
 }
 0x30b88f5 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x30b88f7; continue;
 }
 0x30b88f7 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b89c3; continue; }
  pc = 0x30b8901; continue;
 }
 0x30b8901 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x3248ae8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b89c3; continue; }
  pc = 0x30b8914; continue;
 }
 0x30b8914 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b896e; continue; }
  pc = 0x30b8929; continue;
 }
 0x30b8929 => {
  { let v = cpu.rf32(0x3153710u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x3153710u32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b89c3; continue; }
  pc = 0x30b8956; continue;
 }
 0x30b8956 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30b89bf; continue;
 }
 0x30b896e => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b89c3; continue; }
  pc = 0x30b89a9; continue;
 }
 0x30b89a9 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30b89bf; continue;
 }
 0x30b89bf => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x30b89c3; continue;
 }
 0x30b89c3 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b88e4; continue; }
  pc = 0x30b89cd; continue;
 }
 0x30b89cd => {
  { let v: u32 = cpu.r32(0x32487a0u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x30b88e7; continue; }
  pc = 0x30b89db; continue;
 }
 0x30b89db => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x30b8b49; continue; }
  pc = 0x30b89e3; continue;
 }
 0x30b89e3 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x32487a4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32487a8u32, v); }
  pc = 0x30b89f3; continue;
 }
 0x30b89f3 => {
  { let v: u32 = 0x32487acu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32487a4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32487a8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30b8a05; continue;
 }
 0x30b8a05 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8a1e; continue; }
  pc = 0x30b8a0a; continue;
 }
 0x30b8a0a => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30b8a1e; continue; }
  pc = 0x30b8a0f; continue;
 }
 0x30b8a0f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3248a2cu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x30b8a1e; continue;
 }
 0x30b8a1e => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8a05; continue; }
  pc = 0x30b8a24; continue;
 }
 0x30b8a24 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8b27; continue; }
  pc = 0x30b8a2d; continue;
 }
 0x30b8a2d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x3248a2cu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x30b8a39; continue;
 }
 0x30b8a39 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x30b8a3d; continue;
 }
 0x30b8a3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30b8a4b; continue; }
  pc = 0x30b8a46; continue;
 }
 0x30b8a46 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.edx = v; } }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  pc = 0x30b8a4b; continue;
 }
 0x30b8a4b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8a3d; continue; }
  pc = 0x30b8a4e; continue;
 }
 0x30b8a4e => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8a39; continue; }
  pc = 0x30b8a51; continue;
 }
 0x30b8a51 => {
  { let v: u32 = cpu.r32(0x32487a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3248af4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3248fa8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3248aecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3248af0u32, v); }
  { let v: u32 = 0x3248a2cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3248adcu32); cpu.ebp = v; }
  pc = 0x30b8a7a; continue;
 }
 0x30b8a7a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8000u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8000u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3248af0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30b8b1e; continue; }
  pc = 0x30b8a9f; continue;
 }
 0x30b8a9f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3248aecu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8b1e; continue; }
  pc = 0x30b8aa7; continue;
 }
 0x30b8aa7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3248aecu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8ab4; continue; }
  pc = 0x30b8aaf; continue;
 }
 0x30b8aaf => {
  { let v: u32 = cpu.r32(0x3248aecu32); cpu.eax = v; }
  pc = 0x30b8ab4; continue;
 }
 0x30b8ab4 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3248af0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b8ac2; continue; }
  pc = 0x30b8abc; continue;
 }
 0x30b8abc => {
  { let v: u32 = cpu.r32(0x3248af0u32); cpu.ebx = v; }
  pc = 0x30b8ac2; continue;
 }
 0x30b8ac2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8b1e; continue; }
  pc = 0x30b8ac7; continue;
 }
 0x30b8ac7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3248ad4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30b8aee; continue; }
  pc = 0x30b8ad9; continue;
 }
 0x30b8ad9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8b1e; continue; }
  pc = 0x30b8aee; continue;
 }
 0x30b8aee => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x30b8b07; continue; }
  pc = 0x30b8af4; continue;
 }
 0x30b8af4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8af4; continue; }
  pc = 0x30b8b07; continue;
 }
 0x30b8b07 => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30b8b1e; continue; }
  pc = 0x30b8b0f; continue;
 }
 0x30b8b0f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8b1e; continue;
 }
 0x30b8b1e => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b8a7a; continue; }
  pc = 0x30b8b27; continue;
 }
 0x30b8b27 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32487a8u32)); cpu.w32(0x32487a8u32, v); }
  { let v: u32 = cpu.r32(0x3248ad8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3248ad4u32).wrapping_add(cpu.eax); cpu.w32(0x3248ad4u32, v); }
  { let v: u32 = cpu.r32(0x32487a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3248ae4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30b89f3; continue; }
  pc = 0x30b8b49; continue;
 }
 0x30b8b49 => {
  { cpu.fpu_control = cpu.r16(0x324879cu32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8b51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8b51); }
 let mut pc: u32 = 0x30b8b51;
 loop { match pc {
 0x30b8b51 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.fpu_control; cpu.w16(0x324879cu32, v); }
  { let v: u16 = cpu.r16(0x324879cu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp | 0xc00u32; cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x324879eu32, v); }
  { cpu.fpu_control = cpu.r16(0x324879eu32); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x3248ae0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x3248ae4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3248ad4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3248ad8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x3248adcu32, v); }
  { let v = (cpu.r32(0x3248ae4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3248ae8u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x3153718u32), cpu.ebx); }
  if cpu.zf { pc = 0x30b8bc5; continue; }
  pc = 0x30b8bab; continue;
 }
 0x30b8bab => {
  { cpu.sub32(cpu.ebx, 0x96u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8e13; continue; }
  pc = 0x30b8bb7; continue;
 }
 0x30b8bb7 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3153718u32, v); }
  cpu.push(0x30b8bc4); f_030b87dd(cpu); if cpu.returned_to != 0x30b8bc4 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x30b8bc5; continue;
 }
 0x30b8bc5 => {
  { let v: u32 = 0x32487acu32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32487a0u32, v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x30b8bd5; continue;
 }
 0x30b8bd5 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x30b8bd8; continue;
 }
 0x30b8bd8 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b8be8; continue; }
  pc = 0x30b8be6; continue;
 }
 0x30b8be6 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x30b8be8; continue;
 }
 0x30b8be8 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b8cb4; continue; }
  pc = 0x30b8bf2; continue;
 }
 0x30b8bf2 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x3248ae8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30b8cb4; continue; }
  pc = 0x30b8c05; continue;
 }
 0x30b8c05 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8c5f; continue; }
  pc = 0x30b8c1a; continue;
 }
 0x30b8c1a => {
  { let v = cpu.rf32(0x3153710u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x3153710u32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8cb4; continue; }
  pc = 0x30b8c47; continue;
 }
 0x30b8c47 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30b8cb0; continue;
 }
 0x30b8c5f => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3153710u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8cb4; continue; }
  pc = 0x30b8c9a; continue;
 }
 0x30b8c9a => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3153714u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30b8cb0; continue;
 }
 0x30b8cb0 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x30b8cb4; continue;
 }
 0x30b8cb4 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b8bd5; continue; }
  pc = 0x30b8cbe; continue;
 }
 0x30b8cbe => {
  { let v: u32 = cpu.r32(0x32487a0u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x30b8bd8; continue; }
  pc = 0x30b8ccc; continue;
 }
 0x30b8ccc => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x30b8e13; continue; }
  pc = 0x30b8cd4; continue;
 }
 0x30b8cd4 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x32487a4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32487a8u32, v); }
  pc = 0x30b8ce4; continue;
 }
 0x30b8ce4 => {
  { let v: u32 = 0x32487acu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32487a4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32487a8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30b8cf6; continue;
 }
 0x30b8cf6 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8d0f; continue; }
  pc = 0x30b8cfb; continue;
 }
 0x30b8cfb => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30b8d0f; continue; }
  pc = 0x30b8d00; continue;
 }
 0x30b8d00 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3248a2cu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x30b8d0f; continue;
 }
 0x30b8d0f => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8cf6; continue; }
  pc = 0x30b8d15; continue;
 }
 0x30b8d15 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8df1; continue; }
  pc = 0x30b8d1e; continue;
 }
 0x30b8d1e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x3248a2cu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x30b8d2a; continue;
 }
 0x30b8d2a => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x30b8d2e; continue;
 }
 0x30b8d2e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30b8d3c; continue; }
  pc = 0x30b8d37; continue;
 }
 0x30b8d37 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.edx = v; } }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  pc = 0x30b8d3c; continue;
 }
 0x30b8d3c => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8d2e; continue; }
  pc = 0x30b8d3f; continue;
 }
 0x30b8d3f => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8d2a; continue; }
  pc = 0x30b8d42; continue;
 }
 0x30b8d42 => {
  { let v: u32 = cpu.r32(0x32487a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3248af4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3248fa8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3248aecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3248af0u32, v); }
  { let v: u32 = 0x3248a2cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3248adcu32); cpu.ebp = v; }
  pc = 0x30b8d6b; continue;
 }
 0x30b8d6b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8000u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8000u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3248af0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30b8de8; continue; }
  pc = 0x30b8d8c; continue;
 }
 0x30b8d8c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3248aecu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8de8; continue; }
  pc = 0x30b8d94; continue;
 }
 0x30b8d94 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3248aecu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8da1; continue; }
  pc = 0x30b8d9c; continue;
 }
 0x30b8d9c => {
  { let v: u32 = cpu.r32(0x3248aecu32); cpu.eax = v; }
  pc = 0x30b8da1; continue;
 }
 0x30b8da1 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3248af0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b8daf; continue; }
  pc = 0x30b8da9; continue;
 }
 0x30b8da9 => {
  { let v: u32 = cpu.r32(0x3248af0u32); cpu.ebx = v; }
  pc = 0x30b8daf; continue;
 }
 0x30b8daf => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8de8; continue; }
  pc = 0x30b8db4; continue;
 }
 0x30b8db4 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3248ad4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30b8dcf; continue; }
  pc = 0x30b8dc6; continue;
 }
 0x30b8dc6 => {
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8de8; continue; }
  pc = 0x30b8dcf; continue;
 }
 0x30b8dcf => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x30b8ddd; continue; }
  pc = 0x30b8dd5; continue;
 }
 0x30b8dd5 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b8dd5; continue; }
  pc = 0x30b8ddd; continue;
 }
 0x30b8ddd => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30b8de8; continue; }
  pc = 0x30b8de5; continue;
 }
 0x30b8de5 => {
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8de8; continue;
 }
 0x30b8de8 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b8d6b; continue; }
  pc = 0x30b8df1; continue;
 }
 0x30b8df1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32487a8u32)); cpu.w32(0x32487a8u32, v); }
  { let v: u32 = cpu.r32(0x3248ad8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3248ad4u32).wrapping_add(cpu.eax); cpu.w32(0x3248ad4u32, v); }
  { let v: u32 = cpu.r32(0x32487a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3248ae4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30b8ce4; continue; }
  pc = 0x30b8e13; continue;
 }
 0x30b8e13 => {
  { cpu.fpu_control = cpu.r16(0x324879cu32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8e20); }
 let mut pc: u32 = 0x30b8e20;
 loop { match pc {
 0x30b8e20 => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255c1cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c20u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c24u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x3255c28u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3255c2cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3255c3cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8e48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8e48); }
 let mut pc: u32 = 0x30b8e48;
 loop { match pc {
 0x30b8e48 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c28u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c30u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c34u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c38u32, v); }
  { cpu.sub32(cpu.r32(0x3255c28u32), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8e84; continue; }
  pc = 0x30b8e78; continue;
 }
 0x30b8e78 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x30b8e7e); f_030baee5(cpu); if cpu.returned_to != 0x30b8e7e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c28u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x30b8e84; continue;
 }
 0x30b8e84 => {
  { let v: u32 = cpu.r32(0x3255c30u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c34u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3255c38u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c1cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c20u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c24u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3255c3cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8eba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8eba); }
 let mut pc: u32 = 0x30b8eba;
 loop { match pc {
 0x30b8eba => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3255db4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255d90u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x3255d88u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255d84u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255d8cu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b8fb9; continue; }
  pc = 0x30b8ee5; continue;
 }
 0x30b8ee5 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x30b8eea; continue;
 }
 0x30b8eea => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 1u32); cpu.edi = v; }
  if cpu.zf { pc = 0x30b8f10; continue; }
  pc = 0x30b8ef2; continue;
 }
 0x30b8ef2 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.r32(0x3255d88u32); cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3255c20u32)); cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b8f0c; continue; }
  pc = 0x30b8f08; continue;
 }
 0x30b8f08 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.poll(); pc = 0x30b8eea; continue;
 }
 0x30b8f0c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.poll(); pc = 0x30b8eea; continue;
 }
 0x30b8f10 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255d98u32, v); }
  { let v = cpu.r32(0x3255d88u32); cpu.imul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x3255d94u32, v); }
  pc = 0x30b8f2a; continue;
 }
 0x30b8f2a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3255c20u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8f37; continue; }
  pc = 0x30b8f35; continue;
 }
 0x30b8f35 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30b8f37; continue;
 }
 0x30b8f37 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3255d84u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8f61; continue; }
  pc = 0x30b8f3f; continue;
 }
 0x30b8f3f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3255d8cu32); cpu.ebx = v; }
  cpu.push(0x30b8f4c); f_030b905c(cpu); if cpu.returned_to != 0x30b8f4c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b8f58; continue; }
  pc = 0x30b8f52; continue;
 }
 0x30b8f52 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x3255d9cu32, v); }
  pc = 0x30b8f58; continue;
 }
 0x30b8f58 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x3255d88u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b8f2a; continue; }
  pc = 0x30b8f61; continue;
 }
 0x30b8f61 => {
  { let v: u32 = cpu.r32(0x3255d94u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3255d98u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3255d88u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x3255d90u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30b8fb9; continue; }
  pc = 0x30b8f7c; continue;
 }
 0x30b8f7c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3255c20u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8f89; continue; }
  pc = 0x30b8f87; continue;
 }
 0x30b8f87 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30b8f89; continue;
 }
 0x30b8f89 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3255d84u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b8fb9; continue; }
  pc = 0x30b8f91; continue;
 }
 0x30b8f91 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3255d8cu32); cpu.ebx = v; }
  cpu.push(0x30b8f9e); f_030b905c(cpu); if cpu.returned_to != 0x30b8f9e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b8faa; continue; }
  pc = 0x30b8fa4; continue;
 }
 0x30b8fa4 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x3255d9cu32, v); }
  pc = 0x30b8faa; continue;
 }
 0x30b8faa => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3255d88u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x3255d90u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30b8f7c; continue; }
  pc = 0x30b8fb9; continue;
 }
 0x30b8fb9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3255db4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30b8fcc; continue; }
  pc = 0x30b8fc4; continue;
 }
 0x30b8fc4 => {
  { let v: u32 = cpu.r32(0x3255d9cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30b8fcc; continue;
 }
 0x30b8fcc => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b8fce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b8fce); }
 let mut pc: u32 = 0x30b8fce;
 loop { match pc {
 0x30b8fce => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3255db4u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b901e; continue; }
  pc = 0x30b8fde; continue;
 }
 0x30b8fde => {
  { let v: u32 = cpu.edx; cpu.w32(0x3255da4u32, v); }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255da0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255da8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3255dacu32, v); }
  pc = 0x30b8ff9; continue;
 }
 0x30b8ff9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3255da8u32); cpu.ebx = v; }
  cpu.push(0x30b9005); f_030b905c(cpu); if cpu.returned_to != 0x30b9005 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b9010; continue; }
  pc = 0x30b900a; continue;
 }
 0x30b900a => {
  { let v: u32 = cpu.esi; cpu.w32(0x3255db0u32, v); }
  pc = 0x30b9010; continue;
 }
 0x30b9010 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3255da4u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x3255da0u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30b8ff9; continue; }
  pc = 0x30b901e; continue;
 }
 0x30b901e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3255db4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30b9043; continue; }
  pc = 0x30b9029; continue;
 }
 0x30b9029 => {
  { let v: u32 = cpu.r32(0x3255db0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255dacu32)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0x3255da4u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3255db4u32); cpu.eax = v; }
  pc = 0x30b9043; continue;
 }
 0x30b9043 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9045(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9045); }
 let mut pc: u32 = 0x30b9045;
 loop { match pc {
 0x30b9045 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3255db4u32, v); }
  cpu.push(0x30b9055); f_030b905c(cpu); if cpu.returned_to != 0x30b9055 { return; }
  { let v: u32 = cpu.r32(0x3255db4u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b905c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b905c); }
 let mut pc: u32 = 0x30b905c;
 loop { match pc {
 0x30b905c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30b91f2; continue; }
  pc = 0x30b906e; continue;
 }
 0x30b906e => {
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebp)), 0x4u16); }
  if cpu.zf { pc = 0x30b9106; continue; }
  pc = 0x30b907a; continue;
 }
 0x30b907a => {
  { let v: u32 = cpu.r32(0x3255c28u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c24u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b9096; continue;
 }
 0x30b9096 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c24u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b90ac; continue;
 }
 0x30b90ac => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c20u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b90c2; continue;
 }
 0x30b90c2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c20u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b90d8; continue;
 }
 0x30b90d8 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c1cu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b90ee; continue;
 }
 0x30b90ee => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c1cu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b9104; continue;
 }
 0x30b9104 => {
  pc = 0x30b915b; continue;
 }
 0x30b9106 => {
  { let v: u32 = cpu.r32(0x3255c28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3255c20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b9123; continue;
 }
 0x30b9123 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b912b; continue;
 }
 0x30b912b => {
  { let v: u32 = cpu.r32(0x3255c1cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b913b; continue;
 }
 0x30b913b => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b9143; continue;
 }
 0x30b9143 => {
  { let v: u32 = cpu.r32(0x3255c24u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b9153; continue;
 }
 0x30b9153 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x30b91f2; continue; }
  pc = 0x30b915b; continue;
 }
 0x30b915b => {
  { let v: u32 = cpu.r32(0x3255c28u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x31547fcu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3154800u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c10u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c40u32, v); }
  { cpu.sub32(cpu.r32(0x3255c10u32), 0x0u32); }
  if cpu.zf { pc = 0x30b91f8; continue; }
  pc = 0x30b9185; continue;
 }
 0x30b9185 => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c50u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c54u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c58u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x31547e0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31547e4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x31547e8u32, v); }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.esi = v; }
  { let v: u32 = 0x2au32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.esi = v; }
  pc = 0x30b91c1; continue;
 }
 0x30b91c1 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30b91dc; continue; }
  pc = 0x30b91cb; continue;
 }
 0x30b91cb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x3154808u32.wrapping_add(cpu.ebx)); cpu.push(0x30b91d6); cpu.call(t); if cpu.returned_to != 0x30b91d6 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30b91c1; continue;
 }
 0x30b91dc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b91f2; continue; }
  pc = 0x30b91e2; continue;
 }
 0x30b91e2 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c14u32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.push(0x30b91f1); f_030b9203(cpu); if cpu.returned_to != 0x30b91f1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b91f2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b91f8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255db4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9203(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9203); }
 let mut pc: u32 = 0x30b9203;
 loop { match pc {
 0x30b9203 => {
  { let v: u32 = cpu.r32(0x3255c14u32); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x3255dbcu32, v); }
  cpu.push(0x30b9218); f_030b9219(cpu); if cpu.returned_to != 0x30b9218 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9219(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9219); }
 let mut pc: u32 = 0x30b9219;
 loop { match pc {
 0x30b9219 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30b923a; continue; }
  pc = 0x30b9223; continue;
 }
 0x30b9223 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x30b9232); f_030b9219(cpu); if cpu.returned_to != 0x30b9232 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x30b9238); f_030b9219(cpu); if cpu.returned_to != 0x30b9238 { return; }
  pc = 0x30b925f; continue;
 }
 0x30b923a => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255db8u32, v); }
  pc = 0x30b923f; continue;
 }
 0x30b923f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x30b9245); f_030b9265(cpu); if cpu.returned_to != 0x30b9245 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b924f; continue; }
  pc = 0x30b924a; continue;
 }
 0x30b924a => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255dbcu32, v); }
  pc = 0x30b924f; continue;
 }
 0x30b924f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x3255db8u32)); cpu.w32(0x3255db8u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30b923f; continue; }
  pc = 0x30b925f; continue;
 }
 0x30b925f => {
  { let v: u32 = cpu.r32(0x3255dbcu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9265(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9265); }
 let mut pc: u32 = 0x30b9265;
 loop { match pc {
 0x30b9265 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x3255c10u32), 0x2u32); }
  if cpu.zf { pc = 0x30b92c0; continue; }
  pc = 0x30b926f; continue;
 }
 0x30b926f => {
  { let v: u32 = cpu.r32(0x3255c1cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c20u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3255c24u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c44u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c48u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c4cu32, v); }
  cpu.push(0x30b9296); f_030b948f(cpu); if cpu.returned_to != 0x30b9296 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255c28u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9343; continue; }
  pc = 0x30b92a2; continue;
 }
 0x30b92a2 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3255c2cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b9343; continue; }
  pc = 0x30b92ae; continue;
 }
 0x30b92ae => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x30b92b6); f_030b9515(cpu); if cpu.returned_to != 0x30b92b6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b9343; continue; }
  pc = 0x30b92be; continue;
 }
 0x30b92be => {
  pc = 0x30b9306; continue;
 }
 0x30b92c0 => {
  { let v: u32 = cpu.r32(0x3255c1cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c20u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3255c24u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255c44u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c48u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c4cu32, v); }
  cpu.push(0x30b92e7); f_030b934a(cpu); if cpu.returned_to != 0x30b92e7 { return; }
  if cpu.zf { pc = 0x30b9343; continue; }
  pc = 0x30b92e9; continue;
 }
 0x30b92e9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255c44u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c48u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c4cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x30b9302); f_030b9515(cpu); if cpu.returned_to != 0x30b9302 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b9343; continue; }
  pc = 0x30b9306; continue;
 }
 0x30b9306 => {
  { let v: u32 = cpu.r32(0x3255c3cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.sf == cpu.of) { pc = 0x30b9337; continue; }
  pc = 0x30b9310; continue;
 }
 0x30b9310 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x3255c68u32, v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v: u32 = 0x3255c50u32; cpu.esi = v; }
  { let v: u32 = 0x3255c6cu32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.inc32(cpu.r32(0x3255c3cu32)); cpu.w32(0x3255c3cu32, v); }
  pc = 0x30b9337; continue;
 }
 0x30b9337 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255db4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b9343 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b934a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b934a); }
 let mut pc: u32 = 0x30b934a;
 loop { match pc {
 0x30b934a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9357; continue; }
  pc = 0x30b9355; continue;
 }
 0x30b9355 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30b9357; continue;
 }
 0x30b9357 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x325ce60u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30b9379; continue; }
  pc = 0x30b9373; continue;
 }
 0x30b9373 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x30b9379; continue;
 }
 0x30b9379 => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255c5cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c60u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c64u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3255c30u32); cpu.eax = v; }
  { let v = cpu.r32(0x3255c5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3255c34u32); cpu.eax = v; }
  { let v = cpu.r32(0x3255c60u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3255c38u32); cpu.eax = v; }
  { let v = cpu.r32(0x3255c64u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.fpush(1.0); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255dccu32, v); }
  { let v = (cpu.r32(0x3255dccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3255c44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c50u32)); cpu.eax = v; }
  { let v = cpu.r32(0x3255c5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3255c48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c54u32)); cpu.eax = v; }
  { let v = cpu.r32(0x3255c60u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3255c4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c58u32)); cpu.eax = v; }
  { let v = cpu.r32(0x3255c64u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255dccu32, v); }
  { let v = (cpu.r32(0x3255dccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x3255dccu32, v); }
  { let v: u32 = cpu.r32(0x3255dccu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3f800000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b948a; continue; }
  pc = 0x30b9437; continue;
 }
 0x30b9437 => {
  { let v = (cpu.r32(0x3255c30u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3255c44u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3255dc0u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x3255c34u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3255c48u32); cpu.ebx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3255dc4u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x3255c38u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3255c4cu32); cpu.ecx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3255dc8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3255dc0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x3255dc4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x3255dc8u32)); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.or32(cpu.edx, 0xffffffffu32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b948a => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b948f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b948f); }
 let mut pc: u32 = 0x30b948f;
 loop { match pc {
 0x30b948f => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b949b; continue; }
  pc = 0x30b9499; continue;
 }
 0x30b9499 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30b949b; continue;
 }
 0x30b949b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x325ce60u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30b94bd; continue; }
  pc = 0x30b94b7; continue;
 }
 0x30b94b7 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x30b94bd; continue;
 }
 0x30b94bd => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255c5cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255c60u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3255c64u32, v); }
  { let v: u32 = cpu.r32(0x3255c44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c50u32)); cpu.eax = v; }
  { let v = cpu.r32(0x3255c5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3255c48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c54u32)); cpu.eax = v; }
  { let v = cpu.r32(0x3255c60u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3255c4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c58u32)); cpu.eax = v; }
  { let v = cpu.r32(0x3255c64u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9515(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9515); }
 let mut pc: u32 = 0x30b9515;
 loop { match pc {
 0x30b9515 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x3255df4u32; cpu.edi = v; }
  cpu.push(0x30b9520); f_030b96b8(cpu); if cpu.returned_to != 0x30b9520 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3255c44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3255c28u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255ddcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b9599; continue; }
  pc = 0x30b9534; continue;
 }
 0x30b9534 => {
  { let v: u32 = cpu.r32(0x3255c44u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c28u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255de8u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9599; continue; }
  pc = 0x30b9547; continue;
 }
 0x30b9547 => {
  { let v: u32 = cpu.r32(0x3255c48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3255c28u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255de0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b9599; continue; }
  pc = 0x30b955a; continue;
 }
 0x30b955a => {
  { let v: u32 = cpu.r32(0x3255c48u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c28u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255decu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9599; continue; }
  pc = 0x30b956d; continue;
 }
 0x30b956d => {
  { let v: u32 = cpu.r32(0x3255c4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3255c28u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255de4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b9599; continue; }
  pc = 0x30b9580; continue;
 }
 0x30b9580 => {
  { let v: u32 = cpu.r32(0x3255c4cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c28u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255df0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9599; continue; }
  pc = 0x30b9593; continue;
 }
 0x30b9593 => {
  cpu.push(0x30b9598); f_030b959c(cpu); if cpu.returned_to != 0x30b9598 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b9599 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b959c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b959c); }
 let mut pc: u32 = 0x30b959c;
 loop { match pc {
 0x30b959c => {
  { let v: u32 = cpu.r32(0x3255c5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3255c60u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3255c64u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b95b3; continue; }
  pc = 0x30b95b1; continue;
 }
 0x30b95b1 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30b95b3; continue;
 }
 0x30b95b3 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b95b9; continue; }
  pc = 0x30b95b7; continue;
 }
 0x30b95b7 => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x30b95b9; continue;
 }
 0x30b95b9 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b95bf; continue; }
  pc = 0x30b95bd; continue;
 }
 0x30b95bd => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x30b95bf; continue;
 }
 0x30b95bf => {
  { let v: u32 = 0x3255df4u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30b961e; continue; }
  pc = 0x30b95cd; continue;
 }
 0x30b95cd => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30b961e; continue; }
  pc = 0x30b95d1; continue;
 }
 0x30b95d1 => {
  { let v: u32 = cpu.r32(0x3255c64u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x30b95da; continue;
 }
 0x30b95da => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c44u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x3255c48u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c44u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x3255c48u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b96b1; continue; }
  pc = 0x30b9613; continue;
 }
 0x30b9613 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b95da; continue; }
  pc = 0x30b9619; continue;
 }
 0x30b9619 => {
  pc = 0x30b96b4; continue;
 }
 0x30b961e => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30b966a; continue; }
  pc = 0x30b9622; continue;
 }
 0x30b9622 => {
  { let v: u32 = cpu.r32(0x3255c60u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  { let v: u32 = !cpu.edi; cpu.edi = v; }
  pc = 0x30b962d; continue;
 }
 0x30b962d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c44u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x3255c4cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c44u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x3255c4cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b96b1; continue; }
  pc = 0x30b9662; continue;
 }
 0x30b9662 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b962d; continue; }
  pc = 0x30b9668; continue;
 }
 0x30b9668 => {
  pc = 0x30b96b4; continue;
 }
 0x30b966a => {
  { let v: u32 = cpu.r32(0x3255c5cu32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x30b9673; continue;
 }
 0x30b9673 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c48u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x3255c4cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3255c48u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x3255c4cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b96b1; continue; }
  pc = 0x30b96a9; continue;
 }
 0x30b96a9 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b9673; continue; }
  pc = 0x30b96af; continue;
 }
 0x30b96af => {
  pc = 0x30b96b4; continue;
 }
 0x30b96b1 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b96b4 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b96b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b96b8); }
 let mut pc: u32 = 0x30b96b8;
 loop { match pc {
 0x30b96b8 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xeu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255ddcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255de0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255de4u32, v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3255de8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255decu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255df0u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x30b96ee; continue;
 }
 0x30b96ee => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3255ddcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9715; continue; }
  pc = 0x30b9710; continue;
 }
 0x30b9710 => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255ddcu32, v); }
  pc = 0x30b9715; continue;
 }
 0x30b9715 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3255de0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9723; continue; }
  pc = 0x30b971d; continue;
 }
 0x30b971d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x3255de0u32, v); }
  pc = 0x30b9723; continue;
 }
 0x30b9723 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x3255de4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b9731; continue; }
  pc = 0x30b972b; continue;
 }
 0x30b972b => {
  { let v: u32 = cpu.ecx; cpu.w32(0x3255de4u32, v); }
  pc = 0x30b9731; continue;
 }
 0x30b9731 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3255de8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b973e; continue; }
  pc = 0x30b9739; continue;
 }
 0x30b9739 => {
  { let v: u32 = cpu.eax; cpu.w32(0x3255de8u32, v); }
  pc = 0x30b973e; continue;
 }
 0x30b973e => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3255decu32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b974c; continue; }
  pc = 0x30b9746; continue;
 }
 0x30b9746 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x3255decu32, v); }
  pc = 0x30b974c; continue;
 }
 0x30b974c => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x3255df0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30b975a; continue; }
  pc = 0x30b9754; continue;
 }
 0x30b9754 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x3255df0u32, v); }
  pc = 0x30b975a; continue;
 }
 0x30b975a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b96ee; continue; }
  pc = 0x30b9768; continue;
 }
 0x30b9768 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9773(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9773); }
 let mut pc: u32 = 0x30b9773;
 loop { match pc {
 0x30b9773 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3255c6cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3255c3cu32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30b97bd; continue; }
  pc = 0x30b9783; continue;
 }
 0x30b9783 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x30b9783; continue; }
  pc = 0x30b97bd; continue;
 }
 0x30b97bd => {
  { let v: u32 = cpu.r32(0x3255c3cu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b97c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b97c4); }
 let mut pc: u32 = 0x30b97c4;
 loop { match pc {
 0x30b97c4 => {
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
pub fn f_030b9816(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9816); }
 let mut pc: u32 = 0x30b9816;
 loop { match pc {
 0x30b9816 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3255f24u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3255f28u32, v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x30b9828; continue;
 }
 0x30b9828 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x30b9833; continue;
 }
 0x30b9833 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b983a; continue; }
  pc = 0x30b9837; continue;
 }
 0x30b9837 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x30b983a; continue;
 }
 0x30b983a => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x3255f28u32)); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b9872; continue; }
  pc = 0x30b9842; continue;
 }
 0x30b9842 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x3255f28u32)); cpu.edx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b9872; continue; }
  pc = 0x30b984a; continue;
 }
 0x30b984a => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x3255f24u32)); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b985c; continue; }
  pc = 0x30b9852; continue;
 }
 0x30b9852 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x3255f24u32)); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b9864; continue; }
  pc = 0x30b985a; continue;
 }
 0x30b985a => {
  pc = 0x30b9872; continue;
 }
 0x30b985c => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x3255f24u32)); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x30b9871; continue; }
  pc = 0x30b9864; continue;
 }
 0x30b9864 => {
  { let v = cpu.edx; cpu.imul32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if (cpu.sf != cpu.of) { pc = 0x30b9872; continue; }
  pc = 0x30b9871; continue;
 }
 0x30b9871 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x30b9872; continue;
 }
 0x30b9872 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30b9828; continue; }
  pc = 0x30b9878; continue;
 }
 0x30b9878 => {
  if (cpu.sf != cpu.of) { pc = 0x30b9887; continue; }
  pc = 0x30b987a; continue;
 }
 0x30b987a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.poll(); pc = 0x30b9833; continue;
 }
 0x30b9887 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9890); }
 let mut pc: u32 = 0x30b9890;
 loop { match pc {
 0x30b9890 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x3155958u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x315594cu32), cpu.ebp); }
  if !cpu.zf { pc = 0x30b98b7; continue; }
  pc = 0x30b98a7; continue;
 }
 0x30b98a7 => {
  { cpu.sub32(cpu.r32(0x3155944u32), cpu.eax); }
  if !cpu.zf { pc = 0x30b98b7; continue; }
  pc = 0x30b98af; continue;
 }
 0x30b98af => {
  { cpu.sub32(cpu.r32(0x3155948u32), cpu.ebx); }
  if cpu.zf { pc = 0x30b98f9; continue; }
  pc = 0x30b98b7; continue;
 }
 0x30b98b7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x3155944u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3155948u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x315594cu32, v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3155950u32, v); }
  { let v = (cpu.r32(0x3155944u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x31559dcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31559e0u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x3155948u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x31559dcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31559e4u32, v); cpu.fpop(); }
  pc = 0x30b98f9; continue;
 }
 0x30b98f9 => {
  { cpu.sub32(cpu.r32(0x315597cu32), cpu.ecx); }
  if !cpu.zf { pc = 0x30b991e; continue; }
  pc = 0x30b9901; continue;
 }
 0x30b9901 => {
  { cpu.sub32(cpu.r32(0x3155980u32), cpu.edx); }
  if !cpu.zf { pc = 0x30b991e; continue; }
  pc = 0x30b9909; continue;
 }
 0x30b9909 => {
  { cpu.sub32(cpu.r32(0x315598cu32), cpu.esi); }
  if !cpu.zf { pc = 0x30b991e; continue; }
  pc = 0x30b9911; continue;
 }
 0x30b9911 => {
  { cpu.sub32(cpu.r32(0x3155960u32), cpu.edi); }
  if !cpu.zf { pc = 0x30b991e; continue; }
  pc = 0x30b9919; continue;
 }
 0x30b9919 => {
  pc = 0x30b9a64; continue;
 }
 0x30b991e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x315597cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x3155980u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x315598cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3155960u32, v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3155990u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x3155990u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x3155994u32, v); cpu.fpop(); }
  { let v = cpu.rf64(0x31545d4u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x3155990u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31559a8u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x31559a0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31559a4u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x3155990u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x31559a0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x3155998u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x315597cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3155984u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x3155980u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3155988u32, v); cpu.fpop(); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x3155964u32, v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x3155968u32, v); }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315596cu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x3155970u32, v); }
  { let v = (cpu.r32(0x315596cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3155974u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x3155970u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3155978u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x315596cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x31545ccu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x31559c4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x31559c4u32)); cpu.w32(0x31559c4u32, v); }
  { let v = (cpu.r32(0x3155970u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x31545ccu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x31559c8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x31559c8u32)); cpu.w32(0x31559c8u32, v); }
  { let v = (cpu.r32(0x315596cu32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x31545ccu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x31559ccu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x3155970u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x315598cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x31545ccu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x31559d0u32, v); cpu.fpop(); }
  pc = 0x30b9a64; continue;
 }
 0x30b9a64 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9a66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9a66); }
 let mut pc: u32 = 0x30b9a66;
 loop { match pc {
 0x30b9a66 => {
  { cpu.pushad(); }
  cpu.push(0x30b9a6c); f_030c3638(cpu); if cpu.returned_to != 0x30b9a6c { return; }
  cpu.push(0x30b9a71); f_030c3713(cpu); if cpu.returned_to != 0x30b9a71 { return; }
  cpu.push(0x30b9a76); f_030b9aea(cpu); if cpu.returned_to != 0x30b9a76 { return; }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  cpu.push(0x30b9a80); f_030b9a94(cpu); if cpu.returned_to != 0x30b9a80 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9a82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9a82); }
 let mut pc: u32 = 0x30b9a82;
 loop { match pc {
 0x30b9a82 => {
  { let v: u32 = cpu.eax; cpu.w32(0x31559d4u32, v); }
  { let v = (cpu.r32(0x31559d4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x31559d8u32, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9a94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9a94); }
 let mut pc: u32 = 0x30b9a94;
 loop { match pc {
 0x30b9a94 => {
  { let v: u32 = cpu.eax; cpu.w32(0x315599cu32, v); }
  { let v = (cpu.r32(0x315599cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x31559a0u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x31559a0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x31559a4u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x3155990u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x31559a0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x3155998u32, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9ac6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9ac6); }
 let mut pc: u32 = 0x30b9ac6;
 loop { match pc {
 0x30b9ac6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x31559b8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31559bcu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x31559c0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x31559acu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x31559b0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x31559b4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9aea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9aea); }
 let mut pc: u32 = 0x30b9aea;
 loop { match pc {
 0x30b9aea => {
  { let v = cpu.eax; cpu.push(v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x31545dcu32, v); }
  { let v: u16 = cpu.r16(0x31545dcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfcffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xf3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0xc00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xffc0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x31545deu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9b1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9b1a); }
 let mut pc: u32 = 0x30b9b1a;
 loop { match pc {
 0x30b9b1a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32cb394u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x32cb398u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x32cb39cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9b35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9b35); }
 let mut pc: u32 = 0x30b9b35;
 loop { match pc {
 0x30b9b35 => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9b40); }
 let mut pc: u32 = 0x30b9b40;
 loop { match pc {
 0x30b9b40 => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  if !cpu.cf { pc = 0x30b9b62; continue; }
  pc = 0x30b9b44; continue;
 }
 0x30b9b44 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffc0ffc0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1f001fu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30b9b62; continue;
 }
 0x30b9b62 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffc0ffc0u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1f001fu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b9b62; continue; }
  pc = 0x30b9b80; continue;
 }
 0x30b9b80 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9ba7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9ba7); }
 let mut pc: u32 = 0x30b9ba7;
 loop { match pc {
 0x30b9ba7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b9c7a; continue; }
  pc = 0x30b9bbc; continue;
 }
 0x30b9bbc => {
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x31547c0u32); cpu.ecx = v; }
  { let v: u32 = 0x325bebcu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a8u32, v); }
  pc = 0x30b9bd9; continue;
 }
 0x30b9bd9 => {
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
  { let v = cpu.r32(0x31547e8u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x31547e4u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x31547e0u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.sub32(cpu.r32(0x32560a8u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30b9bd9; continue; }
  pc = 0x30b9c7a; continue;
 }
 0x30b9c7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9c7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9c7b); }
 let mut pc: u32 = 0x30b9c7b;
 loop { match pc {
 0x30b9c7b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547ccu32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30b9cf9; continue; }
  pc = 0x30b9c8c; continue;
 }
 0x30b9c8c => {
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x31547c4u32); cpu.ecx = v; }
  { let v: u32 = 0x325e5d4u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30b9ca6); f_030ba2db(cpu); if cpu.returned_to != 0x30b9ca6 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30b9cac; continue;
 }
 0x30b9cac => {
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x315478cu32); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x3154790u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x3154794u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x30b9ce4; continue; }
  pc = 0x30b9ce2; continue;
 }
 0x30b9ce2 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30b9ce4; continue;
 }
 0x30b9ce4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x300u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x3fffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30b9cf4; continue; }
  pc = 0x30b9cef; continue;
 }
 0x30b9cef => {
  { let v: u32 = 0x3fffu32; cpu.eax = v; }
  pc = 0x30b9cf4; continue;
 }
 0x30b9cf4 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b9cac; continue; }
  pc = 0x30b9cf9; continue;
 }
 0x30b9cf9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9cfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9cfa); }
 let mut pc: u32 = 0x30b9cfa;
 loop { match pc {
 0x30b9cfa => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d8u32, v); }
  { let v: u32 = cpu.r32(0x31547c8u32); cpu.ecx = v; }
  { let v: u32 = 0x325eda8u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a8u32, v); }
  pc = 0x30b9d1c; continue;
 }
 0x30b9d1c => {
  { let v: u32 = 0x3154690u32; cpu.ebx = v; }
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
  { let v: u32 = cpu.eax; cpu.w32(0x31545f0u32, v); }
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
  { let v: u32 = cpu.eax; cpu.w32(0x31545f4u32, v); }
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
  { let v: u32 = cpu.eax; cpu.w32(0x31545f8u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x31545f0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31545f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x30b9da6); f_030baf3f(cpu); if cpu.returned_to != 0x30b9da6 { return; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x31545f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31545f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x30b9dc6); f_030baf3f(cpu); if cpu.returned_to != 0x30b9dc6 { return; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x32560a8u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30b9d1c; continue; }
  pc = 0x30b9de9; continue;
 }
 0x30b9de9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9dea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9dea); }
 let mut pc: u32 = 0x30b9dea;
 loop { match pc {
 0x30b9dea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b9deb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9deb); }
 let mut pc: u32 = 0x30b9deb;
 loop { match pc {
 0x30b9deb => {
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
pub fn f_030b9e6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9e6a); }
 let mut pc: u32 = 0x30b9e6a;
 loop { match pc {
 0x30b9e6a => {
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
pub fn f_030b9ed3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9ed3); }
 let mut pc: u32 = 0x30b9ed3;
 loop { match pc {
 0x30b9ed3 => {
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
pub fn f_030b9f3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9f3c); }
 let mut pc: u32 = 0x30b9f3c;
 loop { match pc {
 0x30b9f3c => {
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
pub fn f_030b9fa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b9fa9); }
 let mut pc: u32 = 0x30b9fa9;
 loop { match pc {
 0x30b9fa9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x325602cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256030u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256034u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256038u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x325603cu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256040u32, v); }
  { let v: u32 = 0x325602cu32; cpu.esi = v; }
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
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x325603cu32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30ba097; continue; }
  pc = 0x30ba059; continue;
 }
 0x30ba059 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x3256044u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3256038u32)); cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3256040u32)); cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x3155970u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30ba097 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ba09e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ba09e); }
 let mut pc: u32 = 0x30ba09e;
 loop { match pc {
 0x30ba09e => {
  { cpu.sub32(cpu.r32(0x31547dcu32), 0x3154624u32); }
  if cpu.zf { pc = 0x30ba1b4; continue; }
  pc = 0x30ba0ae; continue;
 }
 0x30ba0ae => {
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.eax = v; }
  { let v: u32 = 0x3256054u32; cpu.ebx = v; }
  cpu.push(0x30ba0bd); f_030bb14b(cpu); if cpu.returned_to != 0x30ba0bd { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d0u32, v); }
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x3256054u32; cpu.ebx = v; }
  { let v = (cpu.r32(0x31547e0u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3256048u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x31547e4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x325604cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x31547e8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3256050u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x315598cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547d0u32); cpu.ecx = v; }
  pc = 0x30ba114; continue;
 }
 0x30ba114 => {
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
  { let v = cpu.st(0) + cpu.rf32(0x3256048u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x325604cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x3256050u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30ba197; continue; }
  pc = 0x30ba173; continue;
 }
 0x30ba173 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155990u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3155978u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30ba197; continue;
 }
 0x30ba197 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x30ba114; continue; }
  pc = 0x30ba1b3; continue;
 }
 0x30ba1b3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30ba1b4 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d0u32, v); }
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x315460cu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256078u32, v); }
  { let v = (cpu.r32(0x3256078u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3154610u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3256078u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x325607cu32, v); }
  { let v = (cpu.r32(0x325607cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x3154614u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x325607cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x3256080u32, v); }
  { let v = (cpu.r32(0x3256080u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3256080u32, v); cpu.fpop(); }
  { let v: u32 = 0x3154648u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315598cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547d0u32); cpu.ecx = v; }
  pc = 0x30ba23b; continue;
 }
 0x30ba23b => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3256080u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x325607cu32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3256078u32); cpu.set_st(0, v); }
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
  if (cpu.sf != cpu.of) { pc = 0x30ba2be; continue; }
  pc = 0x30ba29a; continue;
 }
 0x30ba29a => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155990u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3155978u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30ba2be; continue;
 }
 0x30ba2be => {
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
  if !cpu.zf { cpu.poll(); pc = 0x30ba23b; continue; }
  pc = 0x30ba2da; continue;
 }
 0x30ba2da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ba2db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ba2db); }
 let mut pc: u32 = 0x30ba2db;
 loop { match pc {
 0x30ba2db => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a8u32, v); }
  pc = 0x30ba2e5; continue;
 }
 0x30ba2e5 => {
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
  { cpu.sub32(cpu.r32(0x32560a8u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30ba2e5; continue; }
  pc = 0x30ba380; continue;
 }
 0x30ba380 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ba759(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ba759); }
 let mut pc: u32 = 0x30ba759;
 loop { match pc {
 0x30ba759 => {
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
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
pub fn f_030ba793(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ba793); }
 let mut pc: u32 = 0x30ba793;
 loop { match pc {
 0x30ba793 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256090u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x325609cu32, v); }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256094u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a0u32, v); }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256098u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a4u32, v); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256094u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256094u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3256094u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030ba8f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30ba8f6); }
 let mut pc: u32 = 0x30ba8f6;
 loop { match pc {
 0x30ba8f6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256090u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x325609cu32, v); }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256094u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a0u32, v); }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3256098u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a4u32, v); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256094u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256094u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x3256098u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3256094u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x32560a0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030baa5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30baa5f); }
 let mut pc: u32 = 0x30baa5f;
 loop { match pc {
 0x30baa5f => {
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
pub fn f_030bab98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bab98); }
 let mut pc: u32 = 0x30bab98;
 loop { match pc {
 0x30bab98 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3256090u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x325609cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256090u32); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x325609cu32); cpu.eax = v; }
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
pub fn f_030bac74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bac74); }
 let mut pc: u32 = 0x30bac74;
 loop { match pc {
 0x30bac74 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3256094u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a0u32, v); }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256094u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a0u32); cpu.eax = v; }
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
pub fn f_030bad4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bad4f); }
 let mut pc: u32 = 0x30bad4f;
 loop { match pc {
 0x30bad4f => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3256098u32, v); }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a4u32, v); }
  { let v: u32 = cpu.r32(0x3256098u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a4u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256098u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a4u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256098u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a4u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256098u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a4u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256098u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a4u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x3256098u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32560a4u32); cpu.eax = v; }
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
pub fn f_030bae24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bae24); }
 let mut pc: u32 = 0x30bae24;
 loop { match pc {
 0x30bae24 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x30bae2a); f_030bae7d(cpu); if cpu.returned_to != 0x30bae2a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x30bae3a); f_030bae7d(cpu); if cpu.returned_to != 0x30bae3a { return; }
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
pub fn f_030bae7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bae7d); }
 let mut pc: u32 = 0x30bae7d;
 loop { match pc {
 0x30bae7d => {
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
  pc = 0x30bae98; continue;
 }
 0x30bae98 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x30baea3; continue; }
  pc = 0x30bae9c; continue;
 }
 0x30bae9c => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x30baead; continue; }
  pc = 0x30baea3; continue;
 }
 0x30baea3 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x30bae98; continue;
 }
 0x30baead => {
  cpu.push(0x30baeb2); f_030baf1c(cpu); if cpu.returned_to != 0x30baeb2 { return; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30baebf; continue; }
  pc = 0x30baeba; continue;
 }
 0x30baeba => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x30baebf; continue;
 }
 0x30baebf => {
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
pub fn f_030baee5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30baee5); }
 let mut pc: u32 = 0x30baee5;
 loop { match pc {
 0x30baee5 => {
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
  pc = 0x30baefe; continue;
 }
 0x30baefe => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x30baf09; continue; }
  pc = 0x30baf02; continue;
 }
 0x30baf02 => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x30baf13; continue; }
  pc = 0x30baf09; continue;
 }
 0x30baf09 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x30baefe; continue;
 }
 0x30baf13 => {
  cpu.push(0x30baf18); f_030baf1c(cpu); if cpu.returned_to != 0x30baf18 { return; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030baf1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30baf1c); }
 let mut pc: u32 = 0x30baf1c;
 loop { match pc {
 0x30baf1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x10000000u32; cpu.ebx = v; }
  pc = 0x30baf26; continue;
 }
 0x30baf26 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x30baf34; continue; }
  pc = 0x30baf30; continue;
 }
 0x30baf30 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  pc = 0x30baf34; continue;
 }
 0x30baf34 => {
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30baf26; continue; }
  pc = 0x30baf39; continue;
 }
 0x30baf39 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030baf3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30baf3f); }
 let mut pc: u32 = 0x30baf3f;
 loop { match pc {
 0x30baf3f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30baf4a; continue; }
  pc = 0x30baf48; continue;
 }
 0x30baf48 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30baf4a; continue;
 }
 0x30baf4a => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30baf50; continue; }
  pc = 0x30baf4e; continue;
 }
 0x30baf4e => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x30baf50; continue;
 }
 0x30baf50 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u8 = ((cpu.zf || cpu.sf != cpu.of)) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30baf58; continue; }
  pc = 0x30baf57; continue;
 }
 0x30baf57 => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x30baf58; continue;
 }
 0x30baf58 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30baf8b; continue; }
  pc = 0x30baf5c; continue;
 }
 0x30baf5c => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x326254cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x30baf7c; continue; }
  pc = 0x30baf75; continue;
 }
 0x30baf75 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x400u32); cpu.eax = v; }
  pc = 0x30baf7c; continue;
 }
 0x30baf7c => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.ebp); cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x30baf82; continue; }
  pc = 0x30baf80; continue;
 }
 0x30baf80 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30baf82; continue;
 }
 0x30baf82 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x30baf8b; continue; }
  pc = 0x30baf86; continue;
 }
 0x30baf86 => {
  { let v: u32 = cpu.eax.wrapping_add(0x800u32); cpu.eax = v; }
  pc = 0x30baf8b; continue;
 }
 0x30baf8b => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030baf92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30baf92); }
 let mut pc: u32 = 0x30baf92;
 loop { match pc {
 0x30baf92 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560acu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560b0u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560b4u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560b8u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560bcu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560c0u32, v); }
  { let v: u32 = cpu.r32(0x32560acu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560b0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560b4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30bafee); f_030baf1c(cpu); if cpu.returned_to != 0x30bafee { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32560acu32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32560acu32, v); }
  { let v: u32 = cpu.r32(0x32560b0u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32560b0u32, v); }
  { let v: u32 = cpu.r32(0x32560b4u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32560b4u32, v); }
  { let v: u32 = cpu.r32(0x32560b8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560bcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560c0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30bb05c); f_030baf1c(cpu); if cpu.returned_to != 0x30bb05c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32560b8u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32560b8u32, v); }
  { let v: u32 = cpu.r32(0x32560bcu32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32560bcu32, v); }
  { let v: u32 = cpu.r32(0x32560c0u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32560c0u32, v); }
  { let v: u32 = cpu.r32(0x32560b4u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560bcu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560b0u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560c0u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560c4u32, v); }
  { let v: u32 = cpu.r32(0x32560acu32); cpu.eax = v; }
  { let v = cpu.r32(0x32560c0u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560b4u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560b8u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560c8u32, v); }
  { let v: u32 = cpu.r32(0x32560b0u32); cpu.eax = v; }
  { let v = cpu.r32(0x32560b8u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32560acu32); cpu.eax = v; }
  { let v = cpu.r32(0x32560bcu32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560ccu32, v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32560acu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560b8u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560c4u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560b0u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560bcu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560c8u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560b4u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560c0u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x32560ccu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb14b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb14b); }
 let mut pc: u32 = 0x30bb14b;
 loop { match pc {
 0x30bb14b => {
  { let v = cpu.rf32(0x3154604u32); cpu.fpush(v); }
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
pub fn f_030bb19a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb19a); }
 let mut pc: u32 = 0x30bb19a;
 loop { match pc {
 0x30bb19a => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb1a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb1a8); }
 let mut pc: u32 = 0x30bb1a8;
 loop { match pc {
 0x30bb1a8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x325fd4cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
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
pub fn f_030bb1e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb1e5); }
 let mut pc: u32 = 0x30bb1e5;
 loop { match pc {
 0x30bb1e5 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb1f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb1f3); }
 let mut pc: u32 = 0x30bb1f3;
 loop { match pc {
 0x30bb1f3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x326054cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
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
pub fn f_030bb230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb230); }
 let mut pc: u32 = 0x30bb230;
 loop { match pc {
 0x30bb230 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547e0u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547e4u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547e8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3154800u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2eu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x30bb260; continue;
 }
 0x30bb260 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30bb27d; continue; }
  pc = 0x30bb267; continue;
 }
 0x30bb267 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x3154808u32.wrapping_add(cpu.ebx)); cpu.push(0x30bb276); cpu.call(t); if cpu.returned_to != 0x30bb276 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30bb260; continue;
 }
 0x30bb27d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb27f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb27f); }
 let mut pc: u32 = 0x30bb27f;
 loop { match pc {
 0x30bb27f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb28d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb28d); }
 let mut pc: u32 = 0x30bb28d;
 loop { match pc {
 0x30bb28d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
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
pub fn f_030bb2c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb2c0); }
 let mut pc: u32 = 0x30bb2c0;
 loop { match pc {
 0x30bb2c0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0x3154608u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x31558d4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x31558d8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x3155a00u32, v); }
  { let v: u32 = cpu.r32(0x31559f8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31559fcu32, v); }
  { cpu.sub32(cpu.r32(0x31554c4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30bb2fe; continue; }
  pc = 0x30bb2f9; continue;
 }
 0x30bb2f9 => {
  cpu.push(0x30bb2fe); f_030c21b5(cpu); if cpu.returned_to != 0x30bb2fe { return; }
  pc = 0x30bb2fe; continue;
 }
 0x30bb2fe => {
  { let v: u32 = 0x32a4de0u32; cpu.w32(0x32a4ddcu32, v); }
  { let v: u32 = 0x32bb434u32; cpu.w32(0x32bb430u32, v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x31545deu32); }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb394u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb398u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = 0x3154624u32; cpu.eax = v; }
  { let v: u32 = 0x3154648u32; cpu.ebx = v; }
  cpu.push(0x30bb348); f_030bb14b(cpu); if cpu.returned_to != 0x30bb348 { return; }
  { let v: u32 = cpu.r32(0x3154628u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3154634u32); cpu.ebx = v; }
  cpu.push(0x30bb358); f_030baf3f(cpu); if cpu.returned_to != 0x30bb358 { return; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x31546d8u32; cpu.edi = v; }
  cpu.push(0x30bb367); f_030bad4f(cpu); if cpu.returned_to != 0x30bb367 { return; }
  { let v: u32 = cpu.r32(0x315463cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3154644u32); cpu.ebx = v; }
  cpu.push(0x30bb377); f_030baf3f(cpu); if cpu.returned_to != 0x30bb377 { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x3154720u32; cpu.edi = v; }
  cpu.push(0x30bb388); f_030bac74(cpu); if cpu.returned_to != 0x30bb388 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb38a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb38a); }
 let mut pc: u32 = 0x30bb38a;
 loop { match pc {
 0x30bb38a => {
  { let v = cpu.ebp; cpu.push(v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x31545dcu32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb396(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb396); }
 let mut pc: u32 = 0x30bb396;
 loop { match pc {
 0x30bb396 => {
  { let v: u32 = 0x3262dacu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547acu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x31547b0u32, v); }
  { let v: u32 = 0x3264cecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x32727acu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547b4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x31547b8u32, v); }
  { let v: u32 = 0x32746ecu32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb3c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb3c1); }
 let mut pc: u32 = 0x30bb3c1;
 loop { match pc {
 0x30bb3c1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x31547acu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547b0u32); cpu.ebx = v; }
  cpu.push(0x30bb3d2); f_030bb872(cpu); if cpu.returned_to != 0x30bb3d2 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb3d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb3d4); }
 let mut pc: u32 = 0x30bb3d4;
 loop { match pc {
 0x30bb3d4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x31547b4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.ebx = v; }
  cpu.push(0x30bb3e5); f_030bb7a6(cpu); if cpu.returned_to != 0x30bb3e5 { return; }
  { let v: u32 = cpu.r32(0x31547b4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.ebx = v; }
  cpu.push(0x30bb3f5); f_030bb872(cpu); if cpu.returned_to != 0x30bb3f5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb3f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb3f7); }
 let mut pc: u32 = 0x30bb3f7;
 loop { match pc {
 0x30bb3f7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb41f; continue; }
  pc = 0x30bb3fb; continue;
 }
 0x30bb3fb => {
  { let v: u32 = cpu.ebx; cpu.w32(0x3262d74u32, v); }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3262d78u32, v); }
  pc = 0x30bb40a; continue;
 }
 0x30bb40a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x30bb410); f_030bb420(cpu); if cpu.returned_to != 0x30bb410 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3262d74u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x3262d78u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb40a; continue; }
  pc = 0x30bb41f; continue;
 }
 0x30bb41f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb420); }
 let mut pc: u32 = 0x30bb420;
 loop { match pc {
 0x30bb420 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x31547fcu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x315460cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x3154610u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x3154614u32)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x31559b8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb45d; continue;
 }
 0x30bb45d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x31559bcu32)); }
  if (cpu.sf != cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb46c; continue;
 }
 0x30bb46c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x31559c0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb47b; continue;
 }
 0x30bb47b => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x31559acu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb48c; continue;
 }
 0x30bb48c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x31559b0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb49b; continue;
 }
 0x30bb49b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x31559b4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb4aa; continue;
 }
 0x30bb4aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x3262d7cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3262d80u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3262d84u32, v); }
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.edx)), 0x1u16); }
  if cpu.zf { pc = 0x30bb4cf; continue; }
  pc = 0x30bb4c9; continue;
 }
 0x30bb4c9 => {
  { let v: u32 = cpu.r32(0x31547b0u32); cpu.ebx = v; }
  pc = 0x30bb4cf; continue;
 }
 0x30bb4cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154628u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d7cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154634u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d80u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154640u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d84u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x315599cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb51c; continue;
 }
 0x30bb51c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x315598cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb52d; continue;
 }
 0x30bb52d => {
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154624u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d7cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154630u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d80u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x315463cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d84u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x31559ccu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb57b; continue; }
  pc = 0x30bb579; continue;
 }
 0x30bb579 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30bb57b; continue;
 }
 0x30bb57b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x31559c4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb591; continue;
 }
 0x30bb591 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30bb5d3; continue; }
  pc = 0x30bb599; continue;
 }
 0x30bb599 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x315462cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d7cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154638u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d80u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154644u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d84u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bb641; continue;
 }
 0x30bb5d3 => {
  { let v: u32 = cpu.r32(0x315462cu32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d7cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154638u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d80u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3154644u32); cpu.eax = v; }
  { let v = cpu.r32(0x3262d84u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x31559d0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb61f; continue; }
  pc = 0x30bb61d; continue;
 }
 0x30bb61d => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30bb61f; continue;
 }
 0x30bb61f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x31559c8u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb635; continue;
 }
 0x30bb635 => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30bb641; continue; }
  pc = 0x30bb63d; continue;
 }
 0x30bb63d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bb641; continue;
 }
 0x30bb641 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x315599cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb65b; continue; }
  pc = 0x30bb64e; continue;
 }
 0x30bb64e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bb65f; continue; }
  pc = 0x30bb65b; continue;
 }
 0x30bb65b => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bb65f; continue;
 }
 0x30bb65f => {
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.eax = v; }
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
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebx)), 0x1u16); }
  if cpu.zf { pc = 0x30bb6b6; continue; }
  pc = 0x30bb69b; continue;
 }
 0x30bb69b => {
  { let v: u32 = cpu.r32(0x31547b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3264cecu32); }
  if (cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb6ac; continue;
 }
 0x30bb6ac => {
  { let v: u32 = cpu.ebx; cpu.w32(0x31547b0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30bb6cf; continue;
 }
 0x30bb6b6 => {
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x32746ecu32); }
  if (cpu.sf == cpu.of) { pc = 0x30bb6d6; continue; }
  pc = 0x30bb6c7; continue;
 }
 0x30bb6c7 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x31547b8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30bb6cf; continue;
 }
 0x30bb6cf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bb6d6 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb6da(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bb6da); } r_030bb6da(cpu, 0x30bb6da); }
pub fn f_030bb6e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bb6e3); } r_030bb6da(cpu, 0x30bb6e3); }
fn r_030bb6da(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bb6da => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb754; continue; }
  pc = 0x30bb6e3; continue;
 }
 0x30bb6e3 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb755; continue; }
  pc = 0x30bb6ec; continue;
 }
 0x30bb6ec => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x30bb6fb; continue;
 }
 0x30bb6fb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30bb713; continue; }
  pc = 0x30bb702; continue;
 }
 0x30bb702 => {
  if cpu.zf { pc = 0x30bb709; continue; }
  pc = 0x30bb704; continue;
 }
 0x30bb704 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x30bb6fb; continue;
 }
 0x30bb709 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x30bb713; continue; }
  pc = 0x30bb70e; continue;
 }
 0x30bb70e => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x30bb6fb; continue;
 }
 0x30bb713 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x30bb72b; continue; }
  pc = 0x30bb71a; continue;
 }
 0x30bb71a => {
  if cpu.zf { pc = 0x30bb721; continue; }
  pc = 0x30bb71c; continue;
 }
 0x30bb71c => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30bb713; continue;
 }
 0x30bb721 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if (cpu.cf || cpu.zf) { pc = 0x30bb72b; continue; }
  pc = 0x30bb726; continue;
 }
 0x30bb726 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30bb713; continue;
 }
 0x30bb72b => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb739; continue; }
  pc = 0x30bb72f; continue;
 }
 0x30bb72f => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x30bb739; continue;
 }
 0x30bb739 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bb6fb; continue; }
  pc = 0x30bb73d; continue;
 }
 0x30bb73d => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30bb74a; continue; }
  pc = 0x30bb741; continue;
 }
 0x30bb741 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x30bb749); f_030bb6e3(cpu); if cpu.returned_to != 0x30bb749 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x30bb74a; continue;
 }
 0x30bb74a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30bb754; continue; }
  pc = 0x30bb74f; continue;
 }
 0x30bb74f => {
  cpu.push(0x30bb754); f_030bb6e3(cpu); if cpu.returned_to != 0x30bb754 { return; }
  pc = 0x30bb754; continue;
 }
 0x30bb754 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bb755 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x30bb757; continue;
 }
 0x30bb757 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb77c; continue; }
  pc = 0x30bb769; continue;
 }
 0x30bb769 => {
  if cpu.zf { pc = 0x30bb771; continue; }
  pc = 0x30bb76b; continue;
 }
 0x30bb76b => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb757; continue; }
  pc = 0x30bb76f; continue;
 }
 0x30bb76f => {
  pc = 0x30bb7a5; continue;
 }
 0x30bb771 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30bb77c; continue; }
  pc = 0x30bb776; continue;
 }
 0x30bb776 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb757; continue; }
  pc = 0x30bb77a; continue;
 }
 0x30bb77a => {
  pc = 0x30bb7a5; continue;
 }
 0x30bb77c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x30bb785; continue;
 }
 0x30bb785 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30bb79d; continue; }
  pc = 0x30bb78c; continue;
 }
 0x30bb78c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bb785; continue; }
  pc = 0x30bb796; continue;
 }
 0x30bb796 => {
  if !cpu.zf { pc = 0x30bb79d; continue; }
  pc = 0x30bb798; continue;
 }
 0x30bb798 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { cpu.poll(); pc = 0x30bb785; continue; }
  pc = 0x30bb79d; continue;
 }
 0x30bb79d => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb757; continue; }
  pc = 0x30bb7a5; continue;
 }
 0x30bb7a5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb7a6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bb7a6); } r_030bb7a6(cpu, 0x30bb7a6); }
pub fn f_030bb7af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bb7af); } r_030bb7a6(cpu, 0x30bb7af); }
fn r_030bb7a6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bb7a6 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb820; continue; }
  pc = 0x30bb7af; continue;
 }
 0x30bb7af => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb821; continue; }
  pc = 0x30bb7b8; continue;
 }
 0x30bb7b8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x30bb7c7; continue;
 }
 0x30bb7c7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb7df; continue; }
  pc = 0x30bb7ce; continue;
 }
 0x30bb7ce => {
  if cpu.zf { pc = 0x30bb7d5; continue; }
  pc = 0x30bb7d0; continue;
 }
 0x30bb7d0 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x30bb7c7; continue;
 }
 0x30bb7d5 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if !cpu.cf { pc = 0x30bb7df; continue; }
  pc = 0x30bb7da; continue;
 }
 0x30bb7da => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x30bb7c7; continue;
 }
 0x30bb7df => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb7f7; continue; }
  pc = 0x30bb7e6; continue;
 }
 0x30bb7e6 => {
  if cpu.zf { pc = 0x30bb7ed; continue; }
  pc = 0x30bb7e8; continue;
 }
 0x30bb7e8 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30bb7df; continue;
 }
 0x30bb7ed => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if !cpu.cf { pc = 0x30bb7f7; continue; }
  pc = 0x30bb7f2; continue;
 }
 0x30bb7f2 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30bb7df; continue;
 }
 0x30bb7f7 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bb805; continue; }
  pc = 0x30bb7fb; continue;
 }
 0x30bb7fb => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x30bb805; continue;
 }
 0x30bb805 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bb7c7; continue; }
  pc = 0x30bb809; continue;
 }
 0x30bb809 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30bb816; continue; }
  pc = 0x30bb80d; continue;
 }
 0x30bb80d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x30bb815); f_030bb7af(cpu); if cpu.returned_to != 0x30bb815 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x30bb816; continue;
 }
 0x30bb816 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30bb820; continue; }
  pc = 0x30bb81b; continue;
 }
 0x30bb81b => {
  cpu.push(0x30bb820); f_030bb7af(cpu); if cpu.returned_to != 0x30bb820 { return; }
  pc = 0x30bb820; continue;
 }
 0x30bb820 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bb821 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x30bb823; continue;
 }
 0x30bb823 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30bb848; continue; }
  pc = 0x30bb835; continue;
 }
 0x30bb835 => {
  if cpu.zf { pc = 0x30bb83d; continue; }
  pc = 0x30bb837; continue;
 }
 0x30bb837 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb823; continue; }
  pc = 0x30bb83b; continue;
 }
 0x30bb83b => {
  pc = 0x30bb871; continue;
 }
 0x30bb83d => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { pc = 0x30bb848; continue; }
  pc = 0x30bb842; continue;
 }
 0x30bb842 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb823; continue; }
  pc = 0x30bb846; continue;
 }
 0x30bb846 => {
  pc = 0x30bb871; continue;
 }
 0x30bb848 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x30bb851; continue;
 }
 0x30bb851 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30bb869; continue; }
  pc = 0x30bb858; continue;
 }
 0x30bb858 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bb851; continue; }
  pc = 0x30bb862; continue;
 }
 0x30bb862 => {
  if !cpu.zf { pc = 0x30bb869; continue; }
  pc = 0x30bb864; continue;
 }
 0x30bb864 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x30bb851; continue; }
  pc = 0x30bb869; continue;
 }
 0x30bb869 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb823; continue; }
  pc = 0x30bb871; continue;
 }
 0x30bb871 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb872(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb872); }
 let mut pc: u32 = 0x30bb872;
 loop { match pc {
 0x30bb872 => {
  { cpu.sub32(cpu.r32(0x31554c4u32), 0x0u32); }
  if cpu.zf { pc = 0x30bb88b; continue; }
  pc = 0x30bb87b; continue;
 }
 0x30bb87b => {
  { cpu.sub8(cpu.r8(0x3154608u32), 0x0u8); }
  if !cpu.zf { pc = 0x30bb88b; continue; }
  pc = 0x30bb884; continue;
 }
 0x30bb884 => {
  { let v: u8 = 0x1u8; cpu.w8(0x3154608u32, v); }
  pc = 0x30bb88b; continue;
 }
 0x30bb88b => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x30bb955; continue; }
  pc = 0x30bb893; continue;
 }
 0x30bb893 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x31547ecu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547e0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31547e4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x31547e8u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547f0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31547f4u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x3154800u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30bb948; continue; }
  pc = 0x30bb8e0; continue;
 }
 0x30bb8e0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x31547fcu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x31547ecu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x31548a8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547a8u32, v); }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = 0x3296dc4u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x5dc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb91f; continue; }
  pc = 0x30bb90d; continue;
 }
 0x30bb90d => {
  { let v: u32 = 0x329adc8u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x13880u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bb91f; continue; }
  pc = 0x30bb91a; continue;
 }
 0x30bb91a => {
  { let v: u32 = 0x329edccu32; cpu.eax = v; }
  pc = 0x30bb91f; continue;
 }
 0x30bb91f => {
  { let v: u32 = cpu.eax; cpu.w32(0x3296dc0u32, v); }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x32u32); cpu.esi = v; }
  pc = 0x30bb92d; continue;
 }
 0x30bb92d => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30bb948; continue; }
  pc = 0x30bb937; continue;
 }
 0x30bb937 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x3154808u32.wrapping_add(cpu.ebx)); cpu.push(0x30bb942); cpu.call(t); if cpu.returned_to != 0x30bb942 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30bb92d; continue;
 }
 0x30bb948 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x30bb893; continue; }
  pc = 0x30bb955; continue;
 }
 0x30bb955 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb96a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb96a); }
 let mut pc: u32 = 0x30bb96a;
 loop { match pc {
 0x30bb96a => {
  { let v: u32 = cpu.add32(cpu.r32(0x31547a8u32), 0xb0u32); cpu.w32(0x31547a8u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x3154804u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb97f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb97f); }
 let mut pc: u32 = 0x30bb97f;
 loop { match pc {
 0x30bb97f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x328c5b0u32)); cpu.w32(0x328c5b0u32, v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  f_030bb987(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb987(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb987); }
 let mut pc: u32 = 0x30bb987;
 loop { match pc {
 0x30bb987 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30bb998; continue; }
  pc = 0x30bb991; continue;
 }
 0x30bb991 => {
  cpu.push(0x30bb996); f_030bb9e2(cpu); if cpu.returned_to != 0x30bb996 { return; }
  pc = 0x30bb99d; continue;
 }
 0x30bb998 => {
  cpu.push(0x30bb99d); f_030bb99e(cpu); if cpu.returned_to != 0x30bb99d { return; }
  pc = 0x30bb99d; continue;
 }
 0x30bb99d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb99e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb99e); }
 let mut pc: u32 = 0x30bb99e;
 loop { match pc {
 0x30bb99e => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30bb9be; continue; }
  pc = 0x30bb9ab; continue;
 }
 0x30bb9ab => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547c0u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x325beb8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30bb9cb; continue;
 }
 0x30bb9be => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547c0u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x325beb8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x30bb9cb; continue;
 }
 0x30bb9cb => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0xfffffffau32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x30bb9d5; continue; }
  pc = 0x30bb9d3; continue;
 }
 0x30bb9d3 => {
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x30bb9d5; continue;
 }
 0x30bb9d5 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x30bb9db); f_030bb987(cpu); if cpu.returned_to != 0x30bb9db { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x30bb9e1); f_030bb987(cpu); if cpu.returned_to != 0x30bb9e1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bb9e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bb9e2); }
 let mut pc: u32 = 0x30bb9e2;
 loop { match pc {
 0x30bb9e2 => {
  { let v: u32 = cpu.r32(0x32cb394u32).wrapping_add(cpu.eax); cpu.w32(0x32cb394u32, v); }
  pc = 0x30bb9e8; continue;
 }
 0x30bb9e8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bba06; continue; }
  pc = 0x30bb9f1; continue;
 }
 0x30bb9f1 => {
  if cpu.zf { pc = 0x30bba18; continue; }
  pc = 0x30bb9f3; continue;
 }
 0x30bb9f3 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547c0u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x325beb8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30bba13; continue;
 }
 0x30bba06 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547c0u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x325beb8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x30bba13; continue;
 }
 0x30bba13 => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x30bba30; continue; }
  pc = 0x30bba18; continue;
 }
 0x30bba18 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb398u32)); cpu.w32(0x32cb398u32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547a8u32)); cpu.ebx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.push(0x30bba2f); cpu.call(t); if cpu.returned_to != 0x30bba2f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x30bba30; continue;
 }
 0x30bba30 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bb9e8; continue; }
  pc = 0x30bba3c; continue;
 }
 0x30bba3c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bba3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bba3d); }
 let mut pc: u32 = 0x30bba3d;
 loop { match pc {
 0x30bba3d => {
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { let v: u32 = 0x31547a8u32; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.esp = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x31547b0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547acu32, v); }
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547b4u32, v); }
  { let v: u32 = cpu.r32(0x31547d0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547bcu32).wrapping_add(cpu.eax); cpu.w32(0x31547bcu32, v); }
  { let v: u32 = cpu.r32(0x31547d4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547c0u32).wrapping_add(cpu.eax); cpu.w32(0x31547c0u32, v); }
  { let v: u32 = cpu.r32(0x31547ccu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547c4u32).wrapping_add(cpu.eax); cpu.w32(0x31547c4u32, v); }
  pc = 0x30bba83; continue;
 }
 0x30bba83 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x30bba89); f_030bb420(cpu); if cpu.returned_to != 0x30bba89 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30bba83; continue; }
  pc = 0x30bba92; continue;
 }
 0x30bba92 => {
  { let v: u32 = cpu.r32(0x31547acu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547b0u32); cpu.ebx = v; }
  cpu.push(0x30bbaa2); f_030bb872(cpu); if cpu.returned_to != 0x30bbaa2 { return; }
  { let v: u32 = cpu.r32(0x31547b4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.ebx = v; }
  cpu.push(0x30bbab2); f_030bb7a6(cpu); if cpu.returned_to != 0x30bbab2 { return; }
  { let v: u32 = cpu.r32(0x31547b4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547b8u32); cpu.ebx = v; }
  cpu.push(0x30bbac2); f_030bb872(cpu); if cpu.returned_to != 0x30bbac2 { return; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x31547a8u32; cpu.edi = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bbad4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bbad4); }
 let mut pc: u32 = 0x30bbad4;
 loop { match pc {
 0x30bbad4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bbad5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bbad5); }
 let mut pc: u32 = 0x30bbad5;
 loop { match pc {
 0x30bbad5 => {
  cpu.push(0x30bbada); f_030bc08e(cpu); if cpu.returned_to != 0x30bbada { return; }
  if !cpu.zf { pc = 0x30bc085; continue; }
  pc = 0x30bbae0; continue;
 }
 0x30bbae0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bc085 => {
  { let v: u32 = 0x3154a14u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bc08e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bc08e); }
 let mut pc: u32 = 0x30bc08e;
 loop { match pc {
 0x30bc08e => {
  { let v: u32 = 0x80000001u32; cpu.eax = v; }
  { let v: u32 = 0x7fffffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32821d0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32821d8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32821e4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc0e3); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc0e3 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc103; continue; }
  pc = 0x30bc0fd; continue;
 }
 0x30bc0fd => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc103; continue;
 }
 0x30bc103 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc111; continue; }
  pc = 0x30bc10b; continue;
 }
 0x30bc10b => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc111; continue;
 }
 0x30bc111 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc11f; continue; }
  pc = 0x30bc119; continue;
 }
 0x30bc119 => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc11f; continue;
 }
 0x30bc11f => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc13a; continue; }
  pc = 0x30bc135; continue;
 }
 0x30bc135 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc13a; continue;
 }
 0x30bc13a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc147; continue; }
  pc = 0x30bc142; continue;
 }
 0x30bc142 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc147; continue;
 }
 0x30bc147 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc166; continue; }
  pc = 0x30bc161; continue;
 }
 0x30bc161 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc166; continue;
 }
 0x30bc166 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc173; continue; }
  pc = 0x30bc16e; continue;
 }
 0x30bc16e => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc173; continue;
 }
 0x30bc173 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc194); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc194 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc1b4; continue; }
  pc = 0x30bc1ae; continue;
 }
 0x30bc1ae => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc1b4; continue;
 }
 0x30bc1b4 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc1c2; continue; }
  pc = 0x30bc1bc; continue;
 }
 0x30bc1bc => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc1c2; continue;
 }
 0x30bc1c2 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc1d0; continue; }
  pc = 0x30bc1ca; continue;
 }
 0x30bc1ca => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc1d0; continue;
 }
 0x30bc1d0 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc1eb; continue; }
  pc = 0x30bc1e6; continue;
 }
 0x30bc1e6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc1eb; continue;
 }
 0x30bc1eb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc1f8; continue; }
  pc = 0x30bc1f3; continue;
 }
 0x30bc1f3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc1f8; continue;
 }
 0x30bc1f8 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc217; continue; }
  pc = 0x30bc212; continue;
 }
 0x30bc212 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc217; continue;
 }
 0x30bc217 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc224; continue; }
  pc = 0x30bc21f; continue;
 }
 0x30bc21f => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc224; continue;
 }
 0x30bc224 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc245); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc245 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc265; continue; }
  pc = 0x30bc25f; continue;
 }
 0x30bc25f => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc265; continue;
 }
 0x30bc265 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc273; continue; }
  pc = 0x30bc26d; continue;
 }
 0x30bc26d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc273; continue;
 }
 0x30bc273 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc281; continue; }
  pc = 0x30bc27b; continue;
 }
 0x30bc27b => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc281; continue;
 }
 0x30bc281 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc29c; continue; }
  pc = 0x30bc297; continue;
 }
 0x30bc297 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc29c; continue;
 }
 0x30bc29c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc2a9; continue; }
  pc = 0x30bc2a4; continue;
 }
 0x30bc2a4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc2a9; continue;
 }
 0x30bc2a9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc2c8; continue; }
  pc = 0x30bc2c3; continue;
 }
 0x30bc2c3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc2c8; continue;
 }
 0x30bc2c8 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc2d5; continue; }
  pc = 0x30bc2d0; continue;
 }
 0x30bc2d0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc2d5; continue;
 }
 0x30bc2d5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc2f6); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc2f6 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc316; continue; }
  pc = 0x30bc310; continue;
 }
 0x30bc310 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc316; continue;
 }
 0x30bc316 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc324; continue; }
  pc = 0x30bc31e; continue;
 }
 0x30bc31e => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc324; continue;
 }
 0x30bc324 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc332; continue; }
  pc = 0x30bc32c; continue;
 }
 0x30bc32c => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc332; continue;
 }
 0x30bc332 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc34d; continue; }
  pc = 0x30bc348; continue;
 }
 0x30bc348 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc34d; continue;
 }
 0x30bc34d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc35a; continue; }
  pc = 0x30bc355; continue;
 }
 0x30bc355 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc35a; continue;
 }
 0x30bc35a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc379; continue; }
  pc = 0x30bc374; continue;
 }
 0x30bc374 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc379; continue;
 }
 0x30bc379 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc386; continue; }
  pc = 0x30bc381; continue;
 }
 0x30bc381 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc386; continue;
 }
 0x30bc386 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc3a7); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc3a7 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc3c7; continue; }
  pc = 0x30bc3c1; continue;
 }
 0x30bc3c1 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc3c7; continue;
 }
 0x30bc3c7 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc3d5; continue; }
  pc = 0x30bc3cf; continue;
 }
 0x30bc3cf => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc3d5; continue;
 }
 0x30bc3d5 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc3e3; continue; }
  pc = 0x30bc3dd; continue;
 }
 0x30bc3dd => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc3e3; continue;
 }
 0x30bc3e3 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc3fe; continue; }
  pc = 0x30bc3f9; continue;
 }
 0x30bc3f9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc3fe; continue;
 }
 0x30bc3fe => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc40b; continue; }
  pc = 0x30bc406; continue;
 }
 0x30bc406 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc40b; continue;
 }
 0x30bc40b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc42a; continue; }
  pc = 0x30bc425; continue;
 }
 0x30bc425 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc42a; continue;
 }
 0x30bc42a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc437; continue; }
  pc = 0x30bc432; continue;
 }
 0x30bc432 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc437; continue;
 }
 0x30bc437 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc458); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc458 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc478; continue; }
  pc = 0x30bc472; continue;
 }
 0x30bc472 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc478; continue;
 }
 0x30bc478 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc486; continue; }
  pc = 0x30bc480; continue;
 }
 0x30bc480 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc486; continue;
 }
 0x30bc486 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc494; continue; }
  pc = 0x30bc48e; continue;
 }
 0x30bc48e => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc494; continue;
 }
 0x30bc494 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc4af; continue; }
  pc = 0x30bc4aa; continue;
 }
 0x30bc4aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc4af; continue;
 }
 0x30bc4af => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc4bc; continue; }
  pc = 0x30bc4b7; continue;
 }
 0x30bc4b7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc4bc; continue;
 }
 0x30bc4bc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc4db; continue; }
  pc = 0x30bc4d6; continue;
 }
 0x30bc4d6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc4db; continue;
 }
 0x30bc4db => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc4e8; continue; }
  pc = 0x30bc4e3; continue;
 }
 0x30bc4e3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc4e8; continue;
 }
 0x30bc4e8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc509); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc509 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc529; continue; }
  pc = 0x30bc523; continue;
 }
 0x30bc523 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc529; continue;
 }
 0x30bc529 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc537; continue; }
  pc = 0x30bc531; continue;
 }
 0x30bc531 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc537; continue;
 }
 0x30bc537 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc545; continue; }
  pc = 0x30bc53f; continue;
 }
 0x30bc53f => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc545; continue;
 }
 0x30bc545 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc560; continue; }
  pc = 0x30bc55b; continue;
 }
 0x30bc55b => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc560; continue;
 }
 0x30bc560 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc56d; continue; }
  pc = 0x30bc568; continue;
 }
 0x30bc568 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc56d; continue;
 }
 0x30bc56d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc58c; continue; }
  pc = 0x30bc587; continue;
 }
 0x30bc587 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc58c; continue;
 }
 0x30bc58c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc599; continue; }
  pc = 0x30bc594; continue;
 }
 0x30bc594 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc599; continue;
 }
 0x30bc599 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  cpu.push(0x30bc5ba); f_030b9e6a(cpu); if cpu.returned_to != 0x30bc5ba { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc5da; continue; }
  pc = 0x30bc5d4; continue;
 }
 0x30bc5d4 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e0u32, v); }
  pc = 0x30bc5da; continue;
 }
 0x30bc5da => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32821e4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc5e8; continue; }
  pc = 0x30bc5e2; continue;
 }
 0x30bc5e2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32821e4u32, v); }
  pc = 0x30bc5e8; continue;
 }
 0x30bc5e8 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc5f6; continue; }
  pc = 0x30bc5f0; continue;
 }
 0x30bc5f0 => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.ebx = v; }
  pc = 0x30bc5f6; continue;
 }
 0x30bc5f6 => {
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x315596cu32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc611; continue; }
  pc = 0x30bc60c; continue;
 }
 0x30bc60c => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d0u32, v); }
  pc = 0x30bc611; continue;
 }
 0x30bc611 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc61e; continue; }
  pc = 0x30bc619; continue;
 }
 0x30bc619 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d4u32, v); }
  pc = 0x30bc61e; continue;
 }
 0x30bc61e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x315598cu32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3155970u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x32821d8u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc63d; continue; }
  pc = 0x30bc638; continue;
 }
 0x30bc638 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821d8u32, v); }
  pc = 0x30bc63d; continue;
 }
 0x30bc63d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x32821dcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc64a; continue; }
  pc = 0x30bc645; continue;
 }
 0x30bc645 => {
  { let v: u32 = cpu.eax; cpu.w32(0x32821dcu32, v); }
  pc = 0x30bc64a; continue;
 }
 0x30bc64a => {
  { let v: u32 = cpu.r32(0x32821e4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x315598cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x30bc68b; continue; }
  pc = 0x30bc657; continue;
 }
 0x30bc657 => {
  { let v: u32 = cpu.r32(0x32821d0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155964u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bc68b; continue; }
  pc = 0x30bc664; continue;
 }
 0x30bc664 => {
  { let v: u32 = cpu.r32(0x32821d8u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155968u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bc68b; continue; }
  pc = 0x30bc672; continue;
 }
 0x30bc672 => {
  { let v: u32 = cpu.r32(0x32821d4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30bc68b; continue; }
  pc = 0x30bc67b; continue;
 }
 0x30bc67b => {
  { let v: u32 = cpu.r32(0x32821dcu32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30bc68b; continue; }
  pc = 0x30bc685; continue;
 }
 0x30bc685 => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bc68b => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bc6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bc6a0); }
 let mut pc: u32 = 0x30bc6a0;
 loop { match pc {
 0x30bc6a0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bc7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bc7a0); }
 let mut pc: u32 = 0x30bc7a0;
 loop { match pc {
 0x30bc7a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30bc7a9; continue;
 }
 0x30bc7a9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bc7a9; continue; }
  pc = 0x30bc7b7; continue;
 }
 0x30bc7b7 => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x32c178cu32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bc7cb; continue;
 }
 0x30bc7cb => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x32c6b94u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bc7ec; continue; }
  pc = 0x30bc7df; continue;
 }
 0x30bc7df => {
  if cpu.zf { pc = 0x30bc948; continue; }
  pc = 0x30bc7e5; continue;
 }
 0x30bc7e5 => {
  { let v: u32 = 0x32c1794u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x30bc7ec; continue;
 }
 0x30bc7ec => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821e8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821ecu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32821ecu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32821e8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x32c1790u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc824; continue; }
  pc = 0x30bc81e; continue;
 }
 0x30bc81e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bc824; continue;
 }
 0x30bc824 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x32c178cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bc832; continue; }
  pc = 0x30bc82c; continue;
 }
 0x30bc82c => {
  { let v: u32 = cpu.edx; cpu.w32(0x32c178cu32, v); }
  pc = 0x30bc832; continue;
 }
 0x30bc832 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc948; continue; }
  pc = 0x30bc83a; continue;
 }
 0x30bc83a => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3154ac4u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32c1424u32, v); }
  { let v = (cpu.r32(0x32821e8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32c1424u32), 0x32c6b94u32); }
  if (cpu.sf != cpu.of) { pc = 0x30bc87e; continue; }
  pc = 0x30bc87c; continue;
 }
 0x30bc87c => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30bc87e; continue;
 }
 0x30bc87e => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32821f0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f4u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ecx = v; }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bc92a; continue; }
  pc = 0x30bc8c9; continue;
 }
 0x30bc8c9 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bc8c9; continue; }
  pc = 0x30bc92a; continue;
 }
 0x30bc92a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x30bc92d; continue;
 }
 0x30bc92d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bc92d; continue; }
  pc = 0x30bc941; continue;
 }
 0x30bc941 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x30bc948; continue;
 }
 0x30bc948 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bc7cb; continue; }
  pc = 0x30bc954; continue;
 }
 0x30bc954 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bc955(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bc955); }
 let mut pc: u32 = 0x30bc955;
 loop { match pc {
 0x30bc955 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x7fffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x30bc965; continue;
 }
 0x30bc965 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bc972; continue; }
  pc = 0x30bc96f; continue;
 }
 0x30bc96f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x30bc972; continue;
 }
 0x30bc972 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x30bc97a; continue; }
  pc = 0x30bc977; continue;
 }
 0x30bc977 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x30bc97a; continue;
 }
 0x30bc97a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bc965; continue; }
  pc = 0x30bc991; continue;
 }
 0x30bc991 => {
  { let v: u32 = cpu.edx; cpu.w32(0x328223cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x3282240u32, v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.rf32(0x328223cu32); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3154ac8u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3282240u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x328223cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = ((!cpu.cf && !cpu.zf)) as u8; cpu.w8(0x3154accu32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x7fffu32; cpu.w32(0x32c178cu32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bc9d9; continue;
 }
 0x30bc9d9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x32c6b94u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bc9fa; continue; }
  pc = 0x30bc9ed; continue;
 }
 0x30bc9ed => {
  if cpu.zf { pc = 0x30bcd3d; continue; }
  pc = 0x30bc9f3; continue;
 }
 0x30bc9f3 => {
  { let v: u32 = 0x32c1794u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x30bc9fa; continue;
 }
 0x30bc9fa => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821e8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821ecu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32821ecu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32821e8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x32c1790u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bca32; continue; }
  pc = 0x30bca2c; continue;
 }
 0x30bca2c => {
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bca32; continue;
 }
 0x30bca32 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x32c178cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bca40; continue; }
  pc = 0x30bca3a; continue;
 }
 0x30bca3a => {
  { let v: u32 = cpu.edx; cpu.w32(0x32c178cu32, v); }
  pc = 0x30bca40; continue;
 }
 0x30bca40 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bcd3d; continue; }
  pc = 0x30bca48; continue;
 }
 0x30bca48 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3154ac4u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32c1424u32, v); }
  { let v = (cpu.r32(0x32821e8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x3282234u32, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f8u32, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x3282208u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821fcu32, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x328220cu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32c1424u32), 0x32c6b94u32); }
  if (cpu.sf != cpu.of) { pc = 0x30bcabe; continue; }
  pc = 0x30bcab9; continue;
 }
 0x30bcab9 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  pc = 0x30bcabe; continue;
 }
 0x30bcabe => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32821f0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bcb6e; continue; }
  pc = 0x30bcb0d; continue;
 }
 0x30bcb0d => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bcb0d; continue; }
  pc = 0x30bcb6e; continue;
 }
 0x30bcb6e => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x30bcb71; continue;
 }
 0x30bcb71 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcb71; continue; }
  pc = 0x30bcb85; continue;
 }
 0x30bcb85 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf32(0x328220cu32); cpu.fpush(v); }
  { let v = cpu.rf32(0x3282208u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x3154ac4u32); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bcbaf; continue; }
  pc = 0x30bcbad; continue;
 }
 0x30bcbad => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30bcbaf; continue;
 }
 0x30bcbaf => {
  { cpu.sub8(cpu.r8(0x3154accu32), 0x0u8); }
  if cpu.zf { pc = 0x30bcbc1; continue; }
  pc = 0x30bcbb8; continue;
 }
 0x30bcbb8 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30bcbea; continue;
 }
 0x30bcbc1 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x30000u32); }
  if (cpu.sf == cpu.of) { pc = 0x30bcbea; continue; }
  pc = 0x30bcbcf; continue;
 }
 0x30bcbcf => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x10000u32); }
  if (cpu.sf == cpu.of) { pc = 0x30bcbea; continue; }
  pc = 0x30bcbe0; continue;
 }
 0x30bcbe0 => {
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x30bcbea; continue;
 }
 0x30bcbea => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if (cpu.sf != cpu.of) { pc = 0x30bcbf4; continue; }
  pc = 0x30bcbf0; continue;
 }
 0x30bcbf0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x30bcbf4; continue;
 }
 0x30bcbf4 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x328222cu32, v); }
  { let v: u32 = cpu.r32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x3282228u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3282230u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x3154ac0u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x3282234u32); cpu.fpush(v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32821fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282200u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282204u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3282200u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x3282204u32)); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x3282228u32); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3282228u32); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282200u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282204u32, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x328222cu32)); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bccfa; continue; }
  pc = 0x30bcc96; continue;
 }
 0x30bcc96 => {
  { let v = cpu.st(0) + cpu.rf32(0x3282228u32); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x3282228u32); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3282230u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3282200u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3282204u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x328222cu32); cpu.ecx = v; }
  pc = 0x30bccc6; continue;
 }
 0x30bccc6 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bccc6; continue; }
  pc = 0x30bccdc; continue;
 }
 0x30bccdc => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282200u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282204u32, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x328222cu32)); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bcc96; continue; }
  pc = 0x30bccfa; continue;
 }
 0x30bccfa => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x328222cu32)); cpu.ebp = v; }
  if cpu.zf { pc = 0x30bcd2e; continue; }
  pc = 0x30bcd02; continue;
 }
 0x30bcd02 => {
  { let v: u32 = cpu.r32(0x3282230u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x3282200u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3282204u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  pc = 0x30bcd18; continue;
 }
 0x30bcd18 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcd18; continue; }
  pc = 0x30bcd2e; continue;
 }
 0x30bcd2e => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x30bcd3d; continue;
 }
 0x30bcd3d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bc9d9; continue; }
  pc = 0x30bcd49; continue;
 }
 0x30bcd49 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bcd4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bcd4a); }
 let mut pc: u32 = 0x30bcd4a;
 loop { match pc {
 0x30bcd4a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30bcd53; continue;
 }
 0x30bcd53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcd53; continue; }
  pc = 0x30bcd6f; continue;
 }
 0x30bcd6f => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x32c178cu32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bcd83; continue;
 }
 0x30bcd83 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x32c6b94u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bcda4; continue; }
  pc = 0x30bcd97; continue;
 }
 0x30bcd97 => {
  if cpu.zf { pc = 0x30bcfe8; continue; }
  pc = 0x30bcd9d; continue;
 }
 0x30bcd9d => {
  { let v: u32 = 0x32c1794u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x30bcda4; continue;
 }
 0x30bcda4 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821e8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821ecu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32821ecu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32821e8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x32c1790u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bcddc; continue; }
  pc = 0x30bcdd6; continue;
 }
 0x30bcdd6 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bcddc; continue;
 }
 0x30bcddc => {
  { cpu.sub32(cpu.edx, cpu.r32(0x32c178cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bcdea; continue; }
  pc = 0x30bcde4; continue;
 }
 0x30bcde4 => {
  { let v: u32 = cpu.edx; cpu.w32(0x32c178cu32, v); }
  pc = 0x30bcdea; continue;
 }
 0x30bcdea => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bcfe8; continue; }
  pc = 0x30bcdf2; continue;
 }
 0x30bcdf2 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3154ac4u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32c1424u32, v); }
  { let v = (cpu.r32(0x32821e8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f8u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821f8u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282200u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3282200u32); cpu.ecx = v; }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821fcu32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282204u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3282204u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bcee0; continue; }
  pc = 0x30bce78; continue;
 }
 0x30bce78 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bce78; continue; }
  pc = 0x30bcee0; continue;
 }
 0x30bcee0 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x30bcee3; continue;
 }
 0x30bcee3 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcee3; continue; }
  pc = 0x30bcef9; continue;
 }
 0x30bcef9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32c1424u32), 0x32c6b94u32); }
  if (cpu.sf != cpu.of) { pc = 0x30bcf19; continue; }
  pc = 0x30bcf17; continue;
 }
 0x30bcf17 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30bcf19; continue;
 }
 0x30bcf19 => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32821f0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bcfc9; continue; }
  pc = 0x30bcf68; continue;
 }
 0x30bcf68 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bcf68; continue; }
  pc = 0x30bcfc9; continue;
 }
 0x30bcfc9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x30bcfcc; continue;
 }
 0x30bcfcc => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcfcc; continue; }
  pc = 0x30bcfe0; continue;
 }
 0x30bcfe0 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x30bcfe8; continue;
 }
 0x30bcfe8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcd83; continue; }
  pc = 0x30bcff4; continue;
 }
 0x30bcff4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bcff5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bcff5); }
 let mut pc: u32 = 0x30bcff5;
 loop { match pc {
 0x30bcff5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30bcffe; continue;
 }
 0x30bcffe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4000000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bcffe; continue; }
  pc = 0x30bd013; continue;
 }
 0x30bd013 => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x32c178cu32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bd027; continue;
 }
 0x30bd027 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x32c6b94u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd048; continue; }
  pc = 0x30bd03b; continue;
 }
 0x30bd03b => {
  if cpu.zf { pc = 0x30bd222; continue; }
  pc = 0x30bd041; continue;
 }
 0x30bd041 => {
  { let v: u32 = 0x32c1794u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x30bd048; continue;
 }
 0x30bd048 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821e8u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821ecu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32821ecu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32821e8u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x32c1790u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bd080; continue; }
  pc = 0x30bd07a; continue;
 }
 0x30bd07a => {
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1790u32, v); }
  pc = 0x30bd080; continue;
 }
 0x30bd080 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x32c178cu32)); }
  if (cpu.sf == cpu.of) { pc = 0x30bd08e; continue; }
  pc = 0x30bd088; continue;
 }
 0x30bd088 => {
  { let v: u32 = cpu.edx; cpu.w32(0x32c178cu32, v); }
  pc = 0x30bd08e; continue;
 }
 0x30bd08e => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bd222; continue; }
  pc = 0x30bd096; continue;
 }
 0x30bd096 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3154ac4u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32c1424u32, v); }
  { let v = (cpu.r32(0x32821e8u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154abcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f8u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821f8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x3282200u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3282200u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bd12a; continue; }
  pc = 0x30bd0f2; continue;
 }
 0x30bd0f2 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bd0f2; continue; }
  pc = 0x30bd12a; continue;
 }
 0x30bd12a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x30bd12d; continue;
 }
 0x30bd12d => {
  { let v: u32 = cpu.eax; cpu.w32(0x2400u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bd12d; continue; }
  pc = 0x30bd13b; continue;
 }
 0x30bd13b => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32c1424u32), 0x32c6b94u32); }
  if (cpu.sf != cpu.of) { pc = 0x30bd158; continue; }
  pc = 0x30bd156; continue;
 }
 0x30bd156 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x30bd158; continue;
 }
 0x30bd158 => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f0u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32821f0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x32821f4u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32821f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1424u32); cpu.edi = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bd204; continue; }
  pc = 0x30bd1a3; continue;
 }
 0x30bd1a3 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bd1a3; continue; }
  pc = 0x30bd204; continue;
 }
 0x30bd204 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x30bd207; continue;
 }
 0x30bd207 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bd207; continue; }
  pc = 0x30bd21b; continue;
 }
 0x30bd21b => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x30bd222; continue;
 }
 0x30bd222 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30bd027; continue; }
  pc = 0x30bd22e; continue;
 }
 0x30bd22e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bd22f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30bd22f); }
 let mut pc: u32 = 0x30bd22f;
 loop { match pc {
 0x30bd22f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bd2cd; continue; }
  pc = 0x30bd249; continue;
 }
 0x30bd249 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3a4u32, v); }
  pc = 0x30bd278; continue;
 }
 0x30bd278 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30bd27e); f_030c2285(cpu); if cpu.returned_to != 0x30bd27e { return; }
  if cpu.zf { pc = 0x30bd2bb; continue; }
  pc = 0x30bd280; continue;
 }
 0x30bd280 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.eax = v; }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  pc = 0x30bd293; continue;
 }
 0x30bd293 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x30bd2b3; continue; }
  pc = 0x30bd29c; continue;
 }
 0x30bd29c => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30bd2a9; continue; }
  pc = 0x30bd2a6; continue;
 }
 0x30bd2a6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, false); }
  pc = 0x30bd2a9; continue;
 }
 0x30bd2a9 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x30bd293; continue; }
  pc = 0x30bd2af; continue;
 }
 0x30bd2af => {
  { cpu.stos(2, false); }
  cpu.poll(); pc = 0x30bd293; continue;
 }
 0x30bd2b3 => {
  if cpu.zf { pc = 0x30bd2bb; continue; }
  pc = 0x30bd2b5; continue;
 }
 0x30bd2b5 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x30bd293; continue;
 }
 0x30bd2bb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bd278; continue; }
  pc = 0x30bd2cd; continue;
 }
 0x30bd2cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bd2ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd2ce); } r_030bd2ce(cpu, 0x30bd2ce); }
pub fn f_030bd3d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd3d2); } r_030bd2ce(cpu, 0x30bd3d2); }
pub fn f_030bd3e2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd3e2); } r_030bd2ce(cpu, 0x30bd3e2); }
pub fn f_030bd3f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd3f2); } r_030bd2ce(cpu, 0x30bd3f2); }
pub fn f_030bd402(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd402); } r_030bd2ce(cpu, 0x30bd402); }
pub fn f_030bd425(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd425); } r_030bd2ce(cpu, 0x30bd425); }
pub fn f_030bd435(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd435); } r_030bd2ce(cpu, 0x30bd435); }
pub fn f_030bd445(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd445); } r_030bd2ce(cpu, 0x30bd445); }
pub fn f_030bd455(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd455); } r_030bd2ce(cpu, 0x30bd455); }
fn r_030bd2ce(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bd2ce => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bd48d; continue; }
  pc = 0x30bd2e8; continue;
 }
 0x30bd2e8 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7bef7befu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  pc = 0x30bd31d; continue;
 }
 0x30bd31d => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30bd323); f_030c2af8(cpu); if cpu.returned_to != 0x30bd323 { return; }
  if cpu.zf { pc = 0x30bd477; continue; }
  pc = 0x30bd329; continue;
 }
 0x30bd329 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.ebx = v; }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd464; continue; }
  pc = 0x30bd347; continue;
 }
 0x30bd347 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30bd364; continue; }
  pc = 0x30bd34f; continue;
 }
 0x30bd34f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30bd364; continue;
 }
 0x30bd364 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd3c1; continue; }
  pc = 0x30bd37a; continue;
 }
 0x30bd37a => {
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
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bd37a; continue; }
  pc = 0x30bd3c1; continue;
 }
 0x30bd3c1 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154ad0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bd3d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bd3e2; continue;
 }
 0x30bd3e2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bd3f2; continue;
 }
 0x30bd3f2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bd402; continue;
 }
 0x30bd402 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bd347; continue; }
  pc = 0x30bd421; continue;
 }
 0x30bd421 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd464; continue; }
  pc = 0x30bd423; continue;
 }
 0x30bd423 => {
  if cpu.zf { pc = 0x30bd477; continue; }
  pc = 0x30bd425; continue;
 }
 0x30bd425 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bd435; continue;
 }
 0x30bd435 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bd445; continue;
 }
 0x30bd445 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bd455; continue;
 }
 0x30bd455 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bd347; continue; }
  pc = 0x30bd462; continue;
 }
 0x30bd462 => {
  if cpu.zf { pc = 0x30bd477; continue; }
  pc = 0x30bd464; continue;
 }
 0x30bd464 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bd347; continue; }
  pc = 0x30bd475; continue;
 }
 0x30bd475 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bd464; continue; }
  pc = 0x30bd477; continue;
 }
 0x30bd477 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bd31d; continue; }
  pc = 0x30bd48d; continue;
 }
 0x30bd48d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bd48e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd48e); } r_030bd48e(cpu, 0x30bd48e); }
pub fn f_030bd9ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bd9ed); } r_030bd48e(cpu, 0x30bd9ed); }
pub fn f_030bda0d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bda0d); } r_030bd48e(cpu, 0x30bda0d); }
pub fn f_030bda2d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bda2d); } r_030bd48e(cpu, 0x30bda2d); }
pub fn f_030bda4d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bda4d); } r_030bd48e(cpu, 0x30bda4d); }
pub fn f_030bda6d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bda6d); } r_030bd48e(cpu, 0x30bda6d); }
pub fn f_030bda8d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bda8d); } r_030bd48e(cpu, 0x30bda8d); }
pub fn f_030bdaad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdaad); } r_030bd48e(cpu, 0x30bdaad); }
pub fn f_030bdacd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdacd); } r_030bd48e(cpu, 0x30bdacd); }
pub fn f_030bdafd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdafd); } r_030bd48e(cpu, 0x30bdafd); }
pub fn f_030bdb1d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdb1d); } r_030bd48e(cpu, 0x30bdb1d); }
pub fn f_030bdb3d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdb3d); } r_030bd48e(cpu, 0x30bdb3d); }
pub fn f_030bdb5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdb5d); } r_030bd48e(cpu, 0x30bdb5d); }
pub fn f_030bdb7d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdb7d); } r_030bd48e(cpu, 0x30bdb7d); }
pub fn f_030bdb9d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdb9d); } r_030bd48e(cpu, 0x30bdb9d); }
pub fn f_030bdbbd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdbbd); } r_030bd48e(cpu, 0x30bdbbd); }
pub fn f_030bdbdd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdbdd); } r_030bd48e(cpu, 0x30bdbdd); }
pub fn f_030bdbfd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bdbfd); } r_030bd48e(cpu, 0x30bdbfd); }
fn r_030bd48e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bd48e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30be070; continue; }
  pc = 0x30bd4a8; continue;
 }
 0x30bd4a8 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30bd4ce; continue;
 }
 0x30bd4ce => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30bd4d5); f_030c2285(cpu); if cpu.returned_to != 0x30bd4d5 { return; }
  if cpu.zf { pc = 0x30be059; continue; }
  pc = 0x30bd4db; continue;
 }
 0x30bd4db => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30bd4f1; continue; }
  pc = 0x30bd4e4; continue;
 }
 0x30bd4e4 => {
  cpu.push(0x30bd4e9); f_030c3525(cpu); if cpu.returned_to != 0x30bd4e9 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30be059; continue; }
  pc = 0x30bd4f1; continue;
 }
 0x30bd4f1 => {
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
  { let v: u32 = cpu.eax; cpu.w32(0x32c1460u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c1468u32, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = (cpu.r32(0x32c1460u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x32c1468u32) as i32 as f64); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf32(0x3154ac4u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3154ac0u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x3154ac0u32); cpu.fpush(v); }
  { let v: u32 = 0x328c28cu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd5f3; continue; }
  pc = 0x30bd570; continue;
 }
 0x30bd570 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x30bd57c; continue; }
  pc = 0x30bd577; continue;
 }
 0x30bd577 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x30bd57c; continue;
 }
 0x30bd57c => {
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3b4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154af0u32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c28cu32); cpu.eax = v; }
  pc = 0x30bd666; continue;
 }
 0x30bd5f3 => {
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3b4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154af0u32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c28cu32); cpu.eax = v; }
  pc = 0x30bdfaf; continue;
 }
 0x30bd666 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x30bdc1b; continue; }
  pc = 0x30bd67c; continue;
 }
 0x30bd67c => {
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd7e4; continue; }
  pc = 0x30bd68b; continue;
 }
 0x30bd68b => {
  { let v = cpu.st(0) + cpu.rf32(0x32a2e18u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2e18u32); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) * cpu.rf32(0x32a3e18u32); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bd68b; continue; }
  pc = 0x30bd7e4; continue;
 }
 0x30bd7e4 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bd94c; continue; }
  pc = 0x30bd7f3; continue;
 }
 0x30bd7f3 => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x30bd94c; continue;
 }
 0x30bd94c => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30bd9c1; continue; }
  pc = 0x30bd95a; continue;
 }
 0x30bd95a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154af4u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bd9c1 => {
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154af4u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bd9ed => {
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
  pc = 0x30bda0d; continue;
 }
 0x30bda0d => {
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
  pc = 0x30bda2d; continue;
 }
 0x30bda2d => {
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
  pc = 0x30bda4d; continue;
 }
 0x30bda4d => {
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
  pc = 0x30bda6d; continue;
 }
 0x30bda6d => {
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
  pc = 0x30bda8d; continue;
 }
 0x30bda8d => {
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
  pc = 0x30bdaad; continue;
 }
 0x30bdaad => {
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
  pc = 0x30bdacd; continue;
 }
 0x30bdacd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30be04d; continue; }
  pc = 0x30bdaee; continue;
 }
 0x30bdaee => {
  { let v: u32 = cpu.r32(0x3154af0u32).wrapping_add(0x4u32); cpu.w32(0x3154af0u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30bdfaf; continue;
 }
 0x30bdafd => {
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
  pc = 0x30bdb1d; continue;
 }
 0x30bdb1d => {
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
  pc = 0x30bdb3d; continue;
 }
 0x30bdb3d => {
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
  pc = 0x30bdb5d; continue;
 }
 0x30bdb5d => {
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
  pc = 0x30bdb7d; continue;
 }
 0x30bdb7d => {
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
  pc = 0x30bdb9d; continue;
 }
 0x30bdb9d => {
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
  pc = 0x30bdbbd; continue;
 }
 0x30bdbbd => {
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
  pc = 0x30bdbdd; continue;
 }
 0x30bdbdd => {
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
  pc = 0x30bdbfd; continue;
 }
 0x30bdbfd => {
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30be04d; continue; }
  pc = 0x30bdc0c; continue;
 }
 0x30bdc0c => {
  { let v: u32 = cpu.r32(0x3154af0u32).wrapping_add(0x4u32); cpu.w32(0x3154af0u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30bdfaf; continue;
 }
 0x30bdc1b => {
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bdd85; continue; }
  pc = 0x30bdc2a; continue;
 }
 0x30bdc2a => {
  { let v = cpu.st(0) + cpu.rf32(0x32a2e18u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2e18u32); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) * cpu.rf32(0x32a3e18u32); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bdc2a; continue; }
  pc = 0x30bdd85; continue;
 }
 0x30bdd85 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bdeef; continue; }
  pc = 0x30bdd94; continue;
 }
 0x30bdd94 => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x30bdeef; continue;
 }
 0x30bdeef => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30bdf64; continue; }
  pc = 0x30bdefd; continue;
 }
 0x30bdefd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
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
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154af4u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bdf64 => {
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x30bdbfd; continue; }
  pc = 0x30bdf8f; continue;
 }
 0x30bdf8f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154af4u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bdfaf => {
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bdfc3; continue; }
  pc = 0x30bdfbe; continue;
 }
 0x30bdfbe => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x30bdfc3; continue;
 }
 0x30bdfc3 => {
  { let v = cpu.st(0) + cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3154af0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30be04d; continue; }
  pc = 0x30be03e; continue;
 }
 0x30be03e => {
  { let v: u32 = cpu.r32(0x3154af0u32).wrapping_add(0x4u32); cpu.w32(0x3154af0u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x30bd666; continue;
 }
 0x30be04d => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x30be059; continue;
 }
 0x30be059 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bd4ce; continue; }
  pc = 0x30be070; continue;
 }
 0x30be070 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030be071(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be071); } r_030be071(cpu, 0x30be071); }
pub fn f_030be31e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be31e); } r_030be071(cpu, 0x30be31e); }
pub fn f_030be346(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be346); } r_030be071(cpu, 0x30be346); }
pub fn f_030be36e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be36e); } r_030be071(cpu, 0x30be36e); }
pub fn f_030be396(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be396); } r_030be071(cpu, 0x30be396); }
pub fn f_030be3be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be3be); } r_030be071(cpu, 0x30be3be); }
pub fn f_030be3e6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be3e6); } r_030be071(cpu, 0x30be3e6); }
pub fn f_030be40e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be40e); } r_030be071(cpu, 0x30be40e); }
pub fn f_030be436(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be436); } r_030be071(cpu, 0x30be436); }
pub fn f_030be465(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be465); } r_030be071(cpu, 0x30be465); }
pub fn f_030be48d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be48d); } r_030be071(cpu, 0x30be48d); }
pub fn f_030be4b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be4b5); } r_030be071(cpu, 0x30be4b5); }
pub fn f_030be4dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be4dd); } r_030be071(cpu, 0x30be4dd); }
pub fn f_030be505(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be505); } r_030be071(cpu, 0x30be505); }
pub fn f_030be52d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be52d); } r_030be071(cpu, 0x30be52d); }
pub fn f_030be555(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be555); } r_030be071(cpu, 0x30be555); }
pub fn f_030be57d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be57d); } r_030be071(cpu, 0x30be57d); }
pub fn f_030be5d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be5d8); } r_030be071(cpu, 0x30be5d8); }
pub fn f_030be5e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be5e4); } r_030be071(cpu, 0x30be5e4); }
pub fn f_030be5f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be5f0); } r_030be071(cpu, 0x30be5f0); }
pub fn f_030be5fc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be5fc); } r_030be071(cpu, 0x30be5fc); }
pub fn f_030be608(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be608); } r_030be071(cpu, 0x30be608); }
pub fn f_030be614(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be614); } r_030be071(cpu, 0x30be614); }
pub fn f_030be620(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be620); } r_030be071(cpu, 0x30be620); }
pub fn f_030be62c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be62c); } r_030be071(cpu, 0x30be62c); }
pub fn f_030be638(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be638); } r_030be071(cpu, 0x30be638); }
pub fn f_030be644(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be644); } r_030be071(cpu, 0x30be644); }
pub fn f_030be650(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be650); } r_030be071(cpu, 0x30be650); }
pub fn f_030be65c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be65c); } r_030be071(cpu, 0x30be65c); }
pub fn f_030be668(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be668); } r_030be071(cpu, 0x30be668); }
pub fn f_030be674(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be674); } r_030be071(cpu, 0x30be674); }
pub fn f_030be680(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be680); } r_030be071(cpu, 0x30be680); }
pub fn f_030be68c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be68c); } r_030be071(cpu, 0x30be68c); }
fn r_030be071(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30be071 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30be6bc; continue; }
  pc = 0x30be08b; continue;
 }
 0x30be08b => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30be0b1; continue;
 }
 0x30be0b1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30be0b8); f_030c2285(cpu); if cpu.returned_to != 0x30be0b8 { return; }
  if cpu.zf { pc = 0x30be6a5; continue; }
  pc = 0x30be0be; continue;
 }
 0x30be0be => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30be0d4; continue; }
  pc = 0x30be0c7; continue;
 }
 0x30be0c7 => {
  cpu.push(0x30be0cc); f_030c3525(cpu); if cpu.returned_to != 0x30be0cc { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30be6a5; continue; }
  pc = 0x30be0d4; continue;
 }
 0x30be0d4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1460u32, v); }
  { let v = (cpu.r32(0x32c1460u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3b4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1468u32, v); }
  { let v = (cpu.r32(0x32c1468u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c1444u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30be590; continue; }
  pc = 0x30be193; continue;
 }
 0x30be193 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30be1b4; continue; }
  pc = 0x30be19b; continue;
 }
 0x30be19b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30be1b4; continue;
 }
 0x30be1b4 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30be30d; continue; }
  pc = 0x30be1ce; continue;
 }
 0x30be1ce => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be1ce; continue; }
  pc = 0x30be30d; continue;
 }
 0x30be30d => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154b38u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30be31e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be346; continue;
 }
 0x30be346 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be36e; continue;
 }
 0x30be36e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be396; continue;
 }
 0x30be396 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be3be; continue;
 }
 0x30be3be => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be3e6; continue;
 }
 0x30be3e6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be40e; continue;
 }
 0x30be40e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be436; continue;
 }
 0x30be436 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be193; continue; }
  pc = 0x30be459; continue;
 }
 0x30be459 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30be590; continue; }
  pc = 0x30be45f; continue;
 }
 0x30be45f => {
  if cpu.zf { pc = 0x30be6a5; continue; }
  pc = 0x30be465; continue;
 }
 0x30be465 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be48d; continue;
 }
 0x30be48d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be4b5; continue;
 }
 0x30be4b5 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be4dd; continue;
 }
 0x30be4dd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be505; continue;
 }
 0x30be505 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be52d; continue;
 }
 0x30be52d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be555; continue;
 }
 0x30be555 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30be57d; continue;
 }
 0x30be57d => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be193; continue; }
  pc = 0x30be58a; continue;
 }
 0x30be58a => {
  if cpu.zf { pc = 0x30be6a5; continue; }
  pc = 0x30be590; continue;
 }
 0x30be590 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30be5c7; continue; }
  pc = 0x30be5a4; continue;
 }
 0x30be5a4 => {
  { let v: u32 = cpu.r32(0x32c1444u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x30be5b1; continue;
 }
 0x30be5b1 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be5b1; continue; }
  pc = 0x30be5c4; continue;
 }
 0x30be5c4 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x30be5c7; continue;
 }
 0x30be5c7 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154b78u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30be5d8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be5e4; continue;
 }
 0x30be5e4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be5f0; continue;
 }
 0x30be5f0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be5fc; continue;
 }
 0x30be5fc => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be608; continue;
 }
 0x30be608 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be614; continue;
 }
 0x30be614 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be620; continue;
 }
 0x30be620 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be62c; continue;
 }
 0x30be62c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be638; continue;
 }
 0x30be638 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be644; continue;
 }
 0x30be644 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be650; continue;
 }
 0x30be650 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be65c; continue;
 }
 0x30be65c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be668; continue;
 }
 0x30be668 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be674; continue;
 }
 0x30be674 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be680; continue;
 }
 0x30be680 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30be68c; continue;
 }
 0x30be68c => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be193; continue; }
  pc = 0x30be69f; continue;
 }
 0x30be69f => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30be590; continue; }
  pc = 0x30be6a5; continue;
 }
 0x30be6a5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30be0b1; continue; }
  pc = 0x30be6bc; continue;
 }
 0x30be6bc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030be6bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30be6bd); }
 let mut pc: u32 = 0x30be6bd;
 loop { match pc {
 0x30be6bd => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30be765; continue; }
  pc = 0x30be6d7; continue;
 }
 0x30be6d7 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3a4u32, v); }
  pc = 0x30be706; continue;
 }
 0x30be706 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30be70c); f_030c3007(cpu); if cpu.returned_to != 0x30be70c { return; }
  if cpu.zf { pc = 0x30be753; continue; }
  pc = 0x30be70e; continue;
 }
 0x30be70e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.eax = v; }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  pc = 0x30be721; continue;
 }
 0x30be721 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x30be74b; continue; }
  pc = 0x30be72a; continue;
 }
 0x30be72a => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30be736; continue; }
  pc = 0x30be731; continue;
 }
 0x30be731 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.poll(); pc = 0x30be721; continue;
 }
 0x30be736 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30be741; continue; }
  pc = 0x30be73e; continue;
 }
 0x30be73e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, false); }
  pc = 0x30be741; continue;
 }
 0x30be741 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x30be721; continue; }
  pc = 0x30be747; continue;
 }
 0x30be747 => {
  { cpu.stos(2, false); }
  cpu.poll(); pc = 0x30be721; continue;
 }
 0x30be74b => {
  if cpu.zf { pc = 0x30be753; continue; }
  pc = 0x30be74d; continue;
 }
 0x30be74d => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x30be721; continue;
 }
 0x30be753 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30be706; continue; }
  pc = 0x30be765; continue;
 }
 0x30be765 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030be766(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30be766); } r_030be766(cpu, 0x30be766); }
pub fn f_030bea20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bea20); } r_030be766(cpu, 0x30bea20); }
pub fn f_030bea48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bea48); } r_030be766(cpu, 0x30bea48); }
pub fn f_030bea70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bea70); } r_030be766(cpu, 0x30bea70); }
pub fn f_030bea98(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bea98); } r_030be766(cpu, 0x30bea98); }
pub fn f_030beac0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30beac0); } r_030be766(cpu, 0x30beac0); }
pub fn f_030beae8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30beae8); } r_030be766(cpu, 0x30beae8); }
pub fn f_030beb10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30beb10); } r_030be766(cpu, 0x30beb10); }
pub fn f_030beb38(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30beb38); } r_030be766(cpu, 0x30beb38); }
pub fn f_030beb67(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30beb67); } r_030be766(cpu, 0x30beb67); }
pub fn f_030beb8f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30beb8f); } r_030be766(cpu, 0x30beb8f); }
pub fn f_030bebb7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bebb7); } r_030be766(cpu, 0x30bebb7); }
pub fn f_030bebdf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bebdf); } r_030be766(cpu, 0x30bebdf); }
pub fn f_030bec07(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bec07); } r_030be766(cpu, 0x30bec07); }
pub fn f_030bec2f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bec2f); } r_030be766(cpu, 0x30bec2f); }
pub fn f_030bec57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bec57); } r_030be766(cpu, 0x30bec57); }
pub fn f_030bec7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bec7f); } r_030be766(cpu, 0x30bec7f); }
pub fn f_030becda(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30becda); } r_030be766(cpu, 0x30becda); }
pub fn f_030bece6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bece6); } r_030be766(cpu, 0x30bece6); }
pub fn f_030becf2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30becf2); } r_030be766(cpu, 0x30becf2); }
pub fn f_030becfe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30becfe); } r_030be766(cpu, 0x30becfe); }
pub fn f_030bed0a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed0a); } r_030be766(cpu, 0x30bed0a); }
pub fn f_030bed16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed16); } r_030be766(cpu, 0x30bed16); }
pub fn f_030bed22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed22); } r_030be766(cpu, 0x30bed22); }
pub fn f_030bed2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed2e); } r_030be766(cpu, 0x30bed2e); }
pub fn f_030bed3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed3a); } r_030be766(cpu, 0x30bed3a); }
pub fn f_030bed46(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed46); } r_030be766(cpu, 0x30bed46); }
pub fn f_030bed52(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed52); } r_030be766(cpu, 0x30bed52); }
pub fn f_030bed5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed5e); } r_030be766(cpu, 0x30bed5e); }
pub fn f_030bed6a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed6a); } r_030be766(cpu, 0x30bed6a); }
pub fn f_030bed76(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed76); } r_030be766(cpu, 0x30bed76); }
pub fn f_030bed82(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed82); } r_030be766(cpu, 0x30bed82); }
pub fn f_030bed8e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bed8e); } r_030be766(cpu, 0x30bed8e); }
fn r_030be766(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30be766 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bedbe; continue; }
  pc = 0x30be780; continue;
 }
 0x30be780 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30be7a6; continue;
 }
 0x30be7a6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30be7ad); f_030c3007(cpu); if cpu.returned_to != 0x30be7ad { return; }
  if cpu.zf { pc = 0x30beda7; continue; }
  pc = 0x30be7b3; continue;
 }
 0x30be7b3 => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30be7c9; continue; }
  pc = 0x30be7bc; continue;
 }
 0x30be7bc => {
  cpu.push(0x30be7c1); f_030c3525(cpu); if cpu.returned_to != 0x30be7c1 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30beda7; continue; }
  pc = 0x30be7c9; continue;
 }
 0x30be7c9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1460u32, v); }
  { let v = (cpu.r32(0x32c1460u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3b4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1468u32, v); }
  { let v = (cpu.r32(0x32c1468u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c1444u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bec92; continue; }
  pc = 0x30be888; continue;
 }
 0x30be888 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if (cpu.sf != cpu.of) { pc = 0x30be895; continue; }
  pc = 0x30be88d; continue;
 }
 0x30be88d => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bec7f; continue;
 }
 0x30be895 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30be8b6; continue; }
  pc = 0x30be89d; continue;
 }
 0x30be89d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30be8b6; continue;
 }
 0x30be8b6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bea0f; continue; }
  pc = 0x30be8d0; continue;
 }
 0x30be8d0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be8d0; continue; }
  pc = 0x30bea0f; continue;
 }
 0x30bea0f => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154bb8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bea20 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bea48; continue;
 }
 0x30bea48 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bea70; continue;
 }
 0x30bea70 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bea98; continue;
 }
 0x30bea98 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30beac0; continue;
 }
 0x30beac0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30beae8; continue;
 }
 0x30beae8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30beb10; continue;
 }
 0x30beb10 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30beb38; continue;
 }
 0x30beb38 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be888; continue; }
  pc = 0x30beb5b; continue;
 }
 0x30beb5b => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bec92; continue; }
  pc = 0x30beb61; continue;
 }
 0x30beb61 => {
  if cpu.zf { pc = 0x30beda7; continue; }
  pc = 0x30beb67; continue;
 }
 0x30beb67 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30beb8f; continue;
 }
 0x30beb8f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bebb7; continue;
 }
 0x30bebb7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bebdf; continue;
 }
 0x30bebdf => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bec07; continue;
 }
 0x30bec07 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bec2f; continue;
 }
 0x30bec2f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bec57; continue;
 }
 0x30bec57 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30bec7f; continue;
 }
 0x30bec7f => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be888; continue; }
  pc = 0x30bec8c; continue;
 }
 0x30bec8c => {
  if cpu.zf { pc = 0x30beda7; continue; }
  pc = 0x30bec92; continue;
 }
 0x30bec92 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30becc9; continue; }
  pc = 0x30beca6; continue;
 }
 0x30beca6 => {
  { let v: u32 = cpu.r32(0x32c1444u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x30becb3; continue;
 }
 0x30becb3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30becb3; continue; }
  pc = 0x30becc6; continue;
 }
 0x30becc6 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x30becc9; continue;
 }
 0x30becc9 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154bf8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30becda => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bece6; continue;
 }
 0x30bece6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30becf2; continue;
 }
 0x30becf2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30becfe; continue;
 }
 0x30becfe => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed0a; continue;
 }
 0x30bed0a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed16; continue;
 }
 0x30bed16 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed22; continue;
 }
 0x30bed22 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed2e; continue;
 }
 0x30bed2e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed3a; continue;
 }
 0x30bed3a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed46; continue;
 }
 0x30bed46 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed52; continue;
 }
 0x30bed52 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed5e; continue;
 }
 0x30bed5e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed6a; continue;
 }
 0x30bed6a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed76; continue;
 }
 0x30bed76 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed82; continue;
 }
 0x30bed82 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bed8e; continue;
 }
 0x30bed8e => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30be888; continue; }
  pc = 0x30beda1; continue;
 }
 0x30beda1 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bec92; continue; }
  pc = 0x30beda7; continue;
 }
 0x30beda7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30be7a6; continue; }
  pc = 0x30bedbe; continue;
 }
 0x30bedbe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
