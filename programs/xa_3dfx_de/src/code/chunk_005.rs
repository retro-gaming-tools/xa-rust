#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_060bd1fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd1fa); }
 let mut pc: u32 = 0x60bd1fa;
 loop { match pc {
 0x60bd1fa => {
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
pub fn f_060bd22d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd22d); }
 let mut pc: u32 = 0x60bd22d;
 loop { match pc {
 0x60bd22d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b668u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x616b66fu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x616b676u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b644u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x616b650u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x616b65cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60bd28a); f_060bd1fa(cpu); if cpu.returned_to != 0x60bd28a { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x60bd298; continue; }
  pc = 0x60bd294; continue;
 }
 0x60bd294 => {
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60bd298; continue;
 }
 0x60bd298 => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60bd2ab); f_060bd1fa(cpu); if cpu.returned_to != 0x60bd2ab { return; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60bd2be); f_060bd1fa(cpu); if cpu.returned_to != 0x60bd2be { return; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x60bd2d0); f_060bd1fa(cpu); if cpu.returned_to != 0x60bd2d0 { return; }
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
  cpu.push(0x60bd2f8); f_060bd1fa(cpu); if cpu.returned_to != 0x60bd2f8 { return; }
  { let v: u32 = 0x16u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x60bd308); f_060bd1fa(cpu); if cpu.returned_to != 0x60bd308 { return; }
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
pub fn f_060bd338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd338); }
 let mut pc: u32 = 0x60bd338;
 loop { match pc {
 0x60bd338 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x62d2194u32; cpu.edx = v; }
  cpu.push(0x60bd343); f_060bd22d(cpu); if cpu.returned_to != 0x60bd343 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd34a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd34a); }
 let mut pc: u32 = 0x60bd34a;
 loop { match pc {
 0x60bd34a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x60bd357); f_060c45ea(cpu); if cpu.returned_to != 0x60bd357 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bd364; continue; }
  pc = 0x60bd35b; continue;
 }
 0x60bd35b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60bd364); f_060adc25(cpu); if cpu.returned_to != 0x60bd364 { return; }
  pc = 0x60bd364; continue;
 }
 0x60bd364 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd368(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd368); }
 let mut pc: u32 = 0x60bd368;
 loop { match pc {
 0x60bd368 => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x60bd36d); cpu.push(0x60bd372); f_060bd3a0(cpu); if cpu.returned_to != 0x60bd372 { return; }
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
  cpu.flat_ds(0x60bd39f); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd3a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd3a0); }
 let mut pc: u32 = 0x60bd3a0;
 loop { match pc {
 0x60bd3a0 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x60bd446u32.wrapping_add(cpu.esi); cpu.eax = v; }
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
pub fn f_060bd412(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd412); }
 let mut pc: u32 = 0x60bd412;
 loop { match pc {
 0x60bd412 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x60bd446u32.wrapping_add(cpu.eax); cpu.eax = v; }
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
pub fn f_060bd446(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd446); }
 let mut pc: u32 = 0x60bd446;
 loop { match pc {
 0x60bd446 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd449(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd449); }
 let mut pc: u32 = 0x60bd449;
 loop { match pc {
 0x60bd449 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd452(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd452); }
 let mut pc: u32 = 0x60bd452;
 loop { match pc {
 0x60bd452 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd455(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd455); }
 let mut pc: u32 = 0x60bd455;
 loop { match pc {
 0x60bd455 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd458(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd458); }
 let mut pc: u32 = 0x60bd458;
 loop { match pc {
 0x60bd458 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd45b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd45b); }
 let mut pc: u32 = 0x60bd45b;
 loop { match pc {
 0x60bd45b => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd45e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd45e); }
 let mut pc: u32 = 0x60bd45e;
 loop { match pc {
 0x60bd45e => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd461(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd461); }
 let mut pc: u32 = 0x60bd461;
 loop { match pc {
 0x60bd461 => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd464); }
 let mut pc: u32 = 0x60bd464;
 loop { match pc {
 0x60bd464 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd467); }
 let mut pc: u32 = 0x60bd467;
 loop { match pc {
 0x60bd467 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd46a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd46a); }
 let mut pc: u32 = 0x60bd46a;
 loop { match pc {
 0x60bd46a => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd46d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd46d); }
 let mut pc: u32 = 0x60bd46d;
 loop { match pc {
 0x60bd46d => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd470); }
 let mut pc: u32 = 0x60bd470;
 loop { match pc {
 0x60bd470 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd473(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd473); }
 let mut pc: u32 = 0x60bd473;
 loop { match pc {
 0x60bd473 => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd476(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd476); }
 let mut pc: u32 = 0x60bd476;
 loop { match pc {
 0x60bd476 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd479(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd479); }
 let mut pc: u32 = 0x60bd479;
 loop { match pc {
 0x60bd479 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd47c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd47c); }
 let mut pc: u32 = 0x60bd47c;
 loop { match pc {
 0x60bd47c => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd47f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd47f); }
 let mut pc: u32 = 0x60bd47f;
 loop { match pc {
 0x60bd47f => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd482(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd482); }
 let mut pc: u32 = 0x60bd482;
 loop { match pc {
 0x60bd482 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd485(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd485); }
 let mut pc: u32 = 0x60bd485;
 loop { match pc {
 0x60bd485 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd488(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd488); }
 let mut pc: u32 = 0x60bd488;
 loop { match pc {
 0x60bd488 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd48b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd48b); }
 let mut pc: u32 = 0x60bd48b;
 loop { match pc {
 0x60bd48b => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd48e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd48e); }
 let mut pc: u32 = 0x60bd48e;
 loop { match pc {
 0x60bd48e => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd491(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd491); }
 let mut pc: u32 = 0x60bd491;
 loop { match pc {
 0x60bd491 => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd494(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd494); }
 let mut pc: u32 = 0x60bd494;
 loop { match pc {
 0x60bd494 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd497(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd497); }
 let mut pc: u32 = 0x60bd497;
 loop { match pc {
 0x60bd497 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd49a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd49a); }
 let mut pc: u32 = 0x60bd49a;
 loop { match pc {
 0x60bd49a => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd49d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd49d); }
 let mut pc: u32 = 0x60bd49d;
 loop { match pc {
 0x60bd49d => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4a0); }
 let mut pc: u32 = 0x60bd4a0;
 loop { match pc {
 0x60bd4a0 => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4a3); }
 let mut pc: u32 = 0x60bd4a3;
 loop { match pc {
 0x60bd4a3 => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4a6); }
 let mut pc: u32 = 0x60bd4a6;
 loop { match pc {
 0x60bd4a6 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4a9); }
 let mut pc: u32 = 0x60bd4a9;
 loop { match pc {
 0x60bd4a9 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4ac); }
 let mut pc: u32 = 0x60bd4ac;
 loop { match pc {
 0x60bd4ac => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4af); }
 let mut pc: u32 = 0x60bd4af;
 loop { match pc {
 0x60bd4af => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4b2); }
 let mut pc: u32 = 0x60bd4b2;
 loop { match pc {
 0x60bd4b2 => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4b5); }
 let mut pc: u32 = 0x60bd4b5;
 loop { match pc {
 0x60bd4b5 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4b8); }
 let mut pc: u32 = 0x60bd4b8;
 loop { match pc {
 0x60bd4b8 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4bb); }
 let mut pc: u32 = 0x60bd4bb;
 loop { match pc {
 0x60bd4bb => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4be); }
 let mut pc: u32 = 0x60bd4be;
 loop { match pc {
 0x60bd4be => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4c1); }
 let mut pc: u32 = 0x60bd4c1;
 loop { match pc {
 0x60bd4c1 => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4c4); }
 let mut pc: u32 = 0x60bd4c4;
 loop { match pc {
 0x60bd4c4 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4c7); }
 let mut pc: u32 = 0x60bd4c7;
 loop { match pc {
 0x60bd4c7 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4ca); }
 let mut pc: u32 = 0x60bd4ca;
 loop { match pc {
 0x60bd4ca => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4cd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4cd); }
 let mut pc: u32 = 0x60bd4cd;
 loop { match pc {
 0x60bd4cd => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4d0); }
 let mut pc: u32 = 0x60bd4d0;
 loop { match pc {
 0x60bd4d0 => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4d3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4d3); }
 let mut pc: u32 = 0x60bd4d3;
 loop { match pc {
 0x60bd4d3 => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4d6); }
 let mut pc: u32 = 0x60bd4d6;
 loop { match pc {
 0x60bd4d6 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4d9); }
 let mut pc: u32 = 0x60bd4d9;
 loop { match pc {
 0x60bd4d9 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4dc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4dc); }
 let mut pc: u32 = 0x60bd4dc;
 loop { match pc {
 0x60bd4dc => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4df); }
 let mut pc: u32 = 0x60bd4df;
 loop { match pc {
 0x60bd4df => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4e2); }
 let mut pc: u32 = 0x60bd4e2;
 loop { match pc {
 0x60bd4e2 => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4e5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4e5); }
 let mut pc: u32 = 0x60bd4e5;
 loop { match pc {
 0x60bd4e5 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4e8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4e8); }
 let mut pc: u32 = 0x60bd4e8;
 loop { match pc {
 0x60bd4e8 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4eb); }
 let mut pc: u32 = 0x60bd4eb;
 loop { match pc {
 0x60bd4eb => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4ee); }
 let mut pc: u32 = 0x60bd4ee;
 loop { match pc {
 0x60bd4ee => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4f1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4f1); }
 let mut pc: u32 = 0x60bd4f1;
 loop { match pc {
 0x60bd4f1 => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4f4); }
 let mut pc: u32 = 0x60bd4f4;
 loop { match pc {
 0x60bd4f4 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4f7); }
 let mut pc: u32 = 0x60bd4f7;
 loop { match pc {
 0x60bd4f7 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4fa); }
 let mut pc: u32 = 0x60bd4fa;
 loop { match pc {
 0x60bd4fa => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd4fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd4fd); }
 let mut pc: u32 = 0x60bd4fd;
 loop { match pc {
 0x60bd4fd => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd500); }
 let mut pc: u32 = 0x60bd500;
 loop { match pc {
 0x60bd500 => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd503(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd503); }
 let mut pc: u32 = 0x60bd503;
 loop { match pc {
 0x60bd503 => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd506(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd506); }
 let mut pc: u32 = 0x60bd506;
 loop { match pc {
 0x60bd506 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd509(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd509); }
 let mut pc: u32 = 0x60bd509;
 loop { match pc {
 0x60bd509 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd50c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd50c); }
 let mut pc: u32 = 0x60bd50c;
 loop { match pc {
 0x60bd50c => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd50f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd50f); }
 let mut pc: u32 = 0x60bd50f;
 loop { match pc {
 0x60bd50f => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd512(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd512); }
 let mut pc: u32 = 0x60bd512;
 loop { match pc {
 0x60bd512 => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd515(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd515); }
 let mut pc: u32 = 0x60bd515;
 loop { match pc {
 0x60bd515 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd518(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd518); }
 let mut pc: u32 = 0x60bd518;
 loop { match pc {
 0x60bd518 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd51b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd51b); }
 let mut pc: u32 = 0x60bd51b;
 loop { match pc {
 0x60bd51b => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd51e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd51e); }
 let mut pc: u32 = 0x60bd51e;
 loop { match pc {
 0x60bd51e => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd521(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd521); }
 let mut pc: u32 = 0x60bd521;
 loop { match pc {
 0x60bd521 => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd524(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd524); }
 let mut pc: u32 = 0x60bd524;
 loop { match pc {
 0x60bd524 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd527(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd527); }
 let mut pc: u32 = 0x60bd527;
 loop { match pc {
 0x60bd527 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd52a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd52a); }
 let mut pc: u32 = 0x60bd52a;
 loop { match pc {
 0x60bd52a => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd52d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd52d); }
 let mut pc: u32 = 0x60bd52d;
 loop { match pc {
 0x60bd52d => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd530); }
 let mut pc: u32 = 0x60bd530;
 loop { match pc {
 0x60bd530 => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd533(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd533); }
 let mut pc: u32 = 0x60bd533;
 loop { match pc {
 0x60bd533 => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd536(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd536); }
 let mut pc: u32 = 0x60bd536;
 loop { match pc {
 0x60bd536 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd539(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd539); }
 let mut pc: u32 = 0x60bd539;
 loop { match pc {
 0x60bd539 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd53c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd53c); }
 let mut pc: u32 = 0x60bd53c;
 loop { match pc {
 0x60bd53c => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd53f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd53f); }
 let mut pc: u32 = 0x60bd53f;
 loop { match pc {
 0x60bd53f => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd542(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd542); }
 let mut pc: u32 = 0x60bd542;
 loop { match pc {
 0x60bd542 => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd545(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd545); }
 let mut pc: u32 = 0x60bd545;
 loop { match pc {
 0x60bd545 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd548); }
 let mut pc: u32 = 0x60bd548;
 loop { match pc {
 0x60bd548 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd54b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd54b); }
 let mut pc: u32 = 0x60bd54b;
 loop { match pc {
 0x60bd54b => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd54e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd54e); }
 let mut pc: u32 = 0x60bd54e;
 loop { match pc {
 0x60bd54e => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd551(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd551); }
 let mut pc: u32 = 0x60bd551;
 loop { match pc {
 0x60bd551 => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd554(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd554); }
 let mut pc: u32 = 0x60bd554;
 loop { match pc {
 0x60bd554 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd557(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd557); }
 let mut pc: u32 = 0x60bd557;
 loop { match pc {
 0x60bd557 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd55a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd55a); }
 let mut pc: u32 = 0x60bd55a;
 loop { match pc {
 0x60bd55a => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd55d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd55d); }
 let mut pc: u32 = 0x60bd55d;
 loop { match pc {
 0x60bd55d => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd560); }
 let mut pc: u32 = 0x60bd560;
 loop { match pc {
 0x60bd560 => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd563(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd563); }
 let mut pc: u32 = 0x60bd563;
 loop { match pc {
 0x60bd563 => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd566(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd566); }
 let mut pc: u32 = 0x60bd566;
 loop { match pc {
 0x60bd566 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd569(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd569); }
 let mut pc: u32 = 0x60bd569;
 loop { match pc {
 0x60bd569 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd56c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd56c); }
 let mut pc: u32 = 0x60bd56c;
 loop { match pc {
 0x60bd56c => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd56f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd56f); }
 let mut pc: u32 = 0x60bd56f;
 loop { match pc {
 0x60bd56f => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd572(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd572); }
 let mut pc: u32 = 0x60bd572;
 loop { match pc {
 0x60bd572 => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd575(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd575); }
 let mut pc: u32 = 0x60bd575;
 loop { match pc {
 0x60bd575 => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd578(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd578); }
 let mut pc: u32 = 0x60bd578;
 loop { match pc {
 0x60bd578 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd57b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd57b); }
 let mut pc: u32 = 0x60bd57b;
 loop { match pc {
 0x60bd57b => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd57e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd57e); }
 let mut pc: u32 = 0x60bd57e;
 loop { match pc {
 0x60bd57e => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd581(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd581); }
 let mut pc: u32 = 0x60bd581;
 loop { match pc {
 0x60bd581 => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd584(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd584); }
 let mut pc: u32 = 0x60bd584;
 loop { match pc {
 0x60bd584 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd587(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd587); }
 let mut pc: u32 = 0x60bd587;
 loop { match pc {
 0x60bd587 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd58a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd58a); }
 let mut pc: u32 = 0x60bd58a;
 loop { match pc {
 0x60bd58a => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd58d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd58d); }
 let mut pc: u32 = 0x60bd58d;
 loop { match pc {
 0x60bd58d => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd590); }
 let mut pc: u32 = 0x60bd590;
 loop { match pc {
 0x60bd590 => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd593(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd593); }
 let mut pc: u32 = 0x60bd593;
 loop { match pc {
 0x60bd593 => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd596(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd596); }
 let mut pc: u32 = 0x60bd596;
 loop { match pc {
 0x60bd596 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd599(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd599); }
 let mut pc: u32 = 0x60bd599;
 loop { match pc {
 0x60bd599 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd59c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd59c); }
 let mut pc: u32 = 0x60bd59c;
 loop { match pc {
 0x60bd59c => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd59f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd59f); }
 let mut pc: u32 = 0x60bd59f;
 loop { match pc {
 0x60bd59f => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5a2); }
 let mut pc: u32 = 0x60bd5a2;
 loop { match pc {
 0x60bd5a2 => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5a5); }
 let mut pc: u32 = 0x60bd5a5;
 loop { match pc {
 0x60bd5a5 => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5a8); }
 let mut pc: u32 = 0x60bd5a8;
 loop { match pc {
 0x60bd5a8 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5ab); }
 let mut pc: u32 = 0x60bd5ab;
 loop { match pc {
 0x60bd5ab => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5ae); }
 let mut pc: u32 = 0x60bd5ae;
 loop { match pc {
 0x60bd5ae => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5b1); }
 let mut pc: u32 = 0x60bd5b1;
 loop { match pc {
 0x60bd5b1 => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5b4); }
 let mut pc: u32 = 0x60bd5b4;
 loop { match pc {
 0x60bd5b4 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5b7); }
 let mut pc: u32 = 0x60bd5b7;
 loop { match pc {
 0x60bd5b7 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5ba); }
 let mut pc: u32 = 0x60bd5ba;
 loop { match pc {
 0x60bd5ba => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5bd); }
 let mut pc: u32 = 0x60bd5bd;
 loop { match pc {
 0x60bd5bd => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5c0); }
 let mut pc: u32 = 0x60bd5c0;
 loop { match pc {
 0x60bd5c0 => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5c3); }
 let mut pc: u32 = 0x60bd5c3;
 loop { match pc {
 0x60bd5c3 => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5c6); }
 let mut pc: u32 = 0x60bd5c6;
 loop { match pc {
 0x60bd5c6 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5c9); }
 let mut pc: u32 = 0x60bd5c9;
 loop { match pc {
 0x60bd5c9 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5cc); }
 let mut pc: u32 = 0x60bd5cc;
 loop { match pc {
 0x60bd5cc => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5cf); }
 let mut pc: u32 = 0x60bd5cf;
 loop { match pc {
 0x60bd5cf => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5d2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5d2); }
 let mut pc: u32 = 0x60bd5d2;
 loop { match pc {
 0x60bd5d2 => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5d5); }
 let mut pc: u32 = 0x60bd5d5;
 loop { match pc {
 0x60bd5d5 => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5d8); }
 let mut pc: u32 = 0x60bd5d8;
 loop { match pc {
 0x60bd5d8 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5db); }
 let mut pc: u32 = 0x60bd5db;
 loop { match pc {
 0x60bd5db => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5de); }
 let mut pc: u32 = 0x60bd5de;
 loop { match pc {
 0x60bd5de => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5e1); }
 let mut pc: u32 = 0x60bd5e1;
 loop { match pc {
 0x60bd5e1 => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5e4); }
 let mut pc: u32 = 0x60bd5e4;
 loop { match pc {
 0x60bd5e4 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5e7); }
 let mut pc: u32 = 0x60bd5e7;
 loop { match pc {
 0x60bd5e7 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5ea); }
 let mut pc: u32 = 0x60bd5ea;
 loop { match pc {
 0x60bd5ea => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5ed); }
 let mut pc: u32 = 0x60bd5ed;
 loop { match pc {
 0x60bd5ed => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5f0); }
 let mut pc: u32 = 0x60bd5f0;
 loop { match pc {
 0x60bd5f0 => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5f3); }
 let mut pc: u32 = 0x60bd5f3;
 loop { match pc {
 0x60bd5f3 => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5f6); }
 let mut pc: u32 = 0x60bd5f6;
 loop { match pc {
 0x60bd5f6 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5f9); }
 let mut pc: u32 = 0x60bd5f9;
 loop { match pc {
 0x60bd5f9 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5fc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5fc); }
 let mut pc: u32 = 0x60bd5fc;
 loop { match pc {
 0x60bd5fc => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd5ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd5ff); }
 let mut pc: u32 = 0x60bd5ff;
 loop { match pc {
 0x60bd5ff => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd602(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd602); }
 let mut pc: u32 = 0x60bd602;
 loop { match pc {
 0x60bd602 => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd605(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd605); }
 let mut pc: u32 = 0x60bd605;
 loop { match pc {
 0x60bd605 => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd608(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd608); }
 let mut pc: u32 = 0x60bd608;
 loop { match pc {
 0x60bd608 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd60b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd60b); }
 let mut pc: u32 = 0x60bd60b;
 loop { match pc {
 0x60bd60b => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd60e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd60e); }
 let mut pc: u32 = 0x60bd60e;
 loop { match pc {
 0x60bd60e => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd611(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd611); }
 let mut pc: u32 = 0x60bd611;
 loop { match pc {
 0x60bd611 => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd614(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd614); }
 let mut pc: u32 = 0x60bd614;
 loop { match pc {
 0x60bd614 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd617(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd617); }
 let mut pc: u32 = 0x60bd617;
 loop { match pc {
 0x60bd617 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd61a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd61a); }
 let mut pc: u32 = 0x60bd61a;
 loop { match pc {
 0x60bd61a => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd61d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd61d); }
 let mut pc: u32 = 0x60bd61d;
 loop { match pc {
 0x60bd61d => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd620); }
 let mut pc: u32 = 0x60bd620;
 loop { match pc {
 0x60bd620 => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd623(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd623); }
 let mut pc: u32 = 0x60bd623;
 loop { match pc {
 0x60bd623 => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd626(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd626); }
 let mut pc: u32 = 0x60bd626;
 loop { match pc {
 0x60bd626 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd629(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd629); }
 let mut pc: u32 = 0x60bd629;
 loop { match pc {
 0x60bd629 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd62c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd62c); }
 let mut pc: u32 = 0x60bd62c;
 loop { match pc {
 0x60bd62c => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd62f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd62f); }
 let mut pc: u32 = 0x60bd62f;
 loop { match pc {
 0x60bd62f => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd632(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd632); }
 let mut pc: u32 = 0x60bd632;
 loop { match pc {
 0x60bd632 => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd635(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd635); }
 let mut pc: u32 = 0x60bd635;
 loop { match pc {
 0x60bd635 => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd638(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd638); }
 let mut pc: u32 = 0x60bd638;
 loop { match pc {
 0x60bd638 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd63b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd63b); }
 let mut pc: u32 = 0x60bd63b;
 loop { match pc {
 0x60bd63b => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd63e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd63e); }
 let mut pc: u32 = 0x60bd63e;
 loop { match pc {
 0x60bd63e => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd641(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd641); }
 let mut pc: u32 = 0x60bd641;
 loop { match pc {
 0x60bd641 => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd644(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd644); }
 let mut pc: u32 = 0x60bd644;
 loop { match pc {
 0x60bd644 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd647(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd647); }
 let mut pc: u32 = 0x60bd647;
 loop { match pc {
 0x60bd647 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd64a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd64a); }
 let mut pc: u32 = 0x60bd64a;
 loop { match pc {
 0x60bd64a => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd64d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd64d); }
 let mut pc: u32 = 0x60bd64d;
 loop { match pc {
 0x60bd64d => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd650); }
 let mut pc: u32 = 0x60bd650;
 loop { match pc {
 0x60bd650 => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd653(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd653); }
 let mut pc: u32 = 0x60bd653;
 loop { match pc {
 0x60bd653 => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd656); }
 let mut pc: u32 = 0x60bd656;
 loop { match pc {
 0x60bd656 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd659(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd659); }
 let mut pc: u32 = 0x60bd659;
 loop { match pc {
 0x60bd659 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd65c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd65c); }
 let mut pc: u32 = 0x60bd65c;
 loop { match pc {
 0x60bd65c => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd65f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd65f); }
 let mut pc: u32 = 0x60bd65f;
 loop { match pc {
 0x60bd65f => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd662(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd662); }
 let mut pc: u32 = 0x60bd662;
 loop { match pc {
 0x60bd662 => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd665(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd665); }
 let mut pc: u32 = 0x60bd665;
 loop { match pc {
 0x60bd665 => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd668(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd668); }
 let mut pc: u32 = 0x60bd668;
 loop { match pc {
 0x60bd668 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd66b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd66b); }
 let mut pc: u32 = 0x60bd66b;
 loop { match pc {
 0x60bd66b => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd66e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd66e); }
 let mut pc: u32 = 0x60bd66e;
 loop { match pc {
 0x60bd66e => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd671(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd671); }
 let mut pc: u32 = 0x60bd671;
 loop { match pc {
 0x60bd671 => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd674(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd674); }
 let mut pc: u32 = 0x60bd674;
 loop { match pc {
 0x60bd674 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd677(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd677); }
 let mut pc: u32 = 0x60bd677;
 loop { match pc {
 0x60bd677 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd67a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd67a); }
 let mut pc: u32 = 0x60bd67a;
 loop { match pc {
 0x60bd67a => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd67d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd67d); }
 let mut pc: u32 = 0x60bd67d;
 loop { match pc {
 0x60bd67d => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd680); }
 let mut pc: u32 = 0x60bd680;
 loop { match pc {
 0x60bd680 => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd683(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd683); }
 let mut pc: u32 = 0x60bd683;
 loop { match pc {
 0x60bd683 => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd686(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd686); }
 let mut pc: u32 = 0x60bd686;
 loop { match pc {
 0x60bd686 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd689(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd689); }
 let mut pc: u32 = 0x60bd689;
 loop { match pc {
 0x60bd689 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd68c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd68c); }
 let mut pc: u32 = 0x60bd68c;
 loop { match pc {
 0x60bd68c => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd68f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd68f); }
 let mut pc: u32 = 0x60bd68f;
 loop { match pc {
 0x60bd68f => {
  { cpu.interrupt(0xc3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd692(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd692); }
 let mut pc: u32 = 0x60bd692;
 loop { match pc {
 0x60bd692 => {
  { cpu.interrupt(0xc4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd695(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd695); }
 let mut pc: u32 = 0x60bd695;
 loop { match pc {
 0x60bd695 => {
  { cpu.interrupt(0xc5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd698(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd698); }
 let mut pc: u32 = 0x60bd698;
 loop { match pc {
 0x60bd698 => {
  { cpu.interrupt(0xc6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd69b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd69b); }
 let mut pc: u32 = 0x60bd69b;
 loop { match pc {
 0x60bd69b => {
  { cpu.interrupt(0xc7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd69e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd69e); }
 let mut pc: u32 = 0x60bd69e;
 loop { match pc {
 0x60bd69e => {
  { cpu.interrupt(0xc8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6a1); }
 let mut pc: u32 = 0x60bd6a1;
 loop { match pc {
 0x60bd6a1 => {
  { cpu.interrupt(0xc9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6a4); }
 let mut pc: u32 = 0x60bd6a4;
 loop { match pc {
 0x60bd6a4 => {
  { cpu.interrupt(0xca); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6a7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6a7); }
 let mut pc: u32 = 0x60bd6a7;
 loop { match pc {
 0x60bd6a7 => {
  { cpu.interrupt(0xcb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6aa); }
 let mut pc: u32 = 0x60bd6aa;
 loop { match pc {
 0x60bd6aa => {
  { cpu.interrupt(0xcc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6ad); }
 let mut pc: u32 = 0x60bd6ad;
 loop { match pc {
 0x60bd6ad => {
  { cpu.interrupt(0xcd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6b0); }
 let mut pc: u32 = 0x60bd6b0;
 loop { match pc {
 0x60bd6b0 => {
  { cpu.interrupt(0xce); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6b3); }
 let mut pc: u32 = 0x60bd6b3;
 loop { match pc {
 0x60bd6b3 => {
  { cpu.interrupt(0xcf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6b6); }
 let mut pc: u32 = 0x60bd6b6;
 loop { match pc {
 0x60bd6b6 => {
  { cpu.interrupt(0xd0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6b9); }
 let mut pc: u32 = 0x60bd6b9;
 loop { match pc {
 0x60bd6b9 => {
  { cpu.interrupt(0xd1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6bc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6bc); }
 let mut pc: u32 = 0x60bd6bc;
 loop { match pc {
 0x60bd6bc => {
  { cpu.interrupt(0xd2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6bf); }
 let mut pc: u32 = 0x60bd6bf;
 loop { match pc {
 0x60bd6bf => {
  { cpu.interrupt(0xd3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6c2); }
 let mut pc: u32 = 0x60bd6c2;
 loop { match pc {
 0x60bd6c2 => {
  { cpu.interrupt(0xd4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6c5); }
 let mut pc: u32 = 0x60bd6c5;
 loop { match pc {
 0x60bd6c5 => {
  { cpu.interrupt(0xd5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6c8); }
 let mut pc: u32 = 0x60bd6c8;
 loop { match pc {
 0x60bd6c8 => {
  { cpu.interrupt(0xd6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6cb); }
 let mut pc: u32 = 0x60bd6cb;
 loop { match pc {
 0x60bd6cb => {
  { cpu.interrupt(0xd7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6ce); }
 let mut pc: u32 = 0x60bd6ce;
 loop { match pc {
 0x60bd6ce => {
  { cpu.interrupt(0xd8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6d1); }
 let mut pc: u32 = 0x60bd6d1;
 loop { match pc {
 0x60bd6d1 => {
  { cpu.interrupt(0xd9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6d4); }
 let mut pc: u32 = 0x60bd6d4;
 loop { match pc {
 0x60bd6d4 => {
  { cpu.interrupt(0xda); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6d7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6d7); }
 let mut pc: u32 = 0x60bd6d7;
 loop { match pc {
 0x60bd6d7 => {
  { cpu.interrupt(0xdb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6da); }
 let mut pc: u32 = 0x60bd6da;
 loop { match pc {
 0x60bd6da => {
  { cpu.interrupt(0xdc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6dd); }
 let mut pc: u32 = 0x60bd6dd;
 loop { match pc {
 0x60bd6dd => {
  { cpu.interrupt(0xdd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6e0); }
 let mut pc: u32 = 0x60bd6e0;
 loop { match pc {
 0x60bd6e0 => {
  { cpu.interrupt(0xde); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6e3); }
 let mut pc: u32 = 0x60bd6e3;
 loop { match pc {
 0x60bd6e3 => {
  { cpu.interrupt(0xdf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6e6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6e6); }
 let mut pc: u32 = 0x60bd6e6;
 loop { match pc {
 0x60bd6e6 => {
  { cpu.interrupt(0xe0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6e9); }
 let mut pc: u32 = 0x60bd6e9;
 loop { match pc {
 0x60bd6e9 => {
  { cpu.interrupt(0xe1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6ec); }
 let mut pc: u32 = 0x60bd6ec;
 loop { match pc {
 0x60bd6ec => {
  { cpu.interrupt(0xe2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6ef); }
 let mut pc: u32 = 0x60bd6ef;
 loop { match pc {
 0x60bd6ef => {
  { cpu.interrupt(0xe3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6f2); }
 let mut pc: u32 = 0x60bd6f2;
 loop { match pc {
 0x60bd6f2 => {
  { cpu.interrupt(0xe4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6f5); }
 let mut pc: u32 = 0x60bd6f5;
 loop { match pc {
 0x60bd6f5 => {
  { cpu.interrupt(0xe5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6f8); }
 let mut pc: u32 = 0x60bd6f8;
 loop { match pc {
 0x60bd6f8 => {
  { cpu.interrupt(0xe6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6fb); }
 let mut pc: u32 = 0x60bd6fb;
 loop { match pc {
 0x60bd6fb => {
  { cpu.interrupt(0xe7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd6fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd6fe); }
 let mut pc: u32 = 0x60bd6fe;
 loop { match pc {
 0x60bd6fe => {
  { cpu.interrupt(0xe8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd701(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd701); }
 let mut pc: u32 = 0x60bd701;
 loop { match pc {
 0x60bd701 => {
  { cpu.interrupt(0xe9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd704(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd704); }
 let mut pc: u32 = 0x60bd704;
 loop { match pc {
 0x60bd704 => {
  { cpu.interrupt(0xea); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd707(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd707); }
 let mut pc: u32 = 0x60bd707;
 loop { match pc {
 0x60bd707 => {
  { cpu.interrupt(0xeb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd70a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd70a); }
 let mut pc: u32 = 0x60bd70a;
 loop { match pc {
 0x60bd70a => {
  { cpu.interrupt(0xec); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd70d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd70d); }
 let mut pc: u32 = 0x60bd70d;
 loop { match pc {
 0x60bd70d => {
  { cpu.interrupt(0xed); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd710); }
 let mut pc: u32 = 0x60bd710;
 loop { match pc {
 0x60bd710 => {
  { cpu.interrupt(0xee); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd713(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd713); }
 let mut pc: u32 = 0x60bd713;
 loop { match pc {
 0x60bd713 => {
  { cpu.interrupt(0xef); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd716(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd716); }
 let mut pc: u32 = 0x60bd716;
 loop { match pc {
 0x60bd716 => {
  { cpu.interrupt(0xf0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd719(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd719); }
 let mut pc: u32 = 0x60bd719;
 loop { match pc {
 0x60bd719 => {
  { cpu.interrupt(0xf1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd71c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd71c); }
 let mut pc: u32 = 0x60bd71c;
 loop { match pc {
 0x60bd71c => {
  { cpu.interrupt(0xf2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd71f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd71f); }
 let mut pc: u32 = 0x60bd71f;
 loop { match pc {
 0x60bd71f => {
  { cpu.interrupt(0xf3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd722(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd722); }
 let mut pc: u32 = 0x60bd722;
 loop { match pc {
 0x60bd722 => {
  { cpu.interrupt(0xf4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd725); }
 let mut pc: u32 = 0x60bd725;
 loop { match pc {
 0x60bd725 => {
  { cpu.interrupt(0xf5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd728(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd728); }
 let mut pc: u32 = 0x60bd728;
 loop { match pc {
 0x60bd728 => {
  { cpu.interrupt(0xf6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd72b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd72b); }
 let mut pc: u32 = 0x60bd72b;
 loop { match pc {
 0x60bd72b => {
  { cpu.interrupt(0xf7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd72e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd72e); }
 let mut pc: u32 = 0x60bd72e;
 loop { match pc {
 0x60bd72e => {
  { cpu.interrupt(0xf8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd731(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd731); }
 let mut pc: u32 = 0x60bd731;
 loop { match pc {
 0x60bd731 => {
  { cpu.interrupt(0xf9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd734(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd734); }
 let mut pc: u32 = 0x60bd734;
 loop { match pc {
 0x60bd734 => {
  { cpu.interrupt(0xfa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd737(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd737); }
 let mut pc: u32 = 0x60bd737;
 loop { match pc {
 0x60bd737 => {
  { cpu.interrupt(0xfb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd73a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd73a); }
 let mut pc: u32 = 0x60bd73a;
 loop { match pc {
 0x60bd73a => {
  { cpu.interrupt(0xfc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd73d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd73d); }
 let mut pc: u32 = 0x60bd73d;
 loop { match pc {
 0x60bd73d => {
  { cpu.interrupt(0xfd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd740); }
 let mut pc: u32 = 0x60bd740;
 loop { match pc {
 0x60bd740 => {
  { cpu.interrupt(0xfe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd743(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd743); }
 let mut pc: u32 = 0x60bd743;
 loop { match pc {
 0x60bd743 => {
  { cpu.interrupt(0xff); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd746(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd746); }
 let mut pc: u32 = 0x60bd746;
 loop { match pc {
 0x60bd746 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bd74d); f_060bd855(cpu); if cpu.returned_to != 0x60bd74d { return; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(8)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x60bd758; continue;
 }
 0x60bd758 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x60bd761; continue; }
  pc = 0x60bd75d; continue;
 }
 0x60bd75d => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x60bd761; continue;
 }
 0x60bd761 => {
  if cpu.cf { pc = 0x60bd769; continue; }
  pc = 0x60bd763; continue;
 }
 0x60bd763 => {
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60bd758; continue;
 }
 0x60bd769 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  pc = 0x60bd76c; continue;
 }
 0x60bd76c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.edi))); }
  if !cpu.zf { pc = 0x60bd775; continue; }
  pc = 0x60bd771; continue;
 }
 0x60bd771 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x4u32.wrapping_add(cpu.edi))); }
  pc = 0x60bd775; continue;
 }
 0x60bd775 => {
  if !cpu.cf { pc = 0x60bd77d; continue; }
  pc = 0x60bd777; continue;
 }
 0x60bd777 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60bd76c; continue;
 }
 0x60bd77d => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd780); }
 let mut pc: u32 = 0x60bd780;
 loop { match pc {
 0x60bd780 => {
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
  if cpu.zf { pc = 0x60bd7de; continue; }
  pc = 0x60bd7a9; continue;
 }
 0x60bd7a9 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bd7b4; continue; }
  pc = 0x60bd7ab; continue;
 }
 0x60bd7ab => {
  { let v: u32 = cpu.shl32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcl32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60bd7ab; continue; }
  pc = 0x60bd7b2; continue;
 }
 0x60bd7b2 => {
  pc = 0x60bd7de; continue;
 }
 0x60bd7b4 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  pc = 0x60bd7b8; continue;
 }
 0x60bd7b8 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.rcr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60bd7b8; continue; }
  pc = 0x60bd7c3; continue;
 }
 0x60bd7c3 => {
  { cpu.sub32(cpu.eax, 0x80000000u32); }
  if cpu.cf { pc = 0x60bd7de; continue; }
  pc = 0x60bd7ca; continue;
 }
 0x60bd7ca => {
  if !cpu.zf { pc = 0x60bd7d8; continue; }
  pc = 0x60bd7cc; continue;
 }
 0x60bd7cc => {
  { let v: u32 = cpu.or32(cpu.esi, cpu.esi); cpu.esi = v; }
  if !cpu.zf { pc = 0x60bd7d8; continue; }
  pc = 0x60bd7d0; continue;
 }
 0x60bd7d0 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x60bd7de; continue; }
  pc = 0x60bd7d8; continue;
 }
 0x60bd7d8 => {
  { let v: u32 = cpu.add32(cpu.ecx, 0x1u32); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, 0x0u32); cpu.ebx = v; }
  pc = 0x60bd7de; continue;
 }
 0x60bd7de => {
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
pub fn f_060bd7eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd7eb); }
 let mut pc: u32 = 0x60bd7eb;
 loop { match pc {
 0x60bd7eb => {
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
  pc = 0x60bd800; continue;
 }
 0x60bd800 => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = 0x2710u32; cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60bd813; continue; }
  pc = 0x60bd80c; continue;
 }
 0x60bd80c => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { pc = 0x60bd815; continue; }
  pc = 0x60bd810; continue;
 }
 0x60bd810 => {
  { let v = cpu.eax; cpu.push(v); }
  pc = 0x60bd82b; continue;
 }
 0x60bd813 => {
  { let v = cpu.edi; cpu.div32(v); }
  pc = 0x60bd815; continue;
 }
 0x60bd815 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.edi; cpu.div32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = 0x64u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = (cpu.edx as u8); cpu.div8(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x60bd824, "Aam");
  { let a = cpu.edx; let b = cpu.eax; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  cpu.unimplemented(0x60bd829, "Aam");
  pc = 0x60bd82b; continue;
 }
 0x60bd82b => {
  { let v: u16 = (cpu.eax as u16).wrapping_add(0x3030u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(0x3030u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x4u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x3u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x60bd84f; continue; }
  pc = 0x60bd83d; continue;
 }
 0x60bd83d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x2u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x60bd84f; continue; }
  pc = 0x60bd843; continue;
 }
 0x60bd843 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if cpu.zf { pc = 0x60bd84f; continue; }
  pc = 0x60bd849; continue;
 }
 0x60bd849 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60bd800; continue; }
  pc = 0x60bd84f; continue;
 }
 0x60bd84f => {
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
pub fn f_060bd855(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd855); }
 let mut pc: u32 = 0x60bd855;
 loop { match pc {
 0x60bd855 => {
  cpu.push(0x60bd85a); f_060bd90a(cpu); if cpu.returned_to != 0x60bd85a { return; }
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
  cpu.unimplemented(0x60bd887, "Daa");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x186u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x42u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x42u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x60bd898, "Sldt");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x9896u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x9896u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x60bd89e; continue; }
  pc = 0x60bd8a9; continue;
 }
 0x60bd89e => {
  { let v: u8 = cpu.adc8(cpu.r8(0x98u32.wrapping_add(cpu.esi)), 0x0u8); cpu.w8(0x98u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && cpu.zf } { cpu.poll(); pc = 0x60bd89e; continue; }
  pc = 0x60bd8a9; continue;
 }
 0x60bd8a9 => {
  { let v: u32 = cpu.eax.wrapping_add(0x0u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8).wrapping_add((cpu.ecx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x60bd8b7); cpu.call(0x0); if cpu.returned_to != 0x60bd8b7 { return; }
  { let v: u8 = cpu.port_in(0xbu16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.esp; cpu.push(v); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60bd8c4); cpu.call(0x6f4213a); if cpu.returned_to != 0x60bd8c4 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(4, false); }
  cpu.unimplemented(0x60bd8c9, "Aam");
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
  cpu.unimplemented(0x60bd8ec, "Arpl");
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xffffffb3u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xe0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax | 0xa7640000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8((cpu.eax as u8), 0x23u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x60bd8fc); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd90a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd90a); }
 let mut pc: u32 = 0x60bd90a;
 loop { match pc {
 0x60bd90a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd90c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd90c); }
 let mut pc: u32 = 0x60bd90c;
 loop { match pc {
 0x60bd90c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x616b234u32; cpu.eax = v; }
  cpu.push(0x60bd91c); f_060bd34a(cpu); if cpu.returned_to != 0x60bd91c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd91e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bd91e); } r_060bd91e(cpu, 0x60bd91e); }
pub fn f_060bd9a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bd9a8); } r_060bd91e(cpu, 0x60bd9a8); }
fn r_060bd91e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60bd91e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x60bd924, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x60bd929, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x60bd9aa; continue; }
  pc = 0x60bd939; continue;
 }
 0x60bd939 => {
  if cpu.cf { pc = 0x60bd9ae; continue; }
  pc = 0x60bd93b; continue;
 }
 0x60bd93b => {
  if cpu.zf { pc = 0x60bd9b2; continue; }
  pc = 0x60bd93d; continue;
 }
 0x60bd93d => {
  if (cpu.cf || cpu.zf) { pc = 0x60bd9b6; continue; }
  pc = 0x60bd93f; continue;
 }
 0x60bd93f => {
  if cpu.sf { pc = 0x60bd9ba; continue; }
  pc = 0x60bd941; continue;
 }
 0x60bd941 => {
  if cpu.pf() { f_060bd943(cpu); return; }
  f_060bd943(cpu); return;
 }
 0x60bd9a8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.and32(cpu.edx, 0x7ff00000u32); }
  if cpu.zf { pc = 0x60bd9f0; continue; }
  pc = 0x60bd9b1; continue;
 }
 0x60bd9aa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bd9ae => {
  if (!cpu.zf && cpu.sf == cpu.of) { f_060bda24(cpu); return; }
  pc = 0x60bd9b0; continue;
 }
 0x60bd9b0 => {
  cpu.unimplemented(0x60bd9b0, "Aas");
  pc = 0x60bd9b1; continue;
 }
 0x60bd9b1 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x60bd9e7; continue; }
  pc = 0x60bd9c3; continue;
 }
 0x60bd9b2 => {
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.ecx)) as i32 as f64); cpu.fpush(v); }
  { let v: u8 = cpu.rcl8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 210u32); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.rcr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x20000000u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x60bd9e7; continue; }
  pc = 0x60bd9c3; continue;
 }
 0x60bd9b6 => {
  { let v: u8 = cpu.rcl8((cpu.ecx as u8), (cpu.ecx & 0xff)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = (cpu.r32(0x20000000u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x60bd9e7; continue; }
  pc = 0x60bd9c3; continue;
 }
 0x60bd9ba => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), ((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  if cpu.zf { pc = 0x60bd9e7; continue; }
  pc = 0x60bd9c3; continue;
 }
 0x60bd9c3 => {
  { cpu.sub32(cpu.edx, 0x8fe00000u32); }
  if !cpu.cf { pc = 0x60bd9e7; continue; }
  pc = 0x60bd9cb; continue;
 }
 0x60bd9cb => {
  { cpu.sub32(cpu.edx, 0x70200000u32); }
  if cpu.cf { pc = 0x60bd9f0; continue; }
  pc = 0x60bd9d3; continue;
 }
 0x60bd9d3 => {
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
 0x60bd9e7 => {
  { let v: u32 = 0x7f800000u32; cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bd9f0 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd943(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd943); }
 let mut pc: u32 = 0x60bd943;
 loop { match pc {
 0x60bd943 => {
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
  pc = 0x60bd959; continue;
 }
 0x60bd959 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x60bd91eu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60bd959; continue; }
  pc = 0x60bd979; continue;
 }
 0x60bd979 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60bd979; continue; }
  pc = 0x60bd983; continue;
 }
 0x60bd983 => {
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
pub fn f_060bd98d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd98d); }
 let mut pc: u32 = 0x60bd98d;
 loop { match pc {
 0x60bd98d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x60bd99f; continue; }
  pc = 0x60bd995; continue;
 }
 0x60bd995 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60bd99f; continue; }
  pc = 0x60bd999; continue;
 }
 0x60bd999 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60bd99f; continue;
 }
 0x60bd99f => {
  cpu.push(0x60bd9a4); f_060bd943(cpu); if cpu.returned_to != 0x60bd9a4 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd9f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd9f4); }
 let mut pc: u32 = 0x60bd9f4;
 loop { match pc {
 0x60bd9f4 => {
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
pub fn f_060bda18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda18); }
 let mut pc: u32 = 0x60bda18;
 loop { match pc {
 0x60bda18 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda1c); }
 let mut pc: u32 = 0x60bda1c;
 loop { match pc {
 0x60bda1c => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda20); }
 let mut pc: u32 = 0x60bda20;
 loop { match pc {
 0x60bda20 => {
  { cpu.interrupt(0x2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda24); }
 let mut pc: u32 = 0x60bda24;
 loop { match pc {
 0x60bda24 => {
  { cpu.interrupt(0x3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda28); }
 let mut pc: u32 = 0x60bda28;
 loop { match pc {
 0x60bda28 => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda2c); }
 let mut pc: u32 = 0x60bda2c;
 loop { match pc {
 0x60bda2c => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda30); }
 let mut pc: u32 = 0x60bda30;
 loop { match pc {
 0x60bda30 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda34); }
 let mut pc: u32 = 0x60bda34;
 loop { match pc {
 0x60bda34 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda38); }
 let mut pc: u32 = 0x60bda38;
 loop { match pc {
 0x60bda38 => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda3c); }
 let mut pc: u32 = 0x60bda3c;
 loop { match pc {
 0x60bda3c => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda40); }
 let mut pc: u32 = 0x60bda40;
 loop { match pc {
 0x60bda40 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda44); }
 let mut pc: u32 = 0x60bda44;
 loop { match pc {
 0x60bda44 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda48); }
 let mut pc: u32 = 0x60bda48;
 loop { match pc {
 0x60bda48 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda4c); }
 let mut pc: u32 = 0x60bda4c;
 loop { match pc {
 0x60bda4c => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda50); }
 let mut pc: u32 = 0x60bda50;
 loop { match pc {
 0x60bda50 => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda54); }
 let mut pc: u32 = 0x60bda54;
 loop { match pc {
 0x60bda54 => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda58); }
 let mut pc: u32 = 0x60bda58;
 loop { match pc {
 0x60bda58 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda5c); }
 let mut pc: u32 = 0x60bda5c;
 loop { match pc {
 0x60bda5c => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda60); }
 let mut pc: u32 = 0x60bda60;
 loop { match pc {
 0x60bda60 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda64); }
 let mut pc: u32 = 0x60bda64;
 loop { match pc {
 0x60bda64 => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda68); }
 let mut pc: u32 = 0x60bda68;
 loop { match pc {
 0x60bda68 => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda6c); }
 let mut pc: u32 = 0x60bda6c;
 loop { match pc {
 0x60bda6c => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda70); }
 let mut pc: u32 = 0x60bda70;
 loop { match pc {
 0x60bda70 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda74); }
 let mut pc: u32 = 0x60bda74;
 loop { match pc {
 0x60bda74 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda78); }
 let mut pc: u32 = 0x60bda78;
 loop { match pc {
 0x60bda78 => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda7c); }
 let mut pc: u32 = 0x60bda7c;
 loop { match pc {
 0x60bda7c => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda80); }
 let mut pc: u32 = 0x60bda80;
 loop { match pc {
 0x60bda80 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bda84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bda84); }
 let mut pc: u32 = 0x60bda84;
 loop { match pc {
 0x60bda84 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
