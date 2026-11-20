#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_03095c18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3095c18); }
 let mut pc: u32 = 0x3095c18;
 loop { match pc {
 0x3095c18 => {
  { cpu.sub32(cpu.eax, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x3095c25; continue; }
  pc = 0x3095c1d; continue;
 }
 0x3095c1d => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x3095c25; continue; }
  pc = 0x3095c22; continue;
 }
 0x3095c22 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x3095c25; continue;
 }
 0x3095c25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03095c28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3095c28); }
 let mut pc: u32 = 0x3095c28;
 loop { match pc {
 0x3095c28 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  cpu.push(0x3095c38); f_030a2cec(cpu); if cpu.returned_to != 0x3095c38 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x3095c3f); f_0309f201(cpu); if cpu.returned_to != 0x3095c3f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03095c48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3095c48); }
 let mut pc: u32 = 0x3095c48;
 loop { match pc {
 0x3095c48 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x3095c77; continue; }
  pc = 0x3095c5d; continue;
 }
 0x3095c5d => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x3095c5f; continue;
 }
 0x3095c5f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x3095c6f); f_030a2d14(cpu); if cpu.returned_to != 0x3095c6f { return; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x3095d70; continue;
 }
 0x3095c77 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x3095cc2; continue; }
  pc = 0x3095c87; continue;
 }
 0x3095c87 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { cpu.poll(); pc = 0x3095c5f; continue; }
  pc = 0x3095c8c; continue;
 }
 0x3095c8c => {
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
  if cpu.pf() { pc = 0x3095cb9; continue; }
  pc = 0x3095ca8; continue;
 }
 0x3095ca8 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x3095cb9; continue; }
  pc = 0x3095cb7; continue;
 }
 0x3095cb7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x3095cb9; continue;
 }
 0x3095cb9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xfffffff9u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x3095cca; continue;
 }
 0x3095cc2 => {
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x3095cca; continue; }
  pc = 0x3095cc5; continue;
 }
 0x3095cc5 => {
  pc = 0x3095d51; continue;
 }
 0x3095cca => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3ff0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if !cpu.cf { pc = 0x3095d51; continue; }
  pc = 0x3095ce2; continue;
 }
 0x3095ce2 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x3095cfc; continue; }
  pc = 0x3095cf1; continue;
 }
 0x3095cf1 => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x3095cfa); f_03095d7b(cpu); if cpu.returned_to != 0x3095cfa { return; }
  pc = 0x3095d27; continue;
 }
 0x3095cfc => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x3095d51; continue; }
  pc = 0x3095d00; continue;
 }
 0x3095d00 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x3095d51; continue; }
  pc = 0x3095d06; continue;
 }
 0x3095d06 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x3095d0e); f_03095d7b(cpu); if cpu.returned_to != 0x3095d0e { return; }
  { cpu.fpush(1.0); }
  { cpu.and8(cpu.r8(0x315bf28u32), 0x1u8); }
  if !cpu.zf { pc = 0x3095d1d; continue; }
  pc = 0x3095d19; continue;
 }
 0x3095d19 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x3095d27; continue;
 }
 0x3095d1d => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x3095d27); f_030a2f2b(cpu); if cpu.returned_to != 0x3095d27 { return; }
  pc = 0x3095d27; continue;
 }
 0x3095d27 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x3095d4d; continue; }
  pc = 0x3095d39; continue;
 }
 0x3095d39 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if !cpu.zf { pc = 0x3095d4d; continue; }
  pc = 0x3095d46; continue;
 }
 0x3095d46 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x3095c5f; continue;
 }
 0x3095d4d => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x3095d70; continue;
 }
 0x3095d51 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x3095d62); f_030a2da8(cpu); if cpu.returned_to != 0x3095d62 { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x3095d46; continue; }
  pc = 0x3095d66; continue;
 }
 0x3095d66 => {
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x3095d6e; continue; }
  pc = 0x3095d6c; continue;
 }
 0x3095d6c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x3095d6e; continue;
 }
 0x3095d6e => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x3095d70; continue;
 }
 0x3095d70 => {
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
pub fn f_03095d7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3095d7b); }
 let mut pc: u32 = 0x3095d7b;
 loop { match pc {
 0x3095d7b => {
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x3095d84; continue; }
  pc = 0x3095d80; continue;
 }
 0x3095d80 => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  cpu.poll(); pc = 0x3095d7b; continue;
 }
 0x3095d84 => {
  if !cpu.cf { pc = 0x3095d99; continue; }
  pc = 0x3095d86; continue;
 }
 0x3095d86 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  pc = 0x3095d88; continue;
 }
 0x3095d88 => {
  if cpu.zf { pc = 0x3095d95; continue; }
  pc = 0x3095d8a; continue;
 }
 0x3095d8a => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x3095d93; continue; }
  pc = 0x3095d91; continue;
 }
 0x3095d91 => {
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  pc = 0x3095d93; continue;
 }
 0x3095d93 => {
  cpu.poll(); pc = 0x3095d88; continue;
 }
 0x3095d95 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x3095d9d; continue;
 }
 0x3095d99 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  pc = 0x3095d9d; continue;
 }
 0x3095d9d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03095db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3095db0); }
 let mut pc: u32 = 0x3095db0;
 loop { match pc {
 0x3095db0 => {
  { cpu.sub32(cpu.r32(0x315a9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x3095dd3; continue; }
  pc = 0x3095db9; continue;
 }
 0x3095db9 => {
  { let v = 0x3095db0u32; cpu.push(v); }
  { let v = 0x309dea0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x3095dca); f_030a3800(cpu); if cpu.returned_to != 0x3095dca { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315a9ecu32, v); }
  pc = 0x3095dd3; continue;
 }
 0x3095dd3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03095de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3095de0); }
 let mut pc: u32 = 0x3095de0;
 loop { match pc {
 0x3095de0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3095e27; continue; }
  pc = 0x3095e00; continue;
 }
 0x3095e00 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3095e0e; continue; }
  pc = 0x3095e05; continue;
 }
 0x3095e05 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3095e27; continue; }
  pc = 0x3095e0e; continue;
 }
 0x3095e0e => {
  cpu.push(0x3095e13); f_030a4341(cpu); if cpu.returned_to != 0x3095e13 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3095e27; continue; }
  pc = 0x3095e17; continue;
 }
 0x3095e17 => {
  cpu.push(0x3095e1c); f_0309daa0(cpu); if cpu.returned_to != 0x3095e1c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3095e27; continue; }
  pc = 0x3095e20; continue;
 }
 0x3095e20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3095e29; continue;
 }
 0x3095e27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3095e29; continue;
 }
 0x3095e29 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3095e77; continue; }
  pc = 0x3095e2d; continue;
 }
 0x3095e2d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ae2cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3095e6f); cpu.call(t); if cpu.returned_to != 0x3095e6f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3095e77); f_0309de70(cpu); if cpu.returned_to != 0x3095e77 { return; }
  pc = 0x3095e77; continue;
 }
 0x3095e77 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3095e94); f_030acd50(cpu); if cpu.returned_to != 0x3095e94 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x3095ee2; continue; }
  pc = 0x3095ea3; continue;
 }
 0x3095ea3 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3095eb5; continue; }
  pc = 0x3095eac; continue;
 }
 0x3095eac => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3095ee2; continue; }
  pc = 0x3095eb5; continue;
 }
 0x3095eb5 => {
  cpu.push(0x3095eba); f_030a4341(cpu); if cpu.returned_to != 0x3095eba { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3095ee2; continue; }
  pc = 0x3095ebe; continue;
 }
 0x3095ebe => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3095ed8); f_030a45d3(cpu); if cpu.returned_to != 0x3095ed8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3095ee2); f_0309ddd0(cpu); if cpu.returned_to != 0x3095ee2 { return; }
  pc = 0x3095ee2; continue;
 }
 0x3095ee2 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03096000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096000); }
 let mut pc: u32 = 0x3096000;
 loop { match pc {
 0x3096000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096047; continue; }
  pc = 0x3096020; continue;
 }
 0x3096020 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309602e; continue; }
  pc = 0x3096025; continue;
 }
 0x3096025 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096047; continue; }
  pc = 0x309602e; continue;
 }
 0x309602e => {
  cpu.push(0x3096033); f_030a4341(cpu); if cpu.returned_to != 0x3096033 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096047; continue; }
  pc = 0x3096037; continue;
 }
 0x3096037 => {
  cpu.push(0x309603c); f_0309daa0(cpu); if cpu.returned_to != 0x309603c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096047; continue; }
  pc = 0x3096040; continue;
 }
 0x3096040 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096049; continue;
 }
 0x3096047 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096049; continue;
 }
 0x3096049 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309607b; continue; }
  pc = 0x309604d; continue;
 }
 0x309604d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314adf8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096073); cpu.call(t); if cpu.returned_to != 0x3096073 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309607b); f_0309de70(cpu); if cpu.returned_to != 0x309607b { return; }
  pc = 0x309607b; continue;
 }
 0x309607b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096088); f_030acde0(cpu); if cpu.returned_to != 0x3096088 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030960b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30960b0); }
 let mut pc: u32 = 0x30960b0;
 loop { match pc {
 0x30960b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30960f8; continue; }
  pc = 0x30960d1; continue;
 }
 0x30960d1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30960df; continue; }
  pc = 0x30960d6; continue;
 }
 0x30960d6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30960f8; continue; }
  pc = 0x30960df; continue;
 }
 0x30960df => {
  cpu.push(0x30960e4); f_030a4341(cpu); if cpu.returned_to != 0x30960e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30960f8; continue; }
  pc = 0x30960e8; continue;
 }
 0x30960e8 => {
  cpu.push(0x30960ed); f_0309daa0(cpu); if cpu.returned_to != 0x30960ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30960f8; continue; }
  pc = 0x30960f1; continue;
 }
 0x30960f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30960fa; continue;
 }
 0x30960f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30960fa; continue;
 }
 0x30960fa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309613a; continue; }
  pc = 0x30960fe; continue;
 }
 0x30960fe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314addcu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096132); cpu.call(t); if cpu.returned_to != 0x3096132 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309613a); f_0309de70(cpu); if cpu.returned_to != 0x309613a { return; }
  pc = 0x309613a; continue;
 }
 0x309613a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309614f); f_030ace30(cpu); if cpu.returned_to != 0x309614f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309619c; continue; }
  pc = 0x309615d; continue;
 }
 0x309615d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309616f; continue; }
  pc = 0x3096166; continue;
 }
 0x3096166 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309619c; continue; }
  pc = 0x309616f; continue;
 }
 0x309616f => {
  cpu.push(0x3096174); f_030a4341(cpu); if cpu.returned_to != 0x3096174 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309619c; continue; }
  pc = 0x3096178; continue;
 }
 0x3096178 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3096192); f_030a45d3(cpu); if cpu.returned_to != 0x3096192 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309619c); f_0309de20(cpu); if cpu.returned_to != 0x309619c { return; }
  pc = 0x309619c; continue;
 }
 0x309619c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_030961c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30961c0); }
 let mut pc: u32 = 0x30961c0;
 loop { match pc {
 0x30961c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096207; continue; }
  pc = 0x30961e0; continue;
 }
 0x30961e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30961ee; continue; }
  pc = 0x30961e5; continue;
 }
 0x30961e5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096207; continue; }
  pc = 0x30961ee; continue;
 }
 0x30961ee => {
  cpu.push(0x30961f3); f_030a4341(cpu); if cpu.returned_to != 0x30961f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096207; continue; }
  pc = 0x30961f7; continue;
 }
 0x30961f7 => {
  cpu.push(0x30961fc); f_0309daa0(cpu); if cpu.returned_to != 0x30961fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096207; continue; }
  pc = 0x3096200; continue;
 }
 0x3096200 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096209; continue;
 }
 0x3096207 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096209; continue;
 }
 0x3096209 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309623b; continue; }
  pc = 0x309620d; continue;
 }
 0x309620d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314adc0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096233); cpu.call(t); if cpu.returned_to != 0x3096233 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309623b); f_0309de70(cpu); if cpu.returned_to != 0x309623b { return; }
  pc = 0x309623b; continue;
 }
 0x309623b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096248); f_030acfa0(cpu); if cpu.returned_to != 0x3096248 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03096270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096270); }
 let mut pc: u32 = 0x3096270;
 loop { match pc {
 0x3096270 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30962b8; continue; }
  pc = 0x3096291; continue;
 }
 0x3096291 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309629f; continue; }
  pc = 0x3096296; continue;
 }
 0x3096296 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30962b8; continue; }
  pc = 0x309629f; continue;
 }
 0x309629f => {
  cpu.push(0x30962a4); f_030a4341(cpu); if cpu.returned_to != 0x30962a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30962b8; continue; }
  pc = 0x30962a8; continue;
 }
 0x30962a8 => {
  cpu.push(0x30962ad); f_0309daa0(cpu); if cpu.returned_to != 0x30962ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30962b8; continue; }
  pc = 0x30962b1; continue;
 }
 0x30962b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30962ba; continue;
 }
 0x30962b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30962ba; continue;
 }
 0x30962ba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30962ec; continue; }
  pc = 0x30962be; continue;
 }
 0x30962be => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ada8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30962e4); cpu.call(t); if cpu.returned_to != 0x30962e4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30962ec); f_0309de70(cpu); if cpu.returned_to != 0x30962ec { return; }
  pc = 0x30962ec; continue;
 }
 0x30962ec => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30962f9); f_030ad040(cpu); if cpu.returned_to != 0x30962f9 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3096347; continue; }
  pc = 0x3096308; continue;
 }
 0x3096308 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309631a; continue; }
  pc = 0x3096311; continue;
 }
 0x3096311 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096347; continue; }
  pc = 0x309631a; continue;
 }
 0x309631a => {
  cpu.push(0x309631f); f_030a4341(cpu); if cpu.returned_to != 0x309631f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096347; continue; }
  pc = 0x3096323; continue;
 }
 0x3096323 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309633d); f_030a45d3(cpu); if cpu.returned_to != 0x309633d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3096347); f_0309ddd0(cpu); if cpu.returned_to != 0x3096347 { return; }
  pc = 0x3096347; continue;
 }
 0x3096347 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_030964f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30964f0); }
 let mut pc: u32 = 0x30964f0;
 loop { match pc {
 0x30964f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096538; continue; }
  pc = 0x3096511; continue;
 }
 0x3096511 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309651f; continue; }
  pc = 0x3096516; continue;
 }
 0x3096516 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096538; continue; }
  pc = 0x309651f; continue;
 }
 0x309651f => {
  cpu.push(0x3096524); f_030a4341(cpu); if cpu.returned_to != 0x3096524 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096538; continue; }
  pc = 0x3096528; continue;
 }
 0x3096528 => {
  cpu.push(0x309652d); f_0309daa0(cpu); if cpu.returned_to != 0x309652d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096538; continue; }
  pc = 0x3096531; continue;
 }
 0x3096531 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309653a; continue;
 }
 0x3096538 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309653a; continue;
 }
 0x309653a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309656c; continue; }
  pc = 0x309653e; continue;
 }
 0x309653e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ad58u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096564); cpu.call(t); if cpu.returned_to != 0x3096564 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309656c); f_0309de70(cpu); if cpu.returned_to != 0x309656c { return; }
  pc = 0x309656c; continue;
 }
 0x309656c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096579); f_030ac030(cpu); if cpu.returned_to != 0x3096579 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30965c7; continue; }
  pc = 0x3096588; continue;
 }
 0x3096588 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309659a; continue; }
  pc = 0x3096591; continue;
 }
 0x3096591 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30965c7; continue; }
  pc = 0x309659a; continue;
 }
 0x309659a => {
  cpu.push(0x309659f); f_030a4341(cpu); if cpu.returned_to != 0x309659f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30965c7; continue; }
  pc = 0x30965a3; continue;
 }
 0x30965a3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30965bd); f_030a45d3(cpu); if cpu.returned_to != 0x30965bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30965c7); f_0309de20(cpu); if cpu.returned_to != 0x30965c7 { return; }
  pc = 0x30965c7; continue;
 }
 0x30965c7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_030965f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30965f0); }
 let mut pc: u32 = 0x30965f0;
 loop { match pc {
 0x30965f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096638; continue; }
  pc = 0x3096611; continue;
 }
 0x3096611 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309661f; continue; }
  pc = 0x3096616; continue;
 }
 0x3096616 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096638; continue; }
  pc = 0x309661f; continue;
 }
 0x309661f => {
  cpu.push(0x3096624); f_030a4341(cpu); if cpu.returned_to != 0x3096624 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096638; continue; }
  pc = 0x3096628; continue;
 }
 0x3096628 => {
  cpu.push(0x309662d); f_0309daa0(cpu); if cpu.returned_to != 0x309662d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096638; continue; }
  pc = 0x3096631; continue;
 }
 0x3096631 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309663a; continue;
 }
 0x3096638 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309663a; continue;
 }
 0x309663a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309666c; continue; }
  pc = 0x309663e; continue;
 }
 0x309663e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ad3cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096664); cpu.call(t); if cpu.returned_to != 0x3096664 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309666c); f_0309de70(cpu); if cpu.returned_to != 0x309666c { return; }
  pc = 0x309666c; continue;
 }
 0x309666c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096679); f_030ac080(cpu); if cpu.returned_to != 0x3096679 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30966c7; continue; }
  pc = 0x3096688; continue;
 }
 0x3096688 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309669a; continue; }
  pc = 0x3096691; continue;
 }
 0x3096691 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30966c7; continue; }
  pc = 0x309669a; continue;
 }
 0x309669a => {
  cpu.push(0x309669f); f_030a4341(cpu); if cpu.returned_to != 0x309669f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30966c7; continue; }
  pc = 0x30966a3; continue;
 }
 0x30966a3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30966bd); f_030a45d3(cpu); if cpu.returned_to != 0x30966bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30966c7); f_0309de20(cpu); if cpu.returned_to != 0x30966c7 { return; }
  pc = 0x30966c7; continue;
 }
 0x30966c7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_030966f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30966f0); }
 let mut pc: u32 = 0x30966f0;
 loop { match pc {
 0x30966f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096737; continue; }
  pc = 0x3096710; continue;
 }
 0x3096710 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309671e; continue; }
  pc = 0x3096715; continue;
 }
 0x3096715 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096737; continue; }
  pc = 0x309671e; continue;
 }
 0x309671e => {
  cpu.push(0x3096723); f_030a4341(cpu); if cpu.returned_to != 0x3096723 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096737; continue; }
  pc = 0x3096727; continue;
 }
 0x3096727 => {
  cpu.push(0x309672c); f_0309daa0(cpu); if cpu.returned_to != 0x309672c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096737; continue; }
  pc = 0x3096730; continue;
 }
 0x3096730 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096739; continue;
 }
 0x3096737 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096739; continue;
 }
 0x3096739 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096787; continue; }
  pc = 0x309673d; continue;
 }
 0x309673d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ad18u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309677f); cpu.call(t); if cpu.returned_to != 0x309677f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3096787); f_0309de70(cpu); if cpu.returned_to != 0x3096787 { return; }
  pc = 0x3096787; continue;
 }
 0x3096787 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30967a4); f_030ac0e0(cpu); if cpu.returned_to != 0x30967a4 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30967f2; continue; }
  pc = 0x30967b3; continue;
 }
 0x30967b3 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30967c5; continue; }
  pc = 0x30967bc; continue;
 }
 0x30967bc => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30967f2; continue; }
  pc = 0x30967c5; continue;
 }
 0x30967c5 => {
  cpu.push(0x30967ca); f_030a4341(cpu); if cpu.returned_to != 0x30967ca { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30967f2; continue; }
  pc = 0x30967ce; continue;
 }
 0x30967ce => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30967e8); f_030a45d3(cpu); if cpu.returned_to != 0x30967e8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30967f2); f_0309de20(cpu); if cpu.returned_to != 0x30967f2 { return; }
  pc = 0x30967f2; continue;
 }
 0x30967f2 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03096810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096810); }
 let mut pc: u32 = 0x3096810;
 loop { match pc {
 0x3096810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096858; continue; }
  pc = 0x3096831; continue;
 }
 0x3096831 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309683f; continue; }
  pc = 0x3096836; continue;
 }
 0x3096836 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096858; continue; }
  pc = 0x309683f; continue;
 }
 0x309683f => {
  cpu.push(0x3096844); f_030a4341(cpu); if cpu.returned_to != 0x3096844 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096858; continue; }
  pc = 0x3096848; continue;
 }
 0x3096848 => {
  cpu.push(0x309684d); f_0309daa0(cpu); if cpu.returned_to != 0x309684d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096858; continue; }
  pc = 0x3096851; continue;
 }
 0x3096851 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309685a; continue;
 }
 0x3096858 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309685a; continue;
 }
 0x309685a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309688c; continue; }
  pc = 0x309685e; continue;
 }
 0x309685e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314acfcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096884); cpu.call(t); if cpu.returned_to != 0x3096884 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309688c); f_0309de70(cpu); if cpu.returned_to != 0x309688c { return; }
  pc = 0x309688c; continue;
 }
 0x309688c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096899); f_030ac170(cpu); if cpu.returned_to != 0x3096899 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30968e7; continue; }
  pc = 0x30968a8; continue;
 }
 0x30968a8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30968ba; continue; }
  pc = 0x30968b1; continue;
 }
 0x30968b1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30968e7; continue; }
  pc = 0x30968ba; continue;
 }
 0x30968ba => {
  cpu.push(0x30968bf); f_030a4341(cpu); if cpu.returned_to != 0x30968bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30968e7; continue; }
  pc = 0x30968c3; continue;
 }
 0x30968c3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30968dd); f_030a45d3(cpu); if cpu.returned_to != 0x30968dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30968e7); f_0309de20(cpu); if cpu.returned_to != 0x30968e7 { return; }
  pc = 0x30968e7; continue;
 }
 0x30968e7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03096910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096910); }
 let mut pc: u32 = 0x3096910;
 loop { match pc {
 0x3096910 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096958; continue; }
  pc = 0x3096931; continue;
 }
 0x3096931 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309693f; continue; }
  pc = 0x3096936; continue;
 }
 0x3096936 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096958; continue; }
  pc = 0x309693f; continue;
 }
 0x309693f => {
  cpu.push(0x3096944); f_030a4341(cpu); if cpu.returned_to != 0x3096944 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096958; continue; }
  pc = 0x3096948; continue;
 }
 0x3096948 => {
  cpu.push(0x309694d); f_0309daa0(cpu); if cpu.returned_to != 0x309694d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096958; continue; }
  pc = 0x3096951; continue;
 }
 0x3096951 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309695a; continue;
 }
 0x3096958 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309695a; continue;
 }
 0x309695a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309698c; continue; }
  pc = 0x309695e; continue;
 }
 0x309695e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314acdcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096984); cpu.call(t); if cpu.returned_to != 0x3096984 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309698c); f_0309de70(cpu); if cpu.returned_to != 0x309698c { return; }
  pc = 0x309698c; continue;
 }
 0x309698c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096999); f_030ac1a0(cpu); if cpu.returned_to != 0x3096999 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30969e7; continue; }
  pc = 0x30969a8; continue;
 }
 0x30969a8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30969ba; continue; }
  pc = 0x30969b1; continue;
 }
 0x30969b1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30969e7; continue; }
  pc = 0x30969ba; continue;
 }
 0x30969ba => {
  cpu.push(0x30969bf); f_030a4341(cpu); if cpu.returned_to != 0x30969bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30969e7; continue; }
  pc = 0x30969c3; continue;
 }
 0x30969c3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30969dd); f_030a45d3(cpu); if cpu.returned_to != 0x30969dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30969e7); f_0309de20(cpu); if cpu.returned_to != 0x30969e7 { return; }
  pc = 0x30969e7; continue;
 }
 0x30969e7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03096b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096b10); }
 let mut pc: u32 = 0x3096b10;
 loop { match pc {
 0x3096b10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096b57; continue; }
  pc = 0x3096b30; continue;
 }
 0x3096b30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3096b3e; continue; }
  pc = 0x3096b35; continue;
 }
 0x3096b35 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096b57; continue; }
  pc = 0x3096b3e; continue;
 }
 0x3096b3e => {
  cpu.push(0x3096b43); f_030a4341(cpu); if cpu.returned_to != 0x3096b43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096b57; continue; }
  pc = 0x3096b47; continue;
 }
 0x3096b47 => {
  cpu.push(0x3096b4c); f_0309daa0(cpu); if cpu.returned_to != 0x3096b4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096b57; continue; }
  pc = 0x3096b50; continue;
 }
 0x3096b50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096b59; continue;
 }
 0x3096b57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096b59; continue;
 }
 0x3096b59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096bb5; continue; }
  pc = 0x3096b5d; continue;
 }
 0x3096b5d => {
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
  { let v = 0x314ac90u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096bad); cpu.call(t); if cpu.returned_to != 0x3096bad { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3096bb5); f_0309de70(cpu); if cpu.returned_to != 0x3096bb5 { return; }
  pc = 0x3096bb5; continue;
 }
 0x3096bb5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x3096bda); f_030ac280(cpu); if cpu.returned_to != 0x3096bda { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03096c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096c00); }
 let mut pc: u32 = 0x3096c00;
 loop { match pc {
 0x3096c00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096c48; continue; }
  pc = 0x3096c21; continue;
 }
 0x3096c21 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3096c2f; continue; }
  pc = 0x3096c26; continue;
 }
 0x3096c26 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096c48; continue; }
  pc = 0x3096c2f; continue;
 }
 0x3096c2f => {
  cpu.push(0x3096c34); f_030a4341(cpu); if cpu.returned_to != 0x3096c34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096c48; continue; }
  pc = 0x3096c38; continue;
 }
 0x3096c38 => {
  cpu.push(0x3096c3d); f_0309daa0(cpu); if cpu.returned_to != 0x3096c3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096c48; continue; }
  pc = 0x3096c41; continue;
 }
 0x3096c41 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096c4a; continue;
 }
 0x3096c48 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096c4a; continue;
 }
 0x3096c4a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096c7c; continue; }
  pc = 0x3096c4e; continue;
 }
 0x3096c4e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ac74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096c74); cpu.call(t); if cpu.returned_to != 0x3096c74 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3096c7c); f_0309de70(cpu); if cpu.returned_to != 0x3096c7c { return; }
  pc = 0x3096c7c; continue;
 }
 0x3096c7c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096c89); f_030ac2e0(cpu); if cpu.returned_to != 0x3096c89 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3096cd7; continue; }
  pc = 0x3096c98; continue;
 }
 0x3096c98 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3096caa; continue; }
  pc = 0x3096ca1; continue;
 }
 0x3096ca1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096cd7; continue; }
  pc = 0x3096caa; continue;
 }
 0x3096caa => {
  cpu.push(0x3096caf); f_030a4341(cpu); if cpu.returned_to != 0x3096caf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096cd7; continue; }
  pc = 0x3096cb3; continue;
 }
 0x3096cb3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x3096ccd); f_030a45d3(cpu); if cpu.returned_to != 0x3096ccd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3096cd7); f_0309de20(cpu); if cpu.returned_to != 0x3096cd7 { return; }
  pc = 0x3096cd7; continue;
 }
 0x3096cd7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03096d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096d00); }
 let mut pc: u32 = 0x3096d00;
 loop { match pc {
 0x3096d00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096d48; continue; }
  pc = 0x3096d21; continue;
 }
 0x3096d21 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3096d2f; continue; }
  pc = 0x3096d26; continue;
 }
 0x3096d26 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096d48; continue; }
  pc = 0x3096d2f; continue;
 }
 0x3096d2f => {
  cpu.push(0x3096d34); f_030a4341(cpu); if cpu.returned_to != 0x3096d34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096d48; continue; }
  pc = 0x3096d38; continue;
 }
 0x3096d38 => {
  cpu.push(0x3096d3d); f_0309daa0(cpu); if cpu.returned_to != 0x3096d3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096d48; continue; }
  pc = 0x3096d41; continue;
 }
 0x3096d41 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096d4a; continue;
 }
 0x3096d48 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096d4a; continue;
 }
 0x3096d4a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096d7c; continue; }
  pc = 0x3096d4e; continue;
 }
 0x3096d4e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ac58u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096d74); cpu.call(t); if cpu.returned_to != 0x3096d74 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3096d7c); f_0309de70(cpu); if cpu.returned_to != 0x3096d7c { return; }
  pc = 0x3096d7c; continue;
 }
 0x3096d7c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096d89); f_030ac300(cpu); if cpu.returned_to != 0x3096d89 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3096dd7; continue; }
  pc = 0x3096d98; continue;
 }
 0x3096d98 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3096daa; continue; }
  pc = 0x3096da1; continue;
 }
 0x3096da1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096dd7; continue; }
  pc = 0x3096daa; continue;
 }
 0x3096daa => {
  cpu.push(0x3096daf); f_030a4341(cpu); if cpu.returned_to != 0x3096daf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096dd7; continue; }
  pc = 0x3096db3; continue;
 }
 0x3096db3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x3096dcd); f_030a45d3(cpu); if cpu.returned_to != 0x3096dcd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3096dd7); f_0309de20(cpu); if cpu.returned_to != 0x3096dd7 { return; }
  pc = 0x3096dd7; continue;
 }
 0x3096dd7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03096e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096e00); }
 let mut pc: u32 = 0x3096e00;
 loop { match pc {
 0x3096e00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096e47; continue; }
  pc = 0x3096e20; continue;
 }
 0x3096e20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3096e2e; continue; }
  pc = 0x3096e25; continue;
 }
 0x3096e25 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096e47; continue; }
  pc = 0x3096e2e; continue;
 }
 0x3096e2e => {
  cpu.push(0x3096e33); f_030a4341(cpu); if cpu.returned_to != 0x3096e33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096e47; continue; }
  pc = 0x3096e37; continue;
 }
 0x3096e37 => {
  cpu.push(0x3096e3c); f_0309daa0(cpu); if cpu.returned_to != 0x3096e3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096e47; continue; }
  pc = 0x3096e40; continue;
 }
 0x3096e40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096e49; continue;
 }
 0x3096e47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096e49; continue;
 }
 0x3096e49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096e7b; continue; }
  pc = 0x3096e4d; continue;
 }
 0x3096e4d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ac38u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096e73); cpu.call(t); if cpu.returned_to != 0x3096e73 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3096e7b); f_0309de70(cpu); if cpu.returned_to != 0x3096e7b { return; }
  pc = 0x3096e7b; continue;
 }
 0x3096e7b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096e88); f_030ac360(cpu); if cpu.returned_to != 0x3096e88 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03096f60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3096f60); }
 let mut pc: u32 = 0x3096f60;
 loop { match pc {
 0x3096f60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3096fa7; continue; }
  pc = 0x3096f80; continue;
 }
 0x3096f80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3096f8e; continue; }
  pc = 0x3096f85; continue;
 }
 0x3096f85 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3096fa7; continue; }
  pc = 0x3096f8e; continue;
 }
 0x3096f8e => {
  cpu.push(0x3096f93); f_030a4341(cpu); if cpu.returned_to != 0x3096f93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3096fa7; continue; }
  pc = 0x3096f97; continue;
 }
 0x3096f97 => {
  cpu.push(0x3096f9c); f_0309daa0(cpu); if cpu.returned_to != 0x3096f9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096fa7; continue; }
  pc = 0x3096fa0; continue;
 }
 0x3096fa0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3096fa9; continue;
 }
 0x3096fa7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3096fa9; continue;
 }
 0x3096fa9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3096fdb; continue; }
  pc = 0x3096fad; continue;
 }
 0x3096fad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314ac00u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3096fd3); cpu.call(t); if cpu.returned_to != 0x3096fd3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3096fdb); f_0309de70(cpu); if cpu.returned_to != 0x3096fdb { return; }
  pc = 0x3096fdb; continue;
 }
 0x3096fdb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3096fe8); f_030ac3a0(cpu); if cpu.returned_to != 0x3096fe8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03097010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097010); }
 let mut pc: u32 = 0x3097010;
 loop { match pc {
 0x3097010 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097058; continue; }
  pc = 0x3097031; continue;
 }
 0x3097031 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309703f; continue; }
  pc = 0x3097036; continue;
 }
 0x3097036 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097058; continue; }
  pc = 0x309703f; continue;
 }
 0x309703f => {
  cpu.push(0x3097044); f_030a4341(cpu); if cpu.returned_to != 0x3097044 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097058; continue; }
  pc = 0x3097048; continue;
 }
 0x3097048 => {
  cpu.push(0x309704d); f_0309daa0(cpu); if cpu.returned_to != 0x309704d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097058; continue; }
  pc = 0x3097051; continue;
 }
 0x3097051 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309705a; continue;
 }
 0x3097058 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309705a; continue;
 }
 0x309705a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309708c; continue; }
  pc = 0x309705e; continue;
 }
 0x309705e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314abe8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097084); cpu.call(t); if cpu.returned_to != 0x3097084 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309708c); f_0309de70(cpu); if cpu.returned_to != 0x309708c { return; }
  pc = 0x309708c; continue;
 }
 0x309708c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3097099); f_030ac3f0(cpu); if cpu.returned_to != 0x3097099 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30970e7; continue; }
  pc = 0x30970a8; continue;
 }
 0x30970a8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30970ba; continue; }
  pc = 0x30970b1; continue;
 }
 0x30970b1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30970e7; continue; }
  pc = 0x30970ba; continue;
 }
 0x30970ba => {
  cpu.push(0x30970bf); f_030a4341(cpu); if cpu.returned_to != 0x30970bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30970e7; continue; }
  pc = 0x30970c3; continue;
 }
 0x30970c3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30970dd); f_030a45d3(cpu); if cpu.returned_to != 0x30970dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30970e7); f_0309ddd0(cpu); if cpu.returned_to != 0x30970e7 { return; }
  pc = 0x30970e7; continue;
 }
 0x30970e7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03097110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097110); }
 let mut pc: u32 = 0x3097110;
 loop { match pc {
 0x3097110 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097157; continue; }
  pc = 0x3097130; continue;
 }
 0x3097130 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309713e; continue; }
  pc = 0x3097135; continue;
 }
 0x3097135 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097157; continue; }
  pc = 0x309713e; continue;
 }
 0x309713e => {
  cpu.push(0x3097143); f_030a4341(cpu); if cpu.returned_to != 0x3097143 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097157; continue; }
  pc = 0x3097147; continue;
 }
 0x3097147 => {
  cpu.push(0x309714c); f_0309daa0(cpu); if cpu.returned_to != 0x309714c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097157; continue; }
  pc = 0x3097150; continue;
 }
 0x3097150 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097159; continue;
 }
 0x3097157 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097159; continue;
 }
 0x3097159 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309718b; continue; }
  pc = 0x309715d; continue;
 }
 0x309715d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314abc0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097183); cpu.call(t); if cpu.returned_to != 0x3097183 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309718b); f_0309de70(cpu); if cpu.returned_to != 0x309718b { return; }
  pc = 0x309718b; continue;
 }
 0x309718b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3097198); f_030ab580(cpu); if cpu.returned_to != 0x3097198 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030975c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30975c0); }
 let mut pc: u32 = 0x30975c0;
 loop { match pc {
 0x30975c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097607; continue; }
  pc = 0x30975e0; continue;
 }
 0x30975e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30975ee; continue; }
  pc = 0x30975e5; continue;
 }
 0x30975e5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097607; continue; }
  pc = 0x30975ee; continue;
 }
 0x30975ee => {
  cpu.push(0x30975f3); f_030a4341(cpu); if cpu.returned_to != 0x30975f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097607; continue; }
  pc = 0x30975f7; continue;
 }
 0x30975f7 => {
  cpu.push(0x30975fc); f_0309daa0(cpu); if cpu.returned_to != 0x30975fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097607; continue; }
  pc = 0x3097600; continue;
 }
 0x3097600 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097609; continue;
 }
 0x3097607 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097609; continue;
 }
 0x3097609 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097657; continue; }
  pc = 0x309760d; continue;
 }
 0x309760d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aadcu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309764f); cpu.call(t); if cpu.returned_to != 0x309764f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097657); f_0309de70(cpu); if cpu.returned_to != 0x3097657 { return; }
  pc = 0x3097657; continue;
 }
 0x3097657 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097674); f_030a7d10(cpu); if cpu.returned_to != 0x3097674 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03097690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097690); }
 let mut pc: u32 = 0x3097690;
 loop { match pc {
 0x3097690 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30976d7; continue; }
  pc = 0x30976b0; continue;
 }
 0x30976b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30976be; continue; }
  pc = 0x30976b5; continue;
 }
 0x30976b5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30976d7; continue; }
  pc = 0x30976be; continue;
 }
 0x30976be => {
  cpu.push(0x30976c3); f_030a4341(cpu); if cpu.returned_to != 0x30976c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30976d7; continue; }
  pc = 0x30976c7; continue;
 }
 0x30976c7 => {
  cpu.push(0x30976cc); f_0309daa0(cpu); if cpu.returned_to != 0x30976cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30976d7; continue; }
  pc = 0x30976d0; continue;
 }
 0x30976d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30976d9; continue;
 }
 0x30976d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30976d9; continue;
 }
 0x30976d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097719; continue; }
  pc = 0x30976dd; continue;
 }
 0x30976dd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aabcu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097711); cpu.call(t); if cpu.returned_to != 0x3097711 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097719); f_0309de70(cpu); if cpu.returned_to != 0x3097719 { return; }
  pc = 0x3097719; continue;
 }
 0x3097719 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309772e); f_030a7d50(cpu); if cpu.returned_to != 0x309772e { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03097750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097750); }
 let mut pc: u32 = 0x3097750;
 loop { match pc {
 0x3097750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097798; continue; }
  pc = 0x3097771; continue;
 }
 0x3097771 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309777f; continue; }
  pc = 0x3097776; continue;
 }
 0x3097776 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097798; continue; }
  pc = 0x309777f; continue;
 }
 0x309777f => {
  cpu.push(0x3097784); f_030a4341(cpu); if cpu.returned_to != 0x3097784 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097798; continue; }
  pc = 0x3097788; continue;
 }
 0x3097788 => {
  cpu.push(0x309778d); f_0309daa0(cpu); if cpu.returned_to != 0x309778d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097798; continue; }
  pc = 0x3097791; continue;
 }
 0x3097791 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309779a; continue;
 }
 0x3097798 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309779a; continue;
 }
 0x309779a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30977cc; continue; }
  pc = 0x309779e; continue;
 }
 0x309779e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aaa0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30977c4); cpu.call(t); if cpu.returned_to != 0x30977c4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30977cc); f_0309de70(cpu); if cpu.returned_to != 0x30977cc { return; }
  pc = 0x30977cc; continue;
 }
 0x30977cc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30977d9); f_030a7e70(cpu); if cpu.returned_to != 0x30977d9 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3097827; continue; }
  pc = 0x30977e8; continue;
 }
 0x30977e8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30977fa; continue; }
  pc = 0x30977f1; continue;
 }
 0x30977f1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097827; continue; }
  pc = 0x30977fa; continue;
 }
 0x30977fa => {
  cpu.push(0x30977ff); f_030a4341(cpu); if cpu.returned_to != 0x30977ff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097827; continue; }
  pc = 0x3097803; continue;
 }
 0x3097803 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309781d); f_030a45d3(cpu); if cpu.returned_to != 0x309781d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3097827); f_0309de20(cpu); if cpu.returned_to != 0x3097827 { return; }
  pc = 0x3097827; continue;
 }
 0x3097827 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03097850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097850); }
 let mut pc: u32 = 0x3097850;
 loop { match pc {
 0x3097850 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097897; continue; }
  pc = 0x3097870; continue;
 }
 0x3097870 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309787e; continue; }
  pc = 0x3097875; continue;
 }
 0x3097875 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097897; continue; }
  pc = 0x309787e; continue;
 }
 0x309787e => {
  cpu.push(0x3097883); f_030a4341(cpu); if cpu.returned_to != 0x3097883 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097897; continue; }
  pc = 0x3097887; continue;
 }
 0x3097887 => {
  cpu.push(0x309788c); f_0309daa0(cpu); if cpu.returned_to != 0x309788c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097897; continue; }
  pc = 0x3097890; continue;
 }
 0x3097890 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097899; continue;
 }
 0x3097897 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097899; continue;
 }
 0x3097899 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30978d9; continue; }
  pc = 0x309789d; continue;
 }
 0x309789d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aa7cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30978d1); cpu.call(t); if cpu.returned_to != 0x30978d1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30978d9); f_0309de70(cpu); if cpu.returned_to != 0x30978d9 { return; }
  pc = 0x30978d9; continue;
 }
 0x30978d9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x30978ee); f_030a7ff0(cpu); if cpu.returned_to != 0x30978ee { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03097910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097910); }
 let mut pc: u32 = 0x3097910;
 loop { match pc {
 0x3097910 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097958; continue; }
  pc = 0x3097931; continue;
 }
 0x3097931 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309793f; continue; }
  pc = 0x3097936; continue;
 }
 0x3097936 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097958; continue; }
  pc = 0x309793f; continue;
 }
 0x309793f => {
  cpu.push(0x3097944); f_030a4341(cpu); if cpu.returned_to != 0x3097944 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097958; continue; }
  pc = 0x3097948; continue;
 }
 0x3097948 => {
  cpu.push(0x309794d); f_0309daa0(cpu); if cpu.returned_to != 0x309794d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097958; continue; }
  pc = 0x3097951; continue;
 }
 0x3097951 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309795a; continue;
 }
 0x3097958 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309795a; continue;
 }
 0x309795a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309799a; continue; }
  pc = 0x309795e; continue;
 }
 0x309795e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aa58u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097992); cpu.call(t); if cpu.returned_to != 0x3097992 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309799a); f_0309de70(cpu); if cpu.returned_to != 0x309799a { return; }
  pc = 0x309799a; continue;
 }
 0x309799a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x30979af); f_030a8000(cpu); if cpu.returned_to != 0x30979af { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30979fc; continue; }
  pc = 0x30979bd; continue;
 }
 0x30979bd => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30979cf; continue; }
  pc = 0x30979c6; continue;
 }
 0x30979c6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30979fc; continue; }
  pc = 0x30979cf; continue;
 }
 0x30979cf => {
  cpu.push(0x30979d4); f_030a4341(cpu); if cpu.returned_to != 0x30979d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30979fc; continue; }
  pc = 0x30979d8; continue;
 }
 0x30979d8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x30979f2); f_030a45d3(cpu); if cpu.returned_to != 0x30979f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30979fc); f_0309de20(cpu); if cpu.returned_to != 0x30979fc { return; }
  pc = 0x30979fc; continue;
 }
 0x30979fc => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03097a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097a20); }
 let mut pc: u32 = 0x3097a20;
 loop { match pc {
 0x3097a20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097a67; continue; }
  pc = 0x3097a40; continue;
 }
 0x3097a40 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3097a4e; continue; }
  pc = 0x3097a45; continue;
 }
 0x3097a45 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097a67; continue; }
  pc = 0x3097a4e; continue;
 }
 0x3097a4e => {
  cpu.push(0x3097a53); f_030a4341(cpu); if cpu.returned_to != 0x3097a53 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097a67; continue; }
  pc = 0x3097a57; continue;
 }
 0x3097a57 => {
  cpu.push(0x3097a5c); f_0309daa0(cpu); if cpu.returned_to != 0x3097a5c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097a67; continue; }
  pc = 0x3097a60; continue;
 }
 0x3097a60 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097a69; continue;
 }
 0x3097a67 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097a69; continue;
 }
 0x3097a69 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097ab7; continue; }
  pc = 0x3097a6d; continue;
 }
 0x3097a6d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aa2cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097aaf); cpu.call(t); if cpu.returned_to != 0x3097aaf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097ab7); f_0309de70(cpu); if cpu.returned_to != 0x3097ab7 { return; }
  pc = 0x3097ab7; continue;
 }
 0x3097ab7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097ad4); f_030a8020(cpu); if cpu.returned_to != 0x3097ad4 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03097af0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097af0); }
 let mut pc: u32 = 0x3097af0;
 loop { match pc {
 0x3097af0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097b38; continue; }
  pc = 0x3097b11; continue;
 }
 0x3097b11 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3097b1f; continue; }
  pc = 0x3097b16; continue;
 }
 0x3097b16 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097b38; continue; }
  pc = 0x3097b1f; continue;
 }
 0x3097b1f => {
  cpu.push(0x3097b24); f_030a4341(cpu); if cpu.returned_to != 0x3097b24 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097b38; continue; }
  pc = 0x3097b28; continue;
 }
 0x3097b28 => {
  cpu.push(0x3097b2d); f_0309daa0(cpu); if cpu.returned_to != 0x3097b2d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097b38; continue; }
  pc = 0x3097b31; continue;
 }
 0x3097b31 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097b3a; continue;
 }
 0x3097b38 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097b3a; continue;
 }
 0x3097b3a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097b7a; continue; }
  pc = 0x3097b3e; continue;
 }
 0x3097b3e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314aa00u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097b72); cpu.call(t); if cpu.returned_to != 0x3097b72 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097b7a); f_0309de70(cpu); if cpu.returned_to != 0x3097b7a { return; }
  pc = 0x3097b7a; continue;
 }
 0x3097b7a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x3097b8f); f_030a8040(cpu); if cpu.returned_to != 0x3097b8f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097bdc; continue; }
  pc = 0x3097b9d; continue;
 }
 0x3097b9d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3097baf; continue; }
  pc = 0x3097ba6; continue;
 }
 0x3097ba6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097bdc; continue; }
  pc = 0x3097baf; continue;
 }
 0x3097baf => {
  cpu.push(0x3097bb4); f_030a4341(cpu); if cpu.returned_to != 0x3097bb4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097bdc; continue; }
  pc = 0x3097bb8; continue;
 }
 0x3097bb8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3097bd2); f_030a45d3(cpu); if cpu.returned_to != 0x3097bd2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3097bdc); f_0309ddd0(cpu); if cpu.returned_to != 0x3097bdc { return; }
  pc = 0x3097bdc; continue;
 }
 0x3097bdc => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03097c00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3097c00); } r_03097c00(cpu, 0x3097c00); }
pub fn f_03097cdf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3097cdf); } r_03097c00(cpu, 0x3097cdf); }
fn r_03097c00(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x3097c00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097c48; continue; }
  pc = 0x3097c21; continue;
 }
 0x3097c21 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3097c2f; continue; }
  pc = 0x3097c26; continue;
 }
 0x3097c26 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097c48; continue; }
  pc = 0x3097c2f; continue;
 }
 0x3097c2f => {
  cpu.push(0x3097c34); f_030a4341(cpu); if cpu.returned_to != 0x3097c34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097c48; continue; }
  pc = 0x3097c38; continue;
 }
 0x3097c38 => {
  cpu.push(0x3097c3d); f_0309daa0(cpu); if cpu.returned_to != 0x3097c3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097c48; continue; }
  pc = 0x3097c41; continue;
 }
 0x3097c41 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097c4a; continue;
 }
 0x3097c48 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097c4a; continue;
 }
 0x3097c4a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097c8a; continue; }
  pc = 0x3097c4e; continue;
 }
 0x3097c4e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a9d4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097c82); cpu.call(t); if cpu.returned_to != 0x3097c82 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097c8a); f_0309de70(cpu); if cpu.returned_to != 0x3097c8a { return; }
  pc = 0x3097c8a; continue;
 }
 0x3097c8a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x3097c9f); f_030a8060(cpu); if cpu.returned_to != 0x3097c9f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097cec; continue; }
  pc = 0x3097cad; continue;
 }
 0x3097cad => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3097cbf; continue; }
  pc = 0x3097cb6; continue;
 }
 0x3097cb6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097cec; continue; }
  pc = 0x3097cbf; continue;
 }
 0x3097cbf => {
  cpu.push(0x3097cc4); f_030a4341(cpu); if cpu.returned_to != 0x3097cc4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097cec; continue; }
  pc = 0x3097cc8; continue;
 }
 0x3097cc8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3097ce2); f_030a45d3(cpu); if cpu.returned_to != 0x3097ce2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3097cec); f_0309ddd0(cpu); if cpu.returned_to != 0x3097cec { return; }
  pc = 0x3097cec; continue;
 }
 0x3097cdf => {
  cpu.unimplemented(0x3097cdf, "Enter");
  { let at = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(4)); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  { let v = cpu.st(0) - cpu.st(0); cpu.set_st(0, v); }
  { let v: u8 = cpu.port_in(0x60u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x3097cec; continue;
 }
 0x3097cec => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03097d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097d10); }
 let mut pc: u32 = 0x3097d10;
 loop { match pc {
 0x3097d10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097d58; continue; }
  pc = 0x3097d31; continue;
 }
 0x3097d31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3097d3f; continue; }
  pc = 0x3097d36; continue;
 }
 0x3097d36 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097d58; continue; }
  pc = 0x3097d3f; continue;
 }
 0x3097d3f => {
  cpu.push(0x3097d44); f_030a4341(cpu); if cpu.returned_to != 0x3097d44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097d58; continue; }
  pc = 0x3097d48; continue;
 }
 0x3097d48 => {
  cpu.push(0x3097d4d); f_0309daa0(cpu); if cpu.returned_to != 0x3097d4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097d58; continue; }
  pc = 0x3097d51; continue;
 }
 0x3097d51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097d5a; continue;
 }
 0x3097d58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097d5a; continue;
 }
 0x3097d5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097d9a; continue; }
  pc = 0x3097d5e; continue;
 }
 0x3097d5e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a9acu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097d92); cpu.call(t); if cpu.returned_to != 0x3097d92 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097d9a); f_0309de70(cpu); if cpu.returned_to != 0x3097d9a { return; }
  pc = 0x3097d9a; continue;
 }
 0x3097d9a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x3097daf); f_030a8080(cpu); if cpu.returned_to != 0x3097daf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097dfc; continue; }
  pc = 0x3097dbd; continue;
 }
 0x3097dbd => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3097dcf; continue; }
  pc = 0x3097dc6; continue;
 }
 0x3097dc6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097dfc; continue; }
  pc = 0x3097dcf; continue;
 }
 0x3097dcf => {
  cpu.push(0x3097dd4); f_030a4341(cpu); if cpu.returned_to != 0x3097dd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097dfc; continue; }
  pc = 0x3097dd8; continue;
 }
 0x3097dd8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3097df2); f_030a45d3(cpu); if cpu.returned_to != 0x3097df2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3097dfc); f_0309ddd0(cpu); if cpu.returned_to != 0x3097dfc { return; }
  pc = 0x3097dfc; continue;
 }
 0x3097dfc => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03097e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3097e20); }
 let mut pc: u32 = 0x3097e20;
 loop { match pc {
 0x3097e20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3097e68; continue; }
  pc = 0x3097e41; continue;
 }
 0x3097e41 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3097e4f; continue; }
  pc = 0x3097e46; continue;
 }
 0x3097e46 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097e68; continue; }
  pc = 0x3097e4f; continue;
 }
 0x3097e4f => {
  cpu.push(0x3097e54); f_030a4341(cpu); if cpu.returned_to != 0x3097e54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097e68; continue; }
  pc = 0x3097e58; continue;
 }
 0x3097e58 => {
  cpu.push(0x3097e5d); f_0309daa0(cpu); if cpu.returned_to != 0x3097e5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097e68; continue; }
  pc = 0x3097e61; continue;
 }
 0x3097e61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3097e6a; continue;
 }
 0x3097e68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3097e6a; continue;
 }
 0x3097e6a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097eaa; continue; }
  pc = 0x3097e6e; continue;
 }
 0x3097e6e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a980u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3097ea2); cpu.call(t); if cpu.returned_to != 0x3097ea2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3097eaa); f_0309de70(cpu); if cpu.returned_to != 0x3097eaa { return; }
  pc = 0x3097eaa; continue;
 }
 0x3097eaa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x3097ebf); f_030a80a0(cpu); if cpu.returned_to != 0x3097ebf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3097f0c; continue; }
  pc = 0x3097ecd; continue;
 }
 0x3097ecd => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3097edf; continue; }
  pc = 0x3097ed6; continue;
 }
 0x3097ed6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3097f0c; continue; }
  pc = 0x3097edf; continue;
 }
 0x3097edf => {
  cpu.push(0x3097ee4); f_030a4341(cpu); if cpu.returned_to != 0x3097ee4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3097f0c; continue; }
  pc = 0x3097ee8; continue;
 }
 0x3097ee8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3097f02); f_030a45d3(cpu); if cpu.returned_to != 0x3097f02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3097f0c); f_0309ddd0(cpu); if cpu.returned_to != 0x3097f0c { return; }
  pc = 0x3097f0c; continue;
 }
 0x3097f0c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03098150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3098150); }
 let mut pc: u32 = 0x3098150;
 loop { match pc {
 0x3098150 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3098197; continue; }
  pc = 0x3098170; continue;
 }
 0x3098170 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309817e; continue; }
  pc = 0x3098175; continue;
 }
 0x3098175 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3098197; continue; }
  pc = 0x309817e; continue;
 }
 0x309817e => {
  cpu.push(0x3098183); f_030a4341(cpu); if cpu.returned_to != 0x3098183 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3098197; continue; }
  pc = 0x3098187; continue;
 }
 0x3098187 => {
  cpu.push(0x309818c); f_0309daa0(cpu); if cpu.returned_to != 0x309818c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098197; continue; }
  pc = 0x3098190; continue;
 }
 0x3098190 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3098199; continue;
 }
 0x3098197 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3098199; continue;
 }
 0x3098199 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30981d9; continue; }
  pc = 0x309819d; continue;
 }
 0x309819d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a90cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30981d1); cpu.call(t); if cpu.returned_to != 0x30981d1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30981d9); f_0309de70(cpu); if cpu.returned_to != 0x30981d9 { return; }
  pc = 0x30981d9; continue;
 }
 0x30981d9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x30981ee); f_030a8100(cpu); if cpu.returned_to != 0x30981ee { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03098210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3098210); }
 let mut pc: u32 = 0x3098210;
 loop { match pc {
 0x3098210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x114u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3098266; continue; }
  pc = 0x309823f; continue;
 }
 0x309823f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309824d; continue; }
  pc = 0x3098244; continue;
 }
 0x3098244 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3098266; continue; }
  pc = 0x309824d; continue;
 }
 0x309824d => {
  cpu.push(0x3098252); f_030a4341(cpu); if cpu.returned_to != 0x3098252 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3098266; continue; }
  pc = 0x3098256; continue;
 }
 0x3098256 => {
  cpu.push(0x309825b); f_0309daa0(cpu); if cpu.returned_to != 0x309825b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098266; continue; }
  pc = 0x309825f; continue;
 }
 0x309825f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3098268; continue;
 }
 0x3098266 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3098268; continue;
 }
 0x3098268 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30982a8; continue; }
  pc = 0x309826c; continue;
 }
 0x309826c => {
  { let v: u32 = cpu.r32(0x128u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a8d0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30982a0); cpu.call(t); if cpu.returned_to != 0x30982a0 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30982a8); f_0309de70(cpu); if cpu.returned_to != 0x30982a8 { return; }
  pc = 0x30982a8; continue;
 }
 0x30982a8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30982b7); f_030a8190(cpu); if cpu.returned_to != 0x30982b7 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309833e; continue; }
  pc = 0x30982c8; continue;
 }
 0x30982c8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30982da; continue; }
  pc = 0x30982d1; continue;
 }
 0x30982d1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309833e; continue; }
  pc = 0x30982da; continue;
 }
 0x30982da => {
  cpu.push(0x30982df); f_030a4341(cpu); if cpu.returned_to != 0x30982df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309833e; continue; }
  pc = 0x30982e3; continue;
 }
 0x30982e3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x30982fd); f_030a45d3(cpu); if cpu.returned_to != 0x30982fd { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a8f8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3098331); cpu.call(t); if cpu.returned_to != 0x3098331 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309833e); f_0309de70(cpu); if cpu.returned_to != 0x309833e { return; }
  pc = 0x309833e; continue;
 }
 0x309833e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x32d0cd0u32)); cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03098350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3098350); }
 let mut pc: u32 = 0x3098350;
 loop { match pc {
 0x3098350 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3098398; continue; }
  pc = 0x3098371; continue;
 }
 0x3098371 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309837f; continue; }
  pc = 0x3098376; continue;
 }
 0x3098376 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3098398; continue; }
  pc = 0x309837f; continue;
 }
 0x309837f => {
  cpu.push(0x3098384); f_030a4341(cpu); if cpu.returned_to != 0x3098384 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3098398; continue; }
  pc = 0x3098388; continue;
 }
 0x3098388 => {
  cpu.push(0x309838d); f_0309daa0(cpu); if cpu.returned_to != 0x309838d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098398; continue; }
  pc = 0x3098391; continue;
 }
 0x3098391 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309839a; continue;
 }
 0x3098398 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309839a; continue;
 }
 0x309839a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30983da; continue; }
  pc = 0x309839e; continue;
 }
 0x309839e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a8b0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30983d2); cpu.call(t); if cpu.returned_to != 0x30983d2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30983da); f_0309de70(cpu); if cpu.returned_to != 0x30983da { return; }
  pc = 0x30983da; continue;
 }
 0x30983da => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x30983ef); f_030a8210(cpu); if cpu.returned_to != 0x30983ef { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309843c; continue; }
  pc = 0x30983fd; continue;
 }
 0x30983fd => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309840f; continue; }
  pc = 0x3098406; continue;
 }
 0x3098406 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309843c; continue; }
  pc = 0x309840f; continue;
 }
 0x309840f => {
  cpu.push(0x3098414); f_030a4341(cpu); if cpu.returned_to != 0x3098414 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309843c; continue; }
  pc = 0x3098418; continue;
 }
 0x3098418 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3098432); f_030a45d3(cpu); if cpu.returned_to != 0x3098432 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309843c); f_0309de20(cpu); if cpu.returned_to != 0x309843c { return; }
  pc = 0x309843c; continue;
 }
 0x309843c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_03098a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3098a60); }
 let mut pc: u32 = 0x3098a60;
 loop { match pc {
 0x3098a60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3098aa7; continue; }
  pc = 0x3098a80; continue;
 }
 0x3098a80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3098a8e; continue; }
  pc = 0x3098a85; continue;
 }
 0x3098a85 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3098aa7; continue; }
  pc = 0x3098a8e; continue;
 }
 0x3098a8e => {
  cpu.push(0x3098a93); f_030a4341(cpu); if cpu.returned_to != 0x3098a93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3098aa7; continue; }
  pc = 0x3098a97; continue;
 }
 0x3098a97 => {
  cpu.push(0x3098a9c); f_0309daa0(cpu); if cpu.returned_to != 0x3098a9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098aa7; continue; }
  pc = 0x3098aa0; continue;
 }
 0x3098aa0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3098aa9; continue;
 }
 0x3098aa7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3098aa9; continue;
 }
 0x3098aa9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098adb; continue; }
  pc = 0x3098aad; continue;
 }
 0x3098aad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a7b8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3098ad3); cpu.call(t); if cpu.returned_to != 0x3098ad3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3098adb); f_0309de70(cpu); if cpu.returned_to != 0x3098adb { return; }
  pc = 0x3098adb; continue;
 }
 0x3098adb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3098ae8); f_030a8ca0(cpu); if cpu.returned_to != 0x3098ae8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03098b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3098b10); }
 let mut pc: u32 = 0x3098b10;
 loop { match pc {
 0x3098b10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3098b58; continue; }
  pc = 0x3098b31; continue;
 }
 0x3098b31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3098b3f; continue; }
  pc = 0x3098b36; continue;
 }
 0x3098b36 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3098b58; continue; }
  pc = 0x3098b3f; continue;
 }
 0x3098b3f => {
  cpu.push(0x3098b44); f_030a4341(cpu); if cpu.returned_to != 0x3098b44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3098b58; continue; }
  pc = 0x3098b48; continue;
 }
 0x3098b48 => {
  cpu.push(0x3098b4d); f_0309daa0(cpu); if cpu.returned_to != 0x3098b4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098b58; continue; }
  pc = 0x3098b51; continue;
 }
 0x3098b51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3098b5a; continue;
 }
 0x3098b58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3098b5a; continue;
 }
 0x3098b5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3098b8c; continue; }
  pc = 0x3098b5e; continue;
 }
 0x3098b5e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a79cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3098b84); cpu.call(t); if cpu.returned_to != 0x3098b84 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3098b8c); f_0309de70(cpu); if cpu.returned_to != 0x3098b8c { return; }
  pc = 0x3098b8c; continue;
 }
 0x3098b8c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3098b99); f_030a8d20(cpu); if cpu.returned_to != 0x3098b99 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3098be7; continue; }
  pc = 0x3098ba8; continue;
 }
 0x3098ba8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3098bba; continue; }
  pc = 0x3098bb1; continue;
 }
 0x3098bb1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3098be7; continue; }
  pc = 0x3098bba; continue;
 }
 0x3098bba => {
  cpu.push(0x3098bbf); f_030a4341(cpu); if cpu.returned_to != 0x3098bbf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3098be7; continue; }
  pc = 0x3098bc3; continue;
 }
 0x3098bc3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x3098bdd); f_030a45d3(cpu); if cpu.returned_to != 0x3098bdd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3098be7); f_0309de20(cpu); if cpu.returned_to != 0x3098be7 { return; }
  pc = 0x3098be7; continue;
 }
 0x3098be7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_030991f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30991f0); }
 let mut pc: u32 = 0x30991f0;
 loop { match pc {
 0x30991f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099238; continue; }
  pc = 0x3099211; continue;
 }
 0x3099211 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309921f; continue; }
  pc = 0x3099216; continue;
 }
 0x3099216 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099238; continue; }
  pc = 0x309921f; continue;
 }
 0x309921f => {
  cpu.push(0x3099224); f_030a4341(cpu); if cpu.returned_to != 0x3099224 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099238; continue; }
  pc = 0x3099228; continue;
 }
 0x3099228 => {
  cpu.push(0x309922d); f_0309daa0(cpu); if cpu.returned_to != 0x309922d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099238; continue; }
  pc = 0x3099231; continue;
 }
 0x3099231 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309923a; continue;
 }
 0x3099238 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309923a; continue;
 }
 0x309923a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309926c; continue; }
  pc = 0x309923e; continue;
 }
 0x309923e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a6b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099264); cpu.call(t); if cpu.returned_to != 0x3099264 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309926c); f_0309de70(cpu); if cpu.returned_to != 0x309926c { return; }
  pc = 0x309926c; continue;
 }
 0x309926c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099279); f_030a8500(cpu); if cpu.returned_to != 0x3099279 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30992c7; continue; }
  pc = 0x3099288; continue;
 }
 0x3099288 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309929a; continue; }
  pc = 0x3099291; continue;
 }
 0x3099291 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30992c7; continue; }
  pc = 0x309929a; continue;
 }
 0x309929a => {
  cpu.push(0x309929f); f_030a4341(cpu); if cpu.returned_to != 0x309929f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30992c7; continue; }
  pc = 0x30992a3; continue;
 }
 0x30992a3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x30992bd); f_030a45d3(cpu); if cpu.returned_to != 0x30992bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30992c7); f_0309de20(cpu); if cpu.returned_to != 0x30992c7 { return; }
  pc = 0x30992c7; continue;
 }
 0x30992c7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_030992f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30992f0); }
 let mut pc: u32 = 0x30992f0;
 loop { match pc {
 0x30992f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099337; continue; }
  pc = 0x3099310; continue;
 }
 0x3099310 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309931e; continue; }
  pc = 0x3099315; continue;
 }
 0x3099315 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099337; continue; }
  pc = 0x309931e; continue;
 }
 0x309931e => {
  cpu.push(0x3099323); f_030a4341(cpu); if cpu.returned_to != 0x3099323 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099337; continue; }
  pc = 0x3099327; continue;
 }
 0x3099327 => {
  cpu.push(0x309932c); f_0309daa0(cpu); if cpu.returned_to != 0x309932c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099337; continue; }
  pc = 0x3099330; continue;
 }
 0x3099330 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099339; continue;
 }
 0x3099337 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099339; continue;
 }
 0x3099339 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099379; continue; }
  pc = 0x309933d; continue;
 }
 0x309933d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a688u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099371); cpu.call(t); if cpu.returned_to != 0x3099371 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099379); f_0309de70(cpu); if cpu.returned_to != 0x3099379 { return; }
  pc = 0x3099379; continue;
 }
 0x3099379 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309938e); f_030a8600(cpu); if cpu.returned_to != 0x309938e { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_030993b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30993b0); }
 let mut pc: u32 = 0x30993b0;
 loop { match pc {
 0x30993b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30993f7; continue; }
  pc = 0x30993d0; continue;
 }
 0x30993d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30993de; continue; }
  pc = 0x30993d5; continue;
 }
 0x30993d5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30993f7; continue; }
  pc = 0x30993de; continue;
 }
 0x30993de => {
  cpu.push(0x30993e3); f_030a4341(cpu); if cpu.returned_to != 0x30993e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30993f7; continue; }
  pc = 0x30993e7; continue;
 }
 0x30993e7 => {
  cpu.push(0x30993ec); f_0309daa0(cpu); if cpu.returned_to != 0x30993ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30993f7; continue; }
  pc = 0x30993f0; continue;
 }
 0x30993f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30993f9; continue;
 }
 0x30993f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30993f9; continue;
 }
 0x30993f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099447; continue; }
  pc = 0x30993fd; continue;
 }
 0x30993fd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a660u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309943f); cpu.call(t); if cpu.returned_to != 0x309943f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099447); f_0309de70(cpu); if cpu.returned_to != 0x3099447 { return; }
  pc = 0x3099447; continue;
 }
 0x3099447 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099464); f_030a8510(cpu); if cpu.returned_to != 0x3099464 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03099480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099480); }
 let mut pc: u32 = 0x3099480;
 loop { match pc {
 0x3099480 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30994c7; continue; }
  pc = 0x30994a0; continue;
 }
 0x30994a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30994ae; continue; }
  pc = 0x30994a5; continue;
 }
 0x30994a5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30994c7; continue; }
  pc = 0x30994ae; continue;
 }
 0x30994ae => {
  cpu.push(0x30994b3); f_030a4341(cpu); if cpu.returned_to != 0x30994b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30994c7; continue; }
  pc = 0x30994b7; continue;
 }
 0x30994b7 => {
  cpu.push(0x30994bc); f_0309daa0(cpu); if cpu.returned_to != 0x30994bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30994c7; continue; }
  pc = 0x30994c0; continue;
 }
 0x30994c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30994c9; continue;
 }
 0x30994c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30994c9; continue;
 }
 0x30994c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099517; continue; }
  pc = 0x30994cd; continue;
 }
 0x30994cd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a638u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309950f); cpu.call(t); if cpu.returned_to != 0x309950f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099517); f_0309de70(cpu); if cpu.returned_to != 0x3099517 { return; }
  pc = 0x3099517; continue;
 }
 0x3099517 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099534); f_030a8590(cpu); if cpu.returned_to != 0x3099534 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_03099550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099550); }
 let mut pc: u32 = 0x3099550;
 loop { match pc {
 0x3099550 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099597; continue; }
  pc = 0x3099570; continue;
 }
 0x3099570 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309957e; continue; }
  pc = 0x3099575; continue;
 }
 0x3099575 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099597; continue; }
  pc = 0x309957e; continue;
 }
 0x309957e => {
  cpu.push(0x3099583); f_030a4341(cpu); if cpu.returned_to != 0x3099583 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099597; continue; }
  pc = 0x3099587; continue;
 }
 0x3099587 => {
  cpu.push(0x309958c); f_0309daa0(cpu); if cpu.returned_to != 0x309958c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099597; continue; }
  pc = 0x3099590; continue;
 }
 0x3099590 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099599; continue;
 }
 0x3099597 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099599; continue;
 }
 0x3099599 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30995cb; continue; }
  pc = 0x309959d; continue;
 }
 0x309959d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a61cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30995c3); cpu.call(t); if cpu.returned_to != 0x30995c3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30995cb); f_0309de70(cpu); if cpu.returned_to != 0x30995cb { return; }
  pc = 0x30995cb; continue;
 }
 0x30995cb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30995d8); f_030a8610(cpu); if cpu.returned_to != 0x30995d8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_03099600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099600); }
 let mut pc: u32 = 0x3099600;
 loop { match pc {
 0x3099600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099647; continue; }
  pc = 0x3099620; continue;
 }
 0x3099620 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309962e; continue; }
  pc = 0x3099625; continue;
 }
 0x3099625 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099647; continue; }
  pc = 0x309962e; continue;
 }
 0x309962e => {
  cpu.push(0x3099633); f_030a4341(cpu); if cpu.returned_to != 0x3099633 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099647; continue; }
  pc = 0x3099637; continue;
 }
 0x3099637 => {
  cpu.push(0x309963c); f_0309daa0(cpu); if cpu.returned_to != 0x309963c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099647; continue; }
  pc = 0x3099640; continue;
 }
 0x3099640 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099649; continue;
 }
 0x3099647 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099649; continue;
 }
 0x3099649 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309967b; continue; }
  pc = 0x309964d; continue;
 }
 0x309964d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a600u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099673); cpu.call(t); if cpu.returned_to != 0x3099673 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309967b); f_0309de70(cpu); if cpu.returned_to != 0x309967b { return; }
  pc = 0x309967b; continue;
 }
 0x309967b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099688); f_030a8650(cpu); if cpu.returned_to != 0x3099688 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_030996b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30996b0); }
 let mut pc: u32 = 0x30996b0;
 loop { match pc {
 0x30996b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30996f7; continue; }
  pc = 0x30996d0; continue;
 }
 0x30996d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x30996de; continue; }
  pc = 0x30996d5; continue;
 }
 0x30996d5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30996f7; continue; }
  pc = 0x30996de; continue;
 }
 0x30996de => {
  cpu.push(0x30996e3); f_030a4341(cpu); if cpu.returned_to != 0x30996e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30996f7; continue; }
  pc = 0x30996e7; continue;
 }
 0x30996e7 => {
  cpu.push(0x30996ec); f_0309daa0(cpu); if cpu.returned_to != 0x30996ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30996f7; continue; }
  pc = 0x30996f0; continue;
 }
 0x30996f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30996f9; continue;
 }
 0x30996f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30996f9; continue;
 }
 0x30996f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309972b; continue; }
  pc = 0x30996fd; continue;
 }
 0x30996fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a5e4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099723); cpu.call(t); if cpu.returned_to != 0x3099723 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309972b); f_0309de70(cpu); if cpu.returned_to != 0x309972b { return; }
  pc = 0x309972b; continue;
 }
 0x309972b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099738); f_030a86b0(cpu); if cpu.returned_to != 0x3099738 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_03099760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099760); }
 let mut pc: u32 = 0x3099760;
 loop { match pc {
 0x3099760 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30997a7; continue; }
  pc = 0x3099780; continue;
 }
 0x3099780 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309978e; continue; }
  pc = 0x3099785; continue;
 }
 0x3099785 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x30997a7; continue; }
  pc = 0x309978e; continue;
 }
 0x309978e => {
  cpu.push(0x3099793); f_030a4341(cpu); if cpu.returned_to != 0x3099793 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30997a7; continue; }
  pc = 0x3099797; continue;
 }
 0x3099797 => {
  cpu.push(0x309979c); f_0309daa0(cpu); if cpu.returned_to != 0x309979c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30997a7; continue; }
  pc = 0x30997a0; continue;
 }
 0x30997a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30997a9; continue;
 }
 0x30997a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30997a9; continue;
 }
 0x30997a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30997db; continue; }
  pc = 0x30997ad; continue;
 }
 0x30997ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a5c8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30997d3); cpu.call(t); if cpu.returned_to != 0x30997d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30997db); f_0309de70(cpu); if cpu.returned_to != 0x30997db { return; }
  pc = 0x30997db; continue;
 }
 0x30997db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30997e8); f_030a8760(cpu); if cpu.returned_to != 0x30997e8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_03099810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099810); }
 let mut pc: u32 = 0x3099810;
 loop { match pc {
 0x3099810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099857; continue; }
  pc = 0x3099830; continue;
 }
 0x3099830 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309983e; continue; }
  pc = 0x3099835; continue;
 }
 0x3099835 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099857; continue; }
  pc = 0x309983e; continue;
 }
 0x309983e => {
  cpu.push(0x3099843); f_030a4341(cpu); if cpu.returned_to != 0x3099843 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099857; continue; }
  pc = 0x3099847; continue;
 }
 0x3099847 => {
  cpu.push(0x309984c); f_0309daa0(cpu); if cpu.returned_to != 0x309984c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099857; continue; }
  pc = 0x3099850; continue;
 }
 0x3099850 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099859; continue;
 }
 0x3099857 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099859; continue;
 }
 0x3099859 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30998a7; continue; }
  pc = 0x309985d; continue;
 }
 0x309985d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a5a4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309989f); cpu.call(t); if cpu.returned_to != 0x309989f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30998a7); f_0309de70(cpu); if cpu.returned_to != 0x30998a7 { return; }
  pc = 0x30998a7; continue;
 }
 0x30998a7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30998c4); f_030a87a0(cpu); if cpu.returned_to != 0x30998c4 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x3099912; continue; }
  pc = 0x30998d3; continue;
 }
 0x30998d3 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x30998e5; continue; }
  pc = 0x30998dc; continue;
 }
 0x30998dc => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099912; continue; }
  pc = 0x30998e5; continue;
 }
 0x30998e5 => {
  cpu.push(0x30998ea); f_030a4341(cpu); if cpu.returned_to != 0x30998ea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099912; continue; }
  pc = 0x30998ee; continue;
 }
 0x30998ee => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099908); f_030a45d3(cpu); if cpu.returned_to != 0x3099908 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3099912); f_0309de20(cpu); if cpu.returned_to != 0x3099912 { return; }
  pc = 0x3099912; continue;
 }
 0x3099912 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_03099930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099930); }
 let mut pc: u32 = 0x3099930;
 loop { match pc {
 0x3099930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099977; continue; }
  pc = 0x3099950; continue;
 }
 0x3099950 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309995e; continue; }
  pc = 0x3099955; continue;
 }
 0x3099955 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099977; continue; }
  pc = 0x309995e; continue;
 }
 0x309995e => {
  cpu.push(0x3099963); f_030a4341(cpu); if cpu.returned_to != 0x3099963 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099977; continue; }
  pc = 0x3099967; continue;
 }
 0x3099967 => {
  cpu.push(0x309996c); f_0309daa0(cpu); if cpu.returned_to != 0x309996c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099977; continue; }
  pc = 0x3099970; continue;
 }
 0x3099970 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099979; continue;
 }
 0x3099977 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099979; continue;
 }
 0x3099979 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30999ab; continue; }
  pc = 0x309997d; continue;
 }
 0x309997d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a580u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x30999a3); cpu.call(t); if cpu.returned_to != 0x30999a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30999ab); f_0309de70(cpu); if cpu.returned_to != 0x30999ab { return; }
  pc = 0x30999ab; continue;
 }
 0x30999ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x30999b8); f_030a8bf0(cpu); if cpu.returned_to != 0x30999b8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_030999e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30999e0); }
 let mut pc: u32 = 0x30999e0;
 loop { match pc {
 0x30999e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099a28; continue; }
  pc = 0x3099a01; continue;
 }
 0x3099a01 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099a0f; continue; }
  pc = 0x3099a06; continue;
 }
 0x3099a06 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099a28; continue; }
  pc = 0x3099a0f; continue;
 }
 0x3099a0f => {
  cpu.push(0x3099a14); f_030a4341(cpu); if cpu.returned_to != 0x3099a14 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099a28; continue; }
  pc = 0x3099a18; continue;
 }
 0x3099a18 => {
  cpu.push(0x3099a1d); f_0309daa0(cpu); if cpu.returned_to != 0x3099a1d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099a28; continue; }
  pc = 0x3099a21; continue;
 }
 0x3099a21 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099a2a; continue;
 }
 0x3099a28 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099a2a; continue;
 }
 0x3099a2a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099a5c; continue; }
  pc = 0x3099a2e; continue;
 }
 0x3099a2e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a558u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099a54); cpu.call(t); if cpu.returned_to != 0x3099a54 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099a5c); f_0309de70(cpu); if cpu.returned_to != 0x3099a5c { return; }
  pc = 0x3099a5c; continue;
 }
 0x3099a5c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099a69); f_030a8c10(cpu); if cpu.returned_to != 0x3099a69 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3099ab7; continue; }
  pc = 0x3099a78; continue;
 }
 0x3099a78 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3099a8a; continue; }
  pc = 0x3099a81; continue;
 }
 0x3099a81 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099ab7; continue; }
  pc = 0x3099a8a; continue;
 }
 0x3099a8a => {
  cpu.push(0x3099a8f); f_030a4341(cpu); if cpu.returned_to != 0x3099a8f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099ab7; continue; }
  pc = 0x3099a93; continue;
 }
 0x3099a93 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x3099aad); f_030a45d3(cpu); if cpu.returned_to != 0x3099aad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3099ab7); f_0309ddd0(cpu); if cpu.returned_to != 0x3099ab7 { return; }
  pc = 0x3099ab7; continue;
 }
 0x3099ab7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_03099ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099ae0); }
 let mut pc: u32 = 0x3099ae0;
 loop { match pc {
 0x3099ae0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099b27; continue; }
  pc = 0x3099b00; continue;
 }
 0x3099b00 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099b0e; continue; }
  pc = 0x3099b05; continue;
 }
 0x3099b05 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099b27; continue; }
  pc = 0x3099b0e; continue;
 }
 0x3099b0e => {
  cpu.push(0x3099b13); f_030a4341(cpu); if cpu.returned_to != 0x3099b13 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099b27; continue; }
  pc = 0x3099b17; continue;
 }
 0x3099b17 => {
  cpu.push(0x3099b1c); f_0309daa0(cpu); if cpu.returned_to != 0x3099b1c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099b27; continue; }
  pc = 0x3099b20; continue;
 }
 0x3099b20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099b29; continue;
 }
 0x3099b27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099b29; continue;
 }
 0x3099b29 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099b5b; continue; }
  pc = 0x3099b2d; continue;
 }
 0x3099b2d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a534u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099b53); cpu.call(t); if cpu.returned_to != 0x3099b53 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099b5b); f_0309de70(cpu); if cpu.returned_to != 0x3099b5b { return; }
  pc = 0x3099b5b; continue;
 }
 0x3099b5b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099b68); f_030a8e80(cpu); if cpu.returned_to != 0x3099b68 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03099b90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099b90); }
 let mut pc: u32 = 0x3099b90;
 loop { match pc {
 0x3099b90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099bd8; continue; }
  pc = 0x3099bb1; continue;
 }
 0x3099bb1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099bbf; continue; }
  pc = 0x3099bb6; continue;
 }
 0x3099bb6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099bd8; continue; }
  pc = 0x3099bbf; continue;
 }
 0x3099bbf => {
  cpu.push(0x3099bc4); f_030a4341(cpu); if cpu.returned_to != 0x3099bc4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099bd8; continue; }
  pc = 0x3099bc8; continue;
 }
 0x3099bc8 => {
  cpu.push(0x3099bcd); f_0309daa0(cpu); if cpu.returned_to != 0x3099bcd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099bd8; continue; }
  pc = 0x3099bd1; continue;
 }
 0x3099bd1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099bda; continue;
 }
 0x3099bd8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099bda; continue;
 }
 0x3099bda => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099c1a; continue; }
  pc = 0x3099bde; continue;
 }
 0x3099bde => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a50cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099c12); cpu.call(t); if cpu.returned_to != 0x3099c12 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099c1a); f_0309de70(cpu); if cpu.returned_to != 0x3099c1a { return; }
  pc = 0x3099c1a; continue;
 }
 0x3099c1a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x3099c2f); f_030a8f60(cpu); if cpu.returned_to != 0x3099c2f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099c7c; continue; }
  pc = 0x3099c3d; continue;
 }
 0x3099c3d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3099c4f; continue; }
  pc = 0x3099c46; continue;
 }
 0x3099c46 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099c7c; continue; }
  pc = 0x3099c4f; continue;
 }
 0x3099c4f => {
  cpu.push(0x3099c54); f_030a4341(cpu); if cpu.returned_to != 0x3099c54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099c7c; continue; }
  pc = 0x3099c58; continue;
 }
 0x3099c58 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3099c72); f_030a45d3(cpu); if cpu.returned_to != 0x3099c72 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3099c7c); f_0309ddd0(cpu); if cpu.returned_to != 0x3099c7c { return; }
  pc = 0x3099c7c; continue;
 }
 0x3099c7c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_03099ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099ca0); }
 let mut pc: u32 = 0x3099ca0;
 loop { match pc {
 0x3099ca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099ce8; continue; }
  pc = 0x3099cc1; continue;
 }
 0x3099cc1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099ccf; continue; }
  pc = 0x3099cc6; continue;
 }
 0x3099cc6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099ce8; continue; }
  pc = 0x3099ccf; continue;
 }
 0x3099ccf => {
  cpu.push(0x3099cd4); f_030a4341(cpu); if cpu.returned_to != 0x3099cd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099ce8; continue; }
  pc = 0x3099cd8; continue;
 }
 0x3099cd8 => {
  cpu.push(0x3099cdd); f_0309daa0(cpu); if cpu.returned_to != 0x3099cdd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099ce8; continue; }
  pc = 0x3099ce1; continue;
 }
 0x3099ce1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099cea; continue;
 }
 0x3099ce8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099cea; continue;
 }
 0x3099cea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099d1c; continue; }
  pc = 0x3099cee; continue;
 }
 0x3099cee => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a4f0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099d14); cpu.call(t); if cpu.returned_to != 0x3099d14 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099d1c); f_0309de70(cpu); if cpu.returned_to != 0x3099d1c { return; }
  pc = 0x3099d1c; continue;
 }
 0x3099d1c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099d29); f_030a8ea0(cpu); if cpu.returned_to != 0x3099d29 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3099d77; continue; }
  pc = 0x3099d38; continue;
 }
 0x3099d38 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3099d4a; continue; }
  pc = 0x3099d41; continue;
 }
 0x3099d41 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099d77; continue; }
  pc = 0x3099d4a; continue;
 }
 0x3099d4a => {
  cpu.push(0x3099d4f); f_030a4341(cpu); if cpu.returned_to != 0x3099d4f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099d77; continue; }
  pc = 0x3099d53; continue;
 }
 0x3099d53 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x3099d6d); f_030a45d3(cpu); if cpu.returned_to != 0x3099d6d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3099d77); f_0309de20(cpu); if cpu.returned_to != 0x3099d77 { return; }
  pc = 0x3099d77; continue;
 }
 0x3099d77 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03099da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099da0); }
 let mut pc: u32 = 0x3099da0;
 loop { match pc {
 0x3099da0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099de8; continue; }
  pc = 0x3099dc1; continue;
 }
 0x3099dc1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099dcf; continue; }
  pc = 0x3099dc6; continue;
 }
 0x3099dc6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099de8; continue; }
  pc = 0x3099dcf; continue;
 }
 0x3099dcf => {
  cpu.push(0x3099dd4); f_030a4341(cpu); if cpu.returned_to != 0x3099dd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099de8; continue; }
  pc = 0x3099dd8; continue;
 }
 0x3099dd8 => {
  cpu.push(0x3099ddd); f_0309daa0(cpu); if cpu.returned_to != 0x3099ddd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099de8; continue; }
  pc = 0x3099de1; continue;
 }
 0x3099de1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099dea; continue;
 }
 0x3099de8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099dea; continue;
 }
 0x3099dea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099e1c; continue; }
  pc = 0x3099dee; continue;
 }
 0x3099dee => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a4d0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099e14); cpu.call(t); if cpu.returned_to != 0x3099e14 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099e1c); f_0309de70(cpu); if cpu.returned_to != 0x3099e1c { return; }
  pc = 0x3099e1c; continue;
 }
 0x3099e1c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x3099e29); f_030a5c60(cpu); if cpu.returned_to != 0x3099e29 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x3099e77; continue; }
  pc = 0x3099e38; continue;
 }
 0x3099e38 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3099e4a; continue; }
  pc = 0x3099e41; continue;
 }
 0x3099e41 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099e77; continue; }
  pc = 0x3099e4a; continue;
 }
 0x3099e4a => {
  cpu.push(0x3099e4f); f_030a4341(cpu); if cpu.returned_to != 0x3099e4f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099e77; continue; }
  pc = 0x3099e53; continue;
 }
 0x3099e53 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x3099e6d); f_030a45d3(cpu); if cpu.returned_to != 0x3099e6d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3099e77); f_0309de20(cpu); if cpu.returned_to != 0x3099e77 { return; }
  pc = 0x3099e77; continue;
 }
 0x3099e77 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03099ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099ea0); }
 let mut pc: u32 = 0x3099ea0;
 loop { match pc {
 0x3099ea0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099ee8; continue; }
  pc = 0x3099ec1; continue;
 }
 0x3099ec1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099ecf; continue; }
  pc = 0x3099ec6; continue;
 }
 0x3099ec6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099ee8; continue; }
  pc = 0x3099ecf; continue;
 }
 0x3099ecf => {
  cpu.push(0x3099ed4); f_030a4341(cpu); if cpu.returned_to != 0x3099ed4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099ee8; continue; }
  pc = 0x3099ed8; continue;
 }
 0x3099ed8 => {
  cpu.push(0x3099edd); f_0309daa0(cpu); if cpu.returned_to != 0x3099edd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099ee8; continue; }
  pc = 0x3099ee1; continue;
 }
 0x3099ee1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099eea; continue;
 }
 0x3099ee8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099eea; continue;
 }
 0x3099eea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099f2a; continue; }
  pc = 0x3099eee; continue;
 }
 0x3099eee => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a4b0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x3099f22); cpu.call(t); if cpu.returned_to != 0x3099f22 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x3099f2a); f_0309de70(cpu); if cpu.returned_to != 0x3099f2a { return; }
  pc = 0x3099f2a; continue;
 }
 0x3099f2a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x3099f3f); f_030a5760(cpu); if cpu.returned_to != 0x3099f3f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099f8c; continue; }
  pc = 0x3099f4d; continue;
 }
 0x3099f4d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x3099f5f; continue; }
  pc = 0x3099f56; continue;
 }
 0x3099f56 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099f8c; continue; }
  pc = 0x3099f5f; continue;
 }
 0x3099f5f => {
  cpu.push(0x3099f64); f_030a4341(cpu); if cpu.returned_to != 0x3099f64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099f8c; continue; }
  pc = 0x3099f68; continue;
 }
 0x3099f68 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x3099f82); f_030a45d3(cpu); if cpu.returned_to != 0x3099f82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x3099f8c); f_0309de20(cpu); if cpu.returned_to != 0x3099f8c { return; }
  pc = 0x3099f8c; continue;
 }
 0x3099f8c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_03099fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3099fb0); }
 let mut pc: u32 = 0x3099fb0;
 loop { match pc {
 0x3099fb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x3099ff7; continue; }
  pc = 0x3099fd0; continue;
 }
 0x3099fd0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x3099fde; continue; }
  pc = 0x3099fd5; continue;
 }
 0x3099fd5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x3099ff7; continue; }
  pc = 0x3099fde; continue;
 }
 0x3099fde => {
  cpu.push(0x3099fe3); f_030a4341(cpu); if cpu.returned_to != 0x3099fe3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x3099ff7; continue; }
  pc = 0x3099fe7; continue;
 }
 0x3099fe7 => {
  cpu.push(0x3099fec); f_0309daa0(cpu); if cpu.returned_to != 0x3099fec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3099ff7; continue; }
  pc = 0x3099ff0; continue;
 }
 0x3099ff0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x3099ff9; continue;
 }
 0x3099ff7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x3099ff9; continue;
 }
 0x3099ff9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309a047; continue; }
  pc = 0x3099ffd; continue;
 }
 0x3099ffd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a488u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309a03f); cpu.call(t); if cpu.returned_to != 0x309a03f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309a047); f_0309de70(cpu); if cpu.returned_to != 0x309a047 { return; }
  pc = 0x309a047; continue;
 }
 0x309a047 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309a064); f_030a5780(cpu); if cpu.returned_to != 0x309a064 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309a190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309a190); }
 let mut pc: u32 = 0x309a190;
 loop { match pc {
 0x309a190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309a1d8; continue; }
  pc = 0x309a1b1; continue;
 }
 0x309a1b1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309a1bf; continue; }
  pc = 0x309a1b6; continue;
 }
 0x309a1b6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309a1d8; continue; }
  pc = 0x309a1bf; continue;
 }
 0x309a1bf => {
  cpu.push(0x309a1c4); f_030a4341(cpu); if cpu.returned_to != 0x309a1c4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309a1d8; continue; }
  pc = 0x309a1c8; continue;
 }
 0x309a1c8 => {
  cpu.push(0x309a1cd); f_0309daa0(cpu); if cpu.returned_to != 0x309a1cd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309a1d8; continue; }
  pc = 0x309a1d1; continue;
 }
 0x309a1d1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309a1da; continue;
 }
 0x309a1d8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309a1da; continue;
 }
 0x309a1da => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309a21a; continue; }
  pc = 0x309a1de; continue;
 }
 0x309a1de => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a438u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309a212); cpu.call(t); if cpu.returned_to != 0x309a212 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309a21a); f_0309de70(cpu); if cpu.returned_to != 0x309a21a { return; }
  pc = 0x309a21a; continue;
 }
 0x309a21a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309a22f); f_030a57a0(cpu); if cpu.returned_to != 0x309a22f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309a27c; continue; }
  pc = 0x309a23d; continue;
 }
 0x309a23d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309a24f; continue; }
  pc = 0x309a246; continue;
 }
 0x309a246 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309a27c; continue; }
  pc = 0x309a24f; continue;
 }
 0x309a24f => {
  cpu.push(0x309a254); f_030a4341(cpu); if cpu.returned_to != 0x309a254 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309a27c; continue; }
  pc = 0x309a258; continue;
 }
 0x309a258 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309a272); f_030a45d3(cpu); if cpu.returned_to != 0x309a272 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309a27c); f_0309ddd0(cpu); if cpu.returned_to != 0x309a27c { return; }
  pc = 0x309a27c; continue;
 }
 0x309a27c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_0309ae50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ae50); }
 let mut pc: u32 = 0x309ae50;
 loop { match pc {
 0x309ae50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309ae97; continue; }
  pc = 0x309ae70; continue;
 }
 0x309ae70 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309ae7e; continue; }
  pc = 0x309ae75; continue;
 }
 0x309ae75 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309ae97; continue; }
  pc = 0x309ae7e; continue;
 }
 0x309ae7e => {
  cpu.push(0x309ae83); f_030a4341(cpu); if cpu.returned_to != 0x309ae83 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309ae97; continue; }
  pc = 0x309ae87; continue;
 }
 0x309ae87 => {
  cpu.push(0x309ae8c); f_0309daa0(cpu); if cpu.returned_to != 0x309ae8c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ae97; continue; }
  pc = 0x309ae90; continue;
 }
 0x309ae90 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309ae99; continue;
 }
 0x309ae97 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309ae99; continue;
 }
 0x309ae99 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309aed9; continue; }
  pc = 0x309ae9d; continue;
 }
 0x309ae9d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a228u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309aed1); cpu.call(t); if cpu.returned_to != 0x309aed1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309aed9); f_0309de70(cpu); if cpu.returned_to != 0x309aed9 { return; }
  pc = 0x309aed9; continue;
 }
 0x309aed9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309aeee); f_030a5720(cpu); if cpu.returned_to != 0x309aeee { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309af10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309af10); }
 let mut pc: u32 = 0x309af10;
 loop { match pc {
 0x309af10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309af58; continue; }
  pc = 0x309af31; continue;
 }
 0x309af31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309af3f; continue; }
  pc = 0x309af36; continue;
 }
 0x309af36 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309af58; continue; }
  pc = 0x309af3f; continue;
 }
 0x309af3f => {
  cpu.push(0x309af44); f_030a4341(cpu); if cpu.returned_to != 0x309af44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309af58; continue; }
  pc = 0x309af48; continue;
 }
 0x309af48 => {
  cpu.push(0x309af4d); f_0309daa0(cpu); if cpu.returned_to != 0x309af4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309af58; continue; }
  pc = 0x309af51; continue;
 }
 0x309af51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309af5a; continue;
 }
 0x309af58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309af5a; continue;
 }
 0x309af5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309af8c; continue; }
  pc = 0x309af5e; continue;
 }
 0x309af5e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a208u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309af84); cpu.call(t); if cpu.returned_to != 0x309af84 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309af8c); f_0309de70(cpu); if cpu.returned_to != 0x309af8c { return; }
  pc = 0x309af8c; continue;
 }
 0x309af8c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309af99); f_030a5990(cpu); if cpu.returned_to != 0x309af99 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309afe7; continue; }
  pc = 0x309afa8; continue;
 }
 0x309afa8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309afba; continue; }
  pc = 0x309afb1; continue;
 }
 0x309afb1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309afe7; continue; }
  pc = 0x309afba; continue;
 }
 0x309afba => {
  cpu.push(0x309afbf); f_030a4341(cpu); if cpu.returned_to != 0x309afbf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309afe7; continue; }
  pc = 0x309afc3; continue;
 }
 0x309afc3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309afdd); f_030a45d3(cpu); if cpu.returned_to != 0x309afdd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309afe7); f_0309de20(cpu); if cpu.returned_to != 0x309afe7 { return; }
  pc = 0x309afe7; continue;
 }
 0x309afe7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_0309b310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b310); }
 let mut pc: u32 = 0x309b310;
 loop { match pc {
 0x309b310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b358; continue; }
  pc = 0x309b331; continue;
 }
 0x309b331 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b33f; continue; }
  pc = 0x309b336; continue;
 }
 0x309b336 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b358; continue; }
  pc = 0x309b33f; continue;
 }
 0x309b33f => {
  cpu.push(0x309b344); f_030a4341(cpu); if cpu.returned_to != 0x309b344 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b358; continue; }
  pc = 0x309b348; continue;
 }
 0x309b348 => {
  cpu.push(0x309b34d); f_0309daa0(cpu); if cpu.returned_to != 0x309b34d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b358; continue; }
  pc = 0x309b351; continue;
 }
 0x309b351 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b35a; continue;
 }
 0x309b358 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b35a; continue;
 }
 0x309b35a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b38c; continue; }
  pc = 0x309b35e; continue;
 }
 0x309b35e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a194u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b384); cpu.call(t); if cpu.returned_to != 0x309b384 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b38c); f_0309de70(cpu); if cpu.returned_to != 0x309b38c { return; }
  pc = 0x309b38c; continue;
 }
 0x309b38c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309b399); f_030a5ad0(cpu); if cpu.returned_to != 0x309b399 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309b3e7; continue; }
  pc = 0x309b3a8; continue;
 }
 0x309b3a8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309b3ba; continue; }
  pc = 0x309b3b1; continue;
 }
 0x309b3b1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b3e7; continue; }
  pc = 0x309b3ba; continue;
 }
 0x309b3ba => {
  cpu.push(0x309b3bf); f_030a4341(cpu); if cpu.returned_to != 0x309b3bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b3e7; continue; }
  pc = 0x309b3c3; continue;
 }
 0x309b3c3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309b3dd); f_030a45d3(cpu); if cpu.returned_to != 0x309b3dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309b3e7); f_0309de20(cpu); if cpu.returned_to != 0x309b3e7 { return; }
  pc = 0x309b3e7; continue;
 }
 0x309b3e7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309b410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b410); }
 let mut pc: u32 = 0x309b410;
 loop { match pc {
 0x309b410 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b457; continue; }
  pc = 0x309b430; continue;
 }
 0x309b430 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b43e; continue; }
  pc = 0x309b435; continue;
 }
 0x309b435 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b457; continue; }
  pc = 0x309b43e; continue;
 }
 0x309b43e => {
  cpu.push(0x309b443); f_030a4341(cpu); if cpu.returned_to != 0x309b443 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b457; continue; }
  pc = 0x309b447; continue;
 }
 0x309b447 => {
  cpu.push(0x309b44c); f_0309daa0(cpu); if cpu.returned_to != 0x309b44c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b457; continue; }
  pc = 0x309b450; continue;
 }
 0x309b450 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b459; continue;
 }
 0x309b457 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b459; continue;
 }
 0x309b459 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b4a7; continue; }
  pc = 0x309b45d; continue;
 }
 0x309b45d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a168u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b49f); cpu.call(t); if cpu.returned_to != 0x309b49f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b4a7); f_0309de70(cpu); if cpu.returned_to != 0x309b4a7 { return; }
  pc = 0x309b4a7; continue;
 }
 0x309b4a7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b4c4); f_030a5960(cpu); if cpu.returned_to != 0x309b4c4 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309b4e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b4e0); }
 let mut pc: u32 = 0x309b4e0;
 loop { match pc {
 0x309b4e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b527; continue; }
  pc = 0x309b500; continue;
 }
 0x309b500 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b50e; continue; }
  pc = 0x309b505; continue;
 }
 0x309b505 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b527; continue; }
  pc = 0x309b50e; continue;
 }
 0x309b50e => {
  cpu.push(0x309b513); f_030a4341(cpu); if cpu.returned_to != 0x309b513 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b527; continue; }
  pc = 0x309b517; continue;
 }
 0x309b517 => {
  cpu.push(0x309b51c); f_0309daa0(cpu); if cpu.returned_to != 0x309b51c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b527; continue; }
  pc = 0x309b520; continue;
 }
 0x309b520 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b529; continue;
 }
 0x309b527 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b529; continue;
 }
 0x309b529 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b569; continue; }
  pc = 0x309b52d; continue;
 }
 0x309b52d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a140u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b561); cpu.call(t); if cpu.returned_to != 0x309b561 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b569); f_0309de70(cpu); if cpu.returned_to != 0x309b569 { return; }
  pc = 0x309b569; continue;
 }
 0x309b569 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309b57e); f_030a5980(cpu); if cpu.returned_to != 0x309b57e { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309b5a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b5a0); }
 let mut pc: u32 = 0x309b5a0;
 loop { match pc {
 0x309b5a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b5e7; continue; }
  pc = 0x309b5c0; continue;
 }
 0x309b5c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b5ce; continue; }
  pc = 0x309b5c5; continue;
 }
 0x309b5c5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b5e7; continue; }
  pc = 0x309b5ce; continue;
 }
 0x309b5ce => {
  cpu.push(0x309b5d3); f_030a4341(cpu); if cpu.returned_to != 0x309b5d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b5e7; continue; }
  pc = 0x309b5d7; continue;
 }
 0x309b5d7 => {
  cpu.push(0x309b5dc); f_0309daa0(cpu); if cpu.returned_to != 0x309b5dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b5e7; continue; }
  pc = 0x309b5e0; continue;
 }
 0x309b5e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b5e9; continue;
 }
 0x309b5e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b5e9; continue;
 }
 0x309b5e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b629; continue; }
  pc = 0x309b5ed; continue;
 }
 0x309b5ed => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a120u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b621); cpu.call(t); if cpu.returned_to != 0x309b621 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b629); f_0309de70(cpu); if cpu.returned_to != 0x309b629 { return; }
  pc = 0x309b629; continue;
 }
 0x309b629 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309b63e); f_030a59f0(cpu); if cpu.returned_to != 0x309b63e { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309b660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b660); }
 let mut pc: u32 = 0x309b660;
 loop { match pc {
 0x309b660 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b6a7; continue; }
  pc = 0x309b680; continue;
 }
 0x309b680 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b68e; continue; }
  pc = 0x309b685; continue;
 }
 0x309b685 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b6a7; continue; }
  pc = 0x309b68e; continue;
 }
 0x309b68e => {
  cpu.push(0x309b693); f_030a4341(cpu); if cpu.returned_to != 0x309b693 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b6a7; continue; }
  pc = 0x309b697; continue;
 }
 0x309b697 => {
  cpu.push(0x309b69c); f_0309daa0(cpu); if cpu.returned_to != 0x309b69c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b6a7; continue; }
  pc = 0x309b6a0; continue;
 }
 0x309b6a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b6a9; continue;
 }
 0x309b6a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b6a9; continue;
 }
 0x309b6a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b6e9; continue; }
  pc = 0x309b6ad; continue;
 }
 0x309b6ad => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a0fcu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b6e1); cpu.call(t); if cpu.returned_to != 0x309b6e1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b6e9); f_0309de70(cpu); if cpu.returned_to != 0x309b6e9 { return; }
  pc = 0x309b6e9; continue;
 }
 0x309b6e9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309b6fe); f_030a5a20(cpu); if cpu.returned_to != 0x309b6fe { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309b720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b720); }
 let mut pc: u32 = 0x309b720;
 loop { match pc {
 0x309b720 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b767; continue; }
  pc = 0x309b740; continue;
 }
 0x309b740 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b74e; continue; }
  pc = 0x309b745; continue;
 }
 0x309b745 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b767; continue; }
  pc = 0x309b74e; continue;
 }
 0x309b74e => {
  cpu.push(0x309b753); f_030a4341(cpu); if cpu.returned_to != 0x309b753 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b767; continue; }
  pc = 0x309b757; continue;
 }
 0x309b757 => {
  cpu.push(0x309b75c); f_0309daa0(cpu); if cpu.returned_to != 0x309b75c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b767; continue; }
  pc = 0x309b760; continue;
 }
 0x309b760 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b769; continue;
 }
 0x309b767 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b769; continue;
 }
 0x309b769 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b7a9; continue; }
  pc = 0x309b76d; continue;
 }
 0x309b76d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a0d4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b7a1); cpu.call(t); if cpu.returned_to != 0x309b7a1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b7a9); f_0309de70(cpu); if cpu.returned_to != 0x309b7a9 { return; }
  pc = 0x309b7a9; continue;
 }
 0x309b7a9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309b7be); f_030a5a50(cpu); if cpu.returned_to != 0x309b7be { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309b7e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b7e0); }
 let mut pc: u32 = 0x309b7e0;
 loop { match pc {
 0x309b7e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b827; continue; }
  pc = 0x309b800; continue;
 }
 0x309b800 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b80e; continue; }
  pc = 0x309b805; continue;
 }
 0x309b805 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b827; continue; }
  pc = 0x309b80e; continue;
 }
 0x309b80e => {
  cpu.push(0x309b813); f_030a4341(cpu); if cpu.returned_to != 0x309b813 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b827; continue; }
  pc = 0x309b817; continue;
 }
 0x309b817 => {
  cpu.push(0x309b81c); f_0309daa0(cpu); if cpu.returned_to != 0x309b81c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b827; continue; }
  pc = 0x309b820; continue;
 }
 0x309b820 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b829; continue;
 }
 0x309b827 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b829; continue;
 }
 0x309b829 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b85b; continue; }
  pc = 0x309b82d; continue;
 }
 0x309b82d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a0bcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b853); cpu.call(t); if cpu.returned_to != 0x309b853 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b85b); f_0309de70(cpu); if cpu.returned_to != 0x309b85b { return; }
  pc = 0x309b85b; continue;
 }
 0x309b85b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309b868); f_030a5800(cpu); if cpu.returned_to != 0x309b868 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309b890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b890); }
 let mut pc: u32 = 0x309b890;
 loop { match pc {
 0x309b890 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309b8d7; continue; }
  pc = 0x309b8b0; continue;
 }
 0x309b8b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309b8be; continue; }
  pc = 0x309b8b5; continue;
 }
 0x309b8b5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309b8d7; continue; }
  pc = 0x309b8be; continue;
 }
 0x309b8be => {
  cpu.push(0x309b8c3); f_030a4341(cpu); if cpu.returned_to != 0x309b8c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309b8d7; continue; }
  pc = 0x309b8c7; continue;
 }
 0x309b8c7 => {
  cpu.push(0x309b8cc); f_0309daa0(cpu); if cpu.returned_to != 0x309b8cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b8d7; continue; }
  pc = 0x309b8d0; continue;
 }
 0x309b8d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309b8d9; continue;
 }
 0x309b8d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309b8d9; continue;
 }
 0x309b8d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309b90b; continue; }
  pc = 0x309b8dd; continue;
 }
 0x309b8dd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a0a0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309b903); cpu.call(t); if cpu.returned_to != 0x309b903 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309b90b); f_0309de70(cpu); if cpu.returned_to != 0x309b90b { return; }
  pc = 0x309b90b; continue;
 }
 0x309b90b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309b918); f_030a5850(cpu); if cpu.returned_to != 0x309b918 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309b9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309b9f0); }
 let mut pc: u32 = 0x309b9f0;
 loop { match pc {
 0x309b9f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309ba37; continue; }
  pc = 0x309ba10; continue;
 }
 0x309ba10 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309ba1e; continue; }
  pc = 0x309ba15; continue;
 }
 0x309ba15 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309ba37; continue; }
  pc = 0x309ba1e; continue;
 }
 0x309ba1e => {
  cpu.push(0x309ba23); f_030a4341(cpu); if cpu.returned_to != 0x309ba23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309ba37; continue; }
  pc = 0x309ba27; continue;
 }
 0x309ba27 => {
  cpu.push(0x309ba2c); f_0309daa0(cpu); if cpu.returned_to != 0x309ba2c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ba37; continue; }
  pc = 0x309ba30; continue;
 }
 0x309ba30 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309ba39; continue;
 }
 0x309ba37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309ba39; continue;
 }
 0x309ba39 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ba6b; continue; }
  pc = 0x309ba3d; continue;
 }
 0x309ba3d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a06cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309ba63); cpu.call(t); if cpu.returned_to != 0x309ba63 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309ba6b); f_0309de70(cpu); if cpu.returned_to != 0x309ba6b { return; }
  pc = 0x309ba6b; continue;
 }
 0x309ba6b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309ba78); f_030a58d0(cpu); if cpu.returned_to != 0x309ba78 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309baa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309baa0); }
 let mut pc: u32 = 0x309baa0;
 loop { match pc {
 0x309baa0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309bae7; continue; }
  pc = 0x309bac0; continue;
 }
 0x309bac0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309bace; continue; }
  pc = 0x309bac5; continue;
 }
 0x309bac5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bae7; continue; }
  pc = 0x309bace; continue;
 }
 0x309bace => {
  cpu.push(0x309bad3); f_030a4341(cpu); if cpu.returned_to != 0x309bad3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bae7; continue; }
  pc = 0x309bad7; continue;
 }
 0x309bad7 => {
  cpu.push(0x309badc); f_0309daa0(cpu); if cpu.returned_to != 0x309badc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bae7; continue; }
  pc = 0x309bae0; continue;
 }
 0x309bae0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309bae9; continue;
 }
 0x309bae7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309bae9; continue;
 }
 0x309bae9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bb37; continue; }
  pc = 0x309baed; continue;
 }
 0x309baed => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a048u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309bb2f); cpu.call(t); if cpu.returned_to != 0x309bb2f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bb37); f_0309de70(cpu); if cpu.returned_to != 0x309bb37 { return; }
  pc = 0x309bb37; continue;
 }
 0x309bb37 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bb54); f_030a5a70(cpu); if cpu.returned_to != 0x309bb54 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309bb70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309bb70); }
 let mut pc: u32 = 0x309bb70;
 loop { match pc {
 0x309bb70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309bbb7; continue; }
  pc = 0x309bb90; continue;
 }
 0x309bb90 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309bb9e; continue; }
  pc = 0x309bb95; continue;
 }
 0x309bb95 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bbb7; continue; }
  pc = 0x309bb9e; continue;
 }
 0x309bb9e => {
  cpu.push(0x309bba3); f_030a4341(cpu); if cpu.returned_to != 0x309bba3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bbb7; continue; }
  pc = 0x309bba7; continue;
 }
 0x309bba7 => {
  cpu.push(0x309bbac); f_0309daa0(cpu); if cpu.returned_to != 0x309bbac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bbb7; continue; }
  pc = 0x309bbb0; continue;
 }
 0x309bbb0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309bbb9; continue;
 }
 0x309bbb7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309bbb9; continue;
 }
 0x309bbb9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bc07; continue; }
  pc = 0x309bbbd; continue;
 }
 0x309bbbd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x314a020u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309bbff); cpu.call(t); if cpu.returned_to != 0x309bbff { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bc07); f_0309de70(cpu); if cpu.returned_to != 0x309bc07 { return; }
  pc = 0x309bc07; continue;
 }
 0x309bc07 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bc24); f_030a5aa0(cpu); if cpu.returned_to != 0x309bc24 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309bc40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309bc40); }
 let mut pc: u32 = 0x309bc40;
 loop { match pc {
 0x309bc40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309bc87; continue; }
  pc = 0x309bc60; continue;
 }
 0x309bc60 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309bc6e; continue; }
  pc = 0x309bc65; continue;
 }
 0x309bc65 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bc87; continue; }
  pc = 0x309bc6e; continue;
 }
 0x309bc6e => {
  cpu.push(0x309bc73); f_030a4341(cpu); if cpu.returned_to != 0x309bc73 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bc87; continue; }
  pc = 0x309bc77; continue;
 }
 0x309bc77 => {
  cpu.push(0x309bc7c); f_0309daa0(cpu); if cpu.returned_to != 0x309bc7c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bc87; continue; }
  pc = 0x309bc80; continue;
 }
 0x309bc80 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309bc89; continue;
 }
 0x309bc87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309bc89; continue;
 }
 0x309bc89 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bcd7; continue; }
  pc = 0x309bc8d; continue;
 }
 0x309bc8d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149ffcu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309bccf); cpu.call(t); if cpu.returned_to != 0x309bccf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bcd7); f_0309de70(cpu); if cpu.returned_to != 0x309bcd7 { return; }
  pc = 0x309bcd7; continue;
 }
 0x309bcd7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bcf4); f_030a7290(cpu); if cpu.returned_to != 0x309bcf4 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309bd42; continue; }
  pc = 0x309bd03; continue;
 }
 0x309bd03 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309bd15; continue; }
  pc = 0x309bd0c; continue;
 }
 0x309bd0c => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bd42; continue; }
  pc = 0x309bd15; continue;
 }
 0x309bd15 => {
  cpu.push(0x309bd1a); f_030a4341(cpu); if cpu.returned_to != 0x309bd1a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bd42; continue; }
  pc = 0x309bd1e; continue;
 }
 0x309bd1e => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309bd38); f_030a45d3(cpu); if cpu.returned_to != 0x309bd38 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309bd42); f_0309de20(cpu); if cpu.returned_to != 0x309bd42 { return; }
  pc = 0x309bd42; continue;
 }
 0x309bd42 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309bd60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309bd60); }
 let mut pc: u32 = 0x309bd60;
 loop { match pc {
 0x309bd60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309bda7; continue; }
  pc = 0x309bd80; continue;
 }
 0x309bd80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309bd8e; continue; }
  pc = 0x309bd85; continue;
 }
 0x309bd85 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bda7; continue; }
  pc = 0x309bd8e; continue;
 }
 0x309bd8e => {
  cpu.push(0x309bd93); f_030a4341(cpu); if cpu.returned_to != 0x309bd93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bda7; continue; }
  pc = 0x309bd97; continue;
 }
 0x309bd97 => {
  cpu.push(0x309bd9c); f_0309daa0(cpu); if cpu.returned_to != 0x309bd9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bda7; continue; }
  pc = 0x309bda0; continue;
 }
 0x309bda0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309bda9; continue;
 }
 0x309bda7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309bda9; continue;
 }
 0x309bda9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bddb; continue; }
  pc = 0x309bdad; continue;
 }
 0x309bdad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149fe4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309bdd3); cpu.call(t); if cpu.returned_to != 0x309bdd3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bddb); f_0309de70(cpu); if cpu.returned_to != 0x309bddb { return; }
  pc = 0x309bddb; continue;
 }
 0x309bddb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309bde8); f_030a5ae0(cpu); if cpu.returned_to != 0x309bde8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309be10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309be10); }
 let mut pc: u32 = 0x309be10;
 loop { match pc {
 0x309be10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309be57; continue; }
  pc = 0x309be30; continue;
 }
 0x309be30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309be3e; continue; }
  pc = 0x309be35; continue;
 }
 0x309be35 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309be57; continue; }
  pc = 0x309be3e; continue;
 }
 0x309be3e => {
  cpu.push(0x309be43); f_030a4341(cpu); if cpu.returned_to != 0x309be43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309be57; continue; }
  pc = 0x309be47; continue;
 }
 0x309be47 => {
  cpu.push(0x309be4c); f_0309daa0(cpu); if cpu.returned_to != 0x309be4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309be57; continue; }
  pc = 0x309be50; continue;
 }
 0x309be50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309be59; continue;
 }
 0x309be57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309be59; continue;
 }
 0x309be59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309be8b; continue; }
  pc = 0x309be5d; continue;
 }
 0x309be5d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149fc0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309be83); cpu.call(t); if cpu.returned_to != 0x309be83 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309be8b); f_0309de70(cpu); if cpu.returned_to != 0x309be8b { return; }
  pc = 0x309be8b; continue;
 }
 0x309be8b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309be98); f_030a5bd0(cpu); if cpu.returned_to != 0x309be98 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309bec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309bec0); }
 let mut pc: u32 = 0x309bec0;
 loop { match pc {
 0x309bec0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309bf07; continue; }
  pc = 0x309bee0; continue;
 }
 0x309bee0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309beee; continue; }
  pc = 0x309bee5; continue;
 }
 0x309bee5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bf07; continue; }
  pc = 0x309beee; continue;
 }
 0x309beee => {
  cpu.push(0x309bef3); f_030a4341(cpu); if cpu.returned_to != 0x309bef3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bf07; continue; }
  pc = 0x309bef7; continue;
 }
 0x309bef7 => {
  cpu.push(0x309befc); f_0309daa0(cpu); if cpu.returned_to != 0x309befc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bf07; continue; }
  pc = 0x309bf00; continue;
 }
 0x309bf00 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309bf09; continue;
 }
 0x309bf07 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309bf09; continue;
 }
 0x309bf09 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309bf57; continue; }
  pc = 0x309bf0d; continue;
 }
 0x309bf0d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149f94u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309bf4f); cpu.call(t); if cpu.returned_to != 0x309bf4f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bf57); f_0309de70(cpu); if cpu.returned_to != 0x309bf57 { return; }
  pc = 0x309bf57; continue;
 }
 0x309bf57 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309bf74); f_030a73e0(cpu); if cpu.returned_to != 0x309bf74 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309bfc2; continue; }
  pc = 0x309bf83; continue;
 }
 0x309bf83 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309bf95; continue; }
  pc = 0x309bf8c; continue;
 }
 0x309bf8c => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309bfc2; continue; }
  pc = 0x309bf95; continue;
 }
 0x309bf95 => {
  cpu.push(0x309bf9a); f_030a4341(cpu); if cpu.returned_to != 0x309bf9a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309bfc2; continue; }
  pc = 0x309bf9e; continue;
 }
 0x309bf9e => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309bfb8); f_030a45d3(cpu); if cpu.returned_to != 0x309bfb8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309bfc2); f_0309ddd0(cpu); if cpu.returned_to != 0x309bfc2 { return; }
  pc = 0x309bfc2; continue;
 }
 0x309bfc2 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0309bfe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309bfe0); }
 let mut pc: u32 = 0x309bfe0;
 loop { match pc {
 0x309bfe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c028; continue; }
  pc = 0x309c001; continue;
 }
 0x309c001 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c00f; continue; }
  pc = 0x309c006; continue;
 }
 0x309c006 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c028; continue; }
  pc = 0x309c00f; continue;
 }
 0x309c00f => {
  cpu.push(0x309c014); f_030a4341(cpu); if cpu.returned_to != 0x309c014 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c028; continue; }
  pc = 0x309c018; continue;
 }
 0x309c018 => {
  cpu.push(0x309c01d); f_0309daa0(cpu); if cpu.returned_to != 0x309c01d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c028; continue; }
  pc = 0x309c021; continue;
 }
 0x309c021 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c02a; continue;
 }
 0x309c028 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c02a; continue;
 }
 0x309c02a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c05c; continue; }
  pc = 0x309c02e; continue;
 }
 0x309c02e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149f70u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c054); cpu.call(t); if cpu.returned_to != 0x309c054 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c05c); f_0309de70(cpu); if cpu.returned_to != 0x309c05c { return; }
  pc = 0x309c05c; continue;
 }
 0x309c05c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c069); f_030a5be0(cpu); if cpu.returned_to != 0x309c069 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309c0b7; continue; }
  pc = 0x309c078; continue;
 }
 0x309c078 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309c08a; continue; }
  pc = 0x309c081; continue;
 }
 0x309c081 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c0b7; continue; }
  pc = 0x309c08a; continue;
 }
 0x309c08a => {
  cpu.push(0x309c08f); f_030a4341(cpu); if cpu.returned_to != 0x309c08f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c0b7; continue; }
  pc = 0x309c093; continue;
 }
 0x309c093 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309c0ad); f_030a45d3(cpu); if cpu.returned_to != 0x309c0ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309c0b7); f_0309ddd0(cpu); if cpu.returned_to != 0x309c0b7 { return; }
  pc = 0x309c0b7; continue;
 }
 0x309c0b7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_0309c0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c0e0); }
 let mut pc: u32 = 0x309c0e0;
 loop { match pc {
 0x309c0e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c127; continue; }
  pc = 0x309c100; continue;
 }
 0x309c100 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c10e; continue; }
  pc = 0x309c105; continue;
 }
 0x309c105 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c127; continue; }
  pc = 0x309c10e; continue;
 }
 0x309c10e => {
  cpu.push(0x309c113); f_030a4341(cpu); if cpu.returned_to != 0x309c113 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c127; continue; }
  pc = 0x309c117; continue;
 }
 0x309c117 => {
  cpu.push(0x309c11c); f_0309daa0(cpu); if cpu.returned_to != 0x309c11c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c127; continue; }
  pc = 0x309c120; continue;
 }
 0x309c120 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c129; continue;
 }
 0x309c127 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c129; continue;
 }
 0x309c129 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c15b; continue; }
  pc = 0x309c12d; continue;
 }
 0x309c12d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149f4cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c153); cpu.call(t); if cpu.returned_to != 0x309c153 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c15b); f_0309de70(cpu); if cpu.returned_to != 0x309c15b { return; }
  pc = 0x309c15b; continue;
 }
 0x309c15b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c168); f_030a5c90(cpu); if cpu.returned_to != 0x309c168 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c190); }
 let mut pc: u32 = 0x309c190;
 loop { match pc {
 0x309c190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c1d8; continue; }
  pc = 0x309c1b1; continue;
 }
 0x309c1b1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c1bf; continue; }
  pc = 0x309c1b6; continue;
 }
 0x309c1b6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c1d8; continue; }
  pc = 0x309c1bf; continue;
 }
 0x309c1bf => {
  cpu.push(0x309c1c4); f_030a4341(cpu); if cpu.returned_to != 0x309c1c4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c1d8; continue; }
  pc = 0x309c1c8; continue;
 }
 0x309c1c8 => {
  cpu.push(0x309c1cd); f_0309daa0(cpu); if cpu.returned_to != 0x309c1cd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c1d8; continue; }
  pc = 0x309c1d1; continue;
 }
 0x309c1d1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c1da; continue;
 }
 0x309c1d8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c1da; continue;
 }
 0x309c1da => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c21a; continue; }
  pc = 0x309c1de; continue;
 }
 0x309c1de => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149f24u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c212); cpu.call(t); if cpu.returned_to != 0x309c212 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c21a); f_0309de70(cpu); if cpu.returned_to != 0x309c21a { return; }
  pc = 0x309c21a; continue;
 }
 0x309c21a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309c22f); f_030a5d70(cpu); if cpu.returned_to != 0x309c22f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c27c; continue; }
  pc = 0x309c23d; continue;
 }
 0x309c23d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309c24f; continue; }
  pc = 0x309c246; continue;
 }
 0x309c246 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c27c; continue; }
  pc = 0x309c24f; continue;
 }
 0x309c24f => {
  cpu.push(0x309c254); f_030a4341(cpu); if cpu.returned_to != 0x309c254 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c27c; continue; }
  pc = 0x309c258; continue;
 }
 0x309c258 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309c272); f_030a45d3(cpu); if cpu.returned_to != 0x309c272 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309c27c); f_0309ddd0(cpu); if cpu.returned_to != 0x309c27c { return; }
  pc = 0x309c27c; continue;
 }
 0x309c27c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn o_0309c2a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c2a0); }
 let mut pc: u32 = 0x309c2a0;
 loop { match pc {
 0x309c2a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c2e8; continue; }
  pc = 0x309c2c1; continue;
 }
 0x309c2c1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c2cf; continue; }
  pc = 0x309c2c6; continue;
 }
 0x309c2c6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c2e8; continue; }
  pc = 0x309c2cf; continue;
 }
 0x309c2cf => {
  cpu.push(0x309c2d4); f_030a4341(cpu); if cpu.returned_to != 0x309c2d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c2e8; continue; }
  pc = 0x309c2d8; continue;
 }
 0x309c2d8 => {
  cpu.push(0x309c2dd); f_0309daa0(cpu); if cpu.returned_to != 0x309c2dd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c2e8; continue; }
  pc = 0x309c2e1; continue;
 }
 0x309c2e1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c2ea; continue;
 }
 0x309c2e8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c2ea; continue;
 }
 0x309c2ea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c31c; continue; }
  pc = 0x309c2ee; continue;
 }
 0x309c2ee => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149f08u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c314); cpu.call(t); if cpu.returned_to != 0x309c314 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c31c); f_0309de70(cpu); if cpu.returned_to != 0x309c31c { return; }
  pc = 0x309c31c; continue;
 }
 0x309c31c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c329); f_030a5cb0(cpu); if cpu.returned_to != 0x309c329 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309c377; continue; }
  pc = 0x309c338; continue;
 }
 0x309c338 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309c34a; continue; }
  pc = 0x309c341; continue;
 }
 0x309c341 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c377; continue; }
  pc = 0x309c34a; continue;
 }
 0x309c34a => {
  cpu.push(0x309c34f); f_030a4341(cpu); if cpu.returned_to != 0x309c34f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c377; continue; }
  pc = 0x309c353; continue;
 }
 0x309c353 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309c36d); f_030a45d3(cpu); if cpu.returned_to != 0x309c36d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309c377); f_0309de20(cpu); if cpu.returned_to != 0x309c377 { return; }
  pc = 0x309c377; continue;
 }
 0x309c377 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309c3a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c3a0); }
 let mut pc: u32 = 0x309c3a0;
 loop { match pc {
 0x309c3a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c3e7; continue; }
  pc = 0x309c3c0; continue;
 }
 0x309c3c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c3ce; continue; }
  pc = 0x309c3c5; continue;
 }
 0x309c3c5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c3e7; continue; }
  pc = 0x309c3ce; continue;
 }
 0x309c3ce => {
  cpu.push(0x309c3d3); f_030a4341(cpu); if cpu.returned_to != 0x309c3d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c3e7; continue; }
  pc = 0x309c3d7; continue;
 }
 0x309c3d7 => {
  cpu.push(0x309c3dc); f_0309daa0(cpu); if cpu.returned_to != 0x309c3dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c3e7; continue; }
  pc = 0x309c3e0; continue;
 }
 0x309c3e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c3e9; continue;
 }
 0x309c3e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c3e9; continue;
 }
 0x309c3e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c41b; continue; }
  pc = 0x309c3ed; continue;
 }
 0x309c3ed => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149ee8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c413); cpu.call(t); if cpu.returned_to != 0x309c413 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c41b); f_0309de70(cpu); if cpu.returned_to != 0x309c41b { return; }
  pc = 0x309c41b; continue;
 }
 0x309c41b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c428); f_030a49a0(cpu); if cpu.returned_to != 0x309c428 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c450); }
 let mut pc: u32 = 0x309c450;
 loop { match pc {
 0x309c450 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c498; continue; }
  pc = 0x309c471; continue;
 }
 0x309c471 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c47f; continue; }
  pc = 0x309c476; continue;
 }
 0x309c476 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c498; continue; }
  pc = 0x309c47f; continue;
 }
 0x309c47f => {
  cpu.push(0x309c484); f_030a4341(cpu); if cpu.returned_to != 0x309c484 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c498; continue; }
  pc = 0x309c488; continue;
 }
 0x309c488 => {
  cpu.push(0x309c48d); f_0309daa0(cpu); if cpu.returned_to != 0x309c48d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c498; continue; }
  pc = 0x309c491; continue;
 }
 0x309c491 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c49a; continue;
 }
 0x309c498 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c49a; continue;
 }
 0x309c49a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c4da; continue; }
  pc = 0x309c49e; continue;
 }
 0x309c49e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149ec8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c4d2); cpu.call(t); if cpu.returned_to != 0x309c4d2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c4da); f_0309de70(cpu); if cpu.returned_to != 0x309c4da { return; }
  pc = 0x309c4da; continue;
 }
 0x309c4da => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309c4ef); f_030a4a40(cpu); if cpu.returned_to != 0x309c4ef { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c53c; continue; }
  pc = 0x309c4fd; continue;
 }
 0x309c4fd => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309c50f; continue; }
  pc = 0x309c506; continue;
 }
 0x309c506 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c53c; continue; }
  pc = 0x309c50f; continue;
 }
 0x309c50f => {
  cpu.push(0x309c514); f_030a4341(cpu); if cpu.returned_to != 0x309c514 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c53c; continue; }
  pc = 0x309c518; continue;
 }
 0x309c518 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309c532); f_030a45d3(cpu); if cpu.returned_to != 0x309c532 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309c53c); f_0309ddd0(cpu); if cpu.returned_to != 0x309c53c { return; }
  pc = 0x309c53c; continue;
 }
 0x309c53c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309c560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c560); }
 let mut pc: u32 = 0x309c560;
 loop { match pc {
 0x309c560 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c5a8; continue; }
  pc = 0x309c581; continue;
 }
 0x309c581 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c58f; continue; }
  pc = 0x309c586; continue;
 }
 0x309c586 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c5a8; continue; }
  pc = 0x309c58f; continue;
 }
 0x309c58f => {
  cpu.push(0x309c594); f_030a4341(cpu); if cpu.returned_to != 0x309c594 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c5a8; continue; }
  pc = 0x309c598; continue;
 }
 0x309c598 => {
  cpu.push(0x309c59d); f_0309daa0(cpu); if cpu.returned_to != 0x309c59d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c5a8; continue; }
  pc = 0x309c5a1; continue;
 }
 0x309c5a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c5aa; continue;
 }
 0x309c5a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c5aa; continue;
 }
 0x309c5aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c5dc; continue; }
  pc = 0x309c5ae; continue;
 }
 0x309c5ae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149ea8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c5d4); cpu.call(t); if cpu.returned_to != 0x309c5d4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c5dc); f_0309de70(cpu); if cpu.returned_to != 0x309c5dc { return; }
  pc = 0x309c5dc; continue;
 }
 0x309c5dc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c5e9); f_030a48f0(cpu); if cpu.returned_to != 0x309c5e9 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309c637; continue; }
  pc = 0x309c5f8; continue;
 }
 0x309c5f8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309c60a; continue; }
  pc = 0x309c601; continue;
 }
 0x309c601 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c637; continue; }
  pc = 0x309c60a; continue;
 }
 0x309c60a => {
  cpu.push(0x309c60f); f_030a4341(cpu); if cpu.returned_to != 0x309c60f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c637; continue; }
  pc = 0x309c613; continue;
 }
 0x309c613 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309c62d); f_030a45d3(cpu); if cpu.returned_to != 0x309c62d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309c637); f_0309ddd0(cpu); if cpu.returned_to != 0x309c637 { return; }
  pc = 0x309c637; continue;
 }
 0x309c637 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309c660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c660); }
 let mut pc: u32 = 0x309c660;
 loop { match pc {
 0x309c660 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c6a0; continue; }
  pc = 0x309c679; continue;
 }
 0x309c679 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c687; continue; }
  pc = 0x309c67e; continue;
 }
 0x309c67e => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c6a0; continue; }
  pc = 0x309c687; continue;
 }
 0x309c687 => {
  cpu.push(0x309c68c); f_030a4341(cpu); if cpu.returned_to != 0x309c68c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c6a0; continue; }
  pc = 0x309c690; continue;
 }
 0x309c690 => {
  cpu.push(0x309c695); f_0309daa0(cpu); if cpu.returned_to != 0x309c695 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c6a0; continue; }
  pc = 0x309c699; continue;
 }
 0x309c699 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c6a2; continue;
 }
 0x309c6a0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c6a2; continue;
 }
 0x309c6a2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c6cc; continue; }
  pc = 0x309c6a6; continue;
 }
 0x309c6a6 => {
  { let v: u32 = 0x3149e8cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149e8cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c6c9); f_030a45d3(cpu); if cpu.returned_to != 0x309c6c9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x309c6cc; continue;
 }
 0x309c6cc => {
  cpu.push(0x309c6d1); f_030a43cb(cpu); if cpu.returned_to != 0x309c6d1 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32d0cd0u32)); cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c6e0); }
 let mut pc: u32 = 0x309c6e0;
 loop { match pc {
 0x309c6e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c727; continue; }
  pc = 0x309c700; continue;
 }
 0x309c700 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c70e; continue; }
  pc = 0x309c705; continue;
 }
 0x309c705 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c727; continue; }
  pc = 0x309c70e; continue;
 }
 0x309c70e => {
  cpu.push(0x309c713); f_030a4341(cpu); if cpu.returned_to != 0x309c713 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c727; continue; }
  pc = 0x309c717; continue;
 }
 0x309c717 => {
  cpu.push(0x309c71c); f_0309daa0(cpu); if cpu.returned_to != 0x309c71c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c727; continue; }
  pc = 0x309c720; continue;
 }
 0x309c720 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c729; continue;
 }
 0x309c727 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c729; continue;
 }
 0x309c729 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c75b; continue; }
  pc = 0x309c72d; continue;
 }
 0x309c72d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149e6cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c753); cpu.call(t); if cpu.returned_to != 0x309c753 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c75b); f_0309de70(cpu); if cpu.returned_to != 0x309c75b { return; }
  pc = 0x309c75b; continue;
 }
 0x309c75b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c768); f_030a43aa(cpu); if cpu.returned_to != 0x309c768 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c790); }
 let mut pc: u32 = 0x309c790;
 loop { match pc {
 0x309c790 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c7d0; continue; }
  pc = 0x309c7a9; continue;
 }
 0x309c7a9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c7b7; continue; }
  pc = 0x309c7ae; continue;
 }
 0x309c7ae => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c7d0; continue; }
  pc = 0x309c7b7; continue;
 }
 0x309c7b7 => {
  cpu.push(0x309c7bc); f_030a4341(cpu); if cpu.returned_to != 0x309c7bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c7d0; continue; }
  pc = 0x309c7c0; continue;
 }
 0x309c7c0 => {
  cpu.push(0x309c7c5); f_0309daa0(cpu); if cpu.returned_to != 0x309c7c5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c7d0; continue; }
  pc = 0x309c7c9; continue;
 }
 0x309c7c9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c7d2; continue;
 }
 0x309c7d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c7d2; continue;
 }
 0x309c7d2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c7fc; continue; }
  pc = 0x309c7d6; continue;
 }
 0x309c7d6 => {
  { let v: u32 = 0x3149e54u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149e54u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c7f9); f_030a45d3(cpu); if cpu.returned_to != 0x309c7f9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x309c7fc; continue;
 }
 0x309c7fc => {
  cpu.push(0x309c801); f_030a445f(cpu); if cpu.returned_to != 0x309c801 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32d0cd0u32)); cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c810); }
 let mut pc: u32 = 0x309c810;
 loop { match pc {
 0x309c810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c857; continue; }
  pc = 0x309c830; continue;
 }
 0x309c830 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c83e; continue; }
  pc = 0x309c835; continue;
 }
 0x309c835 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c857; continue; }
  pc = 0x309c83e; continue;
 }
 0x309c83e => {
  cpu.push(0x309c843); f_030a4341(cpu); if cpu.returned_to != 0x309c843 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c857; continue; }
  pc = 0x309c847; continue;
 }
 0x309c847 => {
  cpu.push(0x309c84c); f_0309daa0(cpu); if cpu.returned_to != 0x309c84c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c857; continue; }
  pc = 0x309c850; continue;
 }
 0x309c850 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c859; continue;
 }
 0x309c857 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c859; continue;
 }
 0x309c859 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c88b; continue; }
  pc = 0x309c85d; continue;
 }
 0x309c85d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149e3cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c883); cpu.call(t); if cpu.returned_to != 0x309c883 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c88b); f_0309de70(cpu); if cpu.returned_to != 0x309c88b { return; }
  pc = 0x309c88b; continue;
 }
 0x309c88b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c898); f_030a4435(cpu); if cpu.returned_to != 0x309c898 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c8c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c8c0); }
 let mut pc: u32 = 0x309c8c0;
 loop { match pc {
 0x309c8c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c900; continue; }
  pc = 0x309c8d9; continue;
 }
 0x309c8d9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c8e7; continue; }
  pc = 0x309c8de; continue;
 }
 0x309c8de => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c900; continue; }
  pc = 0x309c8e7; continue;
 }
 0x309c8e7 => {
  cpu.push(0x309c8ec); f_030a4341(cpu); if cpu.returned_to != 0x309c8ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c900; continue; }
  pc = 0x309c8f0; continue;
 }
 0x309c8f0 => {
  cpu.push(0x309c8f5); f_0309daa0(cpu); if cpu.returned_to != 0x309c8f5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c900; continue; }
  pc = 0x309c8f9; continue;
 }
 0x309c8f9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c902; continue;
 }
 0x309c900 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c902; continue;
 }
 0x309c902 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c92c; continue; }
  pc = 0x309c906; continue;
 }
 0x309c906 => {
  { let v: u32 = 0x3149e20u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149e20u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c929); f_030a45d3(cpu); if cpu.returned_to != 0x309c929 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x309c92c; continue;
 }
 0x309c92c => {
  cpu.push(0x309c931); f_030a4415(cpu); if cpu.returned_to != 0x309c931 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x32d0cd0u32)); cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c940); }
 let mut pc: u32 = 0x309c940;
 loop { match pc {
 0x309c940 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309c987; continue; }
  pc = 0x309c960; continue;
 }
 0x309c960 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309c96e; continue; }
  pc = 0x309c965; continue;
 }
 0x309c965 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309c987; continue; }
  pc = 0x309c96e; continue;
 }
 0x309c96e => {
  cpu.push(0x309c973); f_030a4341(cpu); if cpu.returned_to != 0x309c973 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309c987; continue; }
  pc = 0x309c977; continue;
 }
 0x309c977 => {
  cpu.push(0x309c97c); f_0309daa0(cpu); if cpu.returned_to != 0x309c97c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c987; continue; }
  pc = 0x309c980; continue;
 }
 0x309c980 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309c989; continue;
 }
 0x309c987 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309c989; continue;
 }
 0x309c989 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309c9bb; continue; }
  pc = 0x309c98d; continue;
 }
 0x309c98d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149e08u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309c9b3); cpu.call(t); if cpu.returned_to != 0x309c9b3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309c9bb); f_0309de70(cpu); if cpu.returned_to != 0x309c9bb { return; }
  pc = 0x309c9bb; continue;
 }
 0x309c9bb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309c9c8); f_030a43eb(cpu); if cpu.returned_to != 0x309c9c8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309c9f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309c9f0); }
 let mut pc: u32 = 0x309c9f0;
 loop { match pc {
 0x309c9f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309ca32; continue; }
  pc = 0x309ca0b; continue;
 }
 0x309ca0b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309ca19; continue; }
  pc = 0x309ca10; continue;
 }
 0x309ca10 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309ca32; continue; }
  pc = 0x309ca19; continue;
 }
 0x309ca19 => {
  cpu.push(0x309ca1e); f_030a4341(cpu); if cpu.returned_to != 0x309ca1e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309ca32; continue; }
  pc = 0x309ca22; continue;
 }
 0x309ca22 => {
  cpu.push(0x309ca27); f_0309daa0(cpu); if cpu.returned_to != 0x309ca27 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ca32; continue; }
  pc = 0x309ca2b; continue;
 }
 0x309ca2b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309ca34; continue;
 }
 0x309ca32 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309ca34; continue;
 }
 0x309ca34 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ca5e; continue; }
  pc = 0x309ca38; continue;
 }
 0x309ca38 => {
  { let v: u32 = 0x3149decu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149decu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309ca5b); f_030a45d3(cpu); if cpu.returned_to != 0x309ca5b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x309ca5e; continue;
 }
 0x309ca5e => {
  cpu.push(0x309ca63); f_030a4515(cpu); if cpu.returned_to != 0x309ca63 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309caae; continue; }
  pc = 0x309ca6f; continue;
 }
 0x309ca6f => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309ca81; continue; }
  pc = 0x309ca78; continue;
 }
 0x309ca78 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309caae; continue; }
  pc = 0x309ca81; continue;
 }
 0x309ca81 => {
  cpu.push(0x309ca86); f_030a4341(cpu); if cpu.returned_to != 0x309ca86 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309caae; continue; }
  pc = 0x309ca8a; continue;
 }
 0x309ca8a => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309caa4); f_030a45d3(cpu); if cpu.returned_to != 0x309caa4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309caae); f_0309de20(cpu); if cpu.returned_to != 0x309caae { return; }
  pc = 0x309caae; continue;
 }
 0x309caae => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309cb90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309cb90); }
 let mut pc: u32 = 0x309cb90;
 loop { match pc {
 0x309cb90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309cbd7; continue; }
  pc = 0x309cbb0; continue;
 }
 0x309cbb0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309cbbe; continue; }
  pc = 0x309cbb5; continue;
 }
 0x309cbb5 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309cbd7; continue; }
  pc = 0x309cbbe; continue;
 }
 0x309cbbe => {
  cpu.push(0x309cbc3); f_030a4341(cpu); if cpu.returned_to != 0x309cbc3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309cbd7; continue; }
  pc = 0x309cbc7; continue;
 }
 0x309cbc7 => {
  cpu.push(0x309cbcc); f_0309daa0(cpu); if cpu.returned_to != 0x309cbcc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cbd7; continue; }
  pc = 0x309cbd0; continue;
 }
 0x309cbd0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309cbd9; continue;
 }
 0x309cbd7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309cbd9; continue;
 }
 0x309cbd9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cc19; continue; }
  pc = 0x309cbdd; continue;
 }
 0x309cbdd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149da8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309cc11); cpu.call(t); if cpu.returned_to != 0x309cc11 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309cc19); f_0309de70(cpu); if cpu.returned_to != 0x309cc19 { return; }
  pc = 0x309cc19; continue;
 }
 0x309cc19 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309cc2e); f_030a44af(cpu); if cpu.returned_to != 0x309cc2e { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309cc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309cc50); }
 let mut pc: u32 = 0x309cc50;
 loop { match pc {
 0x309cc50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309cc97; continue; }
  pc = 0x309cc70; continue;
 }
 0x309cc70 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309cc7e; continue; }
  pc = 0x309cc75; continue;
 }
 0x309cc75 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309cc97; continue; }
  pc = 0x309cc7e; continue;
 }
 0x309cc7e => {
  cpu.push(0x309cc83); f_030a4341(cpu); if cpu.returned_to != 0x309cc83 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309cc97; continue; }
  pc = 0x309cc87; continue;
 }
 0x309cc87 => {
  cpu.push(0x309cc8c); f_0309daa0(cpu); if cpu.returned_to != 0x309cc8c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cc97; continue; }
  pc = 0x309cc90; continue;
 }
 0x309cc90 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309cc99; continue;
 }
 0x309cc97 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309cc99; continue;
 }
 0x309cc99 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ccd9; continue; }
  pc = 0x309cc9d; continue;
 }
 0x309cc9d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d88u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309ccd1); cpu.call(t); if cpu.returned_to != 0x309ccd1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309ccd9); f_0309de70(cpu); if cpu.returned_to != 0x309ccd9 { return; }
  pc = 0x309ccd9; continue;
 }
 0x309ccd9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309ccee); f_030a447f(cpu); if cpu.returned_to != 0x309ccee { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309cd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309cd10); }
 let mut pc: u32 = 0x309cd10;
 loop { match pc {
 0x309cd10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309cd58; continue; }
  pc = 0x309cd31; continue;
 }
 0x309cd31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309cd3f; continue; }
  pc = 0x309cd36; continue;
 }
 0x309cd36 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309cd58; continue; }
  pc = 0x309cd3f; continue;
 }
 0x309cd3f => {
  cpu.push(0x309cd44); f_030a4341(cpu); if cpu.returned_to != 0x309cd44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309cd58; continue; }
  pc = 0x309cd48; continue;
 }
 0x309cd48 => {
  cpu.push(0x309cd4d); f_0309daa0(cpu); if cpu.returned_to != 0x309cd4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cd58; continue; }
  pc = 0x309cd51; continue;
 }
 0x309cd51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309cd5a; continue;
 }
 0x309cd58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309cd5a; continue;
 }
 0x309cd5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cd9a; continue; }
  pc = 0x309cd5e; continue;
 }
 0x309cd5e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d6cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309cd92); cpu.call(t); if cpu.returned_to != 0x309cd92 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309cd9a); f_0309de70(cpu); if cpu.returned_to != 0x309cd9a { return; }
  pc = 0x309cd9a; continue;
 }
 0x309cd9a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309cdaf); f_030a4388(cpu); if cpu.returned_to != 0x309cdaf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cdfc; continue; }
  pc = 0x309cdbd; continue;
 }
 0x309cdbd => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309cdcf; continue; }
  pc = 0x309cdc6; continue;
 }
 0x309cdc6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309cdfc; continue; }
  pc = 0x309cdcf; continue;
 }
 0x309cdcf => {
  cpu.push(0x309cdd4); f_030a4341(cpu); if cpu.returned_to != 0x309cdd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309cdfc; continue; }
  pc = 0x309cdd8; continue;
 }
 0x309cdd8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309cdf2); f_030a45d3(cpu); if cpu.returned_to != 0x309cdf2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309cdfc); f_0309de20(cpu); if cpu.returned_to != 0x309cdfc { return; }
  pc = 0x309cdfc; continue;
 }
 0x309cdfc => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309ce20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ce20); }
 let mut pc: u32 = 0x309ce20;
 loop { match pc {
 0x309ce20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309ce68; continue; }
  pc = 0x309ce41; continue;
 }
 0x309ce41 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309ce4f; continue; }
  pc = 0x309ce46; continue;
 }
 0x309ce46 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309ce68; continue; }
  pc = 0x309ce4f; continue;
 }
 0x309ce4f => {
  cpu.push(0x309ce54); f_030a4341(cpu); if cpu.returned_to != 0x309ce54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309ce68; continue; }
  pc = 0x309ce58; continue;
 }
 0x309ce58 => {
  cpu.push(0x309ce5d); f_0309daa0(cpu); if cpu.returned_to != 0x309ce5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ce68; continue; }
  pc = 0x309ce61; continue;
 }
 0x309ce61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309ce6a; continue;
 }
 0x309ce68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309ce6a; continue;
 }
 0x309ce6a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309ce9c; continue; }
  pc = 0x309ce6e; continue;
 }
 0x309ce6e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d50u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309ce94); cpu.call(t); if cpu.returned_to != 0x309ce94 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309ce9c); f_0309de70(cpu); if cpu.returned_to != 0x309ce9c { return; }
  pc = 0x309ce9c; continue;
 }
 0x309ce9c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309cea9); f_030a4347(cpu); if cpu.returned_to != 0x309cea9 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309cef7; continue; }
  pc = 0x309ceb8; continue;
 }
 0x309ceb8 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309ceca; continue; }
  pc = 0x309cec1; continue;
 }
 0x309cec1 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309cef7; continue; }
  pc = 0x309ceca; continue;
 }
 0x309ceca => {
  cpu.push(0x309cecf); f_030a4341(cpu); if cpu.returned_to != 0x309cecf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309cef7; continue; }
  pc = 0x309ced3; continue;
 }
 0x309ced3 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309ceed); f_030a45d3(cpu); if cpu.returned_to != 0x309ceed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309cef7); f_0309de20(cpu); if cpu.returned_to != 0x309cef7 { return; }
  pc = 0x309cef7; continue;
 }
 0x309cef7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309cf20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309cf20); }
 let mut pc: u32 = 0x309cf20;
 loop { match pc {
 0x309cf20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3a0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3b4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3b8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309cf76; continue; }
  pc = 0x309cf4f; continue;
 }
 0x309cf4f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309cf5d; continue; }
  pc = 0x309cf54; continue;
 }
 0x309cf54 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309cf76; continue; }
  pc = 0x309cf5d; continue;
 }
 0x309cf5d => {
  cpu.push(0x309cf62); f_030a4341(cpu); if cpu.returned_to != 0x309cf62 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309cf76; continue; }
  pc = 0x309cf66; continue;
 }
 0x309cf66 => {
  cpu.push(0x309cf6b); f_0309daa0(cpu); if cpu.returned_to != 0x309cf6b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cf76; continue; }
  pc = 0x309cf6f; continue;
 }
 0x309cf6f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309cf78; continue;
 }
 0x309cf76 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309cf78; continue;
 }
 0x309cf78 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309cfb4; continue; }
  pc = 0x309cf7c; continue;
 }
 0x309cf7c => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149cd4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x38cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309cfa7); cpu.call(t); if cpu.returned_to != 0x309cfa7 { return; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309cfb4); f_0309de70(cpu); if cpu.returned_to != 0x309cfb4 { return; }
  pc = 0x309cfb4; continue;
 }
 0x309cfb4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309cfbb); f_030a4d50(cpu); if cpu.returned_to != 0x309cfbb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d1ed; continue; }
  pc = 0x309cfc8; continue;
 }
 0x309cfc8 => {
  { cpu.sub32(cpu.r32(0x32d0cd4u32), 0x0u32); }
  if cpu.zf { pc = 0x309d235; continue; }
  pc = 0x309cfd5; continue;
 }
 0x309cfd5 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309cfeb; continue; }
  pc = 0x309cfde; continue;
 }
 0x309cfde => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d235; continue; }
  pc = 0x309cfeb; continue;
 }
 0x309cfeb => {
  cpu.push(0x309cff0); f_030a4341(cpu); if cpu.returned_to != 0x309cff0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d235; continue; }
  pc = 0x309cff8; continue;
 }
 0x309cff8 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x309d012); f_030a45d3(cpu); if cpu.returned_to != 0x309d012 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x388u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149cf0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x394u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d03f); cpu.call(t); if cpu.returned_to != 0x309d03f { return; }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d04c); f_0309de70(cpu); if cpu.returned_to != 0x309d04c { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309d066); f_030a45d3(cpu); if cpu.returned_to != 0x309d066 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d00u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3acu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d088); cpu.call(t); if cpu.returned_to != 0x309d088 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d090); f_0309de70(cpu); if cpu.returned_to != 0x309d090 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309d0aa); f_030a45d3(cpu); if cpu.returned_to != 0x309d0aa { return; }
  { let v: u32 = cpu.r32(0xfeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d10u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x398u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d0da); cpu.call(t); if cpu.returned_to != 0x309d0da { return; }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d0e7); f_0309de70(cpu); if cpu.returned_to != 0x309d0e7 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d100); f_030a45d3(cpu); if cpu.returned_to != 0x309d100 { return; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d20u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x39cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d130); cpu.call(t); if cpu.returned_to != 0x309d130 { return; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d13d); f_0309de70(cpu); if cpu.returned_to != 0x309d13d { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309d157); f_030a45d3(cpu); if cpu.returned_to != 0x309d157 { return; }
  { let v: u32 = cpu.r32(0x102u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d30u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d187); cpu.call(t); if cpu.returned_to != 0x309d187 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d194); f_0309de70(cpu); if cpu.returned_to != 0x309d194 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x309d1ae); f_030a45d3(cpu); if cpu.returned_to != 0x309d1ae { return; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x398u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149d40u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d1de); cpu.call(t); if cpu.returned_to != 0x309d1de { return; }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d1eb); f_0309de70(cpu); if cpu.returned_to != 0x309d1eb { return; }
  pc = 0x309d235; continue;
 }
 0x309d1ed => {
  { cpu.sub32(cpu.r32(0x32d0cd4u32), 0x0u32); }
  if cpu.zf { pc = 0x309d235; continue; }
  pc = 0x309d1f6; continue;
 }
 0x309d1f6 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309d208; continue; }
  pc = 0x309d1ff; continue;
 }
 0x309d1ff => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d235; continue; }
  pc = 0x309d208; continue;
 }
 0x309d208 => {
  cpu.push(0x309d20d); f_030a4341(cpu); if cpu.returned_to != 0x309d20d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d235; continue; }
  pc = 0x309d211; continue;
 }
 0x309d211 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309d22b); f_030a45d3(cpu); if cpu.returned_to != 0x309d22b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x309d235); f_0309de20(cpu); if cpu.returned_to != 0x309d235 { return; }
  pc = 0x309d235; continue;
 }
 0x309d235 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309d250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d250); }
 let mut pc: u32 = 0x309d250;
 loop { match pc {
 0x309d250 => {
  f_030a4341(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d260); }
 let mut pc: u32 = 0x309d260;
 loop { match pc {
 0x309d260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d2a7; continue; }
  pc = 0x309d280; continue;
 }
 0x309d280 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d28e; continue; }
  pc = 0x309d285; continue;
 }
 0x309d285 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d2a7; continue; }
  pc = 0x309d28e; continue;
 }
 0x309d28e => {
  cpu.push(0x309d293); f_030a4341(cpu); if cpu.returned_to != 0x309d293 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d2a7; continue; }
  pc = 0x309d297; continue;
 }
 0x309d297 => {
  cpu.push(0x309d29c); f_0309daa0(cpu); if cpu.returned_to != 0x309d29c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d2a7; continue; }
  pc = 0x309d2a0; continue;
 }
 0x309d2a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d2a9; continue;
 }
 0x309d2a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d2a9; continue;
 }
 0x309d2a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d2db; continue; }
  pc = 0x309d2ad; continue;
 }
 0x309d2ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149cc4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d2d3); cpu.call(t); if cpu.returned_to != 0x309d2d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d2db); f_0309de70(cpu); if cpu.returned_to != 0x309d2db { return; }
  pc = 0x309d2db; continue;
 }
 0x309d2db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309d2e8); f_030a4710(cpu); if cpu.returned_to != 0x309d2e8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d310); }
 let mut pc: u32 = 0x309d310;
 loop { match pc {
 0x309d310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d357; continue; }
  pc = 0x309d330; continue;
 }
 0x309d330 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d33e; continue; }
  pc = 0x309d335; continue;
 }
 0x309d335 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d357; continue; }
  pc = 0x309d33e; continue;
 }
 0x309d33e => {
  cpu.push(0x309d343); f_030a4341(cpu); if cpu.returned_to != 0x309d343 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d357; continue; }
  pc = 0x309d347; continue;
 }
 0x309d347 => {
  cpu.push(0x309d34c); f_0309daa0(cpu); if cpu.returned_to != 0x309d34c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d357; continue; }
  pc = 0x309d350; continue;
 }
 0x309d350 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x309d359; continue;
 }
 0x309d357 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x309d359; continue;
 }
 0x309d359 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d3b5; continue; }
  pc = 0x309d35d; continue;
 }
 0x309d35d => {
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
  { let v = 0x3149c9cu32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d3ad); cpu.call(t); if cpu.returned_to != 0x309d3ad { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d3b5); f_0309de70(cpu); if cpu.returned_to != 0x309d3b5 { return; }
  pc = 0x309d3b5; continue;
 }
 0x309d3b5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309d3da); f_030a4267(cpu); if cpu.returned_to != 0x309d3da { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x309d428; continue; }
  pc = 0x309d3e9; continue;
 }
 0x309d3e9 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309d3fb; continue; }
  pc = 0x309d3f2; continue;
 }
 0x309d3f2 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d428; continue; }
  pc = 0x309d3fb; continue;
 }
 0x309d3fb => {
  cpu.push(0x309d400); f_030a4341(cpu); if cpu.returned_to != 0x309d400 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d428; continue; }
  pc = 0x309d404; continue;
 }
 0x309d404 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309d41e); f_030a45d3(cpu); if cpu.returned_to != 0x309d41e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309d428); f_0309de20(cpu); if cpu.returned_to != 0x309d428 { return; }
  pc = 0x309d428; continue;
 }
 0x309d428 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x32d0cd0u32)); cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309d440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d440); }
 let mut pc: u32 = 0x309d440;
 loop { match pc {
 0x309d440 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309d44a); f_030a4200(cpu); if cpu.returned_to != 0x309d44a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d450); }
 let mut pc: u32 = 0x309d450;
 loop { match pc {
 0x309d450 => {
  pc = 0x30a41fc; continue;
 }
 0x30a41fc => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d460); }
 let mut pc: u32 = 0x309d460;
 loop { match pc {
 0x309d460 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d4a7; continue; }
  pc = 0x309d480; continue;
 }
 0x309d480 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d48e; continue; }
  pc = 0x309d485; continue;
 }
 0x309d485 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d4a7; continue; }
  pc = 0x309d48e; continue;
 }
 0x309d48e => {
  cpu.push(0x309d493); f_030a4341(cpu); if cpu.returned_to != 0x309d493 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d4a7; continue; }
  pc = 0x309d497; continue;
 }
 0x309d497 => {
  cpu.push(0x309d49c); f_0309daa0(cpu); if cpu.returned_to != 0x309d49c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d4a7; continue; }
  pc = 0x309d4a0; continue;
 }
 0x309d4a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d4a9; continue;
 }
 0x309d4a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d4a9; continue;
 }
 0x309d4a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d4db; continue; }
  pc = 0x309d4ad; continue;
 }
 0x309d4ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149c80u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d4d3); cpu.call(t); if cpu.returned_to != 0x309d4d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d4db); f_0309de70(cpu); if cpu.returned_to != 0x309d4db { return; }
  pc = 0x309d4db; continue;
 }
 0x309d4db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309d4e8); f_030a41b7(cpu); if cpu.returned_to != 0x309d4e8 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d510); }
 let mut pc: u32 = 0x309d510;
 loop { match pc {
 0x309d510 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d557; continue; }
  pc = 0x309d530; continue;
 }
 0x309d530 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d53e; continue; }
  pc = 0x309d535; continue;
 }
 0x309d535 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d557; continue; }
  pc = 0x309d53e; continue;
 }
 0x309d53e => {
  cpu.push(0x309d543); f_030a4341(cpu); if cpu.returned_to != 0x309d543 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d557; continue; }
  pc = 0x309d547; continue;
 }
 0x309d547 => {
  cpu.push(0x309d54c); f_0309daa0(cpu); if cpu.returned_to != 0x309d54c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d557; continue; }
  pc = 0x309d550; continue;
 }
 0x309d550 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d559; continue;
 }
 0x309d557 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d559; continue;
 }
 0x309d559 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d5a7; continue; }
  pc = 0x309d55d; continue;
 }
 0x309d55d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149c60u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d59f); cpu.call(t); if cpu.returned_to != 0x309d59f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d5a7); f_0309de70(cpu); if cpu.returned_to != 0x309d5a7 { return; }
  pc = 0x309d5a7; continue;
 }
 0x309d5a7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d5c4); f_030a3fa1(cpu); if cpu.returned_to != 0x309d5c4 { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d5e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d5e0); }
 let mut pc: u32 = 0x309d5e0;
 loop { match pc {
 0x309d5e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d627; continue; }
  pc = 0x309d600; continue;
 }
 0x309d600 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d60e; continue; }
  pc = 0x309d605; continue;
 }
 0x309d605 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d627; continue; }
  pc = 0x309d60e; continue;
 }
 0x309d60e => {
  cpu.push(0x309d613); f_030a4341(cpu); if cpu.returned_to != 0x309d613 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d627; continue; }
  pc = 0x309d617; continue;
 }
 0x309d617 => {
  cpu.push(0x309d61c); f_0309daa0(cpu); if cpu.returned_to != 0x309d61c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d627; continue; }
  pc = 0x309d620; continue;
 }
 0x309d620 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d629; continue;
 }
 0x309d627 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d629; continue;
 }
 0x309d629 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d669; continue; }
  pc = 0x309d62d; continue;
 }
 0x309d62d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149c40u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d661); cpu.call(t); if cpu.returned_to != 0x309d661 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d669); f_0309de70(cpu); if cpu.returned_to != 0x309d669 { return; }
  pc = 0x309d669; continue;
 }
 0x309d669 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309d67e); f_030a3f29(cpu); if cpu.returned_to != 0x309d67e { return; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d6a0); }
 let mut pc: u32 = 0x309d6a0;
 loop { match pc {
 0x309d6a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d6e8; continue; }
  pc = 0x309d6c1; continue;
 }
 0x309d6c1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d6cf; continue; }
  pc = 0x309d6c6; continue;
 }
 0x309d6c6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d6e8; continue; }
  pc = 0x309d6cf; continue;
 }
 0x309d6cf => {
  cpu.push(0x309d6d4); f_030a4341(cpu); if cpu.returned_to != 0x309d6d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d6e8; continue; }
  pc = 0x309d6d8; continue;
 }
 0x309d6d8 => {
  cpu.push(0x309d6dd); f_0309daa0(cpu); if cpu.returned_to != 0x309d6dd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d6e8; continue; }
  pc = 0x309d6e1; continue;
 }
 0x309d6e1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d6ea; continue;
 }
 0x309d6e8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d6ea; continue;
 }
 0x309d6ea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d71c; continue; }
  pc = 0x309d6ee; continue;
 }
 0x309d6ee => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149c24u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d714); cpu.call(t); if cpu.returned_to != 0x309d714 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d71c); f_0309de70(cpu); if cpu.returned_to != 0x309d71c { return; }
  pc = 0x309d71c; continue;
 }
 0x309d71c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309d729); f_030a3f01(cpu); if cpu.returned_to != 0x309d729 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309d777; continue; }
  pc = 0x309d738; continue;
 }
 0x309d738 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309d74a; continue; }
  pc = 0x309d741; continue;
 }
 0x309d741 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d777; continue; }
  pc = 0x309d74a; continue;
 }
 0x309d74a => {
  cpu.push(0x309d74f); f_030a4341(cpu); if cpu.returned_to != 0x309d74f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d777; continue; }
  pc = 0x309d753; continue;
 }
 0x309d753 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309d76d); f_030a45d3(cpu); if cpu.returned_to != 0x309d76d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309d777); f_0309ddd0(cpu); if cpu.returned_to != 0x309d777 { return; }
  pc = 0x309d777; continue;
 }
 0x309d777 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309d7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d7a0); }
 let mut pc: u32 = 0x309d7a0;
 loop { match pc {
 0x309d7a0 => {
  pc = 0x30a3a97; continue;
 }
 0x30a3a97 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x315c102u32)); cpu.w32(0x315c102u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d7b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309d7b0); } r_0309d7b0(cpu, 0x309d7b0); }
pub fn f_030a3a90(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3a90); } r_0309d7b0(cpu, 0x30a3a90); }
fn r_0309d7b0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x309d7b0 => {
  pc = 0x30a3a90; continue;
 }
 0x30a3a90 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x315c102u32)); cpu.w32(0x315c102u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309d7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d7c0); }
 let mut pc: u32 = 0x309d7c0;
 loop { match pc {
 0x309d7c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d808; continue; }
  pc = 0x309d7e1; continue;
 }
 0x309d7e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d7ef; continue; }
  pc = 0x309d7e6; continue;
 }
 0x309d7e6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d808; continue; }
  pc = 0x309d7ef; continue;
 }
 0x309d7ef => {
  cpu.push(0x309d7f4); f_030a4341(cpu); if cpu.returned_to != 0x309d7f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d808; continue; }
  pc = 0x309d7f8; continue;
 }
 0x309d7f8 => {
  cpu.push(0x309d7fd); f_0309daa0(cpu); if cpu.returned_to != 0x309d7fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d808; continue; }
  pc = 0x309d801; continue;
 }
 0x309d801 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d80a; continue;
 }
 0x309d808 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d80a; continue;
 }
 0x309d80a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d84a; continue; }
  pc = 0x309d80e; continue;
 }
 0x309d80e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149c08u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d842); cpu.call(t); if cpu.returned_to != 0x309d842 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d84a); f_0309de70(cpu); if cpu.returned_to != 0x309d84a { return; }
  pc = 0x309d84a; continue;
 }
 0x309d84a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309d85f); f_030a4d30(cpu); if cpu.returned_to != 0x309d85f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d8ac; continue; }
  pc = 0x309d86d; continue;
 }
 0x309d86d => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309d87f; continue; }
  pc = 0x309d876; continue;
 }
 0x309d876 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d8ac; continue; }
  pc = 0x309d87f; continue;
 }
 0x309d87f => {
  cpu.push(0x309d884); f_030a4341(cpu); if cpu.returned_to != 0x309d884 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d8ac; continue; }
  pc = 0x309d888; continue;
 }
 0x309d888 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309d8a2); f_030a45d3(cpu); if cpu.returned_to != 0x309d8a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309d8ac); f_0309de20(cpu); if cpu.returned_to != 0x309d8ac { return; }
  pc = 0x309d8ac; continue;
 }
 0x309d8ac => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309d8d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d8d0); }
 let mut pc: u32 = 0x309d8d0;
 loop { match pc {
 0x309d8d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x309d918; continue; }
  pc = 0x309d8f1; continue;
 }
 0x309d8f1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x309d8ff; continue; }
  pc = 0x309d8f6; continue;
 }
 0x309d8f6 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d918; continue; }
  pc = 0x309d8ff; continue;
 }
 0x309d8ff => {
  cpu.push(0x309d904); f_030a4341(cpu); if cpu.returned_to != 0x309d904 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d918; continue; }
  pc = 0x309d908; continue;
 }
 0x309d908 => {
  cpu.push(0x309d90d); f_0309daa0(cpu); if cpu.returned_to != 0x309d90d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d918; continue; }
  pc = 0x309d911; continue;
 }
 0x309d911 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309d91a; continue;
 }
 0x309d918 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309d91a; continue;
 }
 0x309d91a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309d94c; continue; }
  pc = 0x309d91e; continue;
 }
 0x309d91e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149be8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309d944); cpu.call(t); if cpu.returned_to != 0x309d944 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309d94c); f_0309de70(cpu); if cpu.returned_to != 0x309d94c { return; }
  pc = 0x309d94c; continue;
 }
 0x309d94c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x309d959); f_030a4730(cpu); if cpu.returned_to != 0x309d959 { return; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309d9a7; continue; }
  pc = 0x309d968; continue;
 }
 0x309d968 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309d97a; continue; }
  pc = 0x309d971; continue;
 }
 0x309d971 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309d9a7; continue; }
  pc = 0x309d97a; continue;
 }
 0x309d97a => {
  cpu.push(0x309d97f); f_030a4341(cpu); if cpu.returned_to != 0x309d97f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309d9a7; continue; }
  pc = 0x309d983; continue;
 }
 0x309d983 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x315a9f0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309d99d); f_030a45d3(cpu); if cpu.returned_to != 0x309d99d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309d9a7); f_0309ddd0(cpu); if cpu.returned_to != 0x309d9a7 { return; }
  pc = 0x309d9a7; continue;
 }
 0x309d9a7 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x32d0cd0u32, v); }
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
pub fn f_0309d9d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309d9d0); }
 let mut pc: u32 = 0x309d9d0;
 loop { match pc {
 0x309d9d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315a9e8u32), 0x0u32); }
  if cpu.zf { pc = 0x309da98; continue; }
  pc = 0x309d9e1; continue;
 }
 0x309d9e1 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x32d0cd4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32d0cd0u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x309da1f; continue; }
  pc = 0x309d9f8; continue;
 }
 0x309d9f8 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x309da06; continue; }
  pc = 0x309d9fd; continue;
 }
 0x309d9fd => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309da1f; continue; }
  pc = 0x309da06; continue;
 }
 0x309da06 => {
  cpu.push(0x309da0b); f_030a4341(cpu); if cpu.returned_to != 0x309da0b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309da1f; continue; }
  pc = 0x309da0f; continue;
 }
 0x309da0f => {
  cpu.push(0x309da14); f_0309daa0(cpu); if cpu.returned_to != 0x309da14 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309da1f; continue; }
  pc = 0x309da18; continue;
 }
 0x309da18 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309da21; continue;
 }
 0x309da1f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309da21; continue;
 }
 0x309da21 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309da4b; continue; }
  pc = 0x309da25; continue;
 }
 0x309da25 => {
  { let v: u32 = 0x3149bd4u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149bd4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309da48); f_030a45d3(cpu); if cpu.returned_to != 0x309da48 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x309da4b; continue;
 }
 0x309da4b => {
  cpu.push(0x309da50); f_030a47a0(cpu); if cpu.returned_to != 0x309da50 { return; }
  { cpu.sub32(cpu.r32(0x32d0cd4u32), 0x0u32); }
  if cpu.zf { pc = 0x309da83; continue; }
  pc = 0x309da59; continue;
 }
 0x309da59 => {
  { cpu.sub32(cpu.r32(0x32d0cd0u32), 0x1u32); }
  if cpu.zf { pc = 0x309da6b; continue; }
  pc = 0x309da62; continue;
 }
 0x309da62 => {
  { cpu.sub32(cpu.r32(0x32d0cd8u32), 0x0u32); }
  if cpu.zf { pc = 0x309da83; continue; }
  pc = 0x309da6b; continue;
 }
 0x309da6b => {
  cpu.push(0x309da70); f_030a4341(cpu); if cpu.returned_to != 0x309da70 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309da83; continue; }
  pc = 0x309da74; continue;
 }
 0x309da74 => {
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x309da80); f_030a4645(cpu); if cpu.returned_to != 0x309da80 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x309da83; continue;
 }
 0x309da83 => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315a9e8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
  pc = 0x309da98; continue;
 }
 0x309da98 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309daa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309daa0); }
 let mut pc: u32 = 0x309daa0;
 loop { match pc {
 0x309daa0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  cpu.push(0x309daae); f_030a3a9e(cpu); if cpu.returned_to != 0x309daae { return; }
  { let v: u32 = cpu.r32(0x32d0cdcu32); cpu.edx = v; }
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
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x309db66; continue; }
  pc = 0x309db0e; continue;
 }
 0x309db0e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149b94u32; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x9cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309db5c); cpu.call(t); if cpu.returned_to != 0x309db5c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x309db64); f_0309de70(cpu); if cpu.returned_to != 0x309db64 { return; }
  pc = 0x309db8c; continue;
 }
 0x309db66 => {
  { let v: u32 = 0x3149bb0u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149bb0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309db89); f_030a45d3(cpu); if cpu.returned_to != 0x309db89 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x309db8c; continue;
 }
 0x309db8c => {
  { let v: u32 = cpu.r32(0x32d0cd0u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x309dbaf; continue; }
  pc = 0x309db97; continue;
 }
 0x309db97 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3149bc0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x309dbac); f_030a45d3(cpu); if cpu.returned_to != 0x309dbac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x309dbaf; continue;
 }
 0x309dbaf => {
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
pub fn f_0309dbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309dbc0); }
 let mut pc: u32 = 0x309dbc0;
 loop { match pc {
 0x309dbc0 => {
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
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309dc10); cpu.call(t); if cpu.returned_to != 0x309dc10 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309dc2e); f_030a45d3(cpu); if cpu.returned_to != 0x309dc2e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309ddd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ddd0); }
 let mut pc: u32 = 0x309ddd0;
 loop { match pc {
 0x309ddd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149b84u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309ddf4); cpu.call(t); if cpu.returned_to != 0x309ddf4 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309de12); f_030a45d3(cpu); if cpu.returned_to != 0x309de12 { return; }
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
pub fn f_0309de20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309de20); }
 let mut pc: u32 = 0x309de20;
 loop { match pc {
 0x309de20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x3149b74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x315bf50u32); cpu.push(0x309de44); cpu.call(t); if cpu.returned_to != 0x309de44 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309de62); f_030a45d3(cpu); if cpu.returned_to != 0x309de62 { return; }
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
pub fn f_0309de70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309de70); }
 let mut pc: u32 = 0x309de70;
 loop { match pc {
 0x309de70 => {
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
  { let v: u32 = cpu.r32(0x32d0ce0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309de91); f_030a45d3(cpu); if cpu.returned_to != 0x309de91 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309dea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309dea0); }
 let mut pc: u32 = 0x309dea0;
 loop { match pc {
 0x309dea0 => {
  { cpu.sub32(cpu.r32(0x315a9ecu32), 0x0u32); }
  if !cpu.zf { pc = 0x309df20; continue; }
  pc = 0x309dea9; continue;
 }
 0x309dea9 => {
  { let v = 0x3095db0u32; cpu.push(v); }
  { let v = 0x309dea0u32; cpu.push(v); }
  cpu.push(0x309deb8); f_030a38a0(cpu); if cpu.returned_to != 0x309deb8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x32d0cd0u32; cpu.push(v); }
  cpu.push(0x309dec7); f_030a37e0(cpu); if cpu.returned_to != 0x309dec7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x32d0cd4u32; cpu.push(v); }
  cpu.push(0x309ded6); f_030a37e0(cpu); if cpu.returned_to != 0x309ded6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x32d0cd8u32; cpu.push(v); }
  cpu.push(0x309dee5); f_030a37e0(cpu); if cpu.returned_to != 0x309dee5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x32d0ce0u32; cpu.push(v); }
  cpu.push(0x309def4); f_030a37e0(cpu); if cpu.returned_to != 0x309def4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x315a9e8u32; cpu.push(v); }
  cpu.push(0x309df03); f_030a37e0(cpu); if cpu.returned_to != 0x309df03 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x32d0cdcu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x309df17); f_030a37e0(cpu); if cpu.returned_to != 0x309df17 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315a9ecu32, v); }
  pc = 0x309df20; continue;
 }
 0x309df20 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309df21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309df21); }
 let mut pc: u32 = 0x309df21;
 loop { match pc {
 0x309df21 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d1ba0u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x309df4b; continue;
 }
 0x309df35 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x309df49; continue; }
  pc = 0x309df3d; continue;
 }
 0x309df3d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x309df49; continue; }
  pc = 0x309df44; continue;
 }
 0x309df44 => {
  cpu.push(0x309df49); f_030ad4ba(cpu); if cpu.returned_to != 0x309df49 { return; }
  pc = 0x309df49; continue;
 }
 0x309df49 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x309df4b; continue;
 }
 0x309df4b => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x309df35; continue; }
  pc = 0x309df4f; continue;
 }
 0x309df4f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309df58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309df58); }
 let mut pc: u32 = 0x309df58;
 loop { match pc {
 0x309df58 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x309dfa1; continue; }
  pc = 0x309df61; continue;
 }
 0x309df61 => {
  if !cpu.cf { pc = 0x309df8b; continue; }
  pc = 0x309df63; continue;
 }
 0x309df63 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x309df8b; continue; }
  pc = 0x309df6a; continue;
 }
 0x309df6a => {
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
 0x309df8b => {
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
  pc = 0x309dfa1; continue;
 }
 0x309dfa1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309dfa5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309dfa5); } r_0309dfa5(cpu, 0x309dfa5); }
pub fn f_0309e035(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e035); } r_0309dfa5(cpu, 0x309e035); }
fn r_0309dfa5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x309dfa5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x309dfab, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x309dfb0, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x309e031; continue; }
  pc = 0x309dfc0; continue;
 }
 0x309dfc0 => {
  if cpu.cf { pc = 0x309e035; continue; }
  pc = 0x309dfc2; continue;
 }
 0x309dfc2 => {
  if cpu.zf { cpu.call(0x309e039); return; }
  pc = 0x309dfc4; continue;
 }
 0x309dfc4 => {
  if (cpu.cf || cpu.zf) { pc = 0x309e03d; continue; }
  pc = 0x309dfc6; continue;
 }
 0x309dfc6 => {
  if cpu.sf { pc = 0x309e041; continue; }
  pc = 0x309dfc8; continue;
 }
 0x309dfc8 => {
  if cpu.pf() { f_0309dfca(cpu); return; }
  f_0309dfca(cpu); return;
 }
 0x309e031 => {
  { let v: u8 = 0x15u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x309e035; continue;
 }
 0x309e035 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309e03b); f_0309e02f(cpu); if cpu.returned_to != 0x309e03b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  pc = 0x309e03d; continue;
 }
 0x309e03d => {
  if cpu.zf { pc = 0x309e057; continue; }
  pc = 0x309e03f; continue;
 }
 0x309e03f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x41c64e6du32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3039u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x309e057; continue;
 }
 0x309e041 => {
  cpu.unimplemented(0x309e041, "Insd");
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0xc2u8; cpu.w8(0xffffff81u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esi); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x309e057; continue;
 }
 0x309e057 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309dfca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309dfca); }
 let mut pc: u32 = 0x309dfca;
 loop { match pc {
 0x309dfca => {
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
  pc = 0x309dfe0; continue;
 }
 0x309dfe0 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x309dfa5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x309dfe0; continue; }
  pc = 0x309e000; continue;
 }
 0x309e000 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x309e000; continue; }
  pc = 0x309e00a; continue;
 }
 0x309e00a => {
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
pub fn f_0309e014(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e014); }
 let mut pc: u32 = 0x309e014;
 loop { match pc {
 0x309e014 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x309e026; continue; }
  pc = 0x309e01c; continue;
 }
 0x309e01c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x309e026; continue; }
  pc = 0x309e020; continue;
 }
 0x309e020 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x309e026; continue;
 }
 0x309e026 => {
  cpu.push(0x309e02b); f_0309dfca(cpu); if cpu.returned_to != 0x309e02b { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e02f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e02f); }
 let mut pc: u32 = 0x309e02f;
 loop { match pc {
 0x309e02f => {
  { let v: u32 = 0x315b290u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e059(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e059); }
 let mut pc: u32 = 0x309e059;
 loop { match pc {
 0x309e059 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x309e061); f_0309e02f(cpu); if cpu.returned_to != 0x309e061 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309e067; continue; }
  pc = 0x309e065; continue;
 }
 0x309e065 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x309e067; continue;
 }
 0x309e067 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e06c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e06c); } r_0309e06c(cpu, 0x309e06c); }
pub fn f_0309e0e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e0e0); } r_0309e06c(cpu, 0x309e0e0); }
pub fn f_0309e2fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e2fd); } r_0309e06c(cpu, 0x309e2fd); }
pub fn f_030b4a0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30b4a0b); } r_0309e06c(cpu, 0x30b4a0b); }
fn r_0309e06c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x309e06c => {
  pc = 0x309e0e4; continue;
 }
 0x309e0e0 => {
  cpu.unimplemented(0x309e0e0, "Arpl");
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be3cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x315be34u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x315be5fu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x315be60u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x309e166; continue; }
  pc = 0x309e128; continue;
 }
 0x309e0e4 => {
  {  }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be3cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x315be34u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x315be5fu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x315be60u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x309e166; continue; }
  pc = 0x309e128; continue;
 }
 0x309e128 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x315be34u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffff000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be28u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x309e19e; continue;
 }
 0x309e166 => {
  { cpu.sub16((cpu.eax as u16), 0x4243u16); }
  if !cpu.zf { pc = 0x309e1a0; continue; }
  pc = 0x309e16c; continue;
 }
 0x309e16c => {
  { let v: u32 = cpu.edx; cpu.w32(0x315b29cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x315be34u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x309e19e; continue;
 }
 0x309e19e => {
  pc = 0x309e1f9; continue;
 }
 0x309e1a0 => {
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xff00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x309e1e2; continue; }
  pc = 0x309e1ae; continue;
 }
 0x309e1ae => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x309e1bd; continue; }
  pc = 0x309e1b7; continue;
 }
 0x309e1b7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x315b2a0u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x309e1bd; continue;
 }
 0x309e1bd => {
  { let v: u16 = 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x309e1d1; continue; }
  pc = 0x309e1cf; continue;
 }
 0x309e1cf => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x309e1d1; continue;
 }
 0x309e1d1 => {
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x315be34u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x309e1f9; continue;
 }
 0x309e1e2 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x24u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x17u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x309e1f9; continue;
 }
 0x309e1f9 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x315be56u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x315be57u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x309e341u32.wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x315be59u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x315be5du32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r16(0x315be34u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x32d2060u32; cpu.edx = v; }
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
  if cpu.zf { pc = 0x309e24e; continue; }
  pc = 0x309e24b; continue;
 }
 0x309e24b => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x309e24e; continue;
 }
 0x309e24e => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r16(0x315be5du32.wrapping_add(cpu.bases[0])); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x309e260; continue;
 }
 0x309e260 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x20202020u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x37386f6eu32); }
  if !cpu.zf { pc = 0x309e275; continue; }
  pc = 0x309e26e; continue;
 }
 0x309e26e => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x3du8); }
  if !cpu.zf { pc = 0x309e275; continue; }
  pc = 0x309e274; continue;
 }
 0x309e274 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x309e275; continue;
 }
 0x309e275 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.lods(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x309e275; continue; }
  pc = 0x309e27b; continue;
 }
 0x309e27b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x309e260; continue; }
  pc = 0x309e280; continue;
 }
 0x309e280 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x309e283; continue;
 }
 0x309e283 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.movs(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x309e283; continue; }
  pc = 0x309e289; continue;
 }
 0x309e289 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x315be54u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x315be38u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be24u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x32d2060u32; cpu.ecx = v; }
  { let v: u32 = 0x315d440u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x309e2c4; continue; }
  pc = 0x309e2b7; continue;
 }
 0x309e2b7 => {
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x309e2c4; continue; }
  pc = 0x309e2bf; continue;
 }
 0x309e2bf => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  pc = 0x309e2c4; continue;
 }
 0x309e2c4 => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x32d2060u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x315be2cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x315be30u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.flat_ds(0x309e2ef); cpu.push(0x309e2f4); f_030ad7b2(cpu); if cpu.returned_to != 0x309e2f4 { return; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.flat_ds(0x309e2f6); cpu.push(0x309e2fb); f_030ad763(cpu); if cpu.returned_to != 0x309e2fb { return; }
  pc = 0x309e2fb; continue;
 }
 0x309e2fb => {
  pc = 0x309e320; continue;
 }
 0x309e2fd => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x309e0e0u32; cpu.edx = v; }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.df = false; }
  pc = 0x309e311; continue;
 }
 0x309e311 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x309e311; continue; }
  pc = 0x309e316; continue;
 }
 0x309e316 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x309e320; continue;
 }
 0x309e320 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.flat_ds(0x309e32b); cpu.push(0x309e330); f_030ad7fd(cpu); if cpu.returned_to != 0x309e330 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x4cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x309e335); f_0309e338(cpu); return;
 }
 0x30b4a0b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x315d004u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x30b4a0e); cpu.push(0x30b4a14); cpu.call(t); if cpu.returned_to != 0x30b4a14 { return; }
  let t = cpu.r32(0x315d008u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x30b4a14); cpu.push(0x30b4a1a); cpu.call(t); if cpu.returned_to != 0x30b4a1a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.poll(); pc = 0x309e2fb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e338(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e338); }
 let mut pc: u32 = 0x309e338;
 loop { match pc {
 0x309e338 => {
  { let v = cpu.r16(0x309e341u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x309e340); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e341(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e341); } r_0309e341(cpu, 0x309e341); }
pub fn f_0309e343(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e343); } r_0309e341(cpu, 0x309e343); }
pub fn f_0309e350(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e350); } r_0309e341(cpu, 0x309e350); }
fn r_0309e341(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x309e341 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x309e343; continue;
 }
 0x309e343 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315b2a4u32), 0x0u32); }
  if cpu.zf { pc = 0x309e376; continue; }
  pc = 0x309e35c; continue;
 }
 0x309e350 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315b2a4u32), 0x0u32); }
  if cpu.zf { pc = 0x309e376; continue; }
  pc = 0x309e35c; continue;
 }
 0x309e35c => {
  { let v = 0x309e350u32; cpu.push(v); }
  { let v = 0x309e650u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x309e36d); f_030a3800(cpu); if cpu.returned_to != 0x309e36d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315b2a4u32, v); }
  pc = 0x309e376; continue;
 }
 0x309e376 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e380(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e380); }
 let mut pc: u32 = 0x309e380;
 loop { match pc {
 0x309e380 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x315a9e8u32), 0x0u32); }
  if !cpu.zf { pc = 0x309e50f; continue; }
  pc = 0x309e394; continue;
 }
 0x309e394 => {
  { let v: u32 = 0x309e640u32; cpu.ebx = v; }
  { let v: u32 = 0x309e630u32; cpu.ecx = v; }
  { let v: u32 = 0x309e620u32; cpu.esi = v; }
  { let v: u32 = 0x309e600u32; cpu.edi = v; }
  { let v: u32 = 0x309e538u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x315b2a4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315bf40u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x315bf44u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x315bf48u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x315bf4cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x315bf50u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309e3f6; continue; }
  pc = 0x309e3d4; continue;
 }
 0x309e3d4 => {
  { let v = 0x309e350u32; cpu.push(v); }
  { let v = 0x309e650u32; cpu.push(v); }
  cpu.push(0x309e3e3); f_030a38a0(cpu); if cpu.returned_to != 0x309e3e3 { return; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315b2a4u32, v); }
  cpu.push(0x309e3f6); f_0309dea0(cpu); if cpu.returned_to != 0x309e3f6 { return; }
  pc = 0x309e3f6; continue;
 }
 0x309e3f6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x314ae50u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x32d0cd4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32d0cd8u32, v); }
  cpu.push(0x309e40e); f_030ad9b1(cpu); if cpu.returned_to != 0x309e40e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x309e42a; continue; }
  pc = 0x309e416; continue;
 }
 0x309e416 => {
  cpu.push(0x309e41b); f_030a47d0(cpu); if cpu.returned_to != 0x309e41b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x315a9e8u32, v); }
  pc = 0x309e50f; continue;
 }
 0x309e42a => {
  { let v: u32 = 0x314ae5cu32; cpu.eax = v; }
  cpu.push(0x309e434); f_030ad9b1(cpu); if cpu.returned_to != 0x309e434 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x309e444; continue; }
  pc = 0x309e43a; continue;
 }
 0x309e43a => {
  { let v: u32 = 0x1u32; cpu.w32(0x32d0cd8u32, v); }
  pc = 0x309e444; continue;
 }
 0x309e444 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x309e44a); f_030a4552(cpu); if cpu.returned_to != 0x309e44a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0ce0u32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x309e46f; continue; }
  pc = 0x309e457; continue;
 }
 0x309e457 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x309e461); f_030a47d0(cpu); if cpu.returned_to != 0x309e461 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315a9e8u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309e46f => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x309e476); f_03095b88(cpu); if cpu.returned_to != 0x309e476 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x309e47d); f_03095c08(cpu); if cpu.returned_to != 0x309e47d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x32d0d08u32; cpu.edi = v; }
  cpu.push(0x309e48b); f_030adb50(cpu); if cpu.returned_to != 0x309e48b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x309e48f; continue;
 }
 0x309e48f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x309e48f; continue; }
  pc = 0x309e499; continue;
 }
 0x309e499 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = 0x314ae6cu32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x32d0d20u32, v); }
  cpu.push(0x309e4ab); f_0309de70(cpu); if cpu.returned_to != 0x309e4ab { return; }
  { let v = 0x314aec0u32; cpu.push(v); }
  cpu.push(0x309e4b5); f_0309de70(cpu); if cpu.returned_to != 0x309e4b5 { return; }
  { let v = 0x314aefcu32; cpu.push(v); }
  cpu.push(0x309e4bf); f_0309de70(cpu); if cpu.returned_to != 0x309e4bf { return; }
  { let v = 0x32d0d08u32; cpu.push(v); }
  cpu.push(0x309e4c9); f_0309de70(cpu); if cpu.returned_to != 0x309e4c9 { return; }
  { let v = 0x314af0cu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x309e4d8); f_0309de70(cpu); if cpu.returned_to != 0x309e4d8 { return; }
  cpu.push(0x309e4dd); f_030a47d0(cpu); if cpu.returned_to != 0x309e4dd { return; }
  cpu.push(0x309e4e2); f_030a3a9e(cpu); if cpu.returned_to != 0x309e4e2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cdcu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0cd0u32, v); }
  cpu.push(0x309e4f8); f_0309daa0(cpu); if cpu.returned_to != 0x309e4f8 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x314af64u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0cd0u32, v); }
  cpu.push(0x309e509); f_0309de70(cpu); if cpu.returned_to != 0x309e509 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x315a9e8u32, v); }
  pc = 0x309e50f; continue;
 }
 0x309e50f => {
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
pub fn f_0309e520(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e520); } r_0309e520(cpu, 0x309e520); }
pub fn f_0309e538(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309e538); } r_0309e520(cpu, 0x309e538); }
fn r_0309e520(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x309e520 => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u32 = cpu.port_in(0x9u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x9u32.wrapping_add(cpu.ebp))); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0xffffffe5u32.wrapping_add(cpu.edi))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  { let v: u32 = cpu.port_in(0x9u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0xd00309e5u32.wrapping_add(cpu.ecx))); cpu.esi = v; }
  { let v: u32 = cpu.port_in(0x9u16, 4) as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x57u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x309e5ed; continue; }
  pc = 0x309e554; continue;
 }
 0x309e538 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x309e5ed; continue; }
  pc = 0x309e554; continue;
 }
 0x309e554 => {
  cpu.poll(); pc = cpu.r32(0x309e520u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x309e55c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x309e563); f_0308d652(cpu); if cpu.returned_to != 0x309e563 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x309e56c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x309e576); f_0308d652(cpu); if cpu.returned_to != 0x309e576 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x309e57f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x309e58d); f_0308d652(cpu); if cpu.returned_to != 0x309e58d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x309e596 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x309e5a8); f_0308d652(cpu); if cpu.returned_to != 0x309e5a8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x309e5b1 => {
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
  cpu.push(0x309e5c7); f_0308d652(cpu); if cpu.returned_to != 0x309e5c7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x309e5d0 => {
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
  cpu.push(0x309e5ea); f_0308d652(cpu); if cpu.returned_to != 0x309e5ea { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x309e5ed; continue;
 }
 0x309e5ed => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e600); }
 let mut pc: u32 = 0x309e600;
 loop { match pc {
 0x309e600 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x309e611); f_03095c28(cpu); if cpu.returned_to != 0x309e611 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e620); }
 let mut pc: u32 = 0x309e620;
 loop { match pc {
 0x309e620 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x309e629); f_030ad9b1(cpu); if cpu.returned_to != 0x309e629 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e630); }
 let mut pc: u32 = 0x309e630;
 loop { match pc {
 0x309e630 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x309e639); f_030ad92f(cpu); if cpu.returned_to != 0x309e639 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e640); }
 let mut pc: u32 = 0x309e640;
 loop { match pc {
 0x309e640 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x309e649); f_030ad84c(cpu); if cpu.returned_to != 0x309e649 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e650); }
 let mut pc: u32 = 0x309e650;
 loop { match pc {
 0x309e650 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x315b2a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x309e67e; continue; }
  pc = 0x309e65c; continue;
 }
 0x309e65c => {
  { let v = 0x309e350u32; cpu.push(v); }
  { let v = 0x309e650u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x309e670); f_030a38a0(cpu); if cpu.returned_to != 0x309e670 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315b2a4u32, v); }
  cpu.push(0x309e67e); f_0309dea0(cpu); if cpu.returned_to != 0x309e67e { return; }
  pc = 0x309e67e; continue;
 }
 0x309e67e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e682(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e682); }
 let mut pc: u32 = 0x309e682;
 loop { match pc {
 0x309e682 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x315b2a8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e68a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e68a); }
 let mut pc: u32 = 0x309e68a;
 loop { match pc {
 0x309e68a => {
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x309e693); f_0309e69d(cpu); if cpu.returned_to != 0x309e693 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e69d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e69d); }
 let mut pc: u32 = 0x309e69d;
 loop { match pc {
 0x309e69d => {
  { cpu.sub32(cpu.eax, cpu.esp); }
  if !cpu.cf { pc = 0x309e6ae; continue; }
  pc = 0x309e6a1; continue;
 }
 0x309e6a1 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x315be38u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x309e6ae; continue; }
  pc = 0x309e6ad; continue;
 }
 0x309e6ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309e6ae => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x315b2a8u32)); }
  if cpu.zf { f_0309e6bb(cpu); return; }
  pc = 0x309e6ba; continue;
 }
 0x309e6ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e6bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e6bb); }
 let mut pc: u32 = 0x309e6bb;
 loop { match pc {
 0x309e6bb => {
  { let v: u32 = 0x315b2aau32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x309e6ca); f_030adb62(cpu); if cpu.returned_to != 0x309e6ca { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_0309e6d0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e6d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e6d0); }
 let mut pc: u32 = 0x309e6d0;
 loop { match pc {
 0x309e6d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x32d0fbcu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x32d0fbcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309e721); f_03095c28(cpu); if cpu.returned_to != 0x309e721 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.r32(0x32d0fbcu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e734(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e734); }
 let mut pc: u32 = 0x309e734;
 loop { match pc {
 0x309e734 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309e765); f_03095c28(cpu); if cpu.returned_to != 0x309e765 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e774(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e774); }
 let mut pc: u32 = 0x309e774;
 loop { match pc {
 0x309e774 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309e7bc); f_03095c28(cpu); if cpu.returned_to != 0x309e7bc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e7cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e7cc); }
 let mut pc: u32 = 0x309e7cc;
 loop { match pc {
 0x309e7cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309e814); f_03095c28(cpu); if cpu.returned_to != 0x309e814 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e824(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e824); }
 let mut pc: u32 = 0x309e824;
 loop { match pc {
 0x309e824 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309e855); f_03095c28(cpu); if cpu.returned_to != 0x309e855 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309e860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e860); }
 let mut pc: u32 = 0x309e860;
 loop { match pc {
 0x309e860 => {
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
  pc = 0x309e933; continue;
 }
 0x309e87d => {
  { let v: u32 = cpu.r32(0x32d0fa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32d0f2cu32); cpu.ebp = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x32d0fa0u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x32d0fa0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d0fb8u32); cpu.ebp = v; }
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
  { let v: u16 = cpu.r16(0x315b2beu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x12u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  cpu.push(0x309e927); f_0309e774(cpu); if cpu.returned_to != 0x309e927 { return; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1eu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32d0fdau32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x309e933; continue;
 }
 0x309e933 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x315b2bcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x309e87d; continue; }
  pc = 0x309e943; continue;
 }
 0x309e943 => {
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
pub fn f_0309e950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309e950); }
 let mut pc: u32 = 0x309e950;
 loop { match pc {
 0x309e950 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x32d0fb8u32); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x315b2c0u32), 0x0u16); }
  if !cpu.zf { pc = 0x309ebea; continue; }
  pc = 0x309e972; continue;
 }
 0x309e972 => {
  { let v: u32 = 0x31300b5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x3130000u32); cpu.ebx = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x309e989); f_0308bfbc(cpu); if cpu.returned_to != 0x309e989 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fc4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309e9ab; continue; }
  pc = 0x309e992; continue;
 }
 0x309e992 => {
  { let v = 0x314af74u32; cpu.push(v); }
  { let v = 0x314afacu32; cpu.push(v); }
  cpu.push(0x309e9a1); f_03094c88(cpu); if cpu.returned_to != 0x309e9a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309ebef; continue;
 }
 0x309e9ab => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x3130000u32; cpu.eax = v; }
  cpu.push(0x309e9b7); f_0308d178(cpu); if cpu.returned_to != 0x309e9b7 { return; }
  { let v: u32 = 0x3130008u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x3130000u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x60u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fc8u32, v); }
  pc = 0x309e9d0; continue;
 }
 0x309e9d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x32d0fd4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fd0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309e9fe; continue; }
  pc = 0x309e9f9; continue;
 }
 0x309e9f9 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x309ea0d; continue; }
  pc = 0x309e9fe; continue;
 }
 0x309e9fe => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0xcfu8); }
  if cpu.zf { pc = 0x309ea0d; continue; }
  pc = 0x309ea06; continue;
 }
 0x309ea06 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x66u8); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x309e9d0; continue; }
  pc = 0x309ea0d; continue;
 }
 0x309ea0d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x32d0fdbu32, v); }
  { cpu.sub8((cpu.ecx as u8), 0x67u8); }
  if !cpu.zf { pc = 0x309ea3b; continue; }
  pc = 0x309ea18; continue;
 }
 0x309ea18 => {
  { let v = 0x314afb8u32; cpu.push(v); }
  { let v = 0x314afacu32; cpu.push(v); }
  cpu.push(0x309ea27); f_03094c88(cpu); if cpu.returned_to != 0x309ea27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309ea3b => {
  { let v: u32 = 0x200u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x309ea55); f_03095c28(cpu); if cpu.returned_to != 0x309ea55 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x32d0fd6u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x32d0fd8u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x32d0fc8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x32d0fc8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x309eaaf); f_03095c28(cpu); if cpu.returned_to != 0x309eaaf { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x315b2bcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x315b2c0u32, v); }
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
  { let v: u16 = cpu.r16(0x315b2beu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  cpu.push(0x309eb0e); f_0308bfbc(cpu); if cpu.returned_to != 0x309eb0e { return; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fc0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309eb40; continue; }
  pc = 0x309eb1d; continue;
 }
 0x309eb1d => {
  { let v = 0x314aff8u32; cpu.push(v); }
  { let v = 0x314afacu32; cpu.push(v); }
  cpu.push(0x309eb2c); f_03094c88(cpu); if cpu.returned_to != 0x309eb2c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309eb40 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x315b2bcu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0f2cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fa8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0facu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fa0u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fa4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1eu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x309eba8; continue;
 }
 0x309eb91 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x315b2beu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x12u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x309eba8; continue;
 }
 0x309eba8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x315b2bcu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x309eb91; continue; }
  pc = 0x309ebb5; continue;
 }
 0x309ebb5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x315b2beu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x32d0fdbu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fbcu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x32d0facu32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x32d0fb8u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x32d0f93u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x309ebea; continue;
 }
 0x309ebea => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309ebef; continue;
 }
 0x309ebef => {
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
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
pub fn f_0309ec00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ec00); }
 let mut pc: u32 = 0x309ec00;
 loop { match pc {
 0x309ec00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x315b2c0u32), 0x0u16); }
  if cpu.zf { pc = 0x309ec6a; continue; }
  pc = 0x309ec0f; continue;
 }
 0x309ec0f => {
  { let v: u32 = cpu.r32(0x32d0fc0u32); cpu.eax = v; }
  cpu.push(0x309ec19); f_0308c640(cpu); if cpu.returned_to != 0x309ec19 { return; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  cpu.push(0x309ec23); f_0308c640(cpu); if cpu.returned_to != 0x309ec23 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x32d0fd6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x32d0fd8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x309ec5c); f_03095c28(cpu); if cpu.returned_to != 0x309ec5c { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x315b2c0u32, v); }
  pc = 0x309ec6a; continue;
 }
 0x309ec6a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309ec70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ec70); }
 let mut pc: u32 = 0x309ec70;
 loop { match pc {
 0x309ec70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309ec9e); f_03095c28(cpu); if cpu.returned_to != 0x309ec9e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309ecac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ecac); }
 let mut pc: u32 = 0x309ecac;
 loop { match pc {
 0x309ecac => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x32d0f2cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x32d0f2cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309ed0d); f_03095c28(cpu); if cpu.returned_to != 0x309ed0d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.r32(0x32d0f2cu32); cpu.eax = v; }
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
pub fn f_0309ed28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ed28); }
 let mut pc: u32 = 0x309ed28;
 loop { match pc {
 0x309ed28 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x32d0f2cu32), 0x0u32); }
  if cpu.zf { pc = 0x309ed7c; continue; }
  pc = 0x309ed36; continue;
 }
 0x309ed36 => {
  { let v: u32 = cpu.r32(0x32d0fc4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x32d0f2cu32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x32d0fdbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x309ed6f); f_03095c28(cpu); if cpu.returned_to != 0x309ed6f { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x32d0fb0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x32d0f2cu32, v); }
  pc = 0x309ed7c; continue;
 }
 0x309ed7c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309ed84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ed84); }
 let mut pc: u32 = 0x309ed84;
 loop { match pc {
 0x309ed84 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x32d0facu32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32d0fb4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  cpu.push(0x309eda8); f_0308d178(cpu); if cpu.returned_to != 0x309eda8 { return; }
  { let v: u32 = cpu.r32(0x32d0fb4u32); cpu.esi = v; }
  cpu.push(0x309edb3); f_0308f9d0(cpu); if cpu.returned_to != 0x309edb3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x32d0f93u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x32d0facu32); cpu.ebp = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32d0f2cu32); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32d0fa4u32); cpu.esi = v; }
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
  pc = 0x309ee0a; continue;
 }
 0x309ee0a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1bu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x309ee0a; continue; }
  pc = 0x309ee15; continue;
 }
 0x309ee15 => {
  { let v: u32 = cpu.r32(0x32d0fa4u32); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x32d0fb4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x32d0fa4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x32d0fb4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0facu32, v); }
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
  cpu.push(0x309ee7f); f_0309e7cc(cpu); if cpu.returned_to != 0x309ee7f { return; }
  { let v: u32 = cpu.r32(0x32d0facu32); cpu.ebp = v; }
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
pub fn f_0309ee90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309ee90); }
 let mut pc: u32 = 0x309ee90;
 loop { match pc {
 0x309ee90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x32d0fb8u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7fffffffu32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x309eee1; continue;
 }
 0x309eebb => {
  { let v: u32 = cpu.r32(0x32d0fa8u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x309eeda; continue; }
  pc = 0x309eec8; continue;
 }
 0x309eec8 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x309eeda; continue; }
  pc = 0x309eed4; continue;
 }
 0x309eed4 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x309eeda; continue;
 }
 0x309eeda => {
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x309eee1; continue;
 }
 0x309eee1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x315b2bcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x309eebb; continue; }
  pc = 0x309eeee; continue;
 }
 0x309eeee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x309effc; continue; }
  pc = 0x309eefb; continue;
 }
 0x309eefb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x12u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x309ef0f; continue; }
  pc = 0x309ef0d; continue;
 }
 0x309ef0d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x309ef0f; continue;
 }
 0x309ef0f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x309ef2d; continue; }
  pc = 0x309ef16; continue;
 }
 0x309ef16 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32d0fa0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x309ef2d; continue;
 }
 0x309ef2d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32d0fa8u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x32d0f2cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x1eu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x32d0fa0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x32d0fa0u32); cpu.ebx = v; }
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
  { let v: u16 = cpu.r16(0x315b2beu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0fb8u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x309efe0); f_0309e774(cpu); if cpu.returned_to != 0x309efe0 { return; }
  { let v: u32 = cpu.r32(0x32d0fb8u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x32d0fdau32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0fb8u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309effc => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x32d0fb8u32, v); }
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
pub fn f_0309f010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f010); }
 let mut pc: u32 = 0x309f010;
 loop { match pc {
 0x309f010 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f040; continue; }
  pc = 0x309f014; continue;
 }
 0x309f014 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x309f016; continue;
 }
 0x309f016 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x309f023; continue; }
  pc = 0x309f01a; continue;
 }
 0x309f01a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x309f016; continue; }
  pc = 0x309f023; continue;
 }
 0x309f023 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x309f02c); f_0309f047(cpu); if cpu.returned_to != 0x309f02c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f040; continue; }
  pc = 0x309f032; continue;
 }
 0x309f032 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f040; continue; }
  pc = 0x309f037; continue;
 }
 0x309f037 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f040; continue; }
  pc = 0x309f03d; continue;
 }
 0x309f03d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x309f040; continue;
 }
 0x309f040 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f047(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f047); }
 let mut pc: u32 = 0x309f047;
 loop { match pc {
 0x309f047 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f0b2; continue; }
  pc = 0x309f04b; continue;
 }
 0x309f04b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x309f057; continue; }
  pc = 0x309f04f; continue;
 }
 0x309f04f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x309f04b; continue; }
  pc = 0x309f057; continue;
 }
 0x309f057 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f097; continue; }
  pc = 0x309f05d; continue;
 }
 0x309f05d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f089; continue; }
  pc = 0x309f060; continue;
 }
 0x309f060 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x309f086; continue; }
  pc = 0x309f06e; continue;
 }
 0x309f06e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x309f060; continue; }
  pc = 0x309f083; continue;
 }
 0x309f083 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x309f086; continue;
 }
 0x309f086 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x309f089; continue;
 }
 0x309f089 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x309f097; continue;
 }
 0x309f097 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f0b2; continue; }
  pc = 0x309f09d; continue;
 }
 0x309f09d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f0b2; continue; }
  pc = 0x309f0a5; continue;
 }
 0x309f0a5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x309f0b2; continue; }
  pc = 0x309f0ad; continue;
 }
 0x309f0ad => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x309f0b2; continue;
 }
 0x309f0b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f0c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f0c0); }
 let mut pc: u32 = 0x309f0c0;
 loop { match pc {
 0x309f0c0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x309f0d5); f_0309f010(cpu); if cpu.returned_to != 0x309f0d5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f0d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f0d8); }
 let mut pc: u32 = 0x309f0d8;
 loop { match pc {
 0x309f0d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x309f0ee; continue; }
  pc = 0x309f0e1; continue;
 }
 0x309f0e1 => {
  { cpu.interrupt(0x1a); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x309f103; continue;
 }
 0x309f0ee => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  if !cpu.zf { pc = 0x309f103; continue; }
  pc = 0x309f0f6; continue;
 }
 0x309f0f6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x1a); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x309f103; continue;
 }
 0x309f103 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f107(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f107); }
 let mut pc: u32 = 0x309f107;
 loop { match pc {
 0x309f107 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x314b028u32; cpu.eax = v; }
  cpu.push(0x309f11c); f_030ad9b1(cpu); if cpu.returned_to != 0x309f11c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x309f129; continue; }
  pc = 0x309f122; continue;
 }
 0x309f122 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x309f160; continue;
 }
 0x309f129 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x309f134; continue; }
  pc = 0x309f12f; continue;
 }
 0x309f12f => {
  { let v: u32 = 0x314b038u32; cpu.ebx = v; }
  pc = 0x309f134; continue;
 }
 0x309f134 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x309f143); f_030adb7f(cpu); if cpu.returned_to != 0x309f143 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x309f14f; continue; }
  pc = 0x309f148; continue;
 }
 0x309f148 => {
  { let v: u32 = 0x314b044u32; cpu.eax = v; }
  pc = 0x309f154; continue;
 }
 0x309f14f => {
  { let v: u32 = 0x314b048u32; cpu.eax = v; }
  pc = 0x309f154; continue;
 }
 0x309f154 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x309f15d); f_030adf0a(cpu); if cpu.returned_to != 0x309f15d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  pc = 0x309f160; continue;
 }
 0x309f160 => {
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
pub fn f_0309f167(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f167); }
 let mut pc: u32 = 0x309f167;
 loop { match pc {
 0x309f167 => {
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
pub fn f_0309f18c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f18c); }
 let mut pc: u32 = 0x309f18c;
 loop { match pc {
 0x309f18c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x315be56u32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x309f1a9; continue; }
  pc = 0x309f19b; continue;
 }
 0x309f19b => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x309f1a9; continue; }
  pc = 0x309f1a0; continue;
 }
 0x309f1a0 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2502u32; cpu.eax = v; }
  pc = 0x309f1b3; continue;
 }
 0x309f1a9 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x309f1b3; continue;
 }
 0x309f1b3 => {
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
pub fn f_0309f1be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f1be); }
 let mut pc: u32 = 0x309f1be;
 loop { match pc {
 0x309f1be => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x309f1e1; continue; }
  pc = 0x309f1ce; continue;
 }
 0x309f1ce => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x309f1e1; continue; }
  pc = 0x309f1d3; continue;
 }
 0x309f1d3 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x309f1e0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309f1e1 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x309f1ea); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f1eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f1eb); }
 let mut pc: u32 = 0x309f1eb;
 loop { match pc {
 0x309f1eb => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f1ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f1ed); }
 let mut pc: u32 = 0x309f1ed;
 loop { match pc {
 0x309f1ed => {
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
pub fn f_0309f1f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f1f7); }
 let mut pc: u32 = 0x309f1f7;
 loop { match pc {
 0x309f1f7 => {
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
pub fn f_0309f1ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f1ff); }
 let mut pc: u32 = 0x309f1ff;
 loop { match pc {
 0x309f1ff => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f201(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f201); }
 let mut pc: u32 = 0x309f201;
 loop { match pc {
 0x309f201 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x309f213); f_030adf20(cpu); if cpu.returned_to != 0x309f213 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f21a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f21a); }
 let mut pc: u32 = 0x309f21a;
 loop { match pc {
 0x309f21a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x309f235; continue; }
  pc = 0x309f229; continue;
 }
 0x309f229 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x309f235; continue;
 }
 0x309f235 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f239(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f239); }
 let mut pc: u32 = 0x309f239;
 loop { match pc {
 0x309f239 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x309f21au32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x309f259); f_0309f568(cpu); if cpu.returned_to != 0x309f259 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f262(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f262); }
 let mut pc: u32 = 0x309f262;
 loop { match pc {
 0x309f262 => {
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
  cpu.push(0x309f282); f_0309f239(cpu); if cpu.returned_to != 0x309f282 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f289(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f289); }
 let mut pc: u32 = 0x309f289;
 loop { match pc {
 0x309f289 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x309f2a4); f_030a1efd(cpu); if cpu.returned_to != 0x309f2a4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f33a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f33a); }
 let mut pc: u32 = 0x309f33a;
 loop { match pc {
 0x309f33a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x309f348; continue;
 }
 0x309f348 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x309f36c; continue; }
  pc = 0x309f34e; continue;
 }
 0x309f34e => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x309f348; continue;
 }
 0x309f36c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x43300000u32; cpu.edi = v; }
  cpu.push(0x309f378); f_0309f385(cpu); if cpu.returned_to != 0x309f378 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f385(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f385); }
 let mut pc: u32 = 0x309f385;
 loop { match pc {
 0x309f385 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.edx); cpu.esi = v; }
  if cpu.zf { pc = 0x309f3d5; continue; }
  pc = 0x309f38d; continue;
 }
 0x309f38d => {
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if !cpu.zf { pc = 0x309f3a9; continue; }
  pc = 0x309f395; continue;
 }
 0x309f395 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if cpu.zf { cpu.poll(); pc = 0x309f395; continue; }
  pc = 0x309f3a7; continue;
 }
 0x309f3a7 => {
  pc = 0x309f3cd; continue;
 }
 0x309f3a9 => {
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if cpu.zf { pc = 0x309f3cd; continue; }
  pc = 0x309f3b1; continue;
 }
 0x309f3b1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x309f3b1; continue; }
  pc = 0x309f3c5; continue;
 }
 0x309f3c5 => {
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x309f3cd; continue;
 }
 0x309f3cd => {
  { let v: u32 = cpu.edx & 0xfffffu32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edi); cpu.edx = v; }
  pc = 0x309f3d5; continue;
 }
 0x309f3d5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f3d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f3d6); }
 let mut pc: u32 = 0x309f3d6;
 loop { match pc {
 0x309f3d6 => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_0309f3db(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f3db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f3db); }
 let mut pc: u32 = 0x309f3db;
 loop { match pc {
 0x309f3db => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x309f3e3); f_030a1ac1(cpu); if cpu.returned_to != 0x309f3e3 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f3e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f3e7); }
 let mut pc: u32 = 0x309f3e7;
 loop { match pc {
 0x309f3e7 => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_0309f3db(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f3ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f3ee); }
 let mut pc: u32 = 0x309f3ee;
 loop { match pc {
 0x309f3ee => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x309f3f8); f_0309f3db(cpu); if cpu.returned_to != 0x309f3f8 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f3fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f3fe); }
 let mut pc: u32 = 0x309f3fe;
 loop { match pc {
 0x309f3fe => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x309f406); f_030a1ac7(cpu); if cpu.returned_to != 0x309f406 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f40a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f40a); }
 let mut pc: u32 = 0x309f40a;
 loop { match pc {
 0x309f40a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x309f430; continue; }
  pc = 0x309f41b; continue;
 }
 0x309f41b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x309f42d); f_0309f44b(cpu); if cpu.returned_to != 0x309f42d { return; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x309f430; continue;
 }
 0x309f430 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x309f43a); f_030ae338(cpu); if cpu.returned_to != 0x309f43a { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x309f443); f_030ae2fe(cpu); if cpu.returned_to != 0x309f443 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f44b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f44b); }
 let mut pc: u32 = 0x309f44b;
 loop { match pc {
 0x309f44b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x314b9acu32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x309f476; continue; }
  pc = 0x309f466; continue;
 }
 0x309f466 => {
  { let v: u32 = cpu.r32(0x314b9bcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x314b9c0u32); cpu.eax = v; }
  pc = 0x309f493; continue;
 }
 0x309f476 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x314b9b4u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x309f497; continue; }
  pc = 0x309f485; continue;
 }
 0x309f485 => {
  { let v: u32 = cpu.r32(0x314b9c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x314b9c8u32); cpu.eax = v; }
  pc = 0x309f493; continue;
 }
 0x309f493 => {
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x309f497; continue;
 }
 0x309f497 => {
  { let v: u32 = 0x314ba4cu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x309f4da; continue; }
  pc = 0x309f4a0; continue;
 }
 0x309f4a0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x309f4f8; continue; }
  pc = 0x309f4ac; continue;
 }
 0x309f4ac => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x315bf28u32), 0x1u8); }
  if cpu.zf { pc = 0x309f4cc; continue; }
  pc = 0x309f4b9; continue;
 }
 0x309f4b9 => {
  { let v = cpu.r32(0x314ba52u32); cpu.push(v); }
  { let v = cpu.r32(0x314ba4eu32); cpu.push(v); }
  cpu.push(0x309f4ca); f_030a344c(cpu); if cpu.returned_to != 0x309f4ca { return; }
  pc = 0x309f4d2; continue;
 }
 0x309f4cc => {
  { let v = cpu.st(0) / cpu.rf64(0x314ba4eu32); cpu.set_st(0, v); }
  pc = 0x309f4d2; continue;
 }
 0x309f4d2 => {
  { let v: u32 = cpu.edx.wrapping_sub(0xd8u32); cpu.edx = v; }
  pc = 0x309f4f4; continue;
 }
 0x309f4da => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x309f4f8; continue; }
  pc = 0x309f4e4; continue;
 }
 0x309f4e4 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x314ba4eu32); cpu.set_st(0, v); }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x309f4f4; continue;
 }
 0x309f4f4 => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x309f4f8; continue;
 }
 0x309f4f8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x309f506; continue;
 }
 0x309f506 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x309f518; continue; }
  pc = 0x309f50d; continue;
 }
 0x309f50d => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x309f518; continue;
 }
 0x309f518 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309f529; continue; }
  pc = 0x309f51c; continue;
 }
 0x309f51c => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x309f506; continue; }
  pc = 0x309f524; continue;
 }
 0x309f524 => {
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.poll(); pc = 0x309f506; continue;
 }
 0x309f529 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x309f54e; continue; }
  pc = 0x309f52d; continue;
 }
 0x309f52d => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x315bf28u32), 0x1u8); }
  if cpu.zf { pc = 0x309f549; continue; }
  pc = 0x309f53a; continue;
 }
 0x309f53a => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x309f547); f_030a344c(cpu); if cpu.returned_to != 0x309f547 { return; }
  pc = 0x309f555; continue;
 }
 0x309f549 => {
  { let v = cpu.st(0) / cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x309f555; continue;
 }
 0x309f54e => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x309f555; continue;
 }
 0x309f555 => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f568(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f568); }
 let mut pc: u32 = 0x309f568;
 loop { match pc {
 0x309f568 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x124u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x120u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  pc = 0x309f63a; continue;
 }
 0x309f5a7 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x309f5b5); f_0309f907(cpu); if cpu.returned_to != 0x309f5b5 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x115u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x309f8f5; continue; }
  pc = 0x309f5c9; continue;
 }
 0x309f5c9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x309f704; continue; }
  pc = 0x309f5db; continue;
 }
 0x309f5db => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309f671; continue; }
  pc = 0x309f603; continue;
 }
 0x309f603 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309f621; continue; }
  pc = 0x309f60b; continue;
 }
 0x309f60b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x309f615; continue;
 }
 0x309f615 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x309f63a; continue;
 }
 0x309f621 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309f666; continue; }
  pc = 0x309f628; continue;
 }
 0x309f628 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x309f631; continue;
 }
 0x309f631 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x309f63a; continue;
 }
 0x309f63a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x309f8f5; continue; }
  pc = 0x309f644; continue;
 }
 0x309f644 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x25u32); }
  if cpu.zf { cpu.poll(); pc = 0x309f5a7; continue; }
  pc = 0x309f654; continue;
 }
 0x309f654 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f664); cpu.call(t); if cpu.returned_to != 0x309f664 { return; }
  cpu.poll(); pc = 0x309f63a; continue;
 }
 0x309f666 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x309f631; continue;
 }
 0x309f671 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309f6cc; continue; }
  pc = 0x309f683; continue;
 }
 0x309f683 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309f6a2; continue; }
  pc = 0x309f68b; continue;
 }
 0x309f68b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.eax); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x309f63a; continue;
 }
 0x309f6a2 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309f6c1; continue; }
  pc = 0x309f6a9; continue;
 }
 0x309f6a9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x309f6b2; continue;
 }
 0x309f6b2 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x309f63a; continue;
 }
 0x309f6c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.poll(); pc = 0x309f6b2; continue;
 }
 0x309f6cc => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309f6e1; continue; }
  pc = 0x309f6d2; continue;
 }
 0x309f6d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  cpu.poll(); pc = 0x309f615; continue;
 }
 0x309f6e1 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309f6f6; continue; }
  pc = 0x309f6e8; continue;
 }
 0x309f6e8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.poll(); pc = 0x309f631; continue;
 }
 0x309f6f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.poll(); pc = 0x309f631; continue;
 }
 0x309f704 => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x309f71b); f_0309fc45(cpu); if cpu.returned_to != 0x309f71b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x309f767; continue; }
  pc = 0x309f72f; continue;
 }
 0x309f72f => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x309f767; continue; }
  pc = 0x309f73b; continue;
 }
 0x309f73b => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x309f767; continue; }
  pc = 0x309f74e; continue;
 }
 0x309f74e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f765); cpu.call(t); if cpu.returned_to != 0x309f765 { return; }
  cpu.poll(); pc = 0x309f73b; continue;
 }
 0x309f767 => {
  { let v: u32 = 0x117u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x309f76e; continue;
 }
 0x309f76e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x309f789; continue; }
  pc = 0x309f774; continue;
 }
 0x309f774 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f787); cpu.call(t); if cpu.returned_to != 0x309f787 { return; }
  cpu.poll(); pc = 0x309f76e; continue;
 }
 0x309f789 => {
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309f7b3; continue; }
  pc = 0x309f79e; continue;
 }
 0x309f79e => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f7b1); cpu.call(t); if cpu.returned_to != 0x309f7b1 { return; }
  cpu.poll(); pc = 0x309f789; continue;
 }
 0x309f7b3 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x309f7fb; continue; }
  pc = 0x309f7c3; continue;
 }
 0x309f7c3 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.zf { pc = 0x309f7fb; continue; }
  pc = 0x309f7cf; continue;
 }
 0x309f7cf => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x309f7fb; continue; }
  pc = 0x309f7e2; continue;
 }
 0x309f7e2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f7f9); cpu.call(t); if cpu.returned_to != 0x309f7f9 { return; }
  cpu.poll(); pc = 0x309f7cf; continue;
 }
 0x309f7fb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.zf { pc = 0x309f80e; continue; }
  pc = 0x309f809; continue;
 }
 0x309f809 => {
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x309f87a; continue; }
  pc = 0x309f80e; continue;
 }
 0x309f80e => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309f84f; continue; }
  pc = 0x309f81e; continue;
 }
 0x309f81e => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309f8a5; continue; }
  pc = 0x309f837; continue;
 }
 0x309f837 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f84a); cpu.call(t); if cpu.returned_to != 0x309f84a { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x309f81e; continue;
 }
 0x309f84f => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309f8a5; continue; }
  pc = 0x309f864; continue;
 }
 0x309f864 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f877); cpu.call(t); if cpu.returned_to != 0x309f877 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x309f84f; continue;
 }
 0x309f87a => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x309f8a5; continue; }
  pc = 0x309f88f; continue;
 }
 0x309f88f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f8a2); cpu.call(t); if cpu.returned_to != 0x309f8a2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x309f87a; continue;
 }
 0x309f8a5 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x309f63a; continue; }
  pc = 0x309f8b9; continue;
 }
 0x309f8b9 => {
  { cpu.sub32(cpu.r32(0x104u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x309f63a; continue; }
  pc = 0x309f8c7; continue;
 }
 0x309f8c7 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x309f63a; continue; }
  pc = 0x309f8e0; continue;
 }
 0x309f8e0 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x309f8f3); cpu.call(t); if cpu.returned_to != 0x309f8f3 { return; }
  cpu.poll(); pc = 0x309f8c7; continue;
 }
 0x309f8f5 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x124u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309f907(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309f907); }
 let mut pc: u32 = 0x309f907;
 loop { match pc {
 0x309f907 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x309f91b); f_0309fa45(cpu); if cpu.returned_to != 0x309f91b { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x309f955; continue; }
  pc = 0x309f92f; continue;
 }
 0x309f92f => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x309f955; continue; }
  pc = 0x309f934; continue;
 }
 0x309f934 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x309f980; continue; }
  pc = 0x309f94e; continue;
 }
 0x309f94e => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x309f934; continue; }
  pc = 0x309f953; continue;
 }
 0x309f953 => {
  pc = 0x309f980; continue;
 }
 0x309f955 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x309f980; continue; }
  pc = 0x309f95e; continue;
 }
 0x309f95e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x309f97f; continue; }
  pc = 0x309f96f; continue;
 }
 0x309f96f => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8) | 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x309f97f; continue;
 }
 0x309f97f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x309f980; continue;
 }
 0x309f980 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x309f9d9; continue; }
  pc = 0x309f990; continue;
 }
 0x309f990 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x309f9bc; continue; }
  pc = 0x309f9a1; continue;
 }
 0x309f9a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x309f9b9; continue; }
  pc = 0x309f9b2; continue;
 }
 0x309f9b2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x309f9b9; continue;
 }
 0x309f9b9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x309f9d9; continue;
 }
 0x309f9bc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x309f9d9; continue; }
  pc = 0x309f9c5; continue;
 }
 0x309f9c5 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x309f9d9; continue; }
  pc = 0x309f9ca; continue;
 }
 0x309f9ca => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x309f9bc; continue;
 }
 0x309f9d9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4eu8); }
  if cpu.cf { pc = 0x309f9fe; continue; }
  pc = 0x309f9e3; continue;
 }
 0x309f9e3 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fa3b; continue; }
  pc = 0x309f9e5; continue;
 }
 0x309f9e5 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x309f9f5; continue; }
  pc = 0x309f9ea; continue;
 }
 0x309f9ea => {
  if (cpu.cf || cpu.zf) { pc = 0x309fa0e; continue; }
  pc = 0x309f9ec; continue;
 }
 0x309f9ec => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x309fa0e; continue; }
  pc = 0x309f9f1; continue;
 }
 0x309f9f1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309f9f5 => {
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.zf { pc = 0x309fa1c; continue; }
  pc = 0x309f9fa; continue;
 }
 0x309f9fa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309f9fe => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x309fa41; continue; }
  pc = 0x309fa03; continue;
 }
 0x309fa03 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fa31; continue; }
  pc = 0x309fa05; continue;
 }
 0x309fa05 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x309fa22; continue; }
  pc = 0x309fa0a; continue;
 }
 0x309fa0a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309fa0e => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x309fa15; continue;
 }
 0x309fa15 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309fa1c => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x10u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x309fa3f; continue;
 }
 0x309fa22 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309fa31 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.poll(); pc = 0x309fa15; continue;
 }
 0x309fa3b => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x309fa3f; continue;
 }
 0x309fa3f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x309fa41; continue;
 }
 0x309fa41 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309fa45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309fa45); }
 let mut pc: u32 = 0x309fa45;
 loop { match pc {
 0x309fa45 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x309fa4d; continue;
 }
 0x309fa4d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x309fa5c; continue; }
  pc = 0x309fa56; continue;
 }
 0x309fa56 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x309fa9c; continue;
 }
 0x309fa5c => {
  { cpu.sub32(cpu.eax, 0x23u32); }
  if !cpu.zf { pc = 0x309fa67; continue; }
  pc = 0x309fa61; continue;
 }
 0x309fa61 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x309fa9c; continue;
 }
 0x309fa67 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x309fa7c; continue; }
  pc = 0x309fa6c; continue;
 }
 0x309fa6c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x309fa9c; continue;
 }
 0x309fa7c => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x309fa93; continue; }
  pc = 0x309fa81; continue;
 }
 0x309fa81 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x309fa9c; continue; }
  pc = 0x309fa8d; continue;
 }
 0x309fa8d => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x2u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x309fa9c; continue;
 }
 0x309fa93 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x309fa9f; continue; }
  pc = 0x309fa98; continue;
 }
 0x309fa98 => {
  { let v: u8 = 0x30u8; cpu.w8(0x16u32.wrapping_add(cpu.edx), v); }
  pc = 0x309fa9c; continue;
 }
 0x309fa9c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x309fa4d; continue;
 }
 0x309fa9f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309faa4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309faa4); } r_0309faa4(cpu, 0x309faa4); }
pub fn f_0309fac8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x309fac8); } r_0309faa4(cpu, 0x309fac8); }
fn r_0309faa4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x309faa4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309fab2; continue;
 }
 0x309fab2 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x309fac3; continue; }
  pc = 0x309fabc; continue;
 }
 0x309fabc => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x309fac3; continue; }
  pc = 0x309fac0; continue;
 }
 0x309fac0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x309fab2; continue;
 }
 0x309fac3 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x309fac8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x309fad6; continue;
 }
 0x309fad6 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x309fac3; continue; }
  pc = 0x309fae4; continue;
 }
 0x309fae4 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x309fac3; continue; }
  pc = 0x309fae8; continue;
 }
 0x309fae8 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x309fad6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309faeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309faeb); }
 let mut pc: u32 = 0x309faeb;
 loop { match pc {
 0x309faeb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x309fb03); f_0309e014(cpu); if cpu.returned_to != 0x309fb03 { return; }
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
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ebx = v; }
  pc = 0x309fb28; continue;
 }
 0x309fb21 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x309fb28; continue;
 }
 0x309fb28 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x309fb21; continue; }
  pc = 0x309fb2c; continue;
 }
 0x309fb2c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x309fb2f; continue;
 }
 0x309fb2f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x309fb3a; continue; }
  pc = 0x309fb33; continue;
 }
 0x309fb33 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x309fb2f; continue;
 }
 0x309fb3a => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.ebp = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
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
pub fn f_0309fb49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309fb49); }
 let mut pc: u32 = 0x309fb49;
 loop { match pc {
 0x309fb49 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x309fb65; continue; }
  pc = 0x309fb5a; continue;
 }
 0x309fb5a => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x309fb65; continue;
 }
 0x309fb65 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x309fb72; continue; }
  pc = 0x309fb6b; continue;
 }
 0x309fb6b => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x309fb72; continue;
 }
 0x309fb72 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x309fb85); f_0309e014(cpu); if cpu.returned_to != 0x309fb85 { return; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x309fb87; continue;
 }
 0x309fb87 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x309fb94; continue; }
  pc = 0x309fb90; continue;
 }
 0x309fb90 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.poll(); pc = 0x309fb87; continue;
 }
 0x309fb94 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x309fbc6; continue; }
  pc = 0x309fb9a; continue;
 }
 0x309fb9a => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x309fbbe; continue;
 }
 0x309fba3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.imul2_32(cpu.edi, 0xau32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(0x30u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x309fbbe; continue;
 }
 0x309fbbe => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x309fba3; continue; }
  pc = 0x309fbc3; continue;
 }
 0x309fbc3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x309fbc6; continue;
 }
 0x309fbc6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309fc37; continue; }
  pc = 0x309fbd9; continue;
 }
 0x309fbd9 => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x309fc16; continue; }
  pc = 0x309fbdd; continue;
 }
 0x309fbdd => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x309fbe3; continue;
 }
 0x309fbe3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x309fbf3; continue; }
  pc = 0x309fbef; continue;
 }
 0x309fbef => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x309fbe3; continue;
 }
 0x309fbf3 => {
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x309fc0d; continue; }
  pc = 0x309fbf8; continue;
 }
 0x309fbf8 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x309fc01; continue;
 }
 0x309fc01 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x309fc0d; continue; }
  pc = 0x309fc0a; continue;
 }
 0x309fc0a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x309fc01; continue;
 }
 0x309fc0d => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x309fc37; continue;
 }
 0x309fc16 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x309fc21; continue; }
  pc = 0x309fc20; continue;
 }
 0x309fc20 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x309fc21; continue;
 }
 0x309fc21 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if cpu.zf { pc = 0x309fc32; continue; }
  pc = 0x309fc2a; continue;
 }
 0x309fc2a => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x309fc37; continue;
 }
 0x309fc32 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.poll(); pc = 0x309fbd9; continue;
 }
 0x309fc37 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309fc3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309fc3e); }
 let mut pc: u32 = 0x309fc3e;
 loop { match pc {
 0x309fc3e => {
  let t = cpu.r32(0x315cd74u32); cpu.push(0x309fc44); cpu.call(t); if cpu.returned_to != 0x309fc44 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0309fc45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x309fc45); }
 let mut pc: u32 = 0x309fc45;
 loop { match pc {
 0x309fc45 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.cf { pc = 0x309fc8e; continue; }
  pc = 0x309fc72; continue;
 }
 0x309fc72 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fcdf; continue; }
  pc = 0x309fc78; continue;
 }
 0x309fc78 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x309fc89; continue; }
  pc = 0x309fc7d; continue;
 }
 0x309fc7d => {
  if (cpu.cf || cpu.zf) { pc = 0x309fc9c; continue; }
  pc = 0x309fc7f; continue;
 }
 0x309fc7f => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  pc = 0x309fc82; continue;
 }
 0x309fc82 => {
  if cpu.zf { pc = 0x309fc9c; continue; }
  pc = 0x309fc84; continue;
 }
 0x309fc84 => {
  pc = 0x309fce9; continue;
 }
 0x309fc89 => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  cpu.poll(); pc = 0x309fc82; continue;
 }
 0x309fc8e => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x309fce9; continue; }
  pc = 0x309fc93; continue;
 }
 0x309fc93 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fc9c; continue; }
  pc = 0x309fc95; continue;
 }
 0x309fc95 => {
  { cpu.sub8((cpu.edx as u8), 0x64u8); }
  if cpu.zf { pc = 0x309fcdf; continue; }
  pc = 0x309fc9a; continue;
 }
 0x309fc9a => {
  pc = 0x309fce9; continue;
 }
 0x309fc9c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309fcb6; continue; }
  pc = 0x309fcaa; continue;
 }
 0x309fcaa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  pc = 0x309fcdb; continue;
 }
 0x309fcb6 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309fcd1; continue; }
  pc = 0x309fcc4; continue;
 }
 0x309fcc4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  pc = 0x309fcdb; continue;
 }
 0x309fcd1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x309fcdb; continue;
 }
 0x309fcdb => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x309fcdf; continue;
 }
 0x309fcdf => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x309fce9; continue; }
  pc = 0x309fce5; continue;
 }
 0x309fce5 => {
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x309fce9; continue;
 }
 0x309fce9 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.cf { pc = 0x309fd56; continue; }
  pc = 0x309fcf6; continue;
 }
 0x309fcf6 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fdd6; continue; }
  pc = 0x309fcfc; continue;
 }
 0x309fcfc => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  if cpu.cf { pc = 0x309fd3d; continue; }
  pc = 0x309fd01; continue;
 }
 0x309fd01 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fdec; continue; }
  pc = 0x309fd07; continue;
 }
 0x309fd07 => {
  { cpu.sub8((cpu.edx as u8), 0x73u8); }
  if cpu.cf { pc = 0x309fd2f; continue; }
  pc = 0x309fd0c; continue;
 }
 0x309fd0c => {
  if (cpu.cf || cpu.zf) { pc = 0x309fe81; continue; }
  pc = 0x309fd12; continue;
 }
 0x309fd12 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x30a0053; continue; }
  pc = 0x309fd1b; continue;
 }
 0x309fd1b => {
  if (cpu.cf || cpu.zf) { pc = 0x309ff6d; continue; }
  pc = 0x309fd21; continue;
 }
 0x309fd21 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  if cpu.zf { pc = 0x309ff4a; continue; }
  pc = 0x309fd2a; continue;
 }
 0x309fd2a => {
  pc = 0x30a0053; continue;
 }
 0x309fd2f => {
  { cpu.sub8((cpu.edx as u8), 0x70u8); }
  if cpu.zf { pc = 0x309ff90; continue; }
  pc = 0x309fd38; continue;
 }
 0x309fd38 => {
  pc = 0x30a0053; continue;
 }
 0x309fd3d => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.cf { pc = 0x309fda4; continue; }
  pc = 0x309fd42; continue;
 }
 0x309fd42 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fdd6; continue; }
  pc = 0x309fd48; continue;
 }
 0x309fd48 => {
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.zf { pc = 0x309fe30; continue; }
  pc = 0x309fd51; continue;
 }
 0x309fd51 => {
  pc = 0x30a0053; continue;
 }
 0x309fd56 => {
  { cpu.sub8((cpu.edx as u8), 0x50u8); }
  if cpu.cf { pc = 0x309fd8e; continue; }
  pc = 0x309fd5b; continue;
 }
 0x309fd5b => {
  if (cpu.cf || cpu.zf) { pc = 0x309ff90; continue; }
  pc = 0x309fd61; continue;
 }
 0x309fd61 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x309fd80; continue; }
  pc = 0x309fd66; continue;
 }
 0x309fd66 => {
  if (cpu.cf || cpu.zf) { pc = 0x309ff4a; continue; }
  pc = 0x309fd6c; continue;
 }
 0x309fd6c => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x30a0053; continue; }
  pc = 0x309fd75; continue;
 }
 0x309fd75 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a0027; continue; }
  pc = 0x309fd7b; continue;
 }
 0x309fd7b => {
  pc = 0x309fe30; continue;
 }
 0x309fd80 => {
  { cpu.sub8((cpu.edx as u8), 0x53u8); }
  if cpu.zf { pc = 0x309fe81; continue; }
  pc = 0x309fd89; continue;
 }
 0x309fd89 => {
  pc = 0x30a0053; continue;
 }
 0x309fd8e => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x309fd9f; continue; }
  pc = 0x309fd93; continue;
 }
 0x309fd93 => {
  if (cpu.cf || cpu.zf) { pc = 0x309fda4; continue; }
  pc = 0x309fd95; continue;
 }
 0x309fd95 => {
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  pc = 0x309fd98; continue;
 }
 0x309fd98 => {
  if cpu.zf { pc = 0x309fdd6; continue; }
  pc = 0x309fd9a; continue;
 }
 0x309fd9a => {
  pc = 0x30a0053; continue;
 }
 0x309fd9f => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  cpu.poll(); pc = 0x309fd98; continue;
 }
 0x309fda4 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309fdd6; continue; }
  pc = 0x309fdb2; continue;
 }
 0x309fdb2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x309fdc5); f_0309fb49(cpu); if cpu.returned_to != 0x309fdc5 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x309fe26; continue;
 }
 0x309fdd6 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x309fde1); f_0309fc3e(cpu); if cpu.returned_to != 0x309fde1 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x309fe26; continue;
 }
 0x309fdec => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309fe01; continue; }
  pc = 0x309fdfd; continue;
 }
 0x309fdfd => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x309fe01; continue;
 }
 0x309fe01 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x309fe0a); f_030ae545(cpu); if cpu.returned_to != 0x309fe0a { return; }
  pc = 0x309fe0a; continue;
 }
 0x309fe0a => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x309fe1d; continue; }
  pc = 0x309fe10; continue;
 }
 0x309fe10 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x309fe1d; continue; }
  pc = 0x309fe19; continue;
 }
 0x309fe19 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x309fe1d; continue;
 }
 0x309fe1d => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x309fe26; continue;
 }
 0x309fe26 => {
  cpu.push(0x309fe2b); f_0309faa4(cpu); if cpu.returned_to != 0x309fe2b { return; }
  pc = 0x30a007c; continue;
 }
 0x309fe30 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309fe51; continue; }
  pc = 0x309fe3e; continue;
 }
 0x309fe3e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x309fe4a; continue;
 }
 0x309fe4a => {
  cpu.push(0x309fe4f); f_030ae545(cpu); if cpu.returned_to != 0x309fe4f { return; }
  cpu.poll(); pc = 0x309fe0a; continue;
 }
 0x309fe51 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309fe6e; continue; }
  pc = 0x309fe5f; continue;
 }
 0x309fe5f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.poll(); pc = 0x309fe4a; continue;
 }
 0x309fe6e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x309fe7f); f_0309e014(cpu); if cpu.returned_to != 0x309fe7f { return; }
  cpu.poll(); pc = 0x309fe0a; continue;
 }
 0x309fe81 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309feb2; continue; }
  pc = 0x309fe8f; continue;
 }
 0x309fe8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x309fea6; continue; }
  pc = 0x309fea1; continue;
 }
 0x309fea1 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x309feaa; continue; }
  pc = 0x309fea6; continue;
 }
 0x309fea6 => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  pc = 0x309fee8; continue;
 }
 0x309feaa => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x309fee6; continue;
 }
 0x309feb2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x40u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309fed2; continue; }
  pc = 0x309fec0; continue;
 }
 0x309fec0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x309feaa; continue; }
  pc = 0x309fece; continue;
 }
 0x309fece => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  pc = 0x309fee6; continue;
 }
 0x309fed2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x309fece; continue; }
  pc = 0x309fee0; continue;
 }
 0x309fee0 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x309fee6; continue;
 }
 0x309fee6 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  pc = 0x309fee8; continue;
 }
 0x309fee8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xf9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x309ff22; continue; }
  pc = 0x309ff00; continue;
 }
 0x309ff00 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309ff19; continue; }
  pc = 0x309ff0b; continue;
 }
 0x309ff0b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x30a007c; continue;
 }
 0x309ff19 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x30a007c; continue;
 }
 0x309ff22 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309ff3e; continue; }
  pc = 0x309ff2d; continue;
 }
 0x309ff2d => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x309ff39); f_0309fac8(cpu); if cpu.returned_to != 0x309ff39 { return; }
  pc = 0x30a007c; continue;
 }
 0x309ff3e => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.poll(); pc = 0x309fe26; continue;
 }
 0x309ff4a => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x309ff68; continue; }
  pc = 0x309ff56; continue;
 }
 0x309ff56 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x309ff68; continue; }
  pc = 0x309ff5a; continue;
 }
 0x309ff5a => {
  { let v: u8 = 0x30u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x309ff68; continue;
 }
 0x309ff68 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x309ff6d; continue;
 }
 0x309ff6d => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x309ff76); f_030ae4fb(cpu); if cpu.returned_to != 0x309ff76 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { cpu.poll(); pc = 0x309fe0a; continue; }
  pc = 0x309ff84; continue;
 }
 0x309ff84 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x309ff8b); f_030a012e(cpu); if cpu.returned_to != 0x309ff8b { return; }
  cpu.poll(); pc = 0x309fe0a; continue;
 }
 0x309ff90 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x309ffb4; continue; }
  pc = 0x309ff96; continue;
 }
 0x309ff96 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x309ffad; continue; }
  pc = 0x309ffa4; continue;
 }
 0x309ffa4 => {
  { let v: u32 = 0xdu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x309ffb4; continue;
 }
 0x309ffad => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x309ffb4; continue;
 }
 0x309ffb4 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x309ffff; continue; }
  pc = 0x309ffd6; continue;
 }
 0x309ffd6 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x309ffef); f_0309faeb(cpu); if cpu.returned_to != 0x309ffef { return; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x30a0008; continue;
 }
 0x309ffff => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x30a0008; continue;
 }
 0x30a0008 => {
  cpu.push(0x30a000d); f_0309faeb(cpu); if cpu.returned_to != 0x30a000d { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if !cpu.zf { cpu.poll(); pc = 0x309fe1d; continue; }
  pc = 0x30a001b; continue;
 }
 0x30a001b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30a0022); f_030a012e(cpu); if cpu.returned_to != 0x30a0022 { return; }
  cpu.poll(); pc = 0x309fe1d; continue;
 }
 0x30a0027 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a007c; continue;
 }
 0x30a0053 => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30a007c; continue;
 }
 0x30a007c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x30a00d1; continue; }
  pc = 0x30a0082; continue;
 }
 0x30a0082 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x30a0094; continue; }
  pc = 0x30a008c; continue;
 }
 0x30a008c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x30a00ba; continue;
 }
 0x30a0094 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x30a00a8; continue; }
  pc = 0x30a00a2; continue;
 }
 0x30a00a2 => {
  { let v: u8 = 0x20u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a00ba; continue;
 }
 0x30a00a8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x30a00be; continue; }
  pc = 0x30a00b6; continue;
 }
 0x30a00b6 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a00ba; continue;
 }
 0x30a00ba => {
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a00be; continue;
 }
 0x30a00be => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a00ca; continue; }
  pc = 0x30a00c5; continue;
 }
 0x30a00c5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a00d1; continue;
 }
 0x30a00ca => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x30a00d1; continue;
 }
 0x30a00d1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x30a00e8; continue; }
  pc = 0x30a00db; continue;
 }
 0x30a00db => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a00e8; continue;
 }
 0x30a00e8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a00f4; continue; }
  pc = 0x30a00f0; continue;
 }
 0x30a00f0 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x30a0101; continue; }
  pc = 0x30a00f4; continue;
 }
 0x30a00f4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if cpu.zf { pc = 0x30a0101; continue; }
  pc = 0x30a00fe; continue;
 }
 0x30a00fe => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a0101; continue;
 }
 0x30a0101 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x30a0110); f_0309faa4(cpu); if cpu.returned_to != 0x30a0110 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a012e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a012e); }
 let mut pc: u32 = 0x30a012e;
 loop { match pc {
 0x30a012e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x30a0131; continue;
 }
 0x30a0131 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x30a0144; continue; }
  pc = 0x30a0136; continue;
 }
 0x30a0136 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x30a013f); f_030949d4(cpu); if cpu.returned_to != 0x30a013f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x30a0131; continue;
 }
 0x30a0144 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0150); }
 let mut pc: u32 = 0x30a0150;
 loop { match pc {
 0x30a0150 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let a = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); let b = cpu.ecx; { let v: u32 = b; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.ecx = v; } }
  { let a = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); } { let v: u32 = a; cpu.eax = v; } }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0170); }
 let mut pc: u32 = 0x30a0170;
 loop { match pc {
 0x30a0170 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x30a0175); cpu.call(t); if cpu.returned_to != 0x30a0175 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0177(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0177); }
 let mut pc: u32 = 0x30a0177;
 loop { match pc {
 0x30a0177 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x30a017d); cpu.call(t); if cpu.returned_to != 0x30a017d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a017f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a017f); }
 let mut pc: u32 = 0x30a017f;
 loop { match pc {
 0x30a017f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfdu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.esi), v); }
  pc = 0x30a019e; continue;
 }
 0x30a019e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  if cpu.zf { pc = 0x30a03a6; continue; }
  pc = 0x30a01b5; continue;
 }
 0x30a01b5 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a01db; continue; }
  pc = 0x30a01cd; continue;
 }
 0x30a01cd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a01d4); f_030a04bb(cpu); if cpu.returned_to != 0x30a01d4 { return; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x30a0366; continue;
 }
 0x30a01db => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x30a0205; continue; }
  pc = 0x30a01e0; continue;
 }
 0x30a01e0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a01e7); f_030a0170(cpu); if cpu.returned_to != 0x30a01e7 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x30a0365; continue; }
  pc = 0x30a01ef; continue;
 }
 0x30a01ef => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x30a03a6; continue; }
  pc = 0x30a01f9; continue;
 }
 0x30a01f9 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x30a0200); f_030a0177(cpu); if cpu.returned_to != 0x30a0200 { return; }
  pc = 0x30a03a6; continue;
 }
 0x30a0205 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a020e); f_030a03c6(cpu); if cpu.returned_to != 0x30a020e { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30a021d; continue; }
  pc = 0x30a0219; continue;
 }
 0x30a0219 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a021d; continue;
 }
 0x30a021d => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if cpu.cf { pc = 0x30a0277; continue; }
  pc = 0x30a0222; continue;
 }
 0x30a0222 => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if (cpu.cf || cpu.zf) { pc = 0x30a0303; continue; }
  pc = 0x30a022b; continue;
 }
 0x30a022b => {
  { cpu.sub32(cpu.ebx, 0x70u32); }
  if cpu.cf { pc = 0x30a025d; continue; }
  pc = 0x30a0230; continue;
 }
 0x30a0230 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a02f7; continue; }
  pc = 0x30a0236; continue;
 }
 0x30a0236 => {
  { cpu.sub32(cpu.ebx, 0x75u32); }
  if cpu.cf { pc = 0x30a024f; continue; }
  pc = 0x30a023b; continue;
 }
 0x30a023b => {
  if (cpu.cf || cpu.zf) { pc = 0x30a02ea; continue; }
  pc = 0x30a0241; continue;
 }
 0x30a0241 => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x30a02f7; continue; }
  pc = 0x30a024a; continue;
 }
 0x30a024a => {
  pc = 0x30a0366; continue;
 }
 0x30a024f => {
  { cpu.sub32(cpu.ebx, 0x73u32); }
  if cpu.zf { pc = 0x30a030e; continue; }
  pc = 0x30a0258; continue;
 }
 0x30a0258 => {
  pc = 0x30a0366; continue;
 }
 0x30a025d => {
  { cpu.sub32(cpu.ebx, 0x6eu32); }
  if cpu.cf { pc = 0x30a026d; continue; }
  pc = 0x30a0262; continue;
 }
 0x30a0262 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a0349; continue; }
  pc = 0x30a0268; continue;
 }
 0x30a0268 => {
  pc = 0x30a02de; continue;
 }
 0x30a026d => {
  { cpu.sub32(cpu.ebx, 0x69u32); }
  if cpu.zf { pc = 0x30a02d1; continue; }
  pc = 0x30a0272; continue;
 }
 0x30a0272 => {
  pc = 0x30a0366; continue;
 }
 0x30a0277 => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if cpu.cf { pc = 0x30a029d; continue; }
  pc = 0x30a027c; continue;
 }
 0x30a027c => {
  if (cpu.cf || cpu.zf) { pc = 0x30a02f7; continue; }
  pc = 0x30a0282; continue;
 }
 0x30a0282 => {
  { cpu.sub32(cpu.ebx, 0x63u32); }
  if cpu.cf { pc = 0x30a028f; continue; }
  pc = 0x30a0287; continue;
 }
 0x30a0287 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a0326; continue; }
  pc = 0x30a028d; continue;
 }
 0x30a028d => {
  pc = 0x30a02bc; continue;
 }
 0x30a028f => {
  { cpu.sub32(cpu.ebx, 0x5bu32); }
  if cpu.zf { pc = 0x30a0319; continue; }
  pc = 0x30a0298; continue;
 }
 0x30a0298 => {
  pc = 0x30a0366; continue;
 }
 0x30a029d => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.cf { pc = 0x30a02ae; continue; }
  pc = 0x30a02a2; continue;
 }
 0x30a02a2 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a0303; continue; }
  pc = 0x30a02a4; continue;
 }
 0x30a02a4 => {
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x30a0303; continue; }
  pc = 0x30a02a9; continue;
 }
 0x30a02a9 => {
  pc = 0x30a0366; continue;
 }
 0x30a02ae => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x30a0356; continue; }
  pc = 0x30a02b7; continue;
 }
 0x30a02b7 => {
  pc = 0x30a0366; continue;
 }
 0x30a02bc => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x30a02c6; continue;
 }
 0x30a02c6 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x30a02ca; continue;
 }
 0x30a02ca => {
  cpu.push(0x30a02cf); f_030a0b64(cpu); if cpu.returned_to != 0x30a02cf { return; }
  pc = 0x30a032f; continue;
 }
 0x30a02d1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.poll(); pc = 0x30a02ca; continue;
 }
 0x30a02de => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x30a02c6; continue;
 }
 0x30a02ea => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x30a02ca; continue;
 }
 0x30a02f7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x30a02c6; continue;
 }
 0x30a0303 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a030c); f_030a084d(cpu); if cpu.returned_to != 0x30a030c { return; }
  pc = 0x30a032f; continue;
 }
 0x30a030e => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a0317); f_030a058f(cpu); if cpu.returned_to != 0x30a0317 { return; }
  pc = 0x30a032f; continue;
 }
 0x30a0319 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a0324); f_030a0753(cpu); if cpu.returned_to != 0x30a0324 { return; }
  pc = 0x30a032f; continue;
 }
 0x30a0326 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a032f); f_030a04fa(cpu); if cpu.returned_to != 0x30a032f { return; }
  pc = 0x30a032f; continue;
 }
 0x30a032f => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a03a6; continue; }
  pc = 0x30a0339; continue;
 }
 0x30a0339 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a0366; continue; }
  pc = 0x30a0343; continue;
 }
 0x30a0343 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a0366; continue;
 }
 0x30a0349 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a0354); f_030a06b2(cpu); if cpu.returned_to != 0x30a0354 { return; }
  pc = 0x30a0366; continue;
 }
 0x30a0356 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a035d); f_030a0170(cpu); if cpu.returned_to != 0x30a035d { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x30a01ef; continue; }
  pc = 0x30a0365; continue;
 }
 0x30a0365 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30a0366; continue;
 }
 0x30a0366 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x30a019e; continue; }
  pc = 0x30a0370; continue;
 }
 0x30a0370 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x25u32); }
  if !cpu.zf { pc = 0x30a03a6; continue; }
  pc = 0x30a037f; continue;
 }
 0x30a037f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x30a038c); f_030a03c6(cpu); if cpu.returned_to != 0x30a038c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x30a03a6; continue; }
  pc = 0x30a039b; continue;
 }
 0x30a039b => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a03a6); f_030a06b2(cpu); if cpu.returned_to != 0x30a03a6 { return; }
  pc = 0x30a03a6; continue;
 }
 0x30a03a6 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x30a03ba; continue; }
  pc = 0x30a03ad; continue;
 }
 0x30a03ad => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x30a03ba; continue; }
  pc = 0x30a03b3; continue;
 }
 0x30a03b3 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a03be; continue;
 }
 0x30a03ba => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x30a03be; continue;
 }
 0x30a03be => {
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
pub fn f_030a03c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a03c6); }
 let mut pc: u32 = 0x30a03c6;
 loop { match pc {
 0x30a03c6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) & 0x3u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x30a03f9; continue; }
  pc = 0x30a03ef; continue;
 }
 0x30a03ef => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a03f9; continue;
 }
 0x30a03f9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x30a044b; continue; }
  pc = 0x30a041c; continue;
 }
 0x30a041c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x30a041e; continue;
 }
 0x30a041e => {
  { let v: u32 = cpu.imul2_32(cpu.edx, 0xau32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x30u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x30a041e; continue; }
  pc = 0x30a0448; continue;
 }
 0x30a0448 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a044b; continue;
 }
 0x30a044b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x4eu32); }
  if !cpu.zf { pc = 0x30a045f; continue; }
  pc = 0x30a0457; continue;
 }
 0x30a0457 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x30a046f; continue;
 }
 0x30a045f => {
  { cpu.sub32(cpu.edx, 0x46u32); }
  if !cpu.zf { pc = 0x30a046f; continue; }
  pc = 0x30a0464; continue;
 }
 0x30a0464 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a046f; continue;
 }
 0x30a046f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.cf { pc = 0x30a0489; continue; }
  pc = 0x30a0479; continue;
 }
 0x30a0479 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a0490; continue; }
  pc = 0x30a047b; continue;
 }
 0x30a047b => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x30a04b4; continue; }
  pc = 0x30a0480; continue;
 }
 0x30a0480 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a049d; continue; }
  pc = 0x30a0482; continue;
 }
 0x30a0482 => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x30a049d; continue; }
  pc = 0x30a0487; continue;
 }
 0x30a0487 => {
  pc = 0x30a04b4; continue;
 }
 0x30a0489 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x30a04a9; continue; }
  pc = 0x30a048e; continue;
 }
 0x30a048e => {
  pc = 0x30a04b4; continue;
 }
 0x30a0490 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x10u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a04b4; continue;
 }
 0x30a049d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a04b4; continue;
 }
 0x30a04a9 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a04b4; continue;
 }
 0x30a04b4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a04bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a04bb); }
 let mut pc: u32 = 0x30a04bb;
 loop { match pc {
 0x30a04bb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x30a04c2; continue;
 }
 0x30a04c2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a04c9); f_030a0170(cpu); if cpu.returned_to != 0x30a04c9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x30a04e7; continue; }
  pc = 0x30a04e4; continue;
 }
 0x30a04e4 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x30a04c2; continue;
 }
 0x30a04e7 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x30a04f4; continue; }
  pc = 0x30a04ed; continue;
 }
 0x30a04ed => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x30a04f4); f_030a0177(cpu); if cpu.returned_to != 0x30a04f4 { return; }
  pc = 0x30a04f4; continue;
 }
 0x30a04f4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a04fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a04fa); } r_030a04fa(cpu, 0x30a04fa); }
pub fn f_030a058f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a058f); } r_030a04fa(cpu, 0x30a058f); }
pub fn f_030a084d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a084d); } r_030a04fa(cpu, 0x30a084d); }
fn r_030a04fa(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30a04fa => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x30a0541; continue; }
  pc = 0x30a050a; continue;
 }
 0x30a050a => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a051e; continue; }
  pc = 0x30a0512; continue;
 }
 0x30a0512 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x30a0541; continue;
 }
 0x30a051e => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x30a0533; continue; }
  pc = 0x30a0523; continue;
 }
 0x30a0523 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x30a0541; continue;
 }
 0x30a0533 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x30a0541; continue;
 }
 0x30a0541 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x30a0550; continue; }
  pc = 0x30a054b; continue;
 }
 0x30a054b => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x30a0550; continue;
 }
 0x30a0550 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a0584; continue; }
  pc = 0x30a0554; continue;
 }
 0x30a0554 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a055b); f_030a0170(cpu); if cpu.returned_to != 0x30a055b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x30a0584; continue; }
  pc = 0x30a0565; continue;
 }
 0x30a0565 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x30a0550; continue; }
  pc = 0x30a056b; continue;
 }
 0x30a056b => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x30a057b; continue; }
  pc = 0x30a056f; continue;
 }
 0x30a056f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x30a0550; continue;
 }
 0x30a057b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x30a0550; continue;
 }
 0x30a0584 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x30a0589; continue;
 }
 0x30a0589 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a058f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x30a05a6; continue; }
  pc = 0x30a059f; continue;
 }
 0x30a059f => {
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a05af; continue;
 }
 0x30a05a6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x10u8); }
  { let v: u8 = 0x1u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a05af; continue;
 }
 0x30a05af => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a05e8; continue; }
  pc = 0x30a05b6; continue;
 }
 0x30a05b6 => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a05c6; continue; }
  pc = 0x30a05ba; continue;
 }
 0x30a05ba => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x30a05e8; continue;
 }
 0x30a05c6 => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x30a05da; continue; }
  pc = 0x30a05ca; continue;
 }
 0x30a05ca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x30a05e8; continue;
 }
 0x30a05da => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x30a05e8; continue;
 }
 0x30a05e8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x30a05ea; continue;
 }
 0x30a05ea => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a05f1); f_030a0170(cpu); if cpu.returned_to != 0x30a05f1 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a0610; continue; }
  pc = 0x30a060d; continue;
 }
 0x30a060d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x30a05ea; continue;
 }
 0x30a0610 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if cpu.zf { pc = 0x30a061d; continue; }
  pc = 0x30a0616; continue;
 }
 0x30a0616 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x30a0687; continue;
 }
 0x30a061d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a067d; continue; }
  pc = 0x30a062a; continue;
 }
 0x30a062a => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a0655; continue; }
  pc = 0x30a0633; continue;
 }
 0x30a0633 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30a0646; continue; }
  pc = 0x30a063e; continue;
 }
 0x30a063e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a064d; continue;
 }
 0x30a0646 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a064d; continue;
 }
 0x30a064d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x30a0655; continue;
 }
 0x30a0655 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a065c); f_030a0d7b(cpu); if cpu.returned_to != 0x30a065c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0687; continue; }
  pc = 0x30a0664; continue;
 }
 0x30a0664 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x30a062a; continue; }
  pc = 0x30a067d; continue;
 }
 0x30a067d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x30a0687); f_030a0177(cpu); if cpu.returned_to != 0x30a0687 { return; }
  pc = 0x30a0687; continue;
 }
 0x30a0687 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x30a06a8; continue; }
  pc = 0x30a068d; continue;
 }
 0x30a068d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a06a8; continue; }
  pc = 0x30a0691; continue;
 }
 0x30a0691 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30a06a2; continue; }
  pc = 0x30a069c; continue;
 }
 0x30a069c => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a06a8; continue;
 }
 0x30a06a2 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a06a8; continue;
 }
 0x30a06a8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x30a0589; continue;
 }
 0x30a084d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a0864; continue;
 }
 0x30a0864 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a086b); f_030a0170(cpu); if cpu.returned_to != 0x30a086b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x30a088b; continue; }
  pc = 0x30a0885; continue;
 }
 0x30a0885 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x60u32.wrapping_add(cpu.esp))); cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x30a0864; continue;
 }
 0x30a088b => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0895; continue;
 }
 0x30a0895 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a0a95; continue; }
  pc = 0x30a08a6; continue;
 }
 0x30a08a6 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x30a08b0; continue; }
  pc = 0x30a08ab; continue;
 }
 0x30a08ab => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x30a08ce; continue; }
  pc = 0x30a08b0; continue;
 }
 0x30a08b0 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30a08bd); f_030a0d7b(cpu); if cpu.returned_to != 0x30a08bd { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a08ce; continue;
 }
 0x30a08ce => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x30a08ef; continue; }
  pc = 0x30a08e6; continue;
 }
 0x30a08e6 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x30a0a95; continue; }
  pc = 0x30a08ef; continue;
 }
 0x30a08ef => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a0957; continue; }
  pc = 0x30a090f; continue;
 }
 0x30a090f => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x30a0914; continue;
 }
 0x30a0914 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x30a092e; continue; }
  pc = 0x30a091f; continue;
 }
 0x30a091f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6au32.wrapping_add(cpu.esp)), 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6au32.wrapping_add(cpu.esp), v); }
  pc = 0x30a092e; continue;
 }
 0x30a092e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x30a0936); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0936 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0941; continue;
 }
 0x30a0941 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30a0914; continue; }
  pc = 0x30a0957; continue;
 }
 0x30a0957 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x30a0a07; continue; }
  pc = 0x30a0960; continue;
 }
 0x30a0960 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30a0969); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0969 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0975; continue;
 }
 0x30a0975 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x30a0993; continue; }
  pc = 0x30a0979; continue;
 }
 0x30a0979 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a0a95; continue; }
  pc = 0x30a0993; continue;
 }
 0x30a0993 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x30a0994; continue;
 }
 0x30a0994 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a09be; continue; }
  pc = 0x30a09ac; continue;
 }
 0x30a09ac => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30a09b6); f_030a0d7b(cpu); if cpu.returned_to != 0x30a09b6 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x30a0994; continue; }
  pc = 0x30a09be; continue;
 }
 0x30a09be => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x30a09fe; continue; }
  pc = 0x30a09c4; continue;
 }
 0x30a09c4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a09cc; continue;
 }
 0x30a09cc => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x30a09f5; continue; }
  pc = 0x30a09d7; continue;
 }
 0x30a09d7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x30a09cc; continue;
 }
 0x30a09f5 => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a09fe; continue;
 }
 0x30a09fe => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0a07; continue;
 }
 0x30a0a07 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x30a0a95; continue; }
  pc = 0x30a0a11; continue;
 }
 0x30a0a11 => {
  { cpu.sub32(cpu.ecx, 0x65u32); }
  if cpu.zf { pc = 0x30a0a1f; continue; }
  pc = 0x30a0a16; continue;
 }
 0x30a0a16 => {
  { cpu.sub32(cpu.ecx, 0x45u32); }
  if !cpu.zf { pc = 0x30a0a95; continue; }
  pc = 0x30a0a1f; continue;
 }
 0x30a0a1f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30a0a29); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0a29 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0a35; continue;
 }
 0x30a0a35 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x30a0a3f; continue; }
  pc = 0x30a0a3a; continue;
 }
 0x30a0a3a => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x30a0a51; continue; }
  pc = 0x30a0a3f; continue;
 }
 0x30a0a3f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30a0a49); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0a49 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0a51; continue;
 }
 0x30a0a51 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x30a0a6d; continue; }
  pc = 0x30a0a69; continue;
 }
 0x30a0a69 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x30a0a95; continue;
 }
 0x30a0a6d => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x30a0a77); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0a77 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0a9e; continue; }
  pc = 0x30a0a7f; continue;
 }
 0x30a0a7f => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30a0a6d; continue; }
  pc = 0x30a0a95; continue;
 }
 0x30a0a95 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a0a9e); f_030a0177(cpu); if cpu.returned_to != 0x30a0a9e { return; }
  pc = 0x30a0a9e; continue;
 }
 0x30a0a9e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a0b59; continue; }
  pc = 0x30a0aa6; continue;
 }
 0x30a0aa6 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a0b59; continue; }
  pc = 0x30a0ab8; continue;
 }
 0x30a0ab8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x30a0ad1; continue; }
  pc = 0x30a0ac1; continue;
 }
 0x30a0ac1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x30a0add; continue; }
  pc = 0x30a0acb; continue;
 }
 0x30a0acb => {
  { let v: u32 = cpu.neg32(cpu.r32(0x68u32.wrapping_add(cpu.esp))); cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a0add; continue;
 }
 0x30a0ad1 => {
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  let t = cpu.r32(0x315cd78u32); cpu.push(0x30a0add); cpu.call(t); if cpu.returned_to != 0x30a0add { return; }
  pc = 0x30a0add; continue;
 }
 0x30a0add => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a0af5; continue; }
  pc = 0x30a0ae5; continue;
 }
 0x30a0ae5 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebx = v; cpu.set_segment(0, selector); }
  pc = 0x30a0b20; continue;
 }
 0x30a0af5 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x30a0b0e; continue; }
  pc = 0x30a0afa; continue;
 }
 0x30a0afa => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  pc = 0x30a0b20; continue;
 }
 0x30a0b0e => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  pc = 0x30a0b20; continue;
 }
 0x30a0b20 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x30a0b2e; continue; }
  pc = 0x30a0b28; continue;
 }
 0x30a0b28 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x30a0b56; continue;
 }
 0x30a0b2e => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x30a0b38; continue; }
  pc = 0x30a0b33; continue;
 }
 0x30a0b33 => {
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x30a0b49; continue; }
  pc = 0x30a0b38; continue;
 }
 0x30a0b38 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a0b59; continue;
 }
 0x30a0b49 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x30a0b56); f_030ae560(cpu); if cpu.returned_to != 0x30a0b56 { return; }
  pc = 0x30a0b56; continue;
 }
 0x30a0b56 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a0b59; continue;
 }
 0x30a0b59 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x30a0589; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a06b2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a06b2); }
 let mut pc: u32 = 0x30a06b2;
 loop { match pc {
 0x30a06b2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a070b; continue; }
  pc = 0x30a06be; continue;
 }
 0x30a06be => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a06cf; continue; }
  pc = 0x30a06c3; continue;
 }
 0x30a06c3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x30a06f4; continue;
 }
 0x30a06cf => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x30a06e4; continue; }
  pc = 0x30a06d4; continue;
 }
 0x30a06d4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x30a06f4; continue;
 }
 0x30a06e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x30a06f4; continue;
 }
 0x30a06f4 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x30a0705; continue; }
  pc = 0x30a06fc; continue;
 }
 0x30a06fc => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a0705 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a070b; continue;
 }
 0x30a070b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0710); }
 let mut pc: u32 = 0x30a0710;
 loop { match pc {
 0x30a0710 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x30a0725); f_0309f0c0(cpu); if cpu.returned_to != 0x30a0725 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a074d; continue; }
  pc = 0x30a072e; continue;
 }
 0x30a072e => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x314b9ccu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a074d; continue; }
  pc = 0x30a0747; continue;
 }
 0x30a0747 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x5du32); }
  if !cpu.zf { cpu.poll(); pc = 0x30a072e; continue; }
  pc = 0x30a074d; continue;
 }
 0x30a074d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0753(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0753); }
 let mut pc: u32 = 0x30a0753;
 loop { match pc {
 0x30a0753 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x5eu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  if cpu.zf { pc = 0x30a077a; continue; }
  pc = 0x30a0777; continue;
 }
 0x30a0777 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a077a; continue;
 }
 0x30a077a => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x30a0783); f_030a0710(cpu); if cpu.returned_to != 0x30a0783 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a07c1; continue; }
  pc = 0x30a078d; continue;
 }
 0x30a078d => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a079e; continue; }
  pc = 0x30a0792; continue;
 }
 0x30a0792 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.esi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(0, selector); }
  pc = 0x30a07c1; continue;
 }
 0x30a079e => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x30a07b3; continue; }
  pc = 0x30a07a3; continue;
 }
 0x30a07a3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x30a07c1; continue;
 }
 0x30a07b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.esi = v; }
  pc = 0x30a07c1; continue;
 }
 0x30a07c1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x30a07c6; continue;
 }
 0x30a07c6 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x30a0834; continue; }
  pc = 0x30a07ca; continue;
 }
 0x30a07ca => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a07d1); f_030a0170(cpu); if cpu.returned_to != 0x30a07d1 { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x30a0834; continue; }
  pc = 0x30a07dd; continue;
 }
 0x30a07dd => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x314b9ccu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebp); }
  { let v: u8 = (cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.zf { pc = 0x30a0820; continue; }
  pc = 0x30a0817; continue;
 }
 0x30a0817 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x30a081e); f_030a0177(cpu); if cpu.returned_to != 0x30a081e { return; }
  pc = 0x30a0834; continue;
 }
 0x30a0820 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x30a07c6; continue; }
  pc = 0x30a082a; continue;
 }
 0x30a082a => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x30a07c6; continue;
 }
 0x30a0834 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x30a0842; continue; }
  pc = 0x30a083a; continue;
 }
 0x30a083a => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a0842; continue; }
  pc = 0x30a083e; continue;
 }
 0x30a083e => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a0842; continue;
 }
 0x30a0842 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0b64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0b64); }
 let mut pc: u32 = 0x30a0b64;
 loop { match pc {
 0x30a0b64 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a0b7e; continue;
 }
 0x30a0b7e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a0b85); f_030a0170(cpu); if cpu.returned_to != 0x30a0b85 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x314b8a0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a0ba0; continue; }
  pc = 0x30a0b9b; continue;
 }
 0x30a0b9b => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x30a0b7e; continue;
 }
 0x30a0ba0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0baa; continue;
 }
 0x30a0baa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a0cd3; continue; }
  pc = 0x30a0bbb; continue;
 }
 0x30a0bbb => {
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x30a0bef; continue; }
  pc = 0x30a0bcc; continue;
 }
 0x30a0bcc => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x30a0bd6; continue; }
  pc = 0x30a0bd1; continue;
 }
 0x30a0bd1 => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if !cpu.zf { pc = 0x30a0bef; continue; }
  pc = 0x30a0bd6; continue;
 }
 0x30a0bd6 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x30a0be4); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0be4 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0bef; continue;
 }
 0x30a0bef => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x30a0c4b; continue; }
  pc = 0x30a0bf3; continue;
 }
 0x30a0bf3 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x30a0c44; continue; }
  pc = 0x30a0bf8; continue;
 }
 0x30a0bf8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x30a0c04); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0c04 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0c0f; continue;
 }
 0x30a0c0f => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x30a0c19; continue; }
  pc = 0x30a0c14; continue;
 }
 0x30a0c14 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x30a0c3d; continue; }
  pc = 0x30a0c19; continue;
 }
 0x30a0c19 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  cpu.push(0x30a0c23); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0c23 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0c36; continue;
 }
 0x30a0c36 => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x30a0c8f; continue;
 }
 0x30a0c3d => {
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x30a0c8f; continue;
 }
 0x30a0c44 => {
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x30a0c8f; continue;
 }
 0x30a0c4b => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x30a0c8f; continue; }
  pc = 0x30a0c50; continue;
 }
 0x30a0c50 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x30a0c8f; continue; }
  pc = 0x30a0c55; continue;
 }
 0x30a0c55 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x30a0c61); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0c61 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0c6c; continue;
 }
 0x30a0c6c => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x30a0c76; continue; }
  pc = 0x30a0c71; continue;
 }
 0x30a0c71 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x30a0c8f; continue; }
  pc = 0x30a0c76; continue;
 }
 0x30a0c76 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x30a0c80); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0c80 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a0c88; continue;
 }
 0x30a0c88 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0c8f; continue;
 }
 0x30a0c8f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a0c96); f_030a0d54(cpu); if cpu.returned_to != 0x30a0c96 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30a0ca9; continue; }
  pc = 0x30a0c9a; continue;
 }
 0x30a0c9a => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x30a0ca7); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0ca7 { return; }
  cpu.poll(); pc = 0x30a0c88; continue;
 }
 0x30a0ca9 => {
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if !cpu.zf { pc = 0x30a0cd3; continue; }
  pc = 0x30a0cae; continue;
 }
 0x30a0cae => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x30a0cd3; continue; }
  pc = 0x30a0cb4; continue;
 }
 0x30a0cb4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x30a0cbc); f_030a0d7b(cpu); if cpu.returned_to != 0x30a0cbc { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a0cdc; continue; }
  pc = 0x30a0cc3; continue;
 }
 0x30a0cc3 => {
  cpu.push(0x30a0cc8); f_030a0d54(cpu); if cpu.returned_to != 0x30a0cc8 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30a0cd3; continue; }
  pc = 0x30a0ccc; continue;
 }
 0x30a0ccc => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  cpu.poll(); pc = 0x30a0cb4; continue;
 }
 0x30a0cd3 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a0cdc); f_030a0177(cpu); if cpu.returned_to != 0x30a0cdc { return; }
  pc = 0x30a0cdc; continue;
 }
 0x30a0cdc => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x2du32); }
  if !cpu.zf { pc = 0x30a0ce5; continue; }
  pc = 0x30a0ce3; continue;
 }
 0x30a0ce3 => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x30a0ce5; continue;
 }
 0x30a0ce5 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a0d4a; continue; }
  pc = 0x30a0ce9; continue;
 }
 0x30a0ce9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a0d4a; continue; }
  pc = 0x30a0cf6; continue;
 }
 0x30a0cf6 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a0d0b; continue; }
  pc = 0x30a0cfb; continue;
 }
 0x30a0cfb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  pc = 0x30a0d36; continue;
 }
 0x30a0d0b => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x30a0d24; continue; }
  pc = 0x30a0d10; continue;
 }
 0x30a0d10 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x30a0d36; continue;
 }
 0x30a0d24 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x30a0d36; continue;
 }
 0x30a0d36 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x30a0d44; continue; }
  pc = 0x30a0d3e; continue;
 }
 0x30a0d3e => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a0d4a; continue;
 }
 0x30a0d44 => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x30a0d4a; continue;
 }
 0x30a0d4a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0d54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0d54); }
 let mut pc: u32 = 0x30a0d54;
 loop { match pc {
 0x30a0d54 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x30a0d62; continue; }
  pc = 0x30a0d59; continue;
 }
 0x30a0d59 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a0d62; continue; }
  pc = 0x30a0d5e; continue;
 }
 0x30a0d5e => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a0d62 => {
  cpu.push(0x30a0d67); f_03095c18(cpu); if cpu.returned_to != 0x30a0d67 { return; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x30a0d75; continue; }
  pc = 0x30a0d6c; continue;
 }
 0x30a0d6c => {
  { cpu.sub32(cpu.eax, 0x66u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a0d75; continue; }
  pc = 0x30a0d71; continue;
 }
 0x30a0d71 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x57u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a0d75 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0d7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0d7b); }
 let mut pc: u32 = 0x30a0d7b;
 loop { match pc {
 0x30a0d7b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a0d99; continue; }
  pc = 0x30a0d8c; continue;
 }
 0x30a0d8c => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30a0d93); f_030a0170(cpu); if cpu.returned_to != 0x30a0d93 { return; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x2u8); }
  if cpu.zf { pc = 0x30a0d9e; continue; }
  pc = 0x30a0d99; continue;
 }
 0x30a0d99 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a0d9e; continue;
 }
 0x30a0d9e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0da4); }
 let mut pc: u32 = 0x30a0da4;
 loop { match pc {
 0x30a0da4 => {
  { let v: u32 = cpu.r32(0x315bdb0u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0e5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0e5e); }
 let mut pc: u32 = 0x30a0e5e;
 loop { match pc {
 0x30a0e5e => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30a0e6b); f_030ae5ac(cpu); if cpu.returned_to != 0x30a0e6b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x30a0e80); f_030ae9e6(cpu); if cpu.returned_to != 0x30a0e80 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0e85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0e85); }
 let mut pc: u32 = 0x30a0e85;
 loop { match pc {
 0x30a0e85 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(24); }
  cpu.push(0x30a0e8e); f_030aeb46(cpu); if cpu.returned_to != 0x30a0e8e { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21u32; cpu.push(v); }
  { let v: u8 = 0x35u8; cpu.w8(0xffffffe9u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a0ea8); f_030a1396(cpu); if cpu.returned_to != 0x30a0ea8 { return; }
  { let v: u32 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffeau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0ef8); }
 let mut pc: u32 = 0x30a0ef8;
 loop { match pc {
 0x30a0ef8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x800u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a0f2a); f_030a0e5e(cpu); if cpu.returned_to != 0x30a0f2a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x30a0f3a; continue; }
  pc = 0x30a0f34; continue;
 }
 0x30a0f34 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a0f3a => {
  { let v: u32 = cpu.r16(0xffffffe8u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0f4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a0f4b); } r_030a0f4b(cpu, 0x30a0f4b); }
pub fn f_030a0fb4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a0fb4); } r_030a0f4b(cpu, 0x30a0fb4); }
fn r_030a0f4b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30a0f4b => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x7u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a0f6a; continue;
 }
 0x30a0f6a => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a0f73); f_030a0e5e(cpu); if cpu.returned_to != 0x30a0f73 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a0fb4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x8u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x30a0f6a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0f84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0f84); }
 let mut pc: u32 = 0x30a0f84;
 loop { match pc {
 0x30a0f84 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x6u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a0fa0); f_030a0e5e(cpu); if cpu.returned_to != 0x30a0fa0 { return; }
  { let v: u32 = cpu.r16(0xffffffecu32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0xfffffff0u32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a0fd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a0fd5); }
 let mut pc: u32 = 0x30a0fd5;
 loop { match pc {
 0x30a0fd5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x1u16; cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a0ff9); f_030a0e5e(cpu); if cpu.returned_to != 0x30a0ff9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x30a1007; continue; }
  pc = 0x30a1003; continue;
 }
 0x30a1003 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x30a1060; continue;
 }
 0x30a1007 => {
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.ebx, 0x100000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x30a1041; continue; }
  pc = 0x30a1013; continue;
 }
 0x30a1013 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x9u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x8092u16; cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a102f); f_030a0e5e(cpu); if cpu.returned_to != 0x30a102f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x30a103b); f_030a0ef8(cpu); if cpu.returned_to != 0x30a103b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a1060; continue; }
  pc = 0x30a1041; continue;
 }
 0x30a1041 => {
  { let v: u32 = (cpu.edi as u16) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a104d); f_030a0f4b(cpu); if cpu.returned_to != 0x30a104d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a1060; continue; }
  pc = 0x30a1051; continue;
 }
 0x30a1051 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a105a); f_030a0fb4(cpu); if cpu.returned_to != 0x30a105a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a1060; continue; }
  pc = 0x30a105e; continue;
 }
 0x30a105e => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30a1060; continue;
 }
 0x30a1060 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a11b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a11b5); }
 let mut pc: u32 = 0x30a11b5;
 loop { match pc {
 0x30a11b5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30a11c7); f_030ae5ac(cpu); if cpu.returned_to != 0x30a11c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  cpu.push(0x30a11d3); f_030a0f84(cpu); if cpu.returned_to != 0x30a11d3 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x30a11e7; continue; }
  pc = 0x30a11dd; continue;
 }
 0x30a11dd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a11e7; continue; }
  pc = 0x30a11e1; continue;
 }
 0x30a11e1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a11e7 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a11f1); f_030a0ef8(cpu); if cpu.returned_to != 0x30a11f1 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a1201; continue; }
  pc = 0x30a11f7; continue;
 }
 0x30a11f7 => {
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x30a1217; continue; }
  pc = 0x30a11ff; continue;
 }
 0x30a11ff => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x30a1201; continue;
 }
 0x30a1201 => {
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  cpu.push(0x30a120f); f_030a0fb4(cpu); if cpu.returned_to != 0x30a120f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a1217; continue; }
  pc = 0x30a1213; continue;
 }
 0x30a1213 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x30a1217; continue;
 }
 0x30a1217 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a121b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a121b); }
 let mut pc: u32 = 0x30a121b;
 loop { match pc {
 0x30a121b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x315bdd3u32), 0x0u32); }
  if !cpu.zf { pc = 0x30a123b; continue; }
  pc = 0x30a1227; continue;
 }
 0x30a1227 => {
  { let v = 0xfffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x30a1233); f_030a1486(cpu); if cpu.returned_to != 0x30a1233 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315bdd3u32, v); }
  pc = 0x30a123b; continue;
 }
 0x30a123b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x315bdd3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1396(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1396); }
 let mut pc: u32 = 0x30a1396;
 loop { match pc {
 0x30a1396 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(92); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x32u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffa4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x30a13b5); f_0309f0c0(cpu); if cpu.returned_to != 0x30a13b5 { return; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ecx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.ecx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.ecx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffbcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.ecx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffb8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0x8u32.wrapping_add(cpu.ecx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r16(0xau32.wrapping_add(cpu.ecx)) as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffa4u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30a13f6); f_030ae5ac(cpu); if cpu.returned_to != 0x30a13f6 { return; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffa4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x300u16; cpu.w16(0xffffffd8u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u8 = 0x0u8; cpu.w8(0xffffffddu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = 0x0u16; cpu.w16(0xffffffe0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a142c); f_030ae9e6(cpu); if cpu.returned_to != 0x30a142c { return; }
  { let v: u32 = cpu.r32(0xffffffc0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xffffffb4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xffffffbcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xffffffb8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xffffffa8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xffffffa4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xffffffc6u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xffffffd0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xffffffc8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1486(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1486); }
 let mut pc: u32 = 0x30a1486;
 loop { match pc {
 0x30a1486 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x30a1494); f_030a0fd5(cpu); if cpu.returned_to != 0x30a1494 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a14f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a14f8); }
 let mut pc: u32 = 0x30a14f8;
 loop { match pc {
 0x30a14f8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x30aeb55u32; cpu.edx = v; }
  { let v: u32 = 0x30aec9eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315cd74u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315cd78u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1544(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1544); }
 let mut pc: u32 = 0x30a1544;
 loop { match pc {
 0x30a1544 => {
  { let v: u8 = 0x3u8; cpu.w8(0x315454cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x315be20u32.wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x30a1552, "Smsw");
  { let v: u16 = cpu.r16(0x315be1eu32.wrapping_add(cpu.bases[3])) & 0x6u16; cpu.w16(0x315be1eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x30a1565, "Sidt");
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x315be58u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x30a1596; continue; }
  pc = 0x30a157c; continue;
 }
 0x30a157c => {
  cpu.flat_ds(0x30a157c); cpu.push(0x30a1581); f_030a160f(cpu); if cpu.returned_to != 0x30a1581 { return; }
  { let v = cpu.r16(0x315b2a2u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  cpu.flat_ds(0x30a158f); cpu.push(0x30a1594); f_030a1679(cpu); if cpu.returned_to != 0x30a1594 { return; }
  pc = 0x30a160e; continue;
 }
 0x30a1596 => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x30a15ba; continue; }
  pc = 0x30a159f; continue;
 }
 0x30a159f => {
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x30af094u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x30a160e; continue;
 }
 0x30a15ba => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x30a15e2; continue; }
  pc = 0x30a15c3; continue;
 }
 0x30a15c3 => {
  cpu.flat_ds(0x30a15c3); cpu.push(0x30a15c8); f_030a160f(cpu); if cpu.returned_to != 0x30a15c8 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315b29cu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x30a15dd); cpu.push(0x30a15e0); cpu.call(t); if cpu.returned_to != 0x30a15e0 { return; }
  pc = 0x30a160e; continue;
 }
 0x30a15e2 => {
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x30a15fb; continue; }
  pc = 0x30a15eb; continue;
 }
 0x30a15eb => {
  { let v: u16 = cpu.r16(0x315b2a0u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x30a15f4); cpu.push(0x30a15f9); f_030b1ed2(cpu); if cpu.returned_to != 0x30a15f9 { return; }
  pc = 0x30a160e; continue;
 }
 0x30a15fb => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x30a1600); cpu.push(0x30a1605); f_030b1ed2(cpu); if cpu.returned_to != 0x30a1605 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a160e; continue; }
  pc = 0x30a1609; continue;
 }
 0x30a1609 => {
  cpu.flat_ds(0x30a1609); cpu.push(0x30a160e); f_030a162c(cpu); if cpu.returned_to != 0x30a160e { return; }
  pc = 0x30a160e; continue;
 }
 0x30a160e => {
  cpu.flat_ds(0x30a160e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a160f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a160f); }
 let mut pc: u32 = 0x30a160f;
 loop { match pc {
 0x30a160f => {
  { let v: u32 = 0x30af094u32; cpu.ecx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.ecx as u16), 13u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.ecx >> 8) as u8), 0x8eu8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a162c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a162c); }
 let mut pc: u32 = 0x30a162c;
 loop { match pc {
 0x30a162c => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315be18u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x315be1cu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x3u8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.sf != cpu.of) { pc = 0x30a1663; continue; }
  pc = 0x30a1653; continue;
 }
 0x30a1653 => {
  { let v: u32 = 0x30af024u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x30a165c); cpu.push(0x30a1661); f_030a1679(cpu); if cpu.returned_to != 0x30a1661 { return; }
  pc = 0x30a1678; continue;
 }
 0x30a1663 => {
  { let v: u32 = 0x30af094u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffdu32; cpu.ecx = v; }
  {  }
  pc = 0x30a1678; continue;
 }
 0x30a1678 => {
  cpu.flat_ds(0x30a1678); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1679(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1679); }
 let mut pc: u32 = 0x30a1679;
 loop { match pc {
 0x30a1679 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x315bdd8u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0x315bdd8u32) | 0x4u32; cpu.w32(0x315bdd8u32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x315bdd8u32), 0xfffffffdu32); cpu.w32(0x315bdd8u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1713(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1713); }
 let mut pc: u32 = 0x30a1713;
 loop { match pc {
 0x30a1713 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x315be18u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x315be1cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.sub8(cpu.r8(0x315be56u32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x30a173a; continue; }
  pc = 0x30a1733; continue;
 }
 0x30a1733 => {
  cpu.flat_ds(0x30a1733); cpu.push(0x30a1738); f_030a174e(cpu); if cpu.returned_to != 0x30a1738 { return; }
  pc = 0x30a174d; continue;
 }
 0x30a173a => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff9u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x315be1eu32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  {  }
  pc = 0x30a174d; continue;
 }
 0x30a174d => {
  cpu.flat_ds(0x30a174d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a174e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a174e); }
 let mut pc: u32 = 0x30a174e;
 loop { match pc {
 0x30a174e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x315bdd8u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x315be1eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315bdd8u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a176f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a176f); }
 let mut pc: u32 = 0x30a176f;
 loop { match pc {
 0x30a176f => {
  {  }
  cpu.unimplemented(0x30a1770, "Fnsave");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1774(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1774); }
 let mut pc: u32 = 0x30a1774;
 loop { match pc {
 0x30a1774 => {
  cpu.unimplemented(0x30a1774, "Frstor");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1778); }
 let mut pc: u32 = 0x30a1778;
 loop { match pc {
 0x30a1778 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x315454du32), 0x0u8); }
  if cpu.zf { pc = 0x30a1799; continue; }
  pc = 0x30a1783; continue;
 }
 0x30a1783 => {
  { let v: u32 = 0x30a176fu32; cpu.edx = v; }
  { let v: u32 = 0x30a1774u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315ce04u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315ce08u32, v); }
  pc = 0x30a1799; continue;
 }
 0x30a1799 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x315ce00u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30a17a6); f_030b2000(cpu); if cpu.returned_to != 0x30a17a6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a17a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a17a9); }
 let mut pc: u32 = 0x30a17a9;
 loop { match pc {
 0x30a17a9 => {
  { cpu.sub8(cpu.r8(0x315454du32), 0x0u8); }
  if !cpu.zf { f_030a1778(cpu); return; }
  pc = 0x30a17b2; continue;
 }
 0x30a17b2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a17b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a17b3); }
 let mut pc: u32 = 0x30a17b3;
 loop { match pc {
 0x30a17b3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x315454cu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x30a17fb; continue; }
  pc = 0x30a17be; continue;
 }
 0x30a17be => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x315454du32, v); }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if !cpu.zf { pc = 0x30a17e6; continue; }
  pc = 0x30a17dd; continue;
 }
 0x30a17dd => {
  cpu.push(0x30a17e2); f_030a1778(cpu); if cpu.returned_to != 0x30a17e2 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30a17e6; continue;
 }
 0x30a17e6 => {
  { cpu.sub8(cpu.r8(0x315be54u32), 0x0u8); }
  if !cpu.zf { pc = 0x30a17fb; continue; }
  pc = 0x30a17ef; continue;
 }
 0x30a17ef => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x315454cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x315454du32, v); }
  pc = 0x30a17fb; continue;
 }
 0x30a17fb => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a17fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a17fd); }
 let mut pc: u32 = 0x30a17fd;
 loop { match pc {
 0x30a17fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a17fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a17fe); }
 let mut pc: u32 = 0x30a17fe;
 loop { match pc {
 0x30a17fe => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x30a1811); f_0309f0c0(cpu); if cpu.returned_to != 0x30a1811 { return; }
  pc = 0x30a1832; continue;
 }
 0x30a1813 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x314b9d4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x30a1832; continue;
 }
 0x30a1832 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1813; continue; }
  pc = 0x30a1838; continue;
 }
 0x30a1838 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a183c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a183c); }
 let mut pc: u32 = 0x30a183c;
 loop { match pc {
 0x30a183c => {
  if !cpu.cf { pc = 0x30a184c; continue; }
  pc = 0x30a183e; continue;
 }
 0x30a183e => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x30a1849); f_030b2032(cpu); if cpu.returned_to != 0x30a1849 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x30a184e; continue;
 }
 0x30a184c => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x30a184e; continue;
 }
 0x30a184e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a184f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a184f); }
 let mut pc: u32 = 0x30a184f;
 loop { match pc {
 0x30a184f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x30a186d); f_030a1873(cpu); if cpu.returned_to != 0x30a186d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1873(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1873); }
 let mut pc: u32 = 0x30a1873;
 loop { match pc {
 0x30a1873 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x30a187f; continue;
 }
 0x30a187f => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x30a188a; continue; }
  pc = 0x30a1887; continue;
 }
 0x30a1887 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x30a187f; continue;
 }
 0x30a188a => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x83u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30a18bd; continue; }
  pc = 0x30a18b7; continue;
 }
 0x30a18b7 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x30a18bd; continue;
 }
 0x30a18bd => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x3u8); }
  if cpu.zf { pc = 0x30a1940; continue; }
  pc = 0x30a18ca; continue;
 }
 0x30a18ca => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a1940; continue; }
  pc = 0x30a18d3; continue;
 }
 0x30a18d3 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30a18da); f_030b20a3(cpu); if cpu.returned_to != 0x30a18da { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a1940; continue; }
  pc = 0x30a18de; continue;
 }
 0x30a18de => {
  { cpu.and8(cpu.r8(0x29u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x30a1908; continue; }
  pc = 0x30a18e5; continue;
 }
 0x30a18e5 => {
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x30a1908; continue; }
  pc = 0x30a18ea; continue;
 }
 0x30a18ea => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x30a18fe); f_0309f3db(cpu); if cpu.returned_to != 0x30a18fe { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a1a53; continue;
 }
 0x30a1908 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x40u8); }
  if cpu.zf { pc = 0x30a1940; continue; }
  pc = 0x30a190f; continue;
 }
 0x30a190f => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a1940; continue; }
  pc = 0x30a1926; continue;
 }
 0x30a1926 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x30a1930; continue;
 }
 0x30a1930 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30a193b); f_030b2032(cpu); if cpu.returned_to != 0x30a193b { return; }
  pc = 0x30a1a53; continue;
 }
 0x30a1940 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x30a19e2; continue; }
  pc = 0x30a1949; continue;
 }
 0x30a1949 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x30a1930; continue; }
  pc = 0x30a1950; continue;
 }
 0x30a1950 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1930; continue; }
  pc = 0x30a195b; continue;
 }
 0x30a195b => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a196e; continue; }
  pc = 0x30a1969; continue;
 }
 0x30a1969 => {
  { let v: u32 = 0x180u32; cpu.eax = v; }
  pc = 0x30a196e; continue;
 }
 0x30a196e => {
  { let v: u32 = cpu.r32(0x315ce20u32); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x30a1983; continue; }
  pc = 0x30a197e; continue;
 }
 0x30a197e => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x30a1983; continue;
 }
 0x30a1983 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1930; continue; }
  pc = 0x30a199a; continue;
 }
 0x30a199a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a19e2; continue; }
  pc = 0x30a19a8; continue;
 }
 0x30a19a8 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1930; continue; }
  pc = 0x30a19bd; continue;
 }
 0x30a19bd => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2cu32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), ((cpu.ecx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1930; continue; }
  pc = 0x30a19dc; continue;
 }
 0x30a19dc => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x30a19e2; continue;
 }
 0x30a19e2 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30a19e9); f_030b20ca(cpu); if cpu.returned_to != 0x30a19e9 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30a19f4); f_030b20a3(cpu); if cpu.returned_to != 0x30a19f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a19fb; continue; }
  pc = 0x30a19f8; continue;
 }
 0x30a19f8 => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x30a19fb; continue;
 }
 0x30a19fb => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x30a1a0a; continue; }
  pc = 0x30a1a07; continue;
 }
 0x30a1a07 => {
  { let v: u8 = (cpu.edx as u8) | 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30a1a0a; continue;
 }
 0x30a1a0a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x30a1a14; continue; }
  pc = 0x30a1a11; continue;
 }
 0x30a1a11 => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30a1a14; continue;
 }
 0x30a1a14 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x30a1a1e; continue; }
  pc = 0x30a1a1b; continue;
 }
 0x30a1a1b => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30a1a1e; continue;
 }
 0x30a1a1e => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x30a1a28; continue; }
  pc = 0x30a1a25; continue;
 }
 0x30a1a25 => {
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x30a1a28; continue;
 }
 0x30a1a28 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if cpu.zf { pc = 0x30a1a3c; continue; }
  pc = 0x30a1a35; continue;
 }
 0x30a1a35 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x30a1a4a; continue; }
  pc = 0x30a1a3a; continue;
 }
 0x30a1a3a => {
  pc = 0x30a1a48; continue;
 }
 0x30a1a3c => {
  { cpu.sub32(cpu.r32(0x315b4cdu32), 0x200u32); }
  if !cpu.zf { pc = 0x30a1a4a; continue; }
  pc = 0x30a1a48; continue;
 }
 0x30a1a48 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x30a1a4a; continue;
 }
 0x30a1a4a => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x30a1a51); f_030b211f(cpu); if cpu.returned_to != 0x30a1a51 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x30a1a53; continue;
 }
 0x30a1a53 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
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
pub fn f_030a1a5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1a5c); }
 let mut pc: u32 = 0x30a1a5c;
 loop { match pc {
 0x30a1a5c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x30a1a6a); f_030b20ca(cpu); if cpu.returned_to != 0x30a1a6a { return; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a1a77); f_030b211f(cpu); if cpu.returned_to != 0x30a1a77 { return; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x30a1ab7; continue; }
  pc = 0x30a1a7c; continue;
 }
 0x30a1a7c => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30a1a9c; continue; }
  pc = 0x30a1a8f; continue;
 }
 0x30a1a8f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30a1a9a); f_030b2032(cpu); if cpu.returned_to != 0x30a1a9a { return; }
  pc = 0x30a1ab9; continue;
 }
 0x30a1a9c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1a8f; continue; }
  pc = 0x30a1ab7; continue;
 }
 0x30a1ab7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30a1ab9; continue;
 }
 0x30a1ab9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1ac1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1ac1); }
 let mut pc: u32 = 0x30a1ac1;
 loop { match pc {
 0x30a1ac1 => {
  { let v: u32 = 0x32d0fecu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1ac7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1ac7); }
 let mut pc: u32 = 0x30a1ac7;
 loop { match pc {
 0x30a1ac7 => {
  { let v: u32 = 0x32d0ff0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1acd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1acd); }
 let mut pc: u32 = 0x30a1acd;
 loop { match pc {
 0x30a1acd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x30a1add; continue; }
  pc = 0x30a1ad1; continue;
 }
 0x30a1ad1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x315ce24u32)); }
  if (cpu.cf || cpu.zf) { f_030b2134(cpu); return; }
  pc = 0x30a1add; continue;
 }
 0x30a1add => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x30a1ae7); f_0309f3db(cpu); if cpu.returned_to != 0x30a1ae7 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1aed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1aed); }
 let mut pc: u32 = 0x30a1aed;
 loop { match pc {
 0x30a1aed => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x30a1afc); f_030b20ca(cpu); if cpu.returned_to != 0x30a1afc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a1b18; continue; }
  pc = 0x30a1b04; continue;
 }
 0x30a1b04 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x30a1b09; continue;
 }
 0x30a1b09 => {
  cpu.push(0x30a1b0e); f_0309f3db(cpu); if cpu.returned_to != 0x30a1b0e { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a1be0; continue;
 }
 0x30a1b18 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x30a1b23; continue; }
  pc = 0x30a1b1c; continue;
 }
 0x30a1b1c => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30a1b09; continue;
 }
 0x30a1b23 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x30a1b4f; continue; }
  pc = 0x30a1b27; continue;
 }
 0x30a1b27 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a1bde; continue; }
  pc = 0x30a1b40; continue;
 }
 0x30a1b40 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x30a1b45; continue;
 }
 0x30a1b45 => {
  cpu.push(0x30a1b4a); f_030b2032(cpu); if cpu.returned_to != 0x30a1b4a { return; }
  pc = 0x30a1be0; continue;
 }
 0x30a1b4f => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x30a1b55; continue;
 }
 0x30a1b55 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a1b79; continue; }
  pc = 0x30a1b72; continue;
 }
 0x30a1b72 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x30a1b45; continue;
 }
 0x30a1b79 => {
  if cpu.zf { pc = 0x30a1bde; continue; }
  pc = 0x30a1b7b; continue;
 }
 0x30a1b7b => {
  { let v: u32 = cpu.edi ^ cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a1bbb; continue;
 }
 0x30a1b8a => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x1au32); }
  if !cpu.zf { pc = 0x30a1bab; continue; }
  pc = 0x30a1b93; continue;
 }
 0x30a1b93 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x30a1ba9); f_030a1dbd(cpu); if cpu.returned_to != 0x30a1ba9 { return; }
  pc = 0x30a1bde; continue;
 }
 0x30a1bab => {
  { cpu.sub32(cpu.ebx, 0xdu32); }
  if cpu.zf { pc = 0x30a1bb9; continue; }
  pc = 0x30a1bb0; continue;
 }
 0x30a1bb0 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x30a1bb9; continue;
 }
 0x30a1bb9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x30a1bbb; continue;
 }
 0x30a1bbb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x30a1b8a; continue; }
  pc = 0x30a1bc1; continue;
 }
 0x30a1bc1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if !cpu.zf { pc = 0x30a1bde; continue; }
  pc = 0x30a1bd6; continue;
 }
 0x30a1bd6 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1b55; continue; }
  pc = 0x30a1bde; continue;
 }
 0x30a1bde => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x30a1be0; continue;
 }
 0x30a1be0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1be8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1be8); }
 let mut pc: u32 = 0x30a1be8;
 loop { match pc {
 0x30a1be8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x30a1bf9); f_030b20ca(cpu); if cpu.returned_to != 0x30a1bf9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a1c14; continue; }
  pc = 0x30a1c00; continue;
 }
 0x30a1c00 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x30a1c05; continue;
 }
 0x30a1c05 => {
  cpu.push(0x30a1c0a); f_0309f3db(cpu); if cpu.returned_to != 0x30a1c0a { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a1db6; continue;
 }
 0x30a1c14 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x30a1c1f; continue; }
  pc = 0x30a1c18; continue;
 }
 0x30a1c18 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30a1c05; continue;
 }
 0x30a1c1f => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x30a1c56; continue; }
  pc = 0x30a1c23; continue;
 }
 0x30a1c23 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = cpu.rcl16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), 1u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30a1c56; continue; }
  pc = 0x30a1c46; continue;
 }
 0x30a1c46 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30a1c51); f_030b2032(cpu); if cpu.returned_to != 0x30a1c51 { return; }
  pc = 0x30a1db6; continue;
 }
 0x30a1c56 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x30a1c91; continue; }
  pc = 0x30a1c5c; continue;
 }
 0x30a1c5c => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1c46; continue; }
  pc = 0x30a1c76; continue;
 }
 0x30a1c76 => {
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x30a1db3; continue; }
  pc = 0x30a1c7f; continue;
 }
 0x30a1c7f => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x30a1c89); f_0309f3db(cpu); if cpu.returned_to != 0x30a1c89 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a1db3; continue;
 }
 0x30a1c91 => {
  cpu.push(0x30a1c96); f_030b2169(cpu); if cpu.returned_to != 0x30a1c96 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if !cpu.cf { pc = 0x30a1ca4; continue; }
  pc = 0x30a1c9f; continue;
 }
 0x30a1c9f => {
  cpu.push(0x30a1ca4); f_0309e6bb(cpu); if cpu.returned_to != 0x30a1ca4 { return; }
  pc = 0x30a1ca4; continue;
 }
 0x30a1ca4 => {
  { let v: u32 = 0x200u32; cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x230u32); }
  if !cpu.cf { pc = 0x30a1cb6; continue; }
  pc = 0x30a1cb1; continue;
 }
 0x30a1cb1 => {
  { let v: u32 = 0x80u32; cpu.edi = v; }
  pc = 0x30a1cb6; continue;
 }
 0x30a1cb6 => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a1cd2; continue;
 }
 0x30a1cd2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x30a1d7d; continue; }
  pc = 0x30a1cde; continue;
 }
 0x30a1cde => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x30a1d32; continue; }
  pc = 0x30a1cea; continue;
 }
 0x30a1cea => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x30a1d32; continue; }
  pc = 0x30a1cf6; continue;
 }
 0x30a1cf6 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1c46; continue; }
  pc = 0x30a1d12; continue;
 }
 0x30a1d12 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x30a1d2a; continue; }
  pc = 0x30a1d16; continue;
 }
 0x30a1d16 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x30a1d20); f_0309f3db(cpu); if cpu.returned_to != 0x30a1d20 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x30a1db6; continue;
 }
 0x30a1d2a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a1d32; continue;
 }
 0x30a1d32 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1cd2; continue; }
  pc = 0x30a1d50; continue;
 }
 0x30a1d50 => {
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1c46; continue; }
  pc = 0x30a1d6c; continue;
 }
 0x30a1d6c => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1d16; continue; }
  pc = 0x30a1d70; continue;
 }
 0x30a1d70 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x30a1cd2; continue;
 }
 0x30a1d7d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30a1db3; continue; }
  pc = 0x30a1d81; continue;
 }
 0x30a1d81 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a1c46; continue; }
  pc = 0x30a1d9e; continue;
 }
 0x30a1d9e => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x30a1db3; continue; }
  pc = 0x30a1da2; continue;
 }
 0x30a1da2 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x30a1dac); f_0309f3db(cpu); if cpu.returned_to != 0x30a1dac { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  pc = 0x30a1db6; continue;
 }
 0x30a1db3 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x30a1db6; continue;
 }
 0x30a1db6 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1dbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1dbd); }
 let mut pc: u32 = 0x30a1dbd;
 loop { match pc {
 0x30a1dbd => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u8 = 0x42u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a1df7; continue; }
  pc = 0x30a1de6; continue;
 }
 0x30a1de6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30a1df0); f_030b2032(cpu); if cpu.returned_to != 0x30a1df0 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a1dfa; continue;
 }
 0x30a1df7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x30a1dfa; continue;
 }
 0x30a1dfa => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1e00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1e00); }
 let mut pc: u32 = 0x30a1e00;
 loop { match pc {
 0x30a1e00 => {
  pc = 0x30b2172; continue;
 }
 0x30b2172 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30b218f; continue; }
  pc = 0x30b2183; continue;
 }
 0x30b2183 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30b218d); f_030b2032(cpu); if cpu.returned_to != 0x30b218d { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b218f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1e05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1e05); }
 let mut pc: u32 = 0x30a1e05;
 loop { match pc {
 0x30a1e05 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u8 = cpu.xor8((cpu.edx as u8), (cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x47u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a1e34; continue; }
  pc = 0x30a1e26; continue;
 }
 0x30a1e26 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x30a1e2b; continue;
 }
 0x30a1e2b => {
  cpu.push(0x30a1e30); f_0309f3db(cpu); if cpu.returned_to != 0x30a1e30 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30a1e81; continue;
 }
 0x30a1e34 => {
  { let v: u8 = 0x19u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = 0x5cu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x41u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = 0x3au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.esp), v); }
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
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x30a1e78; continue; }
  pc = 0x30a1e62; continue;
 }
 0x30a1e62 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30a1e69); f_030ad84c(cpu); if cpu.returned_to != 0x30a1e69 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a1e76; continue; }
  pc = 0x30a1e6f; continue;
 }
 0x30a1e6f => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30a1e2b; continue;
 }
 0x30a1e76 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x30a1e78; continue;
 }
 0x30a1e78 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x30a1e81); f_0308fb14(cpu); if cpu.returned_to != 0x30a1e81 { return; }
  pc = 0x30a1e81; continue;
 }
 0x30a1e81 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
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
pub fn f_030a1e8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1e8d); }
 let mut pc: u32 = 0x30a1e8d;
 loop { match pc {
 0x30a1e8d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x3bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a1eaa; continue; }
  pc = 0x30a1e9e; continue;
 }
 0x30a1e9e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x30a1ea8); f_030b2032(cpu); if cpu.returned_to != 0x30a1ea8 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a1eaa => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1eae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1eae); }
 let mut pc: u32 = 0x30a1eae;
 loop { match pc {
 0x30a1eae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x30a1eb7); f_03094d04(cpu); if cpu.returned_to != 0x30a1eb7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a1ed8; continue; }
  pc = 0x30a1ebe; continue;
 }
 0x30a1ebe => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x30a1ec5); f_030a1edb(cpu); if cpu.returned_to != 0x30a1ec5 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x30a1ed8; continue; }
  pc = 0x30a1eca; continue;
 }
 0x30a1eca => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x30a1ed6; continue; }
  pc = 0x30a1ece; continue;
 }
 0x30a1ece => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a1ed6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x30a1ed8; continue;
 }
 0x30a1ed8 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1edb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1edb); }
 let mut pc: u32 = 0x30a1edb;
 loop { match pc {
 0x30a1edb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x30a1ee9); f_030a1dbd(cpu); if cpu.returned_to != 0x30a1ee9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1eec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1eec); }
 let mut pc: u32 = 0x30a1eec;
 loop { match pc {
 0x30a1eec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.push(0x30a1ef8); f_030b2193(cpu); if cpu.returned_to != 0x30a1ef8 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1efd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1efd); }
 let mut pc: u32 = 0x30a1efd;
 loop { match pc {
 0x30a1efd => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xcfu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0x30u32; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x30a1f20; continue; }
  pc = 0x30a1f19; continue;
 }
 0x30a1f19 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a1f20); f_030b2237(cpu); if cpu.returned_to != 0x30a1f20 { return; }
  pc = 0x30a1f20; continue;
 }
 0x30a1f20 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x30a1f3b; continue; }
  pc = 0x30a1f2a; continue;
 }
 0x30a1f2a => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  pc = 0x30a1f3b; continue;
 }
 0x30a1f3b => {
  { let v: u32 = 0x30a1eecu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x30a1f47); f_0309f568(cpu); if cpu.returned_to != 0x30a1f47 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x30a1f62; continue; }
  pc = 0x30a1f4d; continue;
 }
 0x30a1f4d => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x30a1f62); f_030ad4ba(cpu); if cpu.returned_to != 0x30a1f62 { return; }
  pc = 0x30a1f62; continue;
 }
 0x30a1f62 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x30a1f6d; continue; }
  pc = 0x30a1f68; continue;
 }
 0x30a1f68 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x30a1f6d; continue;
 }
 0x30a1f6d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.or32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1f7c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1f7c); }
 let mut pc: u32 = 0x30a1f7c;
 loop { match pc {
 0x30a1f7c => {
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x30a1f86; continue; }
  pc = 0x30a1f81; continue;
 }
 0x30a1f81 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x30a1f8f; continue; }
  pc = 0x30a1f86; continue;
 }
 0x30a1f86 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x30a1f8d; continue; }
  pc = 0x30a1f8b; continue;
 }
 0x30a1f8b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x30a1f8d; continue;
 }
 0x30a1f8d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x30a1f8f; continue;
 }
 0x30a1f8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a1f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a1f90); }
 let mut pc: u32 = 0x30a1f90;
 loop { match pc {
 0x30a1f90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a1fb4; continue; }
  pc = 0x30a1fa5; continue;
 }
 0x30a1fa5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x30a1fb4; continue; }
  pc = 0x30a1fab; continue;
 }
 0x30a1fab => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30a1fb4; continue;
 }
 0x30a1fb4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30a2072; continue; }
  pc = 0x30a1fc0; continue;
 }
 0x30a1fc0 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x30a2072; continue; }
  pc = 0x30a1fc9; continue;
 }
 0x30a1fc9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x30a1fd4); f_030b22ae(cpu); if cpu.returned_to != 0x30a1fd4 { return; }
  cpu.push(0x30a1fd9); f_030a1f7c(cpu); if cpu.returned_to != 0x30a1fd9 { return; }
  cpu.push(0x30a1fde); f_030b2306(cpu); if cpu.returned_to != 0x30a1fde { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x30a1fe3; continue;
 }
 0x30a1fe3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x30a1ffb; continue; }
  pc = 0x30a1feb; continue;
 }
 0x30a1feb => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1fe3; continue; }
  pc = 0x30a1ffb; continue;
 }
 0x30a1ffb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x315ce7cu32), 0x0u32); }
  if cpu.zf { pc = 0x30a2019; continue; }
  pc = 0x30a2005; continue;
 }
 0x30a2005 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x30a201b; continue;
 }
 0x30a2019 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30a201b; continue;
 }
 0x30a201b => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x315ce7cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a2038; continue; }
  pc = 0x30a2027; continue;
 }
 0x30a2027 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x30a2038; continue;
 }
 0x30a2038 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30a1fc9; continue; }
  pc = 0x30a2040; continue;
 }
 0x30a2040 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x30a204f; continue; }
  pc = 0x30a2047; continue;
 }
 0x30a2047 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a204f; continue;
 }
 0x30a204f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x30a2059); f_030b257a(cpu); if cpu.returned_to != 0x30a2059 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x30a206b; continue; }
  pc = 0x30a2068; continue;
 }
 0x30a2068 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30a2072; continue;
 }
 0x30a206b => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a2072; continue;
 }
 0x30a2072 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x30a2081; continue; }
  pc = 0x30a2079; continue;
 }
 0x30a2079 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a2081; continue;
 }
 0x30a2081 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30a2129; continue; }
  pc = 0x30a208c; continue;
 }
 0x30a208c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x30a2097); f_030b22ae(cpu); if cpu.returned_to != 0x30a2097 { return; }
  cpu.push(0x30a209c); f_030a1f7c(cpu); if cpu.returned_to != 0x30a209c { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x30a20af; continue; }
  pc = 0x30a20a4; continue;
 }
 0x30a20a4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x30a20af; continue; }
  pc = 0x30a20ad; continue;
 }
 0x30a20ad => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x30a20af; continue;
 }
 0x30a20af => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x30a20ba); f_030b22ae(cpu); if cpu.returned_to != 0x30a20ba { return; }
  cpu.push(0x30a20bf); f_030a1f7c(cpu); if cpu.returned_to != 0x30a20bf { return; }
  cpu.push(0x30a20c4); f_030b2306(cpu); if cpu.returned_to != 0x30a20c4 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x30a20c9; continue;
 }
 0x30a20c9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x30a20e1; continue; }
  pc = 0x30a20d1; continue;
 }
 0x30a20d1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30a20c9; continue; }
  pc = 0x30a20e1; continue;
 }
 0x30a20e1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x315ce7cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a20fd; continue; }
  pc = 0x30a20eb; continue;
 }
 0x30a20eb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x30a20fd; continue;
 }
 0x30a20fd => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x315ce7cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a211d; continue; }
  pc = 0x30a210a; continue;
 }
 0x30a210a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315ce81u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x30a211f; continue;
 }
 0x30a211d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x30a211f; continue;
 }
 0x30a211f => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x30a20af; continue; }
  pc = 0x30a2127; continue;
 }
 0x30a2127 => {
  pc = 0x30a2138; continue;
 }
 0x30a2129 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x30a2138; continue; }
  pc = 0x30a2136; continue;
 }
 0x30a2136 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x30a2138; continue;
 }
 0x30a2138 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x30a216e; continue; }
  pc = 0x30a2140; continue;
 }
 0x30a2140 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x30a216e; continue; }
  pc = 0x30a2145; continue;
 }
 0x30a2145 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x30a2157; continue; }
  pc = 0x30a2152; continue;
 }
 0x30a2152 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30a2157; continue;
 }
 0x30a2157 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x30a216e; continue; }
  pc = 0x30a2161; continue;
 }
 0x30a2161 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x30a2157; continue;
 }
 0x30a216e => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a217b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a217b); }
 let mut pc: u32 = 0x30a217b;
 loop { match pc {
 0x30a217b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x2au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x76cu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0xau32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x2cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x2au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.ecx as u16).wrapping_sub(0x76cu16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.esi); cpu.esi = v; }
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x30a222d; continue; }
  pc = 0x30a220b; continue;
 }
 0x30a220b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x17u32); }
  if cpu.zf { pc = 0x30a222d; continue; }
  pc = 0x30a2211; continue;
 }
 0x30a2211 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x6u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a222d; continue;
 }
 0x30a222d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a223e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a223e); }
 let mut pc: u32 = 0x30a223e;
 loop { match pc {
 0x30a223e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = 0xcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = 0x314b9dcu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0xf4fb7ea1u32); }
  if (cpu.sf == cpu.of) { pc = 0x30a2272; continue; }
  pc = 0x30a2268; continue;
 }
 0x30a2268 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x30a238b; continue;
 }
 0x30a2272 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x30a2283; continue;
 }
 0x30a2283 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x30a228d; continue; }
  pc = 0x30a2287; continue;
 }
 0x30a2287 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  cpu.poll(); pc = 0x30a2283; continue;
 }
 0x30a228d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a2268; continue; }
  pc = 0x30a2291; continue;
 }
 0x30a2291 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x30a229c); f_030a28a2(cpu); if cpu.returned_to != 0x30a229c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a22a5; continue; }
  pc = 0x30a22a0; continue;
 }
 0x30a22a0 => {
  { let v: u32 = 0x314b9f6u32; cpu.ebx = v; }
  pc = 0x30a22a5; continue;
 }
 0x30a22a5 => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.edi.wrapping_mul(2))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30a22d2; continue; }
  pc = 0x30a22d1; continue;
 }
 0x30a22d1 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x30a22d2; continue;
 }
 0x30a22d2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x30a22f3; continue;
 }
 0x30a22f3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30a2300; continue; }
  pc = 0x30a22f7; continue;
 }
 0x30a22f7 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30a22f3; continue;
 }
 0x30a2300 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x30a230d); f_030a2762(cpu); if cpu.returned_to != 0x30a230d { return; }
  cpu.push(0x30a2312); f_030a23fe(cpu); if cpu.returned_to != 0x30a2312 { return; }
  { let v: u32 = cpu.r32(0x315beb0u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x30a2328; continue; }
  pc = 0x30a2321; continue;
 }
 0x30a2321 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x30a2328); f_030a29f6(cpu); if cpu.returned_to != 0x30a2328 { return; }
  pc = 0x30a2328; continue;
 }
 0x30a2328 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2334; continue; }
  pc = 0x30a232e; continue;
 }
 0x30a232e => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x315beb4u32)); cpu.esi = v; }
  pc = 0x30a2334; continue;
 }
 0x30a2334 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30a2341; continue; }
  pc = 0x30a2338; continue;
 }
 0x30a2338 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x30a2334; continue;
 }
 0x30a2341 => {
  { cpu.sub32(cpu.edi, 0x63deu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a2268; continue; }
  pc = 0x30a234d; continue;
 }
 0x30a234d => {
  if !cpu.zf { pc = 0x30a236b; continue; }
  pc = 0x30a234f; continue;
 }
 0x30a234f => {
  { let v: u32 = cpu.r32(0x315beb0u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x15180u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x30a2268; continue; }
  pc = 0x30a2362; continue;
 }
 0x30a2362 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x30a2389; continue; }
  pc = 0x30a2366; continue;
 }
 0x30a2366 => {
  cpu.poll(); pc = 0x30a2268; continue;
 }
 0x30a236b => {
  { let v: u32 = 0xffff9c21u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 5u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 7u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edi); cpu.esi = v; }
  pc = 0x30a2389; continue;
 }
 0x30a2389 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x30a238b; continue;
 }
 0x30a238b => {
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
pub fn f_030a2392(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2392); }
 let mut pc: u32 = 0x30a2392;
 loop { match pc {
 0x30a2392 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x315bf02u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x315bf02u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x315bf02u32, v); }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315bf02u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x315bf02u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x315bf02u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x315bf02u32, v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x315bf02u32), cpu.edx); cpu.w32(0x315bf02u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a23fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a23fe); }
 let mut pc: u32 = 0x30a23fe;
 loop { match pc {
 0x30a23e6 => {
  { let v: u8 = cpu.r8(0x315bf02u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x30a23f6; continue; }
  pc = 0x30a23f1; continue;
 }
 0x30a23f1 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.zf { pc = 0x30a23fd; continue; }
  pc = 0x30a23f6; continue;
 }
 0x30a23f6 => {
  { let v: u8 = cpu.or8(cpu.r8(0x315bf02u32), 0x2u8); cpu.w8(0x315bf02u32, v); }
  pc = 0x30a23fd; continue;
 }
 0x30a23fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a23fe => {
  { let v: u32 = 0x314b050u32; cpu.eax = v; }
  cpu.push(0x30a2408); f_030ad9b1(cpu); if cpu.returned_to != 0x30a2408 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x30a23e6; continue; }
  pc = 0x30a240c; continue;
 }
 0x30a240c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x315beb0u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315beb8u32, v); }
  { let v: u32 = 0x315bebcu32; cpu.edx = v; }
  cpu.push(0x30a242c); f_030a2467(cpu); if cpu.returned_to != 0x30a242c { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x30a2694; continue; }
  pc = 0x30a2438; continue;
 }
 0x30a2438 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x315bedbu32, v); }
  pc = 0x30a2758; continue;
 }
 0x30a2694 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x315bedbu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x315beb0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315beb8u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a26ba); f_030a2467(cpu); if cpu.returned_to != 0x30a26ba { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x315beb0u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x315beb4u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, 0x2cu32); }
  if !cpu.zf { pc = 0x30a26e3; continue; }
  pc = 0x30a26d6; continue;
 }
 0x30a26d6 => {
  { let v: u32 = 0x315be68u32; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x30a26e1); f_030a2590(cpu); if cpu.returned_to != 0x30a26e1 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x30a26e3; continue;
 }
 0x30a26e3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if !cpu.zf { pc = 0x30a2758; continue; }
  pc = 0x30a26ec; continue;
 }
 0x30a26ec => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x315be8cu32; cpu.edx = v; }
  cpu.push(0x30a26f9); f_030a2590(cpu); if cpu.returned_to != 0x30a26f9 { return; }
  { let v: u32 = cpu.r32(0x315beb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xe10u32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x315be94u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315beb4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x315be90u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x315beb4u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x315be8cu32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315be94u32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x315be90u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x315be8cu32, v); }
  pc = 0x30a2758; continue;
 }
 0x30a2758 => {
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
pub fn f_030a2443(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2443); }
 let mut pc: u32 = 0x30a2443;
 loop { match pc {
 0x30a2443 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x30a2449; continue;
 }
 0x30a2449 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x30a2462; continue; }
  pc = 0x30a2452; continue;
 }
 0x30a2452 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2462; continue; }
  pc = 0x30a2457; continue;
 }
 0x30a2457 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x30a2449; continue;
 }
 0x30a2462 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2467(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2467); }
 let mut pc: u32 = 0x30a2467;
 loop { match pc {
 0x30a2467 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x3au32); }
  if !cpu.zf { pc = 0x30a247f; continue; }
  pc = 0x30a247e; continue;
 }
 0x30a247e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x30a247f; continue;
 }
 0x30a247f => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x30a2481; continue;
 }
 0x30a2481 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x30a24a8; continue; }
  pc = 0x30a2488; continue;
 }
 0x30a2488 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if cpu.zf { pc = 0x30a24a8; continue; }
  pc = 0x30a2491; continue;
 }
 0x30a2491 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if cpu.zf { pc = 0x30a24a8; continue; }
  pc = 0x30a2496; continue;
 }
 0x30a2496 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x30a24a8; continue; }
  pc = 0x30a249b; continue;
 }
 0x30a249b => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x30a24a5; continue; }
  pc = 0x30a24a0; continue;
 }
 0x30a24a0 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a24a8; continue; }
  pc = 0x30a24a5; continue;
 }
 0x30a24a5 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = 0x30a2481; continue;
 }
 0x30a24a8 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a24bd; continue; }
  pc = 0x30a24b5; continue;
 }
 0x30a24b5 => {
  { let v: u32 = 0x1eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x30a24bd; continue;
 }
 0x30a24bd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
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
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x30a24fa; continue; }
  pc = 0x30a24f3; continue;
 }
 0x30a24f3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x30a24ff; continue;
 }
 0x30a24fa => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x30a2501; continue; }
  pc = 0x30a24ff; continue;
 }
 0x30a24ff => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x30a2501; continue;
 }
 0x30a2501 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x30a2586; continue; }
  pc = 0x30a2512; continue;
 }
 0x30a2512 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2586; continue; }
  pc = 0x30a2517; continue;
 }
 0x30a2517 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x30a252f); f_030a2443(cpu); if cpu.returned_to != 0x30a252f { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x30a2559; continue; }
  pc = 0x30a253a; continue;
 }
 0x30a253a => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x30a2544); f_030a2443(cpu); if cpu.returned_to != 0x30a2544 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x30a2559; continue; }
  pc = 0x30a254f; continue;
 }
 0x30a254f => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x30a2557); f_030a2443(cpu); if cpu.returned_to != 0x30a2557 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x30a2559; continue;
 }
 0x30a2559 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x30a2586; continue; }
  pc = 0x30a2582; continue;
 }
 0x30a2582 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x30a2586; continue;
 }
 0x30a2586 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2590); }
 let mut pc: u32 = 0x30a2590;
 loop { match pc {
 0x30a2590 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xffffffffu32; cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x4au32); }
  if !cpu.zf { pc = 0x30a25af; continue; }
  pc = 0x30a25a8; continue;
 }
 0x30a25a8 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x30a25af; continue;
 }
 0x30a25af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4du32); }
  if !cpu.zf { pc = 0x30a25bb; continue; }
  pc = 0x30a25b8; continue;
 }
 0x30a25b8 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x30a25bb; continue;
 }
 0x30a25bb => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x30a25c9); f_030a2443(cpu); if cpu.returned_to != 0x30a25c9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30a25da; continue; }
  pc = 0x30a25d1; continue;
 }
 0x30a25d1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x30a2627; continue;
 }
 0x30a25da => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x30a2620; continue; }
  pc = 0x30a25eb; continue;
 }
 0x30a25eb => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x30a25f7); f_030a2443(cpu); if cpu.returned_to != 0x30a25f7 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x30a2620; continue; }
  pc = 0x30a260b; continue;
 }
 0x30a260b => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x30a2617); f_030a2443(cpu); if cpu.returned_to != 0x30a2617 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  pc = 0x30a2620; continue;
 }
 0x30a2620 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x30a2627; continue;
 }
 0x30a2627 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x30a2678; continue; }
  pc = 0x30a2642; continue;
 }
 0x30a2642 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x30a264e); f_030a2443(cpu); if cpu.returned_to != 0x30a264e { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x30a2678; continue; }
  pc = 0x30a2659; continue;
 }
 0x30a2659 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x30a2663); f_030a2443(cpu); if cpu.returned_to != 0x30a2663 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x30a2678; continue; }
  pc = 0x30a266e; continue;
 }
 0x30a266e => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x30a2676); f_030a2443(cpu); if cpu.returned_to != 0x30a2676 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x30a2678; continue;
 }
 0x30a2678 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2762(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2762); }
 let mut pc: u32 = 0x30a2762;
 loop { match pc {
 0x30a2762 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xa8c0u32); }
  if !cpu.cf { pc = 0x30a278f; continue; }
  pc = 0x30a2773; continue;
 }
 0x30a2773 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a278f; continue; }
  pc = 0x30a2777; continue;
 }
 0x30a2777 => {
  { let v: u32 = 0x15180u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x30a279e; continue;
 }
 0x30a278f => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x30a279e; continue;
 }
 0x30a279e => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0xe10u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = 0x3cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.esi; cpu.div32(v); }
  { let v: u32 = 0x16eu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x30a2805; continue; }
  pc = 0x30a27fd; continue;
 }
 0x30a27fd => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x30a2805; continue;
 }
 0x30a2805 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x30a2810); f_030a28a2(cpu); if cpu.returned_to != 0x30a2810 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x16du32); cpu.eax = v; }
  pc = 0x30a2815; continue;
 }
 0x30a2815 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x30a281e; continue; }
  pc = 0x30a2819; continue;
 }
 0x30a2819 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x30a2815; continue;
 }
 0x30a281e => {
  { let v: u32 = 0x314b9dcu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x30a2834); f_030a28a2(cpu); if cpu.returned_to != 0x30a2834 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a283d; continue; }
  pc = 0x30a2838; continue;
 }
 0x30a2838 => {
  { let v: u32 = 0x314b9f6u32; cpu.esi = v; }
  pc = 0x30a283d; continue;
 }
 0x30a283d => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x30a2855; continue; }
  pc = 0x30a2854; continue;
 }
 0x30a2854 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x30a2855; continue;
 }
 0x30a2855 => {
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x7u32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2877(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2877); }
 let mut pc: u32 = 0x30a2877;
 loop { match pc {
 0x30a2877 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  cpu.push(0x30a2892); f_030a2762(cpu); if cpu.returned_to != 0x30a2892 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a28a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a28a2); }
 let mut pc: u32 = 0x30a28a2;
 loop { match pc {
 0x30a28a2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x30a28d7; continue; }
  pc = 0x30a28ac; continue;
 }
 0x30a28ac => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a28c5; continue; }
  pc = 0x30a28bc; continue;
 }
 0x30a28bc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a28c5 => {
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x30a28bc; continue; }
  pc = 0x30a28d7; continue;
 }
 0x30a28d7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a28dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a28dd); }
 let mut pc: u32 = 0x30a28dd;
 loop { match pc {
 0x30a28dd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x30a29aa; continue; }
  pc = 0x30a28f2; continue;
 }
 0x30a28f2 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x30a28fd); f_030a28a2(cpu); if cpu.returned_to != 0x30a28fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a2914; continue; }
  pc = 0x30a2901; continue;
 }
 0x30a2901 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x314b9f6u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x314b9f4u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x30a2925; continue;
 }
 0x30a2914 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x314b9dcu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x314b9dau32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x30a2925; continue;
 }
 0x30a2925 => {
  { let v: u32 = cpu.sar32(cpu.esi, 16u32); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x30a2959); f_030a223e(cpu); if cpu.returned_to != 0x30a2959 { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x7u32); cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x5u32); }
  if !cpu.zf { pc = 0x30a2998; continue; }
  pc = 0x30a2979; continue;
 }
 0x30a2979 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2994; continue; }
  pc = 0x30a298c; continue;
 }
 0x30a298c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  pc = 0x30a2999; continue;
 }
 0x30a2994 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x30a2999; continue;
 }
 0x30a2998 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x30a2999; continue;
 }
 0x30a2999 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x30a29b9; continue;
 }
 0x30a29aa => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x30a29b6; continue; }
  pc = 0x30a29af; continue;
 }
 0x30a29af => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x30a29b9; continue;
 }
 0x30a29b6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30a29b9; continue;
 }
 0x30a29b9 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
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
pub fn f_030a29c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a29c2); }
 let mut pc: u32 = 0x30a29c2;
 loop { match pc {
 0x30a29c2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a29d6; continue; }
  pc = 0x30a29ce; continue;
 }
 0x30a29ce => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a29d6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.ecx))); }
  if (cpu.sf != cpu.of) { pc = 0x30a29f1; continue; }
  pc = 0x30a29db; continue;
 }
 0x30a29db => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x30a29e2); f_030a28dd(cpu); if cpu.returned_to != 0x30a29e2 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a29ed); f_030a28dd(cpu); if cpu.returned_to != 0x30a29ed { return; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30a29ce; continue; }
  pc = 0x30a29f1; continue;
 }
 0x30a29f1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a29f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a29f6); }
 let mut pc: u32 = 0x30a29f6;
 loop { match pc {
 0x30a29f6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x315befeu32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x30a2caa; continue; }
  pc = 0x30a2a12; continue;
 }
 0x30a2a12 => {
  { let v: u32 = 0x315be8cu32; cpu.edx = v; }
  { let v: u32 = 0x315be68u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  cpu.push(0x30a2a24); f_030a29c2(cpu); if cpu.returned_to != 0x30a2a24 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a2a38; continue; }
  pc = 0x30a2a2c; continue;
 }
 0x30a2a2c => {
  { let v: u32 = 0x315be8cu32; cpu.esi = v; }
  { let v: u32 = 0x315be68u32; cpu.ebx = v; }
  pc = 0x30a2a42; continue;
 }
 0x30a2a38 => {
  { let v: u32 = 0x315be68u32; cpu.esi = v; }
  { let v: u32 = 0x315be8cu32; cpu.ebx = v; }
  pc = 0x30a2a42; continue;
 }
 0x30a2a42 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  cpu.push(0x30a2a52); f_030a28a2(cpu); if cpu.returned_to != 0x30a2a52 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a2a6a; continue; }
  pc = 0x30a2a5c; continue;
 }
 0x30a2a5c => {
  { let v: u32 = cpu.r32(0x314b9f6u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x314b9f4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30a2a76; continue;
 }
 0x30a2a6a => {
  { let v: u32 = cpu.r32(0x314b9dcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x314b9dau32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x30a2a76; continue;
 }
 0x30a2a76 => {
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a2b46; continue; }
  pc = 0x30a2a93; continue;
 }
 0x30a2a93 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2aa4; continue; }
  pc = 0x30a2a9a; continue;
 }
 0x30a2a9a => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x30a2b7f; continue;
 }
 0x30a2aa4 => {
  if !cpu.zf { pc = 0x30a2b7f; continue; }
  pc = 0x30a2aaa; continue;
 }
 0x30a2aaa => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x7u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0x5u32); }
  if !cpu.zf { pc = 0x30a2b1f; continue; }
  pc = 0x30a2afa; continue;
 }
 0x30a2afa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x30a2b7f; continue; }
  pc = 0x30a2b0c; continue;
 }
 0x30a2b0c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x30a2b7f; continue; }
  pc = 0x30a2b1d; continue;
 }
 0x30a2b1d => {
  pc = 0x30a2b7b; continue;
 }
 0x30a2b1f => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2b7f; continue; }
  pc = 0x30a2b37; continue;
 }
 0x30a2b37 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2b7f; continue; }
  pc = 0x30a2b44; continue;
 }
 0x30a2b44 => {
  pc = 0x30a2b7b; continue;
 }
 0x30a2b46 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30a2b6d; continue; }
  pc = 0x30a2b4e; continue;
 }
 0x30a2b4e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x30a2b5b); f_030a28a2(cpu); if cpu.returned_to != 0x30a2b5b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a2b6c; continue; }
  pc = 0x30a2b5f; continue;
 }
 0x30a2b5f => {
  { let v: u32 = cpu.r32(0x314b9deu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2b6c; continue; }
  pc = 0x30a2b6b; continue;
 }
 0x30a2b6b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x30a2b6c; continue;
 }
 0x30a2b6c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x30a2b6d; continue;
 }
 0x30a2b6d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2b7f; continue; }
  pc = 0x30a2b74; continue;
 }
 0x30a2b74 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  if !cpu.zf { pc = 0x30a2b7f; continue; }
  pc = 0x30a2b7b; continue;
 }
 0x30a2b7b => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x30a2b7f; continue;
 }
 0x30a2b7f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x30a2b9c; continue; }
  pc = 0x30a2b86; continue;
 }
 0x30a2b86 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a2b8f); f_030a2cb9(cpu); if cpu.returned_to != 0x30a2b8f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  pc = 0x30a2b9c; continue;
 }
 0x30a2b9c => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x30a2bb3; continue; }
  pc = 0x30a2ba0; continue;
 }
 0x30a2ba0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x30a2caa; continue; }
  pc = 0x30a2bac; continue;
 }
 0x30a2bac => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x30a2caa; continue;
 }
 0x30a2bb3 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x30a2c56; continue; }
  pc = 0x30a2bc0; continue;
 }
 0x30a2bc0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2bce; continue; }
  pc = 0x30a2bc7; continue;
 }
 0x30a2bc7 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x30a2c8f; continue;
 }
 0x30a2bce => {
  if !cpu.zf { pc = 0x30a2c8f; continue; }
  pc = 0x30a2bd4; continue;
 }
 0x30a2bd4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x7u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x7u32; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ebp, 0x5u32); }
  if !cpu.zf { pc = 0x30a2c37; continue; }
  pc = 0x30a2c18; continue;
 }
 0x30a2c18 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x30a2c31; continue; }
  pc = 0x30a2c27; continue;
 }
 0x30a2c27 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x30a2c8f; continue;
 }
 0x30a2c31 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2c8f; continue; }
  pc = 0x30a2c35; continue;
 }
 0x30a2c35 => {
  pc = 0x30a2c8a; continue;
 }
 0x30a2c37 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x30a2c27; continue; }
  pc = 0x30a2c50; continue;
 }
 0x30a2c50 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x30a2c8f; continue; }
  pc = 0x30a2c54; continue;
 }
 0x30a2c54 => {
  pc = 0x30a2c8a; continue;
 }
 0x30a2c56 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x30a2c7d; continue; }
  pc = 0x30a2c5e; continue;
 }
 0x30a2c5e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x30a2c6b); f_030a28a2(cpu); if cpu.returned_to != 0x30a2c6b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a2c7c; continue; }
  pc = 0x30a2c6f; continue;
 }
 0x30a2c6f => {
  { let v: u32 = cpu.r32(0x314b9deu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x30a2c7c; continue; }
  pc = 0x30a2c7b; continue;
 }
 0x30a2c7b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x30a2c7c; continue;
 }
 0x30a2c7c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x30a2c7d; continue;
 }
 0x30a2c7d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2c8f; continue; }
  pc = 0x30a2c84; continue;
 }
 0x30a2c84 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x30a2c8f; continue; }
  pc = 0x30a2c8a; continue;
 }
 0x30a2c8a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x30a2c8f; continue;
 }
 0x30a2c8f => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x30a2c9e; continue; }
  pc = 0x30a2c93; continue;
 }
 0x30a2c93 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x30a2c9c); f_030a2cb9(cpu); if cpu.returned_to != 0x30a2c9c { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x30a2c9e; continue;
 }
 0x30a2c9e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x30a2caa; continue; }
  pc = 0x30a2ca6; continue;
 }
 0x30a2ca6 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x30a2caa; continue;
 }
 0x30a2caa => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
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
pub fn f_030a2cb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2cb9); }
 let mut pc: u32 = 0x30a2cb9;
 loop { match pc {
 0x30a2cb9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x30a2cdd; continue; }
  pc = 0x30a2cc9; continue;
 }
 0x30a2cc9 => {
  if !cpu.zf { pc = 0x30a2ce2; continue; }
  pc = 0x30a2ccb; continue;
 }
 0x30a2ccb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x30a2cdd; continue; }
  pc = 0x30a2cd5; continue;
 }
 0x30a2cd5 => {
  if !cpu.zf { pc = 0x30a2ce2; continue; }
  pc = 0x30a2cd7; continue;
 }
 0x30a2cd7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x30a2ce2; continue; }
  pc = 0x30a2cdd; continue;
 }
 0x30a2cdd => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x30a2ce2; continue;
 }
 0x30a2ce2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2cec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2cec); }
 let mut pc: u32 = 0x30a2cec;
 loop { match pc {
 0x30a2cec => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = (cpu.segments[1] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[0] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[2] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[4] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = (cpu.segments[5] as u32); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2d14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2d14); }
 let mut pc: u32 = 0x30a2d14;
 loop { match pc {
 0x30a2d14 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30a2d57; continue; }
  pc = 0x30a2d28; continue;
 }
 0x30a2d28 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x30a2d50; continue; }
  pc = 0x30a2d2c; continue;
 }
 0x30a2d2c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x30a2d40; continue; }
  pc = 0x30a2d36; continue;
 }
 0x30a2d36 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a2d8f; continue;
 }
 0x30a2d40 => {
  if (cpu.cf || cpu.zf) { pc = 0x30a2d49; continue; }
  pc = 0x30a2d42; continue;
 }
 0x30a2d42 => {
  { let v: u32 = 0x8047u32; cpu.eax = v; }
  pc = 0x30a2d81; continue;
 }
 0x30a2d49 => {
  { let v: u32 = 0x4047u32; cpu.eax = v; }
  pc = 0x30a2d81; continue;
 }
 0x30a2d50 => {
  { let v: u32 = 0x2047u32; cpu.eax = v; }
  pc = 0x30a2d81; continue;
 }
 0x30a2d57 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.cf { pc = 0x30a2d6b; continue; }
  pc = 0x30a2d61; continue;
 }
 0x30a2d61 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x30a2d8f; continue;
 }
 0x30a2d6b => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x30a2d7c; continue; }
  pc = 0x30a2d75; continue;
 }
 0x30a2d75 => {
  { let v: u32 = 0x8107u32; cpu.eax = v; }
  pc = 0x30a2d81; continue;
 }
 0x30a2d7c => {
  { let v: u32 = 0x1107u32; cpu.eax = v; }
  pc = 0x30a2d81; continue;
 }
 0x30a2d81 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x30a2d8c); f_030b265f(cpu); if cpu.returned_to != 0x30a2d8c { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x30a2d8f; continue;
 }
 0x30a2d8f => {
  { let v = cpu.rf64(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2da8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2da8); }
 let mut pc: u32 = 0x30a2da8;
 loop { match pc {
 0x30a2da8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x315bf2cu32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x30a2dd7; continue; }
  pc = 0x30a2dbf; continue;
 }
 0x30a2dbf => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x30a2dd1; continue; }
  pc = 0x30a2dc3; continue;
 }
 0x30a2dc3 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x30a2dd1); f_030b273a(cpu); if cpu.returned_to != 0x30a2dd1 { return; }
  pc = 0x30a2dd1; continue;
 }
 0x30a2dd1 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2dd7 => {
  { let a = cpu.st(0); let b = cpu.rf64(0x315bf34u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x30a2dec; continue; }
  pc = 0x30a2de8; continue;
 }
 0x30a2de8 => {
  { cpu.fpush(0.0); }
  pc = 0x30a2e00; continue;
 }
 0x30a2dec => {
  { cpu.fpush(std::f64::consts::LOG2_E); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) - cpu.st(1); cpu.set_st(0, v); }
  { let v = 2f64.powf(cpu.st(0)) - 1.0; cpu.set_st(0, v); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { cpu.fscale(); }
  pc = 0x30a2e00; continue;
 }
 0x30a2e00 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2e14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30a2e14); }
 let mut pc: u32 = 0x30a2e14;
 loop { match pc {
 0x30a2e14 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x30a2e1c; continue;
 }
 0x30a2e1c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x30a2eae; continue; }
  pc = 0x30a2e28; continue;
 }
 0x30a2e28 => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x30a2e37; continue; }
  pc = 0x30a2e34; continue;
 }
 0x30a2e34 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2e37 => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x315d320u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x30a2e46; continue; }
  pc = 0x30a2e43; continue;
 }
 0x30a2e43 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2e46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a2eb8; continue; }
  pc = 0x30a2e51; continue;
 }
 0x30a2e51 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x30a2eb8; continue; }
  pc = 0x30a2e58; continue;
 }
 0x30a2e58 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x30a2e97; continue; }
  pc = 0x30a2e80; continue;
 }
 0x30a2e80 => {
  { let v = cpu.st(0) * cpu.rf32(0x315d330u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x315d330u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2e97 => {
  { let v = cpu.st(0) * cpu.rf32(0x315d334u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x315d334u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2eae => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x30a2ebb; continue; }
  pc = 0x30a2eb8; continue;
 }
 0x30a2eb8 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2ebb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30a2eb8; continue; }
  pc = 0x30a2ec6; continue;
 }
 0x30a2ec6 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x30a2efc; continue; }
  pc = 0x30a2eeb; continue;
 }
 0x30a2eeb => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x30a2f24; continue; }
  pc = 0x30a2ef2; continue;
 }
 0x30a2ef2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x30a2f24; continue; }
  pc = 0x30a2efa; continue;
 }
 0x30a2efa => {
  pc = 0x30a2f04; continue;
 }
 0x30a2efc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x30a2f24; continue; }
  pc = 0x30a2f04; continue;
 }
 0x30a2f04 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x315d338u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  cpu.poll(); pc = 0x30a2e1c; continue;
 }
 0x30a2f24 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030a2f2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f2b); } r_030a2f2b(cpu, 0x30a2f2b); }
pub fn f_030a2f35(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f35); } r_030a2f2b(cpu, 0x30a2f35); }
pub fn f_030a2f3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f3b); } r_030a2f2b(cpu, 0x30a2f3b); }
pub fn f_030a2f40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f40); } r_030a2f2b(cpu, 0x30a2f40); }
pub fn f_030a2f46(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f46); } r_030a2f2b(cpu, 0x30a2f46); }
pub fn f_030a2f4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f4b); } r_030a2f2b(cpu, 0x30a2f4b); }
pub fn f_030a2f51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f51); } r_030a2f2b(cpu, 0x30a2f51); }
pub fn f_030a2f57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f57); } r_030a2f2b(cpu, 0x30a2f57); }
pub fn f_030a2f5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f5d); } r_030a2f2b(cpu, 0x30a2f5d); }
pub fn f_030a2f63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f63); } r_030a2f2b(cpu, 0x30a2f63); }
pub fn f_030a2f7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f7f); } r_030a2f2b(cpu, 0x30a2f7f); }
pub fn f_030a2f84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f84); } r_030a2f2b(cpu, 0x30a2f84); }
pub fn f_030a2f9a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f9a); } r_030a2f2b(cpu, 0x30a2f9a); }
pub fn f_030a2f9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2f9f); } r_030a2f2b(cpu, 0x30a2f9f); }
pub fn f_030a2fbb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2fbb); } r_030a2f2b(cpu, 0x30a2fbb); }
pub fn f_030a2fcb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2fcb); } r_030a2f2b(cpu, 0x30a2fcb); }
pub fn f_030a2fdf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2fdf); } r_030a2f2b(cpu, 0x30a2fdf); }
pub fn f_030a2fef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a2fef); } r_030a2f2b(cpu, 0x30a2fef); }
pub fn f_030a300f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a300f); } r_030a2f2b(cpu, 0x30a300f); }
pub fn f_030a3014(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3014); } r_030a2f2b(cpu, 0x30a3014); }
pub fn f_030a302e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a302e); } r_030a2f2b(cpu, 0x30a302e); }
pub fn f_030a3033(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3033); } r_030a2f2b(cpu, 0x30a3033); }
pub fn f_030a3053(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3053); } r_030a2f2b(cpu, 0x30a3053); }
pub fn f_030a3067(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3067); } r_030a2f2b(cpu, 0x30a3067); }
pub fn f_030a307f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a307f); } r_030a2f2b(cpu, 0x30a307f); }
pub fn f_030a3093(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3093); } r_030a2f2b(cpu, 0x30a3093); }
pub fn f_030a30b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a30b3); } r_030a2f2b(cpu, 0x30a30b3); }
pub fn f_030a30b8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a30b8); } r_030a2f2b(cpu, 0x30a30b8); }
pub fn f_030a30d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a30d2); } r_030a2f2b(cpu, 0x30a30d2); }
pub fn f_030a30d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a30d7); } r_030a2f2b(cpu, 0x30a30d7); }
pub fn f_030a30f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a30f7); } r_030a2f2b(cpu, 0x30a30f7); }
pub fn f_030a310b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a310b); } r_030a2f2b(cpu, 0x30a310b); }
pub fn f_030a3123(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3123); } r_030a2f2b(cpu, 0x30a3123); }
pub fn f_030a3137(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3137); } r_030a2f2b(cpu, 0x30a3137); }
pub fn f_030a3157(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3157); } r_030a2f2b(cpu, 0x30a3157); }
pub fn f_030a315c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a315c); } r_030a2f2b(cpu, 0x30a315c); }
pub fn f_030a3176(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3176); } r_030a2f2b(cpu, 0x30a3176); }
pub fn f_030a317b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a317b); } r_030a2f2b(cpu, 0x30a317b); }
pub fn f_030a319b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a319b); } r_030a2f2b(cpu, 0x30a319b); }
pub fn f_030a31af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a31af); } r_030a2f2b(cpu, 0x30a31af); }
pub fn f_030a31c7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a31c7); } r_030a2f2b(cpu, 0x30a31c7); }
pub fn f_030a31db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a31db); } r_030a2f2b(cpu, 0x30a31db); }
pub fn f_030a31fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a31fb); } r_030a2f2b(cpu, 0x30a31fb); }
pub fn f_030a3200(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3200); } r_030a2f2b(cpu, 0x30a3200); }
pub fn f_030a321a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a321a); } r_030a2f2b(cpu, 0x30a321a); }
pub fn f_030a321f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a321f); } r_030a2f2b(cpu, 0x30a321f); }
pub fn f_030a323f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a323f); } r_030a2f2b(cpu, 0x30a323f); }
pub fn f_030a3253(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3253); } r_030a2f2b(cpu, 0x30a3253); }
pub fn f_030a326b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a326b); } r_030a2f2b(cpu, 0x30a326b); }
pub fn f_030a327f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a327f); } r_030a2f2b(cpu, 0x30a327f); }
pub fn f_030a329f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a329f); } r_030a2f2b(cpu, 0x30a329f); }
pub fn f_030a32a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a32a4); } r_030a2f2b(cpu, 0x30a32a4); }
pub fn f_030a32be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a32be); } r_030a2f2b(cpu, 0x30a32be); }
pub fn f_030a32c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a32c3); } r_030a2f2b(cpu, 0x30a32c3); }
pub fn f_030a32e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a32e3); } r_030a2f2b(cpu, 0x30a32e3); }
pub fn f_030a32f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a32f7); } r_030a2f2b(cpu, 0x30a32f7); }
pub fn f_030a330f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a330f); } r_030a2f2b(cpu, 0x30a330f); }
pub fn f_030a3323(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3323); } r_030a2f2b(cpu, 0x30a3323); }
pub fn f_030a3343(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3343); } r_030a2f2b(cpu, 0x30a3343); }
pub fn f_030a3348(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3348); } r_030a2f2b(cpu, 0x30a3348); }
pub fn f_030a3362(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3362); } r_030a2f2b(cpu, 0x30a3362); }
pub fn f_030a3367(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3367); } r_030a2f2b(cpu, 0x30a3367); }
pub fn f_030a3387(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a3387); } r_030a2f2b(cpu, 0x30a3387); }
pub fn f_030a339b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a339b); } r_030a2f2b(cpu, 0x30a339b); }
pub fn f_030a33b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x30a33b3); } r_030a2f2b(cpu, 0x30a33b3); }
fn r_030a2f2b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x30a2f2b => {
  { let v: u32 = cpu.sub32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.poll(); pc = cpu.r32(0x315d33cu32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x30a2f35 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f3b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a2f40; continue;
 }
 0x30a2f40 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f46 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a2f4b; continue;
 }
 0x30a2f4b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f51 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f57 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f5d => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f63 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a2f75); f_030a2e14(cpu); if cpu.returned_to != 0x30a2f75 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f7f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a2f84; continue;
 }
 0x30a2f84 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a2f90); f_030a2e14(cpu); if cpu.returned_to != 0x30a2f90 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2f9a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a2f9f; continue;
 }
 0x30a2f9f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a2fb3); f_030a2e14(cpu); if cpu.returned_to != 0x30a2fb3 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2fbb => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a2fc7); f_030a2e14(cpu); if cpu.returned_to != 0x30a2fc7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2fcb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a2fd7); f_030a2e14(cpu); if cpu.returned_to != 0x30a2fd7 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2fdf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a2feb); f_030a2e14(cpu); if cpu.returned_to != 0x30a2feb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a2fef => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3003); f_030a2e14(cpu); if cpu.returned_to != 0x30a3003 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a300f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a3014; continue;
 }
 0x30a3014 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3022); f_030a2e14(cpu); if cpu.returned_to != 0x30a3022 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a302e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a3033; continue;
 }
 0x30a3033 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3049); f_030a2e14(cpu); if cpu.returned_to != 0x30a3049 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3053 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3061); f_030a2e14(cpu); if cpu.returned_to != 0x30a3061 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3067 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3075); f_030a2e14(cpu); if cpu.returned_to != 0x30a3075 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a307f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a308d); f_030a2e14(cpu); if cpu.returned_to != 0x30a308d { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3093 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a30a7); f_030a2e14(cpu); if cpu.returned_to != 0x30a30a7 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a30b3 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a30b8; continue;
 }
 0x30a30b8 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a30c6); f_030a2e14(cpu); if cpu.returned_to != 0x30a30c6 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a30d2 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a30d7; continue;
 }
 0x30a30d7 => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a30ed); f_030a2e14(cpu); if cpu.returned_to != 0x30a30ed { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a30f7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3105); f_030a2e14(cpu); if cpu.returned_to != 0x30a3105 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a310b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3119); f_030a2e14(cpu); if cpu.returned_to != 0x30a3119 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3123 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3131); f_030a2e14(cpu); if cpu.returned_to != 0x30a3131 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3137 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a314b); f_030a2e14(cpu); if cpu.returned_to != 0x30a314b { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3157 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a315c; continue;
 }
 0x30a315c => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a316a); f_030a2e14(cpu); if cpu.returned_to != 0x30a316a { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3176 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a317b; continue;
 }
 0x30a317b => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3191); f_030a2e14(cpu); if cpu.returned_to != 0x30a3191 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a319b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a31a9); f_030a2e14(cpu); if cpu.returned_to != 0x30a31a9 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a31af => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a31bd); f_030a2e14(cpu); if cpu.returned_to != 0x30a31bd { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a31c7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a31d5); f_030a2e14(cpu); if cpu.returned_to != 0x30a31d5 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a31db => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a31ef); f_030a2e14(cpu); if cpu.returned_to != 0x30a31ef { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a31fb => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a3200; continue;
 }
 0x30a3200 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a320e); f_030a2e14(cpu); if cpu.returned_to != 0x30a320e { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a321a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a321f; continue;
 }
 0x30a321f => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3235); f_030a2e14(cpu); if cpu.returned_to != 0x30a3235 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a323f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a324d); f_030a2e14(cpu); if cpu.returned_to != 0x30a324d { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3253 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3261); f_030a2e14(cpu); if cpu.returned_to != 0x30a3261 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a326b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3279); f_030a2e14(cpu); if cpu.returned_to != 0x30a3279 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a327f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3293); f_030a2e14(cpu); if cpu.returned_to != 0x30a3293 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a329f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a32a4; continue;
 }
 0x30a32a4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a32b2); f_030a2e14(cpu); if cpu.returned_to != 0x30a32b2 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a32be => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a32c3; continue;
 }
 0x30a32c3 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a32d9); f_030a2e14(cpu); if cpu.returned_to != 0x30a32d9 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a32e3 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a32f1); f_030a2e14(cpu); if cpu.returned_to != 0x30a32f1 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a32f7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3305); f_030a2e14(cpu); if cpu.returned_to != 0x30a3305 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a330f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a331d); f_030a2e14(cpu); if cpu.returned_to != 0x30a331d { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3323 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3337); f_030a2e14(cpu); if cpu.returned_to != 0x30a3337 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3343 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a3348; continue;
 }
 0x30a3348 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3356); f_030a2e14(cpu); if cpu.returned_to != 0x30a3356 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3362 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x30a3367; continue;
 }
 0x30a3367 => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a337d); f_030a2e14(cpu); if cpu.returned_to != 0x30a337d { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a3387 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a3395); f_030a2e14(cpu); if cpu.returned_to != 0x30a3395 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a339b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a33a9); f_030a2e14(cpu); if cpu.returned_to != 0x30a33a9 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30a33b3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x30a33c1); f_030a2e14(cpu); if cpu.returned_to != 0x30a33c1 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
