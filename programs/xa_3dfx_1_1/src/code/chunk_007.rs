#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_020c409a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c409a); } r_020c409a(cpu, 0x20c409a); }
pub fn f_020c40c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c40c7); } r_020c409a(cpu, 0x20c40c7); }
fn r_020c409a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c409a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x217cb58u32)); }
  if !cpu.cf { pc = 0x20c40ea; continue; }
  pc = 0x20c40a2; continue;
 }
 0x20c40a2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x217cb64u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x217cb68u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c409a; continue; }
  pc = 0x20c40ea; continue;
 }
 0x20c40c7 => {
  cpu.push(0x20c40cc); cpu.call(0xc336efdb); if cpu.returned_to != 0x20c40cc { return; }
  { cpu.df = true; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x217cb60u32)); }
  if cpu.cf { cpu.poll(); pc = 0x20c409a; continue; }
  pc = 0x20c40ea; continue;
 }
 0x20c40ea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4370); }
 let mut pc: u32 = 0x20c4370;
 loop { match pc {
 0x20c4370 => {
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
  if cpu.zf { pc = 0x20c438f; continue; }
  pc = 0x20c4389; continue;
 }
 0x20c4389 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x20c4392; continue; }
  pc = 0x20c438f; continue;
 }
 0x20c438f => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x20c4392; continue;
 }
 0x20c4392 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x20c439e; continue; }
  pc = 0x20c4398; continue;
 }
 0x20c4398 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x20c43ad; continue; }
  pc = 0x20c439e; continue;
 }
 0x20c439e => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x20c43ad; continue; }
  pc = 0x20c43aa; continue;
 }
 0x20c43aa => {
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  pc = 0x20c43ad; continue;
 }
 0x20c43ad => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x20c43b9; continue; }
  pc = 0x20c43b3; continue;
 }
 0x20c43b3 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x20c43bc; continue; }
  pc = 0x20c43b9; continue;
 }
 0x20c43b9 => {
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  pc = 0x20c43bc; continue;
 }
 0x20c43bc => {
  { cpu.and32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x20c43ca; continue; }
  pc = 0x20c43c3; continue;
 }
 0x20c43c3 => {
  { cpu.and32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20c43dc; continue; }
  pc = 0x20c43ca; continue;
 }
 0x20c43ca => {
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x20c43dc; continue; }
  pc = 0x20c43d0; continue;
 }
 0x20c43d0 => {
  { cpu.sub32(cpu.r32(0x6cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x20c43dc; continue; }
  pc = 0x20c43d6; continue;
 }
 0x20c43d6 => {
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x40u32); }
  if cpu.zf { pc = 0x20c43e6; continue; }
  pc = 0x20c43dc; continue;
 }
 0x20c43dc => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x20c43f2; continue; }
  pc = 0x20c43e6; continue;
 }
 0x20c43e6 => {
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x20c43f2; continue; }
  pc = 0x20c43ef; continue;
 }
 0x20c43ef => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x20c43f2; continue;
 }
 0x20c43f2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x217cb50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c4409; continue; }
  pc = 0x20c4404; continue;
 }
 0x20c4404 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x20c4409; continue;
 }
 0x20c4409 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x10000u32); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c441d; continue; }
  pc = 0x20c441b; continue;
 }
 0x20c441b => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x20c441d; continue;
 }
 0x20c441d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x22d1fd8u32.wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x20c4467; continue; }
  pc = 0x20c4425; continue;
 }
 0x20c4425 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cb64u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.r32(0x217cb50u32.wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x20c4445; continue; }
  pc = 0x20c4441; continue;
 }
 0x20c4441 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x20c4445; continue;
 }
 0x20c4445 => {
  { cpu.and32(cpu.r32(0x217cb50u32.wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x20c4455; continue; }
  pc = 0x20c4451; continue;
 }
 0x20c4451 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x20c4455; continue;
 }
 0x20c4455 => {
  { let v: u32 = cpu.eax; cpu.w32(0x217cb68u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cb6cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x217cb50u32.wrapping_add(cpu.bases[3])) | 0x20u32; cpu.w32(0x217cb50u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c4467; continue;
 }
 0x20c4467 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cb5cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cb60u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cb58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217cb50u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  let t = cpu.r32(0x20c2744u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.flat_ds(0x20c44ad); cpu.push(0x20c44b5); cpu.call(t); if cpu.returned_to != 0x20c44b5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x217cb5cu32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x20c44d2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c44d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c44d3); }
 let mut pc: u32 = 0x20c44d3;
 loop { match pc {
 0x20c44d3 => {
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
  if cpu.zf { pc = 0x20c44f0; continue; }
  pc = 0x20c44ea; continue;
 }
 0x20c44ea => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x20c44f3; continue; }
  pc = 0x20c44f0; continue;
 }
 0x20c44f0 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x20c44f3; continue;
 }
 0x20c44f3 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x20c44ff; continue; }
  pc = 0x20c44f9; continue;
 }
 0x20c44f9 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x20c4502; continue; }
  pc = 0x20c44ff; continue;
 }
 0x20c44ff => {
  { let v: u32 = cpu.eax | 0x20u32; cpu.eax = v; }
  pc = 0x20c4502; continue;
 }
 0x20c4502 => {
  { cpu.sub32(cpu.r32(0x68u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20c450b; continue; }
  pc = 0x20c4508; continue;
 }
 0x20c4508 => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  pc = 0x20c450b; continue;
 }
 0x20c450b => {
  { let v: u32 = cpu.eax; cpu.w32(0x217cb50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x20u32); }
  if !cpu.zf { pc = 0x20c4539; continue; }
  pc = 0x20c451f; continue;
 }
 0x20c451f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x20c4552; continue; }
  pc = 0x20c452f; continue;
 }
 0x20c452f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x20c4552; continue;
 }
 0x20c4539 => {
  { let v: u32 = cpu.r32(0x217cb50u32.wrapping_add(cpu.bases[3])) | 0x8u32; cpu.w32(0x217cb50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  pc = 0x20c4552; continue;
 }
 0x20c4552 => {
  { let v: u32 = cpu.r32(0x217cb50u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  let t = cpu.r32(0x20c2544u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.flat_ds(0x20c4557); cpu.push(0x20c455f); cpu.call(t); if cpu.returned_to != 0x20c455f { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x20c4565); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4566(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4566); }
 let mut pc: u32 = 0x20c4566;
 loop { match pc {
 0x20c4566 => {
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
  cpu.flat_ds(0x20c4591); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4592(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4592); }
 let mut pc: u32 = 0x20c4592;
 loop { match pc {
 0x20c4592 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0x217cb70u32; cpu.push(v); }
  { let v = 0x217cb50u32; cpu.push(v); }
  cpu.flat_ds(0x20c45a1); cpu.push(0x20c45a6); f_020b35a0(cpu); if cpu.returned_to != 0x20c45a6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x20c4592u32; cpu.push(v); }
  { let v = 0x20c2544u32; cpu.push(v); }
  cpu.flat_ds(0x20c45b3); cpu.push(0x20c45b8); f_020b35a0(cpu); if cpu.returned_to != 0x20c45b8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c45c0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c45c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c45c1); }
 let mut pc: u32 = 0x20c45c1;
 loop { match pc {
 0x20c45c1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x8000u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc44u32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c45e2; continue; }
  pc = 0x20c45dd; continue;
 }
 0x20c45dd => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c45e9; continue; }
  pc = 0x20c45e2; continue;
 }
 0x20c45e2 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c45eb; continue;
 }
 0x20c45e9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c45eb; continue;
 }
 0x20c45eb => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c4624; continue; }
  pc = 0x20c45ef; continue;
 }
 0x20c45ef => {
  cpu.push(0x20c45f4); f_020c687f(cpu); if cpu.returned_to != 0x20c45f4 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c4624; continue; }
  pc = 0x20c45f9; continue;
 }
 0x20c45f9 => {
  { let v: u32 = 0x20ae04bu32; cpu.edx = v; }
  { let v: u32 = 0x20ae040u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x20ae040u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20c4624; continue;
 }
 0x20c4624 => {
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
pub fn f_020c462a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c462a); }
 let mut pc: u32 = 0x20c462a;
 loop { match pc {
 0x20c462a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c462b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c462b); }
 let mut pc: u32 = 0x20c462b;
 loop { match pc {
 0x20c462b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x217cb70u32); cpu.push(0x20c4634); cpu.call(t); if cpu.returned_to != 0x20c4634 { return; }
  let t = cpu.r32(0x217cb74u32); cpu.push(0x20c463a); cpu.call(t); if cpu.returned_to != 0x20c463a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c4641); f_020c4643(cpu); if cpu.returned_to != 0x20c4641 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c465a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c465a); }
 let mut pc: u32 = 0x20c465a;
 loop { match pc {
 0x20c465a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x20c46a9; continue; }
  pc = 0x20c4664; continue;
 }
 0x20c4664 => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x20c46a9; continue; }
  pc = 0x20c4669; continue;
 }
 0x20c4669 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c46a9; continue; }
  pc = 0x20c467e; continue;
 }
 0x20c467e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c468d; continue; }
  pc = 0x20c4688; continue;
 }
 0x20c4688 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x20c468d; continue;
 }
 0x20c468d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x20c46ad; continue; }
  pc = 0x20c4693; continue;
 }
 0x20c4693 => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c468d; continue; }
  pc = 0x20c46a6; continue;
 }
 0x20c46a6 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c46a9; continue;
 }
 0x20c46a9 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c46f8; continue;
 }
 0x20c46ad => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x20c46db; continue; }
  pc = 0x20c46bd; continue;
 }
 0x20c46bd => {
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
  pc = 0x20c46f0; continue;
 }
 0x20c46db => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20c46f0; continue;
 }
 0x20c46f0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x20c46f8; continue;
 }
 0x20c46f8 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x20c46fc); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4702(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4702); }
 let mut pc: u32 = 0x20c4702;
 loop { match pc {
 0x20c46fd => {
  pc = 0x20c4808; continue;
 }
 0x20c4702 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x20c46fd; continue; }
  pc = 0x20c470c; continue;
 }
 0x20c470c => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x20c46fd; continue; }
  pc = 0x20c4717; continue;
 }
 0x20c4717 => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x20c4744; continue; }
  pc = 0x20c4725; continue;
 }
 0x20c4725 => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x20c472d; continue; }
  pc = 0x20c472a; continue;
 }
 0x20c472a => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c472d; continue;
 }
 0x20c472d => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c47c0; continue;
 }
 0x20c4744 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x20c475b; continue; }
  pc = 0x20c474d; continue;
 }
 0x20c474d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c47c0; continue; }
  pc = 0x20c4752; continue;
 }
 0x20c4752 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x20c47c0; continue; }
  pc = 0x20c4759; continue;
 }
 0x20c4759 => {
  pc = 0x20c476a; continue;
 }
 0x20c475b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x20c47c0; continue; }
  pc = 0x20c4762; continue;
 }
 0x20c4762 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c47c0; continue; }
  pc = 0x20c476a; continue;
 }
 0x20c476a => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x20c479f; continue; }
  pc = 0x20c477a; continue;
 }
 0x20c477a => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c4787; continue; }
  pc = 0x20c4785; continue;
 }
 0x20c4785 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x20c4787; continue;
 }
 0x20c4787 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x20c478b; continue;
 }
 0x20c478b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c47c0; continue; }
  pc = 0x20c4792; continue;
 }
 0x20c4792 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20c479f; continue; }
  pc = 0x20c4797; continue;
 }
 0x20c4797 => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c478b; continue; }
  pc = 0x20c479f; continue;
 }
 0x20c479f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x20c47a9; continue; }
  pc = 0x20c47a6; continue;
 }
 0x20c47a6 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x20c47a9; continue;
 }
 0x20c47a9 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x20c47c0; continue; }
  pc = 0x20c47ad; continue;
 }
 0x20c47ad => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x20c47c0; continue; }
  pc = 0x20c47b4; continue;
 }
 0x20c47b4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x20c47c0; continue; }
  pc = 0x20c47bb; continue;
 }
 0x20c47bb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x20c47a9; continue;
 }
 0x20c47c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x20c47df; continue; }
  pc = 0x20c47cf; continue;
 }
 0x20c47cf => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x20c47db; continue; }
  pc = 0x20c47d8; continue;
 }
 0x20c47d8 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c47db; continue;
 }
 0x20c47db => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x20c47f0; continue;
 }
 0x20c47df => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c47f0; continue;
 }
 0x20c47f0 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x20c4800; continue; }
  pc = 0x20c47f8; continue;
 }
 0x20c47f8 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x20c4800; continue; }
  pc = 0x20c47fd; continue;
 }
 0x20c47fd => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c4800; continue;
 }
 0x20c4800 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x20c4808; continue; }
  pc = 0x20c4805; continue;
 }
 0x20c4805 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c4808; continue;
 }
 0x20c4808 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x20c480c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c480d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c480d); }
 let mut pc: u32 = 0x20c480d;
 loop { match pc {
 0x20c480d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c854u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x20c4823; continue; }
  pc = 0x20c481a; continue;
 }
 0x20c481a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217c854u32, v); }
  pc = 0x20c4823; continue;
 }
 0x20c4823 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x22d22b0u32)); }
  if !cpu.zf { pc = 0x20c4833; continue; }
  pc = 0x20c482b; continue;
 }
 0x20c482b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x22d22b0u32, v); }
  pc = 0x20c4833; continue;
 }
 0x20c4833 => {
  { let v: u32 = cpu.r32(0x217c850u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x20c4846; continue; }
  pc = 0x20c483d; continue;
 }
 0x20c483d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217c850u32, v); }
  pc = 0x20c4846; continue;
 }
 0x20c4846 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c4853; continue; }
  pc = 0x20c4850; continue;
 }
 0x20c4850 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c4853; continue;
 }
 0x20c4853 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c485a; continue; }
  pc = 0x20c4857; continue;
 }
 0x20c4857 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c485a; continue;
 }
 0x20c485a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c485e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c485e); } r_020c485e(cpu, 0x20c485e); }
pub fn f_020c4a7d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c4a7d); } r_020c485e(cpu, 0x20c4a7d); }
pub fn f_020c4b4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c4b4e); } r_020c485e(cpu, 0x20c4b4e); }
fn r_020c485e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c485e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c850u32); cpu.edx = v; }
  pc = 0x20c4869; continue;
 }
 0x20c4869 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c4b48; continue; }
  pc = 0x20c4871; continue;
 }
 0x20c4871 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x20c48b5; continue; }
  pc = 0x20c487f; continue;
 }
 0x20c487f => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x20c488c); f_020c480d(cpu); if cpu.returned_to != 0x20c488c { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c48a5; continue; }
  pc = 0x20c4895; continue;
 }
 0x20c4895 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x20c4869; continue;
 }
 0x20c48a5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x20c4869; continue;
 }
 0x20c48b5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x20c4869; continue;
 }
 0x20c4a7d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x20c4a91); f_020c485e(cpu); if cpu.returned_to != 0x20c4a91 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x20c4aac; continue; }
  pc = 0x20c4a9c; continue;
 }
 0x20c4a9c => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c4aac; continue;
 }
 0x20c4aac => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c4ac9; continue; }
  pc = 0x20c4ab0; continue;
 }
 0x20c4ab0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c4b45; continue;
 }
 0x20c4ac9 => {
  { cpu.and16(cpu.r16(0x22d2886u32), 0xfff0u16); }
  if cpu.zf { pc = 0x20c4ade; continue; }
  pc = 0x20c4ad4; continue;
 }
 0x20c4ad4 => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x22d2884u32, v); }
  pc = 0x20c4ade; continue;
 }
 0x20c4ade => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c4b43; continue; }
  pc = 0x20c4ae7; continue;
 }
 0x20c4ae7 => {
  { let v: u32 = cpu.r32(0x22d2884u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x20c4b43; continue; }
  pc = 0x20c4b01; continue;
 }
 0x20c4b01 => {
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
  if (cpu.sf != cpu.of) { pc = 0x20c4b43; continue; }
  pc = 0x20c4b25; continue;
 }
 0x20c4b25 => {
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
  pc = 0x20c4b45; continue;
 }
 0x20c4b43 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c4b45; continue;
 }
 0x20c4b45 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20c4b48; continue;
 }
 0x20c4b48 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c4b4e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217cc64u32), 0x0u32); }
  if !cpu.zf { pc = 0x20c4b61; continue; }
  pc = 0x20c4b5d; continue;
 }
 0x20c4b5d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x20c4b45; continue;
 }
 0x20c4b61 => {
  { cpu.sub32(cpu.r32(0x217b910u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x20c4b5d; continue; }
  pc = 0x20c4b6a; continue;
 }
 0x20c4b6a => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x20c4b71); f_020c4d4a(cpu); if cpu.returned_to != 0x20c4b71 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x20c4b45; continue; }
  pc = 0x20c4b75; continue;
 }
 0x20c4b75 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c4b96; continue; }
  pc = 0x20c4b81; continue;
 }
 0x20c4b81 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20c4b89); f_020c4a7d(cpu); if cpu.returned_to != 0x20c4b89 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c4bb1; continue; }
  pc = 0x20c4b8f; continue;
 }
 0x20c4b8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c4bb1; continue;
 }
 0x20c4b96 => {
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
  pc = 0x20c4bb1; continue;
 }
 0x20c4bb1 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x20c4b5d; continue; }
  pc = 0x20c4bb5; continue;
 }
 0x20c4bb5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x20c4b5d; continue; }
  pc = 0x20c4bc2; continue;
 }
 0x20c4bc2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x20c4b5d; continue; }
  pc = 0x20c4bca; continue;
 }
 0x20c4bca => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c4bd3); f_020c49b9(cpu); if cpu.returned_to != 0x20c4bd3 { return; }
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
  cpu.push(0x20c4bf4); f_020bd557(cpu); if cpu.returned_to != 0x20c4bf4 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c4b45; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c48ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c48ba); }
 let mut pc: u32 = 0x20c48ba;
 loop { match pc {
 0x20c48ba => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x20c48c8); f_020c485e(cpu); if cpu.returned_to != 0x20c48c8 { return; }
  { let v: u32 = cpu.r32(0x217c850u32); cpu.edx = v; }
  pc = 0x20c49a9; continue;
 }
 0x20c48d3 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c49b1; continue; }
  pc = 0x20c48db; continue;
 }
 0x20c48db => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x20c49a6; continue; }
  pc = 0x20c48e6; continue;
 }
 0x20c48e6 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x20c49a6; continue; }
  pc = 0x20c48f0; continue;
 }
 0x20c48f0 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c48f7); f_020c480d(cpu); if cpu.returned_to != 0x20c48f7 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20c49b1; continue; }
  pc = 0x20c4904; continue;
 }
 0x20c4904 => {
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
  if cpu.zf { pc = 0x20c4948; continue; }
  pc = 0x20c4938; continue;
 }
 0x20c4938 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c4948; continue;
 }
 0x20c4948 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c49b3; continue; }
  pc = 0x20c494c; continue;
 }
 0x20c494c => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20c4963); f_020c49b9(cpu); if cpu.returned_to != 0x20c4963 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x20c499f; continue; }
  pc = 0x20c4978; continue;
 }
 0x20c4978 => {
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
  cpu.push(0x20c499d); f_020bd557(cpu); if cpu.returned_to != 0x20c499d { return; }
  pc = 0x20c49a2; continue;
 }
 0x20c499f => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c49a2; continue;
 }
 0x20c49a2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20c49b3; continue;
 }
 0x20c49a6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x20c49a9; continue;
 }
 0x20c49a9 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c48d3; continue; }
  pc = 0x20c49b1; continue;
 }
 0x20c49b1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c49b3; continue;
 }
 0x20c49b3 => {
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
pub fn f_020c49b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c49b9); }
 let mut pc: u32 = 0x20c49b9;
 loop { match pc {
 0x20c49b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217c850u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20c49cf; continue;
 }
 0x20c49c6 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x20c49d3; continue; }
  pc = 0x20c49ca; continue;
 }
 0x20c49ca => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x20c49cf; continue;
 }
 0x20c49cf => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20c49c6; continue; }
  pc = 0x20c49d3; continue;
 }
 0x20c49d3 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c49e2; continue; }
  pc = 0x20c49dd; continue;
 }
 0x20c49dd => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c49e8; continue;
 }
 0x20c49e2 => {
  { let v: u32 = cpu.edx; cpu.w32(0x217c850u32, v); }
  pc = 0x20c49e8; continue;
 }
 0x20c49e8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c49ef; continue; }
  pc = 0x20c49ec; continue;
 }
 0x20c49ec => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c49ef; continue;
 }
 0x20c49ef => {
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
pub fn f_020c4a2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4a2d); }
 let mut pc: u32 = 0x20c4a2d;
 loop { match pc {
 0x20c4a2d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217c850u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c4a78; continue; }
  pc = 0x20c4a39; continue;
 }
 0x20c4a39 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c4a6b; continue; }
  pc = 0x20c4a4f; continue;
 }
 0x20c4a4f => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c4a6b; continue; }
  pc = 0x20c4a54; continue;
 }
 0x20c4a54 => {
  { cpu.sub8(cpu.r8(0x217b940u32), 0x0u8); }
  if !cpu.zf { pc = 0x20c4a6b; continue; }
  pc = 0x20c4a5d; continue;
 }
 0x20c4a5d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x20c4a62, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b910u32, v); }
  pc = 0x20c4a6b; continue;
 }
 0x20c4a6b => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x217b910u32)); }
  if !cpu.zf { pc = 0x20c4a78; continue; }
  pc = 0x20c4a73; continue;
 }
 0x20c4a73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c4a78 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4bfe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4bfe); }
 let mut pc: u32 = 0x20c4bfe;
 loop { match pc {
 0x20c4bfe => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c4c19; continue; }
  pc = 0x20c4c10; continue;
 }
 0x20c4c10 => {
  { cpu.sub8(cpu.r8(0x217b93fu32), 0x0u8); }
  if cpu.zf { pc = 0x20c4c25; continue; }
  pc = 0x20c4c19; continue;
 }
 0x20c4c19 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x20c4c32; continue; }
  pc = 0x20c4c25; continue;
 }
 0x20c4c25 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20c4c2d); f_020c4b4e(cpu); if cpu.returned_to != 0x20c4c2d { return; }
  pc = 0x20c4d41; continue;
 }
 0x20c4c32 => {
  { cpu.sub32(cpu.r32(0x217cc64u32), 0x0u32); }
  if !cpu.zf { pc = 0x20c4c42; continue; }
  pc = 0x20c4c3b; continue;
 }
 0x20c4c3b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c4d41; continue;
 }
 0x20c4c42 => {
  { cpu.sub32(cpu.r32(0x217b910u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x20c4c3b; continue; }
  pc = 0x20c4c4b; continue;
 }
 0x20c4c4b => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x20c4c52); f_020c4d4a(cpu); if cpu.returned_to != 0x20c4c52 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c4d41; continue; }
  pc = 0x20c4c5a; continue;
 }
 0x20c4c5a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c4c82; continue; }
  pc = 0x20c4c66; continue;
 }
 0x20c4c66 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c4c82; continue; }
  pc = 0x20c4c6b; continue;
 }
 0x20c4c6b => {
  { cpu.sub8(cpu.r8(0x217b940u32), 0x0u8); }
  if !cpu.zf { pc = 0x20c4c82; continue; }
  pc = 0x20c4c74; continue;
 }
 0x20c4c74 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x20c4c79, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b910u32, v); }
  pc = 0x20c4c82; continue;
 }
 0x20c4c82 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217b910u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x20c4c96; continue; }
  pc = 0x20c4c91; continue;
 }
 0x20c4c91 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x20c4c96; continue;
 }
 0x20c4c96 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c4c9d); f_020c6e0f(cpu); if cpu.returned_to != 0x20c4c9d { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x20c4c3b; continue; }
  pc = 0x20c4ca2; continue;
 }
 0x20c4ca2 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x20c4c3b; continue; }
  pc = 0x20c4ca9; continue;
 }
 0x20c4ca9 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x20c4c3b; continue; }
  pc = 0x20c4cad; continue;
 }
 0x20c4cad => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x20c4c3b; continue; }
  pc = 0x20c4cb9; continue;
 }
 0x20c4cb9 => {
  { let v: u32 = cpu.r32(0x217c850u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c4cd6; continue;
 }
 0x20c4cc4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20c4cda; continue; }
  pc = 0x20c4ccb; continue;
 }
 0x20c4ccb => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c4cd3; continue; }
  pc = 0x20c4ccf; continue;
 }
 0x20c4ccf => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x20c4cda; continue; }
  pc = 0x20c4cd3; continue;
 }
 0x20c4cd3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x20c4cd6; continue;
 }
 0x20c4cd6 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c4cc4; continue; }
  pc = 0x20c4cda; continue;
 }
 0x20c4cda => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c4d04; continue; }
  pc = 0x20c4cde; continue;
 }
 0x20c4cde => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x20c4d04; continue; }
  pc = 0x20c4ce9; continue;
 }
 0x20c4ce9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c4d23; continue;
 }
 0x20c4d04 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x20c4c3b; continue; }
  pc = 0x20c4d0e; continue;
 }
 0x20c4d0e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20c4d1c); f_020c49b9(cpu); if cpu.returned_to != 0x20c4d1c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c4d23; continue;
 }
 0x20c4d23 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x20c4d3c); f_020bd557(cpu); if cpu.returned_to != 0x20c4d3c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c4d41; continue;
 }
 0x20c4d41 => {
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
pub fn f_020c4d4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4d4a); }
 let mut pc: u32 = 0x20c4d4a;
 loop { match pc {
 0x20c4d4a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20c4d61; continue; }
  pc = 0x20c4d5b; continue;
 }
 0x20c4d5b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c4d61 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c4d76; continue; }
  pc = 0x20c4d6d; continue;
 }
 0x20c4d6d => {
  { cpu.sub8(cpu.r8(0x217b93fu32), 0x0u8); }
  if cpu.zf { pc = 0x20c4d82; continue; }
  pc = 0x20c4d76; continue;
 }
 0x20c4d76 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x20c4d87; continue; }
  pc = 0x20c4d82; continue;
 }
 0x20c4d82 => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x20c4d8e; continue;
 }
 0x20c4d87 => {
  cpu.push(0x20c4d8c); f_020c4a2d(cpu); if cpu.returned_to != 0x20c4d8c { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x20c4d8e; continue;
 }
 0x20c4d8e => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x20c4d5b; continue; }
  pc = 0x20c4d97; continue;
 }
 0x20c4d97 => {
  { let v: u32 = cpu.r32(0x217cc44u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x20c4da6; continue; }
  pc = 0x20c4da1; continue;
 }
 0x20c4da1 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c4da6; continue;
 }
 0x20c4da6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x20c4d5b; continue; }
  pc = 0x20c4db2; continue;
 }
 0x20c4db2 => {
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
pub fn f_020c4dc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4dc7); }
 let mut pc: u32 = 0x20c4dc7;
 loop { match pc {
 0x20c4dc7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4dca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4dca); }
 let mut pc: u32 = 0x20c4dca;
 loop { match pc {
 0x20c4dca => {
  { cpu.sub8(cpu.r8(0x217cb84u32), 0x0u8); }
  if cpu.zf { pc = 0x20c4ded; continue; }
  pc = 0x20c4dd3; continue;
 }
 0x20c4dd3 => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x20c4ddb, "Int3");
  pc = 0x20c4de4; continue;
 }
 0x20c4de4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c4ded => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4df0); }
 let mut pc: u32 = 0x20c4df0;
 loop { match pc {
 0x20c4df0 => {
  { let v = cpu.ebx; cpu.push(v); }
  pc = 0x20c4df1; continue;
 }
 0x20c4df1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x20c4dfd; continue; }
  pc = 0x20c4df9; continue;
 }
 0x20c4df9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x20c4df1; continue;
 }
 0x20c4dfd => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4dff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4dff); }
 let mut pc: u32 = 0x20c4dff;
 loop { match pc {
 0x20c4dff => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(8); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20c4e16; continue; }
  pc = 0x20c4e10; continue;
 }
 0x20c4e10 => {
  { let v: u32 = cpu.r32(0x217cb7cu32); cpu.edi = v; }
  pc = 0x20c4e16; continue;
 }
 0x20c4e16 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c4e32; continue; }
  pc = 0x20c4e1c; continue;
 }
 0x20c4e1c => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20c4e2d; continue;
 }
 0x20c4e20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x20c4e27); f_020a4a64(cpu); if cpu.returned_to != 0x20c4e27 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x20c4e2d; continue;
 }
 0x20c4e2d => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c4e20; continue; }
  pc = 0x20c4e32; continue;
 }
 0x20c4e32 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x20c4e45; continue; }
  pc = 0x20c4e39; continue;
 }
 0x20c4e39 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x20c4e40); f_020a4a64(cpu); if cpu.returned_to != 0x20c4e40 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x20c4e45; continue;
 }
 0x20c4e45 => {
  { let v: u32 = cpu.r32(0x217cc44u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x10u32; cpu.w32(0x217cc44u32, v); }
  cpu.push(0x20c4e61); f_020bd474(cpu); if cpu.returned_to != 0x20c4e61 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c4e96; continue; }
  pc = 0x20c4e65; continue;
 }
 0x20c4e65 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c4e6c); f_020bd474(cpu); if cpu.returned_to != 0x20c4e6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c4e96; continue; }
  pc = 0x20c4e70; continue;
 }
 0x20c4e70 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x20c4e7a); f_020af0db(cpu); if cpu.returned_to != 0x20c4e7a { return; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x20c4e84); f_020af0fe(cpu); if cpu.returned_to != 0x20c4e84 { return; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cc44u32, v); }
  pc = 0x20c4e8c; continue;
 }
 0x20c4e8c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20c4f31; continue;
 }
 0x20c4e96 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc44u32, v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c4ec7; continue; }
  pc = 0x20c4eb3; continue;
 }
 0x20c4eb3 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x20c4ec2; continue;
 }
 0x20c4eb7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.push(0x20c4ebe); f_020c4df0(cpu); if cpu.returned_to != 0x20c4ebe { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  pc = 0x20c4ec2; continue;
 }
 0x20c4ec2 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c4eb7; continue; }
  pc = 0x20c4ec7; continue;
 }
 0x20c4ec7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if cpu.zf { pc = 0x20c4eda; continue; }
  pc = 0x20c4ed1; continue;
 }
 0x20c4ed1 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.push(0x20c4eda); f_020a3df1(cpu); if cpu.returned_to != 0x20c4eda { return; }
  pc = 0x20c4eda; continue;
 }
 0x20c4eda => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20c4efb; continue; }
  pc = 0x20c4ee1; continue;
 }
 0x20c4ee1 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x20c4ef5; continue;
 }
 0x20c4ee6 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c4eeb; continue; }
  pc = 0x20c4eea; continue;
 }
 0x20c4eea => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20c4eeb; continue;
 }
 0x20c4eeb => {
  cpu.push(0x20c4ef0); f_020a4a64(cpu); if cpu.returned_to != 0x20c4ef0 { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x20c4ef5; continue;
 }
 0x20c4ef5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20c4ee6; continue; }
  pc = 0x20c4efb; continue;
 }
 0x20c4efb => {
  { cpu.sub32(cpu.edx, 0x7eu32); }
  if (cpu.cf || cpu.zf) { pc = 0x20c4f23; continue; }
  pc = 0x20c4f00; continue;
 }
 0x20c4f00 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x20c4f0a); f_020af0db(cpu); if cpu.returned_to != 0x20c4f0a { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x20c4f14); f_020af0fe(cpu); if cpu.returned_to != 0x20c4f14 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x20c4f1e); f_020bd557(cpu); if cpu.returned_to != 0x20c4f1e { return; }
  cpu.poll(); pc = 0x20c4e8c; continue;
 }
 0x20c4f23 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x90u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x20c4f31; continue;
 }
 0x20c4f31 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c4f37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4f37); }
 let mut pc: u32 = 0x20c4f37;
 loop { match pc {
 0x20c4f37 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c4f46; continue; }
  pc = 0x20c4f45; continue;
 }
 0x20c4f45 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x20c4f46; continue;
 }
 0x20c4f46 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20c4f6e; continue; }
  pc = 0x20c4f4b; continue;
 }
 0x20c4f4b => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20c4f6e; continue; }
  pc = 0x20c4f53; continue;
 }
 0x20c4f53 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.push(0x20c4f5f); f_020c4df0(cpu); if cpu.returned_to != 0x20c4f5f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20c4f6e; continue; }
  pc = 0x20c4f66; continue;
 }
 0x20c4f66 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x20c4f53; continue;
 }
 0x20c4f6e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20c4f78; continue; }
  pc = 0x20c4f72; continue;
 }
 0x20c4f72 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c4f78 => {
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
pub fn f_020c4f8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c4f8a); }
 let mut pc: u32 = 0x20c4f8a;
 loop { match pc {
 0x20c4f8a => {
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
  { let v: u32 = cpu.eax; cpu.w32(0x217cb88u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x217cb8cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cb94u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x217cb9au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x217cb98u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x217cb9eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x217cba0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x217cba4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cb8eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x217cb92u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x217cbacu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x217cba8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x217cbaeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x217cb88u32; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if !cpu.zf { pc = 0x20c5004; continue; }
  pc = 0x20c4ffe; continue;
 }
 0x20c4ffe => {
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x20c5006; continue;
 }
 0x20c5004 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c5006; continue;
 }
 0x20c5006 => {
  { cpu.cf = false; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x217b938u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u8 = 0x4bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.r16(0x217cbacu32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x217cba8u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v = cpu.r16(0x217cbaeu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x0u32; cpu.w32(0x217b938u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  if cpu.cf { pc = 0x20c5045; continue; }
  pc = 0x20c5041; continue;
 }
 0x20c5041 => {
  { let v: u8 = 0x4du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x20c5045; continue;
 }
 0x20c5045 => {
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = (cpu.eax as u16) as u32; cpu.eax = v; }
  cpu.flat_ds(0x20c504a); cpu.push(0x20c504f); f_020c1c56(cpu); if cpu.returned_to != 0x20c504f { return; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.flat_ds(0x20c5057); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5058(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5058); }
 let mut pc: u32 = 0x20c5058;
 loop { match pc {
 0x20c5058 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c5067; continue; }
  pc = 0x20c5062; continue;
 }
 0x20c5062 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5067 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x93u32); }
  if cpu.cf { pc = 0x20c507a; continue; }
  pc = 0x20c5075; continue;
 }
 0x20c5075 => {
  { let v: u32 = 0x92u32; cpu.edx = v; }
  pc = 0x20c507a; continue;
 }
 0x20c507a => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c5081); f_020c6fae(cpu); if cpu.returned_to != 0x20c5081 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c508e); f_020c727b(cpu); if cpu.returned_to != 0x20c508e { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c5097); f_020c6fec(cpu); if cpu.returned_to != 0x20c5097 { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c50a4); f_020c6fec(cpu); if cpu.returned_to != 0x20c50a4 { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c50aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c50aa); }
 let mut pc: u32 = 0x20c50aa;
 loop { match pc {
 0x20c50aa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20c50eb; continue; }
  pc = 0x20c50bc; continue;
 }
 0x20c50bc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x20c50eb; continue; }
  pc = 0x20c50c6; continue;
 }
 0x20c50c6 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c50db; continue; }
  pc = 0x20c50ca; continue;
 }
 0x20c50ca => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3au8; cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c50db; continue;
 }
 0x20c50db => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c50e7); f_020c701d(cpu); if cpu.returned_to != 0x20c50e7 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20c50f6; continue;
 }
 0x20c50eb => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c50f6; continue; }
  pc = 0x20c50ef; continue;
 }
 0x20c50ef => {
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20c50f6; continue;
 }
 0x20c50f6 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x20c50fc; continue;
 }
 0x20c50fc => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20c50fe; continue;
 }
 0x20c50fe => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c5105); f_020c203c(cpu); if cpu.returned_to != 0x20c5105 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c5146; continue; }
  pc = 0x20c510b; continue;
 }
 0x20c510b => {
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x20c5115; continue; }
  pc = 0x20c5110; continue;
 }
 0x20c5110 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20c50fe; continue;
 }
 0x20c5115 => {
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20c5131; continue; }
  pc = 0x20c511e; continue;
 }
 0x20c511e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x20c5133; continue;
 }
 0x20c5131 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c5133; continue;
 }
 0x20c5133 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x20c5140; continue; }
  pc = 0x20c513b; continue;
 }
 0x20c513b => {
  { cpu.sub32(cpu.edx, 0x2fu32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c50fe; continue; }
  pc = 0x20c5140; continue;
 }
 0x20c5140 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20c50fc; continue;
 }
 0x20c5146 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x20c5154); f_020c5058(cpu); if cpu.returned_to != 0x20c5154 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20c515c; continue; }
  pc = 0x20c515a; continue;
 }
 0x20c515a => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x20c515c; continue;
 }
 0x20c515c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x20c516d); f_020c5058(cpu); if cpu.returned_to != 0x20c516d { return; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20c517c); f_020c5058(cpu); if cpu.returned_to != 0x20c517c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5185(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5185); }
 let mut pc: u32 = 0x20c5185;
 loop { match pc {
 0x20c5185 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217cb7cu32); cpu.ecx = v; }
  cpu.push(0x20c519f); f_020bd81f(cpu); if cpu.returned_to != 0x20c519f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c51a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c51a3); }
 let mut pc: u32 = 0x20c51a3;
 loop { match pc {
 0x20c51a3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x217cb7cu32); cpu.ecx = v; }
  cpu.push(0x20c51af); f_020c734e(cpu); if cpu.returned_to != 0x20c51af { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c51b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c51b1); }
 let mut pc: u32 = 0x20c51b1;
 loop { match pc {
 0x20c51b1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x217b145u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x217b145u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x217b104u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x217b145u32, v); }
  pc = 0x20c5217; continue;
 }
 0x20c51d5 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x20c51df); f_020bd474(cpu); if cpu.returned_to != 0x20c51df { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c5204; continue; }
  pc = 0x20c51e5; continue;
 }
 0x20c51e5 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x20c51ef); f_020bd474(cpu); if cpu.returned_to != 0x20c51ef { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c5204; continue; }
  pc = 0x20c51f5; continue;
 }
 0x20c51f5 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x216b3a0u32; cpu.eax = v; }
  cpu.push(0x20c5204); f_020bd78a(cpu); if cpu.returned_to != 0x20c5204 { return; }
  pc = 0x20c5204; continue;
 }
 0x20c5204 => {
  { let v: u32 = cpu.r32(0x22d22d0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d22d0u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c5217; continue;
 }
 0x20c5217 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c51d5; continue; }
  pc = 0x20c521e; continue;
 }
 0x20c521e => {
  { let v: u32 = cpu.edx; cpu.w32(0x22d16ecu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5228(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5228); }
 let mut pc: u32 = 0x20c5228;
 loop { match pc {
 0x20c5228 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x20c522f); f_020c5239(cpu); if cpu.returned_to != 0x20c522f { return; }
  pc = 0x20c757d; continue;
 }
 0x20c757d => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x20c757e; continue;
 }
 0x20c757e => {
  { let v: u32 = cpu.r32(0x22d16ecu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c7599; continue; }
  pc = 0x20c7588; continue;
 }
 0x20c7588 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x20c7591); f_020bd557(cpu); if cpu.returned_to != 0x20c7591 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d16ecu32, v); }
  cpu.poll(); pc = 0x20c757e; continue;
 }
 0x20c7599 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5239(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5239); }
 let mut pc: u32 = 0x20c5239;
 loop { match pc {
 0x20c5239 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x217b104u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x22d22d0u32); cpu.eax = v; }
  pc = 0x20c5286; continue;
 }
 0x20c525b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x20c527e; continue; }
  pc = 0x20c526b; continue;
 }
 0x20c526b => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x20c527e; continue; }
  pc = 0x20c5271; continue;
 }
 0x20c5271 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x20c5284; continue; }
  pc = 0x20c5275; continue;
 }
 0x20c5275 => {
  { cpu.sub32(cpu.eax, 0x217b186u32); }
  if !cpu.cf { pc = 0x20c527e; continue; }
  pc = 0x20c527c; continue;
 }
 0x20c527c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20c527e; continue;
 }
 0x20c527e => {
  cpu.push(0x20c5283); f_020c75c8(cpu); if cpu.returned_to != 0x20c5283 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x20c5284; continue;
 }
 0x20c5284 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20c5286; continue;
 }
 0x20c5286 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20c525b; continue; }
  pc = 0x20c528a; continue;
 }
 0x20c528a => {
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
pub fn f_020c5297(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c5297); } r_020c5297(cpu, 0x20c5297); }
pub fn f_020c533a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c533a); } r_020c5297(cpu, 0x20c533a); }
fn r_020c5297(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c5297 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20c52b0; continue; }
  pc = 0x20c52aa; continue;
 }
 0x20c52aa => {
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x20c52b2; continue;
 }
 0x20c52b0 => {
  { let v: u8 = 0x2bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c52b2; continue;
 }
 0x20c52b2 => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20c52d5; continue; }
  pc = 0x20c52c0; continue;
 }
 0x20c52c0 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20c52d5; continue; }
  pc = 0x20c52ce; continue;
 }
 0x20c52ce => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x20c52d5; continue;
 }
 0x20c52d5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c52e5; continue; }
  pc = 0x20c52d9; continue;
 }
 0x20c52d9 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x20c52e5; continue; }
  pc = 0x20c52e3; continue;
 }
 0x20c52e3 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c52e5; continue;
 }
 0x20c52e5 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c5331; continue; }
  pc = 0x20c52ed; continue;
 }
 0x20c52ed => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c532e; continue; }
  pc = 0x20c52f1; continue;
 }
 0x20c52f1 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x20c52f3; continue;
 }
 0x20c52f3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c52fe; continue; }
  pc = 0x20c52f8; continue;
 }
 0x20c52f8 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x20c52f3; continue;
 }
 0x20c52fe => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20c52fe; continue; }
  pc = 0x20c532c; continue;
 }
 0x20c532c => {
  pc = 0x20c5331; continue;
 }
 0x20c532e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  pc = 0x20c5331; continue;
 }
 0x20c5331 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x20c5338; continue;
 }
 0x20c5338 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c533a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c534f; continue; }
  pc = 0x20c5341; continue;
 }
 0x20c5341 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c534f; continue; }
  pc = 0x20c5345; continue;
 }
 0x20c5345 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c534f => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c5360; continue; }
  pc = 0x20c5353; continue;
 }
 0x20c5353 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x20c5360; continue; }
  pc = 0x20c5359; continue;
 }
 0x20c5359 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20c534f; continue;
 }
 0x20c5360 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c536d; continue; }
  pc = 0x20c5364; continue;
 }
 0x20c5364 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20c5364; continue; }
  pc = 0x20c536d; continue;
 }
 0x20c536d => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c5381; continue; }
  pc = 0x20c5375; continue;
 }
 0x20c5375 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c5381; continue; }
  pc = 0x20c5379; continue;
 }
 0x20c5379 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c5375; continue; }
  pc = 0x20c5381; continue;
 }
 0x20c5381 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x20c5338; continue; }
  pc = 0x20c5385; continue;
 }
 0x20c5385 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x20c5392; continue; }
  pc = 0x20c538b; continue;
 }
 0x20c538b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20c5385; continue; }
  pc = 0x20c5392; continue;
 }
 0x20c5392 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x20c5338; continue; }
  pc = 0x20c5396; continue;
 }
 0x20c5396 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20c5392; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c539d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c539d); }
 let mut pc: u32 = 0x20c539d;
 loop { match pc {
 0x20c539d => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x20c5406; continue; }
  pc = 0x20c53ae; continue;
 }
 0x20c53ae => {
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x20c53bd; continue;
 }
 0x20c53bd => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c53bd; continue; }
  pc = 0x20c53cc; continue;
 }
 0x20c53cc => {
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x20c53dc; continue; }
  pc = 0x20c53d5; continue;
 }
 0x20c53d5 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x20c53fa; continue;
 }
 0x20c53dc => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x20c53e6; continue; }
  pc = 0x20c53e1; continue;
 }
 0x20c53e1 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x20c53fa; continue; }
  pc = 0x20c53e6; continue;
 }
 0x20c53e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x20c53fa; continue; }
  pc = 0x20c53f0; continue;
 }
 0x20c53f0 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x20c53fa; continue;
 }
 0x20c53fa => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c5406); f_020aedc0(cpu); if cpu.returned_to != 0x20c5406 { return; }
  pc = 0x20c5406; continue;
 }
 0x20c5406 => {
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
pub fn f_020c540e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c540e); }
 let mut pc: u32 = 0x20c540e;
 loop { match pc {
 0x20c540e => {
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
  cpu.push(0x20c542e); f_020c76de(cpu); if cpu.returned_to != 0x20c542e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c544e; continue; }
  pc = 0x20c5432; continue;
 }
 0x20c5432 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20c5441; continue;
 }
 0x20c5439 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x20c543f; continue; }
  pc = 0x20c543d; continue;
 }
 0x20c543d => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c543f; continue;
 }
 0x20c543f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x20c5441; continue;
 }
 0x20c5441 => {
  { let v: u8 = cpu.r8(0xffffffc0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20c5439; continue; }
  pc = 0x20c5449; continue;
 }
 0x20c5449 => {
  pc = 0x20c562b; continue;
 }
 0x20c544e => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x47u32); }
  if !cpu.zf { pc = 0x20c54c3; continue; }
  pc = 0x20c545c; continue;
 }
 0x20c545c => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.esi, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x20c5475; continue; }
  pc = 0x20c5471; continue;
 }
 0x20c5471 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c54bc; continue; }
  pc = 0x20c5475; continue;
 }
 0x20c5475 => {
  { let v = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  cpu.push(0x20c547d); f_020c77b7(cpu); if cpu.returned_to != 0x20c547d { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20c548d); f_020c77df(cpu); if cpu.returned_to != 0x20c548d { return; }
  cpu.push(0x20c5492); f_0209c6d2(cpu); if cpu.returned_to != 0x20c5492 { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if (cpu.sf != cpu.of) { pc = 0x20c54a1; continue; }
  pc = 0x20c549d; continue;
 }
 0x20c549d => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c54a5; continue; }
  pc = 0x20c54a1; continue;
 }
 0x20c54a1 => {
  { let v: u8 = 0x45u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c54c3; continue;
 }
 0x20c54a5 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c54b2; continue; }
  pc = 0x20c54af; continue;
 }
 0x20c54af => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c54b2; continue;
 }
 0x20c54b2 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20c54bc; continue; }
  pc = 0x20c54bb; continue;
 }
 0x20c54bb => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x20c54bc; continue;
 }
 0x20c54bc => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = 0x46u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c54c3; continue;
 }
 0x20c54c3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x20c54f2; continue; }
  pc = 0x20c54cc; continue;
 }
 0x20c54cc => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20c54de; continue; }
  pc = 0x20c54d7; continue;
 }
 0x20c54d7 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c54f2; continue; }
  pc = 0x20c54de; continue;
 }
 0x20c54de => {
  { let v: u32 = 0x2au32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x20c54ed); f_020aedc0(cpu); if cpu.returned_to != 0x20c54ed { return; }
  pc = 0x20c563f; continue;
 }
 0x20c54f2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x20c550f; continue; }
  pc = 0x20c54fe; continue;
 }
 0x20c54fe => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c550a; continue; }
  pc = 0x20c5505; continue;
 }
 0x20c5505 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x20c550f; continue;
 }
 0x20c550a => {
  if (cpu.sf == cpu.of) { pc = 0x20c550f; continue; }
  pc = 0x20c550c; continue;
 }
 0x20c550c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x20c550f; continue;
 }
 0x20c550f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x20c5528; continue; }
  pc = 0x20c5524; continue;
 }
 0x20c5524 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c5549; continue; }
  pc = 0x20c5528; continue;
 }
 0x20c5528 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if cpu.zf { pc = 0x20c5549; continue; }
  pc = 0x20c5531; continue;
 }
 0x20c5531 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c5549; continue; }
  pc = 0x20c5538; continue;
 }
 0x20c5538 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20c5546); f_020af14b(cpu); if cpu.returned_to != 0x20c5546 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20c5549; continue;
 }
 0x20c5549 => {
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
  cpu.push(0x20c5568); f_020c781b(cpu); if cpu.returned_to != 0x20c5568 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20c5581; continue; }
  pc = 0x20c5576; continue;
 }
 0x20c5576 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c558e; continue;
 }
 0x20c5581 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x20c558e; continue; }
  pc = 0x20c5585; continue;
 }
 0x20c5585 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2bu8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x20c558e; continue;
 }
 0x20c558e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x20c5600; continue; }
  pc = 0x20c559a; continue;
 }
 0x20c559a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x20c5629; continue; }
  pc = 0x20c55a9; continue;
 }
 0x20c55a9 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x20c55b6); f_020c533a(cpu); if cpu.returned_to != 0x20c55b6 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20c55d2; continue; }
  pc = 0x20c55c7; continue;
 }
 0x20c55c7 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x20c55d2; continue; }
  pc = 0x20c55cb; continue;
 }
 0x20c55cb => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c55d2; continue;
 }
 0x20c55d2 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x20c55e4; continue; }
  pc = 0x20c55e0; continue;
 }
 0x20c55e0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c55ea; continue; }
  pc = 0x20c55e4; continue;
 }
 0x20c55e4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c55ea; continue;
 }
 0x20c55ea => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x20c55fc); f_020c5297(cpu); if cpu.returned_to != 0x20c55fc { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x20c562b; continue;
 }
 0x20c5600 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x20c5629; continue; }
  pc = 0x20c5614; continue;
 }
 0x20c5614 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x20c5627); f_020c533a(cpu); if cpu.returned_to != 0x20c5627 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x20c5629; continue;
 }
 0x20c5629 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20c562b; continue;
 }
 0x20c562b => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20c54de; continue; }
  pc = 0x20c5633; continue;
 }
 0x20c5633 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x20c563f); f_020c539d(cpu); if cpu.returned_to != 0x20c563f { return; }
  pc = 0x20c563f; continue;
 }
 0x20c563f => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5648(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5648); }
 let mut pc: u32 = 0x20c5648;
 loop { match pc {
 0x20c5648 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x20c5675; continue; }
  pc = 0x20c5653; continue;
 }
 0x20c5653 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x20c565b); f_020c1ccb(cpu); if cpu.returned_to != 0x20c565b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c5675; continue; }
  pc = 0x20c565f; continue;
 }
 0x20c565f => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x20c5675; continue; }
  pc = 0x20c566d; continue;
 }
 0x20c566d => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x20c5675; continue;
 }
 0x20c5675 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5679(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5679); }
 let mut pc: u32 = 0x20c5679;
 loop { match pc {
 0x20c5679 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x20c568b); f_020c1cf2(cpu); if cpu.returned_to != 0x20c568b { return; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x20c56be; continue; }
  pc = 0x20c568f; continue;
 }
 0x20c568f => {
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
  if (cpu.sf == cpu.of) { pc = 0x20c56be; continue; }
  pc = 0x20c56b1; continue;
 }
 0x20c56b1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20c56bc); f_020c1c5a(cpu); if cpu.returned_to != 0x20c56bc { return; }
  pc = 0x20c56e5; continue;
 }
 0x20c56be => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20c56b1; continue; }
  pc = 0x20c56d5; continue;
 }
 0x20c56d5 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x20c56e3; continue; }
  pc = 0x20c56d9; continue;
 }
 0x20c56d9 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x20c56e3); f_020af0db(cpu); if cpu.returned_to != 0x20c56e3 { return; }
  pc = 0x20c56e3; continue;
 }
 0x20c56e3 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c56e5; continue;
 }
 0x20c56e5 => {
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
pub fn f_020c56f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c56f0); }
 let mut pc: u32 = 0x20c56f0;
 loop { match pc {
 0x20c56f0 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x20c5713; continue; }
  pc = 0x20c56fa; continue;
 }
 0x20c56fa => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x20c5706; continue; }
  pc = 0x20c56ff; continue;
 }
 0x20c56ff => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5706 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), (cpu.ecx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x20c56fa; continue; }
  pc = 0x20c5713; continue;
 }
 0x20c5713 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5718(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5718); }
 let mut pc: u32 = 0x20c5718;
 loop { match pc {
 0x20c5718 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x217b0e0u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x20c5736; continue; }
  pc = 0x20c572f; continue;
 }
 0x20c572f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c58d8; continue;
 }
 0x20c5736 => {
  { let v: u32 = 0x42u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20c575d; continue; }
  pc = 0x20c5754; continue;
 }
 0x20c5754 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c58d6; continue; }
  pc = 0x20c575d; continue;
 }
 0x20c575d => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c577c; continue; }
  pc = 0x20c5777; continue;
 }
 0x20c5777 => {
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x20c57b1; continue; }
  pc = 0x20c577c; continue;
 }
 0x20c577c => {
  { let v = cpu.edi; cpu.set_segment(5, v as u16); }
  { let v: u32 = 0x216b448u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x216b448u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x20c57af; continue; }
  pc = 0x20c5794; continue;
 }
 0x20c5794 => {
  { let v = cpu.esi; cpu.set_segment(5, v as u16); }
  pc = 0x20c5796; continue;
 }
 0x20c5796 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x20c57a2; continue; }
  pc = 0x20c579b; continue;
 }
 0x20c579b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c57b1; continue;
 }
 0x20c57a2 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), (cpu.ecx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x20c5796; continue; }
  pc = 0x20c57af; continue;
 }
 0x20c57af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c57b1; continue;
 }
 0x20c57b1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c58aa; continue; }
  pc = 0x20c57b9; continue;
 }
 0x20c57b9 => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d2372u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c588f; continue;
 }
 0x20c57ee => {
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c5813; continue; }
  pc = 0x20c580a; continue;
 }
 0x20c580a => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x20c587d; continue; }
  pc = 0x20c5813; continue;
 }
 0x20c5813 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20c5820; continue;
 }
 0x20c5820 => {
  { let v: u32 = cpu.r32(0x217cbb4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x20c585c; continue; }
  pc = 0x20c583f; continue;
 }
 0x20c583f => {
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  pc = 0x20c5843; continue;
 }
 0x20c5843 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x20c584f; continue; }
  pc = 0x20c5848; continue;
 }
 0x20c5848 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c585e; continue;
 }
 0x20c584f => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x20c5843; continue; }
  pc = 0x20c585c; continue;
 }
 0x20c585c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c585e; continue;
 }
 0x20c585e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c5875; continue; }
  pc = 0x20c5862; continue;
 }
 0x20c5862 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cbb8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c5875; continue;
 }
 0x20c5875 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c5820; continue; }
  pc = 0x20c587d; continue;
 }
 0x20c587d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c588f; continue;
 }
 0x20c588f => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20c57ee; continue; }
  pc = 0x20c58aa; continue;
 }
 0x20c58aa => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c575d; continue; }
  pc = 0x20c58cd; continue;
 }
 0x20c58cd => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { cpu.poll(); pc = 0x20c575d; continue; }
  pc = 0x20c58d6; continue;
 }
 0x20c58d6 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x20c58d8; continue;
 }
 0x20c58d8 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
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
pub fn f_020c58e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c58e4); }
 let mut pc: u32 = 0x20c58e4;
 loop { match pc {
 0x20c58e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x22d2372u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x20c590e; continue; }
  pc = 0x20c5907; continue;
 }
 0x20c5907 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c5ce2; continue;
 }
 0x20c590e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x20c5913, "Lsl");
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { pc = 0x20c5924; continue; }
  pc = 0x20c591d; continue;
 }
 0x20c591d => {
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x20c5930; continue; }
  pc = 0x20c5924; continue;
 }
 0x20c5924 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c592f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5930 => {
  { let v: u32 = 0x20u32; cpu.ebp = v; }
  { let v = cpu.r16(0x22d2372u32.wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.cf || cpu.zf) { pc = 0x20c5a11; continue; }
  pc = 0x20c5951; continue;
 }
 0x20c5951 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x217cbf4u32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x20c5970; continue; }
  pc = 0x20c595c; continue;
 }
 0x20c595c => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x217cc25u32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x20c5970; continue; }
  pc = 0x20c5964; continue;
 }
 0x20c5964 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x217cc31u32.wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x20c5a03; continue; }
  pc = 0x20c5970; continue;
 }
 0x20c5970 => {
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v: u32 = 0x217cbf4u32; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[5] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x20c5992; continue; }
  pc = 0x20c598d; continue;
 }
 0x20c598d => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x20c5992; continue;
 }
 0x20c5992 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c59ab; continue; }
  pc = 0x20c5998; continue;
 }
 0x20c5998 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c59ab; continue; }
  pc = 0x20c59a4; continue;
 }
 0x20c59a4 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20c5a11; continue; }
  pc = 0x20c59ab; continue;
 }
 0x20c59ab => {
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { let v: u32 = 0x217cc25u32; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[5] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x20c59cd; continue; }
  pc = 0x20c59c8; continue;
 }
 0x20c59c8 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x20c59cd; continue;
 }
 0x20c59cd => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c59d7; continue; }
  pc = 0x20c59d3; continue;
 }
 0x20c59d3 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c59d7; continue;
 }
 0x20c59d7 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x217cc31u32; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[5] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x20c59f9; continue; }
  pc = 0x20c59f4; continue;
 }
 0x20c59f4 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x20c59f9; continue;
 }
 0x20c59f9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c5a03; continue; }
  pc = 0x20c59ff; continue;
 }
 0x20c59ff => {
  { let v: u32 = cpu.ebp; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  pc = 0x20c5a03; continue;
 }
 0x20c5a03 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x20c5951; continue; }
  pc = 0x20c5a11; continue;
 }
 0x20c5a11 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c5a7a; continue; }
  pc = 0x20c5a19; continue;
 }
 0x20c5a19 => {
  { let v = cpu.r16(0x22d2372u32.wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = (cpu.segments[5] as u32); cpu.edx = v; }
  pc = 0x20c5a26; continue;
 }
 0x20c5a26 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u32 = 0x217cc0eu32; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x20c5a46; continue; }
  pc = 0x20c5a41; continue;
 }
 0x20c5a41 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x20c5a46; continue;
 }
 0x20c5a46 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c5a60; continue; }
  pc = 0x20c5a4c; continue;
 }
 0x20c5a4c => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5])), 0x57u8); }
  if !cpu.zf { pc = 0x20c5a58; continue; }
  pc = 0x20c5a53; continue;
 }
 0x20c5a53 => {
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5]), v); }
  pc = 0x20c5a58; continue;
 }
 0x20c5a58 => {
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c5a60; continue;
 }
 0x20c5a60 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, 0x20u32); }
  if cpu.cf { cpu.poll(); pc = 0x20c5a26; continue; }
  pc = 0x20c5a67; continue;
 }
 0x20c5a67 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20c5a7a; continue; }
  pc = 0x20c5a6e; continue;
 }
 0x20c5a6e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c5a79); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5a7a => {
  { let v: u16 = cpu.r16(0x217cbbau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.unimplemented(0x20c5a85, "Lsl");
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { pc = 0x20c5bd3; continue; }
  pc = 0x20c5a93; continue;
 }
 0x20c5a93 => {
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c5bd3; continue; }
  pc = 0x20c5a9e; continue;
 }
 0x20c5a9e => {
  { let v: u16 = cpu.r16(0x217cbbau32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x20c5afc; continue; }
  pc = 0x20c5aba; continue;
 }
 0x20c5aba => {
  { let v = cpu.r16(0x34u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  pc = 0x20c5abe; continue;
 }
 0x20c5abe => {
  { let v: u8 = cpu.r8(0x217cc15u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5]))); }
  if !cpu.zf { pc = 0x20c5af2; continue; }
  pc = 0x20c5aca; continue;
 }
 0x20c5aca => {
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { let v: u32 = 0x217cc15u32; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[5] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x20c5aec; continue; }
  pc = 0x20c5ae7; continue;
 }
 0x20c5ae7 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x20c5aec; continue;
 }
 0x20c5aec => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20c5afc; continue; }
  pc = 0x20c5af2; continue;
 }
 0x20c5af2 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { cpu.poll(); pc = 0x20c5abe; continue; }
  pc = 0x20c5afc; continue;
 }
 0x20c5afc => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x217cc21u32; cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x2u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x20c5b25; continue; }
  pc = 0x20c5b20; continue;
 }
 0x20c5b20 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x20c5b25; continue;
 }
 0x20c5b25 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20c5b7b; continue; }
  pc = 0x20c5b2b; continue;
 }
 0x20c5b2b => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp.wrapping_add(0xdu32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x20c5bd3; continue; }
  pc = 0x20c5b3d; continue;
 }
 0x20c5b3d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x217cbbau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.flat_ds(0x20c5b5a); cpu.push(0x20c5b5f); f_020a594c(cpu); if cpu.returned_to != 0x20c5b5f { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c5bd3; continue;
 }
 0x20c5b7b => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0x6au8); }
  if !cpu.zf { pc = 0x20c5bd3; continue; }
  pc = 0x20c5b86; continue;
 }
 0x20c5b86 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x22d2378u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x20c5bd3; continue; }
  pc = 0x20c5b98; continue;
 }
 0x20c5b98 => {
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x217cbbau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.flat_ds(0x20c5bb5); cpu.push(0x20c5bba); f_020a594c(cpu); if cpu.returned_to != 0x20c5bba { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c5bd3; continue;
 }
 0x20c5bd3 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x104u32); }
  if cpu.cf { pc = 0x20c5ca9; continue; }
  pc = 0x20c5be3; continue;
 }
 0x20c5be3 => {
  { let v = cpu.r16(0x22d2372u32.wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = 0xffffff00u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2eu8); }
  if cpu.zf { pc = 0x20c5c09; continue; }
  pc = 0x20c5bfd; continue;
 }
 0x20c5bfd => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x20c5c09; continue; }
  pc = 0x20c5c07; continue;
 }
 0x20c5c07 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x20c5c09; continue;
 }
 0x20c5c09 => {
  { let v = cpu.r16(0x38u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5])), 0x2eu8); }
  if !cpu.zf { pc = 0x20c5ca9; continue; }
  pc = 0x20c5c18; continue;
 }
 0x20c5c18 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x20c5c20; continue;
 }
 0x20c5c20 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5])), 0x0u8); }
  if !cpu.zf { pc = 0x20c5ca9; continue; }
  pc = 0x20c5c2a; continue;
 }
 0x20c5c2a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { cpu.poll(); pc = 0x20c5c20; continue; }
  pc = 0x20c5c33; continue;
 }
 0x20c5c33 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217b0d8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = (cpu.edx as u16); cpu.set_segment(0, v as u16); }
  { let v = (cpu.ecx as u16); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x20c5c50; continue;
 }
 0x20c5c50 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c5c6a; continue; }
  pc = 0x20c5c59; continue;
 }
 0x20c5c59 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c5c50; continue; }
  pc = 0x20c5c6a; continue;
 }
 0x20c5c6a => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x217b0d8u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x20c5ca9; continue; }
  pc = 0x20c5c85; continue;
 }
 0x20c5c85 => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x20c5c8a; continue;
 }
 0x20c5c8a => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x5cu8); }
  if cpu.zf { pc = 0x20c5c9a; continue; }
  pc = 0x20c5c95; continue;
 }
 0x20c5c95 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x20c5ca5; continue; }
  pc = 0x20c5c9a; continue;
 }
 0x20c5c9a => {
  { let v = cpu.r16(0x38u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[5]), v); }
  pc = 0x20c5ca9; continue;
 }
 0x20c5ca5 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x20c5c8a; continue; }
  pc = 0x20c5ca9; continue;
 }
 0x20c5ca9 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c5cdd; continue; }
  pc = 0x20c5cb1; continue;
 }
 0x20c5cb1 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x216b458u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x22d2372u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edi; cpu.push(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { cpu.movs(4, false); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x20c5cdd; continue;
 }
 0x20c5cdd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c5ce2; continue;
 }
 0x20c5ce2 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c5ceb); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5cec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5cec); }
 let mut pc: u32 = 0x20c5cec;
 loop { match pc {
 0x20c5cec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c5d14; continue; }
  pc = 0x20c5cf6; continue;
 }
 0x20c5cf6 => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20c5d14); f_020a594c(cpu); if cpu.returned_to != 0x20c5d14 { return; }
  pc = 0x20c5d14; continue;
 }
 0x20c5d14 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5d1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5d1c); }
 let mut pc: u32 = 0x20c5d1c;
 loop { match pc {
 0x20c5d1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x20c5d48); f_020a594c(cpu); if cpu.returned_to != 0x20c5d48 { return; }
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20c5d56; continue; }
  pc = 0x20c5d4f; continue;
 }
 0x20c5d4f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c5e38; continue;
 }
 0x20c5d56 => {
  { let v: u32 = 0x9u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  cpu.unimplemented(0x20c5d69, "Lar");
  { let v: u32 = cpu.ror32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0ffu32; cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xbfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c5d84; continue; }
  pc = 0x20c5d7f; continue;
 }
 0x20c5d7f => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.esp)) | 0x9u8; cpu.w8(0x40u32.wrapping_add(cpu.esp), v); }
  pc = 0x20c5d84; continue;
 }
 0x20c5d84 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20c5d96); f_020a594c(cpu); if cpu.returned_to != 0x20c5d96 { return; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20c5dd3; continue; }
  pc = 0x20c5d9e; continue;
 }
 0x20c5d9e => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if cpu.zf { pc = 0x20c5dc7; continue; }
  pc = 0x20c5da3; continue;
 }
 0x20c5da3 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20c5dc7); f_020a594c(cpu); if cpu.returned_to != 0x20c5dc7 { return; }
  pc = 0x20c5dc7; continue;
 }
 0x20c5dc7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5dd3 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20c5dff); f_020a594c(cpu); if cpu.returned_to != 0x20c5dff { return; }
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20c5e36; continue; }
  pc = 0x20c5e06; continue;
 }
 0x20c5e06 => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if cpu.zf { pc = 0x20c5e2a; continue; }
  pc = 0x20c5e0b; continue;
 }
 0x20c5e0b => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20c5e2a); f_020a594c(cpu); if cpu.returned_to != 0x20c5e2a { return; }
  pc = 0x20c5e2a; continue;
 }
 0x20c5e2a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5e36 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20c5e38; continue;
 }
 0x20c5e38 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
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
pub fn f_020c5e44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5e44); }
 let mut pc: u32 = 0x20c5e44;
 loop { match pc {
 0x20c5e44 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  cpu.push(0x20c5e62); f_020a594c(cpu); if cpu.returned_to != 0x20c5e62 { return; }
  { let v: u32 = 0x25e9u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x217b91cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x217b0e0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  cpu.push(0x20c5e96); f_020aef01(cpu); if cpu.returned_to != 0x20c5e96 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5ea0); }
 let mut pc: u32 = 0x20c5ea0;
 loop { match pc {
 0x20c5ea0 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x22d2374u32), 0x0u16); }
  if cpu.zf { pc = 0x20c5ec6; continue; }
  pc = 0x20c5eab; continue;
 }
 0x20c5eab => {
  { cpu.sub16(cpu.r16(0x22d2376u32), 0x0u16); }
  if cpu.zf { pc = 0x20c5ec6; continue; }
  pc = 0x20c5eb5; continue;
 }
 0x20c5eb5 => {
  { cpu.sub16(cpu.r16(0x22d2370u32), 0x0u16); }
  if cpu.zf { pc = 0x20c5ec6; continue; }
  pc = 0x20c5ebf; continue;
 }
 0x20c5ebf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5ec6 => {
  cpu.push(0x20c5ecb); f_020c5718(cpu); if cpu.returned_to != 0x20c5ecb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c5f6d; continue; }
  pc = 0x20c5ed3; continue;
 }
 0x20c5ed3 => {
  cpu.push(0x20c5ed8); f_020c58e4(cpu); if cpu.returned_to != 0x20c5ed8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c5f6d; continue; }
  pc = 0x20c5ee0; continue;
 }
 0x20c5ee0 => {
  { let v: u32 = 0x2150000u32; cpu.edx = v; }
  cpu.push(0x20c5eea); f_020c5e44(cpu); if cpu.returned_to != 0x20c5eea { return; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x2150002u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x20c5f02); f_020c5d1c(cpu); if cpu.returned_to != 0x20c5f02 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d2374u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x20c5f0f); f_020c5d1c(cpu); if cpu.returned_to != 0x20c5f0f { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d2376u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x20c5f1c); f_020c5d1c(cpu); if cpu.returned_to != 0x20c5f1c { return; }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d2370u32, v); }
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  if cpu.zf { pc = 0x20c5f44; continue; }
  pc = 0x20c5f2e; continue;
 }
 0x20c5f2e => {
  { cpu.sub16(cpu.r16(0x22d2376u32), 0x0u16); }
  if cpu.zf { pc = 0x20c5f44; continue; }
  pc = 0x20c5f38; continue;
 }
 0x20c5f38 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c5f44; continue; }
  pc = 0x20c5f3d; continue;
 }
 0x20c5f3d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c5f44 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20c5f51); f_020c5cec(cpu); if cpu.returned_to != 0x20c5f51 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20c5f5e); f_020c5cec(cpu); if cpu.returned_to != 0x20c5f5e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20c5f6b); f_020c5cec(cpu); if cpu.returned_to != 0x20c5f6b { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c5f6d; continue;
 }
 0x20c5f6d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c5f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c5f70); }
 let mut pc: u32 = 0x20c5f70;
 loop { match pc {
 0x20c5f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c5fa2; continue; }
  pc = 0x20c5f7f; continue;
 }
 0x20c5f7f => {
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20c5fa2); f_020a594c(cpu); if cpu.returned_to != 0x20c5fa2 { return; }
  pc = 0x20c5fa2; continue;
 }
 0x20c5fa2 => {
  { let v: u16 = cpu.r16(0x22d2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c5fcb; continue; }
  pc = 0x20c5fad; continue;
 }
 0x20c5fad => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20c5fcb); f_020a594c(cpu); if cpu.returned_to != 0x20c5fcb { return; }
  pc = 0x20c5fcb; continue;
 }
 0x20c5fcb => {
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c5ff9; continue; }
  pc = 0x20c5fd6; continue;
 }
 0x20c5fd6 => {
  { let v: u32 = 0x1u32; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20c5ff9); f_020a594c(cpu); if cpu.returned_to != 0x20c5ff9 { return; }
  pc = 0x20c5ff9; continue;
 }
 0x20c5ff9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6000); }
 let mut pc: u32 = 0x20c6000;
 loop { match pc {
 0x20c6000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x8u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c620d; continue; }
  pc = 0x20c601f; continue;
 }
 0x20c601f => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c6022; continue;
 }
 0x20c6022 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c603a; continue; }
  pc = 0x20c602a; continue;
 }
 0x20c602a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c6022; continue; }
  pc = 0x20c603a; continue;
 }
 0x20c603a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x216b468u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20c604b; continue;
 }
 0x20c604b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c6063; continue; }
  pc = 0x20c6053; continue;
 }
 0x20c6053 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c604b; continue; }
  pc = 0x20c6063; continue;
 }
 0x20c6063 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x22d2379u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c606e; continue;
 }
 0x20c606e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c6086; continue; }
  pc = 0x20c6076; continue;
 }
 0x20c6076 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c606e; continue; }
  pc = 0x20c6086; continue;
 }
 0x20c6086 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c608e); f_020c79f8(cpu); if cpu.returned_to != 0x20c608e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c6099; continue; }
  pc = 0x20c6092; continue;
 }
 0x20c6092 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20c620f; continue;
 }
 0x20c6099 => {
  { let v: u32 = cpu.r32(0x217b0d8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x5cu32; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c60a7; continue;
 }
 0x20c60a7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c60bf; continue; }
  pc = 0x20c60af; continue;
 }
 0x20c60af => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c60a7; continue; }
  pc = 0x20c60bf; continue;
 }
 0x20c60bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c60c7); f_020a2514(cpu); if cpu.returned_to != 0x20c60c7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c60d7; continue; }
  pc = 0x20c60cb; continue;
 }
 0x20c60cb => {
  { let v: u32 = 0x2fu32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c60d7); f_020a2514(cpu); if cpu.returned_to != 0x20c60d7 { return; }
  pc = 0x20c60d7; continue;
 }
 0x20c60d7 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c6116; continue; }
  pc = 0x20c60db; continue;
 }
 0x20c60db => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c60e1; continue;
 }
 0x20c60e1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c60f9; continue; }
  pc = 0x20c60e9; continue;
 }
 0x20c60e9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c60e1; continue; }
  pc = 0x20c60f9; continue;
 }
 0x20c60f9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x22d2379u32; cpu.eax = v; }
  cpu.push(0x20c6106); f_020c79f8(cpu); if cpu.returned_to != 0x20c6106 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c6116; continue; }
  pc = 0x20c610a; continue;
 }
 0x20c610a => {
  { let v: u32 = 0x22d2379u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c6116 => {
  { let v: u32 = 0x22d2379u32; cpu.ebx = v; }
  { let v: u32 = 0x216b470u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x20c6127); f_020c7a33(cpu); if cpu.returned_to != 0x20c6127 { return; }
  { cpu.sub8(cpu.r8(0x22d2379u32), 0x0u8); }
  if cpu.zf { pc = 0x20c613c; continue; }
  pc = 0x20c6130; continue;
 }
 0x20c6130 => {
  { let v: u32 = 0x22d2379u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c613c => {
  { let v: u32 = 0x22d2379u32; cpu.edi = v; }
  { let v: u32 = 0x216b478u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x22d2379u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20c615f; continue;
 }
 0x20c615f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c6177; continue; }
  pc = 0x20c6167; continue;
 }
 0x20c6167 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c615f; continue; }
  pc = 0x20c6177; continue;
 }
 0x20c6177 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c617f); f_020c79f8(cpu); if cpu.returned_to != 0x20c617f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c618c; continue; }
  pc = 0x20c6183; continue;
 }
 0x20c6183 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c618c => {
  { let v: u32 = 0x216b48cu32; cpu.eax = v; }
  cpu.push(0x20c6196); f_020bd5d9(cpu); if cpu.returned_to != 0x20c6196 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x5cu32; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c619e; continue;
 }
 0x20c619e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c61b6; continue; }
  pc = 0x20c61a6; continue;
 }
 0x20c61a6 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c619e; continue; }
  pc = 0x20c61b6; continue;
 }
 0x20c61b6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c61be); f_020a2514(cpu); if cpu.returned_to != 0x20c61be { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c61ce; continue; }
  pc = 0x20c61c2; continue;
 }
 0x20c61c2 => {
  { let v: u32 = 0x2fu32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c61ce); f_020a2514(cpu); if cpu.returned_to != 0x20c61ce { return; }
  pc = 0x20c61ce; continue;
 }
 0x20c61ce => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c620d; continue; }
  pc = 0x20c61d2; continue;
 }
 0x20c61d2 => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c61d8; continue;
 }
 0x20c61d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c61f0; continue; }
  pc = 0x20c61e0; continue;
 }
 0x20c61e0 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c61d8; continue; }
  pc = 0x20c61f0; continue;
 }
 0x20c61f0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x22d2379u32; cpu.eax = v; }
  cpu.push(0x20c61fd); f_020c79f8(cpu); if cpu.returned_to != 0x20c61fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c620d; continue; }
  pc = 0x20c6201; continue;
 }
 0x20c6201 => {
  { let v: u32 = 0x22d2379u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c620d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c620f; continue;
 }
 0x20c620f => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6218(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6218); }
 let mut pc: u32 = 0x20c6218;
 loop { match pc {
 0x20c6218 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x28u32); cpu.esp = v; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  cpu.push(0x20c624b); f_020aef01(cpu); if cpu.returned_to != 0x20c624b { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x20c6255); f_020c462b(cpu); if cpu.returned_to != 0x20c6255 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c625c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c625c); } r_020c625c(cpu, 0x20c625c); }
pub fn f_020c62cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c62cc); } r_020c625c(cpu, 0x20c62cc); }
pub fn f_020c6357(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c6357); } r_020c625c(cpu, 0x20c6357); }
pub fn f_020c6388(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c6388); } r_020c625c(cpu, 0x20c6388); }
pub fn f_020c6464(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c6464); } r_020c625c(cpu, 0x20c6464); }
pub fn f_020c6510(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c6510); } r_020c625c(cpu, 0x20c6510); }
pub fn f_020c65ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c65ce); } r_020c625c(cpu, 0x20c65ce); }
fn r_020c625c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c625c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20c6268); f_020c5ea0(cpu); if cpu.returned_to != 0x20c6268 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c65c8; continue; }
  pc = 0x20c6270; continue;
 }
 0x20c6270 => {
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x20c6278); f_020c6000(cpu); if cpu.returned_to != 0x20c6278 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x20c62c2; continue; }
  pc = 0x20c6280; continue;
 }
 0x20c6280 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x20c65c8; continue; }
  pc = 0x20c629c; continue;
 }
 0x20c629c => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x20c65ceu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x217cbb8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x20c62c2 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20c65ce; continue;
 }
 0x20c62cc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20c62d8); f_020c5ea0(cpu); if cpu.returned_to != 0x20c62d8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c65c8; continue; }
  pc = 0x20c62e0; continue;
 }
 0x20c62e0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x20c65ceu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x217cbc0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x20c6357 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x20c65c8; continue; }
  pc = 0x20c636f; continue;
 }
 0x20c636f => {
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20c65ce; continue;
 }
 0x20c6388 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20c6394); f_020c5ea0(cpu); if cpu.returned_to != 0x20c6394 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c65c8; continue; }
  pc = 0x20c639c; continue;
 }
 0x20c639c => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x20c65c8; continue; }
  pc = 0x20c63b8; continue;
 }
 0x20c63b8 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x20c65ceu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x217cbd0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x20c6464 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20c6470); f_020c5ea0(cpu); if cpu.returned_to != 0x20c6470 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c65c8; continue; }
  pc = 0x20c6478; continue;
 }
 0x20c6478 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2376u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x20c65c8; continue; }
  pc = 0x20c6494; continue;
 }
 0x20c6494 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x20c65c8; continue; }
  pc = 0x20c64b0; continue;
 }
 0x20c64b0 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x20c65ceu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x22d2378u32), 0x0u8); }
  if !cpu.zf { pc = 0x20c64da; continue; }
  pc = 0x20c64c4; continue;
 }
 0x20c64c4 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x20c64f6; continue;
 }
 0x20c64da => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2376u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x20c64f6; continue;
 }
 0x20c64f6 => {
  { let v: u32 = 0x217cbe0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x20c6510 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x20c651c); f_020c5ea0(cpu); if cpu.returned_to != 0x20c651c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c65c8; continue; }
  pc = 0x20c6524; continue;
 }
 0x20c6524 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x20c65c8; continue; }
  pc = 0x20c6540; continue;
 }
 0x20c6540 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x20c65ceu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x22d2374u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x217cbe8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x22d2370u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x20c65c8 => {
  cpu.unimplemented(0x20c65c8, "Int3");
  { let v: u32 = 0x8badf00du32; cpu.eax = v; }
  pc = 0x20c65ce; continue;
 }
 0x20c65ce => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c65d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c65d4); }
 let mut pc: u32 = 0x20c65d4;
 loop { match pc {
 0x20bdba5 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[5] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x20bdbad); cpu.push(0x20bdbb2); f_020bdbf2(cpu); if cpu.returned_to != 0x20bdbb2 { return; }
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
  cpu.flat_ds(0x20bdbf1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c65d4 => {
  cpu.poll(); pc = 0x20bdba5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c65d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c65d9); }
 let mut pc: u32 = 0x20c65d9;
 loop { match pc {
 0x20c65d9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x20c6601; continue; }
  pc = 0x20c65e8; continue;
 }
 0x20c65e8 => {
  { cpu.sub8(cpu.r8(0x217b940u32), 0x0u8); }
  if cpu.zf { pc = 0x20c6601; continue; }
  pc = 0x20c65f1; continue;
 }
 0x20c65f1 => {
  { let v: u16 = cpu.r16(0x217b0e2u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20c65f7; continue;
 }
 0x20c65f7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d2480u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c6601 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c6612; continue; }
  pc = 0x20c660d; continue;
 }
 0x20c660d => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c661c; continue; }
  pc = 0x20c6612; continue;
 }
 0x20c6612 => {
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x20c6627; continue; }
  pc = 0x20c661c; continue;
 }
 0x20c661c => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x34u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x20c65f7; continue;
 }
 0x20c6627 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c6639; continue; }
  pc = 0x20c6630; continue;
 }
 0x20c6630 => {
  { cpu.sub8(cpu.r8(0x217b93fu32), 0x0u8); }
  if cpu.zf { pc = 0x20c6645; continue; }
  pc = 0x20c6639; continue;
 }
 0x20c6639 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x20c664a; continue; }
  pc = 0x20c6645; continue;
 }
 0x20c6645 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20c65f7; continue;
 }
 0x20c664a => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c66c0; continue; }
  pc = 0x20c664f; continue;
 }
 0x20c664f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c66c0; continue; }
  pc = 0x20c665b; continue;
 }
 0x20c665b => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20c6687; continue; }
  pc = 0x20c6678; continue;
 }
 0x20c6678 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x216b494u32; cpu.eax = v; }
  cpu.push(0x20c6687); f_020bd78a(cpu); if cpu.returned_to != 0x20c6687 { return; }
  pc = 0x20c6687; continue;
 }
 0x20c6687 => {
  { let v: u32 = 0xfffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x22d2480u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x20c66c9; continue; }
  pc = 0x20c66ad; continue;
 }
 0x20c66ad => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x216b4bau32; cpu.eax = v; }
  cpu.push(0x20c66bc); f_020bd78a(cpu); if cpu.returned_to != 0x20c66bc { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c66c0 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x22d2480u32, v); }
  pc = 0x20c66c9; continue;
 }
 0x20c66c9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c66cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c66cd); }
 let mut pc: u32 = 0x20c66cd;
 loop { match pc {
 0x20c66cd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c66f3; continue; }
  pc = 0x20c66da; continue;
 }
 0x20c66da => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c66f3; continue; }
  pc = 0x20c66e6; continue;
 }
 0x20c66e6 => {
  { let v: u16 = cpu.r16(0x22d2480u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x20c66f3; continue;
 }
 0x20c66f3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c66fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c66fb); }
 let mut pc: u32 = 0x20c66fb;
 loop { match pc {
 0x20c66fb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x20c670c); f_020c7b2d(cpu); if cpu.returned_to != 0x20c670c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x216b79cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x20c671e); f_020c7b4c(cpu); if cpu.returned_to != 0x20c671e { return; }
  { let v: u32 = 0x216b569u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20c672a); f_020c7b4c(cpu); if cpu.returned_to != 0x20c672a { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c6733); f_020c7b4c(cpu); if cpu.returned_to != 0x20c6733 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20c673f); f_020c1dbb(cpu); if cpu.returned_to != 0x20c673f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6743(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6743); }
 let mut pc: u32 = 0x20c6743;
 loop { match pc {
 0x20c6743 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x217cc40u32); cpu.push(0x20c674d); cpu.call(t); if cpu.returned_to != 0x20c674d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c676c; continue; }
  pc = 0x20c6751; continue;
 }
 0x20c6751 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x20c675b); f_020c66fb(cpu); if cpu.returned_to != 0x20c675b { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x20c6767; continue; }
  pc = 0x20c6760; continue;
 }
 0x20c6760 => {
  cpu.push(0x20c6765); f_020af0d6(cpu); if cpu.returned_to != 0x20c6765 { return; }
  pc = 0x20c676c; continue;
 }
 0x20c6767 => {
  cpu.push(0x20c676c); f_020af0e7(cpu); if cpu.returned_to != 0x20c676c { return; }
  pc = 0x20c676c; continue;
 }
 0x20c676c => {
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6772(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c6772); } r_020c6772(cpu, 0x20c6772); }
pub fn f_020c6807(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c6807); } r_020c6772(cpu, 0x20c6807); }
fn r_020c6772(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c6772 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x20c677c); cpu.push(0x20c6781); f_020ae040(cpu); if cpu.returned_to != 0x20c6781 { return; }
  { let v: u32 = 0x22d3474u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217b920u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x217b938u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x22d2484u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217b920u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20c67d6; continue; }
  pc = 0x20c67b5; continue;
 }
 0x20c67b5 => {
  { let v: u32 = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x20c462au32); }
  if cpu.zf { pc = 0x20c67d6; continue; }
  pc = 0x20c67c1; continue;
 }
 0x20c67c1 => {
  { cpu.sub32(cpu.eax, 0x20c6ac9u32); }
  if cpu.zf { pc = 0x20c67d6; continue; }
  pc = 0x20c67c8; continue;
 }
 0x20c67c8 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x20c67cf); cpu.push(0x20c67d4); f_020c7cbb(cpu); if cpu.returned_to != 0x20c67d4 { return; }
  pc = 0x20c67e7; continue;
 }
 0x20c67d6 => {
  { let v: u16 = cpu.r16(0x217cc4cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc48u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x20c67e2; continue;
 }
 0x20c67e2 => {
  cpu.flat_ds(0x20c67e2); cpu.push(0x20c67e7); f_020afe50(cpu); if cpu.returned_to != 0x20c67e7 { return; }
  pc = 0x20c67e7; continue;
 }
 0x20c67e7 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217b920u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x20c6806); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x20c6807 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x20c6811); cpu.push(0x20c6816); f_020ae040(cpu); if cpu.returned_to != 0x20c6816 { return; }
  { let v: u32 = 0x22d3474u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217b920u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x217b938u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x22d2484u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217b920u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20c686e; continue; }
  pc = 0x20c684a; continue;
 }
 0x20c684a => {
  { let v: u32 = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x20c462au32); }
  if cpu.zf { pc = 0x20c686e; continue; }
  pc = 0x20c6856; continue;
 }
 0x20c6856 => {
  { cpu.sub32(cpu.eax, 0x20c6913u32); }
  if cpu.zf { pc = 0x20c686e; continue; }
  pc = 0x20c685d; continue;
 }
 0x20c685d => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x20c6864); cpu.push(0x20c6869); f_020c7cbb(cpu); if cpu.returned_to != 0x20c6869 { return; }
  cpu.poll(); pc = 0x20c67e7; continue;
 }
 0x20c686e => {
  { let v: u16 = cpu.r16(0x217cc52u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc4eu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.poll(); pc = 0x20c67e2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c687f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c687f); }
 let mut pc: u32 = 0x20c687f;
 loop { match pc {
 0x20c687f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x217cc60u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20c6906; continue; }
  pc = 0x20c6893; continue;
 }
 0x20c6893 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20c68aa; continue; }
  pc = 0x20c689f; continue;
 }
 0x20c689f => {
  { let v: u16 = 0x1686u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x20c68fc; continue; }
  pc = 0x20c68aa; continue;
 }
 0x20c68aa => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c68fc; continue; }
  pc = 0x20c68f0; continue;
 }
 0x20c68f0 => {
  { let v: u32 = 0x1u32; cpu.w32(0x217cc60u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6906; continue;
 }
 0x20c68fc => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x217cc60u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6906; continue;
 }
 0x20c6906 => {
  { let v: u32 = cpu.r32(0x217cc60u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6912); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6913(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6913); }
 let mut pc: u32 = 0x20c6913;
 loop { match pc {
 0x20c6913 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217cc48u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20c692e; continue; }
  pc = 0x20c6920; continue;
 }
 0x20c6920 => {
  { cpu.sub16(cpu.r16(0x217cc4cu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x20c6a01; continue; }
  pc = 0x20c692e; continue;
 }
 0x20c692e => {
  { let v: u32 = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20c6ac4u32); }
  if !cpu.zf { pc = 0x20c6948; continue; }
  pc = 0x20c693c; continue;
 }
 0x20c693c => {
  { let v: u32 = 0x20c6ac9u32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c695a; continue;
 }
 0x20c6948 => {
  { cpu.sub32(cpu.ebx, 0x20c6913u32); }
  if !cpu.zf { pc = 0x20c695a; continue; }
  pc = 0x20c6950; continue;
 }
 0x20c6950 => {
  { let v: u32 = 0x20c462au32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c695a; continue;
 }
 0x20c695a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c69a3; continue; }
  pc = 0x20c6966; continue;
 }
 0x20c6966 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c69a3; continue; }
  pc = 0x20c696b; continue;
 }
 0x20c696b => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc4cu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc48u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc58u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc54u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x20c69f0; continue;
 }
 0x20c69a3 => {
  cpu.flat_ds(0x20c69a3); cpu.push(0x20c69a8); f_020c687f(cpu); if cpu.returned_to != 0x20c69a8 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c69d9; continue; }
  pc = 0x20c69ad; continue;
 }
 0x20c69ad => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc4cu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc48u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc58u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc54u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x20c69f0; continue;
 }
 0x20c69d9 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x217cc4cu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc48u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x20c69eb); cpu.push(0x20c69f0); f_020aeebe(cpu); if cpu.returned_to != 0x20c69f0 { return; }
  pc = 0x20c69f0; continue;
 }
 0x20c69f0 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x217cc4cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x217cc48u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6a01; continue;
 }
 0x20c6a01 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6a05); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6ac4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6ac4); }
 let mut pc: u32 = 0x20c6ac4;
 loop { match pc {
 0x20c6ac4 => {
  cpu.push(0x20c6ac9); f_020c6913(cpu); if cpu.returned_to != 0x20c6ac9 { return; }
  f_020c6ac9(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6ac9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6ac9); }
 let mut pc: u32 = 0x20c6ac9;
 loop { match pc {
 0x20c6a06 => {
  { cpu.sub32(cpu.ebx, 0x20c6ac9u32); }
  if !cpu.zf { pc = 0x20c6a18; continue; }
  pc = 0x20c6a0e; continue;
 }
 0x20c6a0e => {
  { let v: u32 = 0x20c462au32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6a18; continue;
 }
 0x20c6a18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c6a61; continue; }
  pc = 0x20c6a24; continue;
 }
 0x20c6a24 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c6a61; continue; }
  pc = 0x20c6a29; continue;
 }
 0x20c6a29 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc52u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc4eu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc5eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc5au32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x20c6aae; continue;
 }
 0x20c6a61 => {
  cpu.flat_ds(0x20c6a61); cpu.push(0x20c6a66); f_020c687f(cpu); if cpu.returned_to != 0x20c6a66 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c6a97; continue; }
  pc = 0x20c6a6b; continue;
 }
 0x20c6a6b => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc52u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc4eu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x217cc5eu32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc5au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x20c6aae; continue;
 }
 0x20c6a97 => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x217cc52u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217cc4eu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x20c6aa9); cpu.push(0x20c6aae); f_020aeebe(cpu); if cpu.returned_to != 0x20c6aae { return; }
  pc = 0x20c6aae; continue;
 }
 0x20c6aae => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x217cc52u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x217cc4eu32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6abf; continue;
 }
 0x20c6abf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6ac3); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c6ac9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217cc4eu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20c6ae0; continue; }
  pc = 0x20c6ad6; continue;
 }
 0x20c6ad6 => {
  { cpu.sub16(cpu.r16(0x217cc52u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x20c6abf; continue; }
  pc = 0x20c6ae0; continue;
 }
 0x20c6ae0 => {
  { let v: u32 = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20c6ac4u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c6a06; continue; }
  pc = 0x20c6af2; continue;
 }
 0x20c6af2 => {
  { let v: u32 = 0x20c6913u32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x20c6a18; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6b01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6b01); }
 let mut pc: u32 = 0x20c6b01;
 loop { match pc {
 0x20c6b01 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217cc48u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20c6c18; continue; }
  pc = 0x20c6b11; continue;
 }
 0x20c6b11 => {
  { cpu.sub16(cpu.r16(0x217cc4cu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x20c6c18; continue; }
  pc = 0x20c6b1f; continue;
 }
 0x20c6b1f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c6b7e; continue; }
  pc = 0x20c6b2b; continue;
 }
 0x20c6b2b => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c6b7e; continue; }
  pc = 0x20c6b30; continue;
 }
 0x20c6b30 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc4cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc48u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x20c6772u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cc54u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x20c6bea; continue;
 }
 0x20c6b7e => {
  cpu.flat_ds(0x20c6b7e); cpu.push(0x20c6b83); f_020c687f(cpu); if cpu.returned_to != 0x20c6b83 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c6bc3; continue; }
  pc = 0x20c6b88; continue;
 }
 0x20c6b88 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc4cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc48u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc58u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x20c6772u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x20c6bea; continue;
 }
 0x20c6bc3 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x20c6772u32; cpu.ebx = v; }
  cpu.flat_ds(0x20c6bcd); cpu.push(0x20c6bd2); f_020aee8c(cpu); if cpu.returned_to != 0x20c6bd2 { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cc48u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x217cc4cu32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x20c6be5); cpu.push(0x20c6bea); f_020aeebe(cpu); if cpu.returned_to != 0x20c6bea { return; }
  pc = 0x20c6bea; continue;
 }
 0x20c6bea => {
  { let v: u32 = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20c462au32); }
  if !cpu.zf { pc = 0x20c6c06; continue; }
  pc = 0x20c6bf8; continue;
 }
 0x20c6bf8 => {
  { let v: u32 = 0x20c6913u32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6c05); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c6c06 => {
  { cpu.sub32(cpu.ebx, 0x20c6ac9u32); }
  if !cpu.zf { pc = 0x20c6c18; continue; }
  pc = 0x20c6c0e; continue;
 }
 0x20c6c0e => {
  { let v: u32 = 0x20c6ac4u32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6c18; continue;
 }
 0x20c6c18 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6c1b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6c1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6c1c); }
 let mut pc: u32 = 0x20c6c1c;
 loop { match pc {
 0x20c6c1c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217cc4eu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x20c6d33; continue; }
  pc = 0x20c6c2c; continue;
 }
 0x20c6c2c => {
  { cpu.sub16(cpu.r16(0x217cc52u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x20c6d33; continue; }
  pc = 0x20c6c3a; continue;
 }
 0x20c6c3a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c6c99; continue; }
  pc = 0x20c6c46; continue;
 }
 0x20c6c46 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20c6c99; continue; }
  pc = 0x20c6c4b; continue;
 }
 0x20c6c4b => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc52u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc4eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x20c6807u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1bu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc5eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cc5au32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x20c6d05; continue;
 }
 0x20c6c99 => {
  cpu.flat_ds(0x20c6c99); cpu.push(0x20c6c9e); f_020c687f(cpu); if cpu.returned_to != 0x20c6c9e { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c6cde; continue; }
  pc = 0x20c6ca3; continue;
 }
 0x20c6ca3 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc52u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc4eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cc5eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc5au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x20c6807u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x20c6d05; continue;
 }
 0x20c6cde => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u32 = 0x20c6807u32; cpu.ebx = v; }
  cpu.flat_ds(0x20c6ce8); cpu.push(0x20c6ced); f_020aee8c(cpu); if cpu.returned_to != 0x20c6ced { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217cc4eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x217cc52u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x20c6d00); cpu.push(0x20c6d05); f_020aeebe(cpu); if cpu.returned_to != 0x20c6d05 { return; }
  pc = 0x20c6d05; continue;
 }
 0x20c6d05 => {
  { let v: u32 = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20c462au32); }
  if !cpu.zf { pc = 0x20c6d21; continue; }
  pc = 0x20c6d13; continue;
 }
 0x20c6d13 => {
  { let v: u32 = 0x20c6ac9u32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6d20); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c6d21 => {
  { cpu.sub32(cpu.ebx, 0x20c6913u32); }
  if !cpu.zf { pc = 0x20c6d33; continue; }
  pc = 0x20c6d29; continue;
 }
 0x20c6d29 => {
  { let v: u32 = 0x20c6ac4u32; cpu.w32(0x217cb74u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c6d33; continue;
 }
 0x20c6d33 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x20c6d36); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c6e0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6e0f); }
 let mut pc: u32 = 0x20c6e0f;
 loop { match pc {
 0x20c6e0f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x217b924u32)); }
  if cpu.cf { pc = 0x20c6e7a; continue; }
  pc = 0x20c6e1f; continue;
 }
 0x20c6e1f => {
  { cpu.sub8(cpu.r8(0x217b93eu32), 0x0u8); }
  if !cpu.zf { pc = 0x20c6e8c; continue; }
  pc = 0x20c6e28; continue;
 }
 0x20c6e28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xfu32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x20c6e3a; continue; }
  pc = 0x20c6e35; continue;
 }
 0x20c6e35 => {
  { let v: u32 = 0xfffffffu32; cpu.ebp = v; }
  pc = 0x20c6e3a; continue;
 }
 0x20c6e3a => {
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
  if (cpu.sf == cpu.of) { pc = 0x20c6e67; continue; }
  pc = 0x20c6e57; continue;
 }
 0x20c6e57 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.edi; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x20c6e7a; continue; }
  pc = 0x20c6e67; continue;
 }
 0x20c6e67 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x20c6e6c; continue;
 }
 0x20c6e6c => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20c6ec1; continue; }
  pc = 0x20c6e7a; continue;
 }
 0x20c6e7a => {
  cpu.push(0x20c6e7f); f_020b17b7(cpu); if cpu.returned_to != 0x20c6e7f { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20c6ecf; continue;
 }
 0x20c6e8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20c6ea1; continue; }
  pc = 0x20c6e9c; continue;
 }
 0x20c6e9c => {
  { let v: u32 = 0xfffffu32; cpu.ebx = v; }
  pc = 0x20c6ea1; continue;
 }
 0x20c6ea1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c6ebc; continue; }
  pc = 0x20c6ead; continue;
 }
 0x20c6ead => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b93fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20c6ebc; continue; }
  pc = 0x20c6eb9; continue;
 }
 0x20c6eb9 => {
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  pc = 0x20c6ebc; continue;
 }
 0x20c6ebc => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20c6e6c; continue;
 }
 0x20c6ec1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217b910u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b910u32, v); }
  pc = 0x20c6ecf; continue;
 }
 0x20c6ecf => {
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
pub fn f_020c6eda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6eda); }
 let mut pc: u32 = 0x20c6eda;
 loop { match pc {
 0x20c6eda => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.sub32(cpu.r32(0x217cb7cu32), 0x0u32); }
  if !cpu.zf { pc = 0x20c6fa5; continue; }
  pc = 0x20c6eef; continue;
 }
 0x20c6eef => {
  { let v = cpu.r16(0x217b945u32); cpu.set_segment(4, v as u16); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217b941u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20c6f05; continue;
 }
 0x20c6f05 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]))); }
  if cpu.zf { pc = 0x20c6f1d; continue; }
  pc = 0x20c6f0a; continue;
 }
 0x20c6f0a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x20c6f18; continue; }
  pc = 0x20c6f14; continue;
 }
 0x20c6f14 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x20c6f0a; continue;
 }
 0x20c6f18 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x20c6f05; continue;
 }
 0x20c6f1d => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x20c6f26; continue; }
  pc = 0x20c6f21; continue;
 }
 0x20c6f21 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c6f26; continue;
 }
 0x20c6f26 => {
  cpu.push(0x20c6f2b); f_020bd474(cpu); if cpu.returned_to != 0x20c6f2b { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c6fa5; continue; }
  pc = 0x20c6f37; continue;
 }
 0x20c6f37 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x20c6f46); f_020bd474(cpu); if cpu.returned_to != 0x20c6f46 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c6f9e; continue; }
  pc = 0x20c6f4a; continue;
 }
 0x20c6f4a => {
  { let v: u32 = cpu.eax; cpu.w32(0x217cb7cu32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20c6f59; continue;
 }
 0x20c6f59 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u8); }
  if cpu.zf { pc = 0x20c6f79; continue; }
  pc = 0x20c6f5f; continue;
 }
 0x20c6f5f => {
  { let v: u32 = cpu.r32(0x217cb7cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x20c6f68; continue;
 }
 0x20c6f68 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20c6f68; continue; }
  pc = 0x20c6f73; continue;
 }
 0x20c6f73 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20c6f59; continue;
 }
 0x20c6f79 => {
  { let v: u32 = cpu.r32(0x217cb7cu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217cb80u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x20c6f9c); f_020aedc0(cpu); if cpu.returned_to != 0x20c6f9c { return; }
  pc = 0x20c6fa5; continue;
 }
 0x20c6f9e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c6fa5); f_020bd557(cpu); if cpu.returned_to != 0x20c6fa5 { return; }
  pc = 0x20c6fa5; continue;
 }
 0x20c6fa5 => {
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
pub fn f_020c6fae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6fae); }
 let mut pc: u32 = 0x20c6fae;
 loop { match pc {
 0x20c6fae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20c6fe2; continue; }
  pc = 0x20c6fc0; continue;
 }
 0x20c6fc0 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x20c6fc5; continue;
 }
 0x20c6fc5 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c6fce); f_020c2203(cpu); if cpu.returned_to != 0x20c6fce { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c6fde; continue; }
  pc = 0x20c6fd7; continue;
 }
 0x20c6fd7 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c6fde; continue; }
  pc = 0x20c6fdb; continue;
 }
 0x20c6fdb => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x20c6fc5; continue;
 }
 0x20c6fde => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20c6fe4; continue;
 }
 0x20c6fe2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c6fe4; continue;
 }
 0x20c6fe4 => {
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
pub fn f_020c6fec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c6fec); }
 let mut pc: u32 = 0x20c6fec;
 loop { match pc {
 0x20c6fec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20c7015; continue; }
  pc = 0x20c6ffe; continue;
 }
 0x20c6ffe => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20c7011; continue; }
  pc = 0x20c7004; continue;
 }
 0x20c7004 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  cpu.push(0x20c700b); f_020c2203(cpu); if cpu.returned_to != 0x20c700b { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c6ffe; continue; }
  pc = 0x20c7011; continue;
 }
 0x20c7011 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x20c7017; continue;
 }
 0x20c7015 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x20c7017; continue;
 }
 0x20c7017 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c701d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c701d); }
 let mut pc: u32 = 0x20c701d;
 loop { match pc {
 0x20c701d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20c7032; continue; }
  pc = 0x20c7029; continue;
 }
 0x20c7029 => {
  cpu.push(0x20c702e); f_020c6fec(cpu); if cpu.returned_to != 0x20c702e { return; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7032 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7189(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7189); }
 let mut pc: u32 = 0x20c7189;
 loop { match pc {
 0x20c7189 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20c7268; continue; }
  pc = 0x20c71a6; continue;
 }
 0x20c71a6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20c722d; continue; }
  pc = 0x20c71af; continue;
 }
 0x20c71af => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c71b3; continue;
 }
 0x20c71b3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20c71cb; continue; }
  pc = 0x20c71b9; continue;
 }
 0x20c71b9 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c71c9; continue; }
  pc = 0x20c71bd; continue;
 }
 0x20c71bd => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20c71cb; continue; }
  pc = 0x20c71c4; continue;
 }
 0x20c71c4 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c71b3; continue; }
  pc = 0x20c71c9; continue;
 }
 0x20c71c9 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x20c71cb; continue;
 }
 0x20c71cb => {
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.esi = v; }
  pc = 0x20c71dc; continue;
 }
 0x20c71dc => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x20c7226; continue; }
  pc = 0x20c71e1; continue;
 }
 0x20c71e1 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x20c7206; continue; }
  pc = 0x20c71e5; continue;
 }
 0x20c71e5 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20c71fe; continue; }
  pc = 0x20c71e9; continue;
 }
 0x20c71e9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x20c7200; continue;
 }
 0x20c71fe => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x20c7200; continue;
 }
 0x20c7200 => {
  if cpu.zf { pc = 0x20c7206; continue; }
  pc = 0x20c7202; continue;
 }
 0x20c7202 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x20c7226; continue;
 }
 0x20c7206 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c721f; continue; }
  pc = 0x20c720a; continue;
 }
 0x20c720a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x20c7221; continue;
 }
 0x20c721f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20c7221; continue;
 }
 0x20c7221 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.poll(); pc = 0x20c71dc; continue;
 }
 0x20c7226 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x20c722d; continue; }
  pc = 0x20c722a; continue;
 }
 0x20c722a => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ebp = v; }
  pc = 0x20c722d; continue;
 }
 0x20c722d => {
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
  pc = 0x20c7271; continue;
 }
 0x20c7268 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x20c7271); f_020c7d7d(cpu); if cpu.returned_to != 0x20c7271 { return; }
  pc = 0x20c7271; continue;
 }
 0x20c7271 => {
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
pub fn f_020c727b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c727b); }
 let mut pc: u32 = 0x20c727b;
 loop { match pc {
 0x20c727b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c728a); f_020c6fec(cpu); if cpu.returned_to != 0x20c728a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c7295); f_020c729a(cpu); if cpu.returned_to != 0x20c7295 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c729a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c729a); }
 let mut pc: u32 = 0x20c729a;
 loop { match pc {
 0x20c7182 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c729a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20c7344; continue; }
  pc = 0x20c72ad; continue;
 }
 0x20c72ad => {
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
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c72d8; continue; }
  pc = 0x20c72c9; continue;
 }
 0x20c72c9 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.esi = v; }
  pc = 0x20c72d8; continue;
 }
 0x20c72d8 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c72e1; continue; }
  pc = 0x20c72dc; continue;
 }
 0x20c72dc => {
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x20c72e1; continue; }
  pc = 0x20c72e0; continue;
 }
 0x20c72e0 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x20c72e1; continue;
 }
 0x20c72e1 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20c7309; continue; }
  pc = 0x20c72e8; continue;
 }
 0x20c72e8 => {
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
  cpu.poll(); pc = 0x20c7182; continue;
 }
 0x20c7309 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c730e; continue;
 }
 0x20c730e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c7326; continue; }
  pc = 0x20c7316; continue;
 }
 0x20c7316 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c730e; continue; }
  pc = 0x20c7326; continue;
 }
 0x20c7326 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if cpu.zf { cpu.poll(); pc = 0x20c7182; continue; }
  pc = 0x20c732f; continue;
 }
 0x20c732f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  cpu.push(0x20c733f); f_020aedc0(cpu); if cpu.returned_to != 0x20c733f { return; }
  cpu.poll(); pc = 0x20c7182; continue;
 }
 0x20c7344 => {
  cpu.push(0x20c7349); f_0209ebc4(cpu); if cpu.returned_to != 0x20c7349 { return; }
  cpu.poll(); pc = 0x20c7182; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c734e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c734e); }
 let mut pc: u32 = 0x20c734e;
 loop { match pc {
 0x20c734e => {
  { let v = 0xb8u32; cpu.push(v); }
  cpu.push(0x20c7358); f_020ae38a(cpu); if cpu.returned_to != 0x20c7358 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(164); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20c7372); f_020bd81f(cpu); if cpu.returned_to != 0x20c7372 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c737d; continue;
 }
 0x20c737d => {
  cpu.push(0x20c7382); f_020b17b7(cpu); if cpu.returned_to != 0x20c7382 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x20c7395; continue; }
  pc = 0x20c7387; continue;
 }
 0x20c7387 => {
  cpu.push(0x20c738c); f_020b17b7(cpu); if cpu.returned_to != 0x20c738c { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c7395; continue;
 }
 0x20c7395 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c73a1; continue;
 }
 0x20c73a1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c73aa; continue;
 }
 0x20c73aa => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c73b7; continue;
 }
 0x20c73b7 => {
  { let v: u32 = 0x216b570u32; cpu.eax = v; }
  cpu.push(0x20c73c1); f_020bd5d9(cpu); if cpu.returned_to != 0x20c73c1 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c73cb; continue;
 }
 0x20c73cb => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c73d2); f_020a4a64(cpu); if cpu.returned_to != 0x20c73d2 { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x90u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c73e1; continue;
 }
 0x20c73e1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c73ea; continue;
 }
 0x20c73ea => {
  { let v: u32 = 0x3bu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c73f6); f_020a4a7d(cpu); if cpu.returned_to != 0x20c73f6 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c7406; continue; }
  pc = 0x20c73fc; continue;
 }
 0x20c73fc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c7403); f_020a4a64(cpu); if cpu.returned_to != 0x20c7403 { return; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edi = v; }
  pc = 0x20c7406; continue;
 }
 0x20c7406 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x6eu32.wrapping_add(cpu.ebp))); }
  if (cpu.cf || cpu.zf) { pc = 0x20c742d; continue; }
  pc = 0x20c7412; continue;
 }
 0x20c7412 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x20c741c); f_020af0db(cpu); if cpu.returned_to != 0x20c741c { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  cpu.push(0x20c7426); f_020af0fe(cpu); if cpu.returned_to != 0x20c7426 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20c7494; continue;
 }
 0x20c742d => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x20c7439); f_020aee67(cpu); if cpu.returned_to != 0x20c7439 { return; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x7eu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x5cu32); }
  if cpu.zf { pc = 0x20c744c; continue; }
  pc = 0x20c7448; continue;
 }
 0x20c7448 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20c744c; continue;
 }
 0x20c744c => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x20c7459); f_020aee67(cpu); if cpu.returned_to != 0x20c7459 { return; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffdeu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffdau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x20c7467); f_020bd81f(cpu); if cpu.returned_to != 0x20c7467 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c746e; continue;
 }
 0x20c746e => {
  cpu.push(0x20c7473); f_020b17b7(cpu); if cpu.returned_to != 0x20c7473 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x20c7482; continue; }
  pc = 0x20c7478; continue;
 }
 0x20c7478 => {
  cpu.push(0x20c747d); f_020b17b7(cpu); if cpu.returned_to != 0x20c747d { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x9u32); }
  if !cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c7482; continue;
 }
 0x20c7482 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.edi)) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3bu32); }
  if !cpu.zf { pc = 0x20c7492; continue; }
  pc = 0x20c748a; continue;
 }
 0x20c748a => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.ecx = v; }
  cpu.poll(); pc = 0x20c73e1; continue;
 }
 0x20c7492 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c7494; continue;
 }
 0x20c7494 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c749e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c749e); }
 let mut pc: u32 = 0x20c749e;
 loop { match pc {
 0x20c749e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d16ecu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20c74c6; continue; }
  pc = 0x20c74ad; continue;
 }
 0x20c74ad => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0x4003u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16ecu32, v); }
  { let v: u16 = (cpu.edi as u16) | 0x3u16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x20c750d; continue;
 }
 0x20c74c6 => {
  { let v: u32 = 0x217b104u32; cpu.ecx = v; }
  pc = 0x20c74ed; continue;
 }
 0x20c74cd => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x3u8); }
  if !cpu.zf { pc = 0x20c74ea; continue; }
  pc = 0x20c74d3; continue;
 }
 0x20c74d3 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x20c74dd); f_020bd474(cpu); if cpu.returned_to != 0x20c74dd { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7532; continue; }
  pc = 0x20c74e3; continue;
 }
 0x20c74e3 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x20c750d; continue;
 }
 0x20c74ea => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  pc = 0x20c74ed; continue;
 }
 0x20c74ed => {
  { cpu.sub32(cpu.ecx, 0x217b30cu32); }
  if cpu.cf { cpu.poll(); pc = 0x20c74cd; continue; }
  pc = 0x20c74f5; continue;
 }
 0x20c74f5 => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u32 = 0x4003u32; cpu.edi = v; }
  cpu.push(0x20c7504); f_020bd474(cpu); if cpu.returned_to != 0x20c7504 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7532; continue; }
  pc = 0x20c750a; continue;
 }
 0x20c750a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x20c750d; continue;
 }
 0x20c750d => {
  { let v: u32 = 0x1au32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x20c751b); f_020aedc0(cpu); if cpu.returned_to != 0x20c751b { return; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x22d22d0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x22d22d0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20c753e; continue;
 }
 0x20c7532 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x20c753c); f_020af0db(cpu); if cpu.returned_to != 0x20c753c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20c753e; continue;
 }
 0x20c753e => {
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
pub fn f_020c7544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7544); }
 let mut pc: u32 = 0x20c7544;
 loop { match pc {
 0x20c7544 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x22d22d0u32; cpu.edx = v; }
  pc = 0x20c754e; continue;
 }
 0x20c754e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7579; continue; }
  pc = 0x20c7554; continue;
 }
 0x20c7554 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x20c755f; continue; }
  pc = 0x20c755b; continue;
 }
 0x20c755b => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x20c754e; continue;
 }
 0x20c755f => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x22d16ecu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16ecu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c7579; continue;
 }
 0x20c7579 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c759b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c759b); }
 let mut pc: u32 = 0x20c759b;
 loop { match pc {
 0x20c759b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d22d0u32); cpu.eax = v; }
  pc = 0x20c75a4; continue;
 }
 0x20c75a4 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c75b0; continue; }
  pc = 0x20c75a8; continue;
 }
 0x20c75a8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c75b0 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x20c75b9; continue; }
  pc = 0x20c75b5; continue;
 }
 0x20c75b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x20c75a4; continue;
 }
 0x20c75b9 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c75c5); f_020c75c8(cpu); if cpu.returned_to != 0x20c75c5 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c75c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c75c8); }
 let mut pc: u32 = 0x20c75c8;
 loop { match pc {
 0x20c75c8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x20c75d0); f_020c764e(cpu); if cpu.returned_to != 0x20c75d0 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c75d9); f_020c7544(cpu); if cpu.returned_to != 0x20c75d9 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c75dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c75dd); }
 let mut pc: u32 = 0x20c75dd;
 loop { match pc {
 0x20c75dd => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c75e8; continue; }
  pc = 0x20c75e5; continue;
 }
 0x20c75e5 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x20c75e8; continue;
 }
 0x20c75e8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c75e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c75e9); }
 let mut pc: u32 = 0x20c75e9;
 loop { match pc {
 0x20c75e9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x20c75f6); f_020c7dbc(cpu); if cpu.returned_to != 0x20c75f6 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x20c7600; continue;
 }
 0x20c7600 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x20c760a); f_020c75dd(cpu); if cpu.returned_to != 0x20c760a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x20c7600; continue; }
  pc = 0x20c7614; continue;
 }
 0x20c7614 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x20c7625); f_020c75dd(cpu); if cpu.returned_to != 0x20c7625 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x20c7632); f_020c75dd(cpu); if cpu.returned_to != 0x20c7632 { return; }
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
pub fn f_020c764e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c764e); }
 let mut pc: u32 = 0x20c764e;
 loop { match pc {
 0x20c764e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x20c766a; continue; }
  pc = 0x20c7660; continue;
 }
 0x20c7660 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20c76d5; continue;
 }
 0x20c766a => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x20c767d; continue; }
  pc = 0x20c7674; continue;
 }
 0x20c7674 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c767b); f_020c1ed6(cpu); if cpu.returned_to != 0x20c767b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20c767d; continue;
 }
 0x20c767d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c7684); f_020c7dc2(cpu); if cpu.returned_to != 0x20c7684 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20c7697; continue; }
  pc = 0x20c7689; continue;
 }
 0x20c7689 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20c7697); f_020b1ab3(cpu); if cpu.returned_to != 0x20c7697 { return; }
  pc = 0x20c7697; continue;
 }
 0x20c7697 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c76a5; continue; }
  pc = 0x20c769b; continue;
 }
 0x20c769b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20c76a3); f_020c1d5c(cpu); if cpu.returned_to != 0x20c76a3 { return; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x20c76a5; continue;
 }
 0x20c76a5 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x20c76ba; continue; }
  pc = 0x20c76ab; continue;
 }
 0x20c76ab => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20c76b3); f_020bd557(cpu); if cpu.returned_to != 0x20c76b3 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c76ba; continue;
 }
 0x20c76ba => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x20c76d3; continue; }
  pc = 0x20c76c0; continue;
 }
 0x20c76c0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x20c76cc); f_020c75e9(cpu); if cpu.returned_to != 0x20c76cc { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x20c76d3); f_020b1af6(cpu); if cpu.returned_to != 0x20c76d3 { return; }
  pc = 0x20c76d3; continue;
 }
 0x20c76d3 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20c76d5; continue;
 }
 0x20c76d5 => {
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
pub fn f_020c76de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c76de); }
 let mut pc: u32 = 0x20c76de;
 loop { match pc {
 0x20c76de => {
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
  if !cpu.zf { pc = 0x20c7766; continue; }
  pc = 0x20c770b; continue;
 }
 0x20c770b => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20c7724; continue; }
  pc = 0x20c771e; continue;
 }
 0x20c771e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c7724; continue;
 }
 0x20c7724 => {
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
  if !cpu.zf { pc = 0x20c7757; continue; }
  pc = 0x20c774f; continue;
 }
 0x20c774f => {
  { let v: u32 = cpu.r32(0x216b578u32); cpu.edx = v; }
  pc = 0x20c775d; continue;
 }
 0x20c7757 => {
  { let v: u32 = cpu.r32(0x216b57cu32); cpu.edx = v; }
  pc = 0x20c775d; continue;
 }
 0x20c775d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20c7768; continue;
 }
 0x20c7766 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c7768; continue;
 }
 0x20c7768 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7770(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c7770); } r_020c7770(cpu, 0x20c7770); }
pub fn f_020c77b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c77b7); } r_020c7770(cpu, 0x20c77b7); }
fn r_020c7770(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c7770 => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c7772; continue;
 }
 0x20c7772 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c7797; continue; }
  pc = 0x20c7785; continue;
 }
 0x20c7785 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20c7793); f_020c7e04(cpu); if cpu.returned_to != 0x20c7793 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c77af; continue;
 }
 0x20c7797 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if !cpu.zf { pc = 0x20c779f; continue; }
  pc = 0x20c779b; continue;
 }
 0x20c779b => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  pc = 0x20c77a9; continue;
 }
 0x20c779f => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x20c77a7; continue; }
  pc = 0x20c77a3; continue;
 }
 0x20c77a3 => {
  { cpu.fpush(1.0); }
  pc = 0x20c77a9; continue;
 }
 0x20c77a7 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x20c77a9; continue;
 }
 0x20c77a9 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c77af; continue;
 }
 0x20c77af => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c77b7 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x20c7772; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c77bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c77bb); }
 let mut pc: u32 = 0x20c77bb;
 loop { match pc {
 0x20c77bb => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x20c77c4); f_020c7770(cpu); if cpu.returned_to != 0x20c77c4 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c77df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c77df); }
 let mut pc: u32 = 0x20c77df;
 loop { match pc {
 0x20c77df => {
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
  cpu.push(0x20c77f9); f_020c7e40(cpu); if cpu.returned_to != 0x20c77f9 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x20c780e; continue; }
  pc = 0x20c7802; continue;
 }
 0x20c7802 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x216b580u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20c780e; continue;
 }
 0x20c780e => {
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
pub fn f_020c781b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c781b); }
 let mut pc: u32 = 0x20c781b;
 loop { match pc {
 0x20c781b => {
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
  cpu.push(0x20c784c); f_020c76de(cpu); if cpu.returned_to != 0x20c784c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7857; continue; }
  pc = 0x20c7850; continue;
 }
 0x20c7850 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c79ec; continue;
 }
 0x20c7857 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x20c7881; continue; }
  pc = 0x20c7879; continue;
 }
 0x20c7879 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c79a9; continue; }
  pc = 0x20c7881; continue;
 }
 0x20c7881 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x20c7899; continue; }
  pc = 0x20c788b; continue;
 }
 0x20c788b => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20c7899; continue;
 }
 0x20c7899 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20c78a7); f_020c7e59(cpu); if cpu.returned_to != 0x20c78a7 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20c78b1; continue; }
  pc = 0x20c78af; continue;
 }
 0x20c78af => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x20c78b1; continue;
 }
 0x20c78b1 => {
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
  if (cpu.sf == cpu.of) { pc = 0x20c78d1; continue; }
  pc = 0x20c78cf; continue;
 }
 0x20c78cf => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x20c78d1; continue;
 }
 0x20c78d1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if !cpu.zf { pc = 0x20c78dd; continue; }
  pc = 0x20c78db; continue;
 }
 0x20c78db => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x20c78dd; continue;
 }
 0x20c78dd => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x20c7977; continue; }
  pc = 0x20c78e5; continue;
 }
 0x20c78e5 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c78ef; continue; }
  pc = 0x20c78ea; continue;
 }
 0x20c78ea => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x20c78ef; continue;
 }
 0x20c78ef => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20c7905); f_020c7e40(cpu); if cpu.returned_to != 0x20c7905 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x20c7919; continue; }
  pc = 0x20c790e; continue;
 }
 0x20c790e => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c7919; continue; }
  pc = 0x20c7912; continue;
 }
 0x20c7912 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20c7919; continue;
 }
 0x20c7919 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20c792d); f_020af10a(cpu); if cpu.returned_to != 0x20c792d { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7977; continue; }
  pc = 0x20c7933; continue;
 }
 0x20c7933 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.zf { pc = 0x20c7941; continue; }
  pc = 0x20c793b; continue;
 }
 0x20c793b => {
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x20c7943; continue; }
  pc = 0x20c7941; continue;
 }
 0x20c7941 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  pc = 0x20c7943; continue;
 }
 0x20c7943 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20c794f; continue; }
  pc = 0x20c7948; continue;
 }
 0x20c7948 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x20c7959; continue;
 }
 0x20c794f => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c7959; continue; }
  pc = 0x20c7954; continue;
 }
 0x20c7954 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x20c7959; continue;
 }
 0x20c7959 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x20c796d); f_020af10a(cpu); if cpu.returned_to != 0x20c796d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c7977; continue; }
  pc = 0x20c7971; continue;
 }
 0x20c7971 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20c7977; continue;
 }
 0x20c7977 => {
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
  if !cpu.zf { pc = 0x20c79a9; continue; }
  pc = 0x20c799d; continue;
 }
 0x20c799d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if !cpu.zf { pc = 0x20c79a9; continue; }
  pc = 0x20c79a7; continue;
 }
 0x20c79a7 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  pc = 0x20c79a9; continue;
 }
 0x20c79a9 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20c79b5; continue; }
  pc = 0x20c79ae; continue;
 }
 0x20c79ae => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x20c79bf; continue;
 }
 0x20c79b5 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c79bf; continue; }
  pc = 0x20c79ba; continue;
 }
 0x20c79ba => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x20c79bf; continue;
 }
 0x20c79bf => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x20c79d0); f_020bdfcb(cpu); if cpu.returned_to != 0x20c79d0 { return; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20c79d1; continue;
 }
 0x20c79d1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c79e9; continue; }
  pc = 0x20c79d9; continue;
 }
 0x20c79d9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20c79d1; continue; }
  pc = 0x20c79e9; continue;
 }
 0x20c79e9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20c79ec; continue;
 }
 0x20c79ec => {
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
pub fn f_020c79f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c79f8); }
 let mut pc: u32 = 0x20c79f8;
 loop { match pc {
 0x20c79f8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x43u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x20c7a15; continue; }
  pc = 0x20c7a06; continue;
 }
 0x20c7a06 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x20c7a15; continue; }
  pc = 0x20c7a0b; continue;
 }
 0x20c7a0b => {
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c7a15; continue; }
  pc = 0x20c7a10; continue;
 }
 0x20c7a10 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = true; }
  pc = 0x20c7a15; continue;
 }
 0x20c7a15 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c7a30); f_020c1c4f(cpu); if cpu.returned_to != 0x20c7a30 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7a33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7a33); }
 let mut pc: u32 = 0x20c7a33;
 loop { match pc {
 0x20c7a33 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.push(0x20c7a45); f_020b17b7(cpu); if cpu.returned_to != 0x20c7a45 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20c7a53); f_020c79f8(cpu); if cpu.returned_to != 0x20c7a53 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c7aa7; continue; }
  pc = 0x20c7a57; continue;
 }
 0x20c7a57 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x5cu32); }
  if cpu.zf { pc = 0x20c7a9b; continue; }
  pc = 0x20c7a61; continue;
 }
 0x20c7a61 => {
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x20c7a9b; continue; }
  pc = 0x20c7a66; continue;
 }
 0x20c7a66 => {
  { cpu.sub32(cpu.ebx, 0x2fu32); }
  if cpu.zf { pc = 0x20c7a9b; continue; }
  pc = 0x20c7a6b; continue;
 }
 0x20c7a6b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x20c7a79; continue; }
  pc = 0x20c7a70; continue;
 }
 0x20c7a70 => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if cpu.zf { pc = 0x20c7a9b; continue; }
  pc = 0x20c7a79; continue;
 }
 0x20c7a79 => {
  { let v: u32 = 0x90u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c7a85); f_020b1afb(cpu); if cpu.returned_to != 0x20c7a85 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20c7a8c); f_020a4a64(cpu); if cpu.returned_to != 0x20c7a8c { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x5cu32); }
  if cpu.zf { pc = 0x20c7a9b; continue; }
  pc = 0x20c7a97; continue;
 }
 0x20c7a97 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20c7a9b; continue;
 }
 0x20c7a9b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x20c7aa2); f_020a3df1(cpu); if cpu.returned_to != 0x20c7aa2 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7aa7 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20c7aae); f_020bd5d9(cpu); if cpu.returned_to != 0x20c7aae { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7b1f; continue; }
  pc = 0x20c7ab8; continue;
 }
 0x20c7ab8 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x20c7b1f; continue; }
  pc = 0x20c7ac1; continue;
 }
 0x20c7ac1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20c7ac3; continue;
 }
 0x20c7ac3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x20c7ad8; continue; }
  pc = 0x20c7ac8; continue;
 }
 0x20c7ac8 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3bu32); }
  if cpu.zf { pc = 0x20c7ad8; continue; }
  pc = 0x20c7ad0; continue;
 }
 0x20c7ad0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20c7ac3; continue;
 }
 0x20c7ad8 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x20c7b17; continue; }
  pc = 0x20c7adc; continue;
 }
 0x20c7adc => {
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x5cu32); }
  if cpu.zf { pc = 0x20c7af3; continue; }
  pc = 0x20c7ae5; continue;
 }
 0x20c7ae5 => {
  { cpu.sub32(cpu.edi, 0x2fu32); }
  if cpu.zf { pc = 0x20c7af3; continue; }
  pc = 0x20c7aea; continue;
 }
 0x20c7aea => {
  { cpu.sub32(cpu.edi, 0x3au32); }
  if cpu.zf { pc = 0x20c7af3; continue; }
  pc = 0x20c7aef; continue;
 }
 0x20c7aef => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20c7af3; continue;
 }
 0x20c7af3 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20c7afd); f_020a3e10(cpu); if cpu.returned_to != 0x20c7afd { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x20c7b06); f_020c79f8(cpu); if cpu.returned_to != 0x20c7b06 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20c7b17; continue; }
  pc = 0x20c7b0a; continue;
 }
 0x20c7b0a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x20c7b12); f_020af0db(cpu); if cpu.returned_to != 0x20c7b12 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7b17 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x20c7b1f; continue; }
  pc = 0x20c7b1c; continue;
 }
 0x20c7b1c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20c7ab8; continue;
 }
 0x20c7b1f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7b28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7b28); }
 let mut pc: u32 = 0x20c7b28;
 loop { match pc {
 0x20c7b28 => {
  pc = 0x20c7ec8; continue;
 }
 0x20c7ec8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7b2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7b2d); }
 let mut pc: u32 = 0x20c7b2d;
 loop { match pc {
 0x20c7b2d => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20c7b37; continue; }
  pc = 0x20c7b33; continue;
 }
 0x20c7b33 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7b37 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x217b104u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7b4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7b4c); }
 let mut pc: u32 = 0x20c7b4c;
 loop { match pc {
 0x20c7b4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x20c7b62; continue; }
  pc = 0x20c7b5b; continue;
 }
 0x20c7b5b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c7b62); f_020c1e5f(cpu); if cpu.returned_to != 0x20c7b62 { return; }
  pc = 0x20c7b62; continue;
 }
 0x20c7b62 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c7b7e; continue; }
  pc = 0x20c7b6c; continue;
 }
 0x20c7b6c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20c7b7e; continue;
 }
 0x20c7b7e => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20c7b82; continue;
 }
 0x20c7b82 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20c7b99; continue; }
  pc = 0x20c7b8a; continue;
 }
 0x20c7b8a => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x20c7b92); f_020c1dbb(cpu); if cpu.returned_to != 0x20c7b92 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x20c7b82; continue; }
  pc = 0x20c7b97; continue;
 }
 0x20c7b97 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20c7b99; continue;
 }
 0x20c7b99 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20c7bb7; continue; }
  pc = 0x20c7b9d; continue;
 }
 0x20c7b9d => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20c7bb7; continue; }
  pc = 0x20c7bae; continue;
 }
 0x20c7bae => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20c7bb5); f_020c1ed6(cpu); if cpu.returned_to != 0x20c7bb5 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x20c7bb7; continue;
 }
 0x20c7bb7 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20c7bbf; continue; }
  pc = 0x20c7bbb; continue;
 }
 0x20c7bbb => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x20c7bbf; continue;
 }
 0x20c7bbf => {
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
pub fn f_020c7bc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7bc7); }
 let mut pc: u32 = 0x20c7bc7;
 loop { match pc {
 0x20c7bc7 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_020c7cbb(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7bd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7bd1); }
 let mut pc: u32 = 0x20c7bd1;
 loop { match pc {
 0x20c7bd1 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x217cc70u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20c7c00; continue; }
  pc = 0x20c7be4; continue;
 }
 0x20c7be4 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x20c7c00; continue; }
  pc = 0x20c7be9; continue;
 }
 0x20c7be9 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x20c7c00; continue; }
  pc = 0x20c7bee; continue;
 }
 0x20c7bee => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217cc70u32, v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x20c7c00); cpu.call(t); if cpu.returned_to != 0x20c7c00 { return; }
  pc = 0x20c7c00; continue;
 }
 0x20c7c00 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7c06(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7c06); }
 let mut pc: u32 = 0x20c7c06;
 loop { match pc {
 0x20c7c06 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x20c7c13; continue; }
  pc = 0x20c7c0e; continue;
 }
 0x20c7c0e => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20c7c24; continue; }
  pc = 0x20c7c13; continue;
 }
 0x20c7c13 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x20c7c1d); f_020af0db(cpu); if cpu.returned_to != 0x20c7c1d { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7c24 => {
  { let v: u32 = 0x20c7bc7u32; cpu.w32(0x217cc9cu32, v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x20c7c4b; continue; }
  pc = 0x20c7c33; continue;
 }
 0x20c7c33 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x20c7c3f; continue; }
  pc = 0x20c7c38; continue;
 }
 0x20c7c38 => {
  cpu.push(0x20c7c3d); f_020c6913(cpu); if cpu.returned_to != 0x20c7c3d { return; }
  pc = 0x20c7c7b; continue;
 }
 0x20c7c3f => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x20c7c7b; continue; }
  pc = 0x20c7c44; continue;
 }
 0x20c7c44 => {
  cpu.push(0x20c7c49); f_020c6b01(cpu); if cpu.returned_to != 0x20c7c49 { return; }
  pc = 0x20c7c7b; continue;
 }
 0x20c7c4b => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if !cpu.zf { pc = 0x20c7c68; continue; }
  pc = 0x20c7c50; continue;
 }
 0x20c7c50 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x20c7c5c; continue; }
  pc = 0x20c7c55; continue;
 }
 0x20c7c55 => {
  cpu.push(0x20c7c5a); f_020c6ac9(cpu); if cpu.returned_to != 0x20c7c5a { return; }
  pc = 0x20c7c7b; continue;
 }
 0x20c7c5c => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x20c7c7b; continue; }
  pc = 0x20c7c61; continue;
 }
 0x20c7c61 => {
  cpu.push(0x20c7c66); f_020c6c1c(cpu); if cpu.returned_to != 0x20c7c66 { return; }
  pc = 0x20c7c7b; continue;
 }
 0x20c7c68 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x20c7c7b; continue; }
  pc = 0x20c7c6d; continue;
 }
 0x20c7c6d => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x20c7c7b; continue; }
  pc = 0x20c7c71; continue;
 }
 0x20c7c71 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x20c7c7b; continue; }
  pc = 0x20c7c76; continue;
 }
 0x20c7c76 => {
  cpu.push(0x20c7c7b); f_020c7ee3(cpu); if cpu.returned_to != 0x20c7c7b { return; }
  pc = 0x20c7c7b; continue;
 }
 0x20c7c7b => {
  { let v: u32 = cpu.r32(0x217cc68u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217cc68u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7c8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c7c8b); } r_020c7c8b(cpu, 0x20c7c8b); }
pub fn f_020c7cbb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c7cbb); } r_020c7c8b(cpu, 0x20c7cbb); }
fn r_020c7c8b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c7c8b => {
  {  }
  if (cpu.sf != cpu.of) { pc = 0x20c7c9a; continue; }
  pc = 0x20c7c8e; continue;
 }
 0x20c7c8e => {
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if (cpu.sf != cpu.of) { pc = 0x20c7c9e; continue; }
  pc = 0x20c7c92; continue;
 }
 0x20c7c92 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.eax, 0x51020c7du32); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x20c7caa; continue; }
  pc = 0x20c7c9e; continue;
 }
 0x20c7c9a => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x20c7cae; continue; }
  pc = 0x20c7ca2; continue;
 }
 0x20c7c9e => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.sf == cpu.of) { pc = 0x20c7cb2; continue; }
  pc = 0x20c7ca6; continue;
 }
 0x20c7ca2 => {
  { let v: u8 = cpu.add8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if (cpu.sf == cpu.of) { pc = 0x20c7cb2; continue; }
  pc = 0x20c7ca6; continue;
 }
 0x20c7ca6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x20c7cba; continue; }
  pc = 0x20c7cae; continue;
 }
 0x20c7caa => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x20c7cbe; continue; }
  pc = 0x20c7cb2; continue;
 }
 0x20c7cae => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x20c7cc2; continue; }
  pc = 0x20c7cb6; continue;
 }
 0x20c7cb2 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  if (cpu.sf == cpu.of) { pc = 0x20c7cc6; continue; }
  pc = 0x20c7cba; continue;
 }
 0x20c7cb6 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x7du32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c7cbb; continue;
 }
 0x20c7cba => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x51u32.wrapping_add(cpu.ebx))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x20c7cbe; continue;
 }
 0x20c7cbb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x20c7cbe; continue;
 }
 0x20c7cbe => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  pc = 0x20c7cc8; continue;
 }
 0x20c7cc2 => {
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x89u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(57541);  return;
 }
 0x20c7cc6 => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x20c7cca; continue; }
  pc = 0x20c7cc8; continue;
 }
 0x20c7cc8 => {
  { let v: u32 = cpu.r32(0x217cc68u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c7d73; continue; }
  pc = 0x20c7cdd; continue;
 }
 0x20c7cca => {
  { let v = 0x890217ccu32; cpu.push(v); }
  { let v: u8 = cpu.sbb8((cpu.eax as u8), 0x24u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20c7d73; continue; }
  pc = 0x20c7cdd; continue;
 }
 0x20c7cdd => {
  cpu.poll(); pc = cpu.r32(0x20c7c8bu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x20c7ce5 => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x20c7cef); f_020c7bd1(cpu); if cpu.returned_to != 0x20c7cef { return; }
  pc = 0x20c7cef; continue;
 }
 0x20c7cef => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20c7cf1; continue;
 }
 0x20c7cf1 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7cfa => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x20c7d05; continue; }
  pc = 0x20c7d00; continue;
 }
 0x20c7d00 => {
  cpu.push(0x20c7d05); f_020c7ed1(cpu); if cpu.returned_to != 0x20c7d05 { return; }
  pc = 0x20c7d05; continue;
 }
 0x20c7d05 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d0d; continue;
 }
 0x20c7d0d => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d12; continue;
 }
 0x20c7d12 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d17; continue;
 }
 0x20c7d17 => {
  { let v: u32 = 0x2u32; cpu.w32(0x217cc68u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.push(0x20c7d27); f_020c6913(cpu); if cpu.returned_to != 0x20c7d27 { return; }
  pc = 0x20c7d27; continue;
 }
 0x20c7d27 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x20c7d29; continue;
 }
 0x20c7d29 => {
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x20c7d2c); cpu.call(t); if cpu.returned_to != 0x20c7d2c { return; }
  cpu.poll(); pc = 0x20c7cef; continue;
 }
 0x20c7d2e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d36; continue;
 }
 0x20c7d36 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d3b; continue;
 }
 0x20c7d3b => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d40; continue;
 }
 0x20c7d40 => {
  { let v: u32 = 0x2u32; cpu.w32(0x217cc68u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20c7d4f); f_020c6ac9(cpu); if cpu.returned_to != 0x20c7d4f { return; }
  cpu.poll(); pc = 0x20c7d27; continue;
 }
 0x20c7d51 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d59; continue;
 }
 0x20c7d59 => {
  { cpu.sub32(cpu.ebp, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d5e; continue;
 }
 0x20c7d5e => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x20c7cef; continue; }
  pc = 0x20c7d63; continue;
 }
 0x20c7d63 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217cc68u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x20c7d29; continue;
 }
 0x20c7d73 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x20c7cf1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7d7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7d7d); }
 let mut pc: u32 = 0x20c7d7d;
 loop { match pc {
 0x20c7d7d => {
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
  if cpu.ecx == 0 { pc = 0x20c7d9c; continue; }
  pc = 0x20c7d91; continue;
 }
 0x20c7d91 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.scas(1, 2); }
  if !cpu.zf { pc = 0x20c7d9c; continue; }
  pc = 0x20c7d99; continue;
 }
 0x20c7d99 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and16((cpu.eax as u16), 0xcf89u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20c7da1; continue;
 }
 0x20c7d9c => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20c7da1; continue;
 }
 0x20c7da1 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20c7db2; continue; }
  pc = 0x20c7da5; continue;
 }
 0x20c7da5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20c7db2; continue; }
  pc = 0x20c7dad; continue;
 }
 0x20c7dad => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20c7da1; continue;
 }
 0x20c7db2 => {
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
pub fn f_020c7dbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7dbc); }
 let mut pc: u32 = 0x20c7dbc;
 loop { match pc {
 0x20c7dbc => {
  { let v: u32 = cpu.r32(0x217b91cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7dc2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7dc2); }
 let mut pc: u32 = 0x20c7dc2;
 loop { match pc {
 0x20c7dc2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x20c7dce); f_020b1bd1(cpu); if cpu.returned_to != 0x20c7dce { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20c7dff; continue; }
  pc = 0x20c7dd5; continue;
 }
 0x20c7dd5 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x20c7de8; continue; }
  pc = 0x20c7ddb; continue;
 }
 0x20c7ddb => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x20c7de8; continue; }
  pc = 0x20c7de1; continue;
 }
 0x20c7de1 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20c7de8); f_020c7ee8(cpu); if cpu.returned_to != 0x20c7de8 { return; }
  pc = 0x20c7de8; continue;
 }
 0x20c7de8 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x20c7dff; continue; }
  pc = 0x20c7dee; continue;
 }
 0x20c7dee => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x20c7dfc; continue; }
  pc = 0x20c7df4; continue;
 }
 0x20c7df4 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7dfc => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x20c7dff; continue;
 }
 0x20c7dff => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7e04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7e04); }
 let mut pc: u32 = 0x20c7e04;
 loop { match pc {
 0x20c7e04 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x20c7e2d; continue; }
  pc = 0x20c7e15; continue;
 }
 0x20c7e15 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x20c7e2d; continue; }
  pc = 0x20c7e23; continue;
 }
 0x20c7e23 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x20c7e2d; continue; }
  pc = 0x20c7e27; continue;
 }
 0x20c7e27 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1080u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20c7e31; continue;
 }
 0x20c7e2d => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1040u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20c7e31; continue;
 }
 0x20c7e31 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x20c7e39); f_020c23e3(cpu); if cpu.returned_to != 0x20c7e39 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7e40); }
 let mut pc: u32 = 0x20c7e40;
 loop { match pc {
 0x20c7e40 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x20c7e4c); f_0209c6d2(cpu); if cpu.returned_to != 0x20c7e4c { return; }
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
pub fn f_020c7e59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7e59); }
 let mut pc: u32 = 0x20c7e59;
 loop { match pc {
 0x20c7e59 => {
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
  if !cpu.zf { pc = 0x20c7e80; continue; }
  pc = 0x20c7e7c; continue;
 }
 0x20c7e7c => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20c7eb5; continue; }
  pc = 0x20c7e80; continue;
 }
 0x20c7e80 => {
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
  pc = 0x20c7eb5; continue;
 }
 0x20c7eb5 => {
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
pub fn f_020c7ed1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7ed1); }
 let mut pc: u32 = 0x20c7ed1;
 loop { match pc {
 0x20c7ed1 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x216b588u32; cpu.eax = v; }
  cpu.push(0x20c7ee1); f_020bd78a(cpu); if cpu.returned_to != 0x20c7ee1 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7ee3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7ee3); }
 let mut pc: u32 = 0x20c7ee3;
 loop { match pc {
 0x20c7ee3 => {
  f_020c7ef8(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7ee8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7ee8); }
 let mut pc: u32 = 0x20c7ee8;
 loop { match pc {
 0x20c7ee8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_020c1ed6(cpu); return; }
  pc = 0x20c7ef0; continue;
 }
 0x20c7ef0 => {
  cpu.push(0x20c7ef5); f_020c7f29(cpu); if cpu.returned_to != 0x20c7ef5 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7ef8); }
 let mut pc: u32 = 0x20c7ef8;
 loop { match pc {
 0x20c7ef8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217cb78u32), 0x20c7fbbu32); }
  if cpu.zf { pc = 0x20c7f21; continue; }
  pc = 0x20c7f06; continue;
 }
 0x20c7f06 => {
  { let v: u32 = 0x20c7bd1u32; cpu.ebx = v; }
  { let v: u32 = 0x20c7fbbu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b949u32, v); }
  cpu.push(0x20c7f1b); f_020c7f5e(cpu); if cpu.returned_to != 0x20c7f1b { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217cb78u32, v); }
  pc = 0x20c7f21; continue;
 }
 0x20c7f21 => {
  cpu.push(0x20c7f26); f_020c8018(cpu); if cpu.returned_to != 0x20c7f26 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7f29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7f29); }
 let mut pc: u32 = 0x20c7f29;
 loop { match pc {
 0x20c7f29 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22d22d0u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20c7f53; continue;
 }
 0x20c7f3d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x20c7f51; continue; }
  pc = 0x20c7f45; continue;
 }
 0x20c7f45 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x20c7f51; continue; }
  pc = 0x20c7f4c; continue;
 }
 0x20c7f4c => {
  cpu.push(0x20c7f51); f_020c1ed6(cpu); if cpu.returned_to != 0x20c7f51 { return; }
  pc = 0x20c7f51; continue;
 }
 0x20c7f51 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x20c7f53; continue;
 }
 0x20c7f53 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x20c7f3d; continue; }
  pc = 0x20c7f57; continue;
 }
 0x20c7f57 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7f5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7f5e); }
 let mut pc: u32 = 0x20c7f5e;
 loop { match pc {
 0x20c7f5e => {
  { cpu.sub16(cpu.r16(0x217cca0u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x20c7f73; continue; }
  pc = 0x20c7f68; continue;
 }
 0x20c7f68 => {
  { cpu.sub8(cpu.r8(0x217420cu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x20c7f73; continue; }
  pc = 0x20c7f71; continue;
 }
 0x20c7f71 => {
  pc = 0x20c7f74; continue;
 }
 0x20c7f73 => {
  cpu.flat_ds(0x20c7f73); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7f74 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x20c7f8a; continue; }
  pc = 0x20c7f83; continue;
 }
 0x20c7f83 => {
  cpu.flat_ds(0x20c7f83); cpu.push(0x20c7f88); f_020c83ae(cpu); if cpu.returned_to != 0x20c7f88 { return; }
  pc = 0x20c7faf; continue;
 }
 0x20c7f8a => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x20c7f9a; continue; }
  pc = 0x20c7f93; continue;
 }
 0x20c7f93 => {
  cpu.flat_ds(0x20c7f93); cpu.push(0x20c7f98); f_020c8368(cpu); if cpu.returned_to != 0x20c7f98 { return; }
  pc = 0x20c7faf; continue;
 }
 0x20c7f9a => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x20c7faa; continue; }
  pc = 0x20c7fa3; continue;
 }
 0x20c7fa3 => {
  cpu.flat_ds(0x20c7fa3); cpu.push(0x20c7fa8); f_020c8406(cpu); if cpu.returned_to != 0x20c7fa8 { return; }
  pc = 0x20c7faf; continue;
 }
 0x20c7faa => {
  cpu.flat_ds(0x20c7faa); cpu.push(0x20c7faf); f_020c8278(cpu); if cpu.returned_to != 0x20c7faf { return; }
  pc = 0x20c7faf; continue;
 }
 0x20c7faf => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c7fb0); cpu.push(0x20c7fb5); f_020c8018(cpu); if cpu.returned_to != 0x20c7fb5 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x20c7fba); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c7fbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c7fbb); }
 let mut pc: u32 = 0x20c7fbb;
 loop { match pc {
 0x20c7fbb => {
  { cpu.sub16(cpu.r16(0x217cca0u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x20c7fc6; continue; }
  pc = 0x20c7fc5; continue;
 }
 0x20c7fc5 => {
  cpu.flat_ds(0x20c7fc5); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c7fc6 => {
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
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x20c7fed; continue; }
  pc = 0x20c7fe6; continue;
 }
 0x20c7fe6 => {
  cpu.flat_ds(0x20c7fe6); cpu.push(0x20c7feb); f_020c83ec(cpu); if cpu.returned_to != 0x20c7feb { return; }
  pc = 0x20c8012; continue;
 }
 0x20c7fed => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x20c7ffd; continue; }
  pc = 0x20c7ff6; continue;
 }
 0x20c7ff6 => {
  cpu.flat_ds(0x20c7ff6); cpu.push(0x20c7ffb); f_020c8395(cpu); if cpu.returned_to != 0x20c7ffb { return; }
  pc = 0x20c8012; continue;
 }
 0x20c7ffd => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x20c800d; continue; }
  pc = 0x20c8006; continue;
 }
 0x20c8006 => {
  cpu.flat_ds(0x20c8006); cpu.push(0x20c800b); f_020c845e(cpu); if cpu.returned_to != 0x20c800b { return; }
  pc = 0x20c8012; continue;
 }
 0x20c800d => {
  cpu.flat_ds(0x20c800d); cpu.push(0x20c8012); f_020c833b(cpu); if cpu.returned_to != 0x20c8012 { return; }
  pc = 0x20c8012; continue;
 }
 0x20c8012 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x20c8016); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8018(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8018); }
 let mut pc: u32 = 0x20c8018;
 loop { match pc {
 0x20c8018 => {
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
pub fn f_020c802d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c802d); } r_020c802d(cpu, 0x20c802d); }
pub fn f_020c803a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c803a); } r_020c802d(cpu, 0x20c803a); }
fn r_020c802d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c802d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { cpu.port_out(0xa0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out(0xf0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x20c803a; continue;
 }
 0x20c803a => {
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
  cpu.unimplemented(0x20c8048, "Fnstenv");
  {  }
  {  }
  {  }
  {  }
  cpu.unimplemented(0x20c8050, "Fndisi");
  {  }
  cpu.flat_ds(0x20c8053); cpu.push(0x20c8058); f_020ae040(cpu); if cpu.returned_to != 0x20c8058 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x20c8068; continue;
 }
 0x20c8068 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if cpu.cf { cpu.poll(); pc = 0x20c8068; continue; }
  pc = 0x20c8072; continue;
 }
 0x20c8072 => {
  { cpu.sub8((cpu.ebx as u8), 0xdfu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x20c8068; continue; }
  pc = 0x20c8077; continue;
 }
 0x20c8077 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let a = (cpu.ebx as u8); let b = ((cpu.ebx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x20c808a; continue; }
  pc = 0x20c8086; continue;
 }
 0x20c8086 => {
  { let v: u8 = 0x86u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c80d6; continue;
 }
 0x20c808a => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x20c809e; continue; }
  pc = 0x20c808f; continue;
 }
 0x20c808f => {
  { cpu.and16((cpu.edx as u16), 0x200u16); }
  if !cpu.zf { pc = 0x20c809a; continue; }
  pc = 0x20c8096; continue;
 }
 0x20c8096 => {
  { let v: u8 = 0x8bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c809c; continue;
 }
 0x20c809a => {
  { let v: u8 = 0x8au8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c809c; continue;
 }
 0x20c809c => {
  pc = 0x20c80d6; continue;
 }
 0x20c809e => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20c80aa; continue; }
  pc = 0x20c80a3; continue;
 }
 0x20c80a3 => {
  cpu.flat_ds(0x20c80a3); cpu.push(0x20c80a8); f_020c8140(cpu); if cpu.returned_to != 0x20c80a8 { return; }
  pc = 0x20c80d6; continue;
 }
 0x20c80aa => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x20c80b3; continue; }
  pc = 0x20c80af; continue;
 }
 0x20c80af => {
  { let v: u8 = 0x82u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c80d6; continue;
 }
 0x20c80b3 => {
  { cpu.and8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x20c80c1; continue; }
  pc = 0x20c80b8; continue;
 }
 0x20c80b8 => {
  cpu.flat_ds(0x20c80b8); cpu.push(0x20c80bd); f_020c81cd(cpu); if cpu.returned_to != 0x20c80bd { return; }
  { let v: u8 = 0x84u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c80d6; continue;
 }
 0x20c80c1 => {
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x20c80ca; continue; }
  pc = 0x20c80c6; continue;
 }
 0x20c80c6 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c80d6; continue;
 }
 0x20c80ca => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c80d6; continue; }
  pc = 0x20c80cf; continue;
 }
 0x20c80cf => {
  cpu.flat_ds(0x20c80cf); cpu.push(0x20c80d4); f_020c821f(cpu); if cpu.returned_to != 0x20c80d4 { return; }
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c80d6; continue;
 }
 0x20c80d6 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x20c8126; continue; }
  pc = 0x20c80db; continue;
 }
 0x20c80db => {
  { let v = cpu.r32(0x217b920u32.wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x217cec2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x217cec4u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x217cec2u32; cpu.esp = v; }
  { let v: u32 = 0x217ccc2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b920u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x217b949u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20c8104); cpu.push(0x20c810a); cpu.call(t); if cpu.returned_to != 0x20c810a { return; }
  { let v = cpu.r16(0x217cec2u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x217cec4u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x217b920u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x217cec2u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20c8126; continue;
 }
 0x20c8126 => {
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
  cpu.flat_ds(0x20c813f); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8140); }
 let mut pc: u32 = 0x20c8140;
 loop { match pc {
 0x20c8140 => {
  { let v: u8 = 0x81u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xd9fau16); }
  if !cpu.zf { pc = 0x20c814c; continue; }
  pc = 0x20c8149; continue;
 }
 0x20c8149 => {
  { let v: u8 = 0x88u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c814c => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f1u16); }
  if !cpu.zf { pc = 0x20c8156; continue; }
  pc = 0x20c8153; continue;
 }
 0x20c8153 => {
  { let v: u8 = 0x8eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c8156 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f8u16); }
  if cpu.zf { pc = 0x20c8162; continue; }
  pc = 0x20c815d; continue;
 }
 0x20c815d => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f5u16); }
  pc = 0x20c8162; continue;
 }
 0x20c8162 => {
  if !cpu.zf { pc = 0x20c8168; continue; }
  pc = 0x20c8164; continue;
 }
 0x20c8164 => {
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c81cc; continue;
 }
 0x20c8168 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x310u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x310u16); }
  if !cpu.zf { pc = 0x20c817b; continue; }
  pc = 0x20c8177; continue;
 }
 0x20c8177 => {
  { let v: u8 = 0x8du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c81cc; continue;
 }
 0x20c817b => {
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x20c81a5; continue; }
  pc = 0x20c8187; continue;
 }
 0x20c8187 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x20c819a; continue; }
  pc = 0x20c8195; continue;
 }
 0x20c8195 => {
  cpu.push(0x20c819a); f_020c826a(cpu); if cpu.returned_to != 0x20c819a { return; }
  pc = 0x20c819a; continue;
 }
 0x20c819a => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x20c81a1; continue; }
  pc = 0x20c819f; continue;
 }
 0x20c819f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x20c81a1; continue;
 }
 0x20c81a1 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c81cc; continue;
 }
 0x20c81a5 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x130u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x30u16); }
  if !cpu.zf { pc = 0x20c81cc; continue; }
  pc = 0x20c81b3; continue;
 }
 0x20c81b3 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x38u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x20c81cc; continue; }
  pc = 0x20c81ca; continue;
 }
 0x20c81ca => {
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x20c81cc; continue;
 }
 0x20c81cc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c81cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c81cd); }
 let mut pc: u32 = 0x20c81cd;
 loop { match pc {
 0x20c81cd => {
  { cpu.sub16((cpu.ebx as u16), 0xd9fdu16); }
  if !cpu.zf { pc = 0x20c81d6; continue; }
  pc = 0x20c81d4; continue;
 }
 0x20c81d4 => {
  pc = 0x20c821e; continue;
 }
 0x20c81d6 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x20c8202; continue; }
  pc = 0x20c81e5; continue;
 }
 0x20c81e5 => {
  cpu.push(0x20c81ea); f_020c821f(cpu); if cpu.returned_to != 0x20c81ea { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x20c81f9; continue; }
  pc = 0x20c81f4; continue;
 }
 0x20c81f4 => {
  cpu.push(0x20c81f9); f_020c826a(cpu); if cpu.returned_to != 0x20c81f9 { return; }
  pc = 0x20c81f9; continue;
 }
 0x20c81f9 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x20c8200; continue; }
  pc = 0x20c81fe; continue;
 }
 0x20c81fe => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x20c8200; continue;
 }
 0x20c8200 => {
  pc = 0x20c821e; continue;
 }
 0x20c8202 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x20c8219; continue; }
  pc = 0x20c820c; continue;
 }
 0x20c820c => {
  { let v: u32 = 0x217ccacu32; cpu.esi = v; }
  cpu.push(0x20c8217); f_020c8245(cpu); if cpu.returned_to != 0x20c8217 { return; }
  pc = 0x20c821e; continue;
 }
 0x20c8219 => {
  cpu.push(0x20c821e); f_020c821f(cpu); if cpu.returned_to != 0x20c821e { return; }
  pc = 0x20c821e; continue;
 }
 0x20c821e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c821f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c821f); }
 let mut pc: u32 = 0x20c821f;
 loop { match pc {
 0x20c821f => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c8234; continue; }
  pc = 0x20c822c; continue;
 }
 0x20c822c => {
  { let v = cpu.rf64(0x217ccb6u32); cpu.fpush(v); }
  pc = 0x20c823a; continue;
 }
 0x20c8234 => {
  { let v = cpu.rf32(0x217ccbeu32); cpu.fpush(v); }
  pc = 0x20c823a; continue;
 }
 0x20c823a => {
  { cpu.and16(cpu.r16(0x14u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if cpu.zf { pc = 0x20c8244; continue; }
  pc = 0x20c8242; continue;
 }
 0x20c8242 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x20c8244; continue;
 }
 0x20c8244 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8245(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8245); }
 let mut pc: u32 = 0x20c8245;
 loop { match pc {
 0x20c8245 => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if !cpu.zf { pc = 0x20c824e; continue; }
  pc = 0x20c824a; continue;
 }
 0x20c824a => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c8253; continue;
 }
 0x20c824e => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.edx as u8), 0x7u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20c8253; continue;
 }
 0x20c8253 => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x20c8254; continue;
 }
 0x20c8254 => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x20c825c; continue; }
  pc = 0x20c8258; continue;
 }
 0x20c8258 => {
  cpu.unimplemented(0x20c8258, "Fincstp");
  cpu.poll(); pc = 0x20c8254; continue;
 }
 0x20c825c => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x20c8261; continue;
 }
 0x20c8261 => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x20c8269; continue; }
  pc = 0x20c8265; continue;
 }
 0x20c8265 => {
  cpu.unimplemented(0x20c8265, "Fdecstp");
  cpu.poll(); pc = 0x20c8261; continue;
 }
 0x20c8269 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c826a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c826a); }
 let mut pc: u32 = 0x20c826a;
 loop { match pc {
 0x20c826a => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20c8274; continue; }
  pc = 0x20c826f; continue;
 }
 0x20c826f => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20c8277; continue;
 }
 0x20c8274 => {
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20c8277; continue;
 }
 0x20c8277 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8278(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8278); }
 let mut pc: u32 = 0x20c8278;
 loop { match pc {
 0x20c8278 => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x217cecbu32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x20c828a; continue; }
  pc = 0x20c8282; continue;
 }
 0x20c8282 => {
  { let v: u32 = 0x20c802du32; cpu.edx = v; }
  pc = 0x20c8290; continue;
 }
 0x20c828a => {
  { let v: u32 = 0x20c803au32; cpu.edx = v; }
  pc = 0x20c8290; continue;
 }
 0x20c8290 => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x20c82b7; continue; }
  pc = 0x20c8299; continue;
 }
 0x20c8299 => {
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
  pc = 0x20c82b7; continue;
 }
 0x20c82b7 => {
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x217cec9u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cca2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x217cca0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cca6u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x20c832e; continue; }
  pc = 0x20c82f0; continue;
 }
 0x20c82f0 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x20c82f3, "Sidt");
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
  pc = 0x20c8339; continue;
 }
 0x20c832e => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2506u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  pc = 0x20c8339; continue;
 }
 0x20c8339 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c833a); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c833b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c833b); }
 let mut pc: u32 = 0x20c833b;
 loop { match pc {
 0x20c833b => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x217cec9u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x217cca6u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217cca2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x217cca0u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c8367); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8368); }
 let mut pc: u32 = 0x20c8368;
 loop { match pc {
 0x20c8368 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x217cec8u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cca2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x217cca0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x217cec8u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x20c803au32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c8394); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8395(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8395); }
 let mut pc: u32 = 0x20c8395;
 loop { match pc {
 0x20c8395 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x217cec8u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x217cca2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x217cca0u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c83ad); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c83ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c83ae); }
 let mut pc: u32 = 0x20c83ae;
 loop { match pc {
 0x20c83ae => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x217cecau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217cca2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x217cca0u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x217cecau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x217cecbu32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x20c83dd; continue; }
  pc = 0x20c83d5; continue;
 }
 0x20c83d5 => {
  { let v: u32 = 0x20c802du32; cpu.edx = v; }
  pc = 0x20c83e3; continue;
 }
 0x20c83dd => {
  { let v: u32 = 0x20c803au32; cpu.edx = v; }
  pc = 0x20c83e3; continue;
 }
 0x20c83e3 => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c83eb); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c83ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c83ec); }
 let mut pc: u32 = 0x20c83ec;
 loop { match pc {
 0x20c83ec => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x217cecau32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x217cca2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x217cca0u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20c8404); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8406(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8406); }
 let mut pc: u32 = 0x20c8406;
 loop { match pc {
 0x20c8406 => {
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x217cec8u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.edx; cpu.w32(0x217cca2u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217cca0u32, v); }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x217cec8u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217cca6u32, v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x217cecbu32), 0x20u8); }
  if !cpu.zf { pc = 0x20c844b; continue; }
  pc = 0x20c8443; continue;
 }
 0x20c8443 => {
  { let v: u32 = 0x20c802du32; cpu.edx = v; }
  pc = 0x20c8451; continue;
 }
 0x20c844b => {
  { let v: u32 = 0x20c803au32; cpu.edx = v; }
  pc = 0x20c8451; continue;
 }
 0x20c8451 => {
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x217cec8u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c845e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c845e); }
 let mut pc: u32 = 0x20c845e;
 loop { match pc {
 0x20c845e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8460(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8460); } r_020c8460(cpu, 0x20c8460); }
pub fn f_020c8470(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8470); } r_020c8460(cpu, 0x20c8470); }
pub fn f_020c8481(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8481); } r_020c8460(cpu, 0x20c8481); }
pub fn f_020c8495(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8495); } r_020c8460(cpu, 0x20c8495); }
pub fn f_020c84b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c84b2); } r_020c8460(cpu, 0x20c84b2); }
pub fn f_020c84c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c84c7); } r_020c8460(cpu, 0x20c84c7); }
pub fn f_020c84e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c84e0); } r_020c8460(cpu, 0x20c84e0); }
pub fn f_020c84f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c84f3); } r_020c8460(cpu, 0x20c84f3); }
pub fn f_020c8513(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8513); } r_020c8460(cpu, 0x20c8513); }
pub fn f_020c852a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c852a); } r_020c8460(cpu, 0x20c852a); }
pub fn f_020c8547(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8547); } r_020c8460(cpu, 0x20c8547); }
pub fn f_020c8559(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8559); } r_020c8460(cpu, 0x20c8559); }
pub fn f_020c856f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c856f); } r_020c8460(cpu, 0x20c856f); }
pub fn f_020c857f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c857f); } r_020c8460(cpu, 0x20c857f); }
pub fn f_020c8591(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8591); } r_020c8460(cpu, 0x20c8591); }
pub fn f_020c85a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c85a4); } r_020c8460(cpu, 0x20c85a4); }
pub fn f_020c85b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c85b8); } r_020c8460(cpu, 0x20c85b8); }
pub fn f_020c85c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c85c9); } r_020c8460(cpu, 0x20c85c9); }
pub fn f_020c85dc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c85dc); } r_020c8460(cpu, 0x20c85dc); }
pub fn f_020c85eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c85eb); } r_020c8460(cpu, 0x20c85eb); }
pub fn f_020c8605(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8605); } r_020c8460(cpu, 0x20c8605); }
pub fn f_020c861d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c861d); } r_020c8460(cpu, 0x20c861d); }
pub fn f_020c862b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c862b); } r_020c8460(cpu, 0x20c862b); }
pub fn f_020c863f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c863f); } r_020c8460(cpu, 0x20c863f); }
pub fn f_020c8658(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8658); } r_020c8460(cpu, 0x20c8658); }
pub fn f_020c866d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c866d); } r_020c8460(cpu, 0x20c866d); }
pub fn f_020c867c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c867c); } r_020c8460(cpu, 0x20c867c); }
pub fn f_020c8693(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8693); } r_020c8460(cpu, 0x20c8693); }
pub fn f_020c86a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c86a3); } r_020c8460(cpu, 0x20c86a3); }
pub fn f_020c86b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c86b1); } r_020c8460(cpu, 0x20c86b1); }
pub fn f_020c86c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c86c9); } r_020c8460(cpu, 0x20c86c9); }
pub fn f_020c86ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c86ea); } r_020c8460(cpu, 0x20c86ea); }
pub fn f_020c86f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c86f9); } r_020c8460(cpu, 0x20c86f9); }
pub fn f_020c870b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c870b); } r_020c8460(cpu, 0x20c870b); }
pub fn f_020c8726(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8726); } r_020c8460(cpu, 0x20c8726); }
pub fn f_020c8739(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8739); } r_020c8460(cpu, 0x20c8739); }
pub fn f_020c874f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c874f); } r_020c8460(cpu, 0x20c874f); }
pub fn f_020c8762(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8762); } r_020c8460(cpu, 0x20c8762); }
pub fn f_020c876f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c876f); } r_020c8460(cpu, 0x20c876f); }
pub fn f_020c877d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c877d); } r_020c8460(cpu, 0x20c877d); }
pub fn f_020c8797(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8797); } r_020c8460(cpu, 0x20c8797); }
pub fn f_020c87aa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c87aa); } r_020c8460(cpu, 0x20c87aa); }
pub fn f_020c87bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c87bf); } r_020c8460(cpu, 0x20c87bf); }
pub fn f_020c87ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c87ce); } r_020c8460(cpu, 0x20c87ce); }
pub fn f_020c87e1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c87e1); } r_020c8460(cpu, 0x20c87e1); }
pub fn f_020c87f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c87f9); } r_020c8460(cpu, 0x20c87f9); }
pub fn f_020c880c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c880c); } r_020c8460(cpu, 0x20c880c); }
pub fn f_020c8817(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8817); } r_020c8460(cpu, 0x20c8817); }
pub fn f_020c882d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c882d); } r_020c8460(cpu, 0x20c882d); }
pub fn f_020c8843(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8843); } r_020c8460(cpu, 0x20c8843); }
pub fn f_020c8851(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8851); } r_020c8460(cpu, 0x20c8851); }
pub fn f_020c8865(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8865); } r_020c8460(cpu, 0x20c8865); }
pub fn f_020c887a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c887a); } r_020c8460(cpu, 0x20c887a); }
pub fn f_020c8889(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8889); } r_020c8460(cpu, 0x20c8889); }
pub fn f_020c88a2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c88a2); } r_020c8460(cpu, 0x20c88a2); }
pub fn f_020c88bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c88bc); } r_020c8460(cpu, 0x20c88bc); }
pub fn f_020c88ce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c88ce); } r_020c8460(cpu, 0x20c88ce); }
pub fn f_020c88e1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c88e1); } r_020c8460(cpu, 0x20c88e1); }
pub fn f_020c88ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c88ee); } r_020c8460(cpu, 0x20c88ee); }
pub fn f_020c8906(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8906); } r_020c8460(cpu, 0x20c8906); }
pub fn f_020c8916(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8916); } r_020c8460(cpu, 0x20c8916); }
pub fn f_020c8923(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8923); } r_020c8460(cpu, 0x20c8923); }
pub fn f_020c8932(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8932); } r_020c8460(cpu, 0x20c8932); }
pub fn f_020c8941(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8941); } r_020c8460(cpu, 0x20c8941); }
pub fn f_020c8953(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8953); } r_020c8460(cpu, 0x20c8953); }
pub fn f_020c8967(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8967); } r_020c8460(cpu, 0x20c8967); }
pub fn f_020c897d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c897d); } r_020c8460(cpu, 0x20c897d); }
pub fn f_020c8994(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8994); } r_020c8460(cpu, 0x20c8994); }
pub fn f_020c89a9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c89a9); } r_020c8460(cpu, 0x20c89a9); }
pub fn f_020c89bd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c89bd); } r_020c8460(cpu, 0x20c89bd); }
pub fn f_020c89cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c89cc); } r_020c8460(cpu, 0x20c89cc); }
pub fn f_020c89db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c89db); } r_020c8460(cpu, 0x20c89db); }
pub fn f_020c89ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c89ed); } r_020c8460(cpu, 0x20c89ed); }
pub fn f_020c89fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c89fd); } r_020c8460(cpu, 0x20c89fd); }
pub fn f_020c8a10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a10); } r_020c8460(cpu, 0x20c8a10); }
pub fn f_020c8a21(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a21); } r_020c8460(cpu, 0x20c8a21); }
pub fn f_020c8a32(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a32); } r_020c8460(cpu, 0x20c8a32); }
pub fn f_020c8a4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a4b); } r_020c8460(cpu, 0x20c8a4b); }
pub fn f_020c8a5e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a5e); } r_020c8460(cpu, 0x20c8a5e); }
pub fn f_020c8a6f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a6f); } r_020c8460(cpu, 0x20c8a6f); }
pub fn f_020c8a87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a87); } r_020c8460(cpu, 0x20c8a87); }
pub fn f_020c8a9e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8a9e); } r_020c8460(cpu, 0x20c8a9e); }
pub fn f_020c8ab6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ab6); } r_020c8460(cpu, 0x20c8ab6); }
pub fn f_020c8ad3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ad3); } r_020c8460(cpu, 0x20c8ad3); }
pub fn f_020c8af7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8af7); } r_020c8460(cpu, 0x20c8af7); }
pub fn f_020c8b0e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b0e); } r_020c8460(cpu, 0x20c8b0e); }
pub fn f_020c8b2c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b2c); } r_020c8460(cpu, 0x20c8b2c); }
pub fn f_020c8b40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b40); } r_020c8460(cpu, 0x20c8b40); }
pub fn f_020c8b55(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b55); } r_020c8460(cpu, 0x20c8b55); }
pub fn f_020c8b68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b68); } r_020c8460(cpu, 0x20c8b68); }
pub fn f_020c8b7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b7b); } r_020c8460(cpu, 0x20c8b7b); }
pub fn f_020c8b8f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8b8f); } r_020c8460(cpu, 0x20c8b8f); }
pub fn f_020c8ba1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ba1); } r_020c8460(cpu, 0x20c8ba1); }
pub fn f_020c8bbb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8bbb); } r_020c8460(cpu, 0x20c8bbb); }
pub fn f_020c8bcc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8bcc); } r_020c8460(cpu, 0x20c8bcc); }
pub fn f_020c8bdc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8bdc); } r_020c8460(cpu, 0x20c8bdc); }
pub fn f_020c8bf7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8bf7); } r_020c8460(cpu, 0x20c8bf7); }
pub fn f_020c8c05(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c05); } r_020c8460(cpu, 0x20c8c05); }
pub fn f_020c8c15(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c15); } r_020c8460(cpu, 0x20c8c15); }
pub fn f_020c8c22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c22); } r_020c8460(cpu, 0x20c8c22); }
pub fn f_020c8c3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c3f); } r_020c8460(cpu, 0x20c8c3f); }
pub fn f_020c8c50(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c50); } r_020c8460(cpu, 0x20c8c50); }
pub fn f_020c8c6a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c6a); } r_020c8460(cpu, 0x20c8c6a); }
pub fn f_020c8c8d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8c8d); } r_020c8460(cpu, 0x20c8c8d); }
pub fn f_020c8ca8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ca8); } r_020c8460(cpu, 0x20c8ca8); }
pub fn f_020c8cba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8cba); } r_020c8460(cpu, 0x20c8cba); }
pub fn f_020c8ccf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ccf); } r_020c8460(cpu, 0x20c8ccf); }
pub fn f_020c8ce4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ce4); } r_020c8460(cpu, 0x20c8ce4); }
pub fn f_020c8cf7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8cf7); } r_020c8460(cpu, 0x20c8cf7); }
pub fn f_020c8d02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d02); } r_020c8460(cpu, 0x20c8d02); }
pub fn f_020c8d0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d0b); } r_020c8460(cpu, 0x20c8d0b); }
pub fn f_020c8d20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d20); } r_020c8460(cpu, 0x20c8d20); }
pub fn f_020c8d34(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d34); } r_020c8460(cpu, 0x20c8d34); }
pub fn f_020c8d4c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d4c); } r_020c8460(cpu, 0x20c8d4c); }
pub fn f_020c8d62(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d62); } r_020c8460(cpu, 0x20c8d62); }
pub fn f_020c8d74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d74); } r_020c8460(cpu, 0x20c8d74); }
pub fn f_020c8d84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d84); } r_020c8460(cpu, 0x20c8d84); }
pub fn f_020c8d93(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8d93); } r_020c8460(cpu, 0x20c8d93); }
pub fn f_020c8da8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8da8); } r_020c8460(cpu, 0x20c8da8); }
pub fn f_020c8db4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8db4); } r_020c8460(cpu, 0x20c8db4); }
pub fn f_020c8dce(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8dce); } r_020c8460(cpu, 0x20c8dce); }
pub fn f_020c8ddf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ddf); } r_020c8460(cpu, 0x20c8ddf); }
pub fn f_020c8df7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8df7); } r_020c8460(cpu, 0x20c8df7); }
pub fn f_020c8e11(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8e11); } r_020c8460(cpu, 0x20c8e11); }
pub fn f_020c8e29(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8e29); } r_020c8460(cpu, 0x20c8e29); }
pub fn f_020c8e43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8e43); } r_020c8460(cpu, 0x20c8e43); }
pub fn f_020c8e5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8e5b); } r_020c8460(cpu, 0x20c8e5b); }
pub fn f_020c8e78(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8e78); } r_020c8460(cpu, 0x20c8e78); }
pub fn f_020c8e91(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8e91); } r_020c8460(cpu, 0x20c8e91); }
pub fn f_020c8ea7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ea7); } r_020c8460(cpu, 0x20c8ea7); }
pub fn f_020c8ebd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ebd); } r_020c8460(cpu, 0x20c8ebd); }
pub fn f_020c8ece(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8ece); } r_020c8460(cpu, 0x20c8ece); }
pub fn f_020c8edd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8edd); } r_020c8460(cpu, 0x20c8edd); }
fn r_020c8460(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c8460 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8470; continue;
 }
 0x20c8470 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8481; continue;
 }
 0x20c8481 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c84b2; continue;
 }
 0x20c8495 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c84b2; continue;
 }
 0x20c84b2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c84c7; continue;
 }
 0x20c84c7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8513; continue;
 }
 0x20c84e0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8513; continue;
 }
 0x20c84f3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8513; continue;
 }
 0x20c8513 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c852a; continue;
 }
 0x20c852a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8547; continue;
 }
 0x20c8547 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c856f; continue;
 }
 0x20c8559 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c856f; continue;
 }
 0x20c856f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c857f; continue;
 }
 0x20c857f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8591; continue;
 }
 0x20c8591 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c85a4; continue;
 }
 0x20c85a4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c85b8; continue;
 }
 0x20c85b8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c85eb; continue;
 }
 0x20c85c9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c85eb; continue;
 }
 0x20c85dc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c85eb; continue;
 }
 0x20c85eb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c861d; continue;
 }
 0x20c8605 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c861d; continue;
 }
 0x20c861d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c862b; continue;
 }
 0x20c862b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c863f; continue;
 }
 0x20c863f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8658; continue;
 }
 0x20c8658 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c866d; continue;
 }
 0x20c866d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c867c; continue;
 }
 0x20c867c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8693; continue;
 }
 0x20c8693 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c86a3; continue;
 }
 0x20c86a3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c86b1; continue;
 }
 0x20c86b1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c86ea; continue;
 }
 0x20c86c9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c86ea; continue;
 }
 0x20c86ea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c86f9; continue;
 }
 0x20c86f9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8726; continue;
 }
 0x20c870b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8726; continue;
 }
 0x20c8726 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c874f; continue;
 }
 0x20c8739 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c874f; continue;
 }
 0x20c874f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8762; continue;
 }
 0x20c8762 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c876f; continue;
 }
 0x20c876f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c877d; continue;
 }
 0x20c877d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8797; continue;
 }
 0x20c8797 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c87aa; continue;
 }
 0x20c87aa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c87bf; continue;
 }
 0x20c87bf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c87ce; continue;
 }
 0x20c87ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c87e1; continue;
 }
 0x20c87e1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c87f9; continue;
 }
 0x20c87f9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c880c; continue;
 }
 0x20c880c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8817; continue;
 }
 0x20c8817 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c882d; continue;
 }
 0x20c882d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8843; continue;
 }
 0x20c8843 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c8851; continue;
 }
 0x20c8851 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8889; continue;
 }
 0x20c8865 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8889; continue;
 }
 0x20c887a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8889; continue;
 }
 0x20c8889 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c88a2; continue;
 }
 0x20c88a2 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c88bc; continue;
 }
 0x20c88bc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c88ce; continue;
 }
 0x20c88ce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c88e1; continue;
 }
 0x20c88e1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8916; continue;
 }
 0x20c88ee => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8916; continue;
 }
 0x20c8906 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8916; continue;
 }
 0x20c8916 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8923; continue;
 }
 0x20c8923 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8932; continue;
 }
 0x20c8932 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8941; continue;
 }
 0x20c8941 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8953; continue;
 }
 0x20c8953 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8967; continue;
 }
 0x20c8967 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c897d; continue;
 }
 0x20c897d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8994; continue;
 }
 0x20c8994 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c89a9; continue;
 }
 0x20c89a9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c89bd; continue;
 }
 0x20c89bd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c89cc; continue;
 }
 0x20c89cc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c89db; continue;
 }
 0x20c89db => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c89ed; continue;
 }
 0x20c89ed => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c89fd; continue;
 }
 0x20c89fd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8a10; continue;
 }
 0x20c8a10 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8a21; continue;
 }
 0x20c8a21 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8a87; continue;
 }
 0x20c8a32 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8a87; continue;
 }
 0x20c8a4b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8a87; continue;
 }
 0x20c8a5e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8a87; continue;
 }
 0x20c8a6f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8a87; continue;
 }
 0x20c8a87 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8a9e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8ab6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8ad3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x30u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8af7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8b0e => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8b2c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8b40 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8b55; continue;
 }
 0x20c8b55 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8b68; continue;
 }
 0x20c8b68 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8b7b; continue;
 }
 0x20c8b7b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8ba1; continue;
 }
 0x20c8b8f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8ba1; continue;
 }
 0x20c8ba1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8bcc; continue;
 }
 0x20c8bbb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8bcc; continue;
 }
 0x20c8bcc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8bf7; continue;
 }
 0x20c8bdc => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8bf7; continue;
 }
 0x20c8bf7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8c05; continue;
 }
 0x20c8c05 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x47u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8c15; continue;
 }
 0x20c8c15 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x4cu32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8c22; continue;
 }
 0x20c8c22 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8c50; continue;
 }
 0x20c8c3f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8c50; continue;
 }
 0x20c8c50 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8c6a; continue;
 }
 0x20c8c6a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8c8d; continue;
 }
 0x20c8c8d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8ccf; continue;
 }
 0x20c8ca8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8ccf; continue;
 }
 0x20c8cba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8ccf; continue;
 }
 0x20c8ccf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8ce4; continue;
 }
 0x20c8ce4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c8cf7; continue;
 }
 0x20c8cf7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x20c8d02; continue;
 }
 0x20c8d02 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20c8d0b; continue;
 }
 0x20c8d0b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) ^ cpu.r8(0x38u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8d34; continue;
 }
 0x20c8d20 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8d34; continue;
 }
 0x20c8d34 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8d62; continue;
 }
 0x20c8d4c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8d62; continue;
 }
 0x20c8d62 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8d74; continue;
 }
 0x20c8d74 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8d84; continue;
 }
 0x20c8d84 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8d93; continue;
 }
 0x20c8d93 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8db4; continue;
 }
 0x20c8da8 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8db4; continue;
 }
 0x20c8db4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8dce; continue;
 }
 0x20c8dce => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8ddf; continue;
 }
 0x20c8ddf => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8df7; continue;
 }
 0x20c8df7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x38u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8e29; continue;
 }
 0x20c8e11 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x20c8e29; continue;
 }
 0x20c8e29 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8e43; continue;
 }
 0x20c8e43 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8e91; continue;
 }
 0x20c8e5b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)) ^ cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x47u32.wrapping_add(cpu.edi)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x47u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8e91; continue;
 }
 0x20c8e78 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8e91; continue;
 }
 0x20c8e91 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8ea7; continue;
 }
 0x20c8ea7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8ebd; continue;
 }
 0x20c8ebd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) ^ (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20c8ece; continue;
 }
 0x20c8ece => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20c8edd; continue;
 }
 0x20c8edd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x90u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.call(0x20c8efb); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8ef8); }
 let mut pc: u32 = 0x20c8ef8;
 loop { match pc {
 0x20c8ef8 => {
  cpu.push(0x20c8efd); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8efd { return; }
  cpu.push(0x20c8f02); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f02 { return; }
  cpu.push(0x20c8f07); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f07 { return; }
  cpu.push(0x20c8f0c); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f0c { return; }
  cpu.push(0x20c8f11); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f11 { return; }
  f_020c8f11(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f11); }
 let mut pc: u32 = 0x20c8f11;
 loop { match pc {
 0x20c8f11 => {
  cpu.push(0x20c8f16); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f16 { return; }
  cpu.push(0x20c8f1b); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f1b { return; }
  cpu.push(0x20c8f20); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f20 { return; }
  cpu.push(0x20c8f25); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f25 { return; }
  cpu.push(0x20c8f2a); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f2a { return; }
  f_020c8f2a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f2a); }
 let mut pc: u32 = 0x20c8f2a;
 loop { match pc {
 0x20c8f2a => {
  cpu.push(0x20c8f2f); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f2f { return; }
  cpu.push(0x20c8f34); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f34 { return; }
  f_020c8f34(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f34); }
 let mut pc: u32 = 0x20c8f34;
 loop { match pc {
 0x20c8f34 => {
  cpu.push(0x20c8f39); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f39 { return; }
  cpu.push(0x20c8f3e); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f3e { return; }
  f_020c8f3e(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f3e); }
 let mut pc: u32 = 0x20c8f3e;
 loop { match pc {
 0x20c8f3e => {
  cpu.push(0x20c8f43); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f43 { return; }
  f_020c8f43(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f43); }
 let mut pc: u32 = 0x20c8f43;
 loop { match pc {
 0x20c8f43 => {
  cpu.push(0x20c8f48); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f48 { return; }
  cpu.push(0x20c8f4d); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f4d { return; }
  f_020c8f4d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f4d); }
 let mut pc: u32 = 0x20c8f4d;
 loop { match pc {
 0x20c8f4d => {
  cpu.push(0x20c8f52); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f52 { return; }
  f_020c8f52(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f52); }
 let mut pc: u32 = 0x20c8f52;
 loop { match pc {
 0x20c8f52 => {
  cpu.push(0x20c8f57); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f57 { return; }
  cpu.push(0x20c8f5c); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f5c { return; }
  f_020c8f5c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f5c); }
 let mut pc: u32 = 0x20c8f5c;
 loop { match pc {
 0x20c8f5c => {
  cpu.push(0x20c8f61); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f61 { return; }
  cpu.push(0x20c8f66); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f66 { return; }
  cpu.push(0x20c8f6b); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f6b { return; }
  f_020c8f6b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f6b); }
 let mut pc: u32 = 0x20c8f6b;
 loop { match pc {
 0x20c8f6b => {
  cpu.push(0x20c8f70); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f70 { return; }
  f_020c8f70(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f70); }
 let mut pc: u32 = 0x20c8f70;
 loop { match pc {
 0x20c8f70 => {
  cpu.push(0x20c8f75); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f75 { return; }
  f_020c8f75(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f75); }
 let mut pc: u32 = 0x20c8f75;
 loop { match pc {
 0x20c8f75 => {
  cpu.push(0x20c8f7a); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f7a { return; }
  f_020c8f7a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8f7a); }
 let mut pc: u32 = 0x20c8f7a;
 loop { match pc {
 0x20c8f7a => {
  cpu.push(0x20c8f7f); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f7f { return; }
  cpu.push(0x20c8f84); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f84 { return; }
  cpu.push(0x20c8f89); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f89 { return; }
  cpu.push(0x20c8f8e); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f8e { return; }
  cpu.push(0x20c8f93); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f93 { return; }
  cpu.push(0x20c8f98); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f98 { return; }
  cpu.push(0x20c8f9d); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8f9d { return; }
  f_020c8f9d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8f9d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8f9d); } r_020c8f9d(cpu, 0x20c8f9d); }
pub fn f_020c8fa2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c8fa2); } r_020c8f9d(cpu, 0x20c8fa2); }
fn r_020c8f9d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20c8f9d => {
  cpu.push(0x20c8fa2); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fa2 { return; }
  pc = 0x20c8fa2; continue;
 }
 0x20c8fa2 => {
  cpu.push(0x20c8fa7); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fa7 { return; }
  cpu.push(0x20c8fac); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fac { return; }
  f_020c8fac(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fac); }
 let mut pc: u32 = 0x20c8fac;
 loop { match pc {
 0x20c8fac => {
  cpu.push(0x20c8fb1); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fb1 { return; }
  f_020c8fb1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fb1); }
 let mut pc: u32 = 0x20c8fb1;
 loop { match pc {
 0x20c8fb1 => {
  cpu.push(0x20c8fb6); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fb6 { return; }
  f_020c8fb6(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fb6); }
 let mut pc: u32 = 0x20c8fb6;
 loop { match pc {
 0x20c8fb6 => {
  cpu.push(0x20c8fbb); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fbb { return; }
  f_020c8fbb(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fbb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fbb); }
 let mut pc: u32 = 0x20c8fbb;
 loop { match pc {
 0x20c8fbb => {
  cpu.push(0x20c8fc0); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fc0 { return; }
  cpu.push(0x20c8fc5); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fc5 { return; }
  cpu.push(0x20c8fca); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fca { return; }
  f_020c8fca(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fca); }
 let mut pc: u32 = 0x20c8fca;
 loop { match pc {
 0x20c8fca => {
  cpu.push(0x20c8fcf); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fcf { return; }
  cpu.push(0x20c8fd4); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fd4 { return; }
  cpu.push(0x20c8fd9); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fd9 { return; }
  f_020c8fd9(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fd9); }
 let mut pc: u32 = 0x20c8fd9;
 loop { match pc {
 0x20c8fd9 => {
  cpu.push(0x20c8fde); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fde { return; }
  cpu.push(0x20c8fe3); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fe3 { return; }
  cpu.push(0x20c8fe8); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fe8 { return; }
  cpu.push(0x20c8fed); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8fed { return; }
  f_020c8fed(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8fed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8fed); }
 let mut pc: u32 = 0x20c8fed;
 loop { match pc {
 0x20c8fed => {
  cpu.push(0x20c8ff2); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8ff2 { return; }
  cpu.push(0x20c8ff7); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8ff7 { return; }
  cpu.push(0x20c8ffc); f_020c1fb0(cpu); if cpu.returned_to != 0x20c8ffc { return; }
  f_020c8ffc(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c8ffc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c8ffc); }
 let mut pc: u32 = 0x20c8ffc;
 loop { match pc {
 0x20c8ffc => {
  cpu.push(0x20c9001); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9001 { return; }
  cpu.push(0x20c9006); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9006 { return; }
  cpu.push(0x20c900b); f_020c1fb0(cpu); if cpu.returned_to != 0x20c900b { return; }
  f_020c900b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c900b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c900b); }
 let mut pc: u32 = 0x20c900b;
 loop { match pc {
 0x20c900b => {
  cpu.push(0x20c9010); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9010 { return; }
  cpu.push(0x20c9015); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9015 { return; }
  cpu.push(0x20c901a); f_020c1fb0(cpu); if cpu.returned_to != 0x20c901a { return; }
  cpu.push(0x20c901f); f_020c1fb0(cpu); if cpu.returned_to != 0x20c901f { return; }
  cpu.push(0x20c9024); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9024 { return; }
  f_020c9024(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c9024(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c9024); }
 let mut pc: u32 = 0x20c9024;
 loop { match pc {
 0x20c9024 => {
  cpu.push(0x20c9029); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9029 { return; }
  cpu.push(0x20c902e); f_020c1fb0(cpu); if cpu.returned_to != 0x20c902e { return; }
  cpu.push(0x20c9033); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9033 { return; }
  cpu.push(0x20c9038); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9038 { return; }
  cpu.push(0x20c903d); f_020c1fb0(cpu); if cpu.returned_to != 0x20c903d { return; }
  f_020c903d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c903d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c903d); }
 let mut pc: u32 = 0x20c903d;
 loop { match pc {
 0x20c903d => {
  cpu.push(0x20c9042); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9042 { return; }
  cpu.push(0x20c9047); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9047 { return; }
  cpu.push(0x20c904c); f_020c1fb0(cpu); if cpu.returned_to != 0x20c904c { return; }
  cpu.push(0x20c9051); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9051 { return; }
  f_020c9051(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c9051(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c9051); }
 let mut pc: u32 = 0x20c9051;
 loop { match pc {
 0x20c9051 => {
  cpu.push(0x20c9056); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9056 { return; }
  cpu.push(0x20c905b); f_020c1fb0(cpu); if cpu.returned_to != 0x20c905b { return; }
  cpu.push(0x20c9060); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9060 { return; }
  cpu.push(0x20c9065); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9065 { return; }
  cpu.push(0x20c906a); f_020c1fb0(cpu); if cpu.returned_to != 0x20c906a { return; }
  cpu.push(0x20c906f); f_020c1fb0(cpu); if cpu.returned_to != 0x20c906f { return; }
  f_020c906f(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c906f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c906f); }
 let mut pc: u32 = 0x20c906f;
 loop { match pc {
 0x20c906f => {
  cpu.push(0x20c9074); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9074 { return; }
  cpu.push(0x20c9079); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9079 { return; }
  cpu.push(0x20c907e); f_020c1fb0(cpu); if cpu.returned_to != 0x20c907e { return; }
  cpu.push(0x20c9083); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9083 { return; }
  f_020c9083(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c9083(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c9083); }
 let mut pc: u32 = 0x20c9083;
 loop { match pc {
 0x20c9083 => {
  cpu.push(0x20c9088); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9088 { return; }
  cpu.push(0x20c908d); f_020c1fb0(cpu); if cpu.returned_to != 0x20c908d { return; }
  f_020c908d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c908d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c908d); }
 let mut pc: u32 = 0x20c908d;
 loop { match pc {
 0x20c908d => {
  cpu.push(0x20c9092); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9092 { return; }
  cpu.push(0x20c9097); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9097 { return; }
  cpu.push(0x20c909c); f_020c1fb0(cpu); if cpu.returned_to != 0x20c909c { return; }
  cpu.push(0x20c90a1); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90a1 { return; }
  cpu.push(0x20c90a6); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90a6 { return; }
  cpu.push(0x20c90ab); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90ab { return; }
  cpu.push(0x20c90b0); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90b0 { return; }
  f_020c90b0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c90b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c90b0); }
 let mut pc: u32 = 0x20c90b0;
 loop { match pc {
 0x20c90b0 => {
  cpu.push(0x20c90b5); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90b5 { return; }
  f_020c90b5(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c90b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c90b5); }
 let mut pc: u32 = 0x20c90b5;
 loop { match pc {
 0x20c90b5 => {
  cpu.push(0x20c90ba); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90ba { return; }
  cpu.push(0x20c90bf); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90bf { return; }
  cpu.push(0x20c90c4); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90c4 { return; }
  cpu.push(0x20c90c9); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90c9 { return; }
  cpu.push(0x20c90ce); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90ce { return; }
  f_020c90ce(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c90ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c90ce); }
 let mut pc: u32 = 0x20c90ce;
 loop { match pc {
 0x20c90ce => {
  cpu.push(0x20c90d3); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90d3 { return; }
  f_020c90d3(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c90d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c90d3); }
 let mut pc: u32 = 0x20c90d3;
 loop { match pc {
 0x20c90d3 => {
  cpu.push(0x20c90d8); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90d8 { return; }
  cpu.push(0x20c90dd); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90dd { return; }
  cpu.push(0x20c90e2); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90e2 { return; }
  cpu.push(0x20c90e7); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90e7 { return; }
  cpu.push(0x20c90ec); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90ec { return; }
  cpu.push(0x20c90f1); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90f1 { return; }
  f_020c90f1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c90f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c90f1); }
 let mut pc: u32 = 0x20c90f1;
 loop { match pc {
 0x20c90f1 => {
  cpu.push(0x20c90f6); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90f6 { return; }
  cpu.push(0x20c90fb); f_020c1fb0(cpu); if cpu.returned_to != 0x20c90fb { return; }
  cpu.push(0x20c9100); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9100 { return; }
  cpu.push(0x20c9105); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9105 { return; }
  cpu.push(0x20c910a); f_020c1fb0(cpu); if cpu.returned_to != 0x20c910a { return; }
  cpu.push(0x20c910f); f_020c1fb0(cpu); if cpu.returned_to != 0x20c910f { return; }
  cpu.push(0x20c9114); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9114 { return; }
  cpu.push(0x20c9119); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9119 { return; }
  cpu.push(0x20c911e); f_020c1fb0(cpu); if cpu.returned_to != 0x20c911e { return; }
  cpu.push(0x20c9123); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9123 { return; }
  f_020c9123(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c9123(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c9123); }
 let mut pc: u32 = 0x20c9123;
 loop { match pc {
 0x20c9123 => {
  cpu.push(0x20c9128); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9128 { return; }
  cpu.push(0x20c912d); f_020c1fb0(cpu); if cpu.returned_to != 0x20c912d { return; }
  cpu.push(0x20c9132); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9132 { return; }
  cpu.push(0x20c9137); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9137 { return; }
  cpu.push(0x20c913c); f_020c1fb0(cpu); if cpu.returned_to != 0x20c913c { return; }
  cpu.push(0x20c9141); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9141 { return; }
  cpu.push(0x20c9146); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9146 { return; }
  cpu.push(0x20c914b); f_020c1fb0(cpu); if cpu.returned_to != 0x20c914b { return; }
  cpu.push(0x20c9150); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9150 { return; }
  f_020c9150(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020c9150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20c9150); }
 let mut pc: u32 = 0x20c9150;
 loop { match pc {
 0x20c9150 => {
  cpu.push(0x20c9155); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9155 { return; }
  cpu.push(0x20c915a); f_020c1fb0(cpu); if cpu.returned_to != 0x20c915a { return; }
  cpu.push(0x20c915f); f_020c1fb0(cpu); if cpu.returned_to != 0x20c915f { return; }
  cpu.push(0x20c9164); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9164 { return; }
  cpu.push(0x20c9169); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9169 { return; }
  cpu.push(0x20c916e); f_020c1fb0(cpu); if cpu.returned_to != 0x20c916e { return; }
  cpu.push(0x20c9173); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9173 { return; }
  cpu.push(0x20c9178); f_020c1fb0(cpu); if cpu.returned_to != 0x20c9178 { return; }
  cpu.push(0x20c917d); f_020c1fb0(cpu); if cpu.returned_to != 0x20c917d { return; }
  f_020c917d(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
