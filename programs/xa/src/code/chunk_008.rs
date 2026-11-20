#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_030bedbf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bedbf); } r_030bedbf(cpu, 0x30bedbf); }
pub fn f_030bf068(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf068); } r_030bedbf(cpu, 0x30bf068); }
pub fn f_030bf090(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf090); } r_030bedbf(cpu, 0x30bf090); }
pub fn f_030bf0b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf0b8); } r_030bedbf(cpu, 0x30bf0b8); }
pub fn f_030bf0e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf0e0); } r_030bedbf(cpu, 0x30bf0e0); }
pub fn f_030bf108(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf108); } r_030bedbf(cpu, 0x30bf108); }
pub fn f_030bf130(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf130); } r_030bedbf(cpu, 0x30bf130); }
pub fn f_030bf158(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf158); } r_030bedbf(cpu, 0x30bf158); }
pub fn f_030bf180(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf180); } r_030bedbf(cpu, 0x30bf180); }
pub fn f_030bf19b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf19b); } r_030bedbf(cpu, 0x30bf19b); }
pub fn f_030bf1c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf1c3); } r_030bedbf(cpu, 0x30bf1c3); }
pub fn f_030bf1eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf1eb); } r_030bedbf(cpu, 0x30bf1eb); }
pub fn f_030bf213(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf213); } r_030bedbf(cpu, 0x30bf213); }
pub fn f_030bf23b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf23b); } r_030bedbf(cpu, 0x30bf23b); }
pub fn f_030bf263(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf263); } r_030bedbf(cpu, 0x30bf263); }
pub fn f_030bf28b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf28b); } r_030bedbf(cpu, 0x30bf28b); }
pub fn f_030bf2b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf2b3); } r_030bedbf(cpu, 0x30bf2b3); }
fn r_030bedbf(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bedbf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bf2ca; continue; }
  pc = 0x30bedd9; continue;
 }
 0x30bedd9 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30bedff; continue;
 }
 0x30bedff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30bee17; continue; }
  pc = 0x30bee0a; continue;
 }
 0x30bee0a => {
  cpu.push(0x30bee0f); f_030c3525(cpu); if cpu.returned_to != 0x30bee0f { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30bf2b3; continue; }
  pc = 0x30bee17; continue;
 }
 0x30bee17 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bf2b3; continue; }
  pc = 0x30bee32; continue;
 }
 0x30bee32 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x3154c38u32, v); }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1460u32, v); }
  { let v = (cpu.r32(0x32c1460u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
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
  { let v: u32 = cpu.r32(0x3154c38u32); cpu.edx = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30beefe; continue; }
  pc = 0x30beee1; continue;
 }
 0x30beee1 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bf2b3; continue; }
  pc = 0x30beefe; continue;
 }
 0x30beefe => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf057; continue; }
  pc = 0x30bef18; continue;
 }
 0x30bef18 => {
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
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bef18; continue; }
  pc = 0x30bf057; continue;
 }
 0x30bf057 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154c3cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bf068 => {
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
  pc = 0x30bf090; continue;
 }
 0x30bf090 => {
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
  pc = 0x30bf0b8; continue;
 }
 0x30bf0b8 => {
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
  pc = 0x30bf0e0; continue;
 }
 0x30bf0e0 => {
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
  pc = 0x30bf108; continue;
 }
 0x30bf108 => {
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
  pc = 0x30bf130; continue;
 }
 0x30bf130 => {
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
  pc = 0x30bf158; continue;
 }
 0x30bf158 => {
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
  pc = 0x30bf180; continue;
 }
 0x30bf180 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf2b3; continue;
 }
 0x30bf19b => {
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
  pc = 0x30bf1c3; continue;
 }
 0x30bf1c3 => {
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
  pc = 0x30bf1eb; continue;
 }
 0x30bf1eb => {
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
  pc = 0x30bf213; continue;
 }
 0x30bf213 => {
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
  pc = 0x30bf23b; continue;
 }
 0x30bf23b => {
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
  pc = 0x30bf263; continue;
 }
 0x30bf263 => {
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
  pc = 0x30bf28b; continue;
 }
 0x30bf28b => {
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
  pc = 0x30bf2b3; continue;
 }
 0x30bf2b3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bedff; continue; }
  pc = 0x30bf2ca; continue;
 }
 0x30bf2ca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bf2cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf2cb); } r_030bf2cb(cpu, 0x30bf2cb); }
pub fn f_030bf409(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf409); } r_030bf2cb(cpu, 0x30bf409); }
pub fn f_030bf41d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf41d); } r_030bf2cb(cpu, 0x30bf41d); }
pub fn f_030bf431(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf431); } r_030bf2cb(cpu, 0x30bf431); }
pub fn f_030bf445(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf445); } r_030bf2cb(cpu, 0x30bf445); }
pub fn f_030bf464(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf464); } r_030bf2cb(cpu, 0x30bf464); }
pub fn f_030bf478(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf478); } r_030bf2cb(cpu, 0x30bf478); }
pub fn f_030bf48c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf48c); } r_030bf2cb(cpu, 0x30bf48c); }
pub fn f_030bf4a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf4a0); } r_030bf2cb(cpu, 0x30bf4a0); }
pub fn f_030bf4ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf4ef); } r_030bf2cb(cpu, 0x30bf4ef); }
pub fn f_030bf4f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf4f3); } r_030bf2cb(cpu, 0x30bf4f3); }
pub fn f_030bf4f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf4f7); } r_030bf2cb(cpu, 0x30bf4f7); }
pub fn f_030bf4fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf4fb); } r_030bf2cb(cpu, 0x30bf4fb); }
pub fn f_030bf4ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf4ff); } r_030bf2cb(cpu, 0x30bf4ff); }
pub fn f_030bf503(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf503); } r_030bf2cb(cpu, 0x30bf503); }
pub fn f_030bf507(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf507); } r_030bf2cb(cpu, 0x30bf507); }
pub fn f_030bf50b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf50b); } r_030bf2cb(cpu, 0x30bf50b); }
fn r_030bf2cb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bf2cb => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bf537; continue; }
  pc = 0x30bf2e5; continue;
 }
 0x30bf2e5 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  pc = 0x30bf301; continue;
 }
 0x30bf301 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30bf308); f_030c2285(cpu); if cpu.returned_to != 0x30bf308 { return; }
  if cpu.zf { pc = 0x30bf520; continue; }
  pc = 0x30bf30e; continue;
 }
 0x30bf30e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x7800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2400u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cb3b0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x2400u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf4af; continue; }
  pc = 0x30bf36a; continue;
 }
 0x30bf36a => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30bf37f; continue; }
  pc = 0x30bf372; continue;
 }
 0x30bf372 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30bf37f; continue;
 }
 0x30bf37f => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf3f8; continue; }
  pc = 0x30bf395; continue;
 }
 0x30bf395 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf395; continue; }
  pc = 0x30bf3f8; continue;
 }
 0x30bf3f8 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154c7cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bf409 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x30bf41d; continue;
 }
 0x30bf41d => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x30bf431; continue;
 }
 0x30bf431 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x30bf445; continue;
 }
 0x30bf445 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf36a; continue; }
  pc = 0x30bf45c; continue;
 }
 0x30bf45c => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf4af; continue; }
  pc = 0x30bf45e; continue;
 }
 0x30bf45e => {
  if cpu.zf { pc = 0x30bf520; continue; }
  pc = 0x30bf464; continue;
 }
 0x30bf464 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x30bf478; continue;
 }
 0x30bf478 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x30bf48c; continue;
 }
 0x30bf48c => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x30bf4a0; continue;
 }
 0x30bf4a0 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf36a; continue; }
  pc = 0x30bf4ad; continue;
 }
 0x30bf4ad => {
  if cpu.zf { pc = 0x30bf520; continue; }
  pc = 0x30bf4af; continue;
 }
 0x30bf4af => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf4de; continue; }
  pc = 0x30bf4c3; continue;
 }
 0x30bf4c3 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x30bf4cc; continue;
 }
 0x30bf4cc => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf4cc; continue; }
  pc = 0x30bf4db; continue;
 }
 0x30bf4db => {
  { let v: u32 = cpu.ror32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x30bf4de; continue;
 }
 0x30bf4de => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154c9cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bf4ef => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf4f3; continue;
 }
 0x30bf4f3 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf4f7; continue;
 }
 0x30bf4f7 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf4fb; continue;
 }
 0x30bf4fb => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf4ff; continue;
 }
 0x30bf4ff => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf503; continue;
 }
 0x30bf503 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf507; continue;
 }
 0x30bf507 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x30bf50b; continue;
 }
 0x30bf50b => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf36a; continue; }
  pc = 0x30bf51e; continue;
 }
 0x30bf51e => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bf4af; continue; }
  pc = 0x30bf520; continue;
 }
 0x30bf520 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bf301; continue; }
  pc = 0x30bf537; continue;
 }
 0x30bf537 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bf538(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf538); } r_030bf538(cpu, 0x30bf538); }
pub fn f_030bf75c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf75c); } r_030bf538(cpu, 0x30bf75c); }
pub fn f_030bf777(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf777); } r_030bf538(cpu, 0x30bf777); }
pub fn f_030bf792(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf792); } r_030bf538(cpu, 0x30bf792); }
pub fn f_030bf7ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf7ad); } r_030bf538(cpu, 0x30bf7ad); }
pub fn f_030bf7c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf7c8); } r_030bf538(cpu, 0x30bf7c8); }
pub fn f_030bf7e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf7e3); } r_030bf538(cpu, 0x30bf7e3); }
pub fn f_030bf7fe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf7fe); } r_030bf538(cpu, 0x30bf7fe); }
pub fn f_030bf819(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf819); } r_030bf538(cpu, 0x30bf819); }
pub fn f_030bf874(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf874); } r_030bf538(cpu, 0x30bf874); }
pub fn f_030bf880(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf880); } r_030bf538(cpu, 0x30bf880); }
pub fn f_030bf88c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf88c); } r_030bf538(cpu, 0x30bf88c); }
pub fn f_030bf898(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf898); } r_030bf538(cpu, 0x30bf898); }
pub fn f_030bf8a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf8a4); } r_030bf538(cpu, 0x30bf8a4); }
pub fn f_030bf8b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf8b0); } r_030bf538(cpu, 0x30bf8b0); }
pub fn f_030bf8bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf8bc); } r_030bf538(cpu, 0x30bf8bc); }
pub fn f_030bf8c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf8c8); } r_030bf538(cpu, 0x30bf8c8); }
fn r_030bf538(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bf538 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30bf8f8; continue; }
  pc = 0x30bf552; continue;
 }
 0x30bf552 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30bf578; continue;
 }
 0x30bf578 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30bf57f); f_030c2af8(cpu); if cpu.returned_to != 0x30bf57f { return; }
  if cpu.zf { pc = 0x30bf8e1; continue; }
  pc = 0x30bf585; continue;
 }
 0x30bf585 => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30bf59b; continue; }
  pc = 0x30bf58e; continue;
 }
 0x30bf58e => {
  cpu.push(0x30bf593); f_030c3525(cpu); if cpu.returned_to != 0x30bf593 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30bf8e1; continue; }
  pc = 0x30bf59b; continue;
 }
 0x30bf59b => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf82c; continue; }
  pc = 0x30bf65a; continue;
 }
 0x30bf65a => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf74b; continue; }
  pc = 0x30bf674; continue;
 }
 0x30bf674 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf68d; continue; }
  pc = 0x30bf689; continue;
 }
 0x30bf689 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf68d; continue;
 }
 0x30bf68d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf6a6; continue; }
  pc = 0x30bf6a2; continue;
 }
 0x30bf6a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf6a6; continue;
 }
 0x30bf6a6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf6bf; continue; }
  pc = 0x30bf6bb; continue;
 }
 0x30bf6bb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf6bf; continue;
 }
 0x30bf6bf => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf6d8; continue; }
  pc = 0x30bf6d4; continue;
 }
 0x30bf6d4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf6d8; continue;
 }
 0x30bf6d8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf6f1; continue; }
  pc = 0x30bf6ed; continue;
 }
 0x30bf6ed => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf6f1; continue;
 }
 0x30bf6f1 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf70a; continue; }
  pc = 0x30bf706; continue;
 }
 0x30bf706 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf70a; continue;
 }
 0x30bf70a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf723; continue; }
  pc = 0x30bf71f; continue;
 }
 0x30bf71f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf723; continue;
 }
 0x30bf723 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf73c; continue; }
  pc = 0x30bf738; continue;
 }
 0x30bf738 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf73c; continue;
 }
 0x30bf73c => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf674; continue; }
  pc = 0x30bf74b; continue;
 }
 0x30bf74b => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154cbcu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bf75c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf774; continue; }
  pc = 0x30bf771; continue;
 }
 0x30bf771 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf774; continue;
 }
 0x30bf774 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf777; continue;
 }
 0x30bf777 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf78f; continue; }
  pc = 0x30bf78c; continue;
 }
 0x30bf78c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf78f; continue;
 }
 0x30bf78f => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf792; continue;
 }
 0x30bf792 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf7aa; continue; }
  pc = 0x30bf7a7; continue;
 }
 0x30bf7a7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf7aa; continue;
 }
 0x30bf7aa => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf7ad; continue;
 }
 0x30bf7ad => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf7c5; continue; }
  pc = 0x30bf7c2; continue;
 }
 0x30bf7c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf7c5; continue;
 }
 0x30bf7c5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf7c8; continue;
 }
 0x30bf7c8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf7e0; continue; }
  pc = 0x30bf7dd; continue;
 }
 0x30bf7dd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf7e0; continue;
 }
 0x30bf7e0 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf7e3; continue;
 }
 0x30bf7e3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf7fb; continue; }
  pc = 0x30bf7f8; continue;
 }
 0x30bf7f8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf7fb; continue;
 }
 0x30bf7fb => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf7fe; continue;
 }
 0x30bf7fe => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bf816; continue; }
  pc = 0x30bf813; continue;
 }
 0x30bf813 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bf816; continue;
 }
 0x30bf816 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bf819; continue;
 }
 0x30bf819 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf65a; continue; }
  pc = 0x30bf826; continue;
 }
 0x30bf826 => {
  if cpu.zf { pc = 0x30bf8e1; continue; }
  pc = 0x30bf82c; continue;
 }
 0x30bf82c => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bf863; continue; }
  pc = 0x30bf840; continue;
 }
 0x30bf840 => {
  { let v: u32 = cpu.r32(0x32c1444u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x30bf84d; continue;
 }
 0x30bf84d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf84d; continue; }
  pc = 0x30bf860; continue;
 }
 0x30bf860 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x30bf863; continue;
 }
 0x30bf863 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154cdcu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bf874 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf880; continue;
 }
 0x30bf880 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf88c; continue;
 }
 0x30bf88c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf898; continue;
 }
 0x30bf898 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf8a4; continue;
 }
 0x30bf8a4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf8b0; continue;
 }
 0x30bf8b0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf8bc; continue;
 }
 0x30bf8bc => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c1444u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30bf8c8; continue;
 }
 0x30bf8c8 => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bf65a; continue; }
  pc = 0x30bf8db; continue;
 }
 0x30bf8db => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30bf82c; continue; }
  pc = 0x30bf8e1; continue;
 }
 0x30bf8e1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bf578; continue; }
  pc = 0x30bf8f8; continue;
 }
 0x30bf8f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030bf8f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bf8f9); } r_030bf8f9(cpu, 0x30bf8f9); }
pub fn f_030bff0c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff0c); } r_030bf8f9(cpu, 0x30bff0c); }
pub fn f_030bff23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff23); } r_030bf8f9(cpu, 0x30bff23); }
pub fn f_030bff3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff3a); } r_030bf8f9(cpu, 0x30bff3a); }
pub fn f_030bff51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff51); } r_030bf8f9(cpu, 0x30bff51); }
pub fn f_030bff68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff68); } r_030bf8f9(cpu, 0x30bff68); }
pub fn f_030bff7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff7f); } r_030bf8f9(cpu, 0x30bff7f); }
pub fn f_030bff96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bff96); } r_030bf8f9(cpu, 0x30bff96); }
pub fn f_030bffad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bffad); } r_030bf8f9(cpu, 0x30bffad); }
pub fn f_030bffc4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bffc4); } r_030bf8f9(cpu, 0x30bffc4); }
pub fn f_030bffdb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bffdb); } r_030bf8f9(cpu, 0x30bffdb); }
pub fn f_030bfff2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30bfff2); } r_030bf8f9(cpu, 0x30bfff2); }
pub fn f_030c0009(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0009); } r_030bf8f9(cpu, 0x30c0009); }
pub fn f_030c0020(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0020); } r_030bf8f9(cpu, 0x30c0020); }
pub fn f_030c0037(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0037); } r_030bf8f9(cpu, 0x30c0037); }
pub fn f_030c004e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c004e); } r_030bf8f9(cpu, 0x30c004e); }
pub fn f_030c0065(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0065); } r_030bf8f9(cpu, 0x30c0065); }
pub fn f_030c007c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c007c); } r_030bf8f9(cpu, 0x30c007c); }
fn r_030bf8f9(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bf8f9 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c0158; continue; }
  pc = 0x30bf913; continue;
 }
 0x30bf913 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30bf939; continue;
 }
 0x30bf939 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30bf940); f_030c2af8(cpu); if cpu.returned_to != 0x30bf940 { return; }
  if cpu.zf { pc = 0x30c0141; continue; }
  pc = 0x30bf946; continue;
 }
 0x30bf946 => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30bf95c; continue; }
  pc = 0x30bf94f; continue;
 }
 0x30bf94f => {
  cpu.push(0x30bf954); f_030c3525(cpu); if cpu.returned_to != 0x30bf954 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30c0141; continue; }
  pc = 0x30bf95c; continue;
 }
 0x30bf95c => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bfa5e; continue; }
  pc = 0x30bf9db; continue;
 }
 0x30bf9db => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x30bf9e7; continue; }
  pc = 0x30bf9e2; continue;
 }
 0x30bf9e2 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x30bf9e7; continue;
 }
 0x30bf9e7 => {
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
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154cfcu32, v); }
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
  pc = 0x30bfad1; continue;
 }
 0x30bfa5e => {
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
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154cfcu32, v); }
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
  pc = 0x30c0097; continue;
 }
 0x30bfad1 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bfca3; continue; }
  pc = 0x30bfaea; continue;
 }
 0x30bfaea => {
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
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfb31; continue; }
  pc = 0x30bfb2d; continue;
 }
 0x30bfb2d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfb31; continue;
 }
 0x30bfb31 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfb46; continue; }
  pc = 0x30bfb42; continue;
 }
 0x30bfb42 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfb46; continue;
 }
 0x30bfb46 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfb5b; continue; }
  pc = 0x30bfb57; continue;
 }
 0x30bfb57 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfb5b; continue;
 }
 0x30bfb5b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfb70; continue; }
  pc = 0x30bfb6c; continue;
 }
 0x30bfb6c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfb70; continue;
 }
 0x30bfb70 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfb85; continue; }
  pc = 0x30bfb81; continue;
 }
 0x30bfb81 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfb85; continue;
 }
 0x30bfb85 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfb9a; continue; }
  pc = 0x30bfb96; continue;
 }
 0x30bfb96 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfb9a; continue;
 }
 0x30bfb9a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfbaf; continue; }
  pc = 0x30bfbab; continue;
 }
 0x30bfbab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfbaf; continue;
 }
 0x30bfbaf => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfbc6; continue; }
  pc = 0x30bfbc2; continue;
 }
 0x30bfbc2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffceu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfbc6; continue;
 }
 0x30bfbc6 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfbdd; continue; }
  pc = 0x30bfbd9; continue;
 }
 0x30bfbd9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfbdd; continue;
 }
 0x30bfbdd => {
  { let v = cpu.st(0) * cpu.rf32(0x32a3e18u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfbf8; continue; }
  pc = 0x30bfbf4; continue;
 }
 0x30bfbf4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfbf8; continue;
 }
 0x30bfbf8 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfc0f; continue; }
  pc = 0x30bfc0b; continue;
 }
 0x30bfc0b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfc0f; continue;
 }
 0x30bfc0f => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfc26; continue; }
  pc = 0x30bfc22; continue;
 }
 0x30bfc22 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfc26; continue;
 }
 0x30bfc26 => {
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfc41; continue; }
  pc = 0x30bfc3d; continue;
 }
 0x30bfc3d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfc41; continue;
 }
 0x30bfc41 => {
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfc5c; continue; }
  pc = 0x30bfc58; continue;
 }
 0x30bfc58 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfc5c; continue;
 }
 0x30bfc5c => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfc73; continue; }
  pc = 0x30bfc6f; continue;
 }
 0x30bfc6f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfc73; continue;
 }
 0x30bfc73 => {
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfc8e; continue; }
  pc = 0x30bfc8a; continue;
 }
 0x30bfc8a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfc8e; continue;
 }
 0x30bfc8e => {
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30bfaea; continue; }
  pc = 0x30bfca3; continue;
 }
 0x30bfca3 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30bfe6b; continue; }
  pc = 0x30bfcb2; continue;
 }
 0x30bfcb2 => {
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
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd03; continue; }
  pc = 0x30bfcff; continue;
 }
 0x30bfcff => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd03; continue;
 }
 0x30bfd03 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd18; continue; }
  pc = 0x30bfd14; continue;
 }
 0x30bfd14 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd18; continue;
 }
 0x30bfd18 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd2d; continue; }
  pc = 0x30bfd29; continue;
 }
 0x30bfd29 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd2d; continue;
 }
 0x30bfd2d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd42; continue; }
  pc = 0x30bfd3e; continue;
 }
 0x30bfd3e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd42; continue;
 }
 0x30bfd42 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd57; continue; }
  pc = 0x30bfd53; continue;
 }
 0x30bfd53 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd57; continue;
 }
 0x30bfd57 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd6c; continue; }
  pc = 0x30bfd68; continue;
 }
 0x30bfd68 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffeau32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd6c; continue;
 }
 0x30bfd6c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd81; continue; }
  pc = 0x30bfd7d; continue;
 }
 0x30bfd7d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd81; continue;
 }
 0x30bfd81 => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfd98; continue; }
  pc = 0x30bfd94; continue;
 }
 0x30bfd94 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffeeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfd98; continue;
 }
 0x30bfd98 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfdaf; continue; }
  pc = 0x30bfdab; continue;
 }
 0x30bfdab => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfdaf; continue;
 }
 0x30bfdaf => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfdcc; continue; }
  pc = 0x30bfdc8; continue;
 }
 0x30bfdc8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfdcc; continue;
 }
 0x30bfdcc => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfde3; continue; }
  pc = 0x30bfddf; continue;
 }
 0x30bfddf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfde3; continue;
 }
 0x30bfde3 => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfdfa; continue; }
  pc = 0x30bfdf6; continue;
 }
 0x30bfdf6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfdfa; continue;
 }
 0x30bfdfa => {
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfe15; continue; }
  pc = 0x30bfe11; continue;
 }
 0x30bfe11 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfe15; continue;
 }
 0x30bfe15 => {
  { let v = cpu.st(0); cpu.wf32(0x32c1460u32, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfe30; continue; }
  pc = 0x30bfe2c; continue;
 }
 0x30bfe2c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfe30; continue;
 }
 0x30bfe30 => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfe47; continue; }
  pc = 0x30bfe43; continue;
 }
 0x30bfe43 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfe47; continue;
 }
 0x30bfe47 => {
  { let v = cpu.st(0) + cpu.rf32(0x3154ee0u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bfe62; continue; }
  pc = 0x30bfe5e; continue;
 }
 0x30bfe5e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30bfe62; continue;
 }
 0x30bfe62 => {
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x30bfe6b; continue;
 }
 0x30bfe6b => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154cfcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30bfee0; continue; }
  pc = 0x30bfe79; continue;
 }
 0x30bfe79 => {
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
  cpu.poll(); pc = cpu.r32(0x3154d00u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bfee0 => {
  { let v: u32 = cpu.r32(0x32c1460u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154d00u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30bff0c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bff20; continue; }
  pc = 0x30bff1d; continue;
 }
 0x30bff1d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bff20; continue;
 }
 0x30bff20 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bff23; continue;
 }
 0x30bff23 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bff37; continue; }
  pc = 0x30bff34; continue;
 }
 0x30bff34 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bff37; continue;
 }
 0x30bff37 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bff3a; continue;
 }
 0x30bff3a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bff4e; continue; }
  pc = 0x30bff4b; continue;
 }
 0x30bff4b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bff4e; continue;
 }
 0x30bff4e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bff51; continue;
 }
 0x30bff51 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bff65; continue; }
  pc = 0x30bff62; continue;
 }
 0x30bff62 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bff65; continue;
 }
 0x30bff65 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bff68; continue;
 }
 0x30bff68 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bff7c; continue; }
  pc = 0x30bff79; continue;
 }
 0x30bff79 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bff7c; continue;
 }
 0x30bff7c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bff7f; continue;
 }
 0x30bff7f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bff93; continue; }
  pc = 0x30bff90; continue;
 }
 0x30bff90 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bff93; continue;
 }
 0x30bff93 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bff96; continue;
 }
 0x30bff96 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bffaa; continue; }
  pc = 0x30bffa7; continue;
 }
 0x30bffa7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bffaa; continue;
 }
 0x30bffaa => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bffad; continue;
 }
 0x30bffad => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bffc1; continue; }
  pc = 0x30bffbe; continue;
 }
 0x30bffbe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bffc1; continue;
 }
 0x30bffc1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bffc4; continue;
 }
 0x30bffc4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bffd8; continue; }
  pc = 0x30bffd5; continue;
 }
 0x30bffd5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bffd8; continue;
 }
 0x30bffd8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bffdb; continue;
 }
 0x30bffdb => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30bffef; continue; }
  pc = 0x30bffec; continue;
 }
 0x30bffec => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30bffef; continue;
 }
 0x30bffef => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30bfff2; continue;
 }
 0x30bfff2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30c0006; continue; }
  pc = 0x30c0003; continue;
 }
 0x30c0003 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0006; continue;
 }
 0x30c0006 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0009; continue;
 }
 0x30c0009 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30c001d; continue; }
  pc = 0x30c001a; continue;
 }
 0x30c001a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c001d; continue;
 }
 0x30c001d => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0020; continue;
 }
 0x30c0020 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30c0034; continue; }
  pc = 0x30c0031; continue;
 }
 0x30c0031 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0034; continue;
 }
 0x30c0034 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0037; continue;
 }
 0x30c0037 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30c004b; continue; }
  pc = 0x30c0048; continue;
 }
 0x30c0048 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c004b; continue;
 }
 0x30c004b => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c004e; continue;
 }
 0x30c004e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30c0062; continue; }
  pc = 0x30c005f; continue;
 }
 0x30c005f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0062; continue;
 }
 0x30c0062 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0065; continue;
 }
 0x30c0065 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x30c0079; continue; }
  pc = 0x30c0076; continue;
 }
 0x30c0076 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0079; continue;
 }
 0x30c0079 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c007c; continue;
 }
 0x30c007c => {
  { let v: u32 = cpu.r32(0x3154cfcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30c0135; continue; }
  pc = 0x30c008b; continue;
 }
 0x30c008b => {
  { let v: u32 = cpu.r32(0x3154cfcu32).wrapping_add(0x4u32); cpu.w32(0x3154cfcu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30c0097; continue;
 }
 0x30c0097 => {
  { let v: u32 = cpu.r32(0x3154cfcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c00ab; continue; }
  pc = 0x30c00a6; continue;
 }
 0x30c00a6 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x30c00ab; continue;
 }
 0x30c00ab => {
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
  { let v: u32 = cpu.r32(0x3154cfcu32); cpu.eax = v; }
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
  { let v: u32 = cpu.r32(0x3154cfcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x30c0135; continue; }
  pc = 0x30c0126; continue;
 }
 0x30c0126 => {
  { let v: u32 = cpu.r32(0x3154cfcu32).wrapping_add(0x4u32); cpu.w32(0x3154cfcu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x30bfad1; continue;
 }
 0x30c0135 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x30c0141; continue;
 }
 0x30c0141 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30bf939; continue; }
  pc = 0x30c0158; continue;
 }
 0x30c0158 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c0159(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0159); } r_030c0159(cpu, 0x30c0159); }
pub fn f_030c04d5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c04d5); } r_030c0159(cpu, 0x30c04d5); }
pub fn f_030c0540(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0540); } r_030c0159(cpu, 0x30c0540); }
pub fn f_030c05ab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c05ab); } r_030c0159(cpu, 0x30c05ab); }
pub fn f_030c0616(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0616); } r_030c0159(cpu, 0x30c0616); }
pub fn f_030c068d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c068d); } r_030c0159(cpu, 0x30c068d); }
pub fn f_030c06f8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c06f8); } r_030c0159(cpu, 0x30c06f8); }
pub fn f_030c0763(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0763); } r_030c0159(cpu, 0x30c0763); }
pub fn f_030c07ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c07ce); } r_030c0159(cpu, 0x30c07ce); }
pub fn f_030c0834(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0834); } r_030c0159(cpu, 0x30c0834); }
pub fn f_030c084c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c084c); } r_030c0159(cpu, 0x30c084c); }
pub fn f_030c0864(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0864); } r_030c0159(cpu, 0x30c0864); }
pub fn f_030c087c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c087c); } r_030c0159(cpu, 0x30c087c); }
pub fn f_030c0894(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0894); } r_030c0159(cpu, 0x30c0894); }
pub fn f_030c08ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c08ac); } r_030c0159(cpu, 0x30c08ac); }
pub fn f_030c08c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c08c4); } r_030c0159(cpu, 0x30c08c4); }
pub fn f_030c08dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c08dc); } r_030c0159(cpu, 0x30c08dc); }
fn r_030c0159(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c0159 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c0916; continue; }
  pc = 0x30c0173; continue;
 }
 0x30c0173 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30c0199; continue;
 }
 0x30c0199 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30c01a0); f_030c2af8(cpu); if cpu.returned_to != 0x30c01a0 { return; }
  if cpu.zf { pc = 0x30c08ff; continue; }
  pc = 0x30c01a6; continue;
 }
 0x30c01a6 => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30c01bc; continue; }
  pc = 0x30c01af; continue;
 }
 0x30c01af => {
  cpu.push(0x30c01b4); f_030c3525(cpu); if cpu.returned_to != 0x30c01b4 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30c08ff; continue; }
  pc = 0x30c01bc; continue;
 }
 0x30c01bc => {
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
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1448u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c144cu32, v); }
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154d44u32, v); }
  { let v: u32 = cpu.r32(0x3154d44u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c07eb; continue; }
  pc = 0x30c028a; continue;
 }
 0x30c028a => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30c02ef; continue; }
  pc = 0x30c0292; continue;
 }
 0x30c0292 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x30c02e3; continue; }
  pc = 0x30c02b6; continue;
 }
 0x30c02b6 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c02e3; continue;
 }
 0x30c02e3 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154d44u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30c02ef; continue;
 }
 0x30c02ef => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c04c4; continue; }
  pc = 0x30c0309; continue;
 }
 0x30c0309 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0374; continue; }
  pc = 0x30c0345; continue;
 }
 0x30c0345 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0374; continue;
 }
 0x30c0374 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c03df; continue; }
  pc = 0x30c03b0; continue;
 }
 0x30c03b0 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c03df; continue;
 }
 0x30c03df => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c044a; continue; }
  pc = 0x30c041b; continue;
 }
 0x30c041b => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c044a; continue;
 }
 0x30c044a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c04b5; continue; }
  pc = 0x30c0486; continue;
 }
 0x30c0486 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c04b5; continue;
 }
 0x30c04b5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c0309; continue; }
  pc = 0x30c04c4; continue;
 }
 0x30c04c4 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154d48u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c04d5 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c053d; continue; }
  pc = 0x30c0511; continue;
 }
 0x30c0511 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c053d; continue;
 }
 0x30c053d => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c0540; continue;
 }
 0x30c0540 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c05a8; continue; }
  pc = 0x30c057c; continue;
 }
 0x30c057c => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c05a8; continue;
 }
 0x30c05a8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c05ab; continue;
 }
 0x30c05ab => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0613; continue; }
  pc = 0x30c05e7; continue;
 }
 0x30c05e7 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0613; continue;
 }
 0x30c0613 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c0616; continue;
 }
 0x30c0616 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x30c0667; continue; }
  pc = 0x30c063a; continue;
 }
 0x30c063a => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0667; continue;
 }
 0x30c0667 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154d44u32).wrapping_add(0x4u32); cpu.w32(0x3154d44u32, v); }
  { let v: u32 = cpu.r32(0x3154d44u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c028a; continue; }
  pc = 0x30c0681; continue;
 }
 0x30c0681 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c07eb; continue; }
  pc = 0x30c0687; continue;
 }
 0x30c0687 => {
  if cpu.zf { pc = 0x30c08ff; continue; }
  pc = 0x30c068d; continue;
 }
 0x30c068d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c06f5; continue; }
  pc = 0x30c06c9; continue;
 }
 0x30c06c9 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c06f5; continue;
 }
 0x30c06f5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c06f8; continue;
 }
 0x30c06f8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0760; continue; }
  pc = 0x30c0734; continue;
 }
 0x30c0734 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0760; continue;
 }
 0x30c0760 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c0763; continue;
 }
 0x30c0763 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c07cb; continue; }
  pc = 0x30c079f; continue;
 }
 0x30c079f => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c07cb; continue;
 }
 0x30c07cb => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c07ce; continue;
 }
 0x30c07ce => {
  { let v: u32 = cpu.r32(0x3154d44u32).wrapping_add(0x4u32); cpu.w32(0x3154d44u32, v); }
  { let v: u32 = cpu.r32(0x3154d44u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c028a; continue; }
  pc = 0x30c07e5; continue;
 }
 0x30c07e5 => {
  if cpu.zf { pc = 0x30c08ff; continue; }
  pc = 0x30c07eb; continue;
 }
 0x30c07eb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c0823; continue; }
  pc = 0x30c07ff; continue;
 }
 0x30c07ff => {
  { let v: u32 = cpu.r32(0x32c1448u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32c144cu32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 3u32); cpu.edx = v; }
  pc = 0x30c0810; continue;
 }
 0x30c0810 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c0810; continue; }
  pc = 0x30c0823; continue;
 }
 0x30c0823 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154d68u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c0834 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c084c; continue;
 }
 0x30c084c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0864; continue;
 }
 0x30c0864 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c087c; continue;
 }
 0x30c087c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0894; continue;
 }
 0x30c0894 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c08ac; continue;
 }
 0x30c08ac => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c08c4; continue;
 }
 0x30c08c4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c08dc; continue;
 }
 0x30c08dc => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154d44u32).wrapping_add(0x4u32); cpu.w32(0x3154d44u32, v); }
  { let v: u32 = cpu.r32(0x3154d44u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c028a; continue; }
  pc = 0x30c08f9; continue;
 }
 0x30c08f9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c07eb; continue; }
  pc = 0x30c08ff; continue;
 }
 0x30c08ff => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c0199; continue; }
  pc = 0x30c0916; continue;
 }
 0x30c0916 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c0917(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0917); } r_030c0917(cpu, 0x30c0917); }
pub fn f_030c0cb4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0cb4); } r_030c0917(cpu, 0x30c0cb4); }
pub fn f_030c0cfb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0cfb); } r_030c0917(cpu, 0x30c0cfb); }
pub fn f_030c0d42(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0d42); } r_030c0917(cpu, 0x30c0d42); }
pub fn f_030c0d89(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0d89); } r_030c0917(cpu, 0x30c0d89); }
pub fn f_030c0dd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0dd0); } r_030c0917(cpu, 0x30c0dd0); }
pub fn f_030c0e17(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0e17); } r_030c0917(cpu, 0x30c0e17); }
pub fn f_030c0e5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0e5e); } r_030c0917(cpu, 0x30c0e5e); }
pub fn f_030c0ea5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0ea5); } r_030c0917(cpu, 0x30c0ea5); }
pub fn f_030c0f0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f0b); } r_030c0917(cpu, 0x30c0f0b); }
pub fn f_030c0f23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f23); } r_030c0917(cpu, 0x30c0f23); }
pub fn f_030c0f3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f3b); } r_030c0917(cpu, 0x30c0f3b); }
pub fn f_030c0f53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f53); } r_030c0917(cpu, 0x30c0f53); }
pub fn f_030c0f6b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f6b); } r_030c0917(cpu, 0x30c0f6b); }
pub fn f_030c0f83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f83); } r_030c0917(cpu, 0x30c0f83); }
pub fn f_030c0f9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0f9b); } r_030c0917(cpu, 0x30c0f9b); }
pub fn f_030c0fb3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c0fb3); } r_030c0917(cpu, 0x30c0fb3); }
fn r_030c0917(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c0917 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c0fed; continue; }
  pc = 0x30c0931; continue;
 }
 0x30c0931 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30c0957; continue;
 }
 0x30c0957 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30c095e); f_030c2af8(cpu); if cpu.returned_to != 0x30c095e { return; }
  if cpu.zf { pc = 0x30c0fd6; continue; }
  pc = 0x30c0964; continue;
 }
 0x30c0964 => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30c097a; continue; }
  pc = 0x30c096d; continue;
 }
 0x30c096d => {
  cpu.push(0x30c0972); f_030c3525(cpu); if cpu.returned_to != 0x30c0972 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30c0fd6; continue; }
  pc = 0x30c097a; continue;
 }
 0x30c097a => {
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
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1448u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c144cu32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154d88u32, v); }
  { let v: u32 = cpu.r32(0x3154d88u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c0ec2; continue; }
  pc = 0x30c0a4a; continue;
 }
 0x30c0a4a => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c0ca3; continue; }
  pc = 0x30c0a64; continue;
 }
 0x30c0a64 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0aaa; continue; }
  pc = 0x30c0a86; continue;
 }
 0x30c0a86 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0aaa; continue;
 }
 0x30c0aaa => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0af0; continue; }
  pc = 0x30c0acc; continue;
 }
 0x30c0acc => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0af0; continue;
 }
 0x30c0af0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0b36; continue; }
  pc = 0x30c0b12; continue;
 }
 0x30c0b12 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0b36; continue;
 }
 0x30c0b36 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0b7c; continue; }
  pc = 0x30c0b58; continue;
 }
 0x30c0b58 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0b7c; continue;
 }
 0x30c0b7c => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0bc2; continue; }
  pc = 0x30c0b9e; continue;
 }
 0x30c0b9e => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0bc2; continue;
 }
 0x30c0bc2 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0c08; continue; }
  pc = 0x30c0be4; continue;
 }
 0x30c0be4 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0c08; continue;
 }
 0x30c0c08 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0c4e; continue; }
  pc = 0x30c0c2a; continue;
 }
 0x30c0c2a => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0c4e; continue;
 }
 0x30c0c4e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0c94; continue; }
  pc = 0x30c0c70; continue;
 }
 0x30c0c70 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0c94; continue;
 }
 0x30c0c94 => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c0a64; continue; }
  pc = 0x30c0ca3; continue;
 }
 0x30c0ca3 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154d8cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c0cb4 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0cf8; continue; }
  pc = 0x30c0cd6; continue;
 }
 0x30c0cd6 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0cf8; continue;
 }
 0x30c0cf8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0cfb; continue;
 }
 0x30c0cfb => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0d3f; continue; }
  pc = 0x30c0d1d; continue;
 }
 0x30c0d1d => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0d3f; continue;
 }
 0x30c0d3f => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0d42; continue;
 }
 0x30c0d42 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0d86; continue; }
  pc = 0x30c0d64; continue;
 }
 0x30c0d64 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0d86; continue;
 }
 0x30c0d86 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0d89; continue;
 }
 0x30c0d89 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0dcd; continue; }
  pc = 0x30c0dab; continue;
 }
 0x30c0dab => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0dcd; continue;
 }
 0x30c0dcd => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0dd0; continue;
 }
 0x30c0dd0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0e14; continue; }
  pc = 0x30c0df2; continue;
 }
 0x30c0df2 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0e14; continue;
 }
 0x30c0e14 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0e17; continue;
 }
 0x30c0e17 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0e5b; continue; }
  pc = 0x30c0e39; continue;
 }
 0x30c0e39 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0e5b; continue;
 }
 0x30c0e5b => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0e5e; continue;
 }
 0x30c0e5e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c0ea2; continue; }
  pc = 0x30c0e80; continue;
 }
 0x30c0e80 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c0ea2; continue;
 }
 0x30c0ea2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c0ea5; continue;
 }
 0x30c0ea5 => {
  { let v: u32 = cpu.r32(0x3154d88u32).wrapping_add(0x4u32); cpu.w32(0x3154d88u32, v); }
  { let v: u32 = cpu.r32(0x3154d88u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c0a4a; continue; }
  pc = 0x30c0ebc; continue;
 }
 0x30c0ebc => {
  if cpu.zf { pc = 0x30c0fd6; continue; }
  pc = 0x30c0ec2; continue;
 }
 0x30c0ec2 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c0efa; continue; }
  pc = 0x30c0ed6; continue;
 }
 0x30c0ed6 => {
  { let v: u32 = cpu.r32(0x32c1448u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32c144cu32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 3u32); cpu.edx = v; }
  pc = 0x30c0ee7; continue;
 }
 0x30c0ee7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c0ee7; continue; }
  pc = 0x30c0efa; continue;
 }
 0x30c0efa => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154dacu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c0f0b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0f23; continue;
 }
 0x30c0f23 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0f3b; continue;
 }
 0x30c0f3b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0f53; continue;
 }
 0x30c0f53 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0f6b; continue;
 }
 0x30c0f6b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0f83; continue;
 }
 0x30c0f83 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0f9b; continue;
 }
 0x30c0f9b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c0fb3; continue;
 }
 0x30c0fb3 => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154d88u32).wrapping_add(0x4u32); cpu.w32(0x3154d88u32, v); }
  { let v: u32 = cpu.r32(0x3154d88u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c0a4a; continue; }
  pc = 0x30c0fd0; continue;
 }
 0x30c0fd0 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c0ec2; continue; }
  pc = 0x30c0fd6; continue;
 }
 0x30c0fd6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c0957; continue; }
  pc = 0x30c0fed; continue;
 }
 0x30c0fed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c0fee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c0fee); }
 let mut pc: u32 = 0x30c0fee;
 loop { match pc {
 0x30c0fee => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30c0ff1; continue;
 }
 0x30c0ff1 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1007; continue; }
  pc = 0x30c0ffb; continue;
 }
 0x30c0ffb => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154de4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154ddcu32); cpu.ecx = v; }
  pc = 0x30c1007; continue;
 }
 0x30c1007 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1017; continue; }
  pc = 0x30c100b; continue;
 }
 0x30c100b => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154de8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154dd4u32); cpu.ecx = v; }
  pc = 0x30c1017; continue;
 }
 0x30c1017 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1027; continue; }
  pc = 0x30c101b; continue;
 }
 0x30c101b => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154decu32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154dccu32); cpu.ecx = v; }
  pc = 0x30c1027; continue;
 }
 0x30c1027 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1037; continue; }
  pc = 0x30c102b; continue;
 }
 0x30c102b => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154df0u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154de0u32); cpu.ecx = v; }
  pc = 0x30c1037; continue;
 }
 0x30c1037 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1047; continue; }
  pc = 0x30c103b; continue;
 }
 0x30c103b => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154df4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154dd8u32); cpu.ecx = v; }
  pc = 0x30c1047; continue;
 }
 0x30c1047 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1057; continue; }
  pc = 0x30c104b; continue;
 }
 0x30c104b => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154df8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154dd0u32); cpu.ecx = v; }
  pc = 0x30c1057; continue;
 }
 0x30c1057 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c0ff1; continue; }
  pc = 0x30c1064; continue;
 }
 0x30c1064 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x30c1068; continue;
 }
 0x30c1068 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x30c1078; continue; }
  pc = 0x30c1075; continue;
 }
 0x30c1075 => {
  { let v: u32 = cpu.eax & 0xffffffe0u32; cpu.eax = v; }
  pc = 0x30c1078; continue;
 }
 0x30c1078 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if !cpu.zf { pc = 0x30c1085; continue; }
  pc = 0x30c1080; continue;
 }
 0x30c1080 => {
  { let v: u32 = cpu.eax & 0xfffff81fu32; cpu.eax = v; }
  pc = 0x30c1085; continue;
 }
 0x30c1085 => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if !cpu.zf { pc = 0x30c1092; continue; }
  pc = 0x30c108d; continue;
 }
 0x30c108d => {
  { let v: u32 = cpu.eax & 0xffff07ffu32; cpu.eax = v; }
  pc = 0x30c1092; continue;
 }
 0x30c1092 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x328a244u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c1068; continue; }
  pc = 0x30c10a5; continue;
 }
 0x30c10a5 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c10a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c10a7); }
 let mut pc: u32 = 0x30c10a7;
 loop { match pc {
 0x30c10a7 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30c10aa; continue;
 }
 0x30c10aa => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c10c0; continue; }
  pc = 0x30c10b4; continue;
 }
 0x30c10b4 => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154e14u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154e0cu32); cpu.ecx = v; }
  pc = 0x30c10c0; continue;
 }
 0x30c10c0 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c10d0; continue; }
  pc = 0x30c10c4; continue;
 }
 0x30c10c4 => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154e18u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154e04u32); cpu.ecx = v; }
  pc = 0x30c10d0; continue;
 }
 0x30c10d0 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c10e0; continue; }
  pc = 0x30c10d4; continue;
 }
 0x30c10d4 => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154e1cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154dfcu32); cpu.ecx = v; }
  pc = 0x30c10e0; continue;
 }
 0x30c10e0 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c10f0; continue; }
  pc = 0x30c10e4; continue;
 }
 0x30c10e4 => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154e20u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154e10u32); cpu.ecx = v; }
  pc = 0x30c10f0; continue;
 }
 0x30c10f0 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1100; continue; }
  pc = 0x30c10f4; continue;
 }
 0x30c10f4 => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154e24u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154e08u32); cpu.ecx = v; }
  pc = 0x30c1100; continue;
 }
 0x30c1100 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x30c1110; continue; }
  pc = 0x30c1104; continue;
 }
 0x30c1104 => {
  { let v: u32 = cpu.ebx | cpu.r32(0x3154e28u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x3154e00u32); cpu.ecx = v; }
  pc = 0x30c1110; continue;
 }
 0x30c1110 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c10aa; continue; }
  pc = 0x30c111d; continue;
 }
 0x30c111d => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x30c1121; continue;
 }
 0x30c1121 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x30c1131; continue; }
  pc = 0x30c112e; continue;
 }
 0x30c112e => {
  { let v: u32 = cpu.eax & 0xffffffe0u32; cpu.eax = v; }
  pc = 0x30c1131; continue;
 }
 0x30c1131 => {
  { cpu.and32(cpu.ecx, 0x20u32); }
  if !cpu.zf { pc = 0x30c113e; continue; }
  pc = 0x30c1139; continue;
 }
 0x30c1139 => {
  { let v: u32 = cpu.eax & 0xfffffc1fu32; cpu.eax = v; }
  pc = 0x30c113e; continue;
 }
 0x30c113e => {
  { cpu.and32(cpu.ecx, 0x400u32); }
  if !cpu.zf { pc = 0x30c114b; continue; }
  pc = 0x30c1146; continue;
 }
 0x30c1146 => {
  { let v: u32 = cpu.eax & 0xffff83ffu32; cpu.eax = v; }
  pc = 0x30c114b; continue;
 }
 0x30c114b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x328a244u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c1121; continue; }
  pc = 0x30c115e; continue;
 }
 0x30c115e => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c1160(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1160); } r_030c1160(cpu, 0x30c1160); }
pub fn f_030c1264(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1264); } r_030c1160(cpu, 0x30c1264); }
pub fn f_030c1274(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1274); } r_030c1160(cpu, 0x30c1274); }
pub fn f_030c1284(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1284); } r_030c1160(cpu, 0x30c1284); }
pub fn f_030c1294(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1294); } r_030c1160(cpu, 0x30c1294); }
pub fn f_030c12b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c12b7); } r_030c1160(cpu, 0x30c12b7); }
pub fn f_030c12c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c12c7); } r_030c1160(cpu, 0x30c12c7); }
pub fn f_030c12d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c12d7); } r_030c1160(cpu, 0x30c12d7); }
pub fn f_030c12e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c12e7); } r_030c1160(cpu, 0x30c12e7); }
fn r_030c1160(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c1160 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c131f; continue; }
  pc = 0x30c117a; continue;
 }
 0x30c117a => {
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
  { let v: u32 = cpu.and32(cpu.eax, 0x3def3defu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  pc = 0x30c11af; continue;
 }
 0x30c11af => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30c11b5); f_030c2af8(cpu); if cpu.returned_to != 0x30c11b5 { return; }
  if cpu.zf { pc = 0x30c1309; continue; }
  pc = 0x30c11bb; continue;
 }
 0x30c11bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.ebx = v; }
  { let v: u32 = 0x328c28cu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c12f6; continue; }
  pc = 0x30c11d9; continue;
 }
 0x30c11d9 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30c11f6; continue; }
  pc = 0x30c11e1; continue;
 }
 0x30c11e1 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30c11f6; continue;
 }
 0x30c11f6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c1253; continue; }
  pc = 0x30c120c; continue;
 }
 0x30c120c => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c120c; continue; }
  pc = 0x30c1253; continue;
 }
 0x30c1253 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154e2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c1264 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c1274; continue;
 }
 0x30c1274 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c1284; continue;
 }
 0x30c1284 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c1294; continue;
 }
 0x30c1294 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c11d9; continue; }
  pc = 0x30c12b3; continue;
 }
 0x30c12b3 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c12f6; continue; }
  pc = 0x30c12b5; continue;
 }
 0x30c12b5 => {
  if cpu.zf { pc = 0x30c1309; continue; }
  pc = 0x30c12b7; continue;
 }
 0x30c12b7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c12c7; continue;
 }
 0x30c12c7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c12d7; continue;
 }
 0x30c12d7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3def3defu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c12e7; continue;
 }
 0x30c12e7 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c11d9; continue; }
  pc = 0x30c12f4; continue;
 }
 0x30c12f4 => {
  if cpu.zf { pc = 0x30c1309; continue; }
  pc = 0x30c12f6; continue;
 }
 0x30c12f6 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c11d9; continue; }
  pc = 0x30c1307; continue;
 }
 0x30c1307 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c12f6; continue; }
  pc = 0x30c1309; continue;
 }
 0x30c1309 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c11af; continue; }
  pc = 0x30c131f; continue;
 }
 0x30c131f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c1320(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1320); } r_030c1320(cpu, 0x30c1320); }
pub fn f_030c169c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c169c); } r_030c1320(cpu, 0x30c169c); }
pub fn f_030c1707(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1707); } r_030c1320(cpu, 0x30c1707); }
pub fn f_030c1772(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1772); } r_030c1320(cpu, 0x30c1772); }
pub fn f_030c17dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c17dd); } r_030c1320(cpu, 0x30c17dd); }
pub fn f_030c1854(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1854); } r_030c1320(cpu, 0x30c1854); }
pub fn f_030c18bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c18bf); } r_030c1320(cpu, 0x30c18bf); }
pub fn f_030c192a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c192a); } r_030c1320(cpu, 0x30c192a); }
pub fn f_030c1995(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1995); } r_030c1320(cpu, 0x30c1995); }
pub fn f_030c19fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c19fb); } r_030c1320(cpu, 0x30c19fb); }
pub fn f_030c1a13(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1a13); } r_030c1320(cpu, 0x30c1a13); }
pub fn f_030c1a2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1a2b); } r_030c1320(cpu, 0x30c1a2b); }
pub fn f_030c1a43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1a43); } r_030c1320(cpu, 0x30c1a43); }
pub fn f_030c1a5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1a5b); } r_030c1320(cpu, 0x30c1a5b); }
pub fn f_030c1a73(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1a73); } r_030c1320(cpu, 0x30c1a73); }
pub fn f_030c1a8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1a8b); } r_030c1320(cpu, 0x30c1a8b); }
pub fn f_030c1aa3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1aa3); } r_030c1320(cpu, 0x30c1aa3); }
fn r_030c1320(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c1320 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c1add; continue; }
  pc = 0x30c133a; continue;
 }
 0x30c133a => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30c1360; continue;
 }
 0x30c1360 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30c1367); f_030c2af8(cpu); if cpu.returned_to != 0x30c1367 { return; }
  if cpu.zf { pc = 0x30c1ac6; continue; }
  pc = 0x30c136d; continue;
 }
 0x30c136d => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30c1383; continue; }
  pc = 0x30c1376; continue;
 }
 0x30c1376 => {
  cpu.push(0x30c137b); f_030c3525(cpu); if cpu.returned_to != 0x30c137b { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30c1ac6; continue; }
  pc = 0x30c1383; continue;
 }
 0x30c1383 => {
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
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1448u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c144cu32, v); }
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154e4cu32, v); }
  { let v: u32 = cpu.r32(0x3154e4cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c19b2; continue; }
  pc = 0x30c1451; continue;
 }
 0x30c1451 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x30c14b6; continue; }
  pc = 0x30c1459; continue;
 }
 0x30c1459 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x30c14aa; continue; }
  pc = 0x30c147d; continue;
 }
 0x30c147d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c14aa; continue;
 }
 0x30c14aa => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154e4cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30c14b6; continue;
 }
 0x30c14b6 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c168b; continue; }
  pc = 0x30c14d0; continue;
 }
 0x30c14d0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c153b; continue; }
  pc = 0x30c150c; continue;
 }
 0x30c150c => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c153b; continue;
 }
 0x30c153b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c15a6; continue; }
  pc = 0x30c1577; continue;
 }
 0x30c1577 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c15a6; continue;
 }
 0x30c15a6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1611; continue; }
  pc = 0x30c15e2; continue;
 }
 0x30c15e2 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1611; continue;
 }
 0x30c1611 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c167c; continue; }
  pc = 0x30c164d; continue;
 }
 0x30c164d => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c167c; continue;
 }
 0x30c167c => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c14d0; continue; }
  pc = 0x30c168b; continue;
 }
 0x30c168b => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154e50u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c169c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1704; continue; }
  pc = 0x30c16d8; continue;
 }
 0x30c16d8 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1704; continue;
 }
 0x30c1704 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c1707; continue;
 }
 0x30c1707 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c176f; continue; }
  pc = 0x30c1743; continue;
 }
 0x30c1743 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c176f; continue;
 }
 0x30c176f => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c1772; continue;
 }
 0x30c1772 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c17da; continue; }
  pc = 0x30c17ae; continue;
 }
 0x30c17ae => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c17da; continue;
 }
 0x30c17da => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c17dd; continue;
 }
 0x30c17dd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x30c182e; continue; }
  pc = 0x30c1801; continue;
 }
 0x30c1801 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c182e; continue;
 }
 0x30c182e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154e4cu32).wrapping_add(0x4u32); cpu.w32(0x3154e4cu32, v); }
  { let v: u32 = cpu.r32(0x3154e4cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c1451; continue; }
  pc = 0x30c1848; continue;
 }
 0x30c1848 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c19b2; continue; }
  pc = 0x30c184e; continue;
 }
 0x30c184e => {
  if cpu.zf { pc = 0x30c1ac6; continue; }
  pc = 0x30c1854; continue;
 }
 0x30c1854 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c18bc; continue; }
  pc = 0x30c1890; continue;
 }
 0x30c1890 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c18bc; continue;
 }
 0x30c18bc => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c18bf; continue;
 }
 0x30c18bf => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1927; continue; }
  pc = 0x30c18fb; continue;
 }
 0x30c18fb => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1927; continue;
 }
 0x30c1927 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c192a; continue;
 }
 0x30c192a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1992; continue; }
  pc = 0x30c1966; continue;
 }
 0x30c1966 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x4210421u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xc63u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x3282244u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1992; continue;
 }
 0x30c1992 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c1995; continue;
 }
 0x30c1995 => {
  { let v: u32 = cpu.r32(0x3154e4cu32).wrapping_add(0x4u32); cpu.w32(0x3154e4cu32, v); }
  { let v: u32 = cpu.r32(0x3154e4cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c1451; continue; }
  pc = 0x30c19ac; continue;
 }
 0x30c19ac => {
  if cpu.zf { pc = 0x30c1ac6; continue; }
  pc = 0x30c19b2; continue;
 }
 0x30c19b2 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c19ea; continue; }
  pc = 0x30c19c6; continue;
 }
 0x30c19c6 => {
  { let v: u32 = cpu.r32(0x32c1448u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32c144cu32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 3u32); cpu.edx = v; }
  pc = 0x30c19d7; continue;
 }
 0x30c19d7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c19d7; continue; }
  pc = 0x30c19ea; continue;
 }
 0x30c19ea => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154e70u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c19fb => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1a13; continue;
 }
 0x30c1a13 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1a2b; continue;
 }
 0x30c1a2b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1a43; continue;
 }
 0x30c1a43 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1a5b; continue;
 }
 0x30c1a5b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1a73; continue;
 }
 0x30c1a73 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1a8b; continue;
 }
 0x30c1a8b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c1aa3; continue;
 }
 0x30c1aa3 => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154e4cu32).wrapping_add(0x4u32); cpu.w32(0x3154e4cu32, v); }
  { let v: u32 = cpu.r32(0x3154e4cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c1451; continue; }
  pc = 0x30c1ac0; continue;
 }
 0x30c1ac0 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c19b2; continue; }
  pc = 0x30c1ac6; continue;
 }
 0x30c1ac6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c1360; continue; }
  pc = 0x30c1add; continue;
 }
 0x30c1add => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c1ade(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1ade); } r_030c1ade(cpu, 0x30c1ade); }
pub fn f_030c1e7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1e7b); } r_030c1ade(cpu, 0x30c1e7b); }
pub fn f_030c1ec2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1ec2); } r_030c1ade(cpu, 0x30c1ec2); }
pub fn f_030c1f09(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1f09); } r_030c1ade(cpu, 0x30c1f09); }
pub fn f_030c1f50(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1f50); } r_030c1ade(cpu, 0x30c1f50); }
pub fn f_030c1f97(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1f97); } r_030c1ade(cpu, 0x30c1f97); }
pub fn f_030c1fde(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c1fde); } r_030c1ade(cpu, 0x30c1fde); }
pub fn f_030c2025(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c2025); } r_030c1ade(cpu, 0x30c2025); }
pub fn f_030c206c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c206c); } r_030c1ade(cpu, 0x30c206c); }
pub fn f_030c20d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c20d2); } r_030c1ade(cpu, 0x30c20d2); }
pub fn f_030c20ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c20ea); } r_030c1ade(cpu, 0x30c20ea); }
pub fn f_030c2102(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c2102); } r_030c1ade(cpu, 0x30c2102); }
pub fn f_030c211a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c211a); } r_030c1ade(cpu, 0x30c211a); }
pub fn f_030c2132(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c2132); } r_030c1ade(cpu, 0x30c2132); }
pub fn f_030c214a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c214a); } r_030c1ade(cpu, 0x30c214a); }
pub fn f_030c2162(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c2162); } r_030c1ade(cpu, 0x30c2162); }
pub fn f_030c217a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c217a); } r_030c1ade(cpu, 0x30c217a); }
fn r_030c1ade(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c1ade => {
  { let v: u32 = cpu.inc32(cpu.r32(0x32cb39cu32)); cpu.w32(0x32cb39cu32, v); }
  { let v: u32 = cpu.r32(0x32c1790u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32c178cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c21b4; continue; }
  pc = 0x30c1af8; continue;
 }
 0x30c1af8 => {
  { let v: u32 = 0x32c1794u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32c143cu32, v); }
  { let v: u32 = cpu.r32(0x3155960u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x3155950u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x32cb3b4u32, v); }
  pc = 0x30c1b1e; continue;
 }
 0x30c1b1e => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30c1b25); f_030c2af8(cpu); if cpu.returned_to != 0x30c1b25 { return; }
  if cpu.zf { pc = 0x30c219d; continue; }
  pc = 0x30c1b2b; continue;
 }
 0x30c1b2b => {
  { cpu.sub32(cpu.r32(0x32cb3b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x30c1b41; continue; }
  pc = 0x30c1b34; continue;
 }
 0x30c1b34 => {
  cpu.push(0x30c1b39); f_030c3525(cpu); if cpu.returned_to != 0x30c1b39 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30c219d; continue; }
  pc = 0x30c1b41; continue;
 }
 0x30c1b41 => {
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
  { let v: u32 = cpu.ecx; cpu.w32(0x32c1448u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32c1468u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32c1468u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c144cu32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x328c28cu32; cpu.w32(0x3154e90u32, v); }
  { let v: u32 = cpu.r32(0x3154e90u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2089; continue; }
  pc = 0x30c1c11; continue;
 }
 0x30c1c11 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c1e6a; continue; }
  pc = 0x30c1c2b; continue;
 }
 0x30c1c2b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1c71; continue; }
  pc = 0x30c1c4d; continue;
 }
 0x30c1c4d => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1c71; continue;
 }
 0x30c1c71 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1cb7; continue; }
  pc = 0x30c1c93; continue;
 }
 0x30c1c93 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff2u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1cb7; continue;
 }
 0x30c1cb7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1cfd; continue; }
  pc = 0x30c1cd9; continue;
 }
 0x30c1cd9 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1cfd; continue;
 }
 0x30c1cfd => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1d43; continue; }
  pc = 0x30c1d1f; continue;
 }
 0x30c1d1f => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1d43; continue;
 }
 0x30c1d43 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1d89; continue; }
  pc = 0x30c1d65; continue;
 }
 0x30c1d65 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1d89; continue;
 }
 0x30c1d89 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1dcf; continue; }
  pc = 0x30c1dab; continue;
 }
 0x30c1dab => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1dcf; continue;
 }
 0x30c1dcf => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1e15; continue; }
  pc = 0x30c1df1; continue;
 }
 0x30c1df1 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1e15; continue;
 }
 0x30c1e15 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1e5b; continue; }
  pc = 0x30c1e37; continue;
 }
 0x30c1e37 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1e5b; continue;
 }
 0x30c1e5b => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c1c2b; continue; }
  pc = 0x30c1e6a; continue;
 }
 0x30c1e6a => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154e94u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c1e7b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1ebf; continue; }
  pc = 0x30c1e9d; continue;
 }
 0x30c1e9d => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1ebf; continue;
 }
 0x30c1ebf => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c1ec2; continue;
 }
 0x30c1ec2 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1f06; continue; }
  pc = 0x30c1ee4; continue;
 }
 0x30c1ee4 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1f06; continue;
 }
 0x30c1f06 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c1f09; continue;
 }
 0x30c1f09 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1f4d; continue; }
  pc = 0x30c1f2b; continue;
 }
 0x30c1f2b => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1f4d; continue;
 }
 0x30c1f4d => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c1f50; continue;
 }
 0x30c1f50 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1f94; continue; }
  pc = 0x30c1f72; continue;
 }
 0x30c1f72 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1f94; continue;
 }
 0x30c1f94 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c1f97; continue;
 }
 0x30c1f97 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c1fdb; continue; }
  pc = 0x30c1fb9; continue;
 }
 0x30c1fb9 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c1fdb; continue;
 }
 0x30c1fdb => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c1fde; continue;
 }
 0x30c1fde => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2022; continue; }
  pc = 0x30c2000; continue;
 }
 0x30c2000 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c2022; continue;
 }
 0x30c2022 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c2025; continue;
 }
 0x30c2025 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2069; continue; }
  pc = 0x30c2047; continue;
 }
 0x30c2047 => {
  { let v: u16 = !(cpu.ecx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8420u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x328a244u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c2069; continue;
 }
 0x30c2069 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x30c206c; continue;
 }
 0x30c206c => {
  { let v: u32 = cpu.r32(0x3154e90u32).wrapping_add(0x4u32); cpu.w32(0x3154e90u32, v); }
  { let v: u32 = cpu.r32(0x3154e90u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c1c11; continue; }
  pc = 0x30c2083; continue;
 }
 0x30c2083 => {
  if cpu.zf { pc = 0x30c219d; continue; }
  pc = 0x30c2089; continue;
 }
 0x30c2089 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c1430u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c20c1; continue; }
  pc = 0x30c209d; continue;
 }
 0x30c209d => {
  { let v: u32 = cpu.r32(0x32c1448u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32c144cu32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 3u32); cpu.edx = v; }
  pc = 0x30c20ae; continue;
 }
 0x30c20ae => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x32c1430u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c20ae; continue; }
  pc = 0x30c20c1; continue;
 }
 0x30c20c1 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x3154eb4u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x30c20d2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c20ea; continue;
 }
 0x30c20ea => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c2102; continue;
 }
 0x30c2102 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c211a; continue;
 }
 0x30c211a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c2132; continue;
 }
 0x30c2132 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c214a; continue;
 }
 0x30c214a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c2162; continue;
 }
 0x30c2162 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x32c1448u32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x32c1448u32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x32c144cu32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x32c144cu32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c217a; continue;
 }
 0x30c217a => {
  { let v: u32 = cpu.r32(0x32c1430u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x3154e90u32).wrapping_add(0x4u32); cpu.w32(0x3154e90u32, v); }
  { let v: u32 = cpu.r32(0x3154e90u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c1c11; continue; }
  pc = 0x30c2197; continue;
 }
 0x30c2197 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c2089; continue; }
  pc = 0x30c219d; continue;
 }
 0x30c219d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x3155960u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32c143cu32)); cpu.w32(0x32c143cu32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c1b1e; continue; }
  pc = 0x30c21b4; continue;
 }
 0x30c21b4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c21b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c21b5); }
 let mut pc: u32 = 0x30c21b5;
 loop { match pc {
 0x30c21b5 => {
  { let v: u32 = cpu.r32(0x3155958u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x328c5acu32, v); }
  { let v: u32 = 0x328c5b4u32; cpu.esi = v; }
  { let v: u32 = 0x32919b4u32; cpu.edi = v; }
  { let v: u32 = 0x32c5f94u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3155968u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  pc = 0x30c21da; continue;
 }
 0x30c21da => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c21da; continue; }
  pc = 0x30c2269; continue;
 }
 0x30c2269 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7u32); cpu.eax = v; }
  if cpu.zf { pc = 0x30c2284; continue; }
  pc = 0x30c2270; continue;
 }
 0x30c2270 => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c2270; continue; }
  pc = 0x30c2284; continue;
 }
 0x30c2284 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c2285(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c2285); }
 let mut pc: u32 = 0x30c2285;
 loop { match pc {
 0x30c2285 => {
  { let v: u32 = 0x328c28cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c2aeb; continue; }
  pc = 0x30c229a; continue;
 }
 0x30c229a => {
  { let v: u32 = cpu.r32(0x4800u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c22ea; continue; }
  pc = 0x30c22a4; continue;
 }
 0x30c22a4 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22ad; continue;
 }
 0x30c22ad => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22b5; continue;
 }
 0x30c22b5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22bd; continue;
 }
 0x30c22bd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22c5; continue;
 }
 0x30c22c5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22cd; continue;
 }
 0x30c22cd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22d5; continue;
 }
 0x30c22d5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22dd; continue;
 }
 0x30c22dd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c22e5; continue;
 }
 0x30c22e5 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c22a4; continue;
 }
 0x30c22ea => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c22ee; continue;
 }
 0x30c22ee => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c22f5; continue;
 }
 0x30c22f5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c22fc; continue;
 }
 0x30c22fc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c2303; continue;
 }
 0x30c2303 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c230a; continue;
 }
 0x30c230a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c2311; continue;
 }
 0x30c2311 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c2318; continue;
 }
 0x30c2318 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2324; continue; }
  pc = 0x30c231f; continue;
 }
 0x30c231f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c22ea; continue;
 }
 0x30c2324 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c232c; continue; }
  pc = 0x30c2329; continue;
 }
 0x30c2329 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c232c; continue;
 }
 0x30c232c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2381; continue; }
  pc = 0x30c2330; continue;
 }
 0x30c2330 => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c2381 => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x9000u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2395; continue; }
  pc = 0x30c238f; continue;
 }
 0x30c238f => {
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  pc = 0x30c2395; continue;
 }
 0x30c2395 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 18u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x3296dc0u32)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x328c258u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x328c254u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30c23b5; continue;
 }
 0x30c23b5 => {
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x18u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c23d8; continue; }
  pc = 0x30c23c0; continue;
 }
 0x30c23c0 => {
  { let v: u32 = cpu.r32(0x328c254u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x8u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2535; continue; }
  pc = 0x30c23cf; continue;
 }
 0x30c23cf => {
  { cpu.sub32(cpu.ebp, cpu.r32(0xcu32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2535; continue; }
  pc = 0x30c23d8; continue;
 }
 0x30c23d8 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c2400; continue; }
  pc = 0x30c23dc; continue;
 }
 0x30c23dc => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c23ec; continue; }
  pc = 0x30c23e1; continue;
 }
 0x30c23e1 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  pc = 0x30c2aeb; continue;
 }
 0x30c23ec => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2a68; continue;
 }
 0x30c2400 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c249a; continue; }
  pc = 0x30c2409; continue;
 }
 0x30c2409 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2412; continue; }
  pc = 0x30c240d; continue;
 }
 0x30c240d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2412; continue;
 }
 0x30c2412 => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c249a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c24a3; continue; }
  pc = 0x30c249e; continue;
 }
 0x30c249e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c24a3; continue;
 }
 0x30c24a3 => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2a68; continue;
 }
 0x30c2535 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c28a3; continue; }
  pc = 0x30c253d; continue;
 }
 0x30c253d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c275f; continue; }
  pc = 0x30c2546; continue;
 }
 0x30c2546 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x30c2620; continue; }
  pc = 0x30c254e; continue;
 }
 0x30c254e => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x328c24cu32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x30c2590; continue; }
  pc = 0x30c255e; continue;
 }
 0x30c255e => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c24cu32, v); cpu.fpop(); }
  pc = 0x30c2590; continue;
 }
 0x30c2590 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c25eb; continue; }
  pc = 0x30c2595; continue;
 }
 0x30c2595 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c248u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c24cu32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c25e6; continue; }
  pc = 0x30c25cd; continue;
 }
 0x30c25cd => {
  { let v: u32 = cpu.r32(0x328c248u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c25e6; continue; }
  pc = 0x30c25e1; continue;
 }
 0x30c25e1 => {
  cpu.poll(); pc = 0x30c23e1; continue;
 }
 0x30c25e6 => {
  pc = 0x30c292f; continue;
 }
 0x30c25eb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c24cu32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2612; continue; }
  pc = 0x30c25fc; continue;
 }
 0x30c25fc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2612; continue; }
  pc = 0x30c260d; continue;
 }
 0x30c260d => {
  cpu.poll(); pc = 0x30c23e1; continue;
 }
 0x30c2612 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c261b; continue; }
  pc = 0x30c2616; continue;
 }
 0x30c2616 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c261b; continue;
 }
 0x30c261b => {
  pc = 0x30c2739; continue;
 }
 0x30c2620 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c244u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c248u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c244u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x3c00u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2691; continue; }
  pc = 0x30c2672; continue;
 }
 0x30c2672 => {
  { let v: u32 = cpu.r32(0x328c248u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2691; continue; }
  pc = 0x30c2686; continue;
 }
 0x30c2686 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  pc = 0x30c2aeb; continue;
 }
 0x30c2691 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c269a; continue; }
  pc = 0x30c2695; continue;
 }
 0x30c2695 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c269a; continue;
 }
 0x30c269a => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x328c244u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c2739; continue; }
  pc = 0x30c26da; continue;
 }
 0x30c26da => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x328c248u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c2739 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c275f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x30c2808; continue; }
  pc = 0x30c2767; continue;
 }
 0x30c2767 => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x328c24cu32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x30c27a9; continue; }
  pc = 0x30c2777; continue;
 }
 0x30c2777 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c24cu32, v); cpu.fpop(); }
  pc = 0x30c27a9; continue;
 }
 0x30c27a9 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c250u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c24cu32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2803; continue; }
  pc = 0x30c27ed; continue;
 }
 0x30c27ed => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c250u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2803; continue; }
  pc = 0x30c27fe; continue;
 }
 0x30c27fe => {
  cpu.poll(); pc = 0x30c23ec; continue;
 }
 0x30c2803 => {
  pc = 0x30c2a5c; continue;
 }
 0x30c2808 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c244u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c250u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c244u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x3c00u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c288c; continue; }
  pc = 0x30c2876; continue;
 }
 0x30c2876 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c250u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c288c; continue; }
  pc = 0x30c2887; continue;
 }
 0x30c2887 => {
  cpu.poll(); pc = 0x30c23ec; continue;
 }
 0x30c288c => {
  { let v: u32 = cpu.r32(0x328c244u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2a68; continue;
 }
 0x30c28a3 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c29e9; continue; }
  pc = 0x30c28ac; continue;
 }
 0x30c28ac => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c24cu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c248u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c24cu32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c292f; continue; }
  pc = 0x30c2916; continue;
 }
 0x30c2916 => {
  { let v: u32 = cpu.r32(0x328c248u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c292f; continue; }
  pc = 0x30c292a; continue;
 }
 0x30c292a => {
  cpu.poll(); pc = 0x30c2409; continue;
 }
 0x30c292f => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2938; continue; }
  pc = 0x30c2933; continue;
 }
 0x30c2933 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2938; continue;
 }
 0x30c2938 => {
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c29c8; continue; }
  pc = 0x30c29ab; continue;
 }
 0x30c29ab => {
  { let v: u32 = cpu.r32(0x328c248u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c29c8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c29e9 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c24cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c250u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c24cu32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2a5c; continue; }
  pc = 0x30c2a46; continue;
 }
 0x30c2a46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x328c258u32)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c250u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2a5c; continue; }
  pc = 0x30c2a57; continue;
 }
 0x30c2a57 => {
  cpu.poll(); pc = 0x30c249a; continue;
 }
 0x30c2a5c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x30c2a68; continue;
 }
 0x30c2a68 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c23b5; continue; }
  pc = 0x30c2a70; continue;
 }
 0x30c2a70 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2a79; continue; }
  pc = 0x30c2a74; continue;
 }
 0x30c2a74 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2a79; continue;
 }
 0x30c2a79 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x328c5acu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x328c5acu32).wrapping_add(0x1cu32); cpu.w32(0x328c5acu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ecx), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4800u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2aeb; continue;
 }
 0x30c2aeb => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edi, 0x328c28cu32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c2af8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c2af8); }
 let mut pc: u32 = 0x30c2af8;
 loop { match pc {
 0x30c2af8 => {
  { let v: u32 = 0x328c28cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c2ffa; continue; }
  pc = 0x30c2b0d; continue;
 }
 0x30c2b0d => {
  { let v: u32 = cpu.r32(0x4800u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c2b5d; continue; }
  pc = 0x30c2b17; continue;
 }
 0x30c2b17 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b20; continue;
 }
 0x30c2b20 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b28; continue;
 }
 0x30c2b28 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b30; continue;
 }
 0x30c2b30 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b38; continue;
 }
 0x30c2b38 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b40; continue;
 }
 0x30c2b40 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b48; continue;
 }
 0x30c2b48 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b50; continue;
 }
 0x30c2b50 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b58; continue;
 }
 0x30c2b58 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c2b17; continue;
 }
 0x30c2b5d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b61; continue;
 }
 0x30c2b61 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b68; continue;
 }
 0x30c2b68 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b6f; continue;
 }
 0x30c2b6f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b76; continue;
 }
 0x30c2b76 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b7d; continue;
 }
 0x30c2b7d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b84; continue;
 }
 0x30c2b84 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b8b; continue;
 }
 0x30c2b8b => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2b97; continue; }
  pc = 0x30c2b92; continue;
 }
 0x30c2b92 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c2b5d; continue;
 }
 0x30c2b97 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c2b9f; continue; }
  pc = 0x30c2b9c; continue;
 }
 0x30c2b9c => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2b9f; continue;
 }
 0x30c2b9f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c2bb2; continue; }
  pc = 0x30c2ba3; continue;
 }
 0x30c2ba3 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2bb2 => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x9000u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2bc6; continue; }
  pc = 0x30c2bc0; continue;
 }
 0x30c2bc0 => {
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  pc = 0x30c2bc6; continue;
 }
 0x30c2bc6 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x328c26cu32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30c2bce; continue;
 }
 0x30c2bce => {
  { let v: u32 = cpu.r32(0x328c5b0u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x18u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c2be9; continue; }
  pc = 0x30c2bd9; continue;
 }
 0x30c2bd9 => {
  { let v: u32 = cpu.r32(0x328c26cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x8u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2c51; continue; }
  pc = 0x30c2be4; continue;
 }
 0x30c2be4 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0xcu32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2c51; continue; }
  pc = 0x30c2be9; continue;
 }
 0x30c2be9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c2c0b; continue; }
  pc = 0x30c2bed; continue;
 }
 0x30c2bed => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2bf7; continue; }
  pc = 0x30c2bf2; continue;
 }
 0x30c2bf2 => {
  pc = 0x30c2ffa; continue;
 }
 0x30c2bf7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2fdf; continue;
 }
 0x30c2c0b => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2c29; continue; }
  pc = 0x30c2c10; continue;
 }
 0x30c2c10 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2c19; continue; }
  pc = 0x30c2c14; continue;
 }
 0x30c2c14 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2c19; continue;
 }
 0x30c2c19 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2c29 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2c32; continue; }
  pc = 0x30c2c2d; continue;
 }
 0x30c2c2d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2c32; continue;
 }
 0x30c2c32 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2fdf; continue;
 }
 0x30c2c51 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c2ec9; continue; }
  pc = 0x30c2c59; continue;
 }
 0x30c2c59 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2dac; continue; }
  pc = 0x30c2c62; continue;
 }
 0x30c2c62 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x30c2d21; continue; }
  pc = 0x30c2c6a; continue;
 }
 0x30c2c6a => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x328c264u32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x30c2cac; continue; }
  pc = 0x30c2c7a; continue;
 }
 0x30c2c7a => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c264u32, v); cpu.fpop(); }
  pc = 0x30c2cac; continue;
 }
 0x30c2cac => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c2cfb; continue; }
  pc = 0x30c2cb1; continue;
 }
 0x30c2cb1 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c260u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c264u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2cf6; continue; }
  pc = 0x30c2ce3; continue;
 }
 0x30c2ce3 => {
  { let v: u32 = cpu.r32(0x328c260u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2cf6; continue; }
  pc = 0x30c2cf1; continue;
 }
 0x30c2cf1 => {
  cpu.poll(); pc = 0x30c2bf2; continue;
 }
 0x30c2cf6 => {
  pc = 0x30c2f49; continue;
 }
 0x30c2cfb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c264u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2d16; continue; }
  pc = 0x30c2d06; continue;
 }
 0x30c2d06 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2d16; continue; }
  pc = 0x30c2d11; continue;
 }
 0x30c2d11 => {
  cpu.poll(); pc = 0x30c2bf2; continue;
 }
 0x30c2d16 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2d1f; continue; }
  pc = 0x30c2d1a; continue;
 }
 0x30c2d1a => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2d1f; continue;
 }
 0x30c2d1f => {
  pc = 0x30c2d9d; continue;
 }
 0x30c2d21 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c25cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c260u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c25cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x3c00u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2d80; continue; }
  pc = 0x30c2d6d; continue;
 }
 0x30c2d6d => {
  { let v: u32 = cpu.r32(0x328c260u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2d80; continue; }
  pc = 0x30c2d7b; continue;
 }
 0x30c2d7b => {
  pc = 0x30c2ffa; continue;
 }
 0x30c2d80 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2d89; continue; }
  pc = 0x30c2d84; continue;
 }
 0x30c2d84 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2d89; continue;
 }
 0x30c2d89 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c2d9d; continue; }
  pc = 0x30c2d8e; continue;
 }
 0x30c2d8e => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2d9d => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2dac => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x30c2e49; continue; }
  pc = 0x30c2db4; continue;
 }
 0x30c2db4 => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x328c264u32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); }
  if cpu.zf { pc = 0x30c2df6; continue; }
  pc = 0x30c2dc4; continue;
 }
 0x30c2dc4 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c264u32, v); cpu.fpop(); }
  pc = 0x30c2df6; continue;
 }
 0x30c2df6 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c268u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c264u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2e44; continue; }
  pc = 0x30c2e34; continue;
 }
 0x30c2e34 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c268u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2e44; continue; }
  pc = 0x30c2e3f; continue;
 }
 0x30c2e3f => {
  cpu.poll(); pc = 0x30c2bf7; continue;
 }
 0x30c2e44 => {
  pc = 0x30c2fdc; continue;
 }
 0x30c2e49 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c25cu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c268u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x328c25cu32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x3c00u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2ec1; continue; }
  pc = 0x30c2eb1; continue;
 }
 0x30c2eb1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c268u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2ec1; continue; }
  pc = 0x30c2ebc; continue;
 }
 0x30c2ebc => {
  cpu.poll(); pc = 0x30c2bf7; continue;
 }
 0x30c2ec1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2fdf; continue;
 }
 0x30c2ec9 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2f75; continue; }
  pc = 0x30c2ed2; continue;
 }
 0x30c2ed2 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c264u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebx); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c260u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c264u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2f49; continue; }
  pc = 0x30c2f36; continue;
 }
 0x30c2f36 => {
  { let v: u32 = cpu.r32(0x328c260u32); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x9000u32.wrapping_add(cpu.esi)), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2f49; continue; }
  pc = 0x30c2f44; continue;
 }
 0x30c2f44 => {
  cpu.poll(); pc = 0x30c2c10; continue;
 }
 0x30c2f49 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2f52; continue; }
  pc = 0x30c2f4d; continue;
 }
 0x30c2f4d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2f52; continue;
 }
 0x30c2f52 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x30c2f66; continue; }
  pc = 0x30c2f57; continue;
 }
 0x30c2f57 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2f66 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2f75 => {
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c264u32, v); cpu.fpop(); }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x9000u32.wrapping_add(cpu.esi)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x328c268u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c264u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2fdc; continue; }
  pc = 0x30c2fcc; continue;
 }
 0x30c2fcc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x328c268u32), cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c2fdc; continue; }
  pc = 0x30c2fd7; continue;
 }
 0x30c2fd7 => {
  cpu.poll(); pc = 0x30c2c29; continue;
 }
 0x30c2fdc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c2fdf; continue;
 }
 0x30c2fdf => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c2bce; continue; }
  pc = 0x30c2fe7; continue;
 }
 0x30c2fe7 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c2ff0; continue; }
  pc = 0x30c2feb; continue;
 }
 0x30c2feb => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ff0; continue;
 }
 0x30c2ff0 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c2ffa; continue;
 }
 0x30c2ffa => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edi, 0x328c28cu32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3007(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3007); }
 let mut pc: u32 = 0x30c3007;
 loop { match pc {
 0x30c3007 => {
  { let v: u32 = 0x328c28cu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c350b; continue; }
  pc = 0x30c301c; continue;
 }
 0x30c301c => {
  { let v: u32 = cpu.r32(0x4800u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c306c; continue; }
  pc = 0x30c3026; continue;
 }
 0x30c3026 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c302f; continue;
 }
 0x30c302f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c3037; continue;
 }
 0x30c3037 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c303f; continue;
 }
 0x30c303f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c3047; continue;
 }
 0x30c3047 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c304f; continue;
 }
 0x30c304f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c3057; continue;
 }
 0x30c3057 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c305f; continue;
 }
 0x30c305f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c3067; continue;
 }
 0x30c3067 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c3026; continue;
 }
 0x30c306c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c3070; continue;
 }
 0x30c3070 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c3077; continue;
 }
 0x30c3077 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c307e; continue;
 }
 0x30c307e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c3085; continue;
 }
 0x30c3085 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c308c; continue;
 }
 0x30c308c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c3093; continue;
 }
 0x30c3093 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c309a; continue;
 }
 0x30c309a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30a6; continue; }
  pc = 0x30c30a1; continue;
 }
 0x30c30a1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c306c; continue;
 }
 0x30c30a6 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c30ae; continue; }
  pc = 0x30c30ab; continue;
 }
 0x30c30ab => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c30ae; continue;
 }
 0x30c30ae => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c30c1; continue; }
  pc = 0x30c30b2; continue;
 }
 0x30c30b2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c350b; continue;
 }
 0x30c30c1 => {
  { let v: u32 = cpu.r32(0x3c00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x9000u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c30d5; continue; }
  pc = 0x30c30cf; continue;
 }
 0x30c30cf => {
  { let v: u32 = cpu.r32(0x9000u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  pc = 0x30c30d5; continue;
 }
 0x30c30d5 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x328c284u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30c30dd; continue;
 }
 0x30c30dd => {
  pc = 0x30c30fa; continue;
 }
 0x30c30fa => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x30c311c; continue; }
  pc = 0x30c30fe; continue;
 }
 0x30c30fe => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c3108; continue; }
  pc = 0x30c3103; continue;
 }
 0x30c3103 => {
  pc = 0x30c350b; continue;
 }
 0x30c3108 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c34f0; continue;
 }
 0x30c311c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c313a; continue; }
  pc = 0x30c3121; continue;
 }
 0x30c3121 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c312a; continue; }
  pc = 0x30c3125; continue;
 }
 0x30c3125 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c312a; continue;
 }
 0x30c312a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c350b; continue;
 }
 0x30c313a => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c3143; continue; }
  pc = 0x30c313e; continue;
 }
 0x30c313e => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c3143; continue;
 }
 0x30c3143 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30c34f0; continue;
 }
 0x30c34f0 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30c30dd; continue; }
  pc = 0x30c34f8; continue;
 }
 0x30c34f8 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c3501; continue; }
  pc = 0x30c34fc; continue;
 }
 0x30c34fc => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c3501; continue;
 }
 0x30c3501 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x30c350b; continue;
 }
 0x30c350b => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edi, 0x328c28cu32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3518(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3518); }
 let mut pc: u32 = 0x30c3518;
 loop { match pc {
 0x30c3518 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32c0914u32, v); }
  cpu.push(0x30c3523); f_030c3525(cpu); if cpu.returned_to != 0x30c3523 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3525(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3525); }
 let mut pc: u32 = 0x30c3525;
 loop { match pc {
 0x30c3525 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x31559f8u32)); cpu.w32(0x31559f8u32, v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x31559f8u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3155a10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x3155a04u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x30c3555; continue; }
  pc = 0x30c354a; continue;
 }
 0x30c354a => {
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.ebx = v; }
  cpu.push(0x30c3555); f_030c5610(cpu); if cpu.returned_to != 0x30c3555 { return; }
  pc = 0x30c3555; continue;
 }
 0x30c3555 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3b4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c355c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c355c); }
 let mut pc: u32 = 0x30c355c;
 loop { match pc {
 0x30c355c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155964u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c3634; continue; }
  pc = 0x30c356e; continue;
 }
 0x30c356e => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155968u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c3634; continue; }
  pc = 0x30c357a; continue;
 }
 0x30c357a => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x315598cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3634; continue; }
  pc = 0x30c3586; continue;
 }
 0x30c3586 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x32a2dd4u32, v); }
  { let v = (cpu.r32(0x32a2dd4u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x32c5f94u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edx = v; }
  { let v = cpu.rf32(0x32a2ddcu32) / cpu.st(0); cpu.set_st(0, v); }
  pc = 0x30c359f; continue;
 }
 0x30c359f => {
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c35c1; continue; }
  pc = 0x30c35a4; continue;
 }
 0x30c35a4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c35c1; continue; }
  pc = 0x30c35ac; continue;
 }
 0x30c35ac => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c35c1; continue; }
  pc = 0x30c35b4; continue;
 }
 0x30c35b4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c35c1; continue; }
  pc = 0x30c35bc; continue;
 }
 0x30c35bc => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c359f; continue;
 }
 0x30c35c1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c35df; continue; }
  pc = 0x30c35c5; continue;
 }
 0x30c35c5 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c35df; continue; }
  pc = 0x30c35cc; continue;
 }
 0x30c35cc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c35df; continue; }
  pc = 0x30c35d3; continue;
 }
 0x30c35d3 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30c35df; continue; }
  pc = 0x30c35da; continue;
 }
 0x30c35da => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x30c35c1; continue;
 }
 0x30c35df => {
  { let v = cpu.st(0); cpu.wf32(0x32a2dd4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32a2dd4u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.edx))); }
  { let v: u8 = ((!cpu.zf && cpu.sf == cpu.of)) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, cpu.r32(0xcu32.wrapping_add(cpu.edx))); }
  { let v: u8 = ((!cpu.zf && cpu.sf == cpu.of)) as u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x30c3603; continue; }
  pc = 0x30c35fb; continue;
 }
 0x30c35fb => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c3634; continue; }
  pc = 0x30c35ff; continue;
 }
 0x30c35ff => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edi); cpu.edi = v; }
  pc = 0x30c3634; continue;
 }
 0x30c3603 => {
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a3dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v = cpu.st(0) * cpu.rf32(0x32a2dd8u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32a2dd4u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.ecx, cpu.r32(0x32a2dd4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c3634; continue; }
  pc = 0x30c3632; continue;
 }
 0x30c3632 => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edi); cpu.edi = v; }
  pc = 0x30c3634; continue;
 }
 0x30c3634 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3638); }
 let mut pc: u32 = 0x30c3638;
 loop { match pc {
 0x30c3638 => {
  { cpu.pushad(); }
  cpu.push(0x30c363e); f_030c3697(cpu); if cpu.returned_to != 0x30c363e { return; }
  cpu.push(0x30c3643); f_030c379e(cpu); if cpu.returned_to != 0x30c3643 { return; }
  { let v: u32 = 0x3154a6cu32; cpu.esi = v; }
  { let v: u32 = 0x3154a1cu32; cpu.edi = v; }
  { let v: u32 = 0x3154a6cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.sub32(cpu.r32(0x3155954u32), 0xfu32); }
  if !cpu.zf { pc = 0x30c3680; continue; }
  pc = 0x30c3662; continue;
 }
 0x30c3662 => {
  { let v: u32 = 0x30c1160u32; cpu.w32(0x3154a24u32, v); }
  { let v: u32 = 0x30c1320u32; cpu.w32(0x3154a4cu32, v); }
  { let v: u32 = 0x30c1adeu32; cpu.w32(0x3154a50u32, v); }
  pc = 0x30c3680; continue;
 }
 0x30c3680 => {
  { cpu.sub32(cpu.r32(0x3155954u32), 0xfu32); }
  if !cpu.zf { pc = 0x30c3690; continue; }
  pc = 0x30c3689; continue;
 }
 0x30c3689 => {
  cpu.push(0x30c368e); f_030c10a7(cpu); if cpu.returned_to != 0x30c368e { return; }
  pc = 0x30c3695; continue;
 }
 0x30c3690 => {
  cpu.push(0x30c3695); f_030c0fee(cpu); if cpu.returned_to != 0x30c3695 { return; }
  pc = 0x30c3695; continue;
 }
 0x30c3695 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3697(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3697); }
 let mut pc: u32 = 0x30c3697;
 loop { match pc {
 0x30c3697 => {
  { let v: u32 = cpu.r32(0x3155958u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x328c5acu32, v); }
  { let v: u32 = 0x328c5b4u32; cpu.esi = v; }
  { let v: u32 = 0x32919b4u32; cpu.edi = v; }
  { let v: u32 = 0x32c5f94u32; cpu.edx = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = 0x80000001u32; cpu.ebx = v; }
  pc = 0x30c36bf; continue;
 }
 0x30c36bf => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c36bf; continue; }
  pc = 0x30c36f9; continue;
 }
 0x30c36f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c36fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c36fa); }
 let mut pc: u32 = 0x30c36fa;
 loop { match pc {
 0x30c36fa => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.w32(0x3154ed4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3154ed8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x3154edcu32, v); }
  cpu.push(0x30c3711); f_030c3713(cpu); if cpu.returned_to != 0x30c3711 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3713(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3713); }
 let mut pc: u32 = 0x30c3713;
 loop { match pc {
 0x30c3713 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.r32(0x3154ed4u32); cpu.eax = v; }
  { let v: u32 = 0x3296dc4u32; cpu.edi = v; }
  cpu.push(0x30c3723); f_030c374d(cpu); if cpu.returned_to != 0x30c3723 { return; }
  { let v: u32 = cpu.r32(0x3154ed8u32); cpu.eax = v; }
  { let v: u32 = 0x329adc8u32; cpu.edi = v; }
  cpu.push(0x30c3732); f_030c374d(cpu); if cpu.returned_to != 0x30c3732 { return; }
  { let v: u32 = cpu.r32(0x3154edcu32); cpu.eax = v; }
  { let v: u32 = 0x329edccu32; cpu.edi = v; }
  cpu.push(0x30c3741); f_030c374d(cpu); if cpu.returned_to != 0x30c3741 { return; }
  { let v: u32 = 0x329edccu32; cpu.w32(0x3296dc0u32, v); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c374d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c374d); }
 let mut pc: u32 = 0x30c374d;
 loop { match pc {
 0x30c374d => {
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3296db4u32, v); }
  { let v = (cpu.r32(0x3296db4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x3296db4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30c3763; continue;
 }
 0x30c3763 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 18u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3296db8u32, v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x3296db8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3296db4u32); cpu.set_st(0, v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x3296dbcu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x3296dbcu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x3296db8u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1001u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c3763; continue; }
  pc = 0x30c379d; continue;
 }
 0x30c379d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c379e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c379e); }
 let mut pc: u32 = 0x30c379e;
 loop { match pc {
 0x30c379e => {
  { cpu.pushad(); }
  { let v: u32 = 0x32a2dd8u32; cpu.ebx = v; }
  { let v: u32 = 0x32a3dd8u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32a2dd0u32, v); }
  pc = 0x30c37b1; continue;
 }
 0x30c37b1 => {
  { let v = (cpu.r32(0x32a2dd0u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30c37cc; continue; }
  pc = 0x30c37c0; continue;
 }
 0x30c37c0 => {
  { cpu.fpush(1.0); }
  { let v = (cpu.r32(0x32a2dd0u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x30c37cc; continue;
 }
 0x30c37cc => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x32a2dd0u32)); cpu.w32(0x32a2dd0u32, v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x400u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c37b1; continue; }
  pc = 0x30c37de; continue;
 }
 0x30c37de => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c37e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c37e0); }
 let mut pc: u32 = 0x30c37e0;
 loop { match pc {
 0x30c37e0 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c37f8; continue;
 }
 0x30c37f8 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c37f8; continue; }
  pc = 0x30c3829; continue;
 }
 0x30c3829 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c3838); cpu.call(t); if cpu.returned_to != 0x30c3838 { return; }
  let t = cpu.r32(0x3154a20u32); cpu.push(0x30c383e); cpu.call(t); if cpu.returned_to != 0x30c383e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c383f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c383f); }
 let mut pc: u32 = 0x30c383f;
 loop { match pc {
 0x30c383f => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3857; continue;
 }
 0x30c3857 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3857; continue; }
  pc = 0x30c3888; continue;
 }
 0x30c3888 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3896); f_030c7611(cpu); if cpu.returned_to != 0x30c3896 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c38a4; continue; }
  pc = 0x30c3898; continue;
 }
 0x30c3898 => {
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c389e); cpu.call(t); if cpu.returned_to != 0x30c389e { return; }
  let t = cpu.r32(0x3154a20u32); cpu.push(0x30c38a4); cpu.call(t); if cpu.returned_to != 0x30c38a4 { return; }
  pc = 0x30c38a4; continue;
 }
 0x30c38a4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c38a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c38a5); }
 let mut pc: u32 = 0x30c38a5;
 loop { match pc {
 0x30c38a5 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c38bd; continue;
 }
 0x30c38bd => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c38bd; continue; }
  pc = 0x30c38e5; continue;
 }
 0x30c38e5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c38f3); f_030c71d0(cpu); if cpu.returned_to != 0x30c38f3 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3901; continue; }
  pc = 0x30c38f5; continue;
 }
 0x30c38f5 => {
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c38fb); cpu.call(t); if cpu.returned_to != 0x30c38fb { return; }
  let t = cpu.r32(0x3154a20u32); cpu.push(0x30c3901); cpu.call(t); if cpu.returned_to != 0x30c3901 { return; }
  pc = 0x30c3901; continue;
 }
 0x30c3901 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3902(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3902); }
 let mut pc: u32 = 0x30c3902;
 loop { match pc {
 0x30c3902 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c391a; continue;
 }
 0x30c391a => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c391a; continue; }
  pc = 0x30c3942; continue;
 }
 0x30c3942 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3950); f_030c71d0(cpu); if cpu.returned_to != 0x30c3950 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3965; continue; }
  pc = 0x30c3952; continue;
 }
 0x30c3952 => {
  cpu.push(0x30c3957); f_030c7611(cpu); if cpu.returned_to != 0x30c3957 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3965; continue; }
  pc = 0x30c3959; continue;
 }
 0x30c3959 => {
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c395f); cpu.call(t); if cpu.returned_to != 0x30c395f { return; }
  let t = cpu.r32(0x3154a20u32); cpu.push(0x30c3965); cpu.call(t); if cpu.returned_to != 0x30c3965 { return; }
  pc = 0x30c3965; continue;
 }
 0x30c3965 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3966(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3966); }
 let mut pc: u32 = 0x30c3966;
 loop { match pc {
 0x30c3966 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c397e; continue;
 }
 0x30c397e => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c397e; continue; }
  pc = 0x30c39af; continue;
 }
 0x30c39af => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c39b6; continue;
 }
 0x30c39b6 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c39b6; continue; }
  pc = 0x30c39c4; continue;
 }
 0x30c39c4 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x32c13a4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3b0u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  let t = cpu.r32(0x3154a28u32); cpu.push(0x30c39e4); cpu.call(t); if cpu.returned_to != 0x30c39e4 { return; }
  let t = cpu.r32(0x3154a2cu32); cpu.push(0x30c39ea); cpu.call(t); if cpu.returned_to != 0x30c39ea { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c39eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c39eb); }
 let mut pc: u32 = 0x30c39eb;
 loop { match pc {
 0x30c39eb => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3a03; continue;
 }
 0x30c3a03 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3a03; continue; }
  pc = 0x30c3a34; continue;
 }
 0x30c3a34 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3a3b; continue;
 }
 0x30c3a3b => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3a3b; continue; }
  pc = 0x30c3a49; continue;
 }
 0x30c3a49 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x32c13a4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3b0u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3a68); f_030c786f(cpu); if cpu.returned_to != 0x30c3a68 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3a76; continue; }
  pc = 0x30c3a6a; continue;
 }
 0x30c3a6a => {
  let t = cpu.r32(0x3154a28u32); cpu.push(0x30c3a70); cpu.call(t); if cpu.returned_to != 0x30c3a70 { return; }
  let t = cpu.r32(0x3154a2cu32); cpu.push(0x30c3a76); cpu.call(t); if cpu.returned_to != 0x30c3a76 { return; }
  pc = 0x30c3a76; continue;
 }
 0x30c3a76 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3a77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3a77); }
 let mut pc: u32 = 0x30c3a77;
 loop { match pc {
 0x30c3a77 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3a8f; continue;
 }
 0x30c3a8f => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3a8f; continue; }
  pc = 0x30c3ab7; continue;
 }
 0x30c3ab7 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3abe; continue;
 }
 0x30c3abe => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3abe; continue; }
  pc = 0x30c3acc; continue;
 }
 0x30c3acc => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x32c13a4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3b0u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3aeb); f_030c732d(cpu); if cpu.returned_to != 0x30c3aeb { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3af9; continue; }
  pc = 0x30c3aed; continue;
 }
 0x30c3aed => {
  let t = cpu.r32(0x3154a28u32); cpu.push(0x30c3af3); cpu.call(t); if cpu.returned_to != 0x30c3af3 { return; }
  let t = cpu.r32(0x3154a2cu32); cpu.push(0x30c3af9); cpu.call(t); if cpu.returned_to != 0x30c3af9 { return; }
  pc = 0x30c3af9; continue;
 }
 0x30c3af9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3afa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3afa); }
 let mut pc: u32 = 0x30c3afa;
 loop { match pc {
 0x30c3afa => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3b12; continue;
 }
 0x30c3b12 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3b12; continue; }
  pc = 0x30c3b3a; continue;
 }
 0x30c3b3a => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3b41; continue;
 }
 0x30c3b41 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3b41; continue; }
  pc = 0x30c3b4f; continue;
 }
 0x30c3b4f => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x32c13a4u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3b0u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3b6e); f_030c732d(cpu); if cpu.returned_to != 0x30c3b6e { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3b83; continue; }
  pc = 0x30c3b70; continue;
 }
 0x30c3b70 => {
  cpu.push(0x30c3b75); f_030c786f(cpu); if cpu.returned_to != 0x30c3b75 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3b83; continue; }
  pc = 0x30c3b77; continue;
 }
 0x30c3b77 => {
  let t = cpu.r32(0x3154a28u32); cpu.push(0x30c3b7d); cpu.call(t); if cpu.returned_to != 0x30c3b7d { return; }
  let t = cpu.r32(0x3154a2cu32); cpu.push(0x30c3b83); cpu.call(t); if cpu.returned_to != 0x30c3b83 { return; }
  pc = 0x30c3b83; continue;
 }
 0x30c3b83 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3b84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3b84); }
 let mut pc: u32 = 0x30c3b84;
 loop { match pc {
 0x30c3b84 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3b9c; continue;
 }
 0x30c3b9c => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3b9c; continue; }
  pc = 0x30c3bcd; continue;
 }
 0x30c3bcd => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c3bd6; continue;
 }
 0x30c3bd6 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3bd6; continue; }
  pc = 0x30c3bea; continue;
 }
 0x30c3bea => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3c06); cpu.call(t); if cpu.returned_to != 0x30c3c06 { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3c0c); cpu.call(t); if cpu.returned_to != 0x30c3c0c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3c0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3c0d); }
 let mut pc: u32 = 0x30c3c0d;
 loop { match pc {
 0x30c3c0d => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3c25; continue;
 }
 0x30c3c25 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3c25; continue; }
  pc = 0x30c3c56; continue;
 }
 0x30c3c56 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c3c5f; continue;
 }
 0x30c3c5f => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3c5f; continue; }
  pc = 0x30c3c73; continue;
 }
 0x30c3c73 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3c8e); f_030c7ae9(cpu); if cpu.returned_to != 0x30c3c8e { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3c9c; continue; }
  pc = 0x30c3c90; continue;
 }
 0x30c3c90 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3c96); cpu.call(t); if cpu.returned_to != 0x30c3c96 { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3c9c); cpu.call(t); if cpu.returned_to != 0x30c3c9c { return; }
  pc = 0x30c3c9c; continue;
 }
 0x30c3c9c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3c9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3c9d); }
 let mut pc: u32 = 0x30c3c9d;
 loop { match pc {
 0x30c3c9d => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3cb5; continue;
 }
 0x30c3cb5 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3cb5; continue; }
  pc = 0x30c3cdd; continue;
 }
 0x30c3cdd => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c3ce6; continue;
 }
 0x30c3ce6 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3ce6; continue; }
  pc = 0x30c3cfa; continue;
 }
 0x30c3cfa => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3d15); f_030c7498(cpu); if cpu.returned_to != 0x30c3d15 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3d23; continue; }
  pc = 0x30c3d17; continue;
 }
 0x30c3d17 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3d1d); cpu.call(t); if cpu.returned_to != 0x30c3d1d { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3d23); cpu.call(t); if cpu.returned_to != 0x30c3d23 { return; }
  pc = 0x30c3d23; continue;
 }
 0x30c3d23 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3d24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3d24); }
 let mut pc: u32 = 0x30c3d24;
 loop { match pc {
 0x30c3d24 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3d3c; continue;
 }
 0x30c3d3c => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3d3c; continue; }
  pc = 0x30c3d64; continue;
 }
 0x30c3d64 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c3d6d; continue;
 }
 0x30c3d6d => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3d6d; continue; }
  pc = 0x30c3d81; continue;
 }
 0x30c3d81 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3d9c); f_030c7498(cpu); if cpu.returned_to != 0x30c3d9c { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3db1; continue; }
  pc = 0x30c3d9e; continue;
 }
 0x30c3d9e => {
  cpu.push(0x30c3da3); f_030c7ae9(cpu); if cpu.returned_to != 0x30c3da3 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3db1; continue; }
  pc = 0x30c3da5; continue;
 }
 0x30c3da5 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3dab); cpu.call(t); if cpu.returned_to != 0x30c3dab { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3db1); cpu.call(t); if cpu.returned_to != 0x30c3db1 { return; }
  pc = 0x30c3db1; continue;
 }
 0x30c3db1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3db2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3db2); }
 let mut pc: u32 = 0x30c3db2;
 loop { match pc {
 0x30c3db2 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3dca; continue;
 }
 0x30c3dca => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3dca; continue; }
  pc = 0x30c3df2; continue;
 }
 0x30c3df2 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x32c0930u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3dfd; continue;
 }
 0x30c3dfd => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x325eda8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3dfd; continue; }
  pc = 0x30c3e18; continue;
 }
 0x30c3e18 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3e31); f_030c52a4(cpu); if cpu.returned_to != 0x30c3e31 { return; }
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3e37); cpu.call(t); if cpu.returned_to != 0x30c3e37 { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3e3d); cpu.call(t); if cpu.returned_to != 0x30c3e3d { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3e3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3e3e); }
 let mut pc: u32 = 0x30c3e3e;
 loop { match pc {
 0x30c3e3e => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3e56; continue;
 }
 0x30c3e56 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3e56; continue; }
  pc = 0x30c3e7e; continue;
 }
 0x30c3e7e => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x32c0930u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3e89; continue;
 }
 0x30c3e89 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x325eda8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3e89; continue; }
  pc = 0x30c3ea4; continue;
 }
 0x30c3ea4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3ebd); f_030c52a4(cpu); if cpu.returned_to != 0x30c3ebd { return; }
  cpu.push(0x30c3ec2); f_030c7ae9(cpu); if cpu.returned_to != 0x30c3ec2 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3ed0; continue; }
  pc = 0x30c3ec4; continue;
 }
 0x30c3ec4 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3eca); cpu.call(t); if cpu.returned_to != 0x30c3eca { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3ed0); cpu.call(t); if cpu.returned_to != 0x30c3ed0 { return; }
  pc = 0x30c3ed0; continue;
 }
 0x30c3ed0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3ed1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3ed1); }
 let mut pc: u32 = 0x30c3ed1;
 loop { match pc {
 0x30c3ed1 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3ee9; continue;
 }
 0x30c3ee9 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3ee9; continue; }
  pc = 0x30c3f11; continue;
 }
 0x30c3f11 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x32c0930u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3f1c; continue;
 }
 0x30c3f1c => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x325eda8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3f1c; continue; }
  pc = 0x30c3f37; continue;
 }
 0x30c3f37 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3f50); f_030c52a4(cpu); if cpu.returned_to != 0x30c3f50 { return; }
  cpu.push(0x30c3f55); f_030c7498(cpu); if cpu.returned_to != 0x30c3f55 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3f63; continue; }
  pc = 0x30c3f57; continue;
 }
 0x30c3f57 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3f5d); cpu.call(t); if cpu.returned_to != 0x30c3f5d { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3f63); cpu.call(t); if cpu.returned_to != 0x30c3f63 { return; }
  pc = 0x30c3f63; continue;
 }
 0x30c3f63 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c3f64); }
 let mut pc: u32 = 0x30c3f64;
 loop { match pc {
 0x30c3f64 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3f7c; continue;
 }
 0x30c3f7c => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3f7c; continue; }
  pc = 0x30c3fa4; continue;
 }
 0x30c3fa4 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x32c0930u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c3faf; continue;
 }
 0x30c3faf => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x325eda8u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c3faf; continue; }
  pc = 0x30c3fca; continue;
 }
 0x30c3fca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c3fe3); f_030c52a4(cpu); if cpu.returned_to != 0x30c3fe3 { return; }
  cpu.push(0x30c3fe8); f_030c7498(cpu); if cpu.returned_to != 0x30c3fe8 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3ffd; continue; }
  pc = 0x30c3fea; continue;
 }
 0x30c3fea => {
  cpu.push(0x30c3fef); f_030c7ae9(cpu); if cpu.returned_to != 0x30c3fef { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c3ffd; continue; }
  pc = 0x30c3ff1; continue;
 }
 0x30c3ff1 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c3ff7); cpu.call(t); if cpu.returned_to != 0x30c3ff7 { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c3ffd); cpu.call(t); if cpu.returned_to != 0x30c3ffd { return; }
  pc = 0x30c3ffd; continue;
 }
 0x30c3ffd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c3ffe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c3ffe); } r_030c3ffe(cpu, 0x30c3ffe); }
pub fn f_030c40ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c40ee); } r_030c3ffe(cpu, 0x30c40ee); }
pub fn f_030c41e9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c41e9); } r_030c3ffe(cpu, 0x30c41e9); }
pub fn f_030c41f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c41f6); } r_030c3ffe(cpu, 0x30c41f6); }
pub fn f_030c42e4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c42e4); } r_030c3ffe(cpu, 0x30c42e4); }
pub fn f_030c43c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c43c7); } r_030c3ffe(cpu, 0x30c43c7); }
fn r_030c3ffe(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c3ffe => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4016; continue;
 }
 0x30c4016 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4016; continue; }
  pc = 0x30c403e; continue;
 }
 0x30c403e => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4047; continue;
 }
 0x30c4047 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4047; continue; }
  pc = 0x30c405b; continue;
 }
 0x30c405b => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c407b; continue;
 }
 0x30c407b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c408d; continue; }
  pc = 0x30c4085; continue;
 }
 0x30c4085 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c407b; continue; }
  pc = 0x30c4088; continue;
 }
 0x30c4088 => {
  pc = 0x30c532f; continue;
 }
 0x30c408d => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c40ed; continue; }
  pc = 0x30c409b; continue;
 }
 0x30c409b => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c409f; continue;
 }
 0x30c409f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c409f; continue; }
  pc = 0x30c40ae; continue;
 }
 0x30c40ae => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c43eau32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c40ed; continue;
 }
 0x30c40ed => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c40ee => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4106; continue;
 }
 0x30c4106 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4106; continue; }
  pc = 0x30c412e; continue;
 }
 0x30c412e => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4137; continue;
 }
 0x30c4137 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4137; continue; }
  pc = 0x30c414b; continue;
 }
 0x30c414b => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4166); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4166 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c41e8; continue; }
  pc = 0x30c416c; continue;
 }
 0x30c416c => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4176; continue;
 }
 0x30c4176 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4188; continue; }
  pc = 0x30c4180; continue;
 }
 0x30c4180 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4176; continue; }
  pc = 0x30c4183; continue;
 }
 0x30c4183 => {
  pc = 0x30c532f; continue;
 }
 0x30c4188 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c41e8; continue; }
  pc = 0x30c4196; continue;
 }
 0x30c4196 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c419a; continue;
 }
 0x30c419a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c419a; continue; }
  pc = 0x30c41a9; continue;
 }
 0x30c41a9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c43eau32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c41e8; continue;
 }
 0x30c41e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c41e9 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4201; continue;
 }
 0x30c41f6 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x18bf02c2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))) | cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4201; continue;
 }
 0x30c4201 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4201; continue; }
  pc = 0x30c4229; continue;
 }
 0x30c4229 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4232; continue;
 }
 0x30c4232 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4232; continue; }
  pc = 0x30c4246; continue;
 }
 0x30c4246 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4261); f_030c7498(cpu); if cpu.returned_to != 0x30c4261 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c42e3; continue; }
  pc = 0x30c4267; continue;
 }
 0x30c4267 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4271; continue;
 }
 0x30c4271 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4283; continue; }
  pc = 0x30c427b; continue;
 }
 0x30c427b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4271; continue; }
  pc = 0x30c427e; continue;
 }
 0x30c427e => {
  pc = 0x30c532f; continue;
 }
 0x30c4283 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c42e3; continue; }
  pc = 0x30c4291; continue;
 }
 0x30c4291 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4295; continue;
 }
 0x30c4295 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4295; continue; }
  pc = 0x30c42a4; continue;
 }
 0x30c42a4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c43eau32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c42e3; continue;
 }
 0x30c42e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c42e4 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c42fc; continue;
 }
 0x30c42fc => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c42fc; continue; }
  pc = 0x30c4324; continue;
 }
 0x30c4324 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c432d; continue;
 }
 0x30c432d => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c432d; continue; }
  pc = 0x30c4341; continue;
 }
 0x30c4341 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c435c); f_030c7498(cpu); if cpu.returned_to != 0x30c435c { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c43e9; continue; }
  pc = 0x30c4362; continue;
 }
 0x30c4362 => {
  cpu.push(0x30c4367); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4367 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c43e9; continue; }
  pc = 0x30c436d; continue;
 }
 0x30c436d => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4377; continue;
 }
 0x30c4377 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4389; continue; }
  pc = 0x30c4381; continue;
 }
 0x30c4381 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4377; continue; }
  pc = 0x30c4384; continue;
 }
 0x30c4384 => {
  pc = 0x30c532f; continue;
 }
 0x30c4389 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c43e9; continue; }
  pc = 0x30c4397; continue;
 }
 0x30c4397 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c439b; continue;
 }
 0x30c439b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c439b; continue; }
  pc = 0x30c43aa; continue;
 }
 0x30c43aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c43c7; continue;
 }
 0x30c43c7 => {
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c43eau32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c43e9; continue;
 }
 0x30c43e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c532f => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c5335); cpu.call(t); if cpu.returned_to != 0x30c5335 { return; }
  let t = cpu.r32(0x3154a34u32); cpu.push(0x30c533b); cpu.call(t); if cpu.returned_to != 0x30c533b { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c43ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c43ea); }
 let mut pc: u32 = 0x30c43ea;
 loop { match pc {
 0x30c43ea => {
  let t = cpu.r32(0x3154a38u32); cpu.push(0x30c43f0); cpu.call(t); if cpu.returned_to != 0x30c43f0 { return; }
  let t = cpu.r32(0x3154a3cu32); cpu.push(0x30c43f6); cpu.call(t); if cpu.returned_to != 0x30c43f6 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c43f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c43f7); }
 let mut pc: u32 = 0x30c43f7;
 loop { match pc {
 0x30c43f7 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c440f; continue;
 }
 0x30c440f => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c440f; continue; }
  pc = 0x30c4440; continue;
 }
 0x30c4440 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4449; continue;
 }
 0x30c4449 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4449; continue; }
  pc = 0x30c445d; continue;
 }
 0x30c445d => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c44d3; continue; }
  pc = 0x30c4481; continue;
 }
 0x30c4481 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4485; continue;
 }
 0x30c4485 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4485; continue; }
  pc = 0x30c4494; continue;
 }
 0x30c4494 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4775u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c44d3; continue;
 }
 0x30c44d3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c44d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c44d4); }
 let mut pc: u32 = 0x30c44d4;
 loop { match pc {
 0x30c44d4 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c44ec; continue;
 }
 0x30c44ec => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c44ec; continue; }
  pc = 0x30c451d; continue;
 }
 0x30c451d => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4526; continue;
 }
 0x30c4526 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4526; continue; }
  pc = 0x30c453a; continue;
 }
 0x30c453a => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4555); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4555 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c45b7; continue; }
  pc = 0x30c4557; continue;
 }
 0x30c4557 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c45b7; continue; }
  pc = 0x30c4565; continue;
 }
 0x30c4565 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4569; continue;
 }
 0x30c4569 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4569; continue; }
  pc = 0x30c4578; continue;
 }
 0x30c4578 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4775u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c45b7; continue;
 }
 0x30c45b7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c45b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c45b8); }
 let mut pc: u32 = 0x30c45b8;
 loop { match pc {
 0x30c45b8 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c45d0; continue;
 }
 0x30c45d0 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c45d0; continue; }
  pc = 0x30c45f8; continue;
 }
 0x30c45f8 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4601; continue;
 }
 0x30c4601 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4601; continue; }
  pc = 0x30c4615; continue;
 }
 0x30c4615 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4630); f_030c7498(cpu); if cpu.returned_to != 0x30c4630 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4692; continue; }
  pc = 0x30c4632; continue;
 }
 0x30c4632 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c4692; continue; }
  pc = 0x30c4640; continue;
 }
 0x30c4640 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4644; continue;
 }
 0x30c4644 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4644; continue; }
  pc = 0x30c4653; continue;
 }
 0x30c4653 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4775u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c4692; continue;
 }
 0x30c4692 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4693(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4693); }
 let mut pc: u32 = 0x30c4693;
 loop { match pc {
 0x30c4693 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c46ab; continue;
 }
 0x30c46ab => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c46ab; continue; }
  pc = 0x30c46d3; continue;
 }
 0x30c46d3 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c46dc; continue;
 }
 0x30c46dc => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c46dc; continue; }
  pc = 0x30c46f0; continue;
 }
 0x30c46f0 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c470b); f_030c7498(cpu); if cpu.returned_to != 0x30c470b { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4774; continue; }
  pc = 0x30c470d; continue;
 }
 0x30c470d => {
  cpu.push(0x30c4712); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4712 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4774; continue; }
  pc = 0x30c4714; continue;
 }
 0x30c4714 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c4774; continue; }
  pc = 0x30c4722; continue;
 }
 0x30c4722 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4726; continue;
 }
 0x30c4726 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4726; continue; }
  pc = 0x30c4735; continue;
 }
 0x30c4735 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4775u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c4774; continue;
 }
 0x30c4774 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4775(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4775); }
 let mut pc: u32 = 0x30c4775;
 loop { match pc {
 0x30c4775 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c477b); cpu.call(t); if cpu.returned_to != 0x30c477b { return; }
  let t = cpu.r32(0x3154a4cu32); cpu.push(0x30c4781); cpu.call(t); if cpu.returned_to != 0x30c4781 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4782(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4782); }
 let mut pc: u32 = 0x30c4782;
 loop { match pc {
 0x30c4782 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c479a; continue;
 }
 0x30c479a => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c479a; continue; }
  pc = 0x30c47cb; continue;
 }
 0x30c47cb => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c47d4; continue;
 }
 0x30c47d4 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c47d4; continue; }
  pc = 0x30c47e8; continue;
 }
 0x30c47e8 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c485e; continue; }
  pc = 0x30c480c; continue;
 }
 0x30c480c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4810; continue;
 }
 0x30c4810 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4810; continue; }
  pc = 0x30c481f; continue;
 }
 0x30c481f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4b00u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c485e; continue;
 }
 0x30c485e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c485f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c485f); }
 let mut pc: u32 = 0x30c485f;
 loop { match pc {
 0x30c485f => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4877; continue;
 }
 0x30c4877 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4877; continue; }
  pc = 0x30c48a8; continue;
 }
 0x30c48a8 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c48b1; continue;
 }
 0x30c48b1 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c48b1; continue; }
  pc = 0x30c48c5; continue;
 }
 0x30c48c5 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c48e0); f_030c7ae9(cpu); if cpu.returned_to != 0x30c48e0 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4942; continue; }
  pc = 0x30c48e2; continue;
 }
 0x30c48e2 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c4942; continue; }
  pc = 0x30c48f0; continue;
 }
 0x30c48f0 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c48f4; continue;
 }
 0x30c48f4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c48f4; continue; }
  pc = 0x30c4903; continue;
 }
 0x30c4903 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4b00u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c4942; continue;
 }
 0x30c4942 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4943(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4943); }
 let mut pc: u32 = 0x30c4943;
 loop { match pc {
 0x30c4943 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c495b; continue;
 }
 0x30c495b => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c495b; continue; }
  pc = 0x30c4983; continue;
 }
 0x30c4983 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c498c; continue;
 }
 0x30c498c => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c498c; continue; }
  pc = 0x30c49a0; continue;
 }
 0x30c49a0 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c49bb); f_030c7498(cpu); if cpu.returned_to != 0x30c49bb { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4a1d; continue; }
  pc = 0x30c49bd; continue;
 }
 0x30c49bd => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c4a1d; continue; }
  pc = 0x30c49cb; continue;
 }
 0x30c49cb => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c49cf; continue;
 }
 0x30c49cf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c49cf; continue; }
  pc = 0x30c49de; continue;
 }
 0x30c49de => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4b00u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c4a1d; continue;
 }
 0x30c4a1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4a1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4a1e); }
 let mut pc: u32 = 0x30c4a1e;
 loop { match pc {
 0x30c4a1e => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4a36; continue;
 }
 0x30c4a36 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4a36; continue; }
  pc = 0x30c4a5e; continue;
 }
 0x30c4a5e => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4a67; continue;
 }
 0x30c4a67 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4a67; continue; }
  pc = 0x30c4a7b; continue;
 }
 0x30c4a7b => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4a96); f_030c7498(cpu); if cpu.returned_to != 0x30c4a96 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4aff; continue; }
  pc = 0x30c4a98; continue;
 }
 0x30c4a98 => {
  cpu.push(0x30c4a9d); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4a9d { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4aff; continue; }
  pc = 0x30c4a9f; continue;
 }
 0x30c4a9f => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c4aff; continue; }
  pc = 0x30c4aad; continue;
 }
 0x30c4aad => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c4ab1; continue;
 }
 0x30c4ab1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4ab1; continue; }
  pc = 0x30c4ac0; continue;
 }
 0x30c4ac0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4b00u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c4aff; continue;
 }
 0x30c4aff => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4b00); }
 let mut pc: u32 = 0x30c4b00;
 loop { match pc {
 0x30c4b00 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c4b06); cpu.call(t); if cpu.returned_to != 0x30c4b06 { return; }
  let t = cpu.r32(0x3154a50u32); cpu.push(0x30c4b0c); cpu.call(t); if cpu.returned_to != 0x30c4b0c { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4b0d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c4b0d); } r_030c4b0d(cpu, 0x30c4b0d); }
pub fn f_030c4bb2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c4bb2); } r_030c4b0d(cpu, 0x30c4bb2); }
pub fn f_030c4c5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c4c5e); } r_030c4b0d(cpu, 0x30c4c5e); }
pub fn f_030c4d01(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c4d01); } r_030c4b0d(cpu, 0x30c4d01); }
fn r_030c4b0d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c4b0d => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4b25; continue;
 }
 0x30c4b25 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4b25; continue; }
  pc = 0x30c4b56; continue;
 }
 0x30c4b56 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4b5f; continue;
 }
 0x30c4b5f => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4b5f; continue; }
  pc = 0x30c4b73; continue;
 }
 0x30c4b73 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4b93; continue;
 }
 0x30c4b93 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4ba5; continue; }
  pc = 0x30c4b9d; continue;
 }
 0x30c4b9d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4b93; continue; }
  pc = 0x30c4ba0; continue;
 }
 0x30c4ba0 => {
  pc = 0x30c5322; continue;
 }
 0x30c4ba5 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c4bab); cpu.call(t); if cpu.returned_to != 0x30c4bab { return; }
  let t = cpu.r32(0x3154a40u32); cpu.push(0x30c4bb1); cpu.call(t); if cpu.returned_to != 0x30c4bb1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c4bb2 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4bca; continue;
 }
 0x30c4bca => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4bca; continue; }
  pc = 0x30c4bfb; continue;
 }
 0x30c4bfb => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4c04; continue;
 }
 0x30c4c04 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4c04; continue; }
  pc = 0x30c4c18; continue;
 }
 0x30c4c18 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4c33); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4c33 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4c5d; continue; }
  pc = 0x30c4c35; continue;
 }
 0x30c4c35 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4c3f; continue;
 }
 0x30c4c3f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4c51; continue; }
  pc = 0x30c4c49; continue;
 }
 0x30c4c49 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4c3f; continue; }
  pc = 0x30c4c4c; continue;
 }
 0x30c4c4c => {
  pc = 0x30c5322; continue;
 }
 0x30c4c51 => {
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c4c57); cpu.call(t); if cpu.returned_to != 0x30c4c57 { return; }
  let t = cpu.r32(0x3154a40u32); cpu.push(0x30c4c5d); cpu.call(t); if cpu.returned_to != 0x30c4c5d { return; }
  pc = 0x30c4c5d; continue;
 }
 0x30c4c5d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c4c5e => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4c76; continue;
 }
 0x30c4c76 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4c76; continue; }
  pc = 0x30c4c9e; continue;
 }
 0x30c4c9e => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4ca7; continue;
 }
 0x30c4ca7 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4ca7; continue; }
  pc = 0x30c4cbb; continue;
 }
 0x30c4cbb => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4cd6); f_030c7498(cpu); if cpu.returned_to != 0x30c4cd6 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4d00; continue; }
  pc = 0x30c4cd8; continue;
 }
 0x30c4cd8 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4ce2; continue;
 }
 0x30c4ce2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4cf4; continue; }
  pc = 0x30c4cec; continue;
 }
 0x30c4cec => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4ce2; continue; }
  pc = 0x30c4cef; continue;
 }
 0x30c4cef => {
  pc = 0x30c5322; continue;
 }
 0x30c4cf4 => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c4cfa); cpu.call(t); if cpu.returned_to != 0x30c4cfa { return; }
  let t = cpu.r32(0x3154a40u32); cpu.push(0x30c4d00); cpu.call(t); if cpu.returned_to != 0x30c4d00 { return; }
  pc = 0x30c4d00; continue;
 }
 0x30c4d00 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c4d01 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4d19; continue;
 }
 0x30c4d19 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4d19; continue; }
  pc = 0x30c4d41; continue;
 }
 0x30c4d41 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4d4a; continue;
 }
 0x30c4d4a => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4d4a; continue; }
  pc = 0x30c4d5e; continue;
 }
 0x30c4d5e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4d79); f_030c7498(cpu); if cpu.returned_to != 0x30c4d79 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4daa; continue; }
  pc = 0x30c4d7b; continue;
 }
 0x30c4d7b => {
  cpu.push(0x30c4d80); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4d80 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4daa; continue; }
  pc = 0x30c4d82; continue;
 }
 0x30c4d82 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31559d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x30c4d8c; continue;
 }
 0x30c4d8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edx)), cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x30c4d9e; continue; }
  pc = 0x30c4d96; continue;
 }
 0x30c4d96 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4d8c; continue; }
  pc = 0x30c4d99; continue;
 }
 0x30c4d99 => {
  pc = 0x30c5322; continue;
 }
 0x30c4d9e => {
  let t = cpu.r32(0x3154a30u32); cpu.push(0x30c4da4); cpu.call(t); if cpu.returned_to != 0x30c4da4 { return; }
  let t = cpu.r32(0x3154a40u32); cpu.push(0x30c4daa); cpu.call(t); if cpu.returned_to != 0x30c4daa { return; }
  pc = 0x30c4daa; continue;
 }
 0x30c4daa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c5322 => {
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c5328); cpu.call(t); if cpu.returned_to != 0x30c5328 { return; }
  let t = cpu.r32(0x3154a20u32); cpu.push(0x30c532e); cpu.call(t); if cpu.returned_to != 0x30c532e { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4dab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4dab); }
 let mut pc: u32 = 0x30c4dab;
 loop { match pc {
 0x30c4dab => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4dc3; continue;
 }
 0x30c4dc3 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4dc3; continue; }
  pc = 0x30c4deb; continue;
 }
 0x30c4deb => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4df4; continue;
 }
 0x30c4df4 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4df4; continue; }
  pc = 0x30c4e08; continue;
 }
 0x30c4e08 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  let t = cpu.r32(0x3154a44u32); cpu.push(0x30c4e24); cpu.call(t); if cpu.returned_to != 0x30c4e24 { return; }
  let t = cpu.r32(0x3154a48u32); cpu.push(0x30c4e2a); cpu.call(t); if cpu.returned_to != 0x30c4e2a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4e2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4e2b); }
 let mut pc: u32 = 0x30c4e2b;
 loop { match pc {
 0x30c4e2b => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4e43; continue;
 }
 0x30c4e43 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4e43; continue; }
  pc = 0x30c4e6b; continue;
 }
 0x30c4e6b => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4e74; continue;
 }
 0x30c4e74 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4e74; continue; }
  pc = 0x30c4e88; continue;
 }
 0x30c4e88 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4ea3); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4ea3 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4eb1; continue; }
  pc = 0x30c4ea5; continue;
 }
 0x30c4ea5 => {
  let t = cpu.r32(0x3154a44u32); cpu.push(0x30c4eab); cpu.call(t); if cpu.returned_to != 0x30c4eab { return; }
  let t = cpu.r32(0x3154a48u32); cpu.push(0x30c4eb1); cpu.call(t); if cpu.returned_to != 0x30c4eb1 { return; }
  pc = 0x30c4eb1; continue;
 }
 0x30c4eb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4eb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4eb2); }
 let mut pc: u32 = 0x30c4eb2;
 loop { match pc {
 0x30c4eb2 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4eca; continue;
 }
 0x30c4eca => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4eca; continue; }
  pc = 0x30c4ef2; continue;
 }
 0x30c4ef2 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4efb; continue;
 }
 0x30c4efb => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4efb; continue; }
  pc = 0x30c4f0f; continue;
 }
 0x30c4f0f => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4f2a); f_030c7498(cpu); if cpu.returned_to != 0x30c4f2a { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4f38; continue; }
  pc = 0x30c4f2c; continue;
 }
 0x30c4f2c => {
  let t = cpu.r32(0x3154a44u32); cpu.push(0x30c4f32); cpu.call(t); if cpu.returned_to != 0x30c4f32 { return; }
  let t = cpu.r32(0x3154a48u32); cpu.push(0x30c4f38); cpu.call(t); if cpu.returned_to != 0x30c4f38 { return; }
  pc = 0x30c4f38; continue;
 }
 0x30c4f38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4f39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4f39); }
 let mut pc: u32 = 0x30c4f39;
 loop { match pc {
 0x30c4f39 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4f51; continue;
 }
 0x30c4f51 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4f51; continue; }
  pc = 0x30c4f79; continue;
 }
 0x30c4f79 => {
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  pc = 0x30c4f82; continue;
 }
 0x30c4f82 => {
  { let v = (cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = (cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4f82; continue; }
  pc = 0x30c4f96; continue;
 }
 0x30c4f96 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.edx, 0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c4fb1); f_030c7498(cpu); if cpu.returned_to != 0x30c4fb1 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4fc6; continue; }
  pc = 0x30c4fb3; continue;
 }
 0x30c4fb3 => {
  cpu.push(0x30c4fb8); f_030c7ae9(cpu); if cpu.returned_to != 0x30c4fb8 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c4fc6; continue; }
  pc = 0x30c4fba; continue;
 }
 0x30c4fba => {
  let t = cpu.r32(0x3154a44u32); cpu.push(0x30c4fc0); cpu.call(t); if cpu.returned_to != 0x30c4fc0 { return; }
  let t = cpu.r32(0x3154a48u32); cpu.push(0x30c4fc6); cpu.call(t); if cpu.returned_to != 0x30c4fc6 { return; }
  pc = 0x30c4fc6; continue;
 }
 0x30c4fc6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c4fc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c4fc7); }
 let mut pc: u32 = 0x30c4fc7;
 loop { match pc {
 0x30c4fc7 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c4fdf; continue;
 }
 0x30c4fdf => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c4fdf; continue; }
  pc = 0x30c5010; continue;
 }
 0x30c5010 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5077; continue; }
  pc = 0x30c5027; continue;
 }
 0x30c5027 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c502b; continue;
 }
 0x30c502b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c502b; continue; }
  pc = 0x30c503a; continue;
 }
 0x30c503a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c5295u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c5077; continue;
 }
 0x30c5077 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5078(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5078); }
 let mut pc: u32 = 0x30c5078;
 loop { match pc {
 0x30c5078 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c5090; continue;
 }
 0x30c5090 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x325803cu32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3258044u32.wrapping_add(cpu.ebx.wrapping_mul(8))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5090; continue; }
  pc = 0x30c50c1; continue;
 }
 0x30c50c1 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c50cf); f_030c7611(cpu); if cpu.returned_to != 0x30c50cf { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c512f; continue; }
  pc = 0x30c50d1; continue;
 }
 0x30c50d1 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c512f; continue; }
  pc = 0x30c50df; continue;
 }
 0x30c50df => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c50e3; continue;
 }
 0x30c50e3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c50e3; continue; }
  pc = 0x30c50f2; continue;
 }
 0x30c50f2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c5295u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c512f; continue;
 }
 0x30c512f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5130); }
 let mut pc: u32 = 0x30c5130;
 loop { match pc {
 0x30c5130 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c5148; continue;
 }
 0x30c5148 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5148; continue; }
  pc = 0x30c5170; continue;
 }
 0x30c5170 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c517e); f_030c71d0(cpu); if cpu.returned_to != 0x30c517e { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c51de; continue; }
  pc = 0x30c5180; continue;
 }
 0x30c5180 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c51de; continue; }
  pc = 0x30c518e; continue;
 }
 0x30c518e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c5192; continue;
 }
 0x30c5192 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5192; continue; }
  pc = 0x30c51a1; continue;
 }
 0x30c51a1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c5295u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c51de; continue;
 }
 0x30c51de => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c51df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c51df); }
 let mut pc: u32 = 0x30c51df;
 loop { match pc {
 0x30c51df => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32c0918u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  pc = 0x30c51f7; continue;
 }
 0x30c51f7 => {
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(cpu.r16(0x0u32.wrapping_add(cpu.edx))); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x32560f4u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ebx.wrapping_mul(8)); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c51f7; continue; }
  pc = 0x30c521f; continue;
 }
 0x30c521f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3155400u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.esi = v; }
  cpu.push(0x30c522d); f_030c71d0(cpu); if cpu.returned_to != 0x30c522d { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c5294; continue; }
  pc = 0x30c522f; continue;
 }
 0x30c522f => {
  cpu.push(0x30c5234); f_030c7611(cpu); if cpu.returned_to != 0x30c5234 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c5294; continue; }
  pc = 0x30c5236; continue;
 }
 0x30c5236 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5294; continue; }
  pc = 0x30c5244; continue;
 }
 0x30c5244 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c5248; continue;
 }
 0x30c5248 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5248; continue; }
  pc = 0x30c5257; continue;
 }
 0x30c5257 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c5295u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c5294; continue;
 }
 0x30c5294 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5295(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5295); }
 let mut pc: u32 = 0x30c5295;
 loop { match pc {
 0x30c5295 => {
  let t = cpu.r32(0x3154a1cu32); cpu.push(0x30c529b); cpu.call(t); if cpu.returned_to != 0x30c529b { return; }
  let t = cpu.r32(0x3154a24u32); cpu.push(0x30c52a1); cpu.call(t); if cpu.returned_to != 0x30c52a1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c52a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c52a2); }
 let mut pc: u32 = 0x30c52a2;
 loop { match pc {
 0x30c52a2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c52a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c52a3); }
 let mut pc: u32 = 0x30c52a3;
 loop { match pc {
 0x30c52a3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c52a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c52a4); }
 let mut pc: u32 = 0x30c52a4;
 loop { match pc {
 0x30c52a4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30c52aa; continue;
 }
 0x30c52aa => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.edi))); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c52c4; continue; }
  pc = 0x30c52bd; continue;
 }
 0x30c52bd => {
  { cpu.sub32(cpu.eax, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c52cb; continue; }
  pc = 0x30c52c4; continue;
 }
 0x30c52c4 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.edi)).wrapping_add(0x4000u32); cpu.w32(0x3cu32.wrapping_add(cpu.edi), v); }
  pc = 0x30c52cb; continue;
 }
 0x30c52cb => {
  { cpu.sub32(cpu.ebx, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c52db; continue; }
  pc = 0x30c52d3; continue;
 }
 0x30c52d3 => {
  { cpu.sub32(cpu.ebx, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c52e2; continue; }
  pc = 0x30c52db; continue;
 }
 0x30c52db => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edi)).wrapping_add(0x4000u32); cpu.w32(0x40u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c52e2; continue;
 }
 0x30c52e2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c52aa; continue; }
  pc = 0x30c52e8; continue;
 }
 0x30c52e8 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1cu32.wrapping_add(cpu.edi))); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c5302; continue; }
  pc = 0x30c52fb; continue;
 }
 0x30c52fb => {
  { cpu.sub32(cpu.eax, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5309; continue; }
  pc = 0x30c5302; continue;
 }
 0x30c5302 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)).wrapping_add(0x4000u32); cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x30c5309; continue;
 }
 0x30c5309 => {
  { cpu.sub32(cpu.ebx, 0x2000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c5319; continue; }
  pc = 0x30c5311; continue;
 }
 0x30c5311 => {
  { cpu.sub32(cpu.ebx, 0xffffe000u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5320; continue; }
  pc = 0x30c5319; continue;
 }
 0x30c5319 => {
  { let v: u32 = cpu.add32(cpu.r32(0x1cu32.wrapping_add(cpu.esi)), 0x4000u32); cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x30c5320; continue;
 }
 0x30c5320 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c533c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c533c); } r_030c533c(cpu, 0x30c533c); }
pub fn f_030c5349(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c5349); } r_030c533c(cpu, 0x30c5349); }
fn r_030c533c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c533c => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.esi = v; }
  { let v: u32 = 0x32a4de0u32; cpu.edi = v; }
  pc = 0x30c5356; continue;
 }
 0x30c5349 => {
  { let v: u32 = cpu.r32(0x32bb430u32); cpu.esi = v; }
  { let v: u32 = 0x32bb434u32; cpu.edi = v; }
  pc = 0x30c5356; continue;
 }
 0x30c5356 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4dd8u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x328c5b0u32)); cpu.w32(0x328c5b0u32, v); }
  pc = 0x30c5363; continue;
 }
 0x30c5363 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x32a4dd8u32)); }
  if cpu.zf { pc = 0x30c53c6; continue; }
  pc = 0x30c536b; continue;
 }
 0x30c536b => {
  { let v: u32 = cpu.esi.wrapping_sub(0x24u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, 0x24u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x3155464u32; cpu.edx = v; }
  pc = 0x30c5381; continue;
 }
 0x30c5381 => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x24u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5381; continue; }
  pc = 0x30c538c; continue;
 }
 0x30c538c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32cb3a4u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32c0914u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3296dc0u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3154a38u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3154a3cu32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x3155464u32; cpu.esi = v; }
  let t = cpu.edx; cpu.push(0x30c53c3); cpu.call(t); if cpu.returned_to != 0x30c53c3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.poll(); pc = 0x30c5363; continue;
 }
 0x30c53c6 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c53c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c53c8); }
 let mut pc: u32 = 0x30c53c8;
 loop { match pc {
 0x30c53c8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x315460cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x3154610u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x3154614u32)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547e0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31547e4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x31547e8u32, v); }
  { let v: u32 = 0x31547e0u32; cpu.esi = v; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  cpu.push(0x30c5404); f_030b9e6a(cpu); if cpu.returned_to != 0x30c5404 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x315598cu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c5484; continue; }
  pc = 0x30c540d; continue;
 }
 0x30c540d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32c0914u32, v); }
  cpu.push(0x30c5422); f_030c7d7f(cpu); if cpu.returned_to != 0x30c5422 { return; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c5484; continue; }
  pc = 0x30c5424; continue;
 }
 0x30c5424 => {
  { let v: u32 = cpu.r32(0x32a4ddcu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x32bad70u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5484; continue; }
  pc = 0x30c5432; continue;
 }
 0x30c5432 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30c5436; continue;
 }
 0x30c5436 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5436; continue; }
  pc = 0x30c5445; continue;
 }
 0x30c5445 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cb3a4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32c0914u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3296dc0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a38u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3154a3cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x30c4775u32; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x24u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32a4ddcu32, v); }
  pc = 0x30c5484; continue;
 }
 0x30c5484 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5610); }
 let mut pc: u32 = 0x30c5610;
 loop { match pc {
 0x30c5610 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x31559f8u32)); cpu.w32(0x31559f8u32, v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x31559e8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31559f8u32); cpu.edx = v; }
  pc = 0x30c5625; continue;
 }
 0x30c5625 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30c5658; continue; }
  pc = 0x30c562b; continue;
 }
 0x30c562b => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c563b; continue; }
  pc = 0x30c5630; continue;
 }
 0x30c5630 => {
  { cpu.sub8(cpu.r8(0x12u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x30c563b; continue; }
  pc = 0x30c5636; continue;
 }
 0x30c5636 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30c563b; continue;
 }
 0x30c563b => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x32cb3c4u32), cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5625; continue; }
  pc = 0x30c5647; continue;
 }
 0x30c5647 => {
  { let v: u32 = cpu.r32(0x31559e8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c565a; continue;
 }
 0x30c5658 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30c565a; continue;
 }
 0x30c565a => {
  { let v: u32 = cpu.r32(0x31559e8u32); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x3155a04u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x31554c8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x32cb3c8u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3b8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x32cb3b8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.r32(0x31559f4u32); cpu.push(0x30c5696); cpu.call(t); if cpu.returned_to != 0x30c5696 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x30c56a5; continue; }
  pc = 0x30c569d; continue;
 }
 0x30c569d => {
  { cpu.popad(); }
  { let v: u32 = cpu.r32(0x32cb3b8u32); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c56a5 => {
  { cpu.popad(); }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c56a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c56a9); }
 let mut pc: u32 = 0x30c56a9;
 loop { match pc {
 0x30c56a9 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31559e8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x31559fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cb3c4u32); cpu.edx = v; }
  pc = 0x30c56bd; continue;
 }
 0x30c56bd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30c56cc; continue; }
  pc = 0x30c56c6; continue;
 }
 0x30c56c6 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.edi)), cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c56cc; continue; }
  pc = 0x30c56cb; continue;
 }
 0x30c56cb => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30c56cc; continue;
 }
 0x30c56cc => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c56bd; continue; }
  pc = 0x30c56cf; continue;
 }
 0x30c56cf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c571c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c571c); }
 let mut pc: u32 = 0x30c571c;
 loop { match pc {
 0x30c571c => {
  { let v: u32 = 0x30c5948u32; cpu.w32(0x315a69cu32, v); }
  pc = 0x30c5728; continue;
 }
 0x30c5728 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x4e534956u32); }
  if !cpu.zf { pc = 0x30c5827; continue; }
  pc = 0x30c5735; continue;
 }
 0x30c5735 => {
  { let v: u32 = cpu.edi; cpu.w32(0x32cb3ccu32, v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x315a6a0u32)); }
  if cpu.zf { pc = 0x30c574d; continue; }
  pc = 0x30c5743; continue;
 }
 0x30c5743 => {
  { let v: u32 = cpu.eax; cpu.w32(0x315a6a0u32, v); }
  cpu.push(0x30c574d); f_030c6650(cpu); if cpu.returned_to != 0x30c574d { return; }
  pc = 0x30c574d; continue;
 }
 0x30c574d => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315a6a4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315a6a8u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d8u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3dcu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.r32(0x315a744u32), cpu.eax); }
  if cpu.zf { pc = 0x30c57ab; continue; }
  pc = 0x30c5784; continue;
 }
 0x30c5784 => {
  { let v: u32 = cpu.eax; cpu.w32(0x315a744u32, v); }
  { cpu.pushad(); }
  { let v: u32 = 0x315a748u32; cpu.esi = v; }
  { let v: u32 = 0x32cc890u32; cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30c579a); f_030c6097(cpu); if cpu.returned_to != 0x30c579a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x315a788u32; cpu.esi = v; }
  { let v: u32 = 0x32cca90u32; cpu.edi = v; }
  cpu.push(0x30c57aa); f_030c6097(cpu); if cpu.returned_to != 0x30c57aa { return; }
  { cpu.popad(); }
  pc = 0x30c57ab; continue;
 }
 0x30c57ab => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x315a694u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315a6acu32, v); }
  { let v: u32 = cpu.esi.wrapping_add(0xau32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u8 = 0x20u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = 0x32cc698u32; cpu.edi = v; }
  cpu.push(0x30c57d3); f_030c5df6(cpu); if cpu.returned_to != 0x30c57d3 { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x32cc390u32; cpu.ecx = v; }
  { let v: u32 = 0x32cc490u32; cpu.edx = v; }
  { let v: u32 = 0x32cc594u32; cpu.ebp = v; }
  { let v: u32 = 0x32cc698u32; cpu.esi = v; }
  cpu.push(0x30c57f2); f_030c5ed7(cpu); if cpu.returned_to != 0x30c57f2 { return; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x30c5827; continue; }
  pc = 0x30c57f5; continue;
 }
 0x30c57f5 => {
  { let v: u32 = 0x32cc30cu32; cpu.edi = v; }
  cpu.push(0x30c57ff); f_030c5df6(cpu); if cpu.returned_to != 0x30c57ff { return; }
  { cpu.pushad(); }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x32cc004u32; cpu.ecx = v; }
  { let v: u32 = 0x32cc104u32; cpu.edx = v; }
  { let v: u32 = 0x32cc208u32; cpu.ebp = v; }
  { let v: u32 = 0x32cc30cu32; cpu.esi = v; }
  cpu.push(0x30c581e); f_030c5ed7(cpu); if cpu.returned_to != 0x30c581e { return; }
  { cpu.popad(); }
  if cpu.zf { pc = 0x30c5827; continue; }
  pc = 0x30c5821; continue;
 }
 0x30c5821 => {
  let t = cpu.r32(0x315a6acu32); cpu.push(0x30c5827); cpu.call(t); if cpu.returned_to != 0x30c5827 { return; }
  pc = 0x30c5827; continue;
 }
 0x30c5827 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5829(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5829); }
 let mut pc: u32 = 0x30c5829;
 loop { match pc {
 0x30c5829 => {
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315a6a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3e0u32, v); }
  { let v: u32 = cpu.r32(0x32cb3dcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d4u32, v); }
  pc = 0x30c584e; continue;
 }
 0x30c584e => {
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d0u32, v); }
  pc = 0x30c5858; continue;
 }
 0x30c5858 => {
  { let v: u32 = 0x32cb404u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3ecu32); cpu.eax = v; }
  cpu.push(0x30c5867); f_030c5c7c(cpu); if cpu.returned_to != 0x30c5867 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3ecu32, v); }
  { let v: u32 = 0x32cb504u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f0u32); cpu.eax = v; }
  cpu.push(0x30c587b); f_030c5c7c(cpu); if cpu.returned_to != 0x30c587b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f0u32, v); }
  { let v: u32 = 0x32cb604u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f4u32); cpu.eax = v; }
  cpu.push(0x30c588f); f_030c5b02(cpu); if cpu.returned_to != 0x30c588f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f4u32, v); }
  { let v: u32 = 0x32cb704u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f8u32); cpu.eax = v; }
  cpu.push(0x30c58a3); f_030c5b02(cpu); if cpu.returned_to != 0x30c58a3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f8u32, v); }
  { let v: u32 = 0x32cb804u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3fcu32); cpu.eax = v; }
  cpu.push(0x30c58b7); f_030c5b02(cpu); if cpu.returned_to != 0x30c58b7 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3fcu32, v); }
  { let v: u32 = 0x32cb904u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb400u32); cpu.eax = v; }
  cpu.push(0x30c58cb); f_030c5b02(cpu); if cpu.returned_to != 0x30c58cb { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb400u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x32cb404u32; cpu.esi = v; }
  cpu.push(0x30c58dd); f_030c60ff(cpu); if cpu.returned_to != 0x30c58dd { return; }
  { let v: u32 = 0x32cb504u32; cpu.esi = v; }
  cpu.push(0x30c58e7); f_030c60ff(cpu); if cpu.returned_to != 0x30c58e7 { return; }
  { let v: u32 = 0x32cb604u32; cpu.esi = v; }
  cpu.push(0x30c58f1); f_030c60ff(cpu); if cpu.returned_to != 0x30c58f1 { return; }
  { let v: u32 = 0x32cb704u32; cpu.esi = v; }
  cpu.push(0x30c58fb); f_030c60ff(cpu); if cpu.returned_to != 0x30c58fb { return; }
  { let v: u32 = 0x32cb804u32; cpu.esi = v; }
  cpu.push(0x30c5905); f_030c60ff(cpu); if cpu.returned_to != 0x30c5905 { return; }
  { let v: u32 = 0x32cb904u32; cpu.esi = v; }
  cpu.push(0x30c590f); f_030c60ff(cpu); if cpu.returned_to != 0x30c590f { return; }
  { let v: u32 = cpu.r32(0x32cb3ccu32); cpu.edi = v; }
  cpu.push(0x30c591a); f_030c63b3(cpu); if cpu.returned_to != 0x30c591a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cb3ccu32).wrapping_add(0x20u32); cpu.w32(0x32cb3ccu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x32cb3d0u32)); cpu.w32(0x32cb3d0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5858; continue; }
  pc = 0x30c5930; continue;
 }
 0x30c5930 => {
  { let v: u32 = cpu.r32(0x32cb3e0u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x32cb3ccu32), cpu.eax); cpu.w32(0x32cb3ccu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x32cb3d4u32)); cpu.w32(0x32cb3d4u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c584e; continue; }
  pc = 0x30c5947; continue;
 }
 0x30c5947 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5948(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5948); }
 let mut pc: u32 = 0x30c5948;
 loop { match pc {
 0x30c5948 => {
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32cb3dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cb3d8u32).wrapping_add(cpu.eax); cpu.w32(0x32cb3d8u32, v); }
  { let v: u32 = cpu.r32(0x32cb3dcu32).wrapping_add(cpu.ebx); cpu.w32(0x32cb3dcu32, v); }
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315a6a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3e0u32, v); }
  { let v: u32 = cpu.r32(0x32cb3dcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d4u32, v); }
  pc = 0x30c5984; continue;
 }
 0x30c5984 => {
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d0u32, v); }
  pc = 0x30c598e; continue;
 }
 0x30c598e => {
  { let v: u32 = 0x32cb404u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3ecu32); cpu.eax = v; }
  cpu.push(0x30c599d); f_030c5c7c(cpu); if cpu.returned_to != 0x30c599d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3ecu32, v); }
  { let v: u32 = 0x32cb504u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f0u32); cpu.eax = v; }
  cpu.push(0x30c59b1); f_030c5c7c(cpu); if cpu.returned_to != 0x30c59b1 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f0u32, v); }
  { let v: u32 = 0x32cb604u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f4u32); cpu.eax = v; }
  cpu.push(0x30c59c5); f_030c5b02(cpu); if cpu.returned_to != 0x30c59c5 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f4u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x32cb404u32; cpu.esi = v; }
  cpu.push(0x30c59d7); f_030c60ff(cpu); if cpu.returned_to != 0x30c59d7 { return; }
  { let v: u32 = 0x32cb504u32; cpu.esi = v; }
  cpu.push(0x30c59e1); f_030c60ff(cpu); if cpu.returned_to != 0x30c59e1 { return; }
  { let v: u32 = 0x32cb604u32; cpu.esi = v; }
  cpu.push(0x30c59eb); f_030c60ff(cpu); if cpu.returned_to != 0x30c59eb { return; }
  { let v: u32 = 0x32cba04u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cb3ccu32); cpu.edi = v; }
  cpu.push(0x30c59fb); f_030c6675(cpu); if cpu.returned_to != 0x30c59fb { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cb3ccu32).wrapping_add(0x10u32); cpu.w32(0x32cb3ccu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x32cb3d0u32)); cpu.w32(0x32cb3d0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c598e; continue; }
  pc = 0x30c5a0d; continue;
 }
 0x30c5a0d => {
  { let v: u32 = cpu.r32(0x32cb3e0u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x32cb3ccu32), cpu.eax); cpu.w32(0x32cb3ccu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x32cb3d4u32)); cpu.w32(0x32cb3d4u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5984; continue; }
  pc = 0x30c5a24; continue;
 }
 0x30c5a24 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5a25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5a25); }
 let mut pc: u32 = 0x30c5a25;
 loop { match pc {
 0x30c5a25 => {
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32cb3dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cb3d8u32).wrapping_add(cpu.eax); cpu.w32(0x32cb3d8u32, v); }
  { let v: u32 = cpu.r32(0x32cb3dcu32).wrapping_add(cpu.ebx); cpu.w32(0x32cb3dcu32, v); }
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315a6a4u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3e0u32, v); }
  { let v: u32 = cpu.r32(0x32cb3dcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d4u32, v); }
  pc = 0x30c5a61; continue;
 }
 0x30c5a61 => {
  { let v: u32 = cpu.r32(0x32cb3d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3d0u32, v); }
  pc = 0x30c5a6b; continue;
 }
 0x30c5a6b => {
  { let v: u32 = 0x32cb404u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3ecu32); cpu.eax = v; }
  cpu.push(0x30c5a7a); f_030c5c7c(cpu); if cpu.returned_to != 0x30c5a7a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3ecu32, v); }
  { let v: u32 = 0x32cb504u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f0u32); cpu.eax = v; }
  cpu.push(0x30c5a8e); f_030c5c7c(cpu); if cpu.returned_to != 0x30c5a8e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f0u32, v); }
  { let v: u32 = 0x32cb604u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cb3f4u32); cpu.eax = v; }
  cpu.push(0x30c5aa2); f_030c5b02(cpu); if cpu.returned_to != 0x30c5aa2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cb3f4u32, v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x32cb404u32; cpu.esi = v; }
  cpu.push(0x30c5ab4); f_030c60ff(cpu); if cpu.returned_to != 0x30c5ab4 { return; }
  { let v: u32 = 0x32cb504u32; cpu.esi = v; }
  cpu.push(0x30c5abe); f_030c60ff(cpu); if cpu.returned_to != 0x30c5abe { return; }
  { let v: u32 = 0x32cb604u32; cpu.esi = v; }
  cpu.push(0x30c5ac8); f_030c60ff(cpu); if cpu.returned_to != 0x30c5ac8 { return; }
  { let v: u32 = 0x32cba04u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cb3ccu32); cpu.edi = v; }
  cpu.push(0x30c5ad8); f_030c68f4(cpu); if cpu.returned_to != 0x30c5ad8 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cb3ccu32).wrapping_add(0x8u32); cpu.w32(0x32cb3ccu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x32cb3d0u32)); cpu.w32(0x32cb3d0u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5a6b; continue; }
  pc = 0x30c5aea; continue;
 }
 0x30c5aea => {
  { let v: u32 = cpu.r32(0x32cb3e0u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x32cb3ccu32), cpu.eax); cpu.w32(0x32cb3ccu32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x32cb3d4u32)); cpu.w32(0x32cb3d4u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5a61; continue; }
  pc = 0x30c5b01; continue;
 }
 0x30c5b01 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5b02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5b02); }
 let mut pc: u32 = 0x30c5b02;
 loop { match pc {
 0x30c5b02 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x32cc890u32; cpu.edx = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5b1f; continue; }
  pc = 0x30c5b0d; continue;
 }
 0x30c5b0d => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5b1f; continue;
 }
 0x30c5b1f => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x6u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 6u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 26u32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3e4u32, v); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5b4b; continue; }
  pc = 0x30c5b39; continue;
 }
 0x30c5b39 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5b4b; continue;
 }
 0x30c5b4b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0xcu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cb3e4u32)); }
  if cpu.zf { pc = 0x30c5c63; continue; }
  pc = 0x30c5b6c; continue;
 }
 0x30c5b6c => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5b83; continue; }
  pc = 0x30c5b71; continue;
 }
 0x30c5b71 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5b83; continue;
 }
 0x30c5b83 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x32cc004u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c5bbf; continue; }
  pc = 0x30c5b95; continue;
 }
 0x30c5b95 => {
  { let v: u8 = cpu.r8(0x32cc30cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5b9f; continue;
 }
 0x30c5b9f => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x3au32); cpu.eax = v; }
  if cpu.zf { pc = 0x30c5baa; continue; }
  pc = 0x30c5ba4; continue;
 }
 0x30c5ba4 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  pc = 0x30c5baa; continue;
 }
 0x30c5baa => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cb3e4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5b6c; continue; }
  pc = 0x30c5bba; continue;
 }
 0x30c5bba => {
  pc = 0x30c5c63; continue;
 }
 0x30c5bbf => {
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5c14; continue; }
  pc = 0x30c5bc6; continue;
 }
 0x30c5bc6 => {
  { let v: u8 = cpu.r8(0x32cc30cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5bd0; continue;
 }
 0x30c5bd0 => {
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), (cpu.eax as u8)); }
  if (cpu.sf == cpu.of) { pc = 0x30c5be9; continue; }
  pc = 0x30c5bd7; continue;
 }
 0x30c5bd7 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5be9; continue;
 }
 0x30c5be9 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cb3e4u32)); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5b6c; continue; }
  pc = 0x30c5c12; continue;
 }
 0x30c5c12 => {
  pc = 0x30c5c63; continue;
 }
 0x30c5c14 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 8u32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5c31; continue; }
  pc = 0x30c5c1f; continue;
 }
 0x30c5c1f => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5c31; continue;
 }
 0x30c5c31 => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if !cpu.cf { pc = 0x30c5c46; continue; }
  pc = 0x30c5c37; continue;
 }
 0x30c5c37 => {
  { let v: u8 = cpu.r8(0x32cc187u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c5c55; continue; }
  pc = 0x30c5c44; continue;
 }
 0x30c5c44 => {
  cpu.poll(); pc = 0x30c5c31; continue;
 }
 0x30c5c46 => {
  { let v: u8 = cpu.r8(0x32cc083u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c5c55; continue; }
  pc = 0x30c5c53; continue;
 }
 0x30c5c53 => {
  cpu.poll(); pc = 0x30c5c31; continue;
 }
 0x30c5c55 => {
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c5b9f; continue; }
  pc = 0x30c5c5e; continue;
 }
 0x30c5c5e => {
  cpu.poll(); pc = 0x30c5bd0; continue;
 }
 0x30c5c63 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c5c76; continue; }
  pc = 0x30c5c68; continue;
 }
 0x30c5c68 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x30c5c6a; continue;
 }
 0x30c5c6a => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5c6a; continue; }
  pc = 0x30c5c76; continue;
 }
 0x30c5c76 => {
  { let v: u32 = cpu.r32(0x32cb3e4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5c7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5c7c); }
 let mut pc: u32 = 0x30c5c7c;
 loop { match pc {
 0x30c5c7c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x32cca90u32; cpu.edx = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x6u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5c99; continue; }
  pc = 0x30c5c87; continue;
 }
 0x30c5c87 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5c99; continue;
 }
 0x30c5c99 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x6u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 6u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 26u32); cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cb3e8u32, v); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5cc5; continue; }
  pc = 0x30c5cb3; continue;
 }
 0x30c5cb3 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5cc5; continue;
 }
 0x30c5cc5 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0xcu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 20u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cb3e8u32)); }
  if cpu.zf { pc = 0x30c5ddd; continue; }
  pc = 0x30c5ce6; continue;
 }
 0x30c5ce6 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5cfd; continue; }
  pc = 0x30c5ceb; continue;
 }
 0x30c5ceb => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5cfd; continue;
 }
 0x30c5cfd => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x32cc390u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c5d39; continue; }
  pc = 0x30c5d0f; continue;
 }
 0x30c5d0f => {
  { let v: u8 = cpu.r8(0x32cc698u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5d19; continue;
 }
 0x30c5d19 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x3au32); cpu.eax = v; }
  if cpu.zf { pc = 0x30c5d24; continue; }
  pc = 0x30c5d1e; continue;
 }
 0x30c5d1e => {
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  pc = 0x30c5d24; continue;
 }
 0x30c5d24 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cb3e8u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5ce6; continue; }
  pc = 0x30c5d34; continue;
 }
 0x30c5d34 => {
  pc = 0x30c5ddd; continue;
 }
 0x30c5d39 => {
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c5d8e; continue; }
  pc = 0x30c5d40; continue;
 }
 0x30c5d40 => {
  { let v: u8 = cpu.r8(0x32cc698u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5d4a; continue;
 }
 0x30c5d4a => {
  { let v: u32 = cpu.eax.wrapping_sub(0x74u32); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), (cpu.eax as u8)); }
  if (cpu.sf == cpu.of) { pc = 0x30c5d63; continue; }
  pc = 0x30c5d51; continue;
 }
 0x30c5d51 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5d63; continue;
 }
 0x30c5d63 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.neg8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 14u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cb3e8u32)); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5ce6; continue; }
  pc = 0x30c5d8c; continue;
 }
 0x30c5d8c => {
  pc = 0x30c5ddd; continue;
 }
 0x30c5d8e => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 8u32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5dab; continue; }
  pc = 0x30c5d99; continue;
 }
 0x30c5d99 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5dab; continue;
 }
 0x30c5dab => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if !cpu.cf { pc = 0x30c5dc0; continue; }
  pc = 0x30c5db1; continue;
 }
 0x30c5db1 => {
  { let v: u8 = cpu.r8(0x32cc513u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c5dcf; continue; }
  pc = 0x30c5dbe; continue;
 }
 0x30c5dbe => {
  cpu.poll(); pc = 0x30c5dab; continue;
 }
 0x30c5dc0 => {
  { let v: u8 = cpu.r8(0x32cc40fu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x81u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c5dcf; continue; }
  pc = 0x30c5dcd; continue;
 }
 0x30c5dcd => {
  cpu.poll(); pc = 0x30c5dab; continue;
 }
 0x30c5dcf => {
  { cpu.sub32(cpu.eax, 0x74u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30c5d19; continue; }
  pc = 0x30c5dd8; continue;
 }
 0x30c5dd8 => {
  cpu.poll(); pc = 0x30c5d4a; continue;
 }
 0x30c5ddd => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c5df0; continue; }
  pc = 0x30c5de2; continue;
 }
 0x30c5de2 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x30c5de4; continue;
 }
 0x30c5de4 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5de4; continue; }
  pc = 0x30c5df0; continue;
 }
 0x30c5df0 => {
  { let v: u32 = cpu.r32(0x32cb3e8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5df6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5df6); }
 let mut pc: u32 = 0x30c5df6;
 loop { match pc {
 0x30c5df6 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0xcu8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5e0d; continue; }
  pc = 0x30c5dfb; continue;
 }
 0x30c5dfb => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5e0d; continue;
 }
 0x30c5e0d => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0xcu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 12u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 20u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(((cpu.edx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5e1d; continue;
 }
 0x30c5e1d => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5e34; continue; }
  pc = 0x30c5e22; continue;
 }
 0x30c5e22 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5e34; continue;
 }
 0x30c5e34 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x32cc734u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x32cc87cu32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebp, (cpu.ecx & 0xff)); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5e4b; continue;
 }
 0x30c5e4b => {
  { cpu.sub8((cpu.eax as u8), 0x11u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5e61; continue; }
  pc = 0x30c5e4f; continue;
 }
 0x30c5e4f => {
  { cpu.sub8((cpu.eax as u8), 0x10u8); }
  if cpu.zf { pc = 0x30c5ea0; continue; }
  pc = 0x30c5e53; continue;
 }
 0x30c5e53 => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.and8(((cpu.edx >> 8) as u8), 0xfu8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5e1d; continue; }
  pc = 0x30c5e5f; continue;
 }
 0x30c5e5f => {
  pc = 0x30c5ed5; continue;
 }
 0x30c5e61 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 8u32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5e7e; continue; }
  pc = 0x30c5e6c; continue;
 }
 0x30c5e6c => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5e7e; continue;
 }
 0x30c5e7e => {
  { let v: u8 = cpu.dec8(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if !cpu.cf { pc = 0x30c5e91; continue; }
  pc = 0x30c5e84; continue;
 }
 0x30c5e84 => {
  { let v: u8 = cpu.r8(0x32cc847u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x11u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c5e9e; continue; }
  pc = 0x30c5e8f; continue;
 }
 0x30c5e8f => {
  cpu.poll(); pc = 0x30c5e7e; continue;
 }
 0x30c5e91 => {
  { let v: u8 = cpu.r8(0x32cc823u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x11u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c5e9e; continue; }
  pc = 0x30c5e9c; continue;
 }
 0x30c5e9c => {
  cpu.poll(); pc = 0x30c5e7e; continue;
 }
 0x30c5e9e => {
  cpu.poll(); pc = 0x30c5e4b; continue;
 }
 0x30c5ea0 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x5u8); }
  if (cpu.sf == cpu.of) { pc = 0x30c5eb7; continue; }
  pc = 0x30c5ea5; continue;
 }
 0x30c5ea5 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, (cpu.ecx & 0xff)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp | cpu.ebx; cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(0x10u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5eb7; continue;
 }
 0x30c5eb7 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_sub(0x5u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ebp, 5u32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 27u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub((cpu.eax as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30c5ec6; continue;
 }
 0x30c5ec6 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.dec8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5ec6; continue; }
  pc = 0x30c5ecd; continue;
 }
 0x30c5ecd => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5e1d; continue; }
  pc = 0x30c5ed5; continue;
 }
 0x30c5ed5 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c5ed7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c5ed7); }
 let mut pc: u32 = 0x30c5ed7;
 loop { match pc {
 0x30c5ed7 => {
  { let v: u32 = cpu.esp.wrapping_sub(0xc0u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cc71cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32cc72cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x32cc720u32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32cc724u32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32cc728u32, v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.r32(0x32cc71cu32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30c5f15; continue;
 }
 0x30c5f15 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0xfu8); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c6057; continue; }
  pc = 0x30c5f1f; continue;
 }
 0x30c5f1f => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x30c5f26; continue; }
  pc = 0x30c5f23; continue;
 }
 0x30c5f23 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)))); cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  pc = 0x30c5f26; continue;
 }
 0x30c5f26 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5f15; continue; }
  pc = 0x30c5f2a; continue;
 }
 0x30c5f2a => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x32cc71cu32)); cpu.esi = v; }
  { let v: u32 = 0x44u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0xfu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30c5f3e; continue;
 }
 0x30c5f3e => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c5f3e; continue; }
  pc = 0x30c5f4d; continue;
 }
 0x30c5f4d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32cc724u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cc71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { cpu.stos(2, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.r32(0x32cc728u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cc71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { cpu.stos(2, true); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.r32(0x32cc71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32cc730u32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x30c5f89; continue;
 }
 0x30c5f89 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x30c603f; continue; }
  pc = 0x30c5f93; continue;
 }
 0x30c5f93 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x32cc72cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c5fd6; continue; }
  pc = 0x30c5f9b; continue;
 }
 0x30c5f9b => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.w32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x32cc72cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edi, (cpu.ecx & 0xff)); cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x32cc720u32)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x30c5fcc; continue; }
  pc = 0x30c5fc8; continue;
 }
 0x30c5fc8 => {
  { cpu.stos(1, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c5fd4; continue; }
  pc = 0x30c5fcc; continue;
 }
 0x30c5fcc => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(2, true); }
  if !cpu.cf { pc = 0x30c5fd4; continue; }
  pc = 0x30c5fd3; continue;
 }
 0x30c5fd3 => {
  { cpu.stos(1, false); }
  pc = 0x30c5fd4; continue;
 }
 0x30c5fd4 => {
  pc = 0x30c603f; continue;
 }
 0x30c5fd6 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.w32(0x40u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x32cc72cu32)); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffffu32; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x32cc720u32)); cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30c603d; continue; }
  pc = 0x30c5ffc; continue;
 }
 0x30c5ffc => {
  { let v: u32 = cpu.r32(0x32cc730u32); cpu.ecx = v; }
  pc = 0x30c6002; continue;
 }
 0x30c6002 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edi)), 0x0u8); }
  if !cpu.zf { pc = 0x30c601e; continue; }
  pc = 0x30c6007; continue;
 }
 0x30c6007 => {
  { let v: u32 = cpu.r32(0x32cc724u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x32cc728u32); cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x30c601e; continue;
 }
 0x30c601e => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x32cc728u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x30c6032; continue; }
  pc = 0x30c602c; continue;
 }
 0x30c602c => {
  { let v: u32 = cpu.r32(0x32cc724u32); cpu.edi = v; }
  pc = 0x30c6032; continue;
 }
 0x30c6032 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c6002; continue; }
  pc = 0x30c6037; continue;
 }
 0x30c6037 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x32cc730u32, v); }
  pc = 0x30c603d; continue;
 }
 0x30c603d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x30c603f; continue;
 }
 0x30c603f => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x32cc71cu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c5f89; continue; }
  pc = 0x30c604d; continue;
 }
 0x30c604d => {
  { let v: u32 = cpu.esp.wrapping_add(0xc0u32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6057 => {
  { let v: u32 = cpu.esp.wrapping_add(0xc0u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6060); }
 let mut pc: u32 = 0x30c6060;
 loop { match pc {
 0x30c6060 => {
  { cpu.pushad(); }
  { let v: u32 = 0x315a6b0u32; cpu.esi = v; }
  { let v: u32 = 0x32cc87cu32; cpu.edi = v; }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { cpu.movs(1, true); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x32cc734u32; cpu.ecx = v; }
  { let v: u32 = 0x32cc834u32; cpu.edx = v; }
  { let v: u32 = 0x32cc858u32; cpu.ebp = v; }
  { let v: u32 = 0x32cc87cu32; cpu.esi = v; }
  cpu.push(0x30c6095); f_030c5ed7(cpu); if cpu.returned_to != 0x30c6095 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6097(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6097); }
 let mut pc: u32 = 0x30c6097;
 loop { match pc {
 0x30c6097 => {
  { cpu.pushad(); }
  { cpu.sub32(cpu.eax, 0x32u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c60b3; continue; }
  pc = 0x30c609d; continue;
 }
 0x30c609d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c60a6; continue; }
  pc = 0x30c60a1; continue;
 }
 0x30c60a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30c60a6; continue;
 }
 0x30c60a6 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x1388u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  pc = 0x30c60ba; continue;
 }
 0x30c60b3 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x30c60ba; continue;
 }
 0x30c60ba => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x30c60be; continue;
 }
 0x30c60be => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(0x32u32); cpu.eax = v; }
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c60d8; continue; }
  pc = 0x30c60d3; continue;
 }
 0x30c60d3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30c60d8; continue;
 }
 0x30c60d8 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c60e4; continue; }
  pc = 0x30c60df; continue;
 }
 0x30c60df => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x30c60e4; continue;
 }
 0x30c60e4 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x315a6c4u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x315a7c8u32.wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(8)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c60be; continue; }
  pc = 0x30c60fd; continue;
 }
 0x30c60fd => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c60ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c60ff); }
 let mut pc: u32 = 0x30c60ff;
 loop { match pc {
 0x30c60ff => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32ccc90u32, v); }
  pc = 0x30c6108; continue;
 }
 0x30c6108 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x40u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30c6177; continue; }
  pc = 0x30c6110; continue;
 }
 0x30c6110 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30c6177; continue; }
  pc = 0x30c6115; continue;
 }
 0x30c6115 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x80u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30c6177; continue; }
  pc = 0x30c611d; continue;
 }
 0x30c611d => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0xa0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30c6177; continue; }
  pc = 0x30c6125; continue;
 }
 0x30c6125 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0xc0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30c6177; continue; }
  pc = 0x30c612d; continue;
 }
 0x30c612d => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0xe0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30c6177; continue; }
  pc = 0x30c6135; continue;
 }
 0x30c6135 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x600u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x620u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x640u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x660u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x680u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6c0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x6e0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32ccc90u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c6108; continue; }
  pc = 0x30c6172; continue;
 }
 0x30c6172 => {
  pc = 0x30c6292; continue;
 }
 0x30c6177 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32ccc94u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x32ccc98u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32ccc9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x32ccca0u32, v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xe0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x1d9u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x115u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0xfffffd63u32); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32ccc94u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccc94u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x600u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x6e0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x32ccc98u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccc98u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x620u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x6c0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x32ccc9cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccc9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x640u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x6a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x32ccca0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccca0u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x680u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x660u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32ccc90u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c6108; continue; }
  pc = 0x30c6292; continue;
 }
 0x30c6292 => {
  { let v: u32 = cpu.esi.wrapping_sub(0xfffffa20u32); cpu.esi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32ccc90u32, v); }
  pc = 0x30c62a4; continue;
 }
 0x30c62a4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32ccc94u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x32ccc98u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32ccc9cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x32ccca0u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x16au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0x1d9u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x115u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0xfffffd63u32); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 8u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32ccc94u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccc94u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x32ccc98u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccc98u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x32ccc9cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccc9cu32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x32ccca0u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32ccca0u32); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32ccc90u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c62a4; continue; }
  pc = 0x30c63b2; continue;
 }
 0x30c63b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c63b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c63b3); }
 let mut pc: u32 = 0x30c63b3;
 loop { match pc {
 0x30c63b3 => {
  { let v: u32 = cpu.edi; cpu.w32(0x32ccca8u32, v); }
  { let v: u32 = 0x32cba04u32; cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = 0x32cbc04u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca8u32); cpu.edi = v; }
  cpu.push(0x30c63d3); f_030c6413(cpu); if cpu.returned_to != 0x30c63d3 { return; }
  { let v: u32 = 0x32cba84u32; cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = 0x32cbe04u32; cpu.esi = v; }
  cpu.push(0x30c63e7); f_030c6413(cpu); if cpu.returned_to != 0x30c63e7 { return; }
  { let v: u32 = 0x32cba14u32; cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = 0x32cbd04u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca8u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  cpu.push(0x30c6404); f_030c6413(cpu); if cpu.returned_to != 0x30c6404 { return; }
  { let v: u32 = 0x32cba94u32; cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = 0x32cbf04u32; cpu.esi = v; }
  f_030c6413(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6413(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c6413); } r_030c6413(cpu, 0x30c6413); }
pub fn f_030c649d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c649d); } r_030c6413(cpu, 0x30c649d); }
pub fn f_030c6523(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c6523); } r_030c6413(cpu, 0x30c6523); }
pub fn f_030c65a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c65a9); } r_030c6413(cpu, 0x30c65a9); }
pub fn f_030c662f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c662f); } r_030c6413(cpu, 0x30c662f); }
fn r_030c6413(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c6413 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cccacu32, v); }
  pc = 0x30c641e; continue;
 }
 0x30c641e => {
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x200u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca4u32).wrapping_add(0x20u32); cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a8u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccacu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c641e; continue; }
  pc = 0x30c664f; continue;
 }
 0x30c649d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x204u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca4u32).wrapping_add(0x20u32); cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a8u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccacu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c641e; continue; }
  pc = 0x30c664f; continue;
 }
 0x30c6523 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x208u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca4u32).wrapping_add(0x20u32); cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a8u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccacu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c641e; continue; }
  pc = 0x30c664f; continue;
 }
 0x30c65a9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x32ccca4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca4u32).wrapping_add(0x20u32); cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a8u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccacu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c641e; continue; }
  pc = 0x30c664f; continue;
 }
 0x30c662f => {
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ccca4u32).wrapping_add(0x20u32); cpu.w32(0x32ccca4u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a8u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccacu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c641e; continue; }
  pc = 0x30c664f; continue;
 }
 0x30c664f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6650); }
 let mut pc: u32 = 0x30c6650;
 loop { match pc {
 0x30c6650 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x315a6a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30c649du32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30c6523u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30c65a9u32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30c662fu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6675(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6675); }
 let mut pc: u32 = 0x30c6675;
 loop { match pc {
 0x30c6675 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32cccb0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32cccb4u32, v); }
  pc = 0x30c6686; continue;
 }
 0x30c6686 => {
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x204u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x208u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x214u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x210u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x11cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x21cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.esi.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x32cccb0u32).wrapping_add(0x20u32); cpu.w32(0x32cccb0u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a4u32)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32cccb0u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccb4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c6686; continue; }
  pc = 0x30c68f3; continue;
 }
 0x30c68f3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c68f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c68f4); }
 let mut pc: u32 = 0x30c68f4;
 loop { match pc {
 0x30c68f4 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32cccb8u32, v); }
  pc = 0x30c68ff; continue;
 }
 0x30c68ff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x104u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x204u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x120u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x220u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x24u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x124u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x224u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x208u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x10cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x20cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x128u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x228u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x2cu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x12cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x22cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x210u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x114u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x214u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x30u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x130u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x230u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x34u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x134u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x234u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x11cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x21cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x38u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x138u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x238u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3cu32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x13cu32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x23cu32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x32cf6bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d06bcu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32ce6bcu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x32cfebcu32.wrapping_add(cpu.ebx.wrapping_mul(4)))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32ceebcu32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp | cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x40u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x315a6a4u32)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x32cccb8u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x30c68ff; continue; }
  pc = 0x30c6ae6; continue;
 }
 0x30c6ae6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6ae7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6ae7); }
 let mut pc: u32 = 0x30c6ae7;
 loop { match pc {
 0x30c6ae7 => {
  { cpu.pushad(); }
  { let v: u32 = 0x315a704u32; cpu.esi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x30c6af4; continue;
 }
 0x30c6af4 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c6af4; continue; }
  pc = 0x30c6b0b; continue;
 }
 0x30c6b0b => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6b0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6b0d); }
 let mut pc: u32 = 0x30c6b0d;
 loop { match pc {
 0x30c6b0d => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x70u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0x700u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 8u32); cpu.ecx = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x32cdebcu32; cpu.edi = v; }
  cpu.push(0x30c6b33); f_030c6b54(cpu); if cpu.returned_to != 0x30c6b33 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = 0x32cd6bcu32; cpu.edi = v; }
  cpu.push(0x30c6b43); f_030c6b54(cpu); if cpu.returned_to != 0x30c6b43 { return; }
  { cpu.popad(); }
  { cpu.pushad(); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x32ccebcu32; cpu.edi = v; }
  cpu.push(0x30c6b51); f_030c6b54(cpu); if cpu.returned_to != 0x30c6b51 { return; }
  { cpu.popad(); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6b54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6b54); }
 let mut pc: u32 = 0x30c6b54;
 loop { match pc {
 0x30c6b54 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(0x200u32); cpu.edi = v; }
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x30c6b71; continue;
 }
 0x30c6b71 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315a8c8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esi; cpu.imul32(v); }
  { let v: u32 = 0xffu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x100u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30c6b71; continue; }
  pc = 0x30c6b8e; continue;
 }
 0x30c6b8e => {
  { let v: u32 = 0x80u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6b96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c6b96); } r_030c6b96(cpu, 0x30c6b96); }
pub fn f_030c6bf7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c6bf7); } r_030c6b96(cpu, 0x30c6bf7); }
fn r_030c6b96(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c6b96 => {
  { cpu.pushad(); }
  { let v: u32 = 0x100u32; cpu.w32(0x32d0cccu32, v); }
  { let v = (cpu.r32(0x32d0cccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x32d0cbcu32, v); cpu.fpop(); }
  { let v: u32 = 0x100u32; cpu.w32(0x32d0cccu32, v); }
  { let v = (cpu.r32(0x32d0cccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0x32d0cc4u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v: u32 = 0x32ce6bcu32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  cpu.push(0x30c6bd3); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6bd3 { return; }
  { let v = cpu.rf64(0x315a9c8u32); cpu.fpush(v); }
  { let v: u32 = 0x32ceebcu32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x32ccebcu32; cpu.esi = v; }
  cpu.push(0x30c6bed); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6bed { return; }
  { let v = cpu.rf64(0x315a9d0u32); cpu.fpush(v); }
  { let v: u32 = 0x32cf6bcu32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x32cd6bcu32; cpu.esi = v; }
  cpu.push(0x30c6c07); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6c07 { return; }
  { let v = cpu.rf64(0x315a9d8u32); cpu.fpush(v); }
  { let v: u32 = 0x32cfebcu32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x30c6c1e); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6c1e { return; }
  { let v = cpu.rf64(0x315a9e0u32); cpu.fpush(v); }
  { let v: u32 = 0x32d06bcu32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x32cdebcu32; cpu.esi = v; }
  cpu.push(0x30c6c38); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6c38 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6bf7 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xffffff80u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = 0x32cd6bcu32; cpu.esi = v; }
  cpu.push(0x30c6c07); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6c07 { return; }
  { let v = cpu.rf64(0x315a9d8u32); cpu.fpush(v); }
  { let v: u32 = 0x32cfebcu32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x30c6c1e); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6c1e { return; }
  { let v = cpu.rf64(0x315a9e0u32); cpu.fpush(v); }
  { let v: u32 = 0x32d06bcu32; cpu.edi = v; }
  { let v: u32 = 0xffffff80u32; cpu.eax = v; }
  { let v: u32 = 0x32cdebcu32; cpu.esi = v; }
  cpu.push(0x30c6c38); f_030c6c3a(cpu); if cpu.returned_to != 0x30c6c38 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6c3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6c3a); }
 let mut pc: u32 = 0x30c6c3a;
 loop { match pc {
 0x30c6c3a => {
  { let v = cpu.st(0) * cpu.rf64(0x32d0cbcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) / cpu.rf64(0x32d0cc4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cccu32, v); }
  { let v = (cpu.r32(0x32d0cccu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x30c6c5c; continue;
 }
 0x30c6c5c => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 2u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c6c5c; continue; }
  pc = 0x30c6c69; continue;
 }
 0x30c6c69 => {
  { let v: u32 = 0x100u32; cpu.ecx = v; }
  pc = 0x30c6c6e; continue;
 }
 0x30c6c6e => {
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 2u32); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c6c6e; continue; }
  pc = 0x30c6c7d; continue;
 }
 0x30c6c7d => {
  { let v: u32 = 0x80u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x30c6c84; continue;
 }
 0x30c6c84 => {
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 2u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c6c84; continue; }
  pc = 0x30c6c91; continue;
 }
 0x30c6c91 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6c96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6c96); }
 let mut pc: u32 = 0x30c6c96;
 loop { match pc {
 0x30c6c96 => {
  { cpu.pushad(); }
  cpu.push(0x30c6c9c); f_030c6b0d(cpu); if cpu.returned_to != 0x30c6c9c { return; }
  cpu.push(0x30c6ca1); f_030c6b96(cpu); if cpu.returned_to != 0x30c6ca1 { return; }
  { let v: u32 = 0x32cc890u32; cpu.edi = v; }
  cpu.push(0x30c6cab); f_030c6ae7(cpu); if cpu.returned_to != 0x30c6cab { return; }
  { let v: u32 = 0x32cca90u32; cpu.edi = v; }
  cpu.push(0x30c6cb5); f_030c6ae7(cpu); if cpu.returned_to != 0x30c6cb5 { return; }
  { let v: u32 = 0x32cca90u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32cb3ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x32cb3f0u32, v); }
  { let v: u32 = 0x32cc890u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32cb3f4u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x32cb3f8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x32cb3fcu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x32cb400u32, v); }
  cpu.push(0x30c6ce8); f_030c6060(cpu); if cpu.returned_to != 0x30c6ce8 { return; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x32cb404u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x32cb504u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x32cb604u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x32cb704u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x32cb804u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x32cb904u32; cpu.edi = v; }
  { let v: u32 = 0x40u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d40); }
 let mut pc: u32 = 0x30c6d40;
 loop { match pc {
 0x30c6d40 => {
  { let v: u32 = 0x0u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d4b); }
 let mut pc: u32 = 0x30c6d4b;
 loop { match pc {
 0x30c6d4b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d4c); }
 let mut pc: u32 = 0x30c6d4c;
 loop { match pc {
 0x30c6d4c => {
  { let v: u32 = 0x3154720u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d57); }
 let mut pc: u32 = 0x30c6d57;
 loop { match pc {
 0x30c6d57 => {
  { let v: u32 = 0x31546d8u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d62); }
 let mut pc: u32 = 0x30c6d62;
 loop { match pc {
 0x30c6d62 => {
  { let v: u32 = 0x31546fcu32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d6d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d6d); }
 let mut pc: u32 = 0x30c6d6d;
 loop { match pc {
 0x30c6d6d => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6d7d; continue; }
  pc = 0x30c6d7a; continue;
 }
 0x30c6d7a => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6d7d; continue;
 }
 0x30c6d7d => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6d90); f_030ba8f6(cpu); if cpu.returned_to != 0x30c6d90 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6d9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6d9b); }
 let mut pc: u32 = 0x30c6d9b;
 loop { match pc {
 0x30c6d9b => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6dab; continue; }
  pc = 0x30c6da8; continue;
 }
 0x30c6da8 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6dab; continue;
 }
 0x30c6dab => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6dc2); f_030ba8f6(cpu); if cpu.returned_to != 0x30c6dc2 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6dcd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6dcd); }
 let mut pc: u32 = 0x30c6dcd;
 loop { match pc {
 0x30c6dcd => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6ddd; continue; }
  pc = 0x30c6dda; continue;
 }
 0x30c6dda => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6ddd; continue;
 }
 0x30c6ddd => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6df4); f_030ba793(cpu); if cpu.returned_to != 0x30c6df4 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6dff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6dff); }
 let mut pc: u32 = 0x30c6dff;
 loop { match pc {
 0x30c6dff => {
  { let v: u32 = 0x3154624u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6e0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6e0a); }
 let mut pc: u32 = 0x30c6e0a;
 loop { match pc {
 0x30c6e0a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6e0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6e0b); }
 let mut pc: u32 = 0x30c6e0b;
 loop { match pc {
 0x30c6e0b => {
  { let v: u32 = 0x3154720u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6e16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6e16); }
 let mut pc: u32 = 0x30c6e16;
 loop { match pc {
 0x30c6e16 => {
  { let v: u32 = 0x31546d8u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6e21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6e21); }
 let mut pc: u32 = 0x30c6e21;
 loop { match pc {
 0x30c6e21 => {
  { let v: u32 = 0x31546fcu32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6e2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6e2c); }
 let mut pc: u32 = 0x30c6e2c;
 loop { match pc {
 0x30c6e2c => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6e3c; continue; }
  pc = 0x30c6e39; continue;
 }
 0x30c6e39 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6e3c; continue;
 }
 0x30c6e3c => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6e50); f_030bad4f(cpu); if cpu.returned_to != 0x30c6e50 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6e5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6e5b); }
 let mut pc: u32 = 0x30c6e5b;
 loop { match pc {
 0x30c6e5b => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6e6b; continue; }
  pc = 0x30c6e68; continue;
 }
 0x30c6e68 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6e6b; continue;
 }
 0x30c6e6b => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x30c6ea8; continue; }
  pc = 0x30c6e7f; continue;
 }
 0x30c6e7f => {
  { let v: u32 = 0x3154690u32; cpu.edi = v; }
  cpu.push(0x30c6e89); f_030ba8f6(cpu); if cpu.returned_to != 0x30c6e89 { return; }
  { let v: u32 = 0x3154690u32; cpu.esi = v; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6e9d); f_030baa5f(cpu); if cpu.returned_to != 0x30c6e9d { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6ea8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3154690u32; cpu.edi = v; }
  cpu.push(0x30c6eb5); f_030ba759(cpu); if cpu.returned_to != 0x30c6eb5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6ec5); f_030bad4f(cpu); if cpu.returned_to != 0x30c6ec5 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6ed0); }
 let mut pc: u32 = 0x30c6ed0;
 loop { match pc {
 0x30c6ed0 => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6ee0; continue; }
  pc = 0x30c6edd; continue;
 }
 0x30c6edd => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6ee0; continue;
 }
 0x30c6ee0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x30c6f1d; continue; }
  pc = 0x30c6ef4; continue;
 }
 0x30c6ef4 => {
  { let v: u32 = 0x3154690u32; cpu.edi = v; }
  cpu.push(0x30c6efe); f_030ba793(cpu); if cpu.returned_to != 0x30c6efe { return; }
  { let v: u32 = 0x3154690u32; cpu.esi = v; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6f12); f_030baa5f(cpu); if cpu.returned_to != 0x30c6f12 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6f1d => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3154690u32; cpu.edi = v; }
  cpu.push(0x30c6f2a); f_030ba759(cpu); if cpu.returned_to != 0x30c6f2a { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x3154624u32; cpu.ebx = v; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6f3a); f_030bad4f(cpu); if cpu.returned_to != 0x30c6f3a { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6f45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6f45); }
 let mut pc: u32 = 0x30c6f45;
 loop { match pc {
 0x30ba4f3 => {
  { cpu.sub32(cpu.r32(0x31547dcu32), 0x0u32); }
  if cpu.zf { pc = 0x30ba5ae; continue; }
  pc = 0x30ba500; continue;
 }
 0x30ba500 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d4u32, v); }
  { let v: u32 = cpu.r32(0x31547c0u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = 0x325ce60u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a8u32, v); }
  pc = 0x30ba530; continue;
 }
 0x30ba530 => {
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
  { cpu.stos(2, false); }
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
  { cpu.stos(2, false); }
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
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x32560a8u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30ba530; continue; }
  pc = 0x30ba5ac; continue;
 }
 0x30ba5ac => {
  pc = 0x30ba5d7; continue;
 }
 0x30ba5ae => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d4u32, v); }
  { let v: u32 = cpu.r32(0x31547c0u32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = 0x325ce60u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x30ba5d7; continue; }
  pc = 0x30ba5d5; continue;
 }
 0x30ba5d5 => {
  { cpu.movs(2, false); }
  pc = 0x30ba5d7; continue;
 }
 0x30ba5d7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6f45 => {
  cpu.poll(); pc = 0x30ba4f3; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6f4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6f4a); }
 let mut pc: u32 = 0x30c6f4a;
 loop { match pc {
 0x30ba3ff => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547d0u32, v); }
  { let v: u32 = cpu.r32(0x31547bcu32); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = 0x3258038u32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32560a8u32, v); }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if cpu.zf { pc = 0x30ba4c2; continue; }
  pc = 0x30ba43a; continue;
 }
 0x30ba43a => {
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e4u32)); cpu.eax = v; }
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
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e8u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x32560a8u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30ba43a; continue; }
  pc = 0x30ba4c0; continue;
 }
 0x30ba4c0 => {
  pc = 0x30ba4f2; continue;
 }
 0x30ba4c2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x31547e0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x31547e4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x31547e8u32)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x32560a8u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30ba4c2; continue; }
  pc = 0x30ba4f2; continue;
 }
 0x30ba4f2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6f4a => {
  cpu.poll(); pc = 0x30ba3ff; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6f4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6f4f); }
 let mut pc: u32 = 0x30c6f4f;
 loop { match pc {
 0x30c6f4f => {
  { let v: u32 = cpu.r32(0x3154800u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x30c6f5f; continue; }
  pc = 0x30c6f5c; continue;
 }
 0x30c6f5c => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x30c6f5f; continue;
 }
 0x30c6f5f => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.ebx = v; }
  { let v: u32 = 0x315aa60u32; cpu.esi = v; }
  cpu.push(0x30c6f72); f_030b9e6a(cpu); if cpu.returned_to != 0x30c6f72 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x31547e0u32).wrapping_add(cpu.eax); cpu.w32(0x31547e0u32, v); }
  { let v: u32 = cpu.r32(0x31547e4u32).wrapping_add(cpu.ebx); cpu.w32(0x31547e4u32, v); }
  { let v: u32 = cpu.add32(cpu.r32(0x31547e8u32), cpu.edx); cpu.w32(0x31547e8u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x3154690u32; cpu.edi = v; }
  cpu.push(0x30c6f9c); f_030ba8f6(cpu); if cpu.returned_to != 0x30c6f9c { return; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.esi = v; }
  { let v: u32 = 0x32d0ce4u32; cpu.edi = v; }
  { let v: u32 = 0x9u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x3154690u32; cpu.esi = v; }
  { let v: u32 = 0x32d0ce4u32; cpu.ebx = v; }
  { let v: u32 = 0x31546b4u32; cpu.edi = v; }
  cpu.push(0x30c6fc2); f_030baa5f(cpu); if cpu.returned_to != 0x30c6fc2 { return; }
  { let v: u32 = 0x31546b4u32; cpu.w32(0x31547dcu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6fcd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c6fcd); } r_030c6fcd(cpu, 0x30c6fcd); }
pub fn f_030c70ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c70ad); } r_030c6fcd(cpu, 0x30c70ad); }
pub fn f_030c70b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c70b4); } r_030c6fcd(cpu, 0x30c70b4); }
pub fn f_030c70bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c70bb); } r_030c6fcd(cpu, 0x30c70bb); }
pub fn f_030c70c2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c70c2); } r_030c6fcd(cpu, 0x30c70c2); }
fn r_030c6fcd(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30bb956 => {
  { cpu.sub32(cpu.r32(0x3154804u32), 0x0u32); }
  if !cpu.zf { pc = 0x30bb969; continue; }
  pc = 0x30bb95f; continue;
 }
 0x30bb95f => {
  { let v: u32 = cpu.add32(cpu.r32(0x31547a8u32), 0xb0u32); cpu.w32(0x31547a8u32, v); }
  pc = 0x30bb969; continue;
 }
 0x30bb969 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6fcd => {
  cpu.poll(); pc = 0x30bb956; continue;
 }
 0x30c7096 => {
  { let v: u32 = cpu.r32(0x31547e4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x30c70ac; continue; }
  pc = 0x30c70a7; continue;
 }
 0x30c70a7 => {
  cpu.poll(); pc = 0x30bb956; continue;
 }
 0x30c70ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c70ad => {
  { let v: u32 = 0x315aa20u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c7096; continue;
 }
 0x30c70b4 => {
  { let v: u32 = 0x315aa30u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c7096; continue;
 }
 0x30c70bb => {
  { let v: u32 = 0x315aa40u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c7096; continue;
 }
 0x30c70c2 => {
  { let v: u32 = 0x315aa50u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c7096; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6fd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6fd2); }
 let mut pc: u32 = 0x30c6fd2;
 loop { match pc {
 0x30c6fd2 => {
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30c6fe9; continue; }
  pc = 0x30c6fdd; continue;
 }
 0x30c6fdd => {
  { let v: u32 = cpu.eax; cpu.w32(0x31547fcu32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30c6fe9; continue;
 }
 0x30c6fe9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6fea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6fea); }
 let mut pc: u32 = 0x30c6fea;
 loop { match pc {
 0x30c6fea => {
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547e4u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x31547f8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c6ffe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c6ffe); }
 let mut pc: u32 = 0x30c6ffe;
 loop { match pc {
 0x30bc6a1 => {
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a44u32, v); }
  { let v: u32 = 0x30bd48eu32; cpu.w32(0x3154a48u32, v); }
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a38u32, v); }
  { let v: u32 = 0x30bf8f9u32; cpu.w32(0x3154a3cu32, v); }
  { let v: u32 = 0x30bd22fu32; cpu.w32(0x3154a20u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bc6d4 => {
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a44u32, v); }
  { let v: u32 = 0x30be071u32; cpu.w32(0x3154a48u32, v); }
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a38u32, v); }
  { let v: u32 = 0x30bf8f9u32; cpu.w32(0x3154a3cu32, v); }
  { let v: u32 = 0x30bd22fu32; cpu.w32(0x3154a20u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30bc707 => {
  { let v: u32 = 0x30bcd4au32; cpu.w32(0x3154a44u32, v); }
  { let v: u32 = 0x30be071u32; cpu.w32(0x3154a48u32, v); }
  { let v: u32 = 0x30bcd4au32; cpu.w32(0x3154a38u32, v); }
  { let v: u32 = 0x30bf538u32; cpu.w32(0x3154a3cu32, v); }
  { let v: u32 = 0x30bd22fu32; cpu.w32(0x3154a20u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c6ffe => {
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x31547e4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30c702b; continue; }
  pc = 0x30c7018; continue;
 }
 0x30c7018 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x30c7025; continue; }
  pc = 0x30c701f; continue;
 }
 0x30c701f => {
  cpu.poll(); pc = 0x30bc707; continue;
 }
 0x30c7025 => {
  cpu.poll(); pc = 0x30bc6d4; continue;
 }
 0x30c702b => {
  cpu.poll(); pc = 0x30bc6a1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7031(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7031); }
 let mut pc: u32 = 0x30c7031;
 loop { match pc {
 0x30bc73a => {
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a44u32, v); }
  { let v: u32 = 0x30be766u32; cpu.w32(0x3154a48u32, v); }
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a38u32, v); }
  { let v: u32 = 0x30bf538u32; cpu.w32(0x3154a3cu32, v); }
  { let v: u32 = 0x30be6bdu32; cpu.w32(0x3154a20u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c7031 => {
  cpu.poll(); pc = 0x30bc73a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7036(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7036); }
 let mut pc: u32 = 0x30c7036;
 loop { match pc {
 0x30bc76d => {
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a44u32, v); }
  { let v: u32 = 0x30bedbfu32; cpu.w32(0x3154a48u32, v); }
  { let v: u32 = 0x30bc955u32; cpu.w32(0x3154a38u32, v); }
  { let v: u32 = 0x30bf538u32; cpu.w32(0x3154a3cu32, v); }
  { let v: u32 = 0x30bd22fu32; cpu.w32(0x3154a20u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c7036 => {
  cpu.poll(); pc = 0x30bc76d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c707a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c707a); } r_030c707a(cpu, 0x30c707a); }
pub fn f_030c7081(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c7081); } r_030c707a(cpu, 0x30c7081); }
pub fn f_030c7088(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c7088); } r_030c707a(cpu, 0x30c7088); }
pub fn f_030c708f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30c708f); } r_030c707a(cpu, 0x30c708f); }
fn r_030c707a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30c703b => {
  { let v: u32 = cpu.r32(0x31547e4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x31547fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  pc = 0x30c7050; continue;
 }
 0x30c7050 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x30c7060; continue; }
  pc = 0x30c7054; continue;
 }
 0x30c7054 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c7050; continue; }
  pc = 0x30c705d; continue;
 }
 0x30c705d => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x30c7060; continue;
 }
 0x30c7060 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3157f84u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30c7079; continue; }
  pc = 0x30c706d; continue;
 }
 0x30c706d => {
  { let v: u32 = cpu.eax; cpu.w32(0x31547fcu32, v); }
  { let v: u32 = cpu.add32(cpu.eax, 0x2eu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30c7079; continue;
 }
 0x30c7079 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c707a => {
  { let v: u32 = 0x315aa20u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c703b; continue;
 }
 0x30c7081 => {
  { let v: u32 = 0x315aa30u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c703b; continue;
 }
 0x30c7088 => {
  { let v: u32 = 0x315aa40u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c703b; continue;
 }
 0x30c708f => {
  { let v: u32 = 0x315aa50u32; cpu.edi = v; }
  cpu.poll(); pc = 0x30c703b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c70c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c70c9); }
 let mut pc: u32 = 0x30c70c9;
 loop { match pc {
 0x30c70c9 => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.cf { pc = 0x30c70e0; continue; }
  pc = 0x30c70ce; continue;
 }
 0x30c70ce => {
  { cpu.sub32(cpu.ebx, 0x4u32); }
  if !cpu.cf { pc = 0x30c70e0; continue; }
  pc = 0x30c70d3; continue;
 }
 0x30c70d3 => {
  { let v: u32 = cpu.r32(0x315aa10u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c70e0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c70e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c70e6); }
 let mut pc: u32 = 0x30c70e6;
 loop { match pc {
 0x30c70e6 => {
  { let v: u32 = 0x30u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x31547e0u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.esi = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x30c710b; continue; }
  pc = 0x30c7109; continue;
 }
 0x30c7109 => {
  { cpu.movs(2, false); }
  pc = 0x30c710b; continue;
 }
 0x30c710b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c710c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c710c); }
 let mut pc: u32 = 0x30c710c;
 loop { match pc {
 0x30c710c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315aa60u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315aa64u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x315aa68u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547bcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31547c0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x31547c4u32, v); }
  { let v: u32 = cpu.r32(0x315aa6cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.esi = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x30c715f; continue; }
  pc = 0x30c715d; continue;
 }
 0x30c715d => {
  { cpu.movs(2, false); }
  pc = 0x30c715f; continue;
 }
 0x30c715f => {
  { let v: u32 = 0x31547e0u32; cpu.esi = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.edi; cpu.w32(0x315aa6cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7172(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7172); }
 let mut pc: u32 = 0x30c7172;
 loop { match pc {
 0x30c7172 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x31547bcu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x31547c0u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x31547c4u32, v); }
  { let v: u32 = cpu.r32(0x315aa6cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x34u32); cpu.esi = v; }
  { cpu.lods(4, false); }
  { let v: u32 = cpu.eax; cpu.w32(0x31547dcu32, v); }
  { let v: u32 = cpu.r32(0x31547dcu32); cpu.edi = v; }
  { let v: u32 = 0x24u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { pc = 0x30c71b5; continue; }
  pc = 0x30c71b3; continue;
 }
 0x30c71b3 => {
  { cpu.movs(2, false); }
  pc = 0x30c71b5; continue;
 }
 0x30c71b5 => {
  { let v: u32 = 0x31547e0u32; cpu.edi = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.sub32(cpu.r32(0x315aa6cu32), 0x34u32); cpu.w32(0x315aa6cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c71d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c71d0); }
 let mut pc: u32 = 0x30c71d0;
 loop { match pc {
 0x30c71d0 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x32d101cu32; cpu.w32(0x32d1018u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x32d11ccu32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c71ec; continue;
 }
 0x30c71ec => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c722f; continue; }
  pc = 0x30c71fa; continue;
 }
 0x30c71fa => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3155990u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30c7255; continue; }
  pc = 0x30c7202; continue;
 }
 0x30c7202 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c7223; continue; }
  pc = 0x30c720f; continue;
 }
 0x30c720f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30c7279; continue; }
  pc = 0x30c7217; continue;
 }
 0x30c7217 => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c7221); f_030c7290(cpu); if cpu.returned_to != 0x30c7221 { return; }
  pc = 0x30c7279; continue;
 }
 0x30c7223 => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c722d); f_030c7290(cpu); if cpu.returned_to != 0x30c722d { return; }
  pc = 0x30c7279; continue;
 }
 0x30c722f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c7279; continue; }
  pc = 0x30c7237; continue;
 }
 0x30c7237 => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c7241); f_030c7290(cpu); if cpu.returned_to != 0x30c7241 { return; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30c7279; continue; }
  pc = 0x30c7249; continue;
 }
 0x30c7249 => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c7253); f_030c7290(cpu); if cpu.returned_to != 0x30c7253 { return; }
  pc = 0x30c7279; continue;
 }
 0x30c7255 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30c7279; continue; }
  pc = 0x30c725d; continue;
 }
 0x30c725d => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c7267); f_030c7290(cpu); if cpu.returned_to != 0x30c7267 { return; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c7279; continue; }
  pc = 0x30c726f; continue;
 }
 0x30c726f => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c7279); f_030c7290(cpu); if cpu.returned_to != 0x30c7279 { return; }
  pc = 0x30c7279; continue;
 }
 0x30c7279 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c71ec; continue; }
  pc = 0x30c7287; continue;
 }
 0x30c7287 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7290); }
 let mut pc: u32 = 0x30c7290;
 loop { match pc {
 0x30c7290 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d1018u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d1018u32).wrapping_add(0x24u32); cpu.w32(0x32d1018u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x31559a0u32)); }
  if cpu.zf { pc = 0x30c72ee; continue; }
  pc = 0x30c72bd; continue;
 }
 0x30c72bd => {
  { let v: u32 = cpu.r32(0x3155994u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155978u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x30c7329; continue;
 }
 0x30c72ee => {
  { let v: u32 = cpu.r32(0x31559a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155998u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155998u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155978u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x30c7329; continue;
 }
 0x30c7329 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c732d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c732d); }
 let mut pc: u32 = 0x30c732d;
 loop { match pc {
 0x30c732d => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x32d101cu32; cpu.w32(0x32d1018u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x32d11ccu32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c7349; continue;
 }
 0x30c7349 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c738c; continue; }
  pc = 0x30c7357; continue;
 }
 0x30c7357 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3155990u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30c73b2; continue; }
  pc = 0x30c735f; continue;
 }
 0x30c735f => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c7380; continue; }
  pc = 0x30c736c; continue;
 }
 0x30c736c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30c73d6; continue; }
  pc = 0x30c7374; continue;
 }
 0x30c7374 => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c737e); f_030c73ed(cpu); if cpu.returned_to != 0x30c737e { return; }
  pc = 0x30c73d6; continue;
 }
 0x30c7380 => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c738a); f_030c73ed(cpu); if cpu.returned_to != 0x30c738a { return; }
  pc = 0x30c73d6; continue;
 }
 0x30c738c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c73d6; continue; }
  pc = 0x30c7394; continue;
 }
 0x30c7394 => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c739e); f_030c73ed(cpu); if cpu.returned_to != 0x30c739e { return; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30c73d6; continue; }
  pc = 0x30c73a6; continue;
 }
 0x30c73a6 => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c73b0); f_030c73ed(cpu); if cpu.returned_to != 0x30c73b0 { return; }
  pc = 0x30c73d6; continue;
 }
 0x30c73b2 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30c73d6; continue; }
  pc = 0x30c73ba; continue;
 }
 0x30c73ba => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c73c4); f_030c73ed(cpu); if cpu.returned_to != 0x30c73c4 { return; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c73d6; continue; }
  pc = 0x30c73cc; continue;
 }
 0x30c73cc => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c73d6); f_030c73ed(cpu); if cpu.returned_to != 0x30c73d6 { return; }
  pc = 0x30c73d6; continue;
 }
 0x30c73d6 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c7349; continue; }
  pc = 0x30c73e4; continue;
 }
 0x30c73e4 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c73ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c73ed); }
 let mut pc: u32 = 0x30c73ed;
 loop { match pc {
 0x30c73ed => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d1018u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d1018u32).wrapping_add(0x24u32); cpu.w32(0x32d1018u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x31559a0u32)); }
  if cpu.zf { pc = 0x30c744b; continue; }
  pc = 0x30c741a; continue;
 }
 0x30c741a => {
  { let v: u32 = cpu.r32(0x3155994u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155978u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x30c7486; continue;
 }
 0x30c744b => {
  { let v: u32 = cpu.r32(0x31559a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155998u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155998u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155978u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x30c7486; continue;
 }
 0x30c7486 => {
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7498(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7498); }
 let mut pc: u32 = 0x30c7498;
 loop { match pc {
 0x30c7498 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x32d101cu32; cpu.w32(0x32d1018u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x32d11ccu32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c74b4; continue;
 }
 0x30c74b4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c74f7; continue; }
  pc = 0x30c74c2; continue;
 }
 0x30c74c2 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x3155990u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30c751d; continue; }
  pc = 0x30c74ca; continue;
 }
 0x30c74ca => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c74eb; continue; }
  pc = 0x30c74d7; continue;
 }
 0x30c74d7 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30c7541; continue; }
  pc = 0x30c74df; continue;
 }
 0x30c74df => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c74e9); f_030c7558(cpu); if cpu.returned_to != 0x30c74e9 { return; }
  pc = 0x30c7541; continue;
 }
 0x30c74eb => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c74f5); f_030c7558(cpu); if cpu.returned_to != 0x30c74f5 { return; }
  pc = 0x30c7541; continue;
 }
 0x30c74f7 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c7541; continue; }
  pc = 0x30c74ff; continue;
 }
 0x30c74ff => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c7509); f_030c7558(cpu); if cpu.returned_to != 0x30c7509 { return; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf == cpu.of) { pc = 0x30c7541; continue; }
  pc = 0x30c7511; continue;
 }
 0x30c7511 => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c751b); f_030c7558(cpu); if cpu.returned_to != 0x30c751b { return; }
  pc = 0x30c7541; continue;
 }
 0x30c751d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155990u32)); }
  if (cpu.sf != cpu.of) { pc = 0x30c7541; continue; }
  pc = 0x30c7525; continue;
 }
 0x30c7525 => {
  { let v: u32 = 0x3155990u32; cpu.eax = v; }
  cpu.push(0x30c752f); f_030c7558(cpu); if cpu.returned_to != 0x30c752f { return; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x31559a0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c7541; continue; }
  pc = 0x30c7537; continue;
 }
 0x30c7537 => {
  { let v: u32 = 0x31559a0u32; cpu.eax = v; }
  cpu.push(0x30c7541); f_030c7558(cpu); if cpu.returned_to != 0x30c7541 { return; }
  pc = 0x30c7541; continue;
 }
 0x30c7541 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c74b4; continue; }
  pc = 0x30c754f; continue;
 }
 0x30c754f => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7558(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7558); }
 let mut pc: u32 = 0x30c7558;
 loop { match pc {
 0x30c7558 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d1018u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d1018u32).wrapping_add(0x24u32); cpu.w32(0x32d1018u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x31559a0u32)); }
  if cpu.zf { pc = 0x30c75b6; continue; }
  pc = 0x30c7585; continue;
 }
 0x30c7585 => {
  { let v: u32 = cpu.r32(0x3155994u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155978u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x30c75f1; continue;
 }
 0x30c75b6 => {
  { let v: u32 = cpu.r32(0x31559a4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x8u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155998u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x10u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155998u32); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155978u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  pc = 0x30c75f1; continue;
 }
 0x30c75f1 => {
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7611(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7611); }
 let mut pc: u32 = 0x30c7611;
 loop { match pc {
 0x30c7611 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x32d12f0u32; cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x32d14a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c762c; continue;
 }
 0x30c762c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155984u32)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c766f; continue; }
  pc = 0x30c7639; continue;
 }
 0x30c7639 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155984u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7655; continue; }
  pc = 0x30c7646; continue;
 }
 0x30c7646 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c762c; continue; }
  pc = 0x30c7650; continue;
 }
 0x30c7650 => {
  pc = 0x30c76d5; continue;
 }
 0x30c7655 => {
  if (cpu.sf != cpu.of) { pc = 0x30c7663; continue; }
  pc = 0x30c7657; continue;
 }
 0x30c7657 => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c7661); f_030c77a7(cpu); if cpu.returned_to != 0x30c7661 { return; }
  pc = 0x30c76c7; continue;
 }
 0x30c7663 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c766d); f_030c77a7(cpu); if cpu.returned_to != 0x30c766d { return; }
  pc = 0x30c76c7; continue;
 }
 0x30c766f => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155984u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7693; continue; }
  pc = 0x30c7677; continue;
 }
 0x30c7677 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c7687; continue; }
  pc = 0x30c767b; continue;
 }
 0x30c767b => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c7685); f_030c77a7(cpu); if cpu.returned_to != 0x30c7685 { return; }
  pc = 0x30c76c7; continue;
 }
 0x30c7687 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7691); f_030c77a7(cpu); if cpu.returned_to != 0x30c7691 { return; }
  pc = 0x30c76c7; continue;
 }
 0x30c7693 => {
  if (cpu.sf != cpu.of) { pc = 0x30c76af; continue; }
  pc = 0x30c7695; continue;
 }
 0x30c7695 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c76c7; continue; }
  pc = 0x30c7699; continue;
 }
 0x30c7699 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c76a3); f_030c77a7(cpu); if cpu.returned_to != 0x30c76a3 { return; }
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c76ad); f_030c77a7(cpu); if cpu.returned_to != 0x30c76ad { return; }
  pc = 0x30c76c7; continue;
 }
 0x30c76af => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c76c7; continue; }
  pc = 0x30c76b3; continue;
 }
 0x30c76b3 => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c76bd); f_030c77a7(cpu); if cpu.returned_to != 0x30c76bd { return; }
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c76c7); f_030c77a7(cpu); if cpu.returned_to != 0x30c76c7 { return; }
  pc = 0x30c76c7; continue;
 }
 0x30c76c7 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c762c; continue; }
  pc = 0x30c76d5; continue;
 }
 0x30c76d5 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c77a4; continue; }
  pc = 0x30c76e0; continue;
 }
 0x30c76e0 => {
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c76f1; continue;
 }
 0x30c76f1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155988u32)); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7735; continue; }
  pc = 0x30c76ff; continue;
 }
 0x30c76ff => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155988u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c771b; continue; }
  pc = 0x30c770c; continue;
 }
 0x30c770c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c76f1; continue; }
  pc = 0x30c7716; continue;
 }
 0x30c7716 => {
  pc = 0x30c779b; continue;
 }
 0x30c771b => {
  if (cpu.sf != cpu.of) { pc = 0x30c7729; continue; }
  pc = 0x30c771d; continue;
 }
 0x30c771d => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7727); f_030c780a(cpu); if cpu.returned_to != 0x30c7727 { return; }
  pc = 0x30c778d; continue;
 }
 0x30c7729 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7733); f_030c780a(cpu); if cpu.returned_to != 0x30c7733 { return; }
  pc = 0x30c778d; continue;
 }
 0x30c7735 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155988u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7759; continue; }
  pc = 0x30c773d; continue;
 }
 0x30c773d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c774d; continue; }
  pc = 0x30c7741; continue;
 }
 0x30c7741 => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c774b); f_030c780a(cpu); if cpu.returned_to != 0x30c774b { return; }
  pc = 0x30c778d; continue;
 }
 0x30c774d => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7757); f_030c780a(cpu); if cpu.returned_to != 0x30c7757 { return; }
  pc = 0x30c778d; continue;
 }
 0x30c7759 => {
  if (cpu.sf != cpu.of) { pc = 0x30c7775; continue; }
  pc = 0x30c775b; continue;
 }
 0x30c775b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c778d; continue; }
  pc = 0x30c775f; continue;
 }
 0x30c775f => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7769); f_030c780a(cpu); if cpu.returned_to != 0x30c7769 { return; }
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7773); f_030c780a(cpu); if cpu.returned_to != 0x30c7773 { return; }
  pc = 0x30c778d; continue;
 }
 0x30c7775 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c778d; continue; }
  pc = 0x30c7779; continue;
 }
 0x30c7779 => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7783); f_030c780a(cpu); if cpu.returned_to != 0x30c7783 { return; }
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c778d); f_030c780a(cpu); if cpu.returned_to != 0x30c778d { return; }
  pc = 0x30c778d; continue;
 }
 0x30c778d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c76f1; continue; }
  pc = 0x30c779b; continue;
 }
 0x30c779b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c77a4 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c77a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c77a7); }
 let mut pc: u32 = 0x30c77a7;
 loop { match pc {
 0x30c77a7 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d12ecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d12ecu32).wrapping_add(0x24u32); cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c780a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c780a); }
 let mut pc: u32 = 0x30c780a;
 loop { match pc {
 0x30c780a => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d12ecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d12ecu32).wrapping_add(0x24u32); cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c786f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c786f); }
 let mut pc: u32 = 0x30c786f;
 loop { match pc {
 0x30c786f => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x32d12f0u32; cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x32d14a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c788a; continue;
 }
 0x30c788a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155984u32)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c78cd; continue; }
  pc = 0x30c7897; continue;
 }
 0x30c7897 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155984u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c78b3; continue; }
  pc = 0x30c78a4; continue;
 }
 0x30c78a4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c788a; continue; }
  pc = 0x30c78ae; continue;
 }
 0x30c78ae => {
  pc = 0x30c7933; continue;
 }
 0x30c78b3 => {
  if (cpu.sf != cpu.of) { pc = 0x30c78c1; continue; }
  pc = 0x30c78b5; continue;
 }
 0x30c78b5 => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c78bf); f_030c7a05(cpu); if cpu.returned_to != 0x30c78bf { return; }
  pc = 0x30c7925; continue;
 }
 0x30c78c1 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c78cb); f_030c7a05(cpu); if cpu.returned_to != 0x30c78cb { return; }
  pc = 0x30c7925; continue;
 }
 0x30c78cd => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155984u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c78f1; continue; }
  pc = 0x30c78d5; continue;
 }
 0x30c78d5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c78e5; continue; }
  pc = 0x30c78d9; continue;
 }
 0x30c78d9 => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c78e3); f_030c7a05(cpu); if cpu.returned_to != 0x30c78e3 { return; }
  pc = 0x30c7925; continue;
 }
 0x30c78e5 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c78ef); f_030c7a05(cpu); if cpu.returned_to != 0x30c78ef { return; }
  pc = 0x30c7925; continue;
 }
 0x30c78f1 => {
  if (cpu.sf != cpu.of) { pc = 0x30c790d; continue; }
  pc = 0x30c78f3; continue;
 }
 0x30c78f3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c7925; continue; }
  pc = 0x30c78f7; continue;
 }
 0x30c78f7 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7901); f_030c7a05(cpu); if cpu.returned_to != 0x30c7901 { return; }
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c790b); f_030c7a05(cpu); if cpu.returned_to != 0x30c790b { return; }
  pc = 0x30c7925; continue;
 }
 0x30c790d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c7925; continue; }
  pc = 0x30c7911; continue;
 }
 0x30c7911 => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c791b); f_030c7a05(cpu); if cpu.returned_to != 0x30c791b { return; }
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7925); f_030c7a05(cpu); if cpu.returned_to != 0x30c7925 { return; }
  pc = 0x30c7925; continue;
 }
 0x30c7925 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c788a; continue; }
  pc = 0x30c7933; continue;
 }
 0x30c7933 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c7a02; continue; }
  pc = 0x30c793e; continue;
 }
 0x30c793e => {
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c794f; continue;
 }
 0x30c794f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155988u32)); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7993; continue; }
  pc = 0x30c795d; continue;
 }
 0x30c795d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155988u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7979; continue; }
  pc = 0x30c796a; continue;
 }
 0x30c796a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c794f; continue; }
  pc = 0x30c7974; continue;
 }
 0x30c7974 => {
  pc = 0x30c79f9; continue;
 }
 0x30c7979 => {
  if (cpu.sf != cpu.of) { pc = 0x30c7987; continue; }
  pc = 0x30c797b; continue;
 }
 0x30c797b => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7985); f_030c7a76(cpu); if cpu.returned_to != 0x30c7985 { return; }
  pc = 0x30c79eb; continue;
 }
 0x30c7987 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7991); f_030c7a76(cpu); if cpu.returned_to != 0x30c7991 { return; }
  pc = 0x30c79eb; continue;
 }
 0x30c7993 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155988u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c79b7; continue; }
  pc = 0x30c799b; continue;
 }
 0x30c799b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c79ab; continue; }
  pc = 0x30c799f; continue;
 }
 0x30c799f => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c79a9); f_030c7a76(cpu); if cpu.returned_to != 0x30c79a9 { return; }
  pc = 0x30c79eb; continue;
 }
 0x30c79ab => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c79b5); f_030c7a76(cpu); if cpu.returned_to != 0x30c79b5 { return; }
  pc = 0x30c79eb; continue;
 }
 0x30c79b7 => {
  if (cpu.sf != cpu.of) { pc = 0x30c79d3; continue; }
  pc = 0x30c79b9; continue;
 }
 0x30c79b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c79eb; continue; }
  pc = 0x30c79bd; continue;
 }
 0x30c79bd => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c79c7); f_030c7a76(cpu); if cpu.returned_to != 0x30c79c7 { return; }
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c79d1); f_030c7a76(cpu); if cpu.returned_to != 0x30c79d1 { return; }
  pc = 0x30c79eb; continue;
 }
 0x30c79d3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c79eb; continue; }
  pc = 0x30c79d7; continue;
 }
 0x30c79d7 => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c79e1); f_030c7a76(cpu); if cpu.returned_to != 0x30c79e1 { return; }
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c79eb); f_030c7a76(cpu); if cpu.returned_to != 0x30c79eb { return; }
  pc = 0x30c79eb; continue;
 }
 0x30c79eb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c794f; continue; }
  pc = 0x30c79f9; continue;
 }
 0x30c79f9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c7a02 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7a05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7a05); }
 let mut pc: u32 = 0x30c7a05;
 loop { match pc {
 0x30c7a05 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d12ecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d12ecu32).wrapping_add(0x24u32); cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7a76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7a76); }
 let mut pc: u32 = 0x30c7a76;
 loop { match pc {
 0x30c7a76 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d12ecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d12ecu32).wrapping_add(0x24u32); cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7ae9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7ae9); }
 let mut pc: u32 = 0x30c7ae9;
 loop { match pc {
 0x30c7ae9 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0x32d12f0u32; cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = 0x32d14a0u32; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c7b04; continue;
 }
 0x30c7b04 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155984u32)); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7b47; continue; }
  pc = 0x30c7b11; continue;
 }
 0x30c7b11 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155984u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7b2d; continue; }
  pc = 0x30c7b1e; continue;
 }
 0x30c7b1e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c7b04; continue; }
  pc = 0x30c7b28; continue;
 }
 0x30c7b28 => {
  pc = 0x30c7bad; continue;
 }
 0x30c7b2d => {
  if (cpu.sf != cpu.of) { pc = 0x30c7b3b; continue; }
  pc = 0x30c7b2f; continue;
 }
 0x30c7b2f => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c7b39); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b39 { return; }
  pc = 0x30c7b9f; continue;
 }
 0x30c7b3b => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7b45); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b45 { return; }
  pc = 0x30c7b9f; continue;
 }
 0x30c7b47 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155984u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7b6b; continue; }
  pc = 0x30c7b4f; continue;
 }
 0x30c7b4f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c7b5f; continue; }
  pc = 0x30c7b53; continue;
 }
 0x30c7b53 => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c7b5d); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b5d { return; }
  pc = 0x30c7b9f; continue;
 }
 0x30c7b5f => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7b69); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b69 { return; }
  pc = 0x30c7b9f; continue;
 }
 0x30c7b6b => {
  if (cpu.sf != cpu.of) { pc = 0x30c7b87; continue; }
  pc = 0x30c7b6d; continue;
 }
 0x30c7b6d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c7b9f; continue; }
  pc = 0x30c7b71; continue;
 }
 0x30c7b71 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7b7b); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b7b { return; }
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c7b85); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b85 { return; }
  pc = 0x30c7b9f; continue;
 }
 0x30c7b87 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c7b9f; continue; }
  pc = 0x30c7b8b; continue;
 }
 0x30c7b8b => {
  { let v: u32 = 0x3155984u32; cpu.eax = v; }
  cpu.push(0x30c7b95); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b95 { return; }
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7b9f); f_030c7c7f(cpu); if cpu.returned_to != 0x30c7b9f { return; }
  pc = 0x30c7b9f; continue;
 }
 0x30c7b9f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c7b04; continue; }
  pc = 0x30c7bad; continue;
 }
 0x30c7bad => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.esi); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c7c7c; continue; }
  pc = 0x30c7bb8; continue;
 }
 0x30c7bb8 => {
  { let v: u32 = cpu.sar32(cpu.ebp, 2u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x30c7bc9; continue;
 }
 0x30c7bc9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x3155988u32)); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7c0d; continue; }
  pc = 0x30c7bd7; continue;
 }
 0x30c7bd7 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155988u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7bf3; continue; }
  pc = 0x30c7be4; continue;
 }
 0x30c7be4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c7bc9; continue; }
  pc = 0x30c7bee; continue;
 }
 0x30c7bee => {
  pc = 0x30c7c73; continue;
 }
 0x30c7bf3 => {
  if (cpu.sf != cpu.of) { pc = 0x30c7c01; continue; }
  pc = 0x30c7bf5; continue;
 }
 0x30c7bf5 => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7bff); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7bff { return; }
  pc = 0x30c7c65; continue;
 }
 0x30c7c01 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7c0b); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c0b { return; }
  pc = 0x30c7c65; continue;
 }
 0x30c7c0d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3155988u32)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x30c7c31; continue; }
  pc = 0x30c7c15; continue;
 }
 0x30c7c15 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c7c25; continue; }
  pc = 0x30c7c19; continue;
 }
 0x30c7c19 => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7c23); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c23 { return; }
  pc = 0x30c7c65; continue;
 }
 0x30c7c25 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7c2f); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c2f { return; }
  pc = 0x30c7c65; continue;
 }
 0x30c7c31 => {
  if (cpu.sf != cpu.of) { pc = 0x30c7c4d; continue; }
  pc = 0x30c7c33; continue;
 }
 0x30c7c33 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30c7c65; continue; }
  pc = 0x30c7c37; continue;
 }
 0x30c7c37 => {
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7c41); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c41 { return; }
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7c4b); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c4b { return; }
  pc = 0x30c7c65; continue;
 }
 0x30c7c4d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30c7c65; continue; }
  pc = 0x30c7c51; continue;
 }
 0x30c7c51 => {
  { let v: u32 = 0x3155988u32; cpu.eax = v; }
  cpu.push(0x30c7c5b); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c5b { return; }
  { let v: u32 = 0x315bf08u32; cpu.eax = v; }
  cpu.push(0x30c7c65); f_030c7cfe(cpu); if cpu.returned_to != 0x30c7c65 { return; }
  pc = 0x30c7c65; continue;
 }
 0x30c7c65 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30c7bc9; continue; }
  pc = 0x30c7c73; continue;
 }
 0x30c7c73 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c7c7c => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7c7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7c7f); }
 let mut pc: u32 = 0x30c7c7f;
 loop { match pc {
 0x30c7c7f => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d12ecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d12ecu32).wrapping_add(0x24u32); cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7cfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7cfe); }
 let mut pc: u32 = 0x30c7cfe;
 loop { match pc {
 0x30c7cfe => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d12ecu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d12ecu32).wrapping_add(0x24u32); cpu.w32(0x32d12ecu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0xcu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x18u32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x1cu32.wrapping_add(cpu.ecx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.esi), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030c7d7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30c7d7f); }
 let mut pc: u32 = 0x30c7d7f;
 loop { match pc {
 0x30c7d7f => {
  { let v: u32 = cpu.r32(0x315598cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c8035; continue; }
  pc = 0x30c7d8d; continue;
 }
 0x30c7d8d => {
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x32d16f4u32, v); }
  { let v = cpu.st(0) * cpu.rf32(0x3155990u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x315bf0cu32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x315bf0cu32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3155984u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x32d16e0u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x32d16e0u32), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c802f; continue; }
  pc = 0x30c7dd5; continue;
 }
 0x30c7dd5 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3155978u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x3155988u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x32d16e4u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x32d16e4u32), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30c802f; continue; }
  pc = 0x30c7df9; continue;
 }
 0x30c7df9 => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x3155974u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d1728u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x32d1728u32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c802f; continue; }
  pc = 0x30c7e18; continue;
 }
 0x30c7e18 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x3155978u32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d172cu32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x32d172cu32), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x30c802f; continue; }
  pc = 0x30c7e38; continue;
 }
 0x30c7e38 => {
  { let v: u32 = cpu.r32(0x32d16e0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d174cu32, v); }
  { let v: u32 = cpu.r32(0x32d1728u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1704u32, v); }
  { let v: u32 = cpu.r32(0x32d16e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1708u32, v); }
  { let v: u32 = cpu.r32(0x32d172cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1750u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d16f8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1764u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d171cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1740u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d16fcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1720u32, v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3800000u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1768u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1744u32, v); }
  { cpu.sub32(cpu.r32(0x32d16e0u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c7efb; continue; }
  pc = 0x30c7eb1; continue;
 }
 0x30c7eb1 => {
  { cpu.fpush(0.0); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e0u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x32d1728u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e0u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = 0x0u32; cpu.w32(0x32d16e0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32d174cu32, v); }
  { let v = cpu.rf32(0x32d1740u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x32d16f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d16f8u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d1764u32, v); cpu.fpop(); }
  pc = 0x30c7efb; continue;
 }
 0x30c7efb => {
  { cpu.sub32(cpu.r32(0x32d16e4u32), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x30c7f4e; continue; }
  pc = 0x30c7f04; continue;
 }
 0x30c7f04 => {
  { cpu.fpush(0.0); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x32d172cu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e4u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = 0x0u32; cpu.w32(0x32d16e4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x32d1708u32, v); }
  { let v = cpu.rf32(0x32d1744u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16fcu32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x32d16fcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d16fcu32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d1720u32, v); cpu.fpop(); }
  pc = 0x30c7f4e; continue;
 }
 0x30c7f4e => {
  { let v: u32 = cpu.r32(0x3155984u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32d1704u32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c7f9f; continue; }
  pc = 0x30c7f5b; continue;
 }
 0x30c7f5b => {
  { let v = cpu.rf32(0x3155984u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e0u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x32d1728u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e0u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1704u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1728u32, v); }
  { let v = cpu.rf32(0x32d1740u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x32d16f8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d171cu32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d1740u32, v); cpu.fpop(); }
  pc = 0x30c7f9f; continue;
 }
 0x30c7f9f => {
  { let v: u32 = cpu.r32(0x3155988u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x32d172cu32), cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30c7ff0; continue; }
  pc = 0x30c7fac; continue;
 }
 0x30c7fac => {
  { let v = cpu.rf32(0x3155988u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e4u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x32d172cu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16e4u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d172cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1750u32, v); }
  { let v = cpu.rf32(0x32d1744u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x32d16fcu32); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x32d16fcu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d1768u32, v); }
  { let v = cpu.st(0); cpu.wf32(0x32d1744u32, v); cpu.fpop(); }
  pc = 0x30c7ff0; continue;
 }
 0x30c7ff0 => {
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x315bf10u32); cpu.set_st(0, v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x32d16f4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x32d16f4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1718u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d173cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d1760u32, v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = 0x315bf14u32; cpu.esi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30c802f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x30c8035; continue;
 }
 0x30c8035 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030d0000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30d0000); }
 let mut pc: u32 = 0x30d0000;
 loop { match pc {
 0x30d0000 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x200000u32); }
  if cpu.zf { pc = 0x30d001c; continue; }
  pc = 0x30d000a; continue;
 }
 0x30d000a => {
  { let v: u32 = cpu.eax & 0xffdfffffu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x200000u32); }
  if cpu.zf { pc = 0x30d002e; continue; }
  pc = 0x30d001a; continue;
 }
 0x30d001a => {
  pc = 0x30d003d; continue;
 }
 0x30d001c => {
  { let v: u32 = cpu.eax | 0x200000u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x200000u32); }
  if !cpu.zf { pc = 0x30d002e; continue; }
  pc = 0x30d002c; continue;
 }
 0x30d002c => {
  pc = 0x30d003d; continue;
 }
 0x30d002e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { cpu.cpuid(); }
  { cpu.and32(cpu.edx, 0x800000u32); }
  if !cpu.zf { pc = 0x30d0041; continue; }
  pc = 0x30d003d; continue;
 }
 0x30d003d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30d0041 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030e0000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30e0000); }
 let mut pc: u32 = 0x30e0000;
 loop { match pc {
 0x30e0000 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x30e0003, "Pxor");
  cpu.unimplemented(0x30e0006, "Pxor");
  cpu.unimplemented(0x30e0009, "Movd");
  cpu.unimplemented(0x30e000d, "Punpckldq");
  cpu.unimplemented(0x30e0011, "Movd");
  cpu.unimplemented(0x30e0015, "Pslld");
  cpu.unimplemented(0x30e0019, "Movq");
  cpu.unimplemented(0x30e001c, "Pand");
  cpu.unimplemented(0x30e0023, "Psrld");
  cpu.unimplemented(0x30e0027, "Movq");
  cpu.unimplemented(0x30e002a, "Punpckhwd");
  cpu.unimplemented(0x30e002d, "Punpckldq");
  cpu.unimplemented(0x30e0030, "Movq");
  cpu.unimplemented(0x30e0033, "Pxor");
  cpu.unimplemented(0x30e003a, "Pxor");
  cpu.unimplemented(0x30e0041, "Movq");
  cpu.unimplemented(0x30e0048, "Movq");
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x30e0058; continue;
 }
 0x30e0058 => {
  cpu.unimplemented(0x30e0058, "Movd");
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.unimplemented(0x30e005e, "Movq");
  cpu.unimplemented(0x30e0062, "Movq");
  cpu.unimplemented(0x30e0065, "Movq");
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.unimplemented(0x30e006d, "Movq");
  cpu.unimplemented(0x30e0072, "Punpcklbw");
  cpu.unimplemented(0x30e0076, "Punpcklbw");
  cpu.unimplemented(0x30e0079, "Por");
  cpu.unimplemented(0x30e007c, "Movq");
  cpu.unimplemented(0x30e007f, "Movq");
  cpu.unimplemented(0x30e0082, "Pand");
  cpu.unimplemented(0x30e0089, "Psrlq");
  cpu.unimplemented(0x30e008d, "Psrld");
  cpu.unimplemented(0x30e0091, "Por");
  cpu.unimplemented(0x30e0094, "Movd");
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30e01a1; continue; }
  pc = 0x30e00a0; continue;
 }
 0x30e00a0 => {
  cpu.unimplemented(0x30e00a0, "Punpcklwd");
  cpu.unimplemented(0x30e00a3, "Movq");
  cpu.unimplemented(0x30e00aa, "Punpckldq");
  cpu.unimplemented(0x30e00ad, "Pmulhw");
  cpu.unimplemented(0x30e00b0, "Movq");
  cpu.unimplemented(0x30e00b7, "Pxor");
  cpu.unimplemented(0x30e00be, "Pmulhw");
  cpu.unimplemented(0x30e00c1, "Movq");
  cpu.unimplemented(0x30e00c8, "Movq");
  cpu.unimplemented(0x30e00cf, "Punpckldq");
  cpu.unimplemented(0x30e00d2, "Pxor");
  cpu.unimplemented(0x30e00d5, "Movq");
  cpu.unimplemented(0x30e00d8, "Punpcklbw");
  cpu.unimplemented(0x30e00db, "Movq");
  cpu.unimplemented(0x30e00de, "Punpcklbw");
  cpu.unimplemented(0x30e00e1, "Punpckhwd");
  cpu.unimplemented(0x30e00e4, "Punpcklbw");
  cpu.unimplemented(0x30e00e7, "Pmaddwd");
  cpu.unimplemented(0x30e00ee, "Pmaddwd");
  cpu.unimplemented(0x30e00f1, "Pmaddwd");
  cpu.unimplemented(0x30e00f8, "Punpckldq");
  cpu.unimplemented(0x30e00fb, "Paddd");
  cpu.unimplemented(0x30e00fe, "Paddd");
  cpu.unimplemented(0x30e0101, "Psrlq");
  cpu.unimplemented(0x30e0105, "Movq");
  cpu.unimplemented(0x30e0108, "Pand");
  cpu.unimplemented(0x30e010f, "Psrlq");
  cpu.unimplemented(0x30e0113, "Pand");
  cpu.unimplemented(0x30e011a, "Por");
  cpu.unimplemented(0x30e011d, "Psrlq");
  cpu.unimplemented(0x30e0121, "Pand");
  cpu.unimplemented(0x30e0128, "Por");
  cpu.unimplemented(0x30e012b, "Movq");
  cpu.unimplemented(0x30e012e, "Movd");
  cpu.unimplemented(0x30e0131, "Punpckldq");
  cpu.unimplemented(0x30e0134, "Movq");
  cpu.unimplemented(0x30e0137, "Psllw");
  cpu.unimplemented(0x30e013b, "Movq");
  cpu.unimplemented(0x30e013e, "Pand");
  cpu.unimplemented(0x30e0145, "Psllw");
  cpu.unimplemented(0x30e0149, "Psrlw");
  cpu.unimplemented(0x30e014d, "Pand");
  cpu.unimplemented(0x30e0154, "Por");
  cpu.unimplemented(0x30e0157, "Pand");
  cpu.unimplemented(0x30e015e, "Movq");
  cpu.unimplemented(0x30e0161, "Punpcklwd");
  cpu.unimplemented(0x30e0164, "Paddusb");
  cpu.unimplemented(0x30e0167, "Movq");
  cpu.unimplemented(0x30e016a, "Psrld");
  cpu.unimplemented(0x30e016e, "Movq");
  cpu.unimplemented(0x30e0171, "Pand");
  cpu.unimplemented(0x30e0178, "Psrld");
  cpu.unimplemented(0x30e017c, "Pand");
  cpu.unimplemented(0x30e0183, "Psrld");
  cpu.unimplemented(0x30e0187, "Por");
  cpu.unimplemented(0x30e018a, "Pand");
  cpu.unimplemented(0x30e0191, "Por");
  cpu.unimplemented(0x30e0194, "Movq");
  cpu.unimplemented(0x30e0197, "Psrlq");
  cpu.unimplemented(0x30e019b, "Punpcklwd");
  cpu.unimplemented(0x30e019e, "Movd");
  pc = 0x30e01a1; continue;
 }
 0x30e01a1 => {
  cpu.unimplemented(0x30e01a1, "Paddd");
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30e0058; continue; }
  pc = 0x30e01b0; continue;
 }
 0x30e01b0 => {
  cpu.unimplemented(0x30e01b0, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030e01b4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30e01b4); } r_030e01b4(cpu, 0x30e01b4); }
pub fn f_030e0312(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30e0312); } r_030e01b4(cpu, 0x30e0312); }
fn r_030e01b4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30e01b4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x30e01b7, "Pxor");
  cpu.unimplemented(0x30e01ba, "Pxor");
  cpu.unimplemented(0x30e01bd, "Movd");
  cpu.unimplemented(0x30e01c1, "Punpckldq");
  cpu.unimplemented(0x30e01c5, "Movd");
  cpu.unimplemented(0x30e01c9, "Pslld");
  cpu.unimplemented(0x30e01cd, "Movq");
  cpu.unimplemented(0x30e01d0, "Pand");
  cpu.unimplemented(0x30e01d7, "Psrld");
  cpu.unimplemented(0x30e01db, "Movq");
  cpu.unimplemented(0x30e01de, "Punpckhwd");
  cpu.unimplemented(0x30e01e1, "Punpckldq");
  cpu.unimplemented(0x30e01e4, "Movq");
  cpu.unimplemented(0x30e01e7, "Pxor");
  cpu.unimplemented(0x30e01ee, "Pxor");
  cpu.unimplemented(0x30e01f5, "Movq");
  cpu.unimplemented(0x30e01fc, "Movq");
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x30e020c; continue;
 }
 0x30e020c => {
  cpu.unimplemented(0x30e020c, "Movd");
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.unimplemented(0x30e0212, "Movq");
  cpu.unimplemented(0x30e0216, "Movq");
  cpu.unimplemented(0x30e0219, "Movq");
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.unimplemented(0x30e0221, "Movq");
  cpu.unimplemented(0x30e0226, "Punpcklbw");
  cpu.unimplemented(0x30e022a, "Punpcklbw");
  cpu.unimplemented(0x30e022d, "Por");
  cpu.unimplemented(0x30e0230, "Movq");
  cpu.unimplemented(0x30e0233, "Movq");
  cpu.unimplemented(0x30e0236, "Pand");
  cpu.unimplemented(0x30e023d, "Psrlq");
  cpu.unimplemented(0x30e0241, "Psrld");
  cpu.unimplemented(0x30e0245, "Por");
  cpu.unimplemented(0x30e0248, "Movd");
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30e0355; continue; }
  pc = 0x30e0254; continue;
 }
 0x30e0254 => {
  cpu.unimplemented(0x30e0254, "Punpcklwd");
  cpu.unimplemented(0x30e0257, "Movq");
  cpu.unimplemented(0x30e025e, "Punpckldq");
  cpu.unimplemented(0x30e0261, "Pmulhw");
  cpu.unimplemented(0x30e0264, "Movq");
  cpu.unimplemented(0x30e026b, "Pxor");
  cpu.unimplemented(0x30e0272, "Pmulhw");
  cpu.unimplemented(0x30e0275, "Movq");
  cpu.unimplemented(0x30e027c, "Movq");
  cpu.unimplemented(0x30e0283, "Punpckldq");
  cpu.unimplemented(0x30e0286, "Pxor");
  cpu.unimplemented(0x30e0289, "Movq");
  cpu.unimplemented(0x30e028c, "Punpcklbw");
  cpu.unimplemented(0x30e028f, "Movq");
  cpu.unimplemented(0x30e0292, "Punpcklbw");
  cpu.unimplemented(0x30e0295, "Punpckhwd");
  cpu.unimplemented(0x30e0298, "Punpcklbw");
  cpu.unimplemented(0x30e029b, "Pmaddwd");
  cpu.unimplemented(0x30e02a2, "Pmaddwd");
  cpu.unimplemented(0x30e02a5, "Pmaddwd");
  cpu.unimplemented(0x30e02ac, "Punpckldq");
  cpu.unimplemented(0x30e02af, "Paddd");
  cpu.unimplemented(0x30e02b2, "Paddd");
  cpu.unimplemented(0x30e02b5, "Psrlq");
  cpu.unimplemented(0x30e02b9, "Movq");
  cpu.unimplemented(0x30e02bc, "Pand");
  cpu.unimplemented(0x30e02c3, "Psrlq");
  cpu.unimplemented(0x30e02c7, "Pand");
  cpu.unimplemented(0x30e02ce, "Por");
  cpu.unimplemented(0x30e02d1, "Psrlq");
  cpu.unimplemented(0x30e02d5, "Pand");
  cpu.unimplemented(0x30e02dc, "Por");
  cpu.unimplemented(0x30e02df, "Movq");
  cpu.unimplemented(0x30e02e2, "Movd");
  cpu.unimplemented(0x30e02e5, "Punpckldq");
  cpu.unimplemented(0x30e02e8, "Movq");
  cpu.unimplemented(0x30e02eb, "Psllw");
  cpu.unimplemented(0x30e02ef, "Movq");
  cpu.unimplemented(0x30e02f2, "Pand");
  cpu.unimplemented(0x30e02f9, "Psllw");
  cpu.unimplemented(0x30e02fd, "Psrlw");
  cpu.unimplemented(0x30e0301, "Pand");
  cpu.unimplemented(0x30e0308, "Por");
  cpu.unimplemented(0x30e030b, "Pand");
  pc = 0x30e0312; continue;
 }
 0x30e0312 => {
  cpu.unimplemented(0x30e0312, "Movq");
  cpu.unimplemented(0x30e0315, "Punpcklwd");
  cpu.unimplemented(0x30e0318, "Psubusb");
  cpu.unimplemented(0x30e031b, "Movq");
  cpu.unimplemented(0x30e031e, "Psrld");
  cpu.unimplemented(0x30e0322, "Movq");
  cpu.unimplemented(0x30e0325, "Pand");
  cpu.unimplemented(0x30e032c, "Psrld");
  cpu.unimplemented(0x30e0330, "Pand");
  cpu.unimplemented(0x30e0337, "Psrld");
  cpu.unimplemented(0x30e033b, "Por");
  cpu.unimplemented(0x30e033e, "Pand");
  cpu.unimplemented(0x30e0345, "Por");
  cpu.unimplemented(0x30e0348, "Movq");
  cpu.unimplemented(0x30e034b, "Psrlq");
  cpu.unimplemented(0x30e034f, "Punpcklwd");
  cpu.unimplemented(0x30e0352, "Movd");
  pc = 0x30e0355; continue;
 }
 0x30e0355 => {
  cpu.unimplemented(0x30e0355, "Paddd");
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30e020c; continue; }
  pc = 0x30e0364; continue;
 }
 0x30e0364 => {
  cpu.unimplemented(0x30e0364, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030e0368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30e0368); }
 let mut pc: u32 = 0x30e0368;
 loop { match pc {
 0x30e0368 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.unimplemented(0x30e036b, "Pxor");
  cpu.unimplemented(0x30e036e, "Pxor");
  cpu.unimplemented(0x30e0371, "Movd");
  cpu.unimplemented(0x30e0375, "Punpckldq");
  cpu.unimplemented(0x30e0379, "Movd");
  cpu.unimplemented(0x30e037d, "Pslld");
  cpu.unimplemented(0x30e0381, "Movq");
  cpu.unimplemented(0x30e0384, "Pand");
  cpu.unimplemented(0x30e038b, "Psrld");
  cpu.unimplemented(0x30e038f, "Movq");
  cpu.unimplemented(0x30e0392, "Punpckhwd");
  cpu.unimplemented(0x30e0395, "Punpckldq");
  cpu.unimplemented(0x30e0398, "Movq");
  cpu.unimplemented(0x30e039b, "Pxor");
  cpu.unimplemented(0x30e03a2, "Pxor");
  cpu.unimplemented(0x30e03a9, "Movq");
  cpu.unimplemented(0x30e03b0, "Movq");
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  pc = 0x30e03c0; continue;
 }
 0x30e03c0 => {
  cpu.unimplemented(0x30e03c0, "Movd");
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.unimplemented(0x30e03c6, "Movq");
  cpu.unimplemented(0x30e03ca, "Movq");
  cpu.unimplemented(0x30e03cd, "Movq");
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x20u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.unimplemented(0x30e03d5, "Movq");
  cpu.unimplemented(0x30e03da, "Punpcklbw");
  cpu.unimplemented(0x30e03de, "Punpcklbw");
  cpu.unimplemented(0x30e03e1, "Por");
  cpu.unimplemented(0x30e03e4, "Movq");
  cpu.unimplemented(0x30e03e7, "Movq");
  cpu.unimplemented(0x30e03ea, "Pand");
  cpu.unimplemented(0x30e03f1, "Psrlq");
  cpu.unimplemented(0x30e03f5, "Psrld");
  cpu.unimplemented(0x30e03f9, "Por");
  cpu.unimplemented(0x30e03fc, "Movd");
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x30e04d3; continue; }
  pc = 0x30e0408; continue;
 }
 0x30e0408 => {
  cpu.unimplemented(0x30e0408, "Punpcklwd");
  cpu.unimplemented(0x30e040b, "Movq");
  cpu.unimplemented(0x30e0412, "Punpckldq");
  cpu.unimplemented(0x30e0415, "Pmulhw");
  cpu.unimplemented(0x30e0418, "Movq");
  cpu.unimplemented(0x30e041f, "Pxor");
  cpu.unimplemented(0x30e0426, "Pmulhw");
  cpu.unimplemented(0x30e0429, "Movq");
  cpu.unimplemented(0x30e0430, "Movq");
  cpu.unimplemented(0x30e0437, "Punpckldq");
  cpu.unimplemented(0x30e043a, "Pxor");
  cpu.unimplemented(0x30e043d, "Movq");
  cpu.unimplemented(0x30e0440, "Punpcklbw");
  cpu.unimplemented(0x30e0443, "Movq");
  cpu.unimplemented(0x30e0446, "Punpcklbw");
  cpu.unimplemented(0x30e0449, "Punpckhwd");
  cpu.unimplemented(0x30e044c, "Punpcklbw");
  cpu.unimplemented(0x30e044f, "Pmaddwd");
  cpu.unimplemented(0x30e0456, "Pmaddwd");
  cpu.unimplemented(0x30e0459, "Pmaddwd");
  cpu.unimplemented(0x30e0460, "Punpckldq");
  cpu.unimplemented(0x30e0463, "Paddd");
  cpu.unimplemented(0x30e0466, "Paddd");
  cpu.unimplemented(0x30e0469, "Psrlq");
  cpu.unimplemented(0x30e046d, "Movq");
  cpu.unimplemented(0x30e0470, "Pand");
  cpu.unimplemented(0x30e0477, "Psrlq");
  cpu.unimplemented(0x30e047b, "Pand");
  cpu.unimplemented(0x30e0482, "Por");
  cpu.unimplemented(0x30e0485, "Psrlq");
  cpu.unimplemented(0x30e0489, "Pand");
  cpu.unimplemented(0x30e0490, "Por");
  cpu.unimplemented(0x30e0493, "Movq");
  cpu.unimplemented(0x30e0496, "Punpckldq");
  cpu.unimplemented(0x30e0499, "Movq");
  cpu.unimplemented(0x30e049c, "Psrld");
  cpu.unimplemented(0x30e04a0, "Movq");
  cpu.unimplemented(0x30e04a3, "Pand");
  cpu.unimplemented(0x30e04aa, "Psrld");
  cpu.unimplemented(0x30e04ae, "Pand");
  cpu.unimplemented(0x30e04b5, "Psrld");
  cpu.unimplemented(0x30e04b9, "Por");
  cpu.unimplemented(0x30e04bc, "Pand");
  cpu.unimplemented(0x30e04c3, "Por");
  cpu.unimplemented(0x30e04c6, "Movq");
  cpu.unimplemented(0x30e04c9, "Psrlq");
  cpu.unimplemented(0x30e04cd, "Punpcklwd");
  cpu.unimplemented(0x30e04d0, "Movd");
  pc = 0x30e04d3; continue;
 }
 0x30e04d3 => {
  cpu.unimplemented(0x30e04d3, "Paddd");
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x30e03c0; continue; }
  pc = 0x30e04e2; continue;
 }
 0x30e04e2 => {
  cpu.unimplemented(0x30e04e2, "Emms");
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
