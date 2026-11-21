#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_060c0308(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0308); }
 let mut pc: u32 = 0x60c0308;
 loop { match pc {
 0x60c0308 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0xffffff00u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 25u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x60c0337; continue; }
  pc = 0x60c0320; continue;
 }
 0x60c0320 => {
  if !cpu.zf { pc = 0x60c0329; continue; }
  pc = 0x60c0322; continue;
 }
 0x60c0322 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c0329; continue; }
  pc = 0x60c0327; continue;
 }
 0x60c0327 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  pc = 0x60c0329; continue;
 }
 0x60c0329 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x100u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x60c0337; continue; }
  pc = 0x60c0330; continue;
 }
 0x60c0330 => {
  { let v: u32 = 0x80000000u32; cpu.eax = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c0337; continue;
 }
 0x60c0337 => {
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0x7fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60c0383; continue; }
  pc = 0x60c0342; continue;
 }
 0x60c0342 => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x60c035a; continue; }
  pc = 0x60c0349; continue;
 }
 0x60c0349 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60c0383; continue;
 }
 0x60c035a => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), 0xc080u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x60c0365; continue; }
  pc = 0x60c0361; continue;
 }
 0x60c0361 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c0383; continue;
 }
 0x60c0365 => {
  { cpu.sub16((cpu.ecx as u16), 0xffu16); }
  if (cpu.sf != cpu.of) { pc = 0x60c0378; continue; }
  pc = 0x60c036c; continue;
 }
 0x60c036c => {
  { let v: u32 = 0xff000000u32; cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60c0383; continue;
 }
 0x60c0378 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.ecx, 8u32); cpu.eax = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60c0383; continue;
 }
 0x60c0383 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0387(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0387); }
 let mut pc: u32 = 0x60c0387;
 loop { match pc {
 0x60c0387 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xfffff800u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 22u32); cpu.ebx = v; }
  if !cpu.cf { pc = 0x60c03b4; continue; }
  pc = 0x60c039f; continue;
 }
 0x60c039f => {
  if !cpu.zf { pc = 0x60c03a3; continue; }
  pc = 0x60c03a1; continue;
 }
 0x60c03a1 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  pc = 0x60c03a3; continue;
 }
 0x60c03a3 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x800u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if !cpu.cf { pc = 0x60c03b4; continue; }
  pc = 0x60c03ad; continue;
 }
 0x60c03ad => {
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.inc16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c03b4; continue;
 }
 0x60c03b4 => {
  { let v: u32 = cpu.eax & cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16) & 0x7fffu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0xc400u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x7ffu16); }
  if !cpu.cf { pc = 0x60c03ea; continue; }
  pc = 0x60c03c9; continue;
 }
 0x60c03c9 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60c03d9; continue; }
  pc = 0x60c03ce; continue;
 }
 0x60c03ce => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 12u32); cpu.edx = v; }
  pc = 0x60c03e3; continue;
 }
 0x60c03d9 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.edx, cpu.ecx, 11u32); cpu.edx = v; }
  pc = 0x60c03e3; continue;
 }
 0x60c03e3 => {
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  pc = 0x60c044a; continue;
 }
 0x60c03ea => {
  { cpu.sub16((cpu.ecx as u16), 0xc400u16); }
  if cpu.cf { pc = 0x60c0424; continue; }
  pc = 0x60c03f1; continue;
 }
 0x60c03f1 => {
  { cpu.sub16((cpu.ecx as u16), 0xffccu16); }
  if (cpu.sf != cpu.of) { pc = 0x60c041e; continue; }
  pc = 0x60c03f7; continue;
 }
 0x60c03f7 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0xcu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x20u8); }
  if cpu.cf { pc = 0x60c040c; continue; }
  pc = 0x60c0403; continue;
 }
 0x60c0403 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x20u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x60c040c; continue;
 }
 0x60c040c => {
  { let v: u32 = cpu.shrd32(cpu.esi, cpu.eax, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x60c0422; continue;
 }
 0x60c041e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x60c0422; continue;
 }
 0x60c0422 => {
  pc = 0x60c044a; continue;
 }
 0x60c0424 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 11u32); cpu.edx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), (cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | 0x7ff00000u32; cpu.edx = v; }
  { cpu.sub16((cpu.ecx as u16), 0x43ffu16); }
  if cpu.zf { pc = 0x60c044a; continue; }
  pc = 0x60c043f; continue;
 }
 0x60c043f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x84u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x60c0449); f_060be710(cpu); if cpu.returned_to != 0x60c0449 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x60c044a; continue;
 }
 0x60c044a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c044e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c044e); }
 let mut pc: u32 = 0x60c044e;
 loop { match pc {
 0x60c044e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 23u32); cpu.ecx = v; }
  { let v: u16 = cpu.and16((cpu.ecx as u16), 0xffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60c0487; continue; }
  pc = 0x60c0460; continue;
 }
 0x60c0460 => {
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { cpu.sub8((cpu.ecx as u8), 0xffu8); }
  if cpu.zf { pc = 0x60c046f; continue; }
  pc = 0x60c0468; continue;
 }
 0x60c0468 => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3f80u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c047d; continue;
 }
 0x60c046f => {
  { let v: u8 = 0xffu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60c047d; continue; }
  pc = 0x60c0478; continue;
 }
 0x60c0478 => {
  { let v: u32 = cpu.eax | 0x40000000u32; cpu.eax = v; }
  pc = 0x60c047d; continue;
 }
 0x60c047d => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.rcr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.eax, 0x80000000u32); cpu.eax = v; }
  pc = 0x60c0487; continue;
 }
 0x60c0487 => {
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
pub fn f_060c0513(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0513); }
 let mut pc: u32 = 0x60c0513;
 loop { match pc {
 0x60c0513 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x60c0528; continue; }
  pc = 0x60c0523; continue;
 }
 0x60c0523 => {
  { cpu.sub16((cpu.edx as u16), 0x800u16); }
  pc = 0x60c0528; continue;
 }
 0x60c0528 => {
  if !cpu.zf { pc = 0x60c0533; continue; }
  pc = 0x60c052a; continue;
 }
 0x60c052a => {
  { let v: u8 = cpu.or8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.sf { pc = 0x60c0533; continue; }
  pc = 0x60c052e; continue;
 }
 0x60c052e => {
  { let v: u16 = (cpu.edx as u16) ^ 0xc00u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x60c0533; continue;
 }
 0x60c0533 => {
  { cpu.sub16((cpu.edx as u16), 0xc00u16); }
  if !cpu.zf { pc = 0x60c053e; continue; }
  pc = 0x60c053a; continue;
 }
 0x60c053a => {
  { let v: u16 = 0x400u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x60c053e; continue;
 }
 0x60c053e => {
  { let v: u16 = (cpu.ebx as u16) & 0x7fffu16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sub16((cpu.ebx as u16), 0x403fu16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x60c054c; continue; }
  pc = 0x60c054a; continue;
 }
 0x60c054a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c054c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x60c05a2; continue; }
  pc = 0x60c0554; continue;
 }
 0x60c0554 => {
  { let v: u32 = cpu.r32(0x60c0493u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ecx & cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)) ^ cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x60c05a0; continue; }
  pc = 0x60c0568; continue;
 }
 0x60c0568 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x60c058d; continue; }
  pc = 0x60c056e; continue;
 }
 0x60c056e => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c05a0; continue; }
  pc = 0x60c0578; continue;
 }
 0x60c0578 => {
  if !cpu.zf { pc = 0x60c058d; continue; }
  pc = 0x60c057a; continue;
 }
 0x60c057a => {
  { let v: u32 = cpu.rol32(cpu.eax, 1u32); cpu.eax = v; }
  if !cpu.cf { pc = 0x60c0583; continue; }
  pc = 0x60c057e; continue;
 }
 0x60c057e => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x60c0585; continue;
 }
 0x60c0583 => {
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x60c0585; continue;
 }
 0x60c0585 => {
  if cpu.zf { pc = 0x60c05a0; continue; }
  pc = 0x60c058b; continue;
 }
 0x60c058b => {
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60c058d; continue;
 }
 0x60c058d => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60c05a0; continue; }
  pc = 0x60c0591; continue;
 }
 0x60c0591 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x60c05a0; continue; }
  pc = 0x60c0599; continue;
 }
 0x60c0599 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60c05a0; continue;
 }
 0x60c05a0 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c05a2 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u16 = cpu.add16((cpu.ebx as u16), 0x20u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x60c0605; continue; }
  pc = 0x60c05b2; continue;
 }
 0x60c05b2 => {
  { let v: u32 = cpu.r32(0x60c0493u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi & cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)) ^ cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x60c0602; continue; }
  pc = 0x60c05c7; continue;
 }
 0x60c05c7 => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x60c05f2; continue; }
  pc = 0x60c05cd; continue;
 }
 0x60c05cd => {
  { let v: u32 = cpu.rol32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c0602; continue; }
  pc = 0x60c05d9; continue;
 }
 0x60c05d9 => {
  if !cpu.zf { pc = 0x60c05f2; continue; }
  pc = 0x60c05db; continue;
 }
 0x60c05db => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x60c05f6; continue; }
  pc = 0x60c05e3; continue;
 }
 0x60c05e3 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x60c05ea; continue; }
  pc = 0x60c05e7; continue;
 }
 0x60c05e7 => {
  { cpu.and32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); }
  pc = 0x60c05ea; continue;
 }
 0x60c05ea => {
  if cpu.zf { pc = 0x60c0602; continue; }
  pc = 0x60c05f0; continue;
 }
 0x60c05f0 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60c05f2; continue;
 }
 0x60c05f2 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.esi); cpu.ecx = v; }
  if cpu.zf { pc = 0x60c0602; continue; }
  pc = 0x60c05f6; continue;
 }
 0x60c05f6 => {
  { let v: u32 = cpu.add32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), cpu.eax); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  if !cpu.cf { pc = 0x60c0602; continue; }
  pc = 0x60c05fb; continue;
 }
 0x60c05fb => {
  { let v: u32 = cpu.rcr32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 1u32); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.inc16(cpu.r16(0x8u32.wrapping_add(cpu.edi))); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60c0602; continue;
 }
 0x60c0602 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c0605 => {
  { cpu.sub16((cpu.edx as u16), 0x400u16); }
  if cpu.zf { pc = 0x60c063a; continue; }
  pc = 0x60c060c; continue;
 }
 0x60c060c => {
  { cpu.sub16((cpu.edx as u16), 0x0u16); }
  if !cpu.zf { pc = 0x60c062a; continue; }
  pc = 0x60c0612; continue;
 }
 0x60c0612 => {
  { let v: u16 = cpu.inc16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60c063a; continue; }
  pc = 0x60c061a; continue;
 }
 0x60c061a => {
  { cpu.sub32(cpu.esi, 0x80000000u32); }
  if !cpu.zf { pc = 0x60c062a; continue; }
  pc = 0x60c0622; continue;
 }
 0x60c0622 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60c063a; continue; }
  pc = 0x60c062a; continue;
 }
 0x60c062a => {
  { let v: u32 = 0x80000000u32; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = cpu.or16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c063a => {
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
pub fn f_060c0646(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0646); }
 let mut pc: u32 = 0x60c0646;
 loop { match pc {
 0x60c0646 => {
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
  cpu.unimplemented(0x60c0663, "Aas");
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(cpu.r8(0xb99d0a6bu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.r32(0x171bffbu32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0xe38e2727u32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  {  }
  cpu.unimplemented(0x60c0677, "Aas");
  { cpu.df = false; }
  { let v = 0x2472dd9cu32; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { cpu.df = false; }
  { let v: u32 = 0x94ebadc1u32; cpu.edi = v; }
  cpu.unimplemented(0x60c0686, "Int3");
  cpu.unimplemented(0x60c0687, "Int3");
  cpu.unimplemented(0x60c0688, "Int3");
  cpu.unimplemented(0x60c0689, "Int3");
  { cpu.df = false; }
  cpu.unimplemented(0x60c068b, "Aas");
  { let v: u32 = cpu.adc32(cpu.r32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0xbffdaau32); cpu.w32(0xaaaaaaaau32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x3fu8); }
  cpu.flat_ds(0x60c069d); f_060c06a0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c06a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c06a0); }
 let mut pc: u32 = 0x60c06a0;
 loop { match pc {
 0x60c06a0 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60c06de; continue; }
  pc = 0x60c06ae; continue;
 }
 0x60c06ae => {
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x60c06c9; continue; }
  pc = 0x60c06b4; continue;
 }
 0x60c06b4 => {
  { let v: u16 = 0x4000u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c06dc; continue;
 }
 0x60c06c9 => {
  { let v: u16 = 0x0u16; cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c06dc; continue;
 }
 0x60c06dc => {
  pc = 0x60c0752; continue;
 }
 0x60c06de => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60c0706; continue; }
  pc = 0x60c06e7; continue;
 }
 0x60c06e7 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x8000u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0xc90fdaa2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x2168c235u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c0752; continue;
 }
 0x60c0706 => {
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.push(0x60c070e); f_060bfe83(cpu); if cpu.returned_to != 0x60c070e { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60c0715); f_060c0756(cpu); if cpu.returned_to != 0x60c0715 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.sf { pc = 0x60c0738; continue; }
  pc = 0x60c071c; continue;
 }
 0x60c071c => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if !cpu.sf { pc = 0x60c0736; continue; }
  pc = 0x60c0720; continue;
 }
 0x60c0720 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x4000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x60c0736); f_060bfc4e(cpu); if cpu.returned_to != 0x60c0736 { return; }
  pc = 0x60c0736; continue;
 }
 0x60c0736 => {
  pc = 0x60c0752; continue;
 }
 0x60c0738 => {
  { let v: u8 = cpu.or8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c0752; continue; }
  pc = 0x60c073c; continue;
 }
 0x60c073c => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xc000u32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x60c0752); f_060bfc4e(cpu); if cpu.returned_to != 0x60c0752 { return; }
  pc = 0x60c0752; continue;
 }
 0x60c0752 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0756(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0756); }
 let mut pc: u32 = 0x60c0756;
 loop { match pc {
 0x60c0756 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.edi)) & 0x7fu8; cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if !cpu.zf { pc = 0x60c0798; continue; }
  pc = 0x60c076c; continue;
 }
 0x60c076c => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x80000000u32); }
  if !cpu.zf { pc = 0x60c0798; continue; }
  pc = 0x60c0775; continue;
 }
 0x60c0775 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x60c0798; continue; }
  pc = 0x60c077a; continue;
 }
 0x60c077a => {
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
 0x60c0798 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3fffu16); }
  if (cpu.sf != cpu.of) { pc = 0x60c07ce; continue; }
  pc = 0x60c07a2; continue;
 }
 0x60c07a2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.push(0x60c07c2); f_060bfead(cpu); if cpu.returned_to != 0x60c07c2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = 0x2u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60c07ce; continue;
 }
 0x60c07ce => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x3ffdu16); }
  if !cpu.zf { pc = 0x60c07e9; continue; }
  pc = 0x60c07d8; continue;
 }
 0x60c07d8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8930a2f4u32); }
  if !cpu.zf { pc = 0x60c07e9; continue; }
  pc = 0x60c07e2; continue;
 }
 0x60c07e2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xf66ab09bu32); }
  pc = 0x60c07e9; continue;
 }
 0x60c07e9 => {
  if (cpu.cf || cpu.zf) { pc = 0x60c085c; continue; }
  pc = 0x60c07eb; continue;
 }
 0x60c07eb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x60c0809); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0809 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xc265539eu32; cpu.ebx = v; }
  { let v: u32 = 0xddb3d742u32; cpu.ecx = v; }
  { let v: u32 = 0x3fff0000u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  cpu.push(0x60c0829); f_060c0082(cpu); if cpu.returned_to != 0x60c0829 { return; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0xbfffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x60c083f); f_060bfc7e(cpu); if cpu.returned_to != 0x60c083f { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.esi as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c084f); f_060bfead(cpu); if cpu.returned_to != 0x60c084f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x1u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60c085c; continue;
 }
 0x60c085c => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x60c0646u32; cpu.edx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.push(0x60c086e); f_060c12d2(cpu); if cpu.returned_to != 0x60c086e { return; }
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x60c088a; continue; }
  pc = 0x60c0872; continue;
 }
 0x60c0872 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0x860a91c1u32; cpu.push(v); }
  { let v = 0x6b9b2c23u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60c088a); f_060bfc4e(cpu); if cpu.returned_to != 0x60c088a { return; }
  pc = 0x60c088a; continue;
 }
 0x60c088a => {
  { let v: u8 = cpu.shr8(((cpu.ecx >> 8) as u8), 1u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x60c08aa; continue; }
  pc = 0x60c088e; continue;
 }
 0x60c088e => {
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60c08a6); f_060bfc4e(cpu); if cpu.returned_to != 0x60c08a6 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c08aa; continue;
 }
 0x60c08aa => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if !cpu.sf { pc = 0x60c08b3; continue; }
  pc = 0x60c08af; continue;
 }
 0x60c08af => {
  { let v: u8 = cpu.xor8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); cpu.w8(0x9u32.wrapping_add(cpu.edi), v); }
  pc = 0x60c08b3; continue;
 }
 0x60c08b3 => {
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
pub fn f_060c08b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c08b9); }
 let mut pc: u32 = 0x60c08b9;
 loop { match pc {
 0x60c08b9 => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.eax)), 0x0u16); }
  if !cpu.zf { pc = 0x60c08ce; continue; }
  pc = 0x60c08c0; continue;
 }
 0x60c08c0 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60c08ce; continue; }
  pc = 0x60c08c6; continue;
 }
 0x60c08c6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60c08ce; continue; }
  pc = 0x60c08cb; continue;
 }
 0x60c08cb => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c08ce => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edx)), 0x0u16); }
  if !cpu.zf { pc = 0x60c08ee; continue; }
  pc = 0x60c08d5; continue;
 }
 0x60c08d5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c08ee; continue; }
  pc = 0x60c08db; continue;
 }
 0x60c08db => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c08ee; continue; }
  pc = 0x60c08e0; continue;
 }
 0x60c08e0 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c08ee => {
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
  cpu.push(0x60c090c); f_060c091f(cpu); if cpu.returned_to != 0x60c090c { return; }
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
pub fn f_060c091f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c091f); }
 let mut pc: u32 = 0x60c091f;
 loop { match pc {
 0x60c091f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi & 0x7fffu32; cpu.esi = v; }
  { let v: u32 = cpu.edi & 0x7fffu32; cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.edi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x60c097e; continue; }
  pc = 0x60c0931; continue;
 }
 0x60c0931 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x60c0933; continue;
 }
 0x60c0933 => {
  { cpu.sub32(cpu.ecx, cpu.edx); }
  if !cpu.zf { pc = 0x60c093f; continue; }
  pc = 0x60c0937; continue;
 }
 0x60c0937 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.zf { pc = 0x60c0941; continue; }
  pc = 0x60c093f; continue;
 }
 0x60c093f => {
  if !cpu.cf { pc = 0x60c0946; continue; }
  pc = 0x60c0941; continue;
 }
 0x60c0941 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { cpu.cf = true; }
  pc = 0x60c0946; continue;
 }
 0x60c0946 => {
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  if (cpu.sf != cpu.of) { pc = 0x60c095b; continue; }
  pc = 0x60c094f; continue;
 }
 0x60c094f => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.cf { cpu.poll(); pc = 0x60c0941; continue; }
  pc = 0x60c0955; continue;
 }
 0x60c0955 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { cpu.poll(); pc = 0x60c0933; continue; }
  pc = 0x60c0959; continue;
 }
 0x60c0959 => {
  cpu.poll(); pc = 0x60c0946; continue;
 }
 0x60c095b => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60c0967; continue; }
  pc = 0x60c0961; continue;
 }
 0x60c0961 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c0967; continue; }
  pc = 0x60c0965; continue;
 }
 0x60c0965 => {
  pc = 0x60c097c; continue;
 }
 0x60c0967 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x60c0972; continue; }
  pc = 0x60c096b; continue;
 }
 0x60c096b => {
  { let v: u32 = cpu.adc32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60c0967; continue;
 }
 0x60c0972 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx & 0x7fffu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x60c097c; continue;
 }
 0x60c097c => {
  pc = 0x60c0982; continue;
 }
 0x60c097e => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x60c0982; continue;
 }
 0x60c0982 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi | cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c0992; continue; }
  pc = 0x60c0990; continue;
 }
 0x60c0990 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x60c0992; continue;
 }
 0x60c0992 => {
  { let v: u16 = cpu.xor16((cpu.ebx as u16), (cpu.ecx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x60c0999; continue; }
  pc = 0x60c0997; continue;
 }
 0x60c0997 => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x60c0999; continue;
 }
 0x60c0999 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c099a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c099a); }
 let mut pc: u32 = 0x60c099a;
 loop { match pc {
 0x60c099a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x60be156u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x60c09b8; continue; }
  pc = 0x60c09b2; continue;
 }
 0x60c09b2 => {
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60c0a2b; continue;
 }
 0x60c09b8 => {
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) & (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.rol8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.shl8(((cpu.edx >> 8) as u8), 1u32); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.and16((cpu.eax as u16), 0x7fffu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60c09ec; continue; }
  pc = 0x60c09d2; continue;
 }
 0x60c09d2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x60c09ea; continue; }
  pc = 0x60c09db; continue;
 }
 0x60c09db => {
  { let v: u16 = cpu.r16(0x60be150u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c09ea; continue;
 }
 0x60c09ea => {
  pc = 0x60c0a2b; continue;
 }
 0x60c09ec => {
  { cpu.sub16((cpu.eax as u16), 0x7fffu16); }
  if !cpu.zf { pc = 0x60c0a14; continue; }
  pc = 0x60c09f2; continue;
 }
 0x60c09f2 => {
  { let v: u16 = cpu.r16(0x60be154u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c0a0c; continue; }
  pc = 0x60c0a04; continue;
 }
 0x60c0a04 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  pc = 0x60c0a0c; continue;
 }
 0x60c0a0c => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x60c0a12; continue; }
  pc = 0x60c0a10; continue;
 }
 0x60c0a10 => {
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c0a12; continue;
 }
 0x60c0a12 => {
  pc = 0x60c0a2b; continue;
 }
 0x60c0a14 => {
  { cpu.and8(cpu.r8(0x7u32.wrapping_add(cpu.ebx)), 0x80u8); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x60c0a2b; continue; }
  pc = 0x60c0a1c; continue;
 }
 0x60c0a1c => {
  { let v: u16 = cpu.r16(0x60be154u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)) | (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c0a2b; continue;
 }
 0x60c0a2b => {
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
pub fn f_060c0a42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0a42); }
 let mut pc: u32 = 0x60c0a42;
 loop { match pc {
 0x60c0a42 => {
  cpu.call(0xa0c1f38d); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0a60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c0a60); } r_060c0a60(cpu, 0x60c0a60); }
pub fn f_060c0aa1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c0aa1); } r_060c0a60(cpu, 0x60c0aa1); }
fn r_060c0a60(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c0a60 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x76bb3fffu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x76bb3fffu32.wrapping_add(cpu.eax), v); }
  if cpu.of { pc = 0x60c0aac; continue; }
  pc = 0x60c0a6e; continue;
 }
 0x60c0a6e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xef60c004u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.r32(0x9c041fd0u32.wrapping_add(cpu.ecx))); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = 0xf4e06c83u32; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u8 = cpu.ror8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 192u32); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_060c0a88(cpu); return;
 }
 0x60c0aa1 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60c0aac); f_060c0ae4(cpu); if cpu.returned_to != 0x60c0aac { return; }
  pc = 0x60c0aac; continue;
 }
 0x60c0aac => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60c0ab5); f_060c0058(cpu); if cpu.returned_to != 0x60c0ab5 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = 0x3fffu16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xb8aa3b29u32; cpu.edx = v; }
  { let v: u32 = 0x5c17f0bdu32; cpu.eax = v; }
  cpu.push(0x60c0ad5); f_060c0082(cpu); if cpu.returned_to != 0x60c0ad5 { return; }
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
pub fn f_060c0a88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0a88); }
 let mut pc: u32 = 0x60c0a88;
 loop { match pc {
 0x60c0a88 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3fffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x60c0a9e); f_060bfc4e(cpu); if cpu.returned_to != 0x60c0a9e { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  f_060c0aa1(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0ae4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0ae4); }
 let mut pc: u32 = 0x60c0ae4;
 loop { match pc {
 0x60c0ae4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x60c0af6; continue; }
  pc = 0x60c0af4; continue;
 }
 0x60c0af4 => {
  pc = 0x60c0af6; continue;
 }
 0x60c0af6 => {
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
  cpu.push(0x60c0b1e); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0b1e { return; }
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), 0x3ffeu16); }
  if !cpu.zf { pc = 0x60c0b35; continue; }
  pc = 0x60c0b26; continue;
 }
 0x60c0b26 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0xb504f333u32); }
  if !cpu.zf { pc = 0x60c0b35; continue; }
  pc = 0x60c0b2f; continue;
 }
 0x60c0b2f => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0xf9de6484u32); }
  pc = 0x60c0b35; continue;
 }
 0x60c0b35 => {
  if (cpu.cf || cpu.zf) { pc = 0x60c0b51; continue; }
  pc = 0x60c0b37; continue;
 }
 0x60c0b37 => {
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbffe0000u32; cpu.esi = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x60c0b4f); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0b4f { return; }
  pc = 0x60c0b5d; continue;
 }
 0x60c0b51 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c0b5d; continue;
 }
 0x60c0b5d => {
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
  cpu.push(0x60c0b83); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0b83 { return; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.push(0x60c0b95); f_060bfead(cpu); if cpu.returned_to != 0x60c0b95 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x60c0ba9); f_060c0082(cpu); if cpu.returned_to != 0x60c0ba9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v: u32 = 0x60c0a60u32; cpu.ecx = v; }
  cpu.push(0x60c0bc2); f_060c1280(cpu); if cpu.returned_to != 0x60c0bc2 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = 0x60c0a42u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  cpu.push(0x60c0bde); f_060c1280(cpu); if cpu.returned_to != 0x60c0bde { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c0bef); f_060bfead(cpu); if cpu.returned_to != 0x60c0bef { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x60c0c03); f_060c0082(cpu); if cpu.returned_to != 0x60c0c03 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x60c0c1a); f_060c0082(cpu); if cpu.returned_to != 0x60c0c1a { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c0c2b); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0c2b { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60c0ca6; continue; }
  pc = 0x60c0c30; continue;
 }
 0x60c0c30 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60c0c3c); f_060c0242(cpu); if cpu.returned_to != 0x60c0c3c { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0xbff20000u32; cpu.esi = v; }
  { let v: u32 = 0xde8082e3u32; cpu.ecx = v; }
  { let v: u32 = 0x865435cu32; cpu.ebx = v; }
  cpu.push(0x60c0c60); f_060c0082(cpu); if cpu.returned_to != 0x60c0c60 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c0c71); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0c71 { return; }
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
  cpu.push(0x60c0c95); f_060c0082(cpu); if cpu.returned_to != 0x60c0c95 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c0ca6); f_060bfc7e(cpu); if cpu.returned_to != 0x60c0ca6 { return; }
  pc = 0x60c0ca6; continue;
 }
 0x60c0ca6 => {
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
pub fn f_060c0cb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0cb6); }
 let mut pc: u32 = 0x60c0cb6;
 loop { match pc {
 0x60c0cb6 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0xffffff85u32; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x3fceca96u32); }
  cpu.call(0x60c0cc0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0d6a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c0d6a); } r_060c0d6a(cpu, 0x60c0d6a); }
pub fn f_060c0d7c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c0d7c); } r_060c0d6a(cpu, 0x60c0d7c); }
fn r_060c0d6a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c0d6a => {
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
  if (cpu.sf != cpu.of) { pc = 0x60c0d9e; continue; }
  pc = 0x60c0d9e; continue;
 }
 0x60c0d7c => {
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
  if (cpu.sf != cpu.of) { pc = 0x60c0d9e; continue; }
  pc = 0x60c0d9e; continue;
 }
 0x60c0d9e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = 0x3ffeu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = 0xc90fdaa2u32; cpu.ecx = v; }
  { let v: u32 = 0x2168c235u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x60c0dbd); f_060c091f(cpu); if cpu.returned_to != 0x60c0dbd { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and8(cpu.r8(0x9u32.wrapping_add(cpu.edi)), 0x80u8); }
  if cpu.zf { pc = 0x60c0ded; continue; }
  pc = 0x60c0dd2; continue;
 }
 0x60c0dd2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x3ffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  cpu.push(0x60c0deb); f_060bfc4e(cpu); if cpu.returned_to != 0x60c0deb { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x60c0ded; continue;
 }
 0x60c0ded => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.ebx as u8) & 0x2u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c0e19; continue; }
  pc = 0x60c0dfd; continue;
 }
 0x60c0dfd => {
  { let v = 0xbffeu32; cpu.push(v); }
  { let v = 0xc90fdaa2u32; cpu.push(v); }
  { let v = 0x2168c235u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60c0e15); f_060bfc4e(cpu); if cpu.returned_to != 0x60c0e15 { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c0e19; continue;
 }
 0x60c0e19 => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3ffeu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0xffe0u16); }
  if (cpu.sf == cpu.of) { pc = 0x60c0e43; continue; }
  pc = 0x60c0e2a; continue;
 }
 0x60c0e2a => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x3fffu16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c0eda; continue;
 }
 0x60c0e43 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.sf { pc = 0x60c0e4c; continue; }
  pc = 0x60c0e48; continue;
 }
 0x60c0e48 => {
  { let v: u16 = 0x0u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c0e4c; continue;
 }
 0x60c0e4c => {
  { let v: u16 = cpu.neg16((cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.ecx as u16), 0x8u16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c0e59; continue; }
  pc = 0x60c0e55; continue;
 }
 0x60c0e55 => {
  { let v: u16 = 0x8u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60c0e59; continue;
 }
 0x60c0e59 => {
  { let v: u16 = cpu.r16(0x60c0d6au32.wrapping_add(cpu.ecx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if !cpu.zf { pc = 0x60c0e73; continue; }
  pc = 0x60c0e6c; continue;
 }
 0x60c0e6c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c0e73; continue; }
  pc = 0x60c0e71; continue;
 }
 0x60c0e71 => {
  pc = 0x60c0eb1; continue;
 }
 0x60c0e73 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x60c0e8d); f_060c0082(cpu); if cpu.returned_to != 0x60c0e8d { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = 0x60c0d10u32.wrapping_add(cpu.ecx.wrapping_mul(2)); cpu.ecx = v; }
  cpu.push(0x60c0ea6); f_060c1280(cpu); if cpu.returned_to != 0x60c0ea6 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x60c0eb1; continue;
 }
 0x60c0eb1 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c0ebc; continue; }
  pc = 0x60c0eb5; continue;
 }
 0x60c0eb5 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c0ebc; continue; }
  pc = 0x60c0eba; continue;
 }
 0x60c0eba => {
  pc = 0x60c0eda; continue;
 }
 0x60c0ebc => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x60c0cb6u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  cpu.push(0x60c0eda); f_060c12d2(cpu); if cpu.returned_to != 0x60c0eda { return; }
  pc = 0x60c0eda; continue;
 }
 0x60c0eda => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.eax as u8), 0x2u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x60c0eef; continue; }
  pc = 0x60c0ee3; continue;
 }
 0x60c0ee3 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0ef8; continue;
 }
 0x60c0eef => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0ef8; continue;
 }
 0x60c0ef8 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c0f05; continue; }
  pc = 0x60c0f00; continue;
 }
 0x60c0f00 => {
  { let v: u16 = (cpu.esi as u16) ^ 0x8000u16; cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0f05; continue;
 }
 0x60c0f05 => {
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x60c0f62; continue; }
  pc = 0x60c0f0a; continue;
 }
 0x60c0f0a => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x2u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  if cpu.zf { pc = 0x60c0f28; continue; }
  pc = 0x60c0f19; continue;
 }
 0x60c0f19 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0f34; continue;
 }
 0x60c0f28 => {
  { let v: u8 = cpu.and8((cpu.ecx as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0f34; continue;
 }
 0x60c0f34 => {
  if cpu.zf { pc = 0x60c0f3c; continue; }
  pc = 0x60c0f36; continue;
 }
 0x60c0f36 => {
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  pc = 0x60c0f3c; continue;
 }
 0x60c0f3c => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x60c0f5d; continue; }
  pc = 0x60c0f43; continue;
 }
 0x60c0f43 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x60c0f5d; continue; }
  pc = 0x60c0f47; continue;
 }
 0x60c0f47 => {
  { cpu.and32(cpu.esi, 0x7fff0000u32); }
  if !cpu.zf { pc = 0x60c0f5d; continue; }
  pc = 0x60c0f4f; continue;
 }
 0x60c0f4f => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x80000000u32; cpu.edx = v; }
  { let v: u16 = cpu.or16((cpu.esi as u16), 0x7fffu16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  pc = 0x60c0f62; continue;
 }
 0x60c0f5d => {
  cpu.push(0x60c0f62); f_060bfead(cpu); if cpu.returned_to != 0x60c0f62 { return; }
  pc = 0x60c0f62; continue;
 }
 0x60c0f62 => {
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
pub fn f_060c0f73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0f73); }
 let mut pc: u32 = 0x60c0f73;
 loop { match pc {
 0x60c0f73 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x60c0f7b); f_060c0d7c(cpu); if cpu.returned_to != 0x60c0f7b { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0f7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0f7d); }
 let mut pc: u32 = 0x60c0f7d;
 loop { match pc {
 0x60c0f7d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.esi = v; }
  cpu.push(0x60c0f88); f_060c0d7c(cpu); if cpu.returned_to != 0x60c0f88 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0f8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0f8a); }
 let mut pc: u32 = 0x60c0f8a;
 loop { match pc {
 0x60c0f8a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.push(0x60c0f95); f_060c0d7c(cpu); if cpu.returned_to != 0x60c0f95 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0f98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0f98); }
 let mut pc: u32 = 0x60c0f98;
 loop { match pc {
 0x60c0f98 => {
  { let v: u32 = cpu.adc32(cpu.eax, 0x67cc487bu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c0fc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c0fc0); }
 let mut pc: u32 = 0x60c0fc0;
 loop { match pc {
 0x60c0fc0 => {
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
  if !cpu.zf { pc = 0x60c0fe2; continue; }
  pc = 0x60c0fd9; continue;
 }
 0x60c0fd9 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edi)), 0x0u32); }
  if !cpu.zf { pc = 0x60c0fe2; continue; }
  pc = 0x60c0fdf; continue;
 }
 0x60c0fdf => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edi)), 0x0u32); }
  pc = 0x60c0fe2; continue;
 }
 0x60c0fe2 => {
  if cpu.zf { pc = 0x60c104a; continue; }
  pc = 0x60c0fe4; continue;
 }
 0x60c0fe4 => {
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if !cpu.sf { pc = 0x60c1003; continue; }
  pc = 0x60c0fe9; continue;
 }
 0x60c0fe9 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60c0fff); f_060bfc4e(cpu); if cpu.returned_to != 0x60c0fff { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.ebx)) ^ 0x80u8; cpu.w8(0x9u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c1003; continue;
 }
 0x60c1003 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16).wrapping_add(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  pc = 0x60c1012; continue;
 }
 0x60c1012 => {
  { cpu.sub16((cpu.ebx as u16), 0x3fffu16); }
  if cpu.cf { pc = 0x60c1023; continue; }
  pc = 0x60c1019; continue;
 }
 0x60c1019 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60c1012; continue;
 }
 0x60c1023 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c1029; continue; }
  pc = 0x60c1027; continue;
 }
 0x60c1027 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c1029; continue;
 }
 0x60c1029 => {
  if !cpu.zf { pc = 0x60c102f; continue; }
  pc = 0x60c102b; continue;
 }
 0x60c102b => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  pc = 0x60c103f; continue;
 }
 0x60c102f => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if cpu.sf { pc = 0x60c103b; continue; }
  pc = 0x60c1033; continue;
 }
 0x60c1033 => {
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.dec16((cpu.ebx as u16)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60c102f; continue;
 }
 0x60c103b => {
  { let v: u16 = (cpu.ebx as u16).wrapping_sub(0x4u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  pc = 0x60c103f; continue;
 }
 0x60c103f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60c104c; continue;
 }
 0x60c104a => {
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x60c104c; continue;
 }
 0x60c104c => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.push(0x60c1067); f_060c0082(cpu); if cpu.returned_to != 0x60c1067 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0xa6829a79u32; cpu.ecx = v; }
  { let v: u32 = 0x1bf21f8cu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40030000u32; cpu.esi = v; }
  cpu.push(0x60c1085); f_060bfc7e(cpu); if cpu.returned_to != 0x60c1085 { return; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = 0xec96f0d6u32; cpu.ecx = v; }
  { let v: u32 = 0x9d7bfdbu32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x3ffa0000u32; cpu.esi = v; }
  cpu.push(0x60c10ab); f_060c0082(cpu); if cpu.returned_to != 0x60c10ab { return; }
  { let v: u32 = 0xe6d5051au32; cpu.ecx = v; }
  { let v: u32 = 0xe536e187u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffffu32; cpu.esi = v; }
  { let v: u32 = cpu.esi | 0x40010000u32; cpu.esi = v; }
  cpu.push(0x60c10c6); f_060bfc7e(cpu); if cpu.returned_to != 0x60c10c6 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x60c10da); f_060c0082(cpu); if cpu.returned_to != 0x60c10da { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esi ^ 0x8000u32; cpu.esi = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x60c10f8); f_060bfc7e(cpu); if cpu.returned_to != 0x60c10f8 { return; }
  { let a = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.edx = v; } }
  { let a = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); let b = cpu.esi; { let v: u32 = b; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.esi = v; } }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x60c1116); f_060bfc7e(cpu); if cpu.returned_to != 0x60c1116 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c1128); f_060bfead(cpu); if cpu.returned_to != 0x60c1128 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = 0x60c0f98u32; cpu.esi = v; }
  pc = 0x60c113b; continue;
 }
 0x60c113b => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60c1173; continue; }
  pc = 0x60c113f; continue;
 }
 0x60c113f => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  if !cpu.cf { pc = 0x60c116e; continue; }
  pc = 0x60c1143; continue;
 }
 0x60c1143 => {
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
  cpu.push(0x60c1163); f_060c0082(cpu); if cpu.returned_to != 0x60c1163 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  pc = 0x60c116e; continue;
 }
 0x60c116e => {
  { let v: u32 = 0xau32.wrapping_add(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60c113b; continue;
 }
 0x60c1173 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u16 = cpu.add16(cpu.r16(0x8u32.wrapping_add(cpu.edi)), (cpu.esi as u16)); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0xbfffu32; cpu.push(v); }
  { let v = 0x80000000u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60c118d); f_060bfc4e(cpu); if cpu.returned_to != 0x60c118d { return; }
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
pub fn f_060c1194(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1194); }
 let mut pc: u32 = 0x60c1194;
 loop { match pc {
 0x60c1194 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { pc = 0x60c11da; continue; }
  pc = 0x60c11a6; continue;
 }
 0x60c11a6 => {
  { let v: u32 = cpu.or32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.zf { pc = 0x60c11cf; continue; }
  pc = 0x60c11aa; continue;
 }
 0x60c11aa => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60c125a; continue; }
  pc = 0x60c11b3; continue;
 }
 0x60c11b3 => {
  { let v: u16 = 0xffffu16; cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c11c6; continue;
 }
 0x60c11c6 => {
  { let v: u8 = cpu.or8(cpu.r8(0x7u32.wrapping_add(cpu.eax)), 0xc0u8); cpu.w8(0x7u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c125a; continue;
 }
 0x60c11cf => {
  { cpu.sub16((cpu.ecx as u16), 0x7fffu16); }
  if cpu.zf { pc = 0x60c125a; continue; }
  pc = 0x60c11da; continue;
 }
 0x60c11da => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x7fu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub16((cpu.edx as u16), 0x7fffu16); }
  if cpu.zf { cpu.poll(); pc = 0x60c11c6; continue; }
  pc = 0x60c11e6; continue;
 }
 0x60c11e6 => {
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.ecx as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { cpu.poll(); pc = 0x60c11c6; continue; }
  pc = 0x60c11eb; continue;
 }
 0x60c11eb => {
  { let v: u16 = cpu.shr16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60c11f7; continue; }
  pc = 0x60c11f0; continue;
 }
 0x60c11f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.edx); cpu.edx = v; }
  if !cpu.cf { cpu.poll(); pc = 0x60c11b3; continue; }
  pc = 0x60c11f7; continue;
 }
 0x60c11f7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sar16((cpu.ecx as u16), 1u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.cf { pc = 0x60c120d; continue; }
  pc = 0x60c1207; continue;
 }
 0x60c1207 => {
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.edi, 1u32); cpu.edi = v; }
  pc = 0x60c120d; continue;
 }
 0x60c120d => {
  { let v: u16 = (cpu.ecx as u16).wrapping_add(0x3fffu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  if cpu.zf { pc = 0x60c1240; continue; }
  pc = 0x60c1222; continue;
 }
 0x60c1222 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x60c1223; continue;
 }
 0x60c1223 => {
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x60c1235; continue; }
  pc = 0x60c122a; continue;
 }
 0x60c122a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x60c1223; continue;
 }
 0x60c1235 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  pc = 0x60c1249; continue;
 }
 0x60c1240 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x60c1252; continue; }
  pc = 0x60c1244; continue;
 }
 0x60c1244 => {
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  pc = 0x60c1249; continue;
 }
 0x60c1249 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { cpu.cf = true; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.edx); cpu.eax = v; }
  pc = 0x60c1252; continue;
 }
 0x60c1252 => {
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x60c125a; continue;
 }
 0x60c125a => {
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
pub fn f_060c1260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1260); }
 let mut pc: u32 = 0x60c1260;
 loop { match pc {
 0x60c1260 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x60c1273); f_060c1280(cpu); if cpu.returned_to != 0x60c1273 { return; }
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
pub fn f_060c1280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1280); }
 let mut pc: u32 = 0x60c1280;
 loop { match pc {
 0x60c1280 => {
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
  pc = 0x60c1297; continue;
 }
 0x60c1297 => {
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.push(0x60c12ac); f_060c0082(cpu); if cpu.returned_to != 0x60c12ac { return; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x60c12c6); f_060bfc7e(cpu); if cpu.returned_to != 0x60c12c6 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x60c1297; continue; }
  pc = 0x60c12cb; continue;
 }
 0x60c12cb => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c12d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c12d2); }
 let mut pc: u32 = 0x60c12d2;
 loop { match pc {
 0x60c12d2 => {
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
  cpu.push(0x60c12f2); f_060c0082(cpu); if cpu.returned_to != 0x60c12f2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.push(0x60c12f9); f_060c1280(cpu); if cpu.returned_to != 0x60c12f9 { return; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.esi, 16u32); cpu.esi = v; }
  cpu.push(0x60c130d); f_060c0082(cpu); if cpu.returned_to != 0x60c130d { return; }
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
pub fn f_060c131a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c131a); }
 let mut pc: u32 = 0x60c131a;
 loop { match pc {
 0x60c131a => {
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
  if cpu.zf { pc = 0x60c138a; continue; }
  pc = 0x60c132d; continue;
 }
 0x60c132d => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x60c138a; continue; }
  pc = 0x60c1339; continue;
 }
 0x60c1339 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.eax = cpu.eax as u16 as i16 as i32 as u32; }
  { cpu.sub32(cpu.eax, 0x666u32); }
  if !cpu.zf { pc = 0x60c138a; continue; }
  pc = 0x60c134d; continue;
 }
 0x60c134d => {
  { let v: u8 = 0x22u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60c138a; continue; }
  pc = 0x60c135e; continue;
 }
 0x60c135e => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1eu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x617c805u32, v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u32 = 0x62d21b4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u8 = 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x2f); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c13e4; continue;
 }
 0x60c138a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x60c1396; continue; }
  pc = 0x60c1391; continue;
 }
 0x60c1391 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x60c13e2; continue; }
  pc = 0x60c1396; continue;
 }
 0x60c1396 => {
  { let v: u32 = 0xa00u32; cpu.eax = v; }
  { let v: u32 = 0x616b258u32; cpu.esi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c13e2; continue; }
  pc = 0x60c13aa; continue;
 }
 0x60c13aa => {
  { let v: u32 = 0x60be4dcu32; cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  cpu.push(0x60c13bb); f_060aea7e(cpu); if cpu.returned_to != 0x60c13bb { return; }
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
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x617c804u32, v); }
  pc = 0x60c13e2; continue;
 }
 0x60c13e2 => {
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c13e4; continue;
 }
 0x60c13e4 => {
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
pub fn f_060c13ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c13ee); }
 let mut pc: u32 = 0x60c13ee;
 loop { match pc {
 0x60c13ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x617c805u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x60c141d; continue; }
  pc = 0x60c13fd; continue;
 }
 0x60c13fd => {
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
 0x60c141d => {
  { cpu.sub8(cpu.r8(0x617c804u32), 0x0u8); }
  if cpu.zf { pc = 0x60c1441; continue; }
  pc = 0x60c1426; continue;
 }
 0x60c1426 => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x3eu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = (cpu.ebx as u8) & 0xfbu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x40u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = 0x11u32; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  {  }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x60c1441; continue;
 }
 0x60c1441 => {
  { let v: u8 = cpu.xor8((cpu.eax as u8), (cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1447(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1447); }
 let mut pc: u32 = 0x60c1447;
 loop { match pc {
 0x60c1447 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1448(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1448); }
 let mut pc: u32 = 0x60c1448;
 loop { match pc {
 0x60c1448 => {
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
  if cpu.zf { pc = 0x60c1462; continue; }
  pc = 0x60c1460; continue;
 }
 0x60c1460 => {
  { let v: u8 = 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c1462; continue;
 }
 0x60c1462 => {
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
pub fn f_060c146f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c146f); }
 let mut pc: u32 = 0x60c146f;
 loop { match pc {
 0x60c146f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60c1476; continue; }
  pc = 0x60c1473; continue;
 }
 0x60c1473 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c1475; continue;
 }
 0x60c1475 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c1476 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x60c1475; continue; }
  f_060c147a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c147a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c147a); }
 let mut pc: u32 = 0x60c147a;
 loop { match pc {
 0x60c147a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60c1489); f_060aecbe(cpu); if cpu.returned_to != 0x60c1489 { return; }
  { cpu.sub32(cpu.ebx, 0x100u32); }
  if !cpu.cf { pc = 0x60c14d4; continue; }
  pc = 0x60c1491; continue;
 }
 0x60c1491 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b867u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c14ba; continue; }
  pc = 0x60c149d; continue;
 }
 0x60c149d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x50u32); }
  if !cpu.zf { pc = 0x60c14aa; continue; }
  pc = 0x60c14a6; continue;
 }
 0x60c14a6 => {
  { let v: u8 = 0xeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c14ba; continue;
 }
 0x60c14aa => {
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c14b3; continue; }
  pc = 0x60c14af; continue;
 }
 0x60c14af => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c14ba; continue;
 }
 0x60c14b3 => {
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c14ba; continue; }
  pc = 0x60c14b8; continue;
 }
 0x60c14b8 => {
  { let v: u8 = 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c14ba; continue;
 }
 0x60c14ba => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x13u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c14c5; continue; }
  pc = 0x60c14c3; continue;
 }
 0x60c14c3 => {
  { let v: u8 = 0x13u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c14c5; continue;
 }
 0x60c14c5 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x617c811u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 24u32); cpu.eax = v; }
  pc = 0x60c14de; continue;
 }
 0x60c14d4 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  pc = 0x60c14de; continue;
 }
 0x60c14de => {
  cpu.push(0x60c14e3); f_060aec9b(cpu); if cpu.returned_to != 0x60c14e3 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c14eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c14eb); }
 let mut pc: u32 = 0x60c14eb;
 loop { match pc {
 0x60c14eb => {
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
pub fn f_060c1512(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1512); }
 let mut pc: u32 = 0x60c1512;
 loop { match pc {
 0x60c1512 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x617c82cu32)); }
  if cpu.cf { pc = 0x60c1525; continue; }
  pc = 0x60c151f; continue;
 }
 0x60c151f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c1525 => {
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c155b; continue; }
  pc = 0x60c152a; continue;
 }
 0x60c152a => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617c880u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if !cpu.zf { pc = 0x60c155b; continue; }
  pc = 0x60c153e; continue;
 }
 0x60c153e => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c154d); f_060c14eb(cpu); if cpu.returned_to != 0x60c154d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c155b; continue; }
  pc = 0x60c1551; continue;
 }
 0x60c1551 => {
  { let v: u32 = cpu.r32(0x617c880u32); cpu.eax = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0x20u8); cpu.w8(0x1u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x60c155b; continue;
 }
 0x60c155b => {
  { let v: u32 = cpu.r32(0x617c880u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1567(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1567); }
 let mut pc: u32 = 0x60c1567;
 loop { match pc {
 0x60c1567 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x40u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617c880u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c157c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c157c); }
 let mut pc: u32 = 0x60c157c;
 loop { match pc {
 0x60c157c => {
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
  if (cpu.sf != cpu.of) { pc = 0x60c159e; continue; }
  pc = 0x60c158f; continue;
 }
 0x60c158f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x60c1598); f_060c1567(cpu); if cpu.returned_to != 0x60c1598 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c159e => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x60c15a8); f_060aec9b(cpu); if cpu.returned_to != 0x60c15a8 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c15b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c15b1); }
 let mut pc: u32 = 0x60c15b1;
 loop { match pc {
 0x60c15b1 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x617b840u32)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c15db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c15db); }
 let mut pc: u32 = 0x60c15db;
 loop { match pc {
 0x60c15db => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x2u8); }
  if !cpu.zf { pc = 0x60c1600; continue; }
  pc = 0x60c15e8; continue;
 }
 0x60c15e8 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x60c15f2); f_060aec9b(cpu); if cpu.returned_to != 0x60c15f2 { return; }
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x20u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x60c15f6; continue;
 }
 0x60c15f6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60c1679; continue;
 }
 0x60c1600 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c160d; continue; }
  pc = 0x60c1606; continue;
 }
 0x60c1606 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c160d); f_060c167f(cpu); if cpu.returned_to != 0x60c160d { return; }
  pc = 0x60c160d; continue;
 }
 0x60c160d => {
  { let v: u32 = 0x400u32; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x60c164a; continue; }
  pc = 0x60c1617; continue;
 }
 0x60c1617 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x600u32; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if !cpu.zf { pc = 0x60c164a; continue; }
  pc = 0x60c1623; continue;
 }
 0x60c1623 => {
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
  if !cpu.zf { pc = 0x60c164a; continue; }
  pc = 0x60c163f; continue;
 }
 0x60c163f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c1646); f_060c16f6(cpu); if cpu.returned_to != 0x60c1646 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60c15f6; continue; }
  pc = 0x60c164a; continue;
 }
 0x60c164a => {
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
  if !cpu.zf { pc = 0x60c166a; continue; }
  pc = 0x60c1665; continue;
 }
 0x60c1665 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.edx))); }
  if !cpu.zf { pc = 0x60c1675; continue; }
  pc = 0x60c166a; continue;
 }
 0x60c166a => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c1671); f_060c16f6(cpu); if cpu.returned_to != 0x60c1671 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60c15f6; continue; }
  pc = 0x60c1675; continue;
 }
 0x60c1675 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c1679; continue;
 }
 0x60c1679 => {
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
pub fn f_060c167f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c167f); }
 let mut pc: u32 = 0x60c167f;
 loop { match pc {
 0x60c167f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x60c1688); f_060c4aa7(cpu); if cpu.returned_to != 0x60c1688 { return; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c16b4; continue; }
  pc = 0x60c168e; continue;
 }
 0x60c168e => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x60c169f; continue; }
  pc = 0x60c1696; continue;
 }
 0x60c1696 => {
  { let v: u32 = 0x86u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60c16b4; continue;
 }
 0x60c169f => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c16ad; continue; }
  pc = 0x60c16a4; continue;
 }
 0x60c16a4 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60c16b4; continue;
 }
 0x60c16ad => {
  { let v: u32 = 0x1000u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60c16b4; continue;
 }
 0x60c16b4 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x60c16bc); f_060bd034(cpu); if cpu.returned_to != 0x60c16bc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c16e3; continue; }
  pc = 0x60c16c3; continue;
 }
 0x60c16c3 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8) & 0xf8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x4u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x60c16e7; continue;
 }
 0x60c16e3 => {
  { let v: u8 = cpu.or8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x8u8); cpu.w8(0xcu32.wrapping_add(cpu.edx), v); }
  pc = 0x60c16e7; continue;
 }
 0x60c16e7 => {
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
pub fn f_060c16f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c16f6); }
 let mut pc: u32 = 0x60c16f6;
 loop { match pc {
 0x60c16f6 => {
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
  if cpu.zf { pc = 0x60c1782; continue; }
  pc = 0x60c170c; continue;
 }
 0x60c170c => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8) & 0xefu8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c1720; continue;
 }
 0x60c1720 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c172b; continue;
 }
 0x60c172b => {
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  pc = 0x60c1730; continue;
 }
 0x60c1730 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c1738; continue;
 }
 0x60c1738 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c1740; continue;
 }
 0x60c1740 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60c174c); f_060c4ad8(cpu); if cpu.returned_to != 0x60c174c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60c1760; continue; }
  pc = 0x60c1753; continue;
 }
 0x60c1753 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c177c; continue;
 }
 0x60c1760 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c177c; continue; }
  pc = 0x60c1764; continue;
 }
 0x60c1764 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x60c176e); f_060aec9b(cpu); if cpu.returned_to != 0x60c176e { return; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c177c; continue;
 }
 0x60c177c => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  cpu.poll(); pc = 0x60c1730; continue;
 }
 0x60c1782 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ecx)), 0x0u32); }
  if cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c1788; continue;
 }
 0x60c1788 => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)) & 0xefu8; cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x20u8); }
  if !cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c1792; continue;
 }
 0x60c1792 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c17aa; continue; }
  pc = 0x60c1799; continue;
 }
 0x60c1799 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60c17aa); f_060b1673(cpu); if cpu.returned_to != 0x60c17aa { return; }
  pc = 0x60c17aa; continue;
 }
 0x60c17aa => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60c17ba; continue; }
  pc = 0x60c17af; continue;
 }
 0x60c17af => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x20u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c17ba; continue;
 }
 0x60c17ba => {
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
pub fn o_060c17d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c17d0); }
 let mut pc: u32 = 0x60c17d0;
 loop { match pc {
 0x60c17d0 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x5u32); cpu.eax = v; }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x617c884u32), 0xffffu32); }
  if !cpu.zf { pc = 0x60c180e; continue; }
  pc = 0x60c17e3; continue;
 }
 0x60c17e3 => {
  cpu.push(0x60c17e8); f_060c5300(cpu); if cpu.returned_to != 0x60c17e8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = 0x617c888u32; cpu.edx = v; }
  if cpu.zf { pc = 0x60c1854; continue; }
  pc = 0x60c17f1; continue;
 }
 0x60c17f1 => {
  { let v = 0x60b1834u32; cpu.push(v); }
  cpu.push(0x60c17fb); f_060c56bc(cpu); if cpu.returned_to != 0x60c17fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617c884u32, v); }
  { cpu.sub16((cpu.eax as u16), 0xffffu16); }
  { let v: u32 = 0x617c8b7u32; cpu.edx = v; }
  if cpu.zf { pc = 0x60c1854; continue; }
  pc = 0x60c180e; continue;
 }
 0x60c180e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u8 = 0x5u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x60c7e2cu32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x60c80c0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x617c884u32); cpu.push(v); }
  cpu.push(0x60c1834); f_060c58c4(cpu); if cpu.returned_to != 0x60c1834 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); }
  { let v: u32 = 0x617c8d9u32; cpu.edx = v; }
  if !cpu.zf { pc = 0x60c1854; continue; }
  pc = 0x60c1846; continue;
 }
 0x60c1846 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u8 = 0xe9u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.poll(); pc = cpu.eax; continue;
 }
 0x60c1854 => {
  cpu.unimplemented(0x60c1854, "Int3");
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60c185b); f_060c5678(cpu); if cpu.returned_to != 0x60c185b { return; }
  cpu.unimplemented(0x60c185b, "Hlt");
  f_060c185c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c180c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c180c); } r_060c180c(cpu, 0x60c180c); }
pub fn f_060c1830(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c1830); } r_060c180c(cpu, 0x60c1830); }
fn r_060c180c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c180c => {
  if cpu.zf { pc = 0x60c1854; continue; }
  pc = 0x60c180e; continue;
 }
 0x60c180e => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u8 = 0x5u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(0x60c7e2cu32); cpu.eax = v; }
  { let v = (cpu.ecx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x60c80c0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x617c884u32); cpu.push(v); }
  cpu.push(0x60c1834); f_060c58c4(cpu); if cpu.returned_to != 0x60c1834 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); }
  { let v: u32 = 0x617c8d9u32; cpu.edx = v; }
  if !cpu.zf { pc = 0x60c1854; continue; }
  pc = 0x60c1846; continue;
 }
 0x60c1830 => {
  {  }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), (cpu.ecx as u16)); }
  { let v: u32 = 0x617c8d9u32; cpu.edx = v; }
  if !cpu.zf { pc = 0x60c1854; continue; }
  pc = 0x60c1846; continue;
 }
 0x60c1846 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u8 = 0xe9u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.sub32(cpu.eax, 0x5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { cpu.popad(); }
  cpu.poll(); pc = cpu.eax; continue;
 }
 0x60c1854 => {
  cpu.unimplemented(0x60c1854, "Int3");
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60c185b); f_060c5678(cpu); if cpu.returned_to != 0x60c185b { return; }
  cpu.unimplemented(0x60c185b, "Hlt");
  f_060c185c(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c185c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c185c); }
 let mut pc: u32 = 0x60c185c;
 loop { match pc {
 0x60c185c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c187a; continue; }
  pc = 0x60c1867; continue;
 }
 0x60c1867 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  pc = 0x60c187a; continue;
 }
 0x60c187a => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c1891; continue; }
  pc = 0x60c187e; continue;
 }
 0x60c187e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c1891 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c189a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c189a); }
 let mut pc: u32 = 0x60c189a;
 loop { match pc {
 0x60c189a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x60c18b0; continue; }
  pc = 0x60c18a4; continue;
 }
 0x60c18a4 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c18b0 => {
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c18b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c18b4); }
 let mut pc: u32 = 0x60c18b4;
 loop { match pc {
 0x60c18b4 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0xffu8); }
  if cpu.zf { pc = 0x60c18d4; continue; }
  pc = 0x60c18ba; continue;
 }
 0x60c18ba => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x62d2234u32, v); }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62d2235u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x62d2236u32, v); }
  pc = 0x60c18e1; continue;
 }
 0x60c18d4 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62d2234u32, v); }
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x62d2235u32, v); }
  pc = 0x60c18e1; continue;
 }
 0x60c18e1 => {
  { let v: u32 = 0x62d2234u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c18e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c18e8); }
 let mut pc: u32 = 0x60c18e8;
 loop { match pc {
 0x60c18e8 => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.push(0x60c18ef); f_060c1916(cpu); if cpu.returned_to != 0x60c18ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c18f8; continue; }
  pc = 0x60c18f3; continue;
 }
 0x60c18f3 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x60c1913; continue; }
  pc = 0x60c18f8; continue;
 }
 0x60c18f8 => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { cpu.sub16(cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if !cpu.zf { pc = 0x60c1909; continue; }
  pc = 0x60c1901; continue;
 }
 0x60c1901 => {
  { cpu.sub16(cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u16); }
  if cpu.zf { pc = 0x60c1913; continue; }
  pc = 0x60c1909; continue;
 }
 0x60c1909 => {
  { let v: u32 = 0x1u32; cpu.w32(0x617c90cu32, v); }
  pc = 0x60c1913; continue;
 }
 0x60c1913 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1916(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1916); }
 let mut pc: u32 = 0x60c1916;
 loop { match pc {
 0x60c1916 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c19bc; continue; }
  pc = 0x60c192c; continue;
 }
 0x60c192c => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c19bc; continue; }
  pc = 0x60c1935; continue;
 }
 0x60c1935 => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0x5cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x60c1945); f_060ae9e0(cpu); if cpu.returned_to != 0x60c1945 { return; }
  { let v: u32 = 0x28u32; cpu.ebx = v; }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x2511u32; cpu.ecx = v; }
  cpu.push(0x60c195a); f_060ae9e0(cpu); if cpu.returned_to != 0x60c195a { return; }
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
  cpu.push(0x60c198e); f_060c5a34(cpu); if cpu.returned_to != 0x60c198e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if cpu.zf { pc = 0x60c1a18; continue; }
  pc = 0x60c19a0; continue;
 }
 0x60c19a0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x48u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  pc = 0x60c19b1; continue;
 }
 0x60c19b1 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2360u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  pc = 0x60c1a1c; continue;
 }
 0x60c19bc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c1a18; continue; }
  pc = 0x60c19c8; continue;
 }
 0x60c19c8 => {
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  cpu.push(0x60c19d8); f_060ae9e0(cpu); if cpu.returned_to != 0x60c19d8 { return; }
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
  if !cpu.zf { pc = 0x60c1a18; continue; }
  pc = 0x60c1a04; continue;
 }
 0x60c1a04 => {
  { let v: u16 = cpu.r16(0x24u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x60c19b1; continue;
 }
 0x60c1a18 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c1a1c; continue;
 }
 0x60c1a1c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x70u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1a23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1a23); }
 let mut pc: u32 = 0x60c1a23;
 loop { match pc {
 0x60c1a23 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.ecx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60c1a81; continue; }
  pc = 0x60c1a33; continue;
 }
 0x60c1a33 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60c1a8c; continue; }
  pc = 0x60c1a3d; continue;
 }
 0x60c1a3d => {
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60c1a5e; continue; }
  pc = 0x60c1a49; continue;
 }
 0x60c1a49 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) & 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  pc = 0x60c1a60; continue;
 }
 0x60c1a5e => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  pc = 0x60c1a60; continue;
 }
 0x60c1a60 => {
  if cpu.zf { pc = 0x60c1a8c; continue; }
  pc = 0x60c1a62; continue;
 }
 0x60c1a62 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x60c1a7a; continue; }
  pc = 0x60c1a68; continue;
 }
 0x60c1a68 => {
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
 0x60c1a7a => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c1a81 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x60c1a8c; continue;
 }
 0x60c1a8c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1b28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1b28); }
 let mut pc: u32 = 0x60c1b28;
 loop { match pc {
 0x60c1b28 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if cpu.cf { pc = 0x60c1b40; continue; }
  pc = 0x60c1b39; continue;
 }
 0x60c1b39 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x60c1ba2; continue;
 }
 0x60c1b40 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c1ba2; continue; }
  pc = 0x60c1b45; continue;
 }
 0x60c1b45 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  if cpu.zf { pc = 0x60c1b62; continue; }
  pc = 0x60c1b49; continue;
 }
 0x60c1b49 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x60c1b64; continue;
 }
 0x60c1b62 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x60c1b64; continue;
 }
 0x60c1b64 => {
  if cpu.zf { pc = 0x60c1b6b; continue; }
  pc = 0x60c1b66; continue;
 }
 0x60c1b66 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  pc = 0x60c1ba2; continue;
 }
 0x60c1b6b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c1b8d; continue; }
  pc = 0x60c1b70; continue;
 }
 0x60c1b70 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c1b89; continue; }
  pc = 0x60c1b74; continue;
 }
 0x60c1b74 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  pc = 0x60c1b8b; continue;
 }
 0x60c1b89 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  pc = 0x60c1b8b; continue;
 }
 0x60c1b8b => {
  if !cpu.zf { cpu.poll(); pc = 0x60c1b6b; continue; }
  pc = 0x60c1b8d; continue;
 }
 0x60c1b8d => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x2u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60c1ba2; continue;
 }
 0x60c1ba2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617c90cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1bae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1bae); }
 let mut pc: u32 = 0x60c1bae;
 loop { match pc {
 0x60c1bae => {
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = 0xc0000000u32; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4150017eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80000000u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4147ffffu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617412du32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c1bff; continue; }
  pc = 0x60c1bdc; continue;
 }
 0x60c1bdc => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.rf64(0x616b268u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x60c1bff; continue; }
  pc = 0x60c1bf8; continue;
 }
 0x60c1bf8 => {
  { let v: u8 = cpu.r8(0x617b930u32) | 0x1u8; cpu.w8(0x617b930u32, v); }
  pc = 0x60c1bff; continue;
 }
 0x60c1bff => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1c03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1c03); }
 let mut pc: u32 = 0x60c1c03;
 loop { match pc {
 0x60c1c03 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  cpu.push(0x60c1c0b); f_060c1c0d(cpu); if cpu.returned_to != 0x60c1c0b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1c0d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c1c0d); } r_060c1c0d(cpu, 0x60c1c0d); }
pub fn f_060c1c18(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c1c18); } r_060c1c0d(cpu, 0x60c1c18); }
fn r_060c1c0d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c1c0d => {
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
  if cpu.zf { pc = 0x60c1c39; continue; }
  pc = 0x60c1c32; continue;
 }
 0x60c1c18 => {
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x428be045u32.wrapping_add(cpu.ecx))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x89u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.port_in(0x8bu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x438be845u32.wrapping_add(cpu.ecx))); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x89u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = cpu.port_in((cpu.edx as u16), 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x60c1c39; continue; }
  pc = 0x60c1c32; continue;
 }
 0x60c1c32 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x60c1c73; continue;
 }
 0x60c1c39 => {
  { cpu.and8((cpu.ecx as u8), 0x80u8); }
  if cpu.zf { pc = 0x60c1c45; continue; }
  pc = 0x60c1c3e; continue;
 }
 0x60c1c3e => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  pc = 0x60c1c73; continue;
 }
 0x60c1c45 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c1c51; continue; }
  pc = 0x60c1c4a; continue;
 }
 0x60c1c4a => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  pc = 0x60c1c73; continue;
 }
 0x60c1c51 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x60c1c5d; continue; }
  pc = 0x60c1c56; continue;
 }
 0x60c1c56 => {
  { let v: u32 = 0x4u32; cpu.esi = v; }
  pc = 0x60c1c73; continue;
 }
 0x60c1c5d => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x2u8); }
  if cpu.zf { pc = 0x60c1c69; continue; }
  pc = 0x60c1c62; continue;
 }
 0x60c1c62 => {
  { let v: u32 = 0x6u32; cpu.esi = v; }
  pc = 0x60c1c73; continue;
 }
 0x60c1c69 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c1c73; continue; }
  pc = 0x60c1c6e; continue;
 }
 0x60c1c6e => {
  { let v: u32 = 0x5u32; cpu.esi = v; }
  pc = 0x60c1c73; continue;
 }
 0x60c1c73 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617ca10u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x60c1c97; continue; }
  pc = 0x60c1c8a; continue;
 }
 0x60c1c8a => {
  { let v = cpu.rf64(0x616b5d8u32); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60c1cd9; continue;
 }
 0x60c1c97 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x60c1ca6; continue; }
  pc = 0x60c1c9c; continue;
 }
 0x60c1c9c => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c1cd9; continue;
 }
 0x60c1ca6 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x40u8); }
  if cpu.zf { pc = 0x60c1cba; continue; }
  pc = 0x60c1cab; continue;
 }
 0x60c1cab => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c1cd9; continue;
 }
 0x60c1cba => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x80u8); }
  if cpu.zf { pc = 0x60c1cce; continue; }
  pc = 0x60c1cbf; continue;
 }
 0x60c1cbf => {
  { let v: u32 = cpu.r32(0x616b5d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x616b5dcu32); cpu.eax = v; }
  pc = 0x60c1cd6; continue;
 }
 0x60c1cce => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x60c1cd6; continue;
 }
 0x60c1cd6 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c1cd9; continue;
 }
 0x60c1cd9 => {
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60c1ce1); f_060c5ba3(cpu); if cpu.returned_to != 0x60c1ce1 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1ce8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1ce8); }
 let mut pc: u32 = 0x60c1ce8;
 loop { match pc {
 0x60c1ce8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.cf { pc = 0x60c1d2a; continue; }
  pc = 0x60c1cf9; continue;
 }
 0x60c1cf9 => {
  { cpu.sub8((cpu.edx as u8), 0x3u8); }
  if (cpu.cf || cpu.zf) { pc = 0x60c1d05; continue; }
  pc = 0x60c1cfe; continue;
 }
 0x60c1cfe => {
  { cpu.sub8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c1d16; continue; }
  pc = 0x60c1d03; continue;
 }
 0x60c1d03 => {
  pc = 0x60c1d2a; continue;
 }
 0x60c1d05 => {
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60c1d4e; continue;
 }
 0x60c1d16 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x60c1d2a; continue; }
  pc = 0x60c1d20; continue;
 }
 0x60c1d20 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c1d56; continue;
 }
 0x60c1d2a => {
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8) | 0x81u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub32(cpu.edx, 0x6u32); }
  if !cpu.zf { pc = 0x60c1d49; continue; }
  pc = 0x60c1d3a; continue;
 }
 0x60c1d3a => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x60c1d49; continue; }
  pc = 0x60c1d44; continue;
 }
 0x60c1d44 => {
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x11u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x60c1d49; continue;
 }
 0x60c1d49 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x60c1d4e; continue;
 }
 0x60c1d4e => {
  cpu.push(0x60c1d53); f_060c1c03(cpu); if cpu.returned_to != 0x60c1d53 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60c1d56; continue;
 }
 0x60c1d56 => {
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
pub fn f_060c1d64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c1d64); } r_060c1d64(cpu, 0x60c1d64); }
pub fn f_060c1e00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c1e00); } r_060c1d64(cpu, 0x60c1e00); }
fn r_060c1d64(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c1d64 => {
  { let v: u32 = cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[4])), cpu.ecx); cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)).wrapping_add(cpu.bases[4]), v); }
  if !cpu.zf { pc = 0x60c1d8b; continue; }
  pc = 0x60c1d6a; continue;
 }
 0x60c1d6a => {
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
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
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)) & 0x21a0060cu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x21u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), (cpu.ecx & 0xff)); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.port_out(0x21u16, 4, cpu.eax as u32); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x22u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x7e060c22u32; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c1e00; continue;
 }
 0x60c1d8b => {
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
  { let v: u8 = cpu.r8(0xa0060c21u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.eax as u8)); cpu.w8(0xa0060c21u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))) & cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = 0x21u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.r32(0x0u32.wrapping_add(cpu.ecx)), (cpu.ecx & 0xff)); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.port_out(0x21u16, 4, cpu.eax as u32); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x22u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.push(cpu.segments[2] as u32); }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp ^ cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = 0x7e060c22u32; cpu.push(v); }
  { let v: u8 = (cpu.ecx as u8) & cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c1e00; continue;
 }
 0x60c1e00 => {
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
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.call(0x60c1e24); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c1f64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c1f64); }
 let mut pc: u32 = 0x60c1f64;
 loop { match pc {
 0x60c1f64 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xc3060c2du32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0x60cu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.port_out(0x2du16, 4, cpu.eax as u32); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x60c1f88, "Outsd");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(3118); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3b90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3b90); }
 let mut pc: u32 = 0x60c3b90;
 loop { match pc {
 0x60c3b90 => {
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
  if cpu.zf { pc = 0x60c3baf; continue; }
  pc = 0x60c3ba9; continue;
 }
 0x60c3ba9 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x60c3bb2; continue; }
  pc = 0x60c3baf; continue;
 }
 0x60c3baf => {
  { let v: u32 = cpu.eax | 0x1u32; cpu.eax = v; }
  pc = 0x60c3bb2; continue;
 }
 0x60c3bb2 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x60c3bbe; continue; }
  pc = 0x60c3bb8; continue;
 }
 0x60c3bb8 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x60c3bcd; continue; }
  pc = 0x60c3bbe; continue;
 }
 0x60c3bbe => {
  { let v: u32 = cpu.eax | 0x2u32; cpu.eax = v; }
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x60c3bcd; continue; }
  pc = 0x60c3bca; continue;
 }
 0x60c3bca => {
  { let v: u32 = cpu.eax | 0x4u32; cpu.eax = v; }
  pc = 0x60c3bcd; continue;
 }
 0x60c3bcd => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x60c3bd9; continue; }
  pc = 0x60c3bd3; continue;
 }
 0x60c3bd3 => {
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x60c3bdc; continue; }
  pc = 0x60c3bd9; continue;
 }
 0x60c3bd9 => {
  { let v: u32 = cpu.eax | 0x8u32; cpu.eax = v; }
  pc = 0x60c3bdc; continue;
 }
 0x60c3bdc => {
  { cpu.and32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x60c3bea; continue; }
  pc = 0x60c3be3; continue;
 }
 0x60c3be3 => {
  { cpu.and32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60c3bfc; continue; }
  pc = 0x60c3bea; continue;
 }
 0x60c3bea => {
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x60c3bfc; continue; }
  pc = 0x60c3bf0; continue;
 }
 0x60c3bf0 => {
  { cpu.sub32(cpu.r32(0x6cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x7fu32); }
  if !cpu.zf { pc = 0x60c3bfc; continue; }
  pc = 0x60c3bf6; continue;
 }
 0x60c3bf6 => {
  { cpu.sub32(cpu.r32(0x54u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x40u32); }
  if cpu.zf { pc = 0x60c3c06; continue; }
  pc = 0x60c3bfc; continue;
 }
 0x60c3bfc => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x60c3c12; continue; }
  pc = 0x60c3c06; continue;
 }
 0x60c3c06 => {
  { cpu.and32(cpu.r32(0x48u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x60c3c12; continue; }
  pc = 0x60c3c0f; continue;
 }
 0x60c3c0f => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x60c3c12; continue;
 }
 0x60c3c12 => {
  { let v: u32 = cpu.eax; cpu.w32(0x617ca70u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c3c29; continue; }
  pc = 0x60c3c24; continue;
 }
 0x60c3c24 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x60c3c29; continue;
 }
 0x60c3c29 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, 0x10000u32); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c3c3d; continue; }
  pc = 0x60c3c3b; continue;
 }
 0x60c3c3b => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x60c3c3d; continue;
 }
 0x60c3c3d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x62d1eb8u32.wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x60c3c87; continue; }
  pc = 0x60c3c45; continue;
 }
 0x60c3c45 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617ca84u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and32(cpu.r32(0x617ca70u32.wrapping_add(cpu.bases[3])), 0x2u32); }
  if cpu.zf { pc = 0x60c3c65; continue; }
  pc = 0x60c3c61; continue;
 }
 0x60c3c61 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x60c3c65; continue;
 }
 0x60c3c65 => {
  { cpu.and32(cpu.r32(0x617ca70u32.wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x60c3c75; continue; }
  pc = 0x60c3c71; continue;
 }
 0x60c3c71 => {
  { let v: u32 = cpu.shl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 1u32); cpu.ebx = v; }
  pc = 0x60c3c75; continue;
 }
 0x60c3c75 => {
  { let v: u32 = cpu.eax; cpu.w32(0x617ca88u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617ca8cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x617ca70u32.wrapping_add(cpu.bases[3])) | 0x20u32; cpu.w32(0x617ca70u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c3c87; continue;
 }
 0x60c3c87 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617ca7cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617ca80u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617ca78u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.ebx = v; }
  { let v: u32 = 0x80000000u32; cpu.ecx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617ca70u32.wrapping_add(cpu.bases[3])); cpu.ebp = v; }
  let t = cpu.r32(0x60c1f64u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.flat_ds(0x60c3ccd); cpu.push(0x60c3cd5); cpu.call(t); if cpu.returned_to != 0x60c3cd5 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0xcu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]))); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x1cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4)).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x617ca7cu32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x60c3cf2); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3cf3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3cf3); }
 let mut pc: u32 = 0x60c3cf3;
 loop { match pc {
 0x60c3cf3 => {
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
  if cpu.zf { pc = 0x60c3d10; continue; }
  pc = 0x60c3d0a; continue;
 }
 0x60c3d0a => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x60c3d13; continue; }
  pc = 0x60c3d10; continue;
 }
 0x60c3d10 => {
  { let v: u32 = cpu.eax | 0x10u32; cpu.eax = v; }
  pc = 0x60c3d13; continue;
 }
 0x60c3d13 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x60c3d1f; continue; }
  pc = 0x60c3d19; continue;
 }
 0x60c3d19 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x3u32); }
  if !cpu.zf { pc = 0x60c3d22; continue; }
  pc = 0x60c3d1f; continue;
 }
 0x60c3d1f => {
  { let v: u32 = cpu.eax | 0x20u32; cpu.eax = v; }
  pc = 0x60c3d22; continue;
 }
 0x60c3d22 => {
  { cpu.sub32(cpu.r32(0x68u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60c3d2b; continue; }
  pc = 0x60c3d28; continue;
 }
 0x60c3d28 => {
  { let v: u32 = cpu.eax | 0x40u32; cpu.eax = v; }
  pc = 0x60c3d2b; continue;
 }
 0x60c3d2b => {
  { let v: u32 = cpu.eax; cpu.w32(0x617ca70u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x20u32); }
  if !cpu.zf { pc = 0x60c3d59; continue; }
  pc = 0x60c3d3f; continue;
 }
 0x60c3d3f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x20u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), 0x8u32); }
  if cpu.zf { pc = 0x60c3d72; continue; }
  pc = 0x60c3d4f; continue;
 }
 0x60c3d4f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x60c3d72; continue;
 }
 0x60c3d59 => {
  { let v: u32 = cpu.r32(0x617ca70u32.wrapping_add(cpu.bases[3])) | 0x8u32; cpu.w32(0x617ca70u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  pc = 0x60c3d72; continue;
 }
 0x60c3d72 => {
  { let v: u32 = cpu.r32(0x617ca70u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  let t = cpu.r32(0x60c1d64u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.flat_ds(0x60c3d77); cpu.push(0x60c3d7f); cpu.call(t); if cpu.returned_to != 0x60c3d7f { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x60c3d85); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3d86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3d86); }
 let mut pc: u32 = 0x60c3d86;
 loop { match pc {
 0x60c3d86 => {
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
  cpu.flat_ds(0x60c3db1); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3db2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3db2); }
 let mut pc: u32 = 0x60c3db2;
 loop { match pc {
 0x60c3db2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = 0x617ca90u32; cpu.push(v); }
  { let v = 0x617ca70u32; cpu.push(v); }
  cpu.flat_ds(0x60c3dc1); cpu.push(0x60c3dc6); f_060b3160(cpu); if cpu.returned_to != 0x60c3dc6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x60c3db2u32; cpu.push(v); }
  { let v = 0x60c1d64u32; cpu.push(v); }
  cpu.flat_ds(0x60c3dd3); cpu.push(0x60c3dd8); f_060b3160(cpu); if cpu.returned_to != 0x60c3dd8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c3de0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3de1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3de1); }
 let mut pc: u32 = 0x60c3de1;
 loop { match pc {
 0x60c3de1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x8000u32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb3cu32, v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c3e02; continue; }
  pc = 0x60c3dfd; continue;
 }
 0x60c3dfd => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c3e09; continue; }
  pc = 0x60c3e02; continue;
 }
 0x60c3e02 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c3e0b; continue;
 }
 0x60c3e09 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c3e0b; continue;
 }
 0x60c3e0b => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c3e44; continue; }
  pc = 0x60c3e0f; continue;
 }
 0x60c3e0f => {
  cpu.push(0x60c3e14); f_060c5cdf(cpu); if cpu.returned_to != 0x60c3e14 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c3e44; continue; }
  pc = 0x60c3e19; continue;
 }
 0x60c3e19 => {
  { let v: u32 = 0x60adc6bu32; cpu.edx = v; }
  { let v: u32 = 0x60adc60u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x60adc60u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x600u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60c3e44; continue;
 }
 0x60c3e44 => {
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
pub fn f_060c3e4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3e4a); }
 let mut pc: u32 = 0x60c3e4a;
 loop { match pc {
 0x60c3e4a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3e4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3e4b); }
 let mut pc: u32 = 0x60c3e4b;
 loop { match pc {
 0x60c3e4b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x617ca90u32); cpu.push(0x60c3e54); cpu.call(t); if cpu.returned_to != 0x60c3e54 { return; }
  let t = cpu.r32(0x617ca94u32); cpu.push(0x60c3e5a); cpu.call(t); if cpu.returned_to != 0x60c3e5a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c3e61); f_060c3e63(cpu); if cpu.returned_to != 0x60c3e61 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3e7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3e7a); }
 let mut pc: u32 = 0x60c3e7a;
 loop { match pc {
 0x60c3e7a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x60c3ec9; continue; }
  pc = 0x60c3e84; continue;
 }
 0x60c3e84 => {
  { let v: u32 = cpu.add32(cpu.eax, 0xbu32); cpu.eax = v; }
  if cpu.cf { pc = 0x60c3ec9; continue; }
  pc = 0x60c3e89; continue;
 }
 0x60c3e89 => {
  { let v: u8 = (cpu.eax as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x10u32; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax & cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c3ec9; continue; }
  pc = 0x60c3e9e; continue;
 }
 0x60c3e9e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c3ead; continue; }
  pc = 0x60c3ea8; continue;
 }
 0x60c3ea8 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x60c3ead; continue;
 }
 0x60c3ead => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x60c3ecd; continue; }
  pc = 0x60c3eb3; continue;
 }
 0x60c3eb3 => {
  { let v: u32 = cpu.sub32(cpu.edi, cpu.edx); cpu.edi = v; }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi & cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c3ead; continue; }
  pc = 0x60c3ec6; continue;
 }
 0x60c3ec6 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c3ec9; continue;
 }
 0x60c3ec9 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c3f18; continue;
 }
 0x60c3ecd => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { pc = 0x60c3efb; continue; }
  pc = 0x60c3edd; continue;
 }
 0x60c3edd => {
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
  pc = 0x60c3f10; continue;
 }
 0x60c3efb => {
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x60c3f10; continue;
 }
 0x60c3f10 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])) | 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x4u32); cpu.eax = v; }
  pc = 0x60c3f18; continue;
 }
 0x60c3f18 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x60c3f1c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c3f22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c3f22); }
 let mut pc: u32 = 0x60c3f22;
 loop { match pc {
 0x60c3f1d => {
  pc = 0x60c4028; continue;
 }
 0x60c3f22 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x60c3f1d; continue; }
  pc = 0x60c3f2c; continue;
 }
 0x60c3f2c => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x60c3f1d; continue; }
  pc = 0x60c3f37; continue;
 }
 0x60c3f37 => {
  { let v: u8 = (cpu.eax as u8) & 0xfeu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x1u32); }
  if !cpu.zf { pc = 0x60c3f64; continue; }
  pc = 0x60c3f45; continue;
 }
 0x60c3f45 => {
  { cpu.sub32(cpu.edi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x60c3f4d; continue; }
  pc = 0x60c3f4a; continue;
 }
 0x60c3f4a => {
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c3f4d; continue;
 }
 0x60c3f4d => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c3fe0; continue;
 }
 0x60c3f64 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x60c3f7b; continue; }
  pc = 0x60c3f6d; continue;
 }
 0x60c3f6d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c3fe0; continue; }
  pc = 0x60c3f72; continue;
 }
 0x60c3f72 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x60c3fe0; continue; }
  pc = 0x60c3f79; continue;
 }
 0x60c3f79 => {
  pc = 0x60c3f8a; continue;
 }
 0x60c3f7b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x60c3fe0; continue; }
  pc = 0x60c3f82; continue;
 }
 0x60c3f82 => {
  { let v: u32 = 0x20u32.wrapping_add(cpu.ebx); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x24u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c3fe0; continue; }
  pc = 0x60c3f8a; continue;
 }
 0x60c3f8a => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x60c3fbf; continue; }
  pc = 0x60c3f9a; continue;
 }
 0x60c3f9a => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c3fa7; continue; }
  pc = 0x60c3fa5; continue;
 }
 0x60c3fa5 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x60c3fa7; continue;
 }
 0x60c3fa7 => {
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); cpu.edi = v; }
  pc = 0x60c3fab; continue;
 }
 0x60c3fab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c3fe0; continue; }
  pc = 0x60c3fb2; continue;
 }
 0x60c3fb2 => {
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60c3fbf; continue; }
  pc = 0x60c3fb7; continue;
 }
 0x60c3fb7 => {
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c3fab; continue; }
  pc = 0x60c3fbf; continue;
 }
 0x60c3fbf => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x60c3fc9; continue; }
  pc = 0x60c3fc6; continue;
 }
 0x60c3fc6 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  pc = 0x60c3fc9; continue;
 }
 0x60c3fc9 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x60c3fe0; continue; }
  pc = 0x60c3fcd; continue;
 }
 0x60c3fcd => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x60c3fe0; continue; }
  pc = 0x60c3fd4; continue;
 }
 0x60c3fd4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.cf { pc = 0x60c3fe0; continue; }
  pc = 0x60c3fdb; continue;
 }
 0x60c3fdb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  cpu.poll(); pc = 0x60c3fc9; continue;
 }
 0x60c3fe0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let a = cpu.edx; let b = cpu.edi; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x60c3fff; continue; }
  pc = 0x60c3fef; continue;
 }
 0x60c3fef => {
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x60c3ffb; continue; }
  pc = 0x60c3ff8; continue;
 }
 0x60c3ff8 => {
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c3ffb; continue;
 }
 0x60c3ffb => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x60c4010; continue;
 }
 0x60c3fff => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x1cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c4010; continue;
 }
 0x60c4010 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); cpu.w32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.esi, cpu.r32(0xcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if !cpu.cf { pc = 0x60c4020; continue; }
  pc = 0x60c4018; continue;
 }
 0x60c4018 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x60c4020; continue; }
  pc = 0x60c401d; continue;
 }
 0x60c401d => {
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c4020; continue;
 }
 0x60c4020 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]))); }
  if (cpu.cf || cpu.zf) { pc = 0x60c4028; continue; }
  pc = 0x60c4025; continue;
 }
 0x60c4025 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c4028; continue;
 }
 0x60c4028 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x60c402c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c402d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c402d); }
 let mut pc: u32 = 0x60c402d;
 loop { match pc {
 0x60c402d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617c774u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x60c4043; continue; }
  pc = 0x60c403a; continue;
 }
 0x60c403a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617c774u32, v); }
  pc = 0x60c4043; continue;
 }
 0x60c4043 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x62d2190u32)); }
  if !cpu.zf { pc = 0x60c4053; continue; }
  pc = 0x60c404b; continue;
 }
 0x60c404b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x62d2190u32, v); }
  pc = 0x60c4053; continue;
 }
 0x60c4053 => {
  { let v: u32 = cpu.r32(0x617c770u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x60c4066; continue; }
  pc = 0x60c405d; continue;
 }
 0x60c405d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617c770u32, v); }
  pc = 0x60c4066; continue;
 }
 0x60c4066 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c4073; continue; }
  pc = 0x60c4070; continue;
 }
 0x60c4070 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  pc = 0x60c4073; continue;
 }
 0x60c4073 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c407a; continue; }
  pc = 0x60c4077; continue;
 }
 0x60c4077 => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c407a; continue;
 }
 0x60c407a => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c407e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c407e); } r_060c407e(cpu, 0x60c407e); }
pub fn f_060c429d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c429d); } r_060c407e(cpu, 0x60c429d); }
pub fn f_060c436e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c436e); } r_060c407e(cpu, 0x60c436e); }
fn r_060c407e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c407e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617c770u32); cpu.edx = v; }
  pc = 0x60c4089; continue;
 }
 0x60c4089 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c4368; continue; }
  pc = 0x60c4091; continue;
 }
 0x60c4091 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x60c40d5; continue; }
  pc = 0x60c409f; continue;
 }
 0x60c409f => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  cpu.push(0x60c40ac); f_060c402d(cpu); if cpu.returned_to != 0x60c40ac { return; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c40c5; continue; }
  pc = 0x60c40b5; continue;
 }
 0x60c40b5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.poll(); pc = 0x60c4089; continue;
 }
 0x60c40c5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x49u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.poll(); pc = 0x60c4089; continue;
 }
 0x60c40d5 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x60c4089; continue;
 }
 0x60c429d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  cpu.push(0x60c42b1); f_060c407e(cpu); if cpu.returned_to != 0x60c42b1 { return; }
  { let v: u8 = 0x5u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x60c42cc; continue; }
  pc = 0x60c42bc; continue;
 }
 0x60c42bc => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c42cc; continue;
 }
 0x60c42cc => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c42e9; continue; }
  pc = 0x60c42d0; continue;
 }
 0x60c42d0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60c4365; continue;
 }
 0x60c42e9 => {
  { cpu.and16(cpu.r16(0x62d2766u32), 0xfff0u16); }
  if cpu.zf { pc = 0x60c42fe; continue; }
  pc = 0x60c42f4; continue;
 }
 0x60c42f4 => {
  { let v: u32 = 0xfffffu32; cpu.w32(0x62d2764u32, v); }
  pc = 0x60c42fe; continue;
 }
 0x60c42fe => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x10000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c4363; continue; }
  pc = 0x60c4307; continue;
 }
 0x60c4307 => {
  { let v: u32 = cpu.r32(0x62d2764u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.ebx as u8), 0x1u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x48u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x60c4363; continue; }
  pc = 0x60c4321; continue;
 }
 0x60c4321 => {
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
  if (cpu.sf != cpu.of) { pc = 0x60c4363; continue; }
  pc = 0x60c4345; continue;
 }
 0x60c4345 => {
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
  pc = 0x60c4365; continue;
 }
 0x60c4363 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c4365; continue;
 }
 0x60c4365 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60c4368; continue;
 }
 0x60c4368 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c436e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617cb5cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60c4381; continue; }
  pc = 0x60c437d; continue;
 }
 0x60c437d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.poll(); pc = 0x60c4365; continue;
 }
 0x60c4381 => {
  { cpu.sub32(cpu.r32(0x617b830u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x60c437d; continue; }
  pc = 0x60c438a; continue;
 }
 0x60c438a => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60c4391); f_060c456a(cpu); if cpu.returned_to != 0x60c4391 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x60c4365; continue; }
  pc = 0x60c4395; continue;
 }
 0x60c4395 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c43b6; continue; }
  pc = 0x60c43a1; continue;
 }
 0x60c43a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60c43a9); f_060c429d(cpu); if cpu.returned_to != 0x60c43a9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c43d1; continue; }
  pc = 0x60c43af; continue;
 }
 0x60c43af => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c43d1; continue;
 }
 0x60c43b6 => {
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
  pc = 0x60c43d1; continue;
 }
 0x60c43d1 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x60c437d; continue; }
  pc = 0x60c43d5; continue;
 }
 0x60c43d5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x60c437d; continue; }
  pc = 0x60c43e2; continue;
 }
 0x60c43e2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x60c437d; continue; }
  pc = 0x60c43ea; continue;
 }
 0x60c43ea => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c43f3); f_060c41d9(cpu); if cpu.returned_to != 0x60c43f3 { return; }
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
  cpu.push(0x60c4414); f_060bd117(cpu); if cpu.returned_to != 0x60c4414 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c4365; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c40da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c40da); }
 let mut pc: u32 = 0x60c40da;
 loop { match pc {
 0x60c40da => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x60c40e8); f_060c407e(cpu); if cpu.returned_to != 0x60c40e8 { return; }
  { let v: u32 = cpu.r32(0x617c770u32); cpu.edx = v; }
  pc = 0x60c41c9; continue;
 }
 0x60c40f3 => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c41d1; continue; }
  pc = 0x60c40fb; continue;
 }
 0x60c40fb => {
  { let v: u32 = 0x2cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x60c41c6; continue; }
  pc = 0x60c4106; continue;
 }
 0x60c4106 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x60c41c6; continue; }
  pc = 0x60c4110; continue;
 }
 0x60c4110 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c4117); f_060c402d(cpu); if cpu.returned_to != 0x60c4117 { return; }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60c41d1; continue; }
  pc = 0x60c4124; continue;
 }
 0x60c4124 => {
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
  if cpu.zf { pc = 0x60c4168; continue; }
  pc = 0x60c4158; continue;
 }
 0x60c4158 => {
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c4168; continue;
 }
 0x60c4168 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c41d3; continue; }
  pc = 0x60c416c; continue;
 }
 0x60c416c => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60c4183); f_060c41d9(cpu); if cpu.returned_to != 0x60c4183 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebp & 0xfffffffeu32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, 0xcu32); }
  if cpu.cf { pc = 0x60c41bf; continue; }
  pc = 0x60c4198; continue;
 }
 0x60c4198 => {
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
  cpu.push(0x60c41bd); f_060bd117(cpu); if cpu.returned_to != 0x60c41bd { return; }
  pc = 0x60c41c2; continue;
 }
 0x60c41bf => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x1u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c41c2; continue;
 }
 0x60c41c2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60c41d3; continue;
 }
 0x60c41c6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x60c41c9; continue;
 }
 0x60c41c9 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c40f3; continue; }
  pc = 0x60c41d1; continue;
 }
 0x60c41d1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c41d3; continue;
 }
 0x60c41d3 => {
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
pub fn f_060c41d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c41d9); }
 let mut pc: u32 = 0x60c41d9;
 loop { match pc {
 0x60c41d9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617c770u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60c41ef; continue;
 }
 0x60c41e6 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.cf { pc = 0x60c41f3; continue; }
  pc = 0x60c41ea; continue;
 }
 0x60c41ea => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x60c41ef; continue;
 }
 0x60c41ef => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60c41e6; continue; }
  pc = 0x60c41f3; continue;
 }
 0x60c41f3 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60c4202; continue; }
  pc = 0x60c41fd; continue;
 }
 0x60c41fd => {
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c4208; continue;
 }
 0x60c4202 => {
  { let v: u32 = cpu.edx; cpu.w32(0x617c770u32, v); }
  pc = 0x60c4208; continue;
 }
 0x60c4208 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c420f; continue; }
  pc = 0x60c420c; continue;
 }
 0x60c420c => {
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c420f; continue;
 }
 0x60c420f => {
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
pub fn f_060c424d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c424d); }
 let mut pc: u32 = 0x60c424d;
 loop { match pc {
 0x60c424d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x617c770u32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c4298; continue; }
  pc = 0x60c4259; continue;
 }
 0x60c4259 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c428b; continue; }
  pc = 0x60c426f; continue;
 }
 0x60c426f => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c428b; continue; }
  pc = 0x60c4274; continue;
 }
 0x60c4274 => {
  { cpu.sub8(cpu.r8(0x617b860u32), 0x0u8); }
  if !cpu.zf { pc = 0x60c428b; continue; }
  pc = 0x60c427d; continue;
 }
 0x60c427d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x60c4282, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617b830u32, v); }
  pc = 0x60c428b; continue;
 }
 0x60c428b => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x617b830u32)); }
  if !cpu.zf { pc = 0x60c4298; continue; }
  pc = 0x60c4293; continue;
 }
 0x60c4293 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c4298 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c441e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c441e); }
 let mut pc: u32 = 0x60c441e;
 loop { match pc {
 0x60c441e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c4439; continue; }
  pc = 0x60c4430; continue;
 }
 0x60c4430 => {
  { cpu.sub8(cpu.r8(0x617b85fu32), 0x0u8); }
  if cpu.zf { pc = 0x60c4445; continue; }
  pc = 0x60c4439; continue;
 }
 0x60c4439 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x60c4452; continue; }
  pc = 0x60c4445; continue;
 }
 0x60c4445 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60c444d); f_060c436e(cpu); if cpu.returned_to != 0x60c444d { return; }
  pc = 0x60c4561; continue;
 }
 0x60c4452 => {
  { cpu.sub32(cpu.r32(0x617cb5cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60c4462; continue; }
  pc = 0x60c445b; continue;
 }
 0x60c445b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c4561; continue;
 }
 0x60c4462 => {
  { cpu.sub32(cpu.r32(0x617b830u32), 0xfffffffeu32); }
  if cpu.zf { cpu.poll(); pc = 0x60c445b; continue; }
  pc = 0x60c446b; continue;
 }
 0x60c446b => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60c4472); f_060c456a(cpu); if cpu.returned_to != 0x60c4472 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c4561; continue; }
  pc = 0x60c447a; continue;
 }
 0x60c447a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c44a2; continue; }
  pc = 0x60c4486; continue;
 }
 0x60c4486 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c44a2; continue; }
  pc = 0x60c448b; continue;
 }
 0x60c448b => {
  { cpu.sub8(cpu.r8(0x617b860u32), 0x0u8); }
  if !cpu.zf { pc = 0x60c44a2; continue; }
  pc = 0x60c4494; continue;
 }
 0x60c4494 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x60c4499, "Lsl");
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617b830u32, v); }
  pc = 0x60c44a2; continue;
 }
 0x60c44a2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617b830u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { pc = 0x60c44b6; continue; }
  pc = 0x60c44b1; continue;
 }
 0x60c44b1 => {
  { let v: u32 = 0xfffffffeu32; cpu.edx = v; }
  pc = 0x60c44b6; continue;
 }
 0x60c44b6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c44bd); f_060c626f(cpu); if cpu.returned_to != 0x60c44bd { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { cpu.poll(); pc = 0x60c445b; continue; }
  pc = 0x60c44c2; continue;
 }
 0x60c44c2 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xfffffff8u32); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x60c445b; continue; }
  pc = 0x60c44c9; continue;
 }
 0x60c44c9 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x60c445b; continue; }
  pc = 0x60c44cd; continue;
 }
 0x60c44cd => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x60c445b; continue; }
  pc = 0x60c44d9; continue;
 }
 0x60c44d9 => {
  { let v: u32 = cpu.r32(0x617c770u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c44f6; continue;
 }
 0x60c44e4 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60c44fa; continue; }
  pc = 0x60c44eb; continue;
 }
 0x60c44eb => {
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c44f3; continue; }
  pc = 0x60c44ef; continue;
 }
 0x60c44ef => {
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { pc = 0x60c44fa; continue; }
  pc = 0x60c44f3; continue;
 }
 0x60c44f3 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x60c44f6; continue;
 }
 0x60c44f6 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c44e4; continue; }
  pc = 0x60c44fa; continue;
 }
 0x60c44fa => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c4524; continue; }
  pc = 0x60c44fe; continue;
 }
 0x60c44fe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if !cpu.zf { pc = 0x60c4524; continue; }
  pc = 0x60c4509; continue;
 }
 0x60c4509 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)).wrapping_add(cpu.esi); cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c4543; continue;
 }
 0x60c4524 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x38u32); }
  if cpu.cf { cpu.poll(); pc = 0x60c445b; continue; }
  pc = 0x60c452e; continue;
 }
 0x60c452e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60c453c); f_060c41d9(cpu); if cpu.returned_to != 0x60c453c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4543; continue;
 }
 0x60c4543 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.edx))); cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60c455c); f_060bd117(cpu); if cpu.returned_to != 0x60c455c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c4561; continue;
 }
 0x60c4561 => {
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
pub fn f_060c456a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c456a); }
 let mut pc: u32 = 0x60c456a;
 loop { match pc {
 0x60c456a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xbu32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xf8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60c4581; continue; }
  pc = 0x60c457b; continue;
 }
 0x60c457b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c4581 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c4596; continue; }
  pc = 0x60c458d; continue;
 }
 0x60c458d => {
  { cpu.sub8(cpu.r8(0x617b85fu32), 0x0u8); }
  if cpu.zf { pc = 0x60c45a2; continue; }
  pc = 0x60c4596; continue;
 }
 0x60c4596 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x60c45a7; continue; }
  pc = 0x60c45a2; continue;
 }
 0x60c45a2 => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  pc = 0x60c45ae; continue;
 }
 0x60c45a7 => {
  cpu.push(0x60c45ac); f_060c424d(cpu); if cpu.returned_to != 0x60c45ac { return; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  pc = 0x60c45ae; continue;
 }
 0x60c45ae => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x3cu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x60c457b; continue; }
  pc = 0x60c45b7; continue;
 }
 0x60c45b7 => {
  { let v: u32 = cpu.r32(0x617cb3cu32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.cf { pc = 0x60c45c6; continue; }
  pc = 0x60c45c1; continue;
 }
 0x60c45c1 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfeu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c45c6; continue;
 }
 0x60c45c6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx.wrapping_add(0xfffu32); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.ebx))); }
  if cpu.cf { cpu.poll(); pc = 0x60c457b; continue; }
  pc = 0x60c45d2; continue;
 }
 0x60c45d2 => {
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
pub fn f_060c45e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c45e7); }
 let mut pc: u32 = 0x60c45e7;
 loop { match pc {
 0x60c45e7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c45ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c45ea); }
 let mut pc: u32 = 0x60c45ea;
 loop { match pc {
 0x60c45ea => {
  { cpu.sub8(cpu.r8(0x617caa4u32), 0x0u8); }
  if cpu.zf { pc = 0x60c460d; continue; }
  pc = 0x60c45f3; continue;
 }
 0x60c45f3 => {
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.unimplemented(0x60c45fb, "Int3");
  pc = 0x60c4604; continue;
 }
 0x60c4604 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c460d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c4610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4610); }
 let mut pc: u32 = 0x60c4610;
 loop { match pc {
 0x60c4610 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x617b065u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xf8u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x617b065u32, v); }
  { let v: u8 = (cpu.edx as u8) | 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x617b024u32; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x617b065u32, v); }
  pc = 0x60c4676; continue;
 }
 0x60c4634 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x60c463e); f_060bd034(cpu); if cpu.returned_to != 0x60c463e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c4663; continue; }
  pc = 0x60c4644; continue;
 }
 0x60c4644 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x60c464e); f_060bd034(cpu); if cpu.returned_to != 0x60c464e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c4663; continue; }
  pc = 0x60c4654; continue;
 }
 0x60c4654 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x616b2dcu32; cpu.eax = v; }
  cpu.push(0x60c4663); f_060bd34a(cpu); if cpu.returned_to != 0x60c4663 { return; }
  pc = 0x60c4663; continue;
 }
 0x60c4663 => {
  { let v: u32 = cpu.r32(0x62d21b0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d21b0u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c4676; continue;
 }
 0x60c4676 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c4634; continue; }
  pc = 0x60c467d; continue;
 }
 0x60c467d => {
  { let v: u32 = cpu.edx; cpu.w32(0x62d15ccu32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c4687(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4687); }
 let mut pc: u32 = 0x60c4687;
 loop { match pc {
 0x60c4687 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x60c468e); f_060c4698(cpu); if cpu.returned_to != 0x60c468e { return; }
  pc = 0x60c64ed; continue;
 }
 0x60c64ed => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x60c64ee; continue;
 }
 0x60c64ee => {
  { let v: u32 = cpu.r32(0x62d15ccu32); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c6509; continue; }
  pc = 0x60c64f8; continue;
 }
 0x60c64f8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.push(0x60c6501); f_060bd117(cpu); if cpu.returned_to != 0x60c6501 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d15ccu32, v); }
  cpu.poll(); pc = 0x60c64ee; continue;
 }
 0x60c6509 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c4698(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4698); }
 let mut pc: u32 = 0x60c4698;
 loop { match pc {
 0x60c4698 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x617b024u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x62d21b0u32); cpu.eax = v; }
  pc = 0x60c46e5; continue;
 }
 0x60c46ba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x40u8); }
  if !cpu.zf { pc = 0x60c46dd; continue; }
  pc = 0x60c46ca; continue;
 }
 0x60c46ca => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x8u8); }
  if !cpu.zf { pc = 0x60c46dd; continue; }
  pc = 0x60c46d0; continue;
 }
 0x60c46d0 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.cf { pc = 0x60c46e3; continue; }
  pc = 0x60c46d4; continue;
 }
 0x60c46d4 => {
  { cpu.sub32(cpu.eax, 0x617b0a6u32); }
  if !cpu.cf { pc = 0x60c46dd; continue; }
  pc = 0x60c46db; continue;
 }
 0x60c46db => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60c46dd; continue;
 }
 0x60c46dd => {
  cpu.push(0x60c46e2); f_060c6538(cpu); if cpu.returned_to != 0x60c46e2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x60c46e3; continue;
 }
 0x60c46e3 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60c46e5; continue;
 }
 0x60c46e5 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60c46ba; continue; }
  pc = 0x60c46e9; continue;
 }
 0x60c46e9 => {
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
pub fn f_060c46f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c46f6); } r_060c46f6(cpu, 0x60c46f6); }
pub fn f_060c4799(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c4799); } r_060c46f6(cpu, 0x60c4799); }
fn r_060c46f6(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c46f6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60c470f; continue; }
  pc = 0x60c4709; continue;
 }
 0x60c4709 => {
  { let v: u8 = 0x2du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  pc = 0x60c4711; continue;
 }
 0x60c470f => {
  { let v: u8 = 0x2bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c4711; continue;
 }
 0x60c4711 => {
  { let v: u32 = 0x64u32; cpu.ebx = v; }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x60c4734; continue; }
  pc = 0x60c471f; continue;
 }
 0x60c471f => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x60c4734; continue; }
  pc = 0x60c472d; continue;
 }
 0x60c472d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x60c4734; continue;
 }
 0x60c4734 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c4744; continue; }
  pc = 0x60c4738; continue;
 }
 0x60c4738 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x3u32); }
  if !cpu.zf { pc = 0x60c4744; continue; }
  pc = 0x60c4742; continue;
 }
 0x60c4742 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60c4744; continue;
 }
 0x60c4744 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.esp))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c4790; continue; }
  pc = 0x60c474c; continue;
 }
 0x60c474c => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c478d; continue; }
  pc = 0x60c4750; continue;
 }
 0x60c4750 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x60c4752; continue;
 }
 0x60c4752 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c475d; continue; }
  pc = 0x60c4757; continue;
 }
 0x60c4757 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  cpu.poll(); pc = 0x60c4752; continue;
 }
 0x60c475d => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60c475d; continue; }
  pc = 0x60c478b; continue;
 }
 0x60c478b => {
  pc = 0x60c4790; continue;
 }
 0x60c478d => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebp = v; }
  pc = 0x60c4790; continue;
 }
 0x60c4790 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  pc = 0x60c4797; continue;
 }
 0x60c4797 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c4799 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c47ae; continue; }
  pc = 0x60c47a0; continue;
 }
 0x60c47a0 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c47ae; continue; }
  pc = 0x60c47a4; continue;
 }
 0x60c47a4 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c47ae => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c47bf; continue; }
  pc = 0x60c47b2; continue;
 }
 0x60c47b2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x60c47bf; continue; }
  pc = 0x60c47b8; continue;
 }
 0x60c47b8 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60c47ae; continue;
 }
 0x60c47bf => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c47cc; continue; }
  pc = 0x60c47c3; continue;
 }
 0x60c47c3 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60c47c3; continue; }
  pc = 0x60c47cc; continue;
 }
 0x60c47cc => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c47e0; continue; }
  pc = 0x60c47d4; continue;
 }
 0x60c47d4 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c47e0; continue; }
  pc = 0x60c47d8; continue;
 }
 0x60c47d8 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c47d4; continue; }
  pc = 0x60c47e0; continue;
 }
 0x60c47e0 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60c4797; continue; }
  pc = 0x60c47e4; continue;
 }
 0x60c47e4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { pc = 0x60c47f1; continue; }
  pc = 0x60c47ea; continue;
 }
 0x60c47ea => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60c47e4; continue; }
  pc = 0x60c47f1; continue;
 }
 0x60c47f1 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x60c4797; continue; }
  pc = 0x60c47f5; continue;
 }
 0x60c47f5 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60c47f1; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c47fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c47fc); }
 let mut pc: u32 = 0x60c47fc;
 loop { match pc {
 0x60c47fc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.zf { pc = 0x60c4865; continue; }
  pc = 0x60c480d; continue;
 }
 0x60c480d => {
  { let v: u32 = cpu.edi; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x60c481c; continue;
 }
 0x60c481c => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c481c; continue; }
  pc = 0x60c482b; continue;
 }
 0x60c482b => {
  { let v: u32 = cpu.eax ^ cpu.ecx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x60c483b; continue; }
  pc = 0x60c4834; continue;
 }
 0x60c4834 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x60c4859; continue;
 }
 0x60c483b => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x60c4845; continue; }
  pc = 0x60c4840; continue;
 }
 0x60c4840 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x60c4859; continue; }
  pc = 0x60c4845; continue;
 }
 0x60c4845 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x60c4859; continue; }
  pc = 0x60c484f; continue;
 }
 0x60c484f => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  pc = 0x60c4859; continue;
 }
 0x60c4859 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c4865); f_060ae9e0(cpu); if cpu.returned_to != 0x60c4865 { return; }
  pc = 0x60c4865; continue;
 }
 0x60c4865 => {
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
pub fn f_060c486d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c486d); }
 let mut pc: u32 = 0x60c486d;
 loop { match pc {
 0x60c486d => {
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
  cpu.push(0x60c488d); f_060c664e(cpu); if cpu.returned_to != 0x60c488d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c48ad; continue; }
  pc = 0x60c4891; continue;
 }
 0x60c4891 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60c48a0; continue;
 }
 0x60c4898 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x60c489e; continue; }
  pc = 0x60c489c; continue;
 }
 0x60c489c => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c489e; continue;
 }
 0x60c489e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x60c48a0; continue;
 }
 0x60c48a0 => {
  { let v: u8 = cpu.r8(0xffffffc0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebp.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60c4898; continue; }
  pc = 0x60c48a8; continue;
 }
 0x60c48a8 => {
  pc = 0x60c4a8a; continue;
 }
 0x60c48ad => {
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x47u32); }
  if !cpu.zf { pc = 0x60c4922; continue; }
  pc = 0x60c48bb; continue;
 }
 0x60c48bb => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.esi, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x60c48d4; continue; }
  pc = 0x60c48d0; continue;
 }
 0x60c48d0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c491b; continue; }
  pc = 0x60c48d4; continue;
 }
 0x60c48d4 => {
  { let v = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  cpu.push(0x60c48dc); f_060c6727(cpu); if cpu.returned_to != 0x60c48dc { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffd4u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60c48ec); f_060c674f(cpu); if cpu.returned_to != 0x60c48ec { return; }
  cpu.push(0x60c48f1); f_0609c3e2(cpu); if cpu.returned_to != 0x60c48f1 { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xfffffffcu32); }
  if (cpu.sf != cpu.of) { pc = 0x60c4900; continue; }
  pc = 0x60c48fc; continue;
 }
 0x60c48fc => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c4904; continue; }
  pc = 0x60c4900; continue;
 }
 0x60c4900 => {
  { let v: u8 = 0x45u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c4922; continue;
 }
 0x60c4904 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c4911; continue; }
  pc = 0x60c490e; continue;
 }
 0x60c490e => {
  { let v: u32 = cpu.inc32(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c4911; continue;
 }
 0x60c4911 => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60c491b; continue; }
  pc = 0x60c491a; continue;
 }
 0x60c491a => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x60c491b; continue;
 }
 0x60c491b => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u8 = 0x46u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c4922; continue;
 }
 0x60c4922 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x60c4951; continue; }
  pc = 0x60c492b; continue;
 }
 0x60c492b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60c493d; continue; }
  pc = 0x60c4936; continue;
 }
 0x60c4936 => {
  { let v: u32 = 0x2u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c4951; continue; }
  pc = 0x60c493d; continue;
 }
 0x60c493d => {
  { let v: u32 = 0x2au32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60c494c); f_060ae9e0(cpu); if cpu.returned_to != 0x60c494c { return; }
  pc = 0x60c4a9e; continue;
 }
 0x60c4951 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if !cpu.zf { pc = 0x60c496e; continue; }
  pc = 0x60c495d; continue;
 }
 0x60c495d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c4969; continue; }
  pc = 0x60c4964; continue;
 }
 0x60c4964 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  pc = 0x60c496e; continue;
 }
 0x60c4969 => {
  if (cpu.sf == cpu.of) { pc = 0x60c496e; continue; }
  pc = 0x60c496b; continue;
 }
 0x60c496b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x60c496e; continue;
 }
 0x60c496e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x60c4987; continue; }
  pc = 0x60c4983; continue;
 }
 0x60c4983 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60c49a8; continue; }
  pc = 0x60c4987; continue;
 }
 0x60c4987 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x45u32); }
  if cpu.zf { pc = 0x60c49a8; continue; }
  pc = 0x60c4990; continue;
 }
 0x60c4990 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c49a8; continue; }
  pc = 0x60c4997; continue;
 }
 0x60c4997 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60c49a5); f_060aed0b(cpu); if cpu.returned_to != 0x60c49a5 { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffdcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60c49a8; continue;
 }
 0x60c49a8 => {
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
  cpu.push(0x60c49c7); f_060c678b(cpu); if cpu.returned_to != 0x60c49c7 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60c49e0; continue; }
  pc = 0x60c49d5; continue;
 }
 0x60c49d5 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x60c49ed; continue;
 }
 0x60c49e0 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), ((cpu.ecx >> 8) as u8)); }
  if cpu.zf { pc = 0x60c49ed; continue; }
  pc = 0x60c49e4; continue;
 }
 0x60c49e4 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = 0x2bu8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x60c49ed; continue;
 }
 0x60c49ed => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x45u32); }
  if !cpu.zf { pc = 0x60c4a5f; continue; }
  pc = 0x60c49f9; continue;
 }
 0x60c49f9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x60c4a88; continue; }
  pc = 0x60c4a08; continue;
 }
 0x60c4a08 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x60c4a15); f_060c4799(cpu); if cpu.returned_to != 0x60c4a15 { return; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x60c4a31; continue; }
  pc = 0x60c4a26; continue;
 }
 0x60c4a26 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x60c4a31; continue; }
  pc = 0x60c4a2a; continue;
 }
 0x60c4a2a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c4a31; continue;
 }
 0x60c4a31 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x60c4a43; continue; }
  pc = 0x60c4a3f; continue;
 }
 0x60c4a3f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c4a49; continue; }
  pc = 0x60c4a43; continue;
 }
 0x60c4a43 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)).wrapping_sub(cpu.eax); cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c4a49; continue;
 }
 0x60c4a49 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.esi); cpu.ecx = v; }
  cpu.push(0x60c4a5b); f_060c46f6(cpu); if cpu.returned_to != 0x60c4a5b { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x60c4a8a; continue;
 }
 0x60c4a5f => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x60c4a88; continue; }
  pc = 0x60c4a73; continue;
 }
 0x60c4a73 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  cpu.push(0x60c4a86); f_060c4799(cpu); if cpu.returned_to != 0x60c4a86 { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  pc = 0x60c4a88; continue;
 }
 0x60c4a88 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x60c4a8a; continue;
 }
 0x60c4a8a => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60c493d; continue; }
  pc = 0x60c4a92; continue;
 }
 0x60c4a92 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x60c4a9e); f_060c47fc(cpu); if cpu.returned_to != 0x60c4a9e { return; }
  pc = 0x60c4a9e; continue;
 }
 0x60c4a9e => {
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c4aa7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4aa7); }
 let mut pc: u32 = 0x60c4aa7;
 loop { match pc {
 0x60c4aa7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x20u8); }
  if !cpu.zf { pc = 0x60c4ad4; continue; }
  pc = 0x60c4ab2; continue;
 }
 0x60c4ab2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  cpu.push(0x60c4aba); f_060c14eb(cpu); if cpu.returned_to != 0x60c4aba { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c4ad4; continue; }
  pc = 0x60c4abe; continue;
 }
 0x60c4abe => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x20u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  { cpu.and8((cpu.ebx as u8), 0x7u8); }
  if !cpu.zf { pc = 0x60c4ad4; continue; }
  pc = 0x60c4acc; continue;
 }
 0x60c4acc => {
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x2u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xdu32.wrapping_add(cpu.edx), v); }
  pc = 0x60c4ad4; continue;
 }
 0x60c4ad4 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c4ad8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4ad8); }
 let mut pc: u32 = 0x60c4ad8;
 loop { match pc {
 0x60c4ad8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  cpu.push(0x60c4aea); f_060c1512(cpu); if cpu.returned_to != 0x60c4aea { return; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x60c4b1d; continue; }
  pc = 0x60c4aee; continue;
 }
 0x60c4aee => {
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
  if (cpu.sf == cpu.of) { pc = 0x60c4b1d; continue; }
  pc = 0x60c4b10; continue;
 }
 0x60c4b10 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60c4b1b); f_060c147a(cpu); if cpu.returned_to != 0x60c4b1b { return; }
  pc = 0x60c4b44; continue;
 }
 0x60c4b1d => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60c4b10; continue; }
  pc = 0x60c4b34; continue;
 }
 0x60c4b34 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x60c4b42; continue; }
  pc = 0x60c4b38; continue;
 }
 0x60c4b38 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x60c4b42); f_060aec9b(cpu); if cpu.returned_to != 0x60c4b42 { return; }
  pc = 0x60c4b42; continue;
 }
 0x60c4b42 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60c4b44; continue;
 }
 0x60c4b44 => {
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
pub fn f_060c4b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4b50); }
 let mut pc: u32 = 0x60c4b50;
 loop { match pc {
 0x60c4b50 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x60c4b73; continue; }
  pc = 0x60c4b5a; continue;
 }
 0x60c4b5a => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { pc = 0x60c4b66; continue; }
  pc = 0x60c4b5f; continue;
 }
 0x60c4b5f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c4b66 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), (cpu.ecx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x60c4b5a; continue; }
  pc = 0x60c4b73; continue;
 }
 0x60c4b73 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c4b78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4b78); }
 let mut pc: u32 = 0x60c4b78;
 loop { match pc {
 0x60c4b78 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x617b000u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x60c4b96; continue; }
  pc = 0x60c4b8f; continue;
 }
 0x60c4b8f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c4d38; continue;
 }
 0x60c4b96 => {
  { let v: u32 = 0x42u32; cpu.eax = v; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60c4bbd; continue; }
  pc = 0x60c4bb4; continue;
 }
 0x60c4bb4 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c4d36; continue; }
  pc = 0x60c4bbd; continue;
 }
 0x60c4bbd => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c4bdc; continue; }
  pc = 0x60c4bd7; continue;
 }
 0x60c4bd7 => {
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { pc = 0x60c4c11; continue; }
  pc = 0x60c4bdc; continue;
 }
 0x60c4bdc => {
  { let v = cpu.edi; cpu.set_segment(5, v as u16); }
  { let v: u32 = 0x616b384u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x616b384u32); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x60c4c0f; continue; }
  pc = 0x60c4bf4; continue;
 }
 0x60c4bf4 => {
  { let v = cpu.esi; cpu.set_segment(5, v as u16); }
  pc = 0x60c4bf6; continue;
 }
 0x60c4bf6 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x60c4c02; continue; }
  pc = 0x60c4bfb; continue;
 }
 0x60c4bfb => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c4c11; continue;
 }
 0x60c4c02 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), (cpu.ecx as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x60c4bf6; continue; }
  pc = 0x60c4c0f; continue;
 }
 0x60c4c0f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c4c11; continue;
 }
 0x60c4c11 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c4d0a; continue; }
  pc = 0x60c4c19; continue;
 }
 0x60c4c19 => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d2252u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4cef; continue;
 }
 0x60c4c4e => {
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c4c73; continue; }
  pc = 0x60c4c6a; continue;
 }
 0x60c4c6a => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x60c4cdd; continue; }
  pc = 0x60c4c73; continue;
 }
 0x60c4c73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4c80; continue;
 }
 0x60c4c80 => {
  { let v: u32 = cpu.r32(0x617caacu32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { pc = 0x60c4cbc; continue; }
  pc = 0x60c4c9f; continue;
 }
 0x60c4c9f => {
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  pc = 0x60c4ca3; continue;
 }
 0x60c4ca3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x60c4caf; continue; }
  pc = 0x60c4ca8; continue;
 }
 0x60c4ca8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c4cbe; continue;
 }
 0x60c4caf => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8((cpu.ebx as u8), ((cpu.ebx >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x60c4ca3; continue; }
  pc = 0x60c4cbc; continue;
 }
 0x60c4cbc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c4cbe; continue;
 }
 0x60c4cbe => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c4cd5; continue; }
  pc = 0x60c4cc2; continue;
 }
 0x60c4cc2 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617cab0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c4cd5; continue;
 }
 0x60c4cd5 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60c4c80; continue; }
  pc = 0x60c4cdd; continue;
 }
 0x60c4cdd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4cef; continue;
 }
 0x60c4cef => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60c4c4e; continue; }
  pc = 0x60c4d0a; continue;
 }
 0x60c4d0a => {
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c4bbd; continue; }
  pc = 0x60c4d2d; continue;
 }
 0x60c4d2d => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { cpu.poll(); pc = 0x60c4bbd; continue; }
  pc = 0x60c4d36; continue;
 }
 0x60c4d36 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x60c4d38; continue;
 }
 0x60c4d38 => {
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
pub fn f_060c4d44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c4d44); }
 let mut pc: u32 = 0x60c4d44;
 loop { match pc {
 0x60c4d44 => {
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
  { let v: u16 = cpu.r16(0x62d2252u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if !cpu.zf { pc = 0x60c4d6e; continue; }
  pc = 0x60c4d67; continue;
 }
 0x60c4d67 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c5142; continue;
 }
 0x60c4d6e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.unimplemented(0x60c4d73, "Lsl");
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { pc = 0x60c4d84; continue; }
  pc = 0x60c4d7d; continue;
 }
 0x60c4d7d => {
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x60c4d90; continue; }
  pc = 0x60c4d84; continue;
 }
 0x60c4d84 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c4d8f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c4d90 => {
  { let v: u32 = 0x20u32; cpu.ebp = v; }
  { let v = cpu.r16(0x62d2252u32.wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.cf || cpu.zf) { pc = 0x60c4e71; continue; }
  pc = 0x60c4db1; continue;
 }
 0x60c4db1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x617caecu32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x60c4dd0; continue; }
  pc = 0x60c4dbc; continue;
 }
 0x60c4dbc => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x617cb1du32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x60c4dd0; continue; }
  pc = 0x60c4dc4; continue;
 }
 0x60c4dc4 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x617cb29u32.wrapping_add(cpu.bases[3]))); }
  if !cpu.zf { pc = 0x60c4e63; continue; }
  pc = 0x60c4dd0; continue;
 }
 0x60c4dd0 => {
  { let v: u32 = 0x19u32; cpu.eax = v; }
  { let v: u32 = 0x617caecu32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60c4df2; continue; }
  pc = 0x60c4ded; continue;
 }
 0x60c4ded => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x60c4df2; continue;
 }
 0x60c4df2 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c4e0b; continue; }
  pc = 0x60c4df8; continue;
 }
 0x60c4df8 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c4e0b; continue; }
  pc = 0x60c4e04; continue;
 }
 0x60c4e04 => {
  { cpu.sub32(cpu.r32(0x2cu32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60c4e71; continue; }
  pc = 0x60c4e0b; continue;
 }
 0x60c4e0b => {
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { let v: u32 = 0x617cb1du32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60c4e2d; continue; }
  pc = 0x60c4e28; continue;
 }
 0x60c4e28 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x60c4e2d; continue;
 }
 0x60c4e2d => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c4e37; continue; }
  pc = 0x60c4e33; continue;
 }
 0x60c4e33 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4e37; continue;
 }
 0x60c4e37 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x617cb29u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60c4e59; continue; }
  pc = 0x60c4e54; continue;
 }
 0x60c4e54 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x60c4e59; continue;
 }
 0x60c4e59 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c4e63; continue; }
  pc = 0x60c4e5f; continue;
 }
 0x60c4e5f => {
  { let v: u32 = cpu.ebp; cpu.w32(0x2cu32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4e63; continue;
 }
 0x60c4e63 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x60c4db1; continue; }
  pc = 0x60c4e71; continue;
 }
 0x60c4e71 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c4eda; continue; }
  pc = 0x60c4e79; continue;
 }
 0x60c4e79 => {
  { let v = cpu.r16(0x62d2252u32.wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = (cpu.segments[5] as u32); cpu.edx = v; }
  pc = 0x60c4e86; continue;
 }
 0x60c4e86 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u32 = 0x617cb06u32; cpu.edi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx | cpu.ecx; cpu.ecx = v; }
  { cpu.cmps(1, 1); }
  if cpu.zf { pc = 0x60c4ea6; continue; }
  pc = 0x60c4ea1; continue;
 }
 0x60c4ea1 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x60c4ea6; continue;
 }
 0x60c4ea6 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c4ec0; continue; }
  pc = 0x60c4eac; continue;
 }
 0x60c4eac => {
  { cpu.sub8(cpu.r8(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5])), 0x57u8); }
  if !cpu.zf { pc = 0x60c4eb8; continue; }
  pc = 0x60c4eb3; continue;
 }
 0x60c4eb3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5]), v); }
  pc = 0x60c4eb8; continue;
 }
 0x60c4eb8 => {
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c4ec0; continue;
 }
 0x60c4ec0 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, 0x20u32); }
  if cpu.cf { cpu.poll(); pc = 0x60c4e86; continue; }
  pc = 0x60c4ec7; continue;
 }
 0x60c4ec7 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60c4eda; continue; }
  pc = 0x60c4ece; continue;
 }
 0x60c4ece => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c4ed9); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c4eda => {
  { let v: u16 = cpu.r16(0x617cab2u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.unimplemented(0x60c4ee5, "Lsl");
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { pc = 0x60c5033; continue; }
  pc = 0x60c4ef3; continue;
 }
 0x60c4ef3 => {
  { cpu.sub32(cpu.eax, 0xffffu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c5033; continue; }
  pc = 0x60c4efe; continue;
 }
 0x60c4efe => {
  { let v: u16 = cpu.r16(0x617cab2u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x60c4f5c; continue; }
  pc = 0x60c4f1a; continue;
 }
 0x60c4f1a => {
  { let v = cpu.r16(0x34u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  pc = 0x60c4f1e; continue;
 }
 0x60c4f1e => {
  { let v: u8 = cpu.r8(0x617cb0du32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.ecx >> 8) as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5]))); }
  if !cpu.zf { pc = 0x60c4f52; continue; }
  pc = 0x60c4f2a; continue;
 }
 0x60c4f2a => {
  { let v: u32 = 0xbu32; cpu.eax = v; }
  { let v: u32 = 0x617cb0du32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60c4f4c; continue; }
  pc = 0x60c4f47; continue;
 }
 0x60c4f47 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x60c4f4c; continue;
 }
 0x60c4f4c => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60c4f5c; continue; }
  pc = 0x60c4f52; continue;
 }
 0x60c4f52 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { cpu.poll(); pc = 0x60c4f1e; continue; }
  pc = 0x60c4f5c; continue;
 }
 0x60c4f5c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x617cb19u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60c4f85; continue; }
  pc = 0x60c4f80; continue;
 }
 0x60c4f80 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ecx, 0xffffffffu32); cpu.ecx = v; }
  pc = 0x60c4f85; continue;
 }
 0x60c4f85 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60c4fdb; continue; }
  pc = 0x60c4f8b; continue;
 }
 0x60c4f8b => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp.wrapping_add(0xdu32); cpu.ebp = v; }
  { cpu.sub8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x60c5033; continue; }
  pc = 0x60c4f9d; continue;
 }
 0x60c4f9d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x617cab2u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.flat_ds(0x60c4fba); cpu.push(0x60c4fbf); f_060a556c(cpu); if cpu.returned_to != 0x60c4fbf { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5033; continue;
 }
 0x60c4fdb => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u32 = 0xfffffffeu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0x6au8); }
  if !cpu.zf { pc = 0x60c5033; continue; }
  pc = 0x60c4fe6; continue;
 }
 0x60c4fe6 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x62d2258u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if !cpu.zf { pc = 0x60c5033; continue; }
  pc = 0x60c4ff8; continue;
 }
 0x60c4ff8 => {
  { let v: u32 = 0x6u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x617cab2u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.flat_ds(0x60c5015); cpu.push(0x60c501a); f_060a556c(cpu); if cpu.returned_to != 0x60c501a { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5033; continue;
 }
 0x60c5033 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x104u32); }
  if cpu.cf { pc = 0x60c5109; continue; }
  pc = 0x60c5043; continue;
 }
 0x60c5043 => {
  { let v = cpu.r16(0x62d2252u32.wrapping_add(cpu.bases[3])); cpu.set_segment(5, v as u16); }
  { let v: u32 = 0xffffff00u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u16 = (cpu.segments[5] as u16); cpu.w16(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub8((cpu.ecx as u8), 0x2eu8); }
  if cpu.zf { pc = 0x60c5069; continue; }
  pc = 0x60c505d; continue;
 }
 0x60c505d => {
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(((cpu.ecx >> 8) as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x60c5069; continue; }
  pc = 0x60c5067; continue;
 }
 0x60c5067 => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x60c5069; continue;
 }
 0x60c5069 => {
  { let v = cpu.r16(0x38u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.bases[5])), 0x2eu8); }
  if !cpu.zf { pc = 0x60c5109; continue; }
  pc = 0x60c5078; continue;
 }
 0x60c5078 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x60c5080; continue;
 }
 0x60c5080 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[5])), 0x0u8); }
  if !cpu.zf { pc = 0x60c5109; continue; }
  pc = 0x60c508a; continue;
 }
 0x60c508a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { cpu.poll(); pc = 0x60c5080; continue; }
  pc = 0x60c5093; continue;
 }
 0x60c5093 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617aff8u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
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
  pc = 0x60c50b0; continue;
 }
 0x60c50b0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c50ca; continue; }
  pc = 0x60c50b9; continue;
 }
 0x60c50b9 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c50b0; continue; }
  pc = 0x60c50ca; continue;
 }
 0x60c50ca => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.r32(0x617aff8u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.cf || cpu.zf) { pc = 0x60c5109; continue; }
  pc = 0x60c50e5; continue;
 }
 0x60c50e5 => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  pc = 0x60c50ea; continue;
 }
 0x60c50ea => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x5cu8); }
  if cpu.zf { pc = 0x60c50fa; continue; }
  pc = 0x60c50f5; continue;
 }
 0x60c50f5 => {
  { cpu.sub8(((cpu.edx >> 8) as u8), 0x2fu8); }
  if !cpu.zf { pc = 0x60c5105; continue; }
  pc = 0x60c50fa; continue;
 }
 0x60c50fa => {
  { let v = cpu.r16(0x38u32.wrapping_add(cpu.esp)); cpu.set_segment(5, v as u16); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)).wrapping_add(cpu.bases[5]), v); }
  pc = 0x60c5109; continue;
 }
 0x60c5105 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x60c50ea; continue; }
  pc = 0x60c5109; continue;
 }
 0x60c5109 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60c513d; continue; }
  pc = 0x60c5111; continue;
 }
 0x60c5111 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  { let v: u32 = 0x616b394u32; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x62d2252u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
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
  pc = 0x60c513d; continue;
 }
 0x60c513d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c5142; continue;
 }
 0x60c5142 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c514b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c514c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c514c); }
 let mut pc: u32 = 0x60c514c;
 loop { match pc {
 0x60c514c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c5174; continue; }
  pc = 0x60c5156; continue;
 }
 0x60c5156 => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60c5174); f_060a556c(cpu); if cpu.returned_to != 0x60c5174 { return; }
  pc = 0x60c5174; continue;
 }
 0x60c5174 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c517c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c517c); }
 let mut pc: u32 = 0x60c517c;
 loop { match pc {
 0x60c517c => {
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
  cpu.push(0x60c51a8); f_060a556c(cpu); if cpu.returned_to != 0x60c51a8 { return; }
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60c51b6; continue; }
  pc = 0x60c51af; continue;
 }
 0x60c51af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c5298; continue;
 }
 0x60c51b6 => {
  { let v: u32 = 0x9u32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  cpu.unimplemented(0x60c51c9, "Lar");
  { let v: u32 = cpu.ror32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0ffu32; cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xbfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c51e4; continue; }
  pc = 0x60c51df; continue;
 }
 0x60c51df => {
  { let v: u8 = cpu.r8(0x40u32.wrapping_add(cpu.esp)) | 0x9u8; cpu.w8(0x40u32.wrapping_add(cpu.esp), v); }
  pc = 0x60c51e4; continue;
 }
 0x60c51e4 => {
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60c51f6); f_060a556c(cpu); if cpu.returned_to != 0x60c51f6 { return; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60c5233; continue; }
  pc = 0x60c51fe; continue;
 }
 0x60c51fe => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if cpu.zf { pc = 0x60c5227; continue; }
  pc = 0x60c5203; continue;
 }
 0x60c5203 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60c5227); f_060a556c(cpu); if cpu.returned_to != 0x60c5227 { return; }
  pc = 0x60c5227; continue;
 }
 0x60c5227 => {
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
 0x60c5233 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0xffffu32; cpu.ebx = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x44u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60c525f); f_060a556c(cpu); if cpu.returned_to != 0x60c525f { return; }
  { cpu.sub32(cpu.r32(0x50u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60c5296; continue; }
  pc = 0x60c5266; continue;
 }
 0x60c5266 => {
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if cpu.zf { pc = 0x60c528a; continue; }
  pc = 0x60c526b; continue;
 }
 0x60c526b => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60c528a); f_060a556c(cpu); if cpu.returned_to != 0x60c528a { return; }
  pc = 0x60c528a; continue;
 }
 0x60c528a => {
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
 0x60c5296 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60c5298; continue;
 }
 0x60c5298 => {
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
pub fn f_060c52a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c52a4); }
 let mut pc: u32 = 0x60c52a4;
 loop { match pc {
 0x60c52a4 => {
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
  cpu.push(0x60c52c2); f_060a556c(cpu); if cpu.returned_to != 0x60c52c2 { return; }
  { let v: u32 = 0x25e9u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x617b83cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x617b000u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x21u32; cpu.eax = v; }
  cpu.push(0x60c52f6); f_060aeac1(cpu); if cpu.returned_to != 0x60c52f6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5300); }
 let mut pc: u32 = 0x60c5300;
 loop { match pc {
 0x60c5300 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub16(cpu.r16(0x62d2254u32), 0x0u16); }
  if cpu.zf { pc = 0x60c5326; continue; }
  pc = 0x60c530b; continue;
 }
 0x60c530b => {
  { cpu.sub16(cpu.r16(0x62d2256u32), 0x0u16); }
  if cpu.zf { pc = 0x60c5326; continue; }
  pc = 0x60c5315; continue;
 }
 0x60c5315 => {
  { cpu.sub16(cpu.r16(0x62d2250u32), 0x0u16); }
  if cpu.zf { pc = 0x60c5326; continue; }
  pc = 0x60c531f; continue;
 }
 0x60c531f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c5326 => {
  cpu.push(0x60c532b); f_060c4b78(cpu); if cpu.returned_to != 0x60c532b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c53cd; continue; }
  pc = 0x60c5333; continue;
 }
 0x60c5333 => {
  cpu.push(0x60c5338); f_060c4d44(cpu); if cpu.returned_to != 0x60c5338 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c53cd; continue; }
  pc = 0x60c5340; continue;
 }
 0x60c5340 => {
  { let v: u32 = 0x6150000u32; cpu.edx = v; }
  cpu.push(0x60c534a); f_060c52a4(cpu); if cpu.returned_to != 0x60c534a { return; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x6150002u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x60c5362); f_060c517c(cpu); if cpu.returned_to != 0x60c5362 { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d2254u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x60c536f); f_060c517c(cpu); if cpu.returned_to != 0x60c536f { return; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d2256u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  cpu.push(0x60c537c); f_060c517c(cpu); if cpu.returned_to != 0x60c537c { return; }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d2250u32, v); }
  { cpu.and16((cpu.esi as u16), (cpu.esi as u16)); }
  if cpu.zf { pc = 0x60c53a4; continue; }
  pc = 0x60c538e; continue;
 }
 0x60c538e => {
  { cpu.sub16(cpu.r16(0x62d2256u32), 0x0u16); }
  if cpu.zf { pc = 0x60c53a4; continue; }
  pc = 0x60c5398; continue;
 }
 0x60c5398 => {
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c53a4; continue; }
  pc = 0x60c539d; continue;
 }
 0x60c539d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c53a4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60c53b1); f_060c514c(cpu); if cpu.returned_to != 0x60c53b1 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2256u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60c53be); f_060c514c(cpu); if cpu.returned_to != 0x60c53be { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60c53cb); f_060c514c(cpu); if cpu.returned_to != 0x60c53cb { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c53cd; continue;
 }
 0x60c53cd => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c53d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c53d0); }
 let mut pc: u32 = 0x60c53d0;
 loop { match pc {
 0x60c53d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x54u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c5402; continue; }
  pc = 0x60c53df; continue;
 }
 0x60c53df => {
  { let v: u32 = 0x1u32; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60c5402); f_060a556c(cpu); if cpu.returned_to != 0x60c5402 { return; }
  pc = 0x60c5402; continue;
 }
 0x60c5402 => {
  { let v: u16 = cpu.r16(0x62d2256u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c542b; continue; }
  pc = 0x60c540d; continue;
 }
 0x60c540d => {
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60c542b); f_060a556c(cpu); if cpu.returned_to != 0x60c542b { return; }
  pc = 0x60c542b; continue;
 }
 0x60c542b => {
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c5459; continue; }
  pc = 0x60c5436; continue;
 }
 0x60c5436 => {
  { let v: u32 = 0x1u32; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60c5459); f_060a556c(cpu); if cpu.returned_to != 0x60c5459 { return; }
  pc = 0x60c5459; continue;
 }
 0x60c5459 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x54u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5460); }
 let mut pc: u32 = 0x60c5460;
 loop { match pc {
 0x60c5460 => {
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
  if (!cpu.cf && !cpu.zf) { pc = 0x60c566d; continue; }
  pc = 0x60c547f; continue;
 }
 0x60c547f => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c5482; continue;
 }
 0x60c5482 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c549a; continue; }
  pc = 0x60c548a; continue;
 }
 0x60c548a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c5482; continue; }
  pc = 0x60c549a; continue;
 }
 0x60c549a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = 0x616b3a4u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60c54ab; continue;
 }
 0x60c54ab => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c54c3; continue; }
  pc = 0x60c54b3; continue;
 }
 0x60c54b3 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c54ab; continue; }
  pc = 0x60c54c3; continue;
 }
 0x60c54c3 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x62d2259u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c54ce; continue;
 }
 0x60c54ce => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c54e6; continue; }
  pc = 0x60c54d6; continue;
 }
 0x60c54d6 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c54ce; continue; }
  pc = 0x60c54e6; continue;
 }
 0x60c54e6 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c54ee); f_060c6968(cpu); if cpu.returned_to != 0x60c54ee { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c54f9; continue; }
  pc = 0x60c54f2; continue;
 }
 0x60c54f2 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60c566f; continue;
 }
 0x60c54f9 => {
  { let v: u32 = cpu.r32(0x617aff8u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x5cu32; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c5507; continue;
 }
 0x60c5507 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c551f; continue; }
  pc = 0x60c550f; continue;
 }
 0x60c550f => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c5507; continue; }
  pc = 0x60c551f; continue;
 }
 0x60c551f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c5527); f_060a2224(cpu); if cpu.returned_to != 0x60c5527 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c5537; continue; }
  pc = 0x60c552b; continue;
 }
 0x60c552b => {
  { let v: u32 = 0x2fu32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c5537); f_060a2224(cpu); if cpu.returned_to != 0x60c5537 { return; }
  pc = 0x60c5537; continue;
 }
 0x60c5537 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c5576; continue; }
  pc = 0x60c553b; continue;
 }
 0x60c553b => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c5541; continue;
 }
 0x60c5541 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c5559; continue; }
  pc = 0x60c5549; continue;
 }
 0x60c5549 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c5541; continue; }
  pc = 0x60c5559; continue;
 }
 0x60c5559 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x62d2259u32; cpu.eax = v; }
  cpu.push(0x60c5566); f_060c6968(cpu); if cpu.returned_to != 0x60c5566 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c5576; continue; }
  pc = 0x60c556a; continue;
 }
 0x60c556a => {
  { let v: u32 = 0x62d2259u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c5576 => {
  { let v: u32 = 0x62d2259u32; cpu.ebx = v; }
  { let v: u32 = 0x616b3acu32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60c5587); f_060c69a3(cpu); if cpu.returned_to != 0x60c5587 { return; }
  { cpu.sub8(cpu.r8(0x62d2259u32), 0x0u8); }
  if cpu.zf { pc = 0x60c559c; continue; }
  pc = 0x60c5590; continue;
 }
 0x60c5590 => {
  { let v: u32 = 0x62d2259u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c559c => {
  { let v: u32 = 0x62d2259u32; cpu.edi = v; }
  { let v: u32 = 0x616b3b4u32; cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(2, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x62d2259u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60c55bf; continue;
 }
 0x60c55bf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c55d7; continue; }
  pc = 0x60c55c7; continue;
 }
 0x60c55c7 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c55bf; continue; }
  pc = 0x60c55d7; continue;
 }
 0x60c55d7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c55df); f_060c6968(cpu); if cpu.returned_to != 0x60c55df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c55ec; continue; }
  pc = 0x60c55e3; continue;
 }
 0x60c55e3 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c55ec => {
  { let v: u32 = 0x616b3c8u32; cpu.eax = v; }
  cpu.push(0x60c55f6); f_060bd199(cpu); if cpu.returned_to != 0x60c55f6 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x5cu32; cpu.edx = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c55fe; continue;
 }
 0x60c55fe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c5616; continue; }
  pc = 0x60c5606; continue;
 }
 0x60c5606 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c55fe; continue; }
  pc = 0x60c5616; continue;
 }
 0x60c5616 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c561e); f_060a2224(cpu); if cpu.returned_to != 0x60c561e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c562e; continue; }
  pc = 0x60c5622; continue;
 }
 0x60c5622 => {
  { let v: u32 = 0x2fu32; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c562e); f_060a2224(cpu); if cpu.returned_to != 0x60c562e { return; }
  pc = 0x60c562e; continue;
 }
 0x60c562e => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c566d; continue; }
  pc = 0x60c5632; continue;
 }
 0x60c5632 => {
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c5638; continue;
 }
 0x60c5638 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c5650; continue; }
  pc = 0x60c5640; continue;
 }
 0x60c5640 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c5638; continue; }
  pc = 0x60c5650; continue;
 }
 0x60c5650 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x62d2259u32; cpu.eax = v; }
  cpu.push(0x60c565d); f_060c6968(cpu); if cpu.returned_to != 0x60c565d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c566d; continue; }
  pc = 0x60c5661; continue;
 }
 0x60c5661 => {
  { let v: u32 = 0x62d2259u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c566d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c566f; continue;
 }
 0x60c566f => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5678(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5678); }
 let mut pc: u32 = 0x60c5678;
 loop { match pc {
 0x60c5678 => {
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
  cpu.push(0x60c56ab); f_060aeac1(cpu); if cpu.returned_to != 0x60c56ab { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.push(0x60c56b5); f_060c3e4b(cpu); if cpu.returned_to != 0x60c56b5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x28u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c56bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c56bc); } r_060c56bc(cpu, 0x60c56bc); }
pub fn f_060c572c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c572c); } r_060c56bc(cpu, 0x60c572c); }
pub fn f_060c57b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c57b7); } r_060c56bc(cpu, 0x60c57b7); }
pub fn f_060c57e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c57e8); } r_060c56bc(cpu, 0x60c57e8); }
pub fn f_060c58c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c58c4); } r_060c56bc(cpu, 0x60c58c4); }
pub fn f_060c5970(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c5970); } r_060c56bc(cpu, 0x60c5970); }
pub fn f_060c5a2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c5a2e); } r_060c56bc(cpu, 0x60c5a2e); }
fn r_060c56bc(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c56bc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60c56c8); f_060c5300(cpu); if cpu.returned_to != 0x60c56c8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c5a28; continue; }
  pc = 0x60c56d0; continue;
 }
 0x60c56d0 => {
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x60c56d8); f_060c5460(cpu); if cpu.returned_to != 0x60c56d8 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  if cpu.zf { pc = 0x60c5722; continue; }
  pc = 0x60c56e0; continue;
 }
 0x60c56e0 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x60c5a28; continue; }
  pc = 0x60c56fc; continue;
 }
 0x60c56fc => {
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x60c5a2eu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x617cab0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x60c5722 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60c5a2e; continue;
 }
 0x60c572c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60c5738); f_060c5300(cpu); if cpu.returned_to != 0x60c5738 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c5a28; continue; }
  pc = 0x60c5740; continue;
 }
 0x60c5740 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x60c5a2eu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x617cab8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x60c57b7 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x6u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x60c5a28; continue; }
  pc = 0x60c57cf; continue;
 }
 0x60c57cf => {
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60c5a2e; continue;
 }
 0x60c57e8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60c57f4); f_060c5300(cpu); if cpu.returned_to != 0x60c57f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c5a28; continue; }
  pc = 0x60c57fc; continue;
 }
 0x60c57fc => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x60c5a28; continue; }
  pc = 0x60c5818; continue;
 }
 0x60c5818 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x60c5a2eu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x617cac8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x60c58c4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60c58d0); f_060c5300(cpu); if cpu.returned_to != 0x60c58d0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c5a28; continue; }
  pc = 0x60c58d8; continue;
 }
 0x60c58d8 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2256u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x22u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x60c5a28; continue; }
  pc = 0x60c58f4; continue;
 }
 0x60c58f4 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x60c5a28; continue; }
  pc = 0x60c5910; continue;
 }
 0x60c5910 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x60c5a2eu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub8(cpu.r8(0x62d2258u32), 0x0u8); }
  if !cpu.zf { pc = 0x60c593a; continue; }
  pc = 0x60c5924; continue;
 }
 0x60c5924 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2256u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x60c5956; continue;
 }
 0x60c593a => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2256u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x60c5956; continue;
 }
 0x60c5956 => {
  { let v: u32 = 0x617cad8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x60c5970 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60c597c); f_060c5300(cpu); if cpu.returned_to != 0x60c597c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c5a28; continue; }
  pc = 0x60c5984; continue;
 }
 0x60c5984 => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  if cpu.cf { pc = 0x60c5a28; continue; }
  pc = 0x60c59a0; continue;
 }
 0x60c59a0 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = 0x60c5a2eu32; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u16 = cpu.r16(0x62d2254u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x617cae0u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u16 = cpu.r16(0x62d2250u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  cpu.call(0x0); return;
 }
 0x60c5a28 => {
  cpu.unimplemented(0x60c5a28, "Int3");
  { let v: u32 = 0x8badf00du32; cpu.eax = v; }
  pc = 0x60c5a2e; continue;
 }
 0x60c5a2e => {
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
pub fn f_060c5a34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5a34); }
 let mut pc: u32 = 0x60c5a34;
 loop { match pc {
 0x60bd3c5 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[5] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x60bd3cd); cpu.push(0x60bd3d2); f_060bd412(cpu); if cpu.returned_to != 0x60bd3d2 { return; }
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
  cpu.flat_ds(0x60bd411); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c5a34 => {
  cpu.poll(); pc = 0x60bd3c5; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5a39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5a39); }
 let mut pc: u32 = 0x60c5a39;
 loop { match pc {
 0x60c5a39 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x60c5a61; continue; }
  pc = 0x60c5a48; continue;
 }
 0x60c5a48 => {
  { cpu.sub8(cpu.r8(0x617b860u32), 0x0u8); }
  if cpu.zf { pc = 0x60c5a61; continue; }
  pc = 0x60c5a51; continue;
 }
 0x60c5a51 => {
  { let v: u16 = cpu.r16(0x617b002u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60c5a57; continue;
 }
 0x60c5a57 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d2360u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c5a61 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c5a72; continue; }
  pc = 0x60c5a6d; continue;
 }
 0x60c5a6d => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c5a7c; continue; }
  pc = 0x60c5a72; continue;
 }
 0x60c5a72 => {
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x60c5a87; continue; }
  pc = 0x60c5a7c; continue;
 }
 0x60c5a7c => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x3u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.eax as u8), 0x34u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x60c5a57; continue;
 }
 0x60c5a87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5a99; continue; }
  pc = 0x60c5a90; continue;
 }
 0x60c5a90 => {
  { cpu.sub8(cpu.r8(0x617b85fu32), 0x0u8); }
  if cpu.zf { pc = 0x60c5aa5; continue; }
  pc = 0x60c5a99; continue;
 }
 0x60c5a99 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x9u32); }
  if !cpu.zf { pc = 0x60c5aaa; continue; }
  pc = 0x60c5aa5; continue;
 }
 0x60c5aa5 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60c5a57; continue;
 }
 0x60c5aaa => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5b20; continue; }
  pc = 0x60c5aaf; continue;
 }
 0x60c5aaf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5b20; continue; }
  pc = 0x60c5abb; continue;
 }
 0x60c5abb => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60c5ae7; continue; }
  pc = 0x60c5ad8; continue;
 }
 0x60c5ad8 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x616b3d0u32; cpu.eax = v; }
  cpu.push(0x60c5ae7); f_060bd34a(cpu); if cpu.returned_to != 0x60c5ae7 { return; }
  pc = 0x60c5ae7; continue;
 }
 0x60c5ae7 => {
  { let v: u32 = 0xfffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x62d2360u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x8u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u16 = cpu.sbb16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x60c5b29; continue; }
  pc = 0x60c5b0d; continue;
 }
 0x60c5b0d => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = 0x616b3f6u32; cpu.eax = v; }
  cpu.push(0x60c5b1c); f_060bd34a(cpu); if cpu.returned_to != 0x60c5b1c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c5b20 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x62d2360u32, v); }
  pc = 0x60c5b29; continue;
 }
 0x60c5b29 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5b2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5b2d); }
 let mut pc: u32 = 0x60c5b2d;
 loop { match pc {
 0x60c5b2d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5b53; continue; }
  pc = 0x60c5b3a; continue;
 }
 0x60c5b3a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5b53; continue; }
  pc = 0x60c5b46; continue;
 }
 0x60c5b46 => {
  { let v: u16 = cpu.r16(0x62d2360u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x1u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x60c5b53; continue;
 }
 0x60c5b53 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5b5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5b5b); }
 let mut pc: u32 = 0x60c5b5b;
 loop { match pc {
 0x60c5b5b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.push(0x60c5b6c); f_060c6a9d(cpu); if cpu.returned_to != 0x60c5b6c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x616b6d0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.push(0x60c5b7e); f_060c6abc(cpu); if cpu.returned_to != 0x60c5b7e { return; }
  { let v: u32 = 0x616b4a5u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60c5b8a); f_060c6abc(cpu); if cpu.returned_to != 0x60c5b8a { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60c5b93); f_060c6abc(cpu); if cpu.returned_to != 0x60c5b93 { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60c5b9f); f_060c15db(cpu); if cpu.returned_to != 0x60c5b9f { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5ba3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5ba3); }
 let mut pc: u32 = 0x60c5ba3;
 loop { match pc {
 0x60c5ba3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  let t = cpu.r32(0x617cb38u32); cpu.push(0x60c5bad); cpu.call(t); if cpu.returned_to != 0x60c5bad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c5bcc; continue; }
  pc = 0x60c5bb1; continue;
 }
 0x60c5bb1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x60c5bbb); f_060c5b5b(cpu); if cpu.returned_to != 0x60c5bbb { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if !cpu.zf { pc = 0x60c5bc7; continue; }
  pc = 0x60c5bc0; continue;
 }
 0x60c5bc0 => {
  cpu.push(0x60c5bc5); f_060aec96(cpu); if cpu.returned_to != 0x60c5bc5 { return; }
  pc = 0x60c5bcc; continue;
 }
 0x60c5bc7 => {
  cpu.push(0x60c5bcc); f_060aeca7(cpu); if cpu.returned_to != 0x60c5bcc { return; }
  pc = 0x60c5bcc; continue;
 }
 0x60c5bcc => {
  { let v = cpu.rf64(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5bd2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c5bd2); } r_060c5bd2(cpu, 0x60c5bd2); }
pub fn f_060c5c67(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c5c67); } r_060c5bd2(cpu, 0x60c5c67); }
fn r_060c5bd2(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c5bd2 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x60c5bdc); cpu.push(0x60c5be1); f_060adc60(cpu); if cpu.returned_to != 0x60c5be1 { return; }
  { let v: u32 = 0x62d3354u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617b840u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x617b858u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x62d2364u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617b840u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60c5c36; continue; }
  pc = 0x60c5c15; continue;
 }
 0x60c5c15 => {
  { let v: u32 = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x60c3e4au32); }
  if cpu.zf { pc = 0x60c5c36; continue; }
  pc = 0x60c5c21; continue;
 }
 0x60c5c21 => {
  { cpu.sub32(cpu.eax, 0x60c5f29u32); }
  if cpu.zf { pc = 0x60c5c36; continue; }
  pc = 0x60c5c28; continue;
 }
 0x60c5c28 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x60c5c2f); cpu.push(0x60c5c34); f_060c6c2b(cpu); if cpu.returned_to != 0x60c5c34 { return; }
  pc = 0x60c5c47; continue;
 }
 0x60c5c36 => {
  { let v: u16 = cpu.r16(0x617cb44u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb40u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  pc = 0x60c5c42; continue;
 }
 0x60c5c42 => {
  cpu.flat_ds(0x60c5c42); cpu.push(0x60c5c47); f_060afa10(cpu); if cpu.returned_to != 0x60c5c47 { return; }
  pc = 0x60c5c47; continue;
 }
 0x60c5c47 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617b840u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.popad(); }
  cpu.flat_ds(0x60c5c66); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x60c5c67 => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  cpu.flat_ds(0x60c5c71); cpu.push(0x60c5c76); f_060adc60(cpu); if cpu.returned_to != 0x60c5c76 { return; }
  { let v: u32 = 0x62d3354u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u16 = (cpu.segments[2] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.eax; cpu.esp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617b840u32.wrapping_add(cpu.bases[3])); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x617b858u32.wrapping_add(cpu.bases[3])); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x62d2364u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617b840u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60c5cce; continue; }
  pc = 0x60c5caa; continue;
 }
 0x60c5caa => {
  { let v: u32 = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x60c3e4au32); }
  if cpu.zf { pc = 0x60c5cce; continue; }
  pc = 0x60c5cb6; continue;
 }
 0x60c5cb6 => {
  { cpu.sub32(cpu.eax, 0x60c5d73u32); }
  if cpu.zf { pc = 0x60c5cce; continue; }
  pc = 0x60c5cbd; continue;
 }
 0x60c5cbd => {
  { let v: u32 = 0x7u32; cpu.eax = v; }
  {  }
  { cpu.df = false; }
  cpu.flat_ds(0x60c5cc4); cpu.push(0x60c5cc9); f_060c6c2b(cpu); if cpu.returned_to != 0x60c5cc9 { return; }
  cpu.poll(); pc = 0x60c5c47; continue;
 }
 0x60c5cce => {
  { let v: u16 = cpu.r16(0x617cb4au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb46u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  cpu.poll(); pc = 0x60c5c42; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5cdf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5cdf); }
 let mut pc: u32 = 0x60c5cdf;
 loop { match pc {
 0x60c5cdf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x617cb58u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60c5d66; continue; }
  pc = 0x60c5cf3; continue;
 }
 0x60c5cf3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60c5d0a; continue; }
  pc = 0x60c5cff; continue;
 }
 0x60c5cff => {
  { let v: u16 = 0x1686u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x2f); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60c5d5c; continue; }
  pc = 0x60c5d0a; continue;
 }
 0x60c5d0a => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c5d5c; continue; }
  pc = 0x60c5d50; continue;
 }
 0x60c5d50 => {
  { let v: u32 = 0x1u32; cpu.w32(0x617cb58u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5d66; continue;
 }
 0x60c5d5c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x617cb58u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5d66; continue;
 }
 0x60c5d66 => {
  { let v: u32 = cpu.r32(0x617cb58u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c5d72); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5d73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5d73); }
 let mut pc: u32 = 0x60c5d73;
 loop { match pc {
 0x60c5d73 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617cb40u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60c5d8e; continue; }
  pc = 0x60c5d80; continue;
 }
 0x60c5d80 => {
  { cpu.sub16(cpu.r16(0x617cb44u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x60c5e61; continue; }
  pc = 0x60c5d8e; continue;
 }
 0x60c5d8e => {
  { let v: u32 = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x60c5f24u32); }
  if !cpu.zf { pc = 0x60c5da8; continue; }
  pc = 0x60c5d9c; continue;
 }
 0x60c5d9c => {
  { let v: u32 = 0x60c5f29u32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5dba; continue;
 }
 0x60c5da8 => {
  { cpu.sub32(cpu.ebx, 0x60c5d73u32); }
  if !cpu.zf { pc = 0x60c5dba; continue; }
  pc = 0x60c5db0; continue;
 }
 0x60c5db0 => {
  { let v: u32 = 0x60c3e4au32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5dba; continue;
 }
 0x60c5dba => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c5e03; continue; }
  pc = 0x60c5dc6; continue;
 }
 0x60c5dc6 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c5e03; continue; }
  pc = 0x60c5dcb; continue;
 }
 0x60c5dcb => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb44u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb40u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb50u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb4cu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x60c5e50; continue;
 }
 0x60c5e03 => {
  cpu.flat_ds(0x60c5e03); cpu.push(0x60c5e08); f_060c5cdf(cpu); if cpu.returned_to != 0x60c5e08 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5e39; continue; }
  pc = 0x60c5e0d; continue;
 }
 0x60c5e0d => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb44u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb40u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb50u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb4cu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x60c5e50; continue;
 }
 0x60c5e39 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x617cb44u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb40u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x60c5e4b); cpu.push(0x60c5e50); f_060aea7e(cpu); if cpu.returned_to != 0x60c5e50 { return; }
  pc = 0x60c5e50; continue;
 }
 0x60c5e50 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x617cb44u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x617cb40u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5e61; continue;
 }
 0x60c5e61 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c5e65); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5f24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5f24); }
 let mut pc: u32 = 0x60c5f24;
 loop { match pc {
 0x60c5f24 => {
  cpu.push(0x60c5f29); f_060c5d73(cpu); if cpu.returned_to != 0x60c5f29 { return; }
  f_060c5f29(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5f29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5f29); }
 let mut pc: u32 = 0x60c5f29;
 loop { match pc {
 0x60c5e66 => {
  { cpu.sub32(cpu.ebx, 0x60c5f29u32); }
  if !cpu.zf { pc = 0x60c5e78; continue; }
  pc = 0x60c5e6e; continue;
 }
 0x60c5e6e => {
  { let v: u32 = 0x60c3e4au32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5e78; continue;
 }
 0x60c5e78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c5ec1; continue; }
  pc = 0x60c5e84; continue;
 }
 0x60c5e84 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c5ec1; continue; }
  pc = 0x60c5e89; continue;
 }
 0x60c5e89 => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb4au32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb46u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2505u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb56u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb52u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x60c5f0e; continue;
 }
 0x60c5ec1 => {
  cpu.flat_ds(0x60c5ec1); cpu.push(0x60c5ec6); f_060c5cdf(cpu); if cpu.returned_to != 0x60c5ec6 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c5ef7; continue; }
  pc = 0x60c5ecb; continue;
 }
 0x60c5ecb => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb4au32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb46u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x201u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x617cb56u32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb52u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x60c5f0e; continue;
 }
 0x60c5ef7 => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x617cb4au32.wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb46u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  cpu.flat_ds(0x60c5f09); cpu.push(0x60c5f0e); f_060aea7e(cpu); if cpu.returned_to != 0x60c5f0e { return; }
  pc = 0x60c5f0e; continue;
 }
 0x60c5f0e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x617cb4au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x617cb46u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c5f1f; continue;
 }
 0x60c5f1f => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c5f23); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c5f29 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617cb46u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60c5f40; continue; }
  pc = 0x60c5f36; continue;
 }
 0x60c5f36 => {
  { cpu.sub16(cpu.r16(0x617cb4au32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { cpu.poll(); pc = 0x60c5f1f; continue; }
  pc = 0x60c5f40; continue;
 }
 0x60c5f40 => {
  { let v: u32 = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x60c5f24u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60c5e66; continue; }
  pc = 0x60c5f52; continue;
 }
 0x60c5f52 => {
  { let v: u32 = 0x60c5d73u32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60c5e78; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c5f61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c5f61); }
 let mut pc: u32 = 0x60c5f61;
 loop { match pc {
 0x60c5f61 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617cb40u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60c6078; continue; }
  pc = 0x60c5f71; continue;
 }
 0x60c5f71 => {
  { cpu.sub16(cpu.r16(0x617cb44u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x60c6078; continue; }
  pc = 0x60c5f7f; continue;
 }
 0x60c5f7f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c5fde; continue; }
  pc = 0x60c5f8b; continue;
 }
 0x60c5f8b => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c5fde; continue; }
  pc = 0x60c5f90; continue;
 }
 0x60c5f90 => {
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb44u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb40u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x23u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x60c5bd2u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x23u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617cb4cu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x60c604a; continue;
 }
 0x60c5fde => {
  cpu.flat_ds(0x60c5fde); cpu.push(0x60c5fe3); f_060c5cdf(cpu); if cpu.returned_to != 0x60c5fe3 { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c6023; continue; }
  pc = 0x60c5fe8; continue;
 }
 0x60c5fe8 => {
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb44u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb40u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x23u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb50u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb4cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x60c5bd2u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x60c604a; continue;
 }
 0x60c6023 => {
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u32 = 0x60c5bd2u32; cpu.ebx = v; }
  cpu.flat_ds(0x60c602d); cpu.push(0x60c6032); f_060aea4c(cpu); if cpu.returned_to != 0x60c6032 { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617cb40u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x23u32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x617cb44u32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x60c6045); cpu.push(0x60c604a); f_060aea7e(cpu); if cpu.returned_to != 0x60c604a { return; }
  pc = 0x60c604a; continue;
 }
 0x60c604a => {
  { let v: u32 = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x60c3e4au32); }
  if !cpu.zf { pc = 0x60c6066; continue; }
  pc = 0x60c6058; continue;
 }
 0x60c6058 => {
  { let v: u32 = 0x60c5d73u32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c6065); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6066 => {
  { cpu.sub32(cpu.ebx, 0x60c5f29u32); }
  if !cpu.zf { pc = 0x60c6078; continue; }
  pc = 0x60c606e; continue;
 }
 0x60c606e => {
  { let v: u32 = 0x60c5f24u32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c6078; continue;
 }
 0x60c6078 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c607b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c607c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c607c); }
 let mut pc: u32 = 0x60c607c;
 loop { match pc {
 0x60c607c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617cb46u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60c6193; continue; }
  pc = 0x60c608c; continue;
 }
 0x60c608c => {
  { cpu.sub16(cpu.r16(0x617cb4au32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x60c6193; continue; }
  pc = 0x60c609a; continue;
 }
 0x60c609a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c60f9; continue; }
  pc = 0x60c60a6; continue;
 }
 0x60c60a6 => {
  { cpu.sub32(cpu.eax, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60c60f9; continue; }
  pc = 0x60c60ab; continue;
 }
 0x60c60ab => {
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ebx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb4au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb46u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x60c5c67u32; cpu.edx = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x1bu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb56u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617cb52u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  pc = 0x60c6165; continue;
 }
 0x60c60f9 => {
  cpu.flat_ds(0x60c60f9); cpu.push(0x60c60fe); f_060c5cdf(cpu); if cpu.returned_to != 0x60c60fe { return; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c613e; continue; }
  pc = 0x60c6103; continue;
 }
 0x60c6103 => {
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb4au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb46u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1bu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb56u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb52u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x60c5c67u32; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  pc = 0x60c6165; continue;
 }
 0x60c613e => {
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u32 = 0x60c5c67u32; cpu.ebx = v; }
  cpu.flat_ds(0x60c6148); cpu.push(0x60c614d); f_060aea4c(cpu); if cpu.returned_to != 0x60c614d { return; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617cb46u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x1bu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x617cb4au32.wrapping_add(cpu.bases[3]), v); }
  cpu.flat_ds(0x60c6160); cpu.push(0x60c6165); f_060aea7e(cpu); if cpu.returned_to != 0x60c6165 { return; }
  pc = 0x60c6165; continue;
 }
 0x60c6165 => {
  { let v: u32 = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x60c3e4au32); }
  if !cpu.zf { pc = 0x60c6181; continue; }
  pc = 0x60c6173; continue;
 }
 0x60c6173 => {
  { let v: u32 = 0x60c5f29u32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c6180); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6181 => {
  { cpu.sub32(cpu.ebx, 0x60c5d73u32); }
  if !cpu.zf { pc = 0x60c6193; continue; }
  pc = 0x60c6189; continue;
 }
 0x60c6189 => {
  { let v: u32 = 0x60c5f24u32; cpu.w32(0x617ca94u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c6193; continue;
 }
 0x60c6193 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60c6196); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c626f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c626f); }
 let mut pc: u32 = 0x60c626f;
 loop { match pc {
 0x60c626f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub32(cpu.eax, cpu.r32(0x617b844u32)); }
  if cpu.cf { pc = 0x60c62da; continue; }
  pc = 0x60c627f; continue;
 }
 0x60c627f => {
  { cpu.sub8(cpu.r8(0x617b85eu32), 0x0u8); }
  if !cpu.zf { pc = 0x60c62ec; continue; }
  pc = 0x60c6288; continue;
 }
 0x60c6288 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0xfu32); cpu.ebp = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x60c629a; continue; }
  pc = 0x60c6295; continue;
 }
 0x60c6295 => {
  { let v: u32 = 0xfffffffu32; cpu.ebp = v; }
  pc = 0x60c629a; continue;
 }
 0x60c629a => {
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
  if (cpu.sf == cpu.of) { pc = 0x60c62c7; continue; }
  pc = 0x60c62b7; continue;
 }
 0x60c62b7 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v = cpu.edi; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x60c62da; continue; }
  pc = 0x60c62c7; continue;
 }
 0x60c62c7 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x60c62cc; continue;
 }
 0x60c62cc => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60c6321; continue; }
  pc = 0x60c62da; continue;
 }
 0x60c62da => {
  cpu.push(0x60c62df); f_060b1377(cpu); if cpu.returned_to != 0x60c62df { return; }
  { let v: u32 = 0x5u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60c632f; continue;
 }
 0x60c62ec => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60c6301; continue; }
  pc = 0x60c62fc; continue;
 }
 0x60c62fc => {
  { let v: u32 = 0xfffffu32; cpu.ebx = v; }
  pc = 0x60c6301; continue;
 }
 0x60c6301 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c631c; continue; }
  pc = 0x60c630d; continue;
 }
 0x60c630d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b85fu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60c631c; continue; }
  pc = 0x60c6319; continue;
 }
 0x60c6319 => {
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  pc = 0x60c631c; continue;
 }
 0x60c631c => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60c62cc; continue;
 }
 0x60c6321 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617b830u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b830u32, v); }
  pc = 0x60c632f; continue;
 }
 0x60c632f => {
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
pub fn f_060c633a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c633a); }
 let mut pc: u32 = 0x60c633a;
 loop { match pc {
 0x60c633a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.sub32(cpu.r32(0x617ca9cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60c6405; continue; }
  pc = 0x60c634f; continue;
 }
 0x60c634f => {
  { let v = cpu.r16(0x617b865u32); cpu.set_segment(4, v as u16); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617b861u32); cpu.edi = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60c6365; continue;
 }
 0x60c6365 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]))); }
  if cpu.zf { pc = 0x60c637d; continue; }
  pc = 0x60c636a; continue;
 }
 0x60c636a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x60c6378; continue; }
  pc = 0x60c6374; continue;
 }
 0x60c6374 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x60c636a; continue;
 }
 0x60c6378 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.poll(); pc = 0x60c6365; continue;
 }
 0x60c637d => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if !cpu.zf { pc = 0x60c6386; continue; }
  pc = 0x60c6381; continue;
 }
 0x60c6381 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c6386; continue;
 }
 0x60c6386 => {
  cpu.push(0x60c638b); f_060bd034(cpu); if cpu.returned_to != 0x60c638b { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c6405; continue; }
  pc = 0x60c6397; continue;
 }
 0x60c6397 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x60c63a6); f_060bd034(cpu); if cpu.returned_to != 0x60c63a6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c63fe; continue; }
  pc = 0x60c63aa; continue;
 }
 0x60c63aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x617ca9cu32, v); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60c63b9; continue;
 }
 0x60c63b9 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])), 0x0u8); }
  if cpu.zf { pc = 0x60c63d9; continue; }
  pc = 0x60c63bf; continue;
 }
 0x60c63bf => {
  { let v: u32 = cpu.r32(0x617ca9cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  pc = 0x60c63c8; continue;
 }
 0x60c63c8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60c63c8; continue; }
  pc = 0x60c63d3; continue;
 }
 0x60c63d3 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60c63b9; continue;
 }
 0x60c63d9 => {
  { let v: u32 = cpu.r32(0x617ca9cu32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617caa0u32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x60c63fc); f_060ae9e0(cpu); if cpu.returned_to != 0x60c63fc { return; }
  pc = 0x60c6405; continue;
 }
 0x60c63fe => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c6405); f_060bd117(cpu); if cpu.returned_to != 0x60c6405 { return; }
  pc = 0x60c6405; continue;
 }
 0x60c6405 => {
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
pub fn f_060c640e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c640e); }
 let mut pc: u32 = 0x60c640e;
 loop { match pc {
 0x60c640e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d15ccu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60c6436; continue; }
  pc = 0x60c641d; continue;
 }
 0x60c641d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi & 0x4003u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15ccu32, v); }
  { let v: u16 = (cpu.edi as u16) | 0x3u16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x60c647d; continue;
 }
 0x60c6436 => {
  { let v: u32 = 0x617b024u32; cpu.ecx = v; }
  pc = 0x60c645d; continue;
 }
 0x60c643d => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x3u8); }
  if !cpu.zf { pc = 0x60c645a; continue; }
  pc = 0x60c6443; continue;
 }
 0x60c6443 => {
  { let v: u32 = 0x8u32; cpu.eax = v; }
  cpu.push(0x60c644d); f_060bd034(cpu); if cpu.returned_to != 0x60c644d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c64a2; continue; }
  pc = 0x60c6453; continue;
 }
 0x60c6453 => {
  { let v: u32 = 0x3u32; cpu.edi = v; }
  pc = 0x60c647d; continue;
 }
 0x60c645a => {
  { let v: u32 = cpu.ecx.wrapping_add(0x1au32); cpu.ecx = v; }
  pc = 0x60c645d; continue;
 }
 0x60c645d => {
  { cpu.sub32(cpu.ecx, 0x617b22cu32); }
  if cpu.cf { cpu.poll(); pc = 0x60c643d; continue; }
  pc = 0x60c6465; continue;
 }
 0x60c6465 => {
  { let v: u32 = 0x22u32; cpu.eax = v; }
  { let v: u32 = 0x4003u32; cpu.edi = v; }
  cpu.push(0x60c6474); f_060bd034(cpu); if cpu.returned_to != 0x60c6474 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c64a2; continue; }
  pc = 0x60c647a; continue;
 }
 0x60c647a => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x60c647d; continue;
 }
 0x60c647d => {
  { let v: u32 = 0x1au32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x60c648b); f_060ae9e0(cpu); if cpu.returned_to != 0x60c648b { return; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x62d21b0u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x62d21b0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60c64ae; continue;
 }
 0x60c64a2 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.push(0x60c64ac); f_060aec9b(cpu); if cpu.returned_to != 0x60c64ac { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60c64ae; continue;
 }
 0x60c64ae => {
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
pub fn f_060c64b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c64b4); }
 let mut pc: u32 = 0x60c64b4;
 loop { match pc {
 0x60c64b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x62d21b0u32; cpu.edx = v; }
  pc = 0x60c64be; continue;
 }
 0x60c64be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c64e9; continue; }
  pc = 0x60c64c4; continue;
 }
 0x60c64c4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if cpu.zf { pc = 0x60c64cf; continue; }
  pc = 0x60c64cb; continue;
 }
 0x60c64cb => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.poll(); pc = 0x60c64be; continue;
 }
 0x60c64cf => {
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ecx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.ecx as u8), 0x3u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x62d15ccu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15ccu32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60c64e9; continue;
 }
 0x60c64e9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c650b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c650b); }
 let mut pc: u32 = 0x60c650b;
 loop { match pc {
 0x60c650b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d21b0u32); cpu.eax = v; }
  pc = 0x60c6514; continue;
 }
 0x60c6514 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c6520; continue; }
  pc = 0x60c6518; continue;
 }
 0x60c6518 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6520 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60c6529; continue; }
  pc = 0x60c6525; continue;
 }
 0x60c6525 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x60c6514; continue;
 }
 0x60c6529 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60c6535); f_060c6538(cpu); if cpu.returned_to != 0x60c6535 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6538(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6538); }
 let mut pc: u32 = 0x60c6538;
 loop { match pc {
 0x60c6538 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x60c6540); f_060c65be(cpu); if cpu.returned_to != 0x60c6540 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60c6549); f_060c64b4(cpu); if cpu.returned_to != 0x60c6549 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c654d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c654d); }
 let mut pc: u32 = 0x60c654d;
 loop { match pc {
 0x60c654d => {
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c6558; continue; }
  pc = 0x60c6555; continue;
 }
 0x60c6555 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x27u32); cpu.eax = v; }
  pc = 0x60c6558; continue;
 }
 0x60c6558 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6559(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6559); }
 let mut pc: u32 = 0x60c6559;
 loop { match pc {
 0x60c6559 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  cpu.push(0x60c6566); f_060c6ced(cpu); if cpu.returned_to != 0x60c6566 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = 0x74u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  pc = 0x60c6570; continue;
 }
 0x60c6570 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  cpu.push(0x60c657a); f_060c654d(cpu); if cpu.returned_to != 0x60c657a { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x60c6570; continue; }
  pc = 0x60c6584; continue;
 }
 0x60c6584 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u8 = 0x5fu8; cpu.w8(0x5u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x60c6595); f_060c654d(cpu); if cpu.returned_to != 0x60c6595 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0xfu32); cpu.eax = v; }
  cpu.push(0x60c65a2); f_060c654d(cpu); if cpu.returned_to != 0x60c65a2 { return; }
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
pub fn f_060c65be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c65be); }
 let mut pc: u32 = 0x60c65be;
 loop { match pc {
 0x60c65be => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60c65da; continue; }
  pc = 0x60c65d0; continue;
 }
 0x60c65d0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60c6645; continue;
 }
 0x60c65da => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x60c65ed; continue; }
  pc = 0x60c65e4; continue;
 }
 0x60c65e4 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60c65eb); f_060c16f6(cpu); if cpu.returned_to != 0x60c65eb { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x60c65ed; continue;
 }
 0x60c65ed => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60c65f4); f_060c6cf3(cpu); if cpu.returned_to != 0x60c65f4 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60c6607; continue; }
  pc = 0x60c65f9; continue;
 }
 0x60c65f9 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60c6607); f_060b1673(cpu); if cpu.returned_to != 0x60c6607 { return; }
  pc = 0x60c6607; continue;
 }
 0x60c6607 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60c6615; continue; }
  pc = 0x60c660b; continue;
 }
 0x60c660b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60c6613); f_060c157c(cpu); if cpu.returned_to != 0x60c6613 { return; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  pc = 0x60c6615; continue;
 }
 0x60c6615 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x60c662a; continue; }
  pc = 0x60c661b; continue;
 }
 0x60c661b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60c6623); f_060bd117(cpu); if cpu.returned_to != 0x60c6623 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c662a; continue;
 }
 0x60c662a => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.ecx)), 0x8u8); }
  if cpu.zf { pc = 0x60c6643; continue; }
  pc = 0x60c6630; continue;
 }
 0x60c6630 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x19u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x60c663c); f_060c6559(cpu); if cpu.returned_to != 0x60c663c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60c6643); f_060b16b6(cpu); if cpu.returned_to != 0x60c6643 { return; }
  pc = 0x60c6643; continue;
 }
 0x60c6643 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60c6645; continue;
 }
 0x60c6645 => {
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
pub fn f_060c664e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c664e); }
 let mut pc: u32 = 0x60c664e;
 loop { match pc {
 0x60c664e => {
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
  if !cpu.zf { pc = 0x60c66d6; continue; }
  pc = 0x60c667b; continue;
 }
 0x60c667b => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.edx, 0xffffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60c6694; continue; }
  pc = 0x60c668e; continue;
 }
 0x60c668e => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c6694; continue;
 }
 0x60c6694 => {
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
  if !cpu.zf { pc = 0x60c66c7; continue; }
  pc = 0x60c66bf; continue;
 }
 0x60c66bf => {
  { let v: u32 = cpu.r32(0x616b4acu32); cpu.edx = v; }
  pc = 0x60c66cd; continue;
 }
 0x60c66c7 => {
  { let v: u32 = cpu.r32(0x616b4b0u32); cpu.edx = v; }
  pc = 0x60c66cd; continue;
 }
 0x60c66cd => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60c66d8; continue;
 }
 0x60c66d6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c66d8; continue;
 }
 0x60c66d8 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c66e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c66e0); } r_060c66e0(cpu, 0x60c66e0); }
pub fn f_060c6727(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c6727); } r_060c66e0(cpu, 0x60c6727); }
fn r_060c66e0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c66e0 => {
  { let v: u8 = 0xau8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c66e2; continue;
 }
 0x60c66e2 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c6707; continue; }
  pc = 0x60c66f5; continue;
 }
 0x60c66f5 => {
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60c6703); f_060c6d35(cpu); if cpu.returned_to != 0x60c6703 { return; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c671f; continue;
 }
 0x60c6707 => {
  { cpu.sub8((cpu.eax as u8), 0xbu8); }
  if !cpu.zf { pc = 0x60c670f; continue; }
  pc = 0x60c670b; continue;
 }
 0x60c670b => {
  { cpu.fpush(std::f64::consts::LOG10_2); }
  pc = 0x60c6719; continue;
 }
 0x60c670f => {
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if !cpu.zf { pc = 0x60c6717; continue; }
  pc = 0x60c6713; continue;
 }
 0x60c6713 => {
  { cpu.fpush(1.0); }
  pc = 0x60c6719; continue;
 }
 0x60c6717 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  pc = 0x60c6719; continue;
 }
 0x60c6719 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60c671f; continue;
 }
 0x60c671f => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6727 => {
  { let v: u8 = 0xbu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x60c66e2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c672b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c672b); }
 let mut pc: u32 = 0x60c672b;
 loop { match pc {
 0x60c672b => {
  { let v = cpu.rf64(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x60c6734); f_060c66e0(cpu); if cpu.returned_to != 0x60c6734 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c674f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c674f); }
 let mut pc: u32 = 0x60c674f;
 loop { match pc {
 0x60c674f => {
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
  cpu.push(0x60c6769); f_060c6d72(cpu); if cpu.returned_to != 0x60c6769 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x60c677e; continue; }
  pc = 0x60c6772; continue;
 }
 0x60c6772 => {
  { let v = cpu.rf64(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf64(0x616b4b4u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffecu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60c677e; continue;
 }
 0x60c677e => {
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
pub fn f_060c678b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c678b); }
 let mut pc: u32 = 0x60c678b;
 loop { match pc {
 0x60c678b => {
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
  cpu.push(0x60c67bc); f_060c664e(cpu); if cpu.returned_to != 0x60c67bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c67c7; continue; }
  pc = 0x60c67c0; continue;
 }
 0x60c67c0 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60c695c; continue;
 }
 0x60c67c7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x60c67f1; continue; }
  pc = 0x60c67e9; continue;
 }
 0x60c67e9 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60c6919; continue; }
  pc = 0x60c67f1; continue;
 }
 0x60c67f1 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x60c6809; continue; }
  pc = 0x60c67fb; continue;
 }
 0x60c67fb => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60c6809; continue;
 }
 0x60c6809 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60c6817); f_060c6d8b(cpu); if cpu.returned_to != 0x60c6817 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60c6821; continue; }
  pc = 0x60c681f; continue;
 }
 0x60c681f => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x60c6821; continue;
 }
 0x60c6821 => {
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
  if (cpu.sf == cpu.of) { pc = 0x60c6841; continue; }
  pc = 0x60c683f; continue;
 }
 0x60c683f => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  pc = 0x60c6841; continue;
 }
 0x60c6841 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.eax, 0x46u32); }
  if !cpu.zf { pc = 0x60c684d; continue; }
  pc = 0x60c684b; continue;
 }
 0x60c684b => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x60c684d; continue;
 }
 0x60c684d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x60c68e7; continue; }
  pc = 0x60c6855; continue;
 }
 0x60c6855 => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c685f; continue; }
  pc = 0x60c685a; continue;
 }
 0x60c685a => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x60c685f; continue;
 }
 0x60c685f => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffdcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60c6875); f_060c6d72(cpu); if cpu.returned_to != 0x60c6875 { return; }
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x60c6889; continue; }
  pc = 0x60c687e; continue;
 }
 0x60c687e => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c6889; continue; }
  pc = 0x60c6882; continue;
 }
 0x60c6882 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60c6889; continue;
 }
 0x60c6889 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60c689d); f_060aecca(cpu); if cpu.returned_to != 0x60c689d { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c68e7; continue; }
  pc = 0x60c68a3; continue;
 }
 0x60c68a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if cpu.zf { pc = 0x60c68b1; continue; }
  pc = 0x60c68ab; continue;
 }
 0x60c68ab => {
  { cpu.sub8(cpu.r8(0xffffffe8u32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x60c68b3; continue; }
  pc = 0x60c68b1; continue;
 }
 0x60c68b1 => {
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  pc = 0x60c68b3; continue;
 }
 0x60c68b3 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60c68bf; continue; }
  pc = 0x60c68b8; continue;
 }
 0x60c68b8 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x60c68c9; continue;
 }
 0x60c68bf => {
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c68c9; continue; }
  pc = 0x60c68c4; continue;
 }
 0x60c68c4 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x60c68c9; continue;
 }
 0x60c68c9 => {
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60c68dd); f_060aecca(cpu); if cpu.returned_to != 0x60c68dd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c68e7; continue; }
  pc = 0x60c68e1; continue;
 }
 0x60c68e1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60c68e7; continue;
 }
 0x60c68e7 => {
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
  if !cpu.zf { pc = 0x60c6919; continue; }
  pc = 0x60c690d; continue;
 }
 0x60c690d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.ebp)), 0x46u32); }
  if !cpu.zf { pc = 0x60c6919; continue; }
  pc = 0x60c6917; continue;
 }
 0x60c6917 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  pc = 0x60c6919; continue;
 }
 0x60c6919 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60c6925; continue; }
  pc = 0x60c691e; continue;
 }
 0x60c691e => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x60c692f; continue;
 }
 0x60c6925 => {
  { cpu.sub32(cpu.ebx, 0x11u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c692f; continue; }
  pc = 0x60c692a; continue;
 }
 0x60c692a => {
  { let v: u32 = 0x11u32; cpu.ebx = v; }
  pc = 0x60c692f; continue;
 }
 0x60c692f => {
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0xffffffc0u32.wrapping_add(cpu.ebp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.push(0x60c6940); f_060bd7eb(cpu); if cpu.returned_to != 0x60c6940 { return; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60c6941; continue;
 }
 0x60c6941 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c6959; continue; }
  pc = 0x60c6949; continue;
 }
 0x60c6949 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60c6941; continue; }
  pc = 0x60c6959; continue;
 }
 0x60c6959 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60c695c; continue;
 }
 0x60c695c => {
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
pub fn f_060c6968(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6968); }
 let mut pc: u32 = 0x60c6968;
 loop { match pc {
 0x60c6968 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x43u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  if cpu.cf { pc = 0x60c6985; continue; }
  pc = 0x60c6976; continue;
 }
 0x60c6976 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x60c6985; continue; }
  pc = 0x60c697b; continue;
 }
 0x60c697b => {
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c6985; continue; }
  pc = 0x60c6980; continue;
 }
 0x60c6980 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x6u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.cf = true; }
  pc = 0x60c6985; continue;
 }
 0x60c6985 => {
  { let v: u32 = cpu.sbb32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60c69a0); f_060c146f(cpu); if cpu.returned_to != 0x60c69a0 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c69a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c69a3); }
 let mut pc: u32 = 0x60c69a3;
 loop { match pc {
 0x60c69a3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.push(0x60c69b5); f_060b1377(cpu); if cpu.returned_to != 0x60c69b5 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60c69c3); f_060c6968(cpu); if cpu.returned_to != 0x60c69c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c6a17; continue; }
  pc = 0x60c69c7; continue;
 }
 0x60c69c7 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x5cu32); }
  if cpu.zf { pc = 0x60c6a0b; continue; }
  pc = 0x60c69d1; continue;
 }
 0x60c69d1 => {
  { cpu.sub32(cpu.ebx, 0x2eu32); }
  if cpu.zf { pc = 0x60c6a0b; continue; }
  pc = 0x60c69d6; continue;
 }
 0x60c69d6 => {
  { cpu.sub32(cpu.ebx, 0x2fu32); }
  if cpu.zf { pc = 0x60c6a0b; continue; }
  pc = 0x60c69db; continue;
 }
 0x60c69db => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if cpu.zf { pc = 0x60c69e9; continue; }
  pc = 0x60c69e0; continue;
 }
 0x60c69e0 => {
  { let v: u32 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if cpu.zf { pc = 0x60c6a0b; continue; }
  pc = 0x60c69e9; continue;
 }
 0x60c69e9 => {
  { let v: u32 = 0x90u32; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60c69f5); f_060b16bb(cpu); if cpu.returned_to != 0x60c69f5 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60c69fc); f_060a4683(cpu); if cpu.returned_to != 0x60c69fc { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x5cu32); }
  if cpu.zf { pc = 0x60c6a0b; continue; }
  pc = 0x60c6a07; continue;
 }
 0x60c6a07 => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60c6a0b; continue;
 }
 0x60c6a0b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x60c6a12); f_060a3b01(cpu); if cpu.returned_to != 0x60c6a12 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6a17 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60c6a1e); f_060bd199(cpu); if cpu.returned_to != 0x60c6a1e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c6a8f; continue; }
  pc = 0x60c6a28; continue;
 }
 0x60c6a28 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x60c6a8f; continue; }
  pc = 0x60c6a31; continue;
 }
 0x60c6a31 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60c6a33; continue;
 }
 0x60c6a33 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x60c6a48; continue; }
  pc = 0x60c6a38; continue;
 }
 0x60c6a38 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3bu32); }
  if cpu.zf { pc = 0x60c6a48; continue; }
  pc = 0x60c6a40; continue;
 }
 0x60c6a40 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60c6a33; continue;
 }
 0x60c6a48 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x60c6a87; continue; }
  pc = 0x60c6a4c; continue;
 }
 0x60c6a4c => {
  { let v: u32 = cpu.r8(0xffffffffu32.wrapping_add(cpu.eax)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x5cu32); }
  if cpu.zf { pc = 0x60c6a63; continue; }
  pc = 0x60c6a55; continue;
 }
 0x60c6a55 => {
  { cpu.sub32(cpu.edi, 0x2fu32); }
  if cpu.zf { pc = 0x60c6a63; continue; }
  pc = 0x60c6a5a; continue;
 }
 0x60c6a5a => {
  { cpu.sub32(cpu.edi, 0x3au32); }
  if cpu.zf { pc = 0x60c6a63; continue; }
  pc = 0x60c6a5f; continue;
 }
 0x60c6a5f => {
  { let v: u8 = 0x5cu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60c6a63; continue;
 }
 0x60c6a63 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60c6a6d); f_060a3b20(cpu); if cpu.returned_to != 0x60c6a6d { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x60c6a76); f_060c6968(cpu); if cpu.returned_to != 0x60c6a76 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60c6a87; continue; }
  pc = 0x60c6a7a; continue;
 }
 0x60c6a7a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60c6a82); f_060aec9b(cpu); if cpu.returned_to != 0x60c6a82 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6a87 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x60c6a8f; continue; }
  pc = 0x60c6a8c; continue;
 }
 0x60c6a8c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60c6a28; continue;
 }
 0x60c6a8f => {
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
pub fn f_060c6a98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6a98); }
 let mut pc: u32 = 0x60c6a98;
 loop { match pc {
 0x60c6a98 => {
  pc = 0x60c6dfa; continue;
 }
 0x60c6dfa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6a9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6a9d); }
 let mut pc: u32 = 0x60c6a9d;
 loop { match pc {
 0x60c6a9d => {
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60c6aa7; continue; }
  pc = 0x60c6aa3; continue;
 }
 0x60c6aa3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6aa7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x617b024u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6abc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6abc); }
 let mut pc: u32 = 0x60c6abc;
 loop { match pc {
 0x60c6abc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x60c6ad2; continue; }
  pc = 0x60c6acb; continue;
 }
 0x60c6acb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60c6ad2); f_060c167f(cpu); if cpu.returned_to != 0x60c6ad2 { return; }
  pc = 0x60c6ad2; continue;
 }
 0x60c6ad2 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c6aee; continue; }
  pc = 0x60c6adc; continue;
 }
 0x60c6adc => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60c6aee; continue;
 }
 0x60c6aee => {
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60c6af2; continue;
 }
 0x60c6af2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60c6b09; continue; }
  pc = 0x60c6afa; continue;
 }
 0x60c6afa => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x60c6b02); f_060c15db(cpu); if cpu.returned_to != 0x60c6b02 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x60c6af2; continue; }
  pc = 0x60c6b07; continue;
 }
 0x60c6b07 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x60c6b09; continue;
 }
 0x60c6b09 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60c6b27; continue; }
  pc = 0x60c6b0d; continue;
 }
 0x60c6b0d => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) | 0x4u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xdu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60c6b27; continue; }
  pc = 0x60c6b1e; continue;
 }
 0x60c6b1e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60c6b25); f_060c16f6(cpu); if cpu.returned_to != 0x60c6b25 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x60c6b27; continue;
 }
 0x60c6b27 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60c6b2f; continue; }
  pc = 0x60c6b2b; continue;
 }
 0x60c6b2b => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.sub32(cpu.esi, cpu.ebp); cpu.esi = v; }
  pc = 0x60c6b2f; continue;
 }
 0x60c6b2f => {
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
pub fn f_060c6b37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6b37); }
 let mut pc: u32 = 0x60c6b37;
 loop { match pc {
 0x60c6b37 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  f_060c6c2b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6b41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6b41); }
 let mut pc: u32 = 0x60c6b41;
 loop { match pc {
 0x60c6b41 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x617cb68u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60c6b70; continue; }
  pc = 0x60c6b54; continue;
 }
 0x60c6b54 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x60c6b70; continue; }
  pc = 0x60c6b59; continue;
 }
 0x60c6b59 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x60c6b70; continue; }
  pc = 0x60c6b5e; continue;
 }
 0x60c6b5e => {
  { let v: u32 = 0x2u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617cb68u32, v); }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x60c6b70); cpu.call(t); if cpu.returned_to != 0x60c6b70 { return; }
  pc = 0x60c6b70; continue;
 }
 0x60c6b70 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6b76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6b76); }
 let mut pc: u32 = 0x60c6b76;
 loop { match pc {
 0x60c6b76 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf != cpu.of) { pc = 0x60c6b83; continue; }
  pc = 0x60c6b7e; continue;
 }
 0x60c6b7e => {
  { cpu.sub32(cpu.eax, 0xcu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60c6b94; continue; }
  pc = 0x60c6b83; continue;
 }
 0x60c6b83 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x60c6b8d); f_060aec9b(cpu); if cpu.returned_to != 0x60c6b8d { return; }
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6b94 => {
  { let v: u32 = 0x60c6b37u32; cpu.w32(0x617cb94u32, v); }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x60c6bbb; continue; }
  pc = 0x60c6ba3; continue;
 }
 0x60c6ba3 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x60c6baf; continue; }
  pc = 0x60c6ba8; continue;
 }
 0x60c6ba8 => {
  cpu.push(0x60c6bad); f_060c5d73(cpu); if cpu.returned_to != 0x60c6bad { return; }
  pc = 0x60c6beb; continue;
 }
 0x60c6baf => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x60c6beb; continue; }
  pc = 0x60c6bb4; continue;
 }
 0x60c6bb4 => {
  cpu.push(0x60c6bb9); f_060c5f61(cpu); if cpu.returned_to != 0x60c6bb9 { return; }
  pc = 0x60c6beb; continue;
 }
 0x60c6bbb => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if !cpu.zf { pc = 0x60c6bd8; continue; }
  pc = 0x60c6bc0; continue;
 }
 0x60c6bc0 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if !cpu.zf { pc = 0x60c6bcc; continue; }
  pc = 0x60c6bc5; continue;
 }
 0x60c6bc5 => {
  cpu.push(0x60c6bca); f_060c5f29(cpu); if cpu.returned_to != 0x60c6bca { return; }
  pc = 0x60c6beb; continue;
 }
 0x60c6bcc => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x60c6beb; continue; }
  pc = 0x60c6bd1; continue;
 }
 0x60c6bd1 => {
  cpu.push(0x60c6bd6); f_060c607c(cpu); if cpu.returned_to != 0x60c6bd6 { return; }
  pc = 0x60c6beb; continue;
 }
 0x60c6bd8 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x60c6beb; continue; }
  pc = 0x60c6bdd; continue;
 }
 0x60c6bdd => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x60c6beb; continue; }
  pc = 0x60c6be1; continue;
 }
 0x60c6be1 => {
  { cpu.sub32(cpu.edx, 0x3u32); }
  if cpu.zf { pc = 0x60c6beb; continue; }
  pc = 0x60c6be6; continue;
 }
 0x60c6be6 => {
  cpu.push(0x60c6beb); f_060c6e15(cpu); if cpu.returned_to != 0x60c6beb { return; }
  pc = 0x60c6beb; continue;
 }
 0x60c6beb => {
  { let v: u32 = cpu.r32(0x617cb60u32.wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb60u32.wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6bfb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c6bfb); } r_060c6bfb(cpu, 0x60c6bfb); }
pub fn f_060c6c2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c6c2b); } r_060c6bfb(cpu, 0x60c6c2b); }
fn r_060c6bfb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c6bfb => {
  { let v = 0x6cu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.unimplemented(0x60c6c00, "Insb");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.r32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), 117u32); cpu.w32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  cpu.unimplemented(0x60c6c08, "Insb");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.r32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), 193u32); cpu.w32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  cpu.unimplemented(0x60c6c10, "Insb");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  cpu.unimplemented(0x60c6c14, "Insb");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.r32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), 193u32); cpu.w32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  cpu.unimplemented(0x60c6c1c, "Insb");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.r32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), 193u32); cpu.w32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  cpu.unimplemented(0x60c6c24, "Insb");
  { let v: u8 = (cpu.eax as u8) | 0x6u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.r32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1))), 83u32); cpu.w32(0x6u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617cb60u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c6ce3; continue; }
  pc = 0x60c6c4d; continue;
 }
 0x60c6c2b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617cb60u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xbu32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60c6ce3; continue; }
  pc = 0x60c6c4d; continue;
 }
 0x60c6c4d => {
  cpu.poll(); pc = cpu.r32(0x60c6bfbu32.wrapping_add(cpu.ebx.wrapping_mul(4))); continue;
 }
 0x60c6c55 => {
  { let v: u32 = 0x8cu32; cpu.eax = v; }
  cpu.push(0x60c6c5f); f_060c6b41(cpu); if cpu.returned_to != 0x60c6c5f { return; }
  pc = 0x60c6c5f; continue;
 }
 0x60c6c5f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60c6c61; continue;
 }
 0x60c6c61 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6c6a => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x60c6c75; continue; }
  pc = 0x60c6c70; continue;
 }
 0x60c6c70 => {
  cpu.push(0x60c6c75); f_060c6e03(cpu); if cpu.returned_to != 0x60c6c75 { return; }
  pc = 0x60c6c75; continue;
 }
 0x60c6c75 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6c7d; continue;
 }
 0x60c6c7d => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6c82; continue;
 }
 0x60c6c82 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6c87; continue;
 }
 0x60c6c87 => {
  { let v: u32 = 0x2u32; cpu.w32(0x617cb60u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.push(0x60c6c97); f_060c5d73(cpu); if cpu.returned_to != 0x60c6c97 { return; }
  pc = 0x60c6c97; continue;
 }
 0x60c6c97 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60c6c99; continue;
 }
 0x60c6c99 => {
  let t = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.push(0x60c6c9c); cpu.call(t); if cpu.returned_to != 0x60c6c9c { return; }
  cpu.poll(); pc = 0x60c6c5f; continue;
 }
 0x60c6c9e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6ca6; continue;
 }
 0x60c6ca6 => {
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6cab; continue;
 }
 0x60c6cab => {
  { cpu.sub32(cpu.ebx, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6cb0; continue;
 }
 0x60c6cb0 => {
  { let v: u32 = 0x2u32; cpu.w32(0x617cb60u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60c6cbf); f_060c5f29(cpu); if cpu.returned_to != 0x60c6cbf { return; }
  cpu.poll(); pc = 0x60c6c97; continue;
 }
 0x60c6cc1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6cc9; continue;
 }
 0x60c6cc9 => {
  { cpu.sub32(cpu.ebp, 0x2u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6cce; continue;
 }
 0x60c6cce => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if cpu.zf { cpu.poll(); pc = 0x60c6c5f; continue; }
  pc = 0x60c6cd3; continue;
 }
 0x60c6cd3 => {
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617cb60u32.wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x60c6c99; continue;
 }
 0x60c6ce3 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x60c6c61; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6ced(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6ced); }
 let mut pc: u32 = 0x60c6ced;
 loop { match pc {
 0x60c6ced => {
  { let v: u32 = cpu.r32(0x617b83cu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6cf3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6cf3); }
 let mut pc: u32 = 0x60c6cf3;
 loop { match pc {
 0x60c6cf3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x60c6cff); f_060b1791(cpu); if cpu.returned_to != 0x60c6cff { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60c6d30; continue; }
  pc = 0x60c6d06; continue;
 }
 0x60c6d06 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.edx)), 0x80u8); }
  if cpu.zf { pc = 0x60c6d19; continue; }
  pc = 0x60c6d0c; continue;
 }
 0x60c6d0c => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x60c6d19; continue; }
  pc = 0x60c6d12; continue;
 }
 0x60c6d12 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60c6d19); f_060c6e1a(cpu); if cpu.returned_to != 0x60c6d19 { return; }
  pc = 0x60c6d19; continue;
 }
 0x60c6d19 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if cpu.zf { pc = 0x60c6d30; continue; }
  pc = 0x60c6d1f; continue;
 }
 0x60c6d1f => {
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.edx)), 0x10u8); }
  if cpu.zf { pc = 0x60c6d2d; continue; }
  pc = 0x60c6d25; continue;
 }
 0x60c6d25 => {
  { let v: u32 = cpu.add32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6d2d => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebx = v; }
  pc = 0x60c6d30; continue;
 }
 0x60c6d30 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6d35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6d35); }
 let mut pc: u32 = 0x60c6d35;
 loop { match pc {
 0x60c6d35 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1fu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x8u32); }
  if cpu.zf { pc = 0x60c6d5e; continue; }
  pc = 0x60c6d46; continue;
 }
 0x60c6d46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.and32(cpu.edx, 0x7fffffffu32); }
  if !cpu.zf { pc = 0x60c6d5e; continue; }
  pc = 0x60c6d54; continue;
 }
 0x60c6d54 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60c6d5e; continue; }
  pc = 0x60c6d58; continue;
 }
 0x60c6d58 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1080u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60c6d62; continue;
 }
 0x60c6d5e => {
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x1040u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60c6d62; continue;
 }
 0x60c6d62 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x60c6d6a); f_060c1c03(cpu); if cpu.returned_to != 0x60c6d6a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6d72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6d72); }
 let mut pc: u32 = 0x60c6d72;
 loop { match pc {
 0x60c6d72 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  cpu.push(0x60c6d7e); f_0609c3e2(cpu); if cpu.returned_to != 0x60c6d7e { return; }
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
pub fn f_060c6d8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6d8b); }
 let mut pc: u32 = 0x60c6d8b;
 loop { match pc {
 0x60c6d8b => {
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
  if !cpu.zf { pc = 0x60c6db2; continue; }
  pc = 0x60c6dae; continue;
 }
 0x60c6dae => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60c6de7; continue; }
  pc = 0x60c6db2; continue;
 }
 0x60c6db2 => {
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
  pc = 0x60c6de7; continue;
 }
 0x60c6de7 => {
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
pub fn f_060c6e03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6e03); }
 let mut pc: u32 = 0x60c6e03;
 loop { match pc {
 0x60c6e03 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x616b4bcu32; cpu.eax = v; }
  cpu.push(0x60c6e13); f_060bd34a(cpu); if cpu.returned_to != 0x60c6e13 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6e15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6e15); }
 let mut pc: u32 = 0x60c6e15;
 loop { match pc {
 0x60c6e15 => {
  f_060c6e2a(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6e1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6e1a); }
 let mut pc: u32 = 0x60c6e1a;
 loop { match pc {
 0x60c6e1a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_060c16f6(cpu); return; }
  pc = 0x60c6e22; continue;
 }
 0x60c6e22 => {
  cpu.push(0x60c6e27); f_060c6e5b(cpu); if cpu.returned_to != 0x60c6e27 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6e2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6e2a); }
 let mut pc: u32 = 0x60c6e2a;
 loop { match pc {
 0x60c6e2a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617ca98u32), 0x60c6eedu32); }
  if cpu.zf { pc = 0x60c6e53; continue; }
  pc = 0x60c6e38; continue;
 }
 0x60c6e38 => {
  { let v: u32 = 0x60c6b41u32; cpu.ebx = v; }
  { let v: u32 = 0x60c6eedu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b869u32, v); }
  cpu.push(0x60c6e4d); f_060c6e90(cpu); if cpu.returned_to != 0x60c6e4d { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617ca98u32, v); }
  pc = 0x60c6e53; continue;
 }
 0x60c6e53 => {
  cpu.push(0x60c6e58); f_060c6f4a(cpu); if cpu.returned_to != 0x60c6e58 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6e5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6e5b); }
 let mut pc: u32 = 0x60c6e5b;
 loop { match pc {
 0x60c6e5b => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x62d21b0u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60c6e85; continue;
 }
 0x60c6e6f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x60c6e83; continue; }
  pc = 0x60c6e77; continue;
 }
 0x60c6e77 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x60c6e83; continue; }
  pc = 0x60c6e7e; continue;
 }
 0x60c6e7e => {
  cpu.push(0x60c6e83); f_060c16f6(cpu); if cpu.returned_to != 0x60c6e83 { return; }
  pc = 0x60c6e83; continue;
 }
 0x60c6e83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x60c6e85; continue;
 }
 0x60c6e85 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60c6e6f; continue; }
  pc = 0x60c6e89; continue;
 }
 0x60c6e89 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6e90); }
 let mut pc: u32 = 0x60c6e90;
 loop { match pc {
 0x60c6e90 => {
  { cpu.sub16(cpu.r16(0x617cb98u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x60c6ea5; continue; }
  pc = 0x60c6e9a; continue;
 }
 0x60c6e9a => {
  { cpu.sub8(cpu.r8(0x617412cu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x60c6ea5; continue; }
  pc = 0x60c6ea3; continue;
 }
 0x60c6ea3 => {
  pc = 0x60c6ea6; continue;
 }
 0x60c6ea5 => {
  cpu.flat_ds(0x60c6ea5); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6ea6 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x60c6ebc; continue; }
  pc = 0x60c6eb5; continue;
 }
 0x60c6eb5 => {
  cpu.flat_ds(0x60c6eb5); cpu.push(0x60c6eba); f_060c72e0(cpu); if cpu.returned_to != 0x60c6eba { return; }
  pc = 0x60c6ee1; continue;
 }
 0x60c6ebc => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x60c6ecc; continue; }
  pc = 0x60c6ec5; continue;
 }
 0x60c6ec5 => {
  cpu.flat_ds(0x60c6ec5); cpu.push(0x60c6eca); f_060c729a(cpu); if cpu.returned_to != 0x60c6eca { return; }
  pc = 0x60c6ee1; continue;
 }
 0x60c6ecc => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x60c6edc; continue; }
  pc = 0x60c6ed5; continue;
 }
 0x60c6ed5 => {
  cpu.flat_ds(0x60c6ed5); cpu.push(0x60c6eda); f_060c7338(cpu); if cpu.returned_to != 0x60c6eda { return; }
  pc = 0x60c6ee1; continue;
 }
 0x60c6edc => {
  cpu.flat_ds(0x60c6edc); cpu.push(0x60c6ee1); f_060c71aa(cpu); if cpu.returned_to != 0x60c6ee1 { return; }
  pc = 0x60c6ee1; continue;
 }
 0x60c6ee1 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c6ee2); cpu.push(0x60c6ee7); f_060c6f4a(cpu); if cpu.returned_to != 0x60c6ee7 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x60c6eec); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6eed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6eed); }
 let mut pc: u32 = 0x60c6eed;
 loop { match pc {
 0x60c6eed => {
  { cpu.sub16(cpu.r16(0x617cb98u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x60c6ef8; continue; }
  pc = 0x60c6ef7; continue;
 }
 0x60c6ef7 => {
  cpu.flat_ds(0x60c6ef7); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c6ef8 => {
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
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x60c6f1f; continue; }
  pc = 0x60c6f18; continue;
 }
 0x60c6f18 => {
  cpu.flat_ds(0x60c6f18); cpu.push(0x60c6f1d); f_060c731e(cpu); if cpu.returned_to != 0x60c6f1d { return; }
  pc = 0x60c6f44; continue;
 }
 0x60c6f1f => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x60c6f2f; continue; }
  pc = 0x60c6f28; continue;
 }
 0x60c6f28 => {
  cpu.flat_ds(0x60c6f28); cpu.push(0x60c6f2d); f_060c72c7(cpu); if cpu.returned_to != 0x60c6f2d { return; }
  pc = 0x60c6f44; continue;
 }
 0x60c6f2f => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x60c6f3f; continue; }
  pc = 0x60c6f38; continue;
 }
 0x60c6f38 => {
  cpu.flat_ds(0x60c6f38); cpu.push(0x60c6f3d); f_060c7390(cpu); if cpu.returned_to != 0x60c6f3d { return; }
  pc = 0x60c6f44; continue;
 }
 0x60c6f3f => {
  cpu.flat_ds(0x60c6f3f); cpu.push(0x60c6f44); f_060c726d(cpu); if cpu.returned_to != 0x60c6f44 { return; }
  pc = 0x60c6f44; continue;
 }
 0x60c6f44 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x60c6f48); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c6f4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c6f4a); }
 let mut pc: u32 = 0x60c6f4a;
 loop { match pc {
 0x60c6f4a => {
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
pub fn f_060c6f5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c6f5f); } r_060c6f5f(cpu, 0x60c6f5f); }
pub fn f_060c6f6c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c6f6c); } r_060c6f5f(cpu, 0x60c6f6c); }
fn r_060c6f5f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c6f5f => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { cpu.port_out(0xa0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out(0xf0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x60c6f6c; continue;
 }
 0x60c6f6c => {
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
  cpu.unimplemented(0x60c6f7a, "Fnstenv");
  {  }
  {  }
  {  }
  {  }
  cpu.unimplemented(0x60c6f82, "Fndisi");
  {  }
  cpu.flat_ds(0x60c6f85); cpu.push(0x60c6f8a); f_060adc60(cpu); if cpu.returned_to != 0x60c6f8a { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x60c6f9a; continue;
 }
 0x60c6f9a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if cpu.cf { cpu.poll(); pc = 0x60c6f9a; continue; }
  pc = 0x60c6fa4; continue;
 }
 0x60c6fa4 => {
  { cpu.sub8((cpu.ebx as u8), 0xdfu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x60c6f9a; continue; }
  pc = 0x60c6fa9; continue;
 }
 0x60c6fa9 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let a = (cpu.ebx as u8); let b = ((cpu.ebx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x60c6fbc; continue; }
  pc = 0x60c6fb8; continue;
 }
 0x60c6fb8 => {
  { let v: u8 = 0x86u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c7008; continue;
 }
 0x60c6fbc => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x60c6fd0; continue; }
  pc = 0x60c6fc1; continue;
 }
 0x60c6fc1 => {
  { cpu.and16((cpu.edx as u16), 0x200u16); }
  if !cpu.zf { pc = 0x60c6fcc; continue; }
  pc = 0x60c6fc8; continue;
 }
 0x60c6fc8 => {
  { let v: u8 = 0x8bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c6fce; continue;
 }
 0x60c6fcc => {
  { let v: u8 = 0x8au8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c6fce; continue;
 }
 0x60c6fce => {
  pc = 0x60c7008; continue;
 }
 0x60c6fd0 => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60c6fdc; continue; }
  pc = 0x60c6fd5; continue;
 }
 0x60c6fd5 => {
  cpu.flat_ds(0x60c6fd5); cpu.push(0x60c6fda); f_060c7072(cpu); if cpu.returned_to != 0x60c6fda { return; }
  pc = 0x60c7008; continue;
 }
 0x60c6fdc => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x60c6fe5; continue; }
  pc = 0x60c6fe1; continue;
 }
 0x60c6fe1 => {
  { let v: u8 = 0x82u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c7008; continue;
 }
 0x60c6fe5 => {
  { cpu.and8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x60c6ff3; continue; }
  pc = 0x60c6fea; continue;
 }
 0x60c6fea => {
  cpu.flat_ds(0x60c6fea); cpu.push(0x60c6fef); f_060c70ff(cpu); if cpu.returned_to != 0x60c6fef { return; }
  { let v: u8 = 0x84u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c7008; continue;
 }
 0x60c6ff3 => {
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x60c6ffc; continue; }
  pc = 0x60c6ff8; continue;
 }
 0x60c6ff8 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c7008; continue;
 }
 0x60c6ffc => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c7008; continue; }
  pc = 0x60c7001; continue;
 }
 0x60c7001 => {
  cpu.flat_ds(0x60c7001); cpu.push(0x60c7006); f_060c7151(cpu); if cpu.returned_to != 0x60c7006 { return; }
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c7008; continue;
 }
 0x60c7008 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x60c7058; continue; }
  pc = 0x60c700d; continue;
 }
 0x60c700d => {
  { let v = cpu.r32(0x617b840u32.wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x617cdbau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x617cdbcu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x617cdbau32; cpu.esp = v; }
  { let v: u32 = 0x617cbbau32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617b840u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x617b869u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60c7036); cpu.push(0x60c703c); cpu.call(t); if cpu.returned_to != 0x60c703c { return; }
  { let v = cpu.r16(0x617cdbau32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x617cdbcu32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x617b840u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x617cdbau32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60c7058; continue;
 }
 0x60c7058 => {
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
  cpu.flat_ds(0x60c7071); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c7072(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c7072); }
 let mut pc: u32 = 0x60c7072;
 loop { match pc {
 0x60c7072 => {
  { let v: u8 = 0x81u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xd9fau16); }
  if !cpu.zf { pc = 0x60c707e; continue; }
  pc = 0x60c707b; continue;
 }
 0x60c707b => {
  { let v: u8 = 0x88u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c707e => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f1u16); }
  if !cpu.zf { pc = 0x60c7088; continue; }
  pc = 0x60c7085; continue;
 }
 0x60c7085 => {
  { let v: u8 = 0x8eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c7088 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f8u16); }
  if cpu.zf { pc = 0x60c7094; continue; }
  pc = 0x60c708f; continue;
 }
 0x60c708f => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f5u16); }
  pc = 0x60c7094; continue;
 }
 0x60c7094 => {
  if !cpu.zf { pc = 0x60c709a; continue; }
  pc = 0x60c7096; continue;
 }
 0x60c7096 => {
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c70fe; continue;
 }
 0x60c709a => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x310u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x310u16); }
  if !cpu.zf { pc = 0x60c70ad; continue; }
  pc = 0x60c70a9; continue;
 }
 0x60c70a9 => {
  { let v: u8 = 0x8du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c70fe; continue;
 }
 0x60c70ad => {
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x60c70d7; continue; }
  pc = 0x60c70b9; continue;
 }
 0x60c70b9 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x60c70cc; continue; }
  pc = 0x60c70c7; continue;
 }
 0x60c70c7 => {
  cpu.push(0x60c70cc); f_060c719c(cpu); if cpu.returned_to != 0x60c70cc { return; }
  pc = 0x60c70cc; continue;
 }
 0x60c70cc => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x60c70d3; continue; }
  pc = 0x60c70d1; continue;
 }
 0x60c70d1 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x60c70d3; continue;
 }
 0x60c70d3 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c70fe; continue;
 }
 0x60c70d7 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x130u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x30u16); }
  if !cpu.zf { pc = 0x60c70fe; continue; }
  pc = 0x60c70e5; continue;
 }
 0x60c70e5 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x38u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x60c70fe; continue; }
  pc = 0x60c70fc; continue;
 }
 0x60c70fc => {
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x60c70fe; continue;
 }
 0x60c70fe => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c70ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c70ff); }
 let mut pc: u32 = 0x60c70ff;
 loop { match pc {
 0x60c70ff => {
  { cpu.sub16((cpu.ebx as u16), 0xd9fdu16); }
  if !cpu.zf { pc = 0x60c7108; continue; }
  pc = 0x60c7106; continue;
 }
 0x60c7106 => {
  pc = 0x60c7150; continue;
 }
 0x60c7108 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x60c7134; continue; }
  pc = 0x60c7117; continue;
 }
 0x60c7117 => {
  cpu.push(0x60c711c); f_060c7151(cpu); if cpu.returned_to != 0x60c711c { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x60c712b; continue; }
  pc = 0x60c7126; continue;
 }
 0x60c7126 => {
  cpu.push(0x60c712b); f_060c719c(cpu); if cpu.returned_to != 0x60c712b { return; }
  pc = 0x60c712b; continue;
 }
 0x60c712b => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x60c7132; continue; }
  pc = 0x60c7130; continue;
 }
 0x60c7130 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x60c7132; continue;
 }
 0x60c7132 => {
  pc = 0x60c7150; continue;
 }
 0x60c7134 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x60c714b; continue; }
  pc = 0x60c713e; continue;
 }
 0x60c713e => {
  { let v: u32 = 0x617cba4u32; cpu.esi = v; }
  cpu.push(0x60c7149); f_060c7177(cpu); if cpu.returned_to != 0x60c7149 { return; }
  pc = 0x60c7150; continue;
 }
 0x60c714b => {
  cpu.push(0x60c7150); f_060c7151(cpu); if cpu.returned_to != 0x60c7150 { return; }
  pc = 0x60c7150; continue;
 }
 0x60c7150 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c7151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c7151); }
 let mut pc: u32 = 0x60c7151;
 loop { match pc {
 0x60c7151 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c7166; continue; }
  pc = 0x60c715e; continue;
 }
 0x60c715e => {
  { let v = cpu.rf64(0x617cbaeu32); cpu.fpush(v); }
  pc = 0x60c716c; continue;
 }
 0x60c7166 => {
  { let v = cpu.rf32(0x617cbb6u32); cpu.fpush(v); }
  pc = 0x60c716c; continue;
 }
 0x60c716c => {
  { cpu.and16(cpu.r16(0x14u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if cpu.zf { pc = 0x60c7176; continue; }
  pc = 0x60c7174; continue;
 }
 0x60c7174 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x60c7176; continue;
 }
 0x60c7176 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c7177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c7177); }
 let mut pc: u32 = 0x60c7177;
 loop { match pc {
 0x60c7177 => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if !cpu.zf { pc = 0x60c7180; continue; }
  pc = 0x60c717c; continue;
 }
 0x60c717c => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c7185; continue;
 }
 0x60c7180 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.edx as u8), 0x7u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60c7185; continue;
 }
 0x60c7185 => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x60c7186; continue;
 }
 0x60c7186 => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x60c718e; continue; }
  pc = 0x60c718a; continue;
 }
 0x60c718a => {
  cpu.unimplemented(0x60c718a, "Fincstp");
  cpu.poll(); pc = 0x60c7186; continue;
 }
 0x60c718e => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x60c7193; continue;
 }
 0x60c7193 => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x60c719b; continue; }
  pc = 0x60c7197; continue;
 }
 0x60c7197 => {
  cpu.unimplemented(0x60c7197, "Fdecstp");
  cpu.poll(); pc = 0x60c7193; continue;
 }
 0x60c719b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c719c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c719c); }
 let mut pc: u32 = 0x60c719c;
 loop { match pc {
 0x60c719c => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60c71a6; continue; }
  pc = 0x60c71a1; continue;
 }
 0x60c71a1 => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60c71a9; continue;
 }
 0x60c71a6 => {
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60c71a9; continue;
 }
 0x60c71a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c71aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c71aa); }
 let mut pc: u32 = 0x60c71aa;
 loop { match pc {
 0x60c71aa => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x617cdc3u32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x60c71bc; continue; }
  pc = 0x60c71b4; continue;
 }
 0x60c71b4 => {
  { let v: u32 = 0x60c6f5fu32; cpu.edx = v; }
  pc = 0x60c71c2; continue;
 }
 0x60c71bc => {
  { let v: u32 = 0x60c6f6cu32; cpu.edx = v; }
  pc = 0x60c71c2; continue;
 }
 0x60c71c2 => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x60c71e9; continue; }
  pc = 0x60c71cb; continue;
 }
 0x60c71cb => {
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
  pc = 0x60c71e9; continue;
 }
 0x60c71e9 => {
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x617cdc1u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617cb9au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x617cb98u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617cb9eu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x60c7260; continue; }
  pc = 0x60c7222; continue;
 }
 0x60c7222 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x60c7225, "Sidt");
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
  pc = 0x60c726b; continue;
 }
 0x60c7260 => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2506u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  pc = 0x60c726b; continue;
 }
 0x60c726b => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c726c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c726d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c726d); }
 let mut pc: u32 = 0x60c726d;
 loop { match pc {
 0x60c726d => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x617cdc1u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb9eu32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617cb9au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x617cb98u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c7299); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c729a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c729a); }
 let mut pc: u32 = 0x60c729a;
 loop { match pc {
 0x60c729a => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x617cdc0u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617cb9au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x617cb98u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x617cdc0u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x60c6f6cu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c72c6); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c72c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c72c7); }
 let mut pc: u32 = 0x60c72c7;
 loop { match pc {
 0x60c72c7 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x617cdc0u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb9au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x617cb98u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c72df); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c72e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c72e0); }
 let mut pc: u32 = 0x60c72e0;
 loop { match pc {
 0x60c72e0 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x617cdc2u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617cb9au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x617cb98u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x617cdc2u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x617cdc3u32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x60c730f; continue; }
  pc = 0x60c7307; continue;
 }
 0x60c7307 => {
  { let v: u32 = 0x60c6f5fu32; cpu.edx = v; }
  pc = 0x60c7315; continue;
 }
 0x60c730f => {
  { let v: u32 = 0x60c6f6cu32; cpu.edx = v; }
  pc = 0x60c7315; continue;
 }
 0x60c7315 => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c731d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c731e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c731e); }
 let mut pc: u32 = 0x60c731e;
 loop { match pc {
 0x60c731e => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x617cdc2u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x617cb9au32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x617cb98u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60c7336); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c7338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c7338); }
 let mut pc: u32 = 0x60c7338;
 loop { match pc {
 0x60c7338 => {
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x617cdc0u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.edx; cpu.w32(0x617cb9au32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617cb98u32, v); }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x617cdc0u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617cb9eu32, v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x617cdc3u32), 0x20u8); }
  if !cpu.zf { pc = 0x60c737d; continue; }
  pc = 0x60c7375; continue;
 }
 0x60c7375 => {
  { let v: u32 = 0x60c6f5fu32; cpu.edx = v; }
  pc = 0x60c7383; continue;
 }
 0x60c737d => {
  { let v: u32 = 0x60c6f6cu32; cpu.edx = v; }
  pc = 0x60c7383; continue;
 }
 0x60c7383 => {
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x617cdc0u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c7390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60c7390); }
 let mut pc: u32 = 0x60c7390;
 loop { match pc {
 0x60c7390 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060c7394(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7394); } r_060c7394(cpu, 0x60c7394); }
pub fn f_060c73a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c73a4); } r_060c7394(cpu, 0x60c73a4); }
pub fn f_060c73b5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c73b5); } r_060c7394(cpu, 0x60c73b5); }
pub fn f_060c73c9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c73c9); } r_060c7394(cpu, 0x60c73c9); }
pub fn f_060c73e6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c73e6); } r_060c7394(cpu, 0x60c73e6); }
pub fn f_060c73fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c73fb); } r_060c7394(cpu, 0x60c73fb); }
pub fn f_060c7414(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7414); } r_060c7394(cpu, 0x60c7414); }
pub fn f_060c7427(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7427); } r_060c7394(cpu, 0x60c7427); }
pub fn f_060c7447(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7447); } r_060c7394(cpu, 0x60c7447); }
pub fn f_060c745e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c745e); } r_060c7394(cpu, 0x60c745e); }
pub fn f_060c747b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c747b); } r_060c7394(cpu, 0x60c747b); }
pub fn f_060c748d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c748d); } r_060c7394(cpu, 0x60c748d); }
pub fn f_060c74a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c74a3); } r_060c7394(cpu, 0x60c74a3); }
pub fn f_060c74b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c74b3); } r_060c7394(cpu, 0x60c74b3); }
pub fn f_060c74c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c74c5); } r_060c7394(cpu, 0x60c74c5); }
pub fn f_060c74d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c74d8); } r_060c7394(cpu, 0x60c74d8); }
pub fn f_060c74ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c74ec); } r_060c7394(cpu, 0x60c74ec); }
pub fn f_060c74fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c74fd); } r_060c7394(cpu, 0x60c74fd); }
pub fn f_060c7510(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7510); } r_060c7394(cpu, 0x60c7510); }
pub fn f_060c751f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c751f); } r_060c7394(cpu, 0x60c751f); }
pub fn f_060c7539(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7539); } r_060c7394(cpu, 0x60c7539); }
pub fn f_060c7551(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7551); } r_060c7394(cpu, 0x60c7551); }
pub fn f_060c755f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c755f); } r_060c7394(cpu, 0x60c755f); }
pub fn f_060c7573(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7573); } r_060c7394(cpu, 0x60c7573); }
pub fn f_060c758c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c758c); } r_060c7394(cpu, 0x60c758c); }
pub fn f_060c75a1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c75a1); } r_060c7394(cpu, 0x60c75a1); }
pub fn f_060c75b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c75b0); } r_060c7394(cpu, 0x60c75b0); }
pub fn f_060c75c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c75c7); } r_060c7394(cpu, 0x60c75c7); }
pub fn f_060c75d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c75d7); } r_060c7394(cpu, 0x60c75d7); }
pub fn f_060c75e5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c75e5); } r_060c7394(cpu, 0x60c75e5); }
pub fn f_060c75fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c75fd); } r_060c7394(cpu, 0x60c75fd); }
pub fn f_060c761e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c761e); } r_060c7394(cpu, 0x60c761e); }
pub fn f_060c762d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c762d); } r_060c7394(cpu, 0x60c762d); }
pub fn f_060c763f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c763f); } r_060c7394(cpu, 0x60c763f); }
pub fn f_060c765a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c765a); } r_060c7394(cpu, 0x60c765a); }
pub fn f_060c766d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c766d); } r_060c7394(cpu, 0x60c766d); }
pub fn f_060c7683(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7683); } r_060c7394(cpu, 0x60c7683); }
pub fn f_060c7696(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7696); } r_060c7394(cpu, 0x60c7696); }
pub fn f_060c76a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c76a3); } r_060c7394(cpu, 0x60c76a3); }
pub fn f_060c76b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c76b1); } r_060c7394(cpu, 0x60c76b1); }
pub fn f_060c76cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c76cb); } r_060c7394(cpu, 0x60c76cb); }
pub fn f_060c76de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c76de); } r_060c7394(cpu, 0x60c76de); }
pub fn f_060c76f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c76f3); } r_060c7394(cpu, 0x60c76f3); }
pub fn f_060c7702(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7702); } r_060c7394(cpu, 0x60c7702); }
pub fn f_060c7715(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7715); } r_060c7394(cpu, 0x60c7715); }
pub fn f_060c772d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c772d); } r_060c7394(cpu, 0x60c772d); }
pub fn f_060c7740(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7740); } r_060c7394(cpu, 0x60c7740); }
pub fn f_060c774b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c774b); } r_060c7394(cpu, 0x60c774b); }
pub fn f_060c7761(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7761); } r_060c7394(cpu, 0x60c7761); }
pub fn f_060c7777(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7777); } r_060c7394(cpu, 0x60c7777); }
pub fn f_060c7785(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7785); } r_060c7394(cpu, 0x60c7785); }
pub fn f_060c7799(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7799); } r_060c7394(cpu, 0x60c7799); }
pub fn f_060c77ae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c77ae); } r_060c7394(cpu, 0x60c77ae); }
pub fn f_060c77bd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c77bd); } r_060c7394(cpu, 0x60c77bd); }
pub fn f_060c77d6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c77d6); } r_060c7394(cpu, 0x60c77d6); }
pub fn f_060c77f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c77f0); } r_060c7394(cpu, 0x60c77f0); }
pub fn f_060c7802(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7802); } r_060c7394(cpu, 0x60c7802); }
pub fn f_060c7815(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7815); } r_060c7394(cpu, 0x60c7815); }
pub fn f_060c7822(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7822); } r_060c7394(cpu, 0x60c7822); }
pub fn f_060c783a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c783a); } r_060c7394(cpu, 0x60c783a); }
pub fn f_060c784a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c784a); } r_060c7394(cpu, 0x60c784a); }
pub fn f_060c7857(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7857); } r_060c7394(cpu, 0x60c7857); }
pub fn f_060c7866(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7866); } r_060c7394(cpu, 0x60c7866); }
pub fn f_060c7875(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7875); } r_060c7394(cpu, 0x60c7875); }
pub fn f_060c7887(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7887); } r_060c7394(cpu, 0x60c7887); }
pub fn f_060c789b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c789b); } r_060c7394(cpu, 0x60c789b); }
pub fn f_060c78b1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c78b1); } r_060c7394(cpu, 0x60c78b1); }
pub fn f_060c78c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c78c8); } r_060c7394(cpu, 0x60c78c8); }
pub fn f_060c78dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c78dd); } r_060c7394(cpu, 0x60c78dd); }
pub fn f_060c78f1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c78f1); } r_060c7394(cpu, 0x60c78f1); }
pub fn f_060c7900(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7900); } r_060c7394(cpu, 0x60c7900); }
pub fn f_060c790f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c790f); } r_060c7394(cpu, 0x60c790f); }
pub fn f_060c7921(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7921); } r_060c7394(cpu, 0x60c7921); }
pub fn f_060c7931(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7931); } r_060c7394(cpu, 0x60c7931); }
pub fn f_060c7944(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7944); } r_060c7394(cpu, 0x60c7944); }
pub fn f_060c7955(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7955); } r_060c7394(cpu, 0x60c7955); }
pub fn f_060c7966(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7966); } r_060c7394(cpu, 0x60c7966); }
pub fn f_060c797f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c797f); } r_060c7394(cpu, 0x60c797f); }
pub fn f_060c7992(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7992); } r_060c7394(cpu, 0x60c7992); }
pub fn f_060c79a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c79a3); } r_060c7394(cpu, 0x60c79a3); }
pub fn f_060c79bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c79bb); } r_060c7394(cpu, 0x60c79bb); }
pub fn f_060c79d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c79d2); } r_060c7394(cpu, 0x60c79d2); }
pub fn f_060c79ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c79ea); } r_060c7394(cpu, 0x60c79ea); }
pub fn f_060c7a07(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a07); } r_060c7394(cpu, 0x60c7a07); }
pub fn f_060c7a2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a2b); } r_060c7394(cpu, 0x60c7a2b); }
pub fn f_060c7a42(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a42); } r_060c7394(cpu, 0x60c7a42); }
pub fn f_060c7a60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a60); } r_060c7394(cpu, 0x60c7a60); }
pub fn f_060c7a74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a74); } r_060c7394(cpu, 0x60c7a74); }
pub fn f_060c7a89(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a89); } r_060c7394(cpu, 0x60c7a89); }
pub fn f_060c7a9c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7a9c); } r_060c7394(cpu, 0x60c7a9c); }
pub fn f_060c7aaf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7aaf); } r_060c7394(cpu, 0x60c7aaf); }
pub fn f_060c7ac3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7ac3); } r_060c7394(cpu, 0x60c7ac3); }
pub fn f_060c7ad5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7ad5); } r_060c7394(cpu, 0x60c7ad5); }
pub fn f_060c7aef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7aef); } r_060c7394(cpu, 0x60c7aef); }
pub fn f_060c7b00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b00); } r_060c7394(cpu, 0x60c7b00); }
pub fn f_060c7b10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b10); } r_060c7394(cpu, 0x60c7b10); }
pub fn f_060c7b2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b2b); } r_060c7394(cpu, 0x60c7b2b); }
pub fn f_060c7b39(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b39); } r_060c7394(cpu, 0x60c7b39); }
pub fn f_060c7b49(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b49); } r_060c7394(cpu, 0x60c7b49); }
pub fn f_060c7b56(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b56); } r_060c7394(cpu, 0x60c7b56); }
pub fn f_060c7b73(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b73); } r_060c7394(cpu, 0x60c7b73); }
pub fn f_060c7b84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b84); } r_060c7394(cpu, 0x60c7b84); }
pub fn f_060c7b9e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7b9e); } r_060c7394(cpu, 0x60c7b9e); }
pub fn f_060c7bc1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7bc1); } r_060c7394(cpu, 0x60c7bc1); }
pub fn f_060c7bdc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7bdc); } r_060c7394(cpu, 0x60c7bdc); }
pub fn f_060c7bee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7bee); } r_060c7394(cpu, 0x60c7bee); }
pub fn f_060c7c03(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c03); } r_060c7394(cpu, 0x60c7c03); }
pub fn f_060c7c18(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c18); } r_060c7394(cpu, 0x60c7c18); }
pub fn f_060c7c2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c2b); } r_060c7394(cpu, 0x60c7c2b); }
pub fn f_060c7c3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c3f); } r_060c7394(cpu, 0x60c7c3f); }
pub fn f_060c7c54(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c54); } r_060c7394(cpu, 0x60c7c54); }
pub fn f_060c7c68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c68); } r_060c7394(cpu, 0x60c7c68); }
pub fn f_060c7c80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c80); } r_060c7394(cpu, 0x60c7c80); }
pub fn f_060c7c96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7c96); } r_060c7394(cpu, 0x60c7c96); }
pub fn f_060c7ca8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7ca8); } r_060c7394(cpu, 0x60c7ca8); }
pub fn f_060c7cb8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7cb8); } r_060c7394(cpu, 0x60c7cb8); }
pub fn f_060c7cc7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7cc7); } r_060c7394(cpu, 0x60c7cc7); }
pub fn f_060c7cdc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7cdc); } r_060c7394(cpu, 0x60c7cdc); }
pub fn f_060c7ce8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7ce8); } r_060c7394(cpu, 0x60c7ce8); }
pub fn f_060c7d02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d02); } r_060c7394(cpu, 0x60c7d02); }
pub fn f_060c7d13(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d13); } r_060c7394(cpu, 0x60c7d13); }
pub fn f_060c7d2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d2b); } r_060c7394(cpu, 0x60c7d2b); }
pub fn f_060c7d45(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d45); } r_060c7394(cpu, 0x60c7d45); }
pub fn f_060c7d5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d5d); } r_060c7394(cpu, 0x60c7d5d); }
pub fn f_060c7d77(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d77); } r_060c7394(cpu, 0x60c7d77); }
pub fn f_060c7d8f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7d8f); } r_060c7394(cpu, 0x60c7d8f); }
pub fn f_060c7dac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7dac); } r_060c7394(cpu, 0x60c7dac); }
pub fn f_060c7dc5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7dc5); } r_060c7394(cpu, 0x60c7dc5); }
pub fn f_060c7ddb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7ddb); } r_060c7394(cpu, 0x60c7ddb); }
pub fn f_060c7df1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7df1); } r_060c7394(cpu, 0x60c7df1); }
pub fn f_060c7e02(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7e02); } r_060c7394(cpu, 0x60c7e02); }
pub fn f_060c7e11(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c7e11); } r_060c7394(cpu, 0x60c7e11); }
fn r_060c7394(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60c7394 => {
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
  pc = 0x60c73a4; continue;
 }
 0x60c73a4 => {
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
  pc = 0x60c73b5; continue;
 }
 0x60c73b5 => {
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
  pc = 0x60c73e6; continue;
 }
 0x60c73c9 => {
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
  pc = 0x60c73e6; continue;
 }
 0x60c73e6 => {
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
  pc = 0x60c73fb; continue;
 }
 0x60c73fb => {
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
  pc = 0x60c7447; continue;
 }
 0x60c7414 => {
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
  pc = 0x60c7447; continue;
 }
 0x60c7427 => {
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
  pc = 0x60c7447; continue;
 }
 0x60c7447 => {
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
  pc = 0x60c745e; continue;
 }
 0x60c745e => {
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
  pc = 0x60c747b; continue;
 }
 0x60c747b => {
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
  pc = 0x60c74a3; continue;
 }
 0x60c748d => {
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
  pc = 0x60c74a3; continue;
 }
 0x60c74a3 => {
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
  pc = 0x60c74b3; continue;
 }
 0x60c74b3 => {
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
  pc = 0x60c74c5; continue;
 }
 0x60c74c5 => {
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
  pc = 0x60c74d8; continue;
 }
 0x60c74d8 => {
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
  pc = 0x60c74ec; continue;
 }
 0x60c74ec => {
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
  pc = 0x60c751f; continue;
 }
 0x60c74fd => {
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
  pc = 0x60c751f; continue;
 }
 0x60c7510 => {
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
  pc = 0x60c751f; continue;
 }
 0x60c751f => {
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
  pc = 0x60c7551; continue;
 }
 0x60c7539 => {
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
  pc = 0x60c7551; continue;
 }
 0x60c7551 => {
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
  pc = 0x60c755f; continue;
 }
 0x60c755f => {
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
  pc = 0x60c7573; continue;
 }
 0x60c7573 => {
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
  pc = 0x60c758c; continue;
 }
 0x60c758c => {
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
  pc = 0x60c75a1; continue;
 }
 0x60c75a1 => {
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
  pc = 0x60c75b0; continue;
 }
 0x60c75b0 => {
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
  pc = 0x60c75c7; continue;
 }
 0x60c75c7 => {
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
  pc = 0x60c75d7; continue;
 }
 0x60c75d7 => {
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
  pc = 0x60c75e5; continue;
 }
 0x60c75e5 => {
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
  pc = 0x60c761e; continue;
 }
 0x60c75fd => {
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
  pc = 0x60c761e; continue;
 }
 0x60c761e => {
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
  pc = 0x60c762d; continue;
 }
 0x60c762d => {
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
  pc = 0x60c765a; continue;
 }
 0x60c763f => {
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
  pc = 0x60c765a; continue;
 }
 0x60c765a => {
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
  pc = 0x60c7683; continue;
 }
 0x60c766d => {
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
  pc = 0x60c7683; continue;
 }
 0x60c7683 => {
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
  pc = 0x60c7696; continue;
 }
 0x60c7696 => {
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
  pc = 0x60c76a3; continue;
 }
 0x60c76a3 => {
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
  pc = 0x60c76b1; continue;
 }
 0x60c76b1 => {
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
  pc = 0x60c76cb; continue;
 }
 0x60c76cb => {
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
  pc = 0x60c76de; continue;
 }
 0x60c76de => {
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
  pc = 0x60c76f3; continue;
 }
 0x60c76f3 => {
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
  pc = 0x60c7702; continue;
 }
 0x60c7702 => {
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
  pc = 0x60c7715; continue;
 }
 0x60c7715 => {
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
  pc = 0x60c772d; continue;
 }
 0x60c772d => {
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
  pc = 0x60c7740; continue;
 }
 0x60c7740 => {
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
  pc = 0x60c774b; continue;
 }
 0x60c774b => {
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
  pc = 0x60c7761; continue;
 }
 0x60c7761 => {
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
  pc = 0x60c7777; continue;
 }
 0x60c7777 => {
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
  pc = 0x60c7785; continue;
 }
 0x60c7785 => {
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
  pc = 0x60c77bd; continue;
 }
 0x60c7799 => {
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
  pc = 0x60c77bd; continue;
 }
 0x60c77ae => {
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
  pc = 0x60c77bd; continue;
 }
 0x60c77bd => {
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
  pc = 0x60c77d6; continue;
 }
 0x60c77d6 => {
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
  pc = 0x60c77f0; continue;
 }
 0x60c77f0 => {
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
  pc = 0x60c7802; continue;
 }
 0x60c7802 => {
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
  pc = 0x60c7815; continue;
 }
 0x60c7815 => {
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
  pc = 0x60c784a; continue;
 }
 0x60c7822 => {
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
  pc = 0x60c784a; continue;
 }
 0x60c783a => {
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
  pc = 0x60c784a; continue;
 }
 0x60c784a => {
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
  pc = 0x60c7857; continue;
 }
 0x60c7857 => {
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
  pc = 0x60c7866; continue;
 }
 0x60c7866 => {
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
  pc = 0x60c7875; continue;
 }
 0x60c7875 => {
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
  pc = 0x60c7887; continue;
 }
 0x60c7887 => {
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
  pc = 0x60c789b; continue;
 }
 0x60c789b => {
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
  pc = 0x60c78b1; continue;
 }
 0x60c78b1 => {
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
  pc = 0x60c78c8; continue;
 }
 0x60c78c8 => {
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
  pc = 0x60c78dd; continue;
 }
 0x60c78dd => {
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
  pc = 0x60c78f1; continue;
 }
 0x60c78f1 => {
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
  pc = 0x60c7900; continue;
 }
 0x60c7900 => {
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
  pc = 0x60c790f; continue;
 }
 0x60c790f => {
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
  pc = 0x60c7921; continue;
 }
 0x60c7921 => {
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
  pc = 0x60c7931; continue;
 }
 0x60c7931 => {
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
  pc = 0x60c7944; continue;
 }
 0x60c7944 => {
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
  pc = 0x60c7955; continue;
 }
 0x60c7955 => {
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
  pc = 0x60c79bb; continue;
 }
 0x60c7966 => {
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
  pc = 0x60c79bb; continue;
 }
 0x60c797f => {
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
  pc = 0x60c79bb; continue;
 }
 0x60c7992 => {
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
  pc = 0x60c79bb; continue;
 }
 0x60c79a3 => {
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
  pc = 0x60c79bb; continue;
 }
 0x60c79bb => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c79d2 => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c79ea => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c7a07 => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c7a2b => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c7a42 => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c7a60 => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c7a74 => {
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
  pc = 0x60c7a89; continue;
 }
 0x60c7a89 => {
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
  pc = 0x60c7a9c; continue;
 }
 0x60c7a9c => {
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
  pc = 0x60c7aaf; continue;
 }
 0x60c7aaf => {
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
  pc = 0x60c7ad5; continue;
 }
 0x60c7ac3 => {
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
  pc = 0x60c7ad5; continue;
 }
 0x60c7ad5 => {
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
  pc = 0x60c7b00; continue;
 }
 0x60c7aef => {
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
  pc = 0x60c7b00; continue;
 }
 0x60c7b00 => {
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
  pc = 0x60c7b2b; continue;
 }
 0x60c7b10 => {
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
  pc = 0x60c7b2b; continue;
 }
 0x60c7b2b => {
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
  pc = 0x60c7b39; continue;
 }
 0x60c7b39 => {
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
  pc = 0x60c7b49; continue;
 }
 0x60c7b49 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.r32(0x4cu32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); }
  pc = 0x60c7b56; continue;
 }
 0x60c7b56 => {
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
  pc = 0x60c7b84; continue;
 }
 0x60c7b73 => {
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
  pc = 0x60c7b84; continue;
 }
 0x60c7b84 => {
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
  pc = 0x60c7b9e; continue;
 }
 0x60c7b9e => {
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
  pc = 0x60c7bc1; continue;
 }
 0x60c7bc1 => {
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
  pc = 0x60c7c03; continue;
 }
 0x60c7bdc => {
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
  pc = 0x60c7c03; continue;
 }
 0x60c7bee => {
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
  pc = 0x60c7c03; continue;
 }
 0x60c7c03 => {
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
  pc = 0x60c7c18; continue;
 }
 0x60c7c18 => {
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
  pc = 0x60c7c2b; continue;
 }
 0x60c7c2b => {
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
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60c7c3f; continue;
 }
 0x60c7c3f => {
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
  pc = 0x60c7c68; continue;
 }
 0x60c7c54 => {
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
  pc = 0x60c7c68; continue;
 }
 0x60c7c68 => {
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
  pc = 0x60c7c96; continue;
 }
 0x60c7c80 => {
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
  pc = 0x60c7c96; continue;
 }
 0x60c7c96 => {
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
  pc = 0x60c7ca8; continue;
 }
 0x60c7ca8 => {
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
  pc = 0x60c7cb8; continue;
 }
 0x60c7cb8 => {
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
  pc = 0x60c7cc7; continue;
 }
 0x60c7cc7 => {
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
  pc = 0x60c7ce8; continue;
 }
 0x60c7cdc => {
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
  pc = 0x60c7ce8; continue;
 }
 0x60c7ce8 => {
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
  pc = 0x60c7d02; continue;
 }
 0x60c7d02 => {
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
  pc = 0x60c7d13; continue;
 }
 0x60c7d13 => {
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
  pc = 0x60c7d2b; continue;
 }
 0x60c7d2b => {
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
  pc = 0x60c7d5d; continue;
 }
 0x60c7d45 => {
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
  pc = 0x60c7d5d; continue;
 }
 0x60c7d5d => {
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
  pc = 0x60c7d77; continue;
 }
 0x60c7d77 => {
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
  pc = 0x60c7dc5; continue;
 }
 0x60c7d8f => {
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
  pc = 0x60c7dc5; continue;
 }
 0x60c7dac => {
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
  pc = 0x60c7dc5; continue;
 }
 0x60c7dc5 => {
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
  pc = 0x60c7ddb; continue;
 }
 0x60c7ddb => {
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
  pc = 0x60c7df1; continue;
 }
 0x60c7df1 => {
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
  pc = 0x60c7e02; continue;
 }
 0x60c7e02 => {
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
  pc = 0x60c7e11; continue;
 }
 0x60c7e11 => {
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
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  cpu.call(0x60c7e2f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
