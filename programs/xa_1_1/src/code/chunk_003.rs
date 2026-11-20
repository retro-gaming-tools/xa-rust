#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_04095424(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095424); }
 let mut pc: u32 = 0x4095424;
 loop { match pc {
 0x4095424 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4255aa0u32; cpu.edx = v; }
  cpu.push(0x409542f); f_040953d3(cpu); if cpu.returned_to != 0x409542f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04095434(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095434); }
 let mut pc: u32 = 0x4095434;
 loop { match pc {
 0x4095434 => {
  { cpu.sub32(cpu.eax, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x4095441; continue; }
  pc = 0x4095439; continue;
 }
 0x4095439 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x4095441; continue; }
  pc = 0x409543e; continue;
 }
 0x409543e => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x4095441; continue;
 }
 0x4095441 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04095444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095444); }
 let mut pc: u32 = 0x4095444;
 loop { match pc {
 0x4095444 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  cpu.push(0x4095454); f_040a250c(cpu); if cpu.returned_to != 0x4095454 { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409545b); f_0409ea21(cpu); if cpu.returned_to != 0x409545b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04095464(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095464); }
 let mut pc: u32 = 0x4095464;
 loop { match pc {
 0x4095464 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x4095493; continue; }
  pc = 0x4095479; continue;
 }
 0x4095479 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x409547b; continue;
 }
 0x409547b => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x409548b); f_040a2534(cpu); if cpu.returned_to != 0x409548b { return; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x409558c; continue;
 }
 0x4095493 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x40954de; continue; }
  pc = 0x40954a3; continue;
 }
 0x40954a3 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { cpu.poll(); pc = 0x409547b; continue; }
  pc = 0x40954a8; continue;
 }
 0x40954a8 => {
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
  if cpu.pf() { pc = 0x40954d5; continue; }
  pc = 0x40954c4; continue;
 }
 0x40954c4 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x40954d5; continue; }
  pc = 0x40954d3; continue;
 }
 0x40954d3 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40954d5; continue;
 }
 0x40954d5 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xfffffff9u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x40954e6; continue;
 }
 0x40954de => {
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x40954e6; continue; }
  pc = 0x40954e1; continue;
 }
 0x40954e1 => {
  pc = 0x409556d; continue;
 }
 0x40954e6 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3ff0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if !cpu.cf { pc = 0x409556d; continue; }
  pc = 0x40954fe; continue;
 }
 0x40954fe => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x4095518; continue; }
  pc = 0x409550d; continue;
 }
 0x409550d => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x4095516); f_04095597(cpu); if cpu.returned_to != 0x4095516 { return; }
  pc = 0x4095543; continue;
 }
 0x4095518 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x409556d; continue; }
  pc = 0x409551c; continue;
 }
 0x409551c => {
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x409556d; continue; }
  pc = 0x4095522; continue;
 }
 0x4095522 => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x409552a); f_04095597(cpu); if cpu.returned_to != 0x409552a { return; }
  { cpu.fpush(1.0); }
  { cpu.and8(cpu.r8(0x415b8f0u32), 0x1u8); }
  if !cpu.zf { pc = 0x4095539; continue; }
  pc = 0x4095535; continue;
 }
 0x4095535 => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x4095543; continue;
 }
 0x4095539 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x4095543); f_040a274b(cpu); if cpu.returned_to != 0x4095543 { return; }
  pc = 0x4095543; continue;
 }
 0x4095543 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x4095569; continue; }
  pc = 0x4095555; continue;
 }
 0x4095555 => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if !cpu.zf { pc = 0x4095569; continue; }
  pc = 0x4095562; continue;
 }
 0x4095562 => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x409547b; continue;
 }
 0x4095569 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x409558c; continue;
 }
 0x409556d => {
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409557e); f_040a25c8(cpu); if cpu.returned_to != 0x409557e { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x4095562; continue; }
  pc = 0x4095582; continue;
 }
 0x4095582 => {
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x409558a; continue; }
  pc = 0x4095588; continue;
 }
 0x4095588 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x409558a; continue;
 }
 0x409558a => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x409558c; continue;
 }
 0x409558c => {
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
pub fn f_04095597(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095597); }
 let mut pc: u32 = 0x4095597;
 loop { match pc {
 0x4095597 => {
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x40955a0; continue; }
  pc = 0x409559c; continue;
 }
 0x409559c => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  cpu.poll(); pc = 0x4095597; continue;
 }
 0x40955a0 => {
  if !cpu.cf { pc = 0x40955b5; continue; }
  pc = 0x40955a2; continue;
 }
 0x40955a2 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  pc = 0x40955a4; continue;
 }
 0x40955a4 => {
  if cpu.zf { pc = 0x40955b1; continue; }
  pc = 0x40955a6; continue;
 }
 0x40955a6 => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x40955af; continue; }
  pc = 0x40955ad; continue;
 }
 0x40955ad => {
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  pc = 0x40955af; continue;
 }
 0x40955af => {
  cpu.poll(); pc = 0x40955a4; continue;
 }
 0x40955b1 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x40955b9; continue;
 }
 0x40955b5 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  pc = 0x40955b9; continue;
 }
 0x40955b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040955d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40955d0); }
 let mut pc: u32 = 0x40955d0;
 loop { match pc {
 0x40955d0 => {
  { cpu.sub32(cpu.r32(0x415a3d0u32), 0x0u32); }
  if cpu.zf { pc = 0x40955f3; continue; }
  pc = 0x40955d9; continue;
 }
 0x40955d9 => {
  { let v = 0x40955d0u32; cpu.push(v); }
  { let v = 0x409d6c0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40955ea); f_040a3020(cpu); if cpu.returned_to != 0x40955ea { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415a3d0u32, v); }
  pc = 0x40955f3; continue;
 }
 0x40955f3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04095600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095600); }
 let mut pc: u32 = 0x4095600;
 loop { match pc {
 0x4095600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095647; continue; }
  pc = 0x4095620; continue;
 }
 0x4095620 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409562e; continue; }
  pc = 0x4095625; continue;
 }
 0x4095625 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095647; continue; }
  pc = 0x409562e; continue;
 }
 0x409562e => {
  cpu.push(0x4095633); f_040a3b61(cpu); if cpu.returned_to != 0x4095633 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095647; continue; }
  pc = 0x4095637; continue;
 }
 0x4095637 => {
  cpu.push(0x409563c); f_0409d2c0(cpu); if cpu.returned_to != 0x409563c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095647; continue; }
  pc = 0x4095640; continue;
 }
 0x4095640 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095649; continue;
 }
 0x4095647 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095649; continue;
 }
 0x4095649 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095697; continue; }
  pc = 0x409564d; continue;
 }
 0x409564d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414adb8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409568f); cpu.call(t); if cpu.returned_to != 0x409568f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095697); f_0409d690(cpu); if cpu.returned_to != 0x4095697 { return; }
  pc = 0x4095697; continue;
 }
 0x4095697 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40956b4); f_040ac570(cpu); if cpu.returned_to != 0x40956b4 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x4095702; continue; }
  pc = 0x40956c3; continue;
 }
 0x40956c3 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40956d5; continue; }
  pc = 0x40956cc; continue;
 }
 0x40956cc => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095702; continue; }
  pc = 0x40956d5; continue;
 }
 0x40956d5 => {
  cpu.push(0x40956da); f_040a3b61(cpu); if cpu.returned_to != 0x40956da { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095702; continue; }
  pc = 0x40956de; continue;
 }
 0x40956de => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40956f8); f_040a3df3(cpu); if cpu.returned_to != 0x40956f8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4095702); f_0409d5f0(cpu); if cpu.returned_to != 0x4095702 { return; }
  pc = 0x4095702; continue;
 }
 0x4095702 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04095820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095820); }
 let mut pc: u32 = 0x4095820;
 loop { match pc {
 0x4095820 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095867; continue; }
  pc = 0x4095840; continue;
 }
 0x4095840 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409584e; continue; }
  pc = 0x4095845; continue;
 }
 0x4095845 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095867; continue; }
  pc = 0x409584e; continue;
 }
 0x409584e => {
  cpu.push(0x4095853); f_040a3b61(cpu); if cpu.returned_to != 0x4095853 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095867; continue; }
  pc = 0x4095857; continue;
 }
 0x4095857 => {
  cpu.push(0x409585c); f_0409d2c0(cpu); if cpu.returned_to != 0x409585c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095867; continue; }
  pc = 0x4095860; continue;
 }
 0x4095860 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095869; continue;
 }
 0x4095867 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095869; continue;
 }
 0x4095869 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409589b; continue; }
  pc = 0x409586d; continue;
 }
 0x409586d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ad84u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095893); cpu.call(t); if cpu.returned_to != 0x4095893 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409589b); f_0409d690(cpu); if cpu.returned_to != 0x409589b { return; }
  pc = 0x409589b; continue;
 }
 0x409589b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40958a8); f_040ac600(cpu); if cpu.returned_to != 0x40958a8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040958d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40958d0); }
 let mut pc: u32 = 0x40958d0;
 loop { match pc {
 0x40958d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095918; continue; }
  pc = 0x40958f1; continue;
 }
 0x40958f1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40958ff; continue; }
  pc = 0x40958f6; continue;
 }
 0x40958f6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095918; continue; }
  pc = 0x40958ff; continue;
 }
 0x40958ff => {
  cpu.push(0x4095904); f_040a3b61(cpu); if cpu.returned_to != 0x4095904 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095918; continue; }
  pc = 0x4095908; continue;
 }
 0x4095908 => {
  cpu.push(0x409590d); f_0409d2c0(cpu); if cpu.returned_to != 0x409590d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095918; continue; }
  pc = 0x4095911; continue;
 }
 0x4095911 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409591a; continue;
 }
 0x4095918 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409591a; continue;
 }
 0x409591a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409595a; continue; }
  pc = 0x409591e; continue;
 }
 0x409591e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ad68u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095952); cpu.call(t); if cpu.returned_to != 0x4095952 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409595a); f_0409d690(cpu); if cpu.returned_to != 0x409595a { return; }
  pc = 0x409595a; continue;
 }
 0x409595a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409596f); f_040ac650(cpu); if cpu.returned_to != 0x409596f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40959bc; continue; }
  pc = 0x409597d; continue;
 }
 0x409597d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409598f; continue; }
  pc = 0x4095986; continue;
 }
 0x4095986 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40959bc; continue; }
  pc = 0x409598f; continue;
 }
 0x409598f => {
  cpu.push(0x4095994); f_040a3b61(cpu); if cpu.returned_to != 0x4095994 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40959bc; continue; }
  pc = 0x4095998; continue;
 }
 0x4095998 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40959b2); f_040a3df3(cpu); if cpu.returned_to != 0x40959b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40959bc); f_0409d640(cpu); if cpu.returned_to != 0x40959bc { return; }
  pc = 0x40959bc; continue;
 }
 0x40959bc => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_040959e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40959e0); }
 let mut pc: u32 = 0x40959e0;
 loop { match pc {
 0x40959e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095a27; continue; }
  pc = 0x4095a00; continue;
 }
 0x4095a00 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4095a0e; continue; }
  pc = 0x4095a05; continue;
 }
 0x4095a05 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095a27; continue; }
  pc = 0x4095a0e; continue;
 }
 0x4095a0e => {
  cpu.push(0x4095a13); f_040a3b61(cpu); if cpu.returned_to != 0x4095a13 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095a27; continue; }
  pc = 0x4095a17; continue;
 }
 0x4095a17 => {
  cpu.push(0x4095a1c); f_0409d2c0(cpu); if cpu.returned_to != 0x4095a1c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095a27; continue; }
  pc = 0x4095a20; continue;
 }
 0x4095a20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095a29; continue;
 }
 0x4095a27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095a29; continue;
 }
 0x4095a29 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095a5b; continue; }
  pc = 0x4095a2d; continue;
 }
 0x4095a2d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ad4cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095a53); cpu.call(t); if cpu.returned_to != 0x4095a53 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095a5b); f_0409d690(cpu); if cpu.returned_to != 0x4095a5b { return; }
  pc = 0x4095a5b; continue;
 }
 0x4095a5b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4095a68); f_040ac7c0(cpu); if cpu.returned_to != 0x4095a68 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04095a90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095a90); }
 let mut pc: u32 = 0x4095a90;
 loop { match pc {
 0x4095a90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095ad8; continue; }
  pc = 0x4095ab1; continue;
 }
 0x4095ab1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4095abf; continue; }
  pc = 0x4095ab6; continue;
 }
 0x4095ab6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095ad8; continue; }
  pc = 0x4095abf; continue;
 }
 0x4095abf => {
  cpu.push(0x4095ac4); f_040a3b61(cpu); if cpu.returned_to != 0x4095ac4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095ad8; continue; }
  pc = 0x4095ac8; continue;
 }
 0x4095ac8 => {
  cpu.push(0x4095acd); f_0409d2c0(cpu); if cpu.returned_to != 0x4095acd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095ad8; continue; }
  pc = 0x4095ad1; continue;
 }
 0x4095ad1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095ada; continue;
 }
 0x4095ad8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095ada; continue;
 }
 0x4095ada => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095b0c; continue; }
  pc = 0x4095ade; continue;
 }
 0x4095ade => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ad34u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095b04); cpu.call(t); if cpu.returned_to != 0x4095b04 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095b0c); f_0409d690(cpu); if cpu.returned_to != 0x4095b0c { return; }
  pc = 0x4095b0c; continue;
 }
 0x4095b0c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4095b19); f_040ac860(cpu); if cpu.returned_to != 0x4095b19 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4095b67; continue; }
  pc = 0x4095b28; continue;
 }
 0x4095b28 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4095b3a; continue; }
  pc = 0x4095b31; continue;
 }
 0x4095b31 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095b67; continue; }
  pc = 0x4095b3a; continue;
 }
 0x4095b3a => {
  cpu.push(0x4095b3f); f_040a3b61(cpu); if cpu.returned_to != 0x4095b3f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095b67; continue; }
  pc = 0x4095b43; continue;
 }
 0x4095b43 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4095b5d); f_040a3df3(cpu); if cpu.returned_to != 0x4095b5d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4095b67); f_0409d5f0(cpu); if cpu.returned_to != 0x4095b67 { return; }
  pc = 0x4095b67; continue;
 }
 0x4095b67 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04095d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095d10); }
 let mut pc: u32 = 0x4095d10;
 loop { match pc {
 0x4095d10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095d58; continue; }
  pc = 0x4095d31; continue;
 }
 0x4095d31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4095d3f; continue; }
  pc = 0x4095d36; continue;
 }
 0x4095d36 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095d58; continue; }
  pc = 0x4095d3f; continue;
 }
 0x4095d3f => {
  cpu.push(0x4095d44); f_040a3b61(cpu); if cpu.returned_to != 0x4095d44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095d58; continue; }
  pc = 0x4095d48; continue;
 }
 0x4095d48 => {
  cpu.push(0x4095d4d); f_0409d2c0(cpu); if cpu.returned_to != 0x4095d4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095d58; continue; }
  pc = 0x4095d51; continue;
 }
 0x4095d51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095d5a; continue;
 }
 0x4095d58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095d5a; continue;
 }
 0x4095d5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095d8c; continue; }
  pc = 0x4095d5e; continue;
 }
 0x4095d5e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ace4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095d84); cpu.call(t); if cpu.returned_to != 0x4095d84 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095d8c); f_0409d690(cpu); if cpu.returned_to != 0x4095d8c { return; }
  pc = 0x4095d8c; continue;
 }
 0x4095d8c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4095d99); f_040ab850(cpu); if cpu.returned_to != 0x4095d99 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4095de7; continue; }
  pc = 0x4095da8; continue;
 }
 0x4095da8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4095dba; continue; }
  pc = 0x4095db1; continue;
 }
 0x4095db1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095de7; continue; }
  pc = 0x4095dba; continue;
 }
 0x4095dba => {
  cpu.push(0x4095dbf); f_040a3b61(cpu); if cpu.returned_to != 0x4095dbf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095de7; continue; }
  pc = 0x4095dc3; continue;
 }
 0x4095dc3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4095ddd); f_040a3df3(cpu); if cpu.returned_to != 0x4095ddd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4095de7); f_0409d640(cpu); if cpu.returned_to != 0x4095de7 { return; }
  pc = 0x4095de7; continue;
 }
 0x4095de7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04095e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095e10); }
 let mut pc: u32 = 0x4095e10;
 loop { match pc {
 0x4095e10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095e58; continue; }
  pc = 0x4095e31; continue;
 }
 0x4095e31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4095e3f; continue; }
  pc = 0x4095e36; continue;
 }
 0x4095e36 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095e58; continue; }
  pc = 0x4095e3f; continue;
 }
 0x4095e3f => {
  cpu.push(0x4095e44); f_040a3b61(cpu); if cpu.returned_to != 0x4095e44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095e58; continue; }
  pc = 0x4095e48; continue;
 }
 0x4095e48 => {
  cpu.push(0x4095e4d); f_0409d2c0(cpu); if cpu.returned_to != 0x4095e4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095e58; continue; }
  pc = 0x4095e51; continue;
 }
 0x4095e51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095e5a; continue;
 }
 0x4095e58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095e5a; continue;
 }
 0x4095e5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095e8c; continue; }
  pc = 0x4095e5e; continue;
 }
 0x4095e5e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414acc8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095e84); cpu.call(t); if cpu.returned_to != 0x4095e84 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095e8c); f_0409d690(cpu); if cpu.returned_to != 0x4095e8c { return; }
  pc = 0x4095e8c; continue;
 }
 0x4095e8c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4095e99); f_040ab8a0(cpu); if cpu.returned_to != 0x4095e99 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4095ee7; continue; }
  pc = 0x4095ea8; continue;
 }
 0x4095ea8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4095eba; continue; }
  pc = 0x4095eb1; continue;
 }
 0x4095eb1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095ee7; continue; }
  pc = 0x4095eba; continue;
 }
 0x4095eba => {
  cpu.push(0x4095ebf); f_040a3b61(cpu); if cpu.returned_to != 0x4095ebf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095ee7; continue; }
  pc = 0x4095ec3; continue;
 }
 0x4095ec3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4095edd); f_040a3df3(cpu); if cpu.returned_to != 0x4095edd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4095ee7); f_0409d640(cpu); if cpu.returned_to != 0x4095ee7 { return; }
  pc = 0x4095ee7; continue;
 }
 0x4095ee7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04095f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4095f10); }
 let mut pc: u32 = 0x4095f10;
 loop { match pc {
 0x4095f10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4095f57; continue; }
  pc = 0x4095f30; continue;
 }
 0x4095f30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4095f3e; continue; }
  pc = 0x4095f35; continue;
 }
 0x4095f35 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4095f57; continue; }
  pc = 0x4095f3e; continue;
 }
 0x4095f3e => {
  cpu.push(0x4095f43); f_040a3b61(cpu); if cpu.returned_to != 0x4095f43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4095f57; continue; }
  pc = 0x4095f47; continue;
 }
 0x4095f47 => {
  cpu.push(0x4095f4c); f_0409d2c0(cpu); if cpu.returned_to != 0x4095f4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095f57; continue; }
  pc = 0x4095f50; continue;
 }
 0x4095f50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4095f59; continue;
 }
 0x4095f57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4095f59; continue;
 }
 0x4095f59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4095fa7; continue; }
  pc = 0x4095f5d; continue;
 }
 0x4095f5d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414aca4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4095f9f); cpu.call(t); if cpu.returned_to != 0x4095f9f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095fa7); f_0409d690(cpu); if cpu.returned_to != 0x4095fa7 { return; }
  pc = 0x4095fa7; continue;
 }
 0x4095fa7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4095fc4); f_040ab900(cpu); if cpu.returned_to != 0x4095fc4 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x4096012; continue; }
  pc = 0x4095fd3; continue;
 }
 0x4095fd3 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4095fe5; continue; }
  pc = 0x4095fdc; continue;
 }
 0x4095fdc => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096012; continue; }
  pc = 0x4095fe5; continue;
 }
 0x4095fe5 => {
  cpu.push(0x4095fea); f_040a3b61(cpu); if cpu.returned_to != 0x4095fea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096012; continue; }
  pc = 0x4095fee; continue;
 }
 0x4095fee => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4096008); f_040a3df3(cpu); if cpu.returned_to != 0x4096008 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4096012); f_0409d640(cpu); if cpu.returned_to != 0x4096012 { return; }
  pc = 0x4096012; continue;
 }
 0x4096012 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096030); }
 let mut pc: u32 = 0x4096030;
 loop { match pc {
 0x4096030 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096078; continue; }
  pc = 0x4096051; continue;
 }
 0x4096051 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409605f; continue; }
  pc = 0x4096056; continue;
 }
 0x4096056 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096078; continue; }
  pc = 0x409605f; continue;
 }
 0x409605f => {
  cpu.push(0x4096064); f_040a3b61(cpu); if cpu.returned_to != 0x4096064 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096078; continue; }
  pc = 0x4096068; continue;
 }
 0x4096068 => {
  cpu.push(0x409606d); f_0409d2c0(cpu); if cpu.returned_to != 0x409606d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096078; continue; }
  pc = 0x4096071; continue;
 }
 0x4096071 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409607a; continue;
 }
 0x4096078 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409607a; continue;
 }
 0x409607a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40960ac; continue; }
  pc = 0x409607e; continue;
 }
 0x409607e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ac88u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40960a4); cpu.call(t); if cpu.returned_to != 0x40960a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40960ac); f_0409d690(cpu); if cpu.returned_to != 0x40960ac { return; }
  pc = 0x40960ac; continue;
 }
 0x40960ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40960b9); f_040ab990(cpu); if cpu.returned_to != 0x40960b9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4096107; continue; }
  pc = 0x40960c8; continue;
 }
 0x40960c8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40960da; continue; }
  pc = 0x40960d1; continue;
 }
 0x40960d1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096107; continue; }
  pc = 0x40960da; continue;
 }
 0x40960da => {
  cpu.push(0x40960df); f_040a3b61(cpu); if cpu.returned_to != 0x40960df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096107; continue; }
  pc = 0x40960e3; continue;
 }
 0x40960e3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40960fd); f_040a3df3(cpu); if cpu.returned_to != 0x40960fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4096107); f_0409d640(cpu); if cpu.returned_to != 0x4096107 { return; }
  pc = 0x4096107; continue;
 }
 0x4096107 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04096130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096130); }
 let mut pc: u32 = 0x4096130;
 loop { match pc {
 0x4096130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096178; continue; }
  pc = 0x4096151; continue;
 }
 0x4096151 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409615f; continue; }
  pc = 0x4096156; continue;
 }
 0x4096156 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096178; continue; }
  pc = 0x409615f; continue;
 }
 0x409615f => {
  cpu.push(0x4096164); f_040a3b61(cpu); if cpu.returned_to != 0x4096164 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096178; continue; }
  pc = 0x4096168; continue;
 }
 0x4096168 => {
  cpu.push(0x409616d); f_0409d2c0(cpu); if cpu.returned_to != 0x409616d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096178; continue; }
  pc = 0x4096171; continue;
 }
 0x4096171 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409617a; continue;
 }
 0x4096178 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409617a; continue;
 }
 0x409617a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40961ac; continue; }
  pc = 0x409617e; continue;
 }
 0x409617e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ac68u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40961a4); cpu.call(t); if cpu.returned_to != 0x40961a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40961ac); f_0409d690(cpu); if cpu.returned_to != 0x40961ac { return; }
  pc = 0x40961ac; continue;
 }
 0x40961ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40961b9); f_040ab9c0(cpu); if cpu.returned_to != 0x40961b9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4096207; continue; }
  pc = 0x40961c8; continue;
 }
 0x40961c8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40961da; continue; }
  pc = 0x40961d1; continue;
 }
 0x40961d1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096207; continue; }
  pc = 0x40961da; continue;
 }
 0x40961da => {
  cpu.push(0x40961df); f_040a3b61(cpu); if cpu.returned_to != 0x40961df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096207; continue; }
  pc = 0x40961e3; continue;
 }
 0x40961e3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40961fd); f_040a3df3(cpu); if cpu.returned_to != 0x40961fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4096207); f_0409d640(cpu); if cpu.returned_to != 0x4096207 { return; }
  pc = 0x4096207; continue;
 }
 0x4096207 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04096330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096330); }
 let mut pc: u32 = 0x4096330;
 loop { match pc {
 0x4096330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096377; continue; }
  pc = 0x4096350; continue;
 }
 0x4096350 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409635e; continue; }
  pc = 0x4096355; continue;
 }
 0x4096355 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096377; continue; }
  pc = 0x409635e; continue;
 }
 0x409635e => {
  cpu.push(0x4096363); f_040a3b61(cpu); if cpu.returned_to != 0x4096363 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096377; continue; }
  pc = 0x4096367; continue;
 }
 0x4096367 => {
  cpu.push(0x409636c); f_0409d2c0(cpu); if cpu.returned_to != 0x409636c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096377; continue; }
  pc = 0x4096370; continue;
 }
 0x4096370 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4096379; continue;
 }
 0x4096377 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4096379; continue;
 }
 0x4096379 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40963d5; continue; }
  pc = 0x409637d; continue;
 }
 0x409637d => {
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
  { let v = 0x414ac1cu32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40963cd); cpu.call(t); if cpu.returned_to != 0x40963cd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40963d5); f_0409d690(cpu); if cpu.returned_to != 0x40963d5 { return; }
  pc = 0x40963d5; continue;
 }
 0x40963d5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x40963fa); f_040abaa0(cpu); if cpu.returned_to != 0x40963fa { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096420); }
 let mut pc: u32 = 0x4096420;
 loop { match pc {
 0x4096420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096468; continue; }
  pc = 0x4096441; continue;
 }
 0x4096441 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409644f; continue; }
  pc = 0x4096446; continue;
 }
 0x4096446 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096468; continue; }
  pc = 0x409644f; continue;
 }
 0x409644f => {
  cpu.push(0x4096454); f_040a3b61(cpu); if cpu.returned_to != 0x4096454 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096468; continue; }
  pc = 0x4096458; continue;
 }
 0x4096458 => {
  cpu.push(0x409645d); f_0409d2c0(cpu); if cpu.returned_to != 0x409645d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096468; continue; }
  pc = 0x4096461; continue;
 }
 0x4096461 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409646a; continue;
 }
 0x4096468 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409646a; continue;
 }
 0x409646a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409649c; continue; }
  pc = 0x409646e; continue;
 }
 0x409646e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ac00u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4096494); cpu.call(t); if cpu.returned_to != 0x4096494 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409649c); f_0409d690(cpu); if cpu.returned_to != 0x409649c { return; }
  pc = 0x409649c; continue;
 }
 0x409649c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40964a9); f_040abb00(cpu); if cpu.returned_to != 0x40964a9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40964f7; continue; }
  pc = 0x40964b8; continue;
 }
 0x40964b8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40964ca; continue; }
  pc = 0x40964c1; continue;
 }
 0x40964c1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40964f7; continue; }
  pc = 0x40964ca; continue;
 }
 0x40964ca => {
  cpu.push(0x40964cf); f_040a3b61(cpu); if cpu.returned_to != 0x40964cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40964f7; continue; }
  pc = 0x40964d3; continue;
 }
 0x40964d3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40964ed); f_040a3df3(cpu); if cpu.returned_to != 0x40964ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40964f7); f_0409d640(cpu); if cpu.returned_to != 0x40964f7 { return; }
  pc = 0x40964f7; continue;
 }
 0x40964f7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04096520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096520); }
 let mut pc: u32 = 0x4096520;
 loop { match pc {
 0x4096520 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096568; continue; }
  pc = 0x4096541; continue;
 }
 0x4096541 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409654f; continue; }
  pc = 0x4096546; continue;
 }
 0x4096546 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096568; continue; }
  pc = 0x409654f; continue;
 }
 0x409654f => {
  cpu.push(0x4096554); f_040a3b61(cpu); if cpu.returned_to != 0x4096554 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096568; continue; }
  pc = 0x4096558; continue;
 }
 0x4096558 => {
  cpu.push(0x409655d); f_0409d2c0(cpu); if cpu.returned_to != 0x409655d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096568; continue; }
  pc = 0x4096561; continue;
 }
 0x4096561 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409656a; continue;
 }
 0x4096568 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409656a; continue;
 }
 0x409656a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409659c; continue; }
  pc = 0x409656e; continue;
 }
 0x409656e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414abe4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4096594); cpu.call(t); if cpu.returned_to != 0x4096594 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409659c); f_0409d690(cpu); if cpu.returned_to != 0x409659c { return; }
  pc = 0x409659c; continue;
 }
 0x409659c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40965a9); f_040abb20(cpu); if cpu.returned_to != 0x40965a9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40965f7; continue; }
  pc = 0x40965b8; continue;
 }
 0x40965b8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40965ca; continue; }
  pc = 0x40965c1; continue;
 }
 0x40965c1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40965f7; continue; }
  pc = 0x40965ca; continue;
 }
 0x40965ca => {
  cpu.push(0x40965cf); f_040a3b61(cpu); if cpu.returned_to != 0x40965cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40965f7; continue; }
  pc = 0x40965d3; continue;
 }
 0x40965d3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40965ed); f_040a3df3(cpu); if cpu.returned_to != 0x40965ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40965f7); f_0409d640(cpu); if cpu.returned_to != 0x40965f7 { return; }
  pc = 0x40965f7; continue;
 }
 0x40965f7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04096620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096620); }
 let mut pc: u32 = 0x4096620;
 loop { match pc {
 0x4096620 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096667; continue; }
  pc = 0x4096640; continue;
 }
 0x4096640 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409664e; continue; }
  pc = 0x4096645; continue;
 }
 0x4096645 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096667; continue; }
  pc = 0x409664e; continue;
 }
 0x409664e => {
  cpu.push(0x4096653); f_040a3b61(cpu); if cpu.returned_to != 0x4096653 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096667; continue; }
  pc = 0x4096657; continue;
 }
 0x4096657 => {
  cpu.push(0x409665c); f_0409d2c0(cpu); if cpu.returned_to != 0x409665c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096667; continue; }
  pc = 0x4096660; continue;
 }
 0x4096660 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4096669; continue;
 }
 0x4096667 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4096669; continue;
 }
 0x4096669 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409669b; continue; }
  pc = 0x409666d; continue;
 }
 0x409666d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414abc4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4096693); cpu.call(t); if cpu.returned_to != 0x4096693 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409669b); f_0409d690(cpu); if cpu.returned_to != 0x409669b { return; }
  pc = 0x409669b; continue;
 }
 0x409669b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40966a8); f_040abb80(cpu); if cpu.returned_to != 0x40966a8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096780); }
 let mut pc: u32 = 0x4096780;
 loop { match pc {
 0x4096780 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40967c7; continue; }
  pc = 0x40967a0; continue;
 }
 0x40967a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40967ae; continue; }
  pc = 0x40967a5; continue;
 }
 0x40967a5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40967c7; continue; }
  pc = 0x40967ae; continue;
 }
 0x40967ae => {
  cpu.push(0x40967b3); f_040a3b61(cpu); if cpu.returned_to != 0x40967b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40967c7; continue; }
  pc = 0x40967b7; continue;
 }
 0x40967b7 => {
  cpu.push(0x40967bc); f_0409d2c0(cpu); if cpu.returned_to != 0x40967bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40967c7; continue; }
  pc = 0x40967c0; continue;
 }
 0x40967c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40967c9; continue;
 }
 0x40967c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40967c9; continue;
 }
 0x40967c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40967fb; continue; }
  pc = 0x40967cd; continue;
 }
 0x40967cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ab8cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40967f3); cpu.call(t); if cpu.returned_to != 0x40967f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40967fb); f_0409d690(cpu); if cpu.returned_to != 0x40967fb { return; }
  pc = 0x40967fb; continue;
 }
 0x40967fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4096808); f_040abbc0(cpu); if cpu.returned_to != 0x4096808 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096830); }
 let mut pc: u32 = 0x4096830;
 loop { match pc {
 0x4096830 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096878; continue; }
  pc = 0x4096851; continue;
 }
 0x4096851 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409685f; continue; }
  pc = 0x4096856; continue;
 }
 0x4096856 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096878; continue; }
  pc = 0x409685f; continue;
 }
 0x409685f => {
  cpu.push(0x4096864); f_040a3b61(cpu); if cpu.returned_to != 0x4096864 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096878; continue; }
  pc = 0x4096868; continue;
 }
 0x4096868 => {
  cpu.push(0x409686d); f_0409d2c0(cpu); if cpu.returned_to != 0x409686d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096878; continue; }
  pc = 0x4096871; continue;
 }
 0x4096871 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409687a; continue;
 }
 0x4096878 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409687a; continue;
 }
 0x409687a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40968ac; continue; }
  pc = 0x409687e; continue;
 }
 0x409687e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ab74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40968a4); cpu.call(t); if cpu.returned_to != 0x40968a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40968ac); f_0409d690(cpu); if cpu.returned_to != 0x40968ac { return; }
  pc = 0x40968ac; continue;
 }
 0x40968ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40968b9); f_040abc10(cpu); if cpu.returned_to != 0x40968b9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4096907; continue; }
  pc = 0x40968c8; continue;
 }
 0x40968c8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40968da; continue; }
  pc = 0x40968d1; continue;
 }
 0x40968d1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096907; continue; }
  pc = 0x40968da; continue;
 }
 0x40968da => {
  cpu.push(0x40968df); f_040a3b61(cpu); if cpu.returned_to != 0x40968df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096907; continue; }
  pc = 0x40968e3; continue;
 }
 0x40968e3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40968fd); f_040a3df3(cpu); if cpu.returned_to != 0x40968fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4096907); f_0409d5f0(cpu); if cpu.returned_to != 0x4096907 { return; }
  pc = 0x4096907; continue;
 }
 0x4096907 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04096930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096930); }
 let mut pc: u32 = 0x4096930;
 loop { match pc {
 0x4096930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096977; continue; }
  pc = 0x4096950; continue;
 }
 0x4096950 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409695e; continue; }
  pc = 0x4096955; continue;
 }
 0x4096955 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096977; continue; }
  pc = 0x409695e; continue;
 }
 0x409695e => {
  cpu.push(0x4096963); f_040a3b61(cpu); if cpu.returned_to != 0x4096963 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096977; continue; }
  pc = 0x4096967; continue;
 }
 0x4096967 => {
  cpu.push(0x409696c); f_0409d2c0(cpu); if cpu.returned_to != 0x409696c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096977; continue; }
  pc = 0x4096970; continue;
 }
 0x4096970 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4096979; continue;
 }
 0x4096977 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4096979; continue;
 }
 0x4096979 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40969ab; continue; }
  pc = 0x409697d; continue;
 }
 0x409697d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414ab4cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40969a3); cpu.call(t); if cpu.returned_to != 0x40969a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40969ab); f_0409d690(cpu); if cpu.returned_to != 0x40969ab { return; }
  pc = 0x40969ab; continue;
 }
 0x40969ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40969b8); f_040aada0(cpu); if cpu.returned_to != 0x40969b8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096de0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096de0); }
 let mut pc: u32 = 0x4096de0;
 loop { match pc {
 0x4096de0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096e27; continue; }
  pc = 0x4096e00; continue;
 }
 0x4096e00 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4096e0e; continue; }
  pc = 0x4096e05; continue;
 }
 0x4096e05 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096e27; continue; }
  pc = 0x4096e0e; continue;
 }
 0x4096e0e => {
  cpu.push(0x4096e13); f_040a3b61(cpu); if cpu.returned_to != 0x4096e13 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096e27; continue; }
  pc = 0x4096e17; continue;
 }
 0x4096e17 => {
  cpu.push(0x4096e1c); f_0409d2c0(cpu); if cpu.returned_to != 0x4096e1c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096e27; continue; }
  pc = 0x4096e20; continue;
 }
 0x4096e20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4096e29; continue;
 }
 0x4096e27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4096e29; continue;
 }
 0x4096e29 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096e77; continue; }
  pc = 0x4096e2d; continue;
 }
 0x4096e2d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414aa68u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4096e6f); cpu.call(t); if cpu.returned_to != 0x4096e6f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4096e77); f_0409d690(cpu); if cpu.returned_to != 0x4096e77 { return; }
  pc = 0x4096e77; continue;
 }
 0x4096e77 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4096e94); f_040a7530(cpu); if cpu.returned_to != 0x4096e94 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096eb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096eb0); }
 let mut pc: u32 = 0x4096eb0;
 loop { match pc {
 0x4096eb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096ef7; continue; }
  pc = 0x4096ed0; continue;
 }
 0x4096ed0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4096ede; continue; }
  pc = 0x4096ed5; continue;
 }
 0x4096ed5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096ef7; continue; }
  pc = 0x4096ede; continue;
 }
 0x4096ede => {
  cpu.push(0x4096ee3); f_040a3b61(cpu); if cpu.returned_to != 0x4096ee3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096ef7; continue; }
  pc = 0x4096ee7; continue;
 }
 0x4096ee7 => {
  cpu.push(0x4096eec); f_0409d2c0(cpu); if cpu.returned_to != 0x4096eec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096ef7; continue; }
  pc = 0x4096ef0; continue;
 }
 0x4096ef0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4096ef9; continue;
 }
 0x4096ef7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4096ef9; continue;
 }
 0x4096ef9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096f39; continue; }
  pc = 0x4096efd; continue;
 }
 0x4096efd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414aa48u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4096f31); cpu.call(t); if cpu.returned_to != 0x4096f31 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4096f39); f_0409d690(cpu); if cpu.returned_to != 0x4096f39 { return; }
  pc = 0x4096f39; continue;
 }
 0x4096f39 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4096f4e); f_040a7570(cpu); if cpu.returned_to != 0x4096f4e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04096f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4096f70); }
 let mut pc: u32 = 0x4096f70;
 loop { match pc {
 0x4096f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4096fb8; continue; }
  pc = 0x4096f91; continue;
 }
 0x4096f91 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4096f9f; continue; }
  pc = 0x4096f96; continue;
 }
 0x4096f96 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4096fb8; continue; }
  pc = 0x4096f9f; continue;
 }
 0x4096f9f => {
  cpu.push(0x4096fa4); f_040a3b61(cpu); if cpu.returned_to != 0x4096fa4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4096fb8; continue; }
  pc = 0x4096fa8; continue;
 }
 0x4096fa8 => {
  cpu.push(0x4096fad); f_0409d2c0(cpu); if cpu.returned_to != 0x4096fad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096fb8; continue; }
  pc = 0x4096fb1; continue;
 }
 0x4096fb1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4096fba; continue;
 }
 0x4096fb8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4096fba; continue;
 }
 0x4096fba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4096fec; continue; }
  pc = 0x4096fbe; continue;
 }
 0x4096fbe => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414aa2cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4096fe4); cpu.call(t); if cpu.returned_to != 0x4096fe4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4096fec); f_0409d690(cpu); if cpu.returned_to != 0x4096fec { return; }
  pc = 0x4096fec; continue;
 }
 0x4096fec => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4096ff9); f_040a7690(cpu); if cpu.returned_to != 0x4096ff9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4097047; continue; }
  pc = 0x4097008; continue;
 }
 0x4097008 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409701a; continue; }
  pc = 0x4097011; continue;
 }
 0x4097011 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097047; continue; }
  pc = 0x409701a; continue;
 }
 0x409701a => {
  cpu.push(0x409701f); f_040a3b61(cpu); if cpu.returned_to != 0x409701f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097047; continue; }
  pc = 0x4097023; continue;
 }
 0x4097023 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409703d); f_040a3df3(cpu); if cpu.returned_to != 0x409703d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4097047); f_0409d640(cpu); if cpu.returned_to != 0x4097047 { return; }
  pc = 0x4097047; continue;
 }
 0x4097047 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04097070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097070); }
 let mut pc: u32 = 0x4097070;
 loop { match pc {
 0x4097070 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40970b7; continue; }
  pc = 0x4097090; continue;
 }
 0x4097090 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409709e; continue; }
  pc = 0x4097095; continue;
 }
 0x4097095 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40970b7; continue; }
  pc = 0x409709e; continue;
 }
 0x409709e => {
  cpu.push(0x40970a3); f_040a3b61(cpu); if cpu.returned_to != 0x40970a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40970b7; continue; }
  pc = 0x40970a7; continue;
 }
 0x40970a7 => {
  cpu.push(0x40970ac); f_0409d2c0(cpu); if cpu.returned_to != 0x40970ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40970b7; continue; }
  pc = 0x40970b0; continue;
 }
 0x40970b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40970b9; continue;
 }
 0x40970b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40970b9; continue;
 }
 0x40970b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40970f9; continue; }
  pc = 0x40970bd; continue;
 }
 0x40970bd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414aa08u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40970f1); cpu.call(t); if cpu.returned_to != 0x40970f1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40970f9); f_0409d690(cpu); if cpu.returned_to != 0x40970f9 { return; }
  pc = 0x40970f9; continue;
 }
 0x40970f9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409710e); f_040a7810(cpu); if cpu.returned_to != 0x409710e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04097130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097130); }
 let mut pc: u32 = 0x4097130;
 loop { match pc {
 0x4097130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097178; continue; }
  pc = 0x4097151; continue;
 }
 0x4097151 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409715f; continue; }
  pc = 0x4097156; continue;
 }
 0x4097156 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097178; continue; }
  pc = 0x409715f; continue;
 }
 0x409715f => {
  cpu.push(0x4097164); f_040a3b61(cpu); if cpu.returned_to != 0x4097164 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097178; continue; }
  pc = 0x4097168; continue;
 }
 0x4097168 => {
  cpu.push(0x409716d); f_0409d2c0(cpu); if cpu.returned_to != 0x409716d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097178; continue; }
  pc = 0x4097171; continue;
 }
 0x4097171 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409717a; continue;
 }
 0x4097178 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409717a; continue;
 }
 0x409717a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40971ba; continue; }
  pc = 0x409717e; continue;
 }
 0x409717e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a9e4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40971b2); cpu.call(t); if cpu.returned_to != 0x40971b2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40971ba); f_0409d690(cpu); if cpu.returned_to != 0x40971ba { return; }
  pc = 0x40971ba; continue;
 }
 0x40971ba => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40971cf); f_040a7820(cpu); if cpu.returned_to != 0x40971cf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409721c; continue; }
  pc = 0x40971dd; continue;
 }
 0x40971dd => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40971ef; continue; }
  pc = 0x40971e6; continue;
 }
 0x40971e6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409721c; continue; }
  pc = 0x40971ef; continue;
 }
 0x40971ef => {
  cpu.push(0x40971f4); f_040a3b61(cpu); if cpu.returned_to != 0x40971f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409721c; continue; }
  pc = 0x40971f8; continue;
 }
 0x40971f8 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4097212); f_040a3df3(cpu); if cpu.returned_to != 0x4097212 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409721c); f_0409d640(cpu); if cpu.returned_to != 0x409721c { return; }
  pc = 0x409721c; continue;
 }
 0x409721c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04097240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097240); }
 let mut pc: u32 = 0x4097240;
 loop { match pc {
 0x4097240 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097287; continue; }
  pc = 0x4097260; continue;
 }
 0x4097260 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409726e; continue; }
  pc = 0x4097265; continue;
 }
 0x4097265 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097287; continue; }
  pc = 0x409726e; continue;
 }
 0x409726e => {
  cpu.push(0x4097273); f_040a3b61(cpu); if cpu.returned_to != 0x4097273 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097287; continue; }
  pc = 0x4097277; continue;
 }
 0x4097277 => {
  cpu.push(0x409727c); f_0409d2c0(cpu); if cpu.returned_to != 0x409727c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097287; continue; }
  pc = 0x4097280; continue;
 }
 0x4097280 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4097289; continue;
 }
 0x4097287 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4097289; continue;
 }
 0x4097289 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40972d7; continue; }
  pc = 0x409728d; continue;
 }
 0x409728d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a9b8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40972cf); cpu.call(t); if cpu.returned_to != 0x40972cf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40972d7); f_0409d690(cpu); if cpu.returned_to != 0x40972d7 { return; }
  pc = 0x40972d7; continue;
 }
 0x40972d7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40972f4); f_040a7840(cpu); if cpu.returned_to != 0x40972f4 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04097310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097310); }
 let mut pc: u32 = 0x4097310;
 loop { match pc {
 0x4097310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097358; continue; }
  pc = 0x4097331; continue;
 }
 0x4097331 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409733f; continue; }
  pc = 0x4097336; continue;
 }
 0x4097336 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097358; continue; }
  pc = 0x409733f; continue;
 }
 0x409733f => {
  cpu.push(0x4097344); f_040a3b61(cpu); if cpu.returned_to != 0x4097344 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097358; continue; }
  pc = 0x4097348; continue;
 }
 0x4097348 => {
  cpu.push(0x409734d); f_0409d2c0(cpu); if cpu.returned_to != 0x409734d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097358; continue; }
  pc = 0x4097351; continue;
 }
 0x4097351 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409735a; continue;
 }
 0x4097358 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409735a; continue;
 }
 0x409735a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409739a; continue; }
  pc = 0x409735e; continue;
 }
 0x409735e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a98cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4097392); cpu.call(t); if cpu.returned_to != 0x4097392 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409739a); f_0409d690(cpu); if cpu.returned_to != 0x409739a { return; }
  pc = 0x409739a; continue;
 }
 0x409739a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40973af); f_040a7860(cpu); if cpu.returned_to != 0x40973af { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40973fc; continue; }
  pc = 0x40973bd; continue;
 }
 0x40973bd => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40973cf; continue; }
  pc = 0x40973c6; continue;
 }
 0x40973c6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40973fc; continue; }
  pc = 0x40973cf; continue;
 }
 0x40973cf => {
  cpu.push(0x40973d4); f_040a3b61(cpu); if cpu.returned_to != 0x40973d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40973fc; continue; }
  pc = 0x40973d8; continue;
 }
 0x40973d8 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40973f2); f_040a3df3(cpu); if cpu.returned_to != 0x40973f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40973fc); f_0409d5f0(cpu); if cpu.returned_to != 0x40973fc { return; }
  pc = 0x40973fc; continue;
 }
 0x40973fc => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04097420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097420); }
 let mut pc: u32 = 0x4097420;
 loop { match pc {
 0x4097420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097468; continue; }
  pc = 0x4097441; continue;
 }
 0x4097441 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409744f; continue; }
  pc = 0x4097446; continue;
 }
 0x4097446 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097468; continue; }
  pc = 0x409744f; continue;
 }
 0x409744f => {
  cpu.push(0x4097454); f_040a3b61(cpu); if cpu.returned_to != 0x4097454 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097468; continue; }
  pc = 0x4097458; continue;
 }
 0x4097458 => {
  cpu.push(0x409745d); f_0409d2c0(cpu); if cpu.returned_to != 0x409745d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097468; continue; }
  pc = 0x4097461; continue;
 }
 0x4097461 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409746a; continue;
 }
 0x4097468 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409746a; continue;
 }
 0x409746a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40974aa; continue; }
  pc = 0x409746e; continue;
 }
 0x409746e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a960u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40974a2); cpu.call(t); if cpu.returned_to != 0x40974a2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40974aa); f_0409d690(cpu); if cpu.returned_to != 0x40974aa { return; }
  pc = 0x40974aa; continue;
 }
 0x40974aa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40974bf); f_040a7880(cpu); if cpu.returned_to != 0x40974bf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409750c; continue; }
  pc = 0x40974cd; continue;
 }
 0x40974cd => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40974df; continue; }
  pc = 0x40974d6; continue;
 }
 0x40974d6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409750c; continue; }
  pc = 0x40974df; continue;
 }
 0x40974df => {
  cpu.push(0x40974e4); f_040a3b61(cpu); if cpu.returned_to != 0x40974e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409750c; continue; }
  pc = 0x40974e8; continue;
 }
 0x40974e8 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4097502); f_040a3df3(cpu); if cpu.returned_to != 0x4097502 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409750c); f_0409d5f0(cpu); if cpu.returned_to != 0x409750c { return; }
  pc = 0x409750c; continue;
 }
 0x409750c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04097640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097640); }
 let mut pc: u32 = 0x4097640;
 loop { match pc {
 0x4097640 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097688; continue; }
  pc = 0x4097661; continue;
 }
 0x4097661 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409766f; continue; }
  pc = 0x4097666; continue;
 }
 0x4097666 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097688; continue; }
  pc = 0x409766f; continue;
 }
 0x409766f => {
  cpu.push(0x4097674); f_040a3b61(cpu); if cpu.returned_to != 0x4097674 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097688; continue; }
  pc = 0x4097678; continue;
 }
 0x4097678 => {
  cpu.push(0x409767d); f_0409d2c0(cpu); if cpu.returned_to != 0x409767d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097688; continue; }
  pc = 0x4097681; continue;
 }
 0x4097681 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409768a; continue;
 }
 0x4097688 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409768a; continue;
 }
 0x409768a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40976ca; continue; }
  pc = 0x409768e; continue;
 }
 0x409768e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a90cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40976c2); cpu.call(t); if cpu.returned_to != 0x40976c2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40976ca); f_0409d690(cpu); if cpu.returned_to != 0x40976ca { return; }
  pc = 0x40976ca; continue;
 }
 0x40976ca => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x40976df); f_040a78c0(cpu); if cpu.returned_to != 0x40976df { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409772c; continue; }
  pc = 0x40976ed; continue;
 }
 0x40976ed => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40976ff; continue; }
  pc = 0x40976f6; continue;
 }
 0x40976f6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409772c; continue; }
  pc = 0x40976ff; continue;
 }
 0x40976ff => {
  cpu.push(0x4097704); f_040a3b61(cpu); if cpu.returned_to != 0x4097704 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409772c; continue; }
  pc = 0x4097708; continue;
 }
 0x4097708 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4097722); f_040a3df3(cpu); if cpu.returned_to != 0x4097722 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409772c); f_0409d5f0(cpu); if cpu.returned_to != 0x409772c { return; }
  pc = 0x409772c; continue;
 }
 0x409772c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04097970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097970); }
 let mut pc: u32 = 0x4097970;
 loop { match pc {
 0x4097970 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40979b7; continue; }
  pc = 0x4097990; continue;
 }
 0x4097990 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409799e; continue; }
  pc = 0x4097995; continue;
 }
 0x4097995 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40979b7; continue; }
  pc = 0x409799e; continue;
 }
 0x409799e => {
  cpu.push(0x40979a3); f_040a3b61(cpu); if cpu.returned_to != 0x40979a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40979b7; continue; }
  pc = 0x40979a7; continue;
 }
 0x40979a7 => {
  cpu.push(0x40979ac); f_0409d2c0(cpu); if cpu.returned_to != 0x40979ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40979b7; continue; }
  pc = 0x40979b0; continue;
 }
 0x40979b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40979b9; continue;
 }
 0x40979b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40979b9; continue;
 }
 0x40979b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40979f9; continue; }
  pc = 0x40979bd; continue;
 }
 0x40979bd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a898u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40979f1); cpu.call(t); if cpu.returned_to != 0x40979f1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40979f9); f_0409d690(cpu); if cpu.returned_to != 0x40979f9 { return; }
  pc = 0x40979f9; continue;
 }
 0x40979f9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4097a0e); f_040a7920(cpu); if cpu.returned_to != 0x4097a0e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04097a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097a30); }
 let mut pc: u32 = 0x4097a30;
 loop { match pc {
 0x4097a30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x114u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097a86; continue; }
  pc = 0x4097a5f; continue;
 }
 0x4097a5f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4097a6d; continue; }
  pc = 0x4097a64; continue;
 }
 0x4097a64 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097a86; continue; }
  pc = 0x4097a6d; continue;
 }
 0x4097a6d => {
  cpu.push(0x4097a72); f_040a3b61(cpu); if cpu.returned_to != 0x4097a72 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097a86; continue; }
  pc = 0x4097a76; continue;
 }
 0x4097a76 => {
  cpu.push(0x4097a7b); f_0409d2c0(cpu); if cpu.returned_to != 0x4097a7b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097a86; continue; }
  pc = 0x4097a7f; continue;
 }
 0x4097a7f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4097a88; continue;
 }
 0x4097a86 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4097a88; continue;
 }
 0x4097a88 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097ac8; continue; }
  pc = 0x4097a8c; continue;
 }
 0x4097a8c => {
  { let v: u32 = cpu.r32(0x128u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a85cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4097ac0); cpu.call(t); if cpu.returned_to != 0x4097ac0 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4097ac8); f_0409d690(cpu); if cpu.returned_to != 0x4097ac8 { return; }
  pc = 0x4097ac8; continue;
 }
 0x4097ac8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4097ad7); f_040a79b0(cpu); if cpu.returned_to != 0x4097ad7 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x4097b5e; continue; }
  pc = 0x4097ae8; continue;
 }
 0x4097ae8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4097afa; continue; }
  pc = 0x4097af1; continue;
 }
 0x4097af1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097b5e; continue; }
  pc = 0x4097afa; continue;
 }
 0x4097afa => {
  cpu.push(0x4097aff); f_040a3b61(cpu); if cpu.returned_to != 0x4097aff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097b5e; continue; }
  pc = 0x4097b03; continue;
 }
 0x4097b03 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4097b1d); f_040a3df3(cpu); if cpu.returned_to != 0x4097b1d { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a884u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4097b51); cpu.call(t); if cpu.returned_to != 0x4097b51 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4097b5e); f_0409d690(cpu); if cpu.returned_to != 0x4097b5e { return; }
  pc = 0x4097b5e; continue;
 }
 0x4097b5e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x42c538cu32)); cpu.w32(0x42c538cu32, v); }
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
pub fn f_04097b70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4097b70); }
 let mut pc: u32 = 0x4097b70;
 loop { match pc {
 0x4097b70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4097bb8; continue; }
  pc = 0x4097b91; continue;
 }
 0x4097b91 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4097b9f; continue; }
  pc = 0x4097b96; continue;
 }
 0x4097b96 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097bb8; continue; }
  pc = 0x4097b9f; continue;
 }
 0x4097b9f => {
  cpu.push(0x4097ba4); f_040a3b61(cpu); if cpu.returned_to != 0x4097ba4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097bb8; continue; }
  pc = 0x4097ba8; continue;
 }
 0x4097ba8 => {
  cpu.push(0x4097bad); f_0409d2c0(cpu); if cpu.returned_to != 0x4097bad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097bb8; continue; }
  pc = 0x4097bb1; continue;
 }
 0x4097bb1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4097bba; continue;
 }
 0x4097bb8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4097bba; continue;
 }
 0x4097bba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097bfa; continue; }
  pc = 0x4097bbe; continue;
 }
 0x4097bbe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a83cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4097bf2); cpu.call(t); if cpu.returned_to != 0x4097bf2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4097bfa); f_0409d690(cpu); if cpu.returned_to != 0x4097bfa { return; }
  pc = 0x4097bfa; continue;
 }
 0x4097bfa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4097c0f); f_040a7a30(cpu); if cpu.returned_to != 0x4097c0f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4097c5c; continue; }
  pc = 0x4097c1d; continue;
 }
 0x4097c1d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4097c2f; continue; }
  pc = 0x4097c26; continue;
 }
 0x4097c26 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4097c5c; continue; }
  pc = 0x4097c2f; continue;
 }
 0x4097c2f => {
  cpu.push(0x4097c34); f_040a3b61(cpu); if cpu.returned_to != 0x4097c34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4097c5c; continue; }
  pc = 0x4097c38; continue;
 }
 0x4097c38 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4097c52); f_040a3df3(cpu); if cpu.returned_to != 0x4097c52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4097c5c); f_0409d640(cpu); if cpu.returned_to != 0x4097c5c { return; }
  pc = 0x4097c5c; continue;
 }
 0x4097c5c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_04098280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098280); }
 let mut pc: u32 = 0x4098280;
 loop { match pc {
 0x4098280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40982c7; continue; }
  pc = 0x40982a0; continue;
 }
 0x40982a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40982ae; continue; }
  pc = 0x40982a5; continue;
 }
 0x40982a5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40982c7; continue; }
  pc = 0x40982ae; continue;
 }
 0x40982ae => {
  cpu.push(0x40982b3); f_040a3b61(cpu); if cpu.returned_to != 0x40982b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40982c7; continue; }
  pc = 0x40982b7; continue;
 }
 0x40982b7 => {
  cpu.push(0x40982bc); f_0409d2c0(cpu); if cpu.returned_to != 0x40982bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40982c7; continue; }
  pc = 0x40982c0; continue;
 }
 0x40982c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40982c9; continue;
 }
 0x40982c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40982c9; continue;
 }
 0x40982c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40982fb; continue; }
  pc = 0x40982cd; continue;
 }
 0x40982cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a744u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40982f3); cpu.call(t); if cpu.returned_to != 0x40982f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40982fb); f_0409d690(cpu); if cpu.returned_to != 0x40982fb { return; }
  pc = 0x40982fb; continue;
 }
 0x40982fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4098308); f_040a84c0(cpu); if cpu.returned_to != 0x4098308 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04098330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098330); }
 let mut pc: u32 = 0x4098330;
 loop { match pc {
 0x4098330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098378; continue; }
  pc = 0x4098351; continue;
 }
 0x4098351 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409835f; continue; }
  pc = 0x4098356; continue;
 }
 0x4098356 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098378; continue; }
  pc = 0x409835f; continue;
 }
 0x409835f => {
  cpu.push(0x4098364); f_040a3b61(cpu); if cpu.returned_to != 0x4098364 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098378; continue; }
  pc = 0x4098368; continue;
 }
 0x4098368 => {
  cpu.push(0x409836d); f_0409d2c0(cpu); if cpu.returned_to != 0x409836d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098378; continue; }
  pc = 0x4098371; continue;
 }
 0x4098371 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409837a; continue;
 }
 0x4098378 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409837a; continue;
 }
 0x409837a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40983ac; continue; }
  pc = 0x409837e; continue;
 }
 0x409837e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a728u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40983a4); cpu.call(t); if cpu.returned_to != 0x40983a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40983ac); f_0409d690(cpu); if cpu.returned_to != 0x40983ac { return; }
  pc = 0x40983ac; continue;
 }
 0x40983ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40983b9); f_040a8540(cpu); if cpu.returned_to != 0x40983b9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4098407; continue; }
  pc = 0x40983c8; continue;
 }
 0x40983c8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40983da; continue; }
  pc = 0x40983d1; continue;
 }
 0x40983d1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098407; continue; }
  pc = 0x40983da; continue;
 }
 0x40983da => {
  cpu.push(0x40983df); f_040a3b61(cpu); if cpu.returned_to != 0x40983df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098407; continue; }
  pc = 0x40983e3; continue;
 }
 0x40983e3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40983fd); f_040a3df3(cpu); if cpu.returned_to != 0x40983fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4098407); f_0409d640(cpu); if cpu.returned_to != 0x4098407 { return; }
  pc = 0x4098407; continue;
 }
 0x4098407 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_04098a10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098a10); }
 let mut pc: u32 = 0x4098a10;
 loop { match pc {
 0x4098a10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098a58; continue; }
  pc = 0x4098a31; continue;
 }
 0x4098a31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098a3f; continue; }
  pc = 0x4098a36; continue;
 }
 0x4098a36 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098a58; continue; }
  pc = 0x4098a3f; continue;
 }
 0x4098a3f => {
  cpu.push(0x4098a44); f_040a3b61(cpu); if cpu.returned_to != 0x4098a44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098a58; continue; }
  pc = 0x4098a48; continue;
 }
 0x4098a48 => {
  cpu.push(0x4098a4d); f_0409d2c0(cpu); if cpu.returned_to != 0x4098a4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098a58; continue; }
  pc = 0x4098a51; continue;
 }
 0x4098a51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098a5a; continue;
 }
 0x4098a58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098a5a; continue;
 }
 0x4098a5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098a8c; continue; }
  pc = 0x4098a5e; continue;
 }
 0x4098a5e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a63cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098a84); cpu.call(t); if cpu.returned_to != 0x4098a84 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098a8c); f_0409d690(cpu); if cpu.returned_to != 0x4098a8c { return; }
  pc = 0x4098a8c; continue;
 }
 0x4098a8c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4098a99); f_040a7d20(cpu); if cpu.returned_to != 0x4098a99 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4098ae7; continue; }
  pc = 0x4098aa8; continue;
 }
 0x4098aa8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4098aba; continue; }
  pc = 0x4098ab1; continue;
 }
 0x4098ab1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098ae7; continue; }
  pc = 0x4098aba; continue;
 }
 0x4098aba => {
  cpu.push(0x4098abf); f_040a3b61(cpu); if cpu.returned_to != 0x4098abf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098ae7; continue; }
  pc = 0x4098ac3; continue;
 }
 0x4098ac3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x4098add); f_040a3df3(cpu); if cpu.returned_to != 0x4098add { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4098ae7); f_0409d640(cpu); if cpu.returned_to != 0x4098ae7 { return; }
  pc = 0x4098ae7; continue;
 }
 0x4098ae7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_04098b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098b10); }
 let mut pc: u32 = 0x4098b10;
 loop { match pc {
 0x4098b10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098b57; continue; }
  pc = 0x4098b30; continue;
 }
 0x4098b30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098b3e; continue; }
  pc = 0x4098b35; continue;
 }
 0x4098b35 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098b57; continue; }
  pc = 0x4098b3e; continue;
 }
 0x4098b3e => {
  cpu.push(0x4098b43); f_040a3b61(cpu); if cpu.returned_to != 0x4098b43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098b57; continue; }
  pc = 0x4098b47; continue;
 }
 0x4098b47 => {
  cpu.push(0x4098b4c); f_0409d2c0(cpu); if cpu.returned_to != 0x4098b4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098b57; continue; }
  pc = 0x4098b50; continue;
 }
 0x4098b50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098b59; continue;
 }
 0x4098b57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098b59; continue;
 }
 0x4098b59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098b99; continue; }
  pc = 0x4098b5d; continue;
 }
 0x4098b5d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a614u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098b91); cpu.call(t); if cpu.returned_to != 0x4098b91 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098b99); f_0409d690(cpu); if cpu.returned_to != 0x4098b99 { return; }
  pc = 0x4098b99; continue;
 }
 0x4098b99 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4098bae); f_040a7e20(cpu); if cpu.returned_to != 0x4098bae { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04098bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098bd0); }
 let mut pc: u32 = 0x4098bd0;
 loop { match pc {
 0x4098bd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098c17; continue; }
  pc = 0x4098bf0; continue;
 }
 0x4098bf0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098bfe; continue; }
  pc = 0x4098bf5; continue;
 }
 0x4098bf5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098c17; continue; }
  pc = 0x4098bfe; continue;
 }
 0x4098bfe => {
  cpu.push(0x4098c03); f_040a3b61(cpu); if cpu.returned_to != 0x4098c03 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098c17; continue; }
  pc = 0x4098c07; continue;
 }
 0x4098c07 => {
  cpu.push(0x4098c0c); f_0409d2c0(cpu); if cpu.returned_to != 0x4098c0c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098c17; continue; }
  pc = 0x4098c10; continue;
 }
 0x4098c10 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098c19; continue;
 }
 0x4098c17 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098c19; continue;
 }
 0x4098c19 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098c67; continue; }
  pc = 0x4098c1d; continue;
 }
 0x4098c1d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a5ecu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098c5f); cpu.call(t); if cpu.returned_to != 0x4098c5f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098c67); f_0409d690(cpu); if cpu.returned_to != 0x4098c67 { return; }
  pc = 0x4098c67; continue;
 }
 0x4098c67 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098c84); f_040a7d30(cpu); if cpu.returned_to != 0x4098c84 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_04098ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098ca0); }
 let mut pc: u32 = 0x4098ca0;
 loop { match pc {
 0x4098ca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098ce7; continue; }
  pc = 0x4098cc0; continue;
 }
 0x4098cc0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098cce; continue; }
  pc = 0x4098cc5; continue;
 }
 0x4098cc5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098ce7; continue; }
  pc = 0x4098cce; continue;
 }
 0x4098cce => {
  cpu.push(0x4098cd3); f_040a3b61(cpu); if cpu.returned_to != 0x4098cd3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098ce7; continue; }
  pc = 0x4098cd7; continue;
 }
 0x4098cd7 => {
  cpu.push(0x4098cdc); f_0409d2c0(cpu); if cpu.returned_to != 0x4098cdc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098ce7; continue; }
  pc = 0x4098ce0; continue;
 }
 0x4098ce0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098ce9; continue;
 }
 0x4098ce7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098ce9; continue;
 }
 0x4098ce9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098d37; continue; }
  pc = 0x4098ced; continue;
 }
 0x4098ced => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a5c4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098d2f); cpu.call(t); if cpu.returned_to != 0x4098d2f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098d37); f_0409d690(cpu); if cpu.returned_to != 0x4098d37 { return; }
  pc = 0x4098d37; continue;
 }
 0x4098d37 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098d54); f_040a7db0(cpu); if cpu.returned_to != 0x4098d54 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04098d70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098d70); }
 let mut pc: u32 = 0x4098d70;
 loop { match pc {
 0x4098d70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098db7; continue; }
  pc = 0x4098d90; continue;
 }
 0x4098d90 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098d9e; continue; }
  pc = 0x4098d95; continue;
 }
 0x4098d95 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098db7; continue; }
  pc = 0x4098d9e; continue;
 }
 0x4098d9e => {
  cpu.push(0x4098da3); f_040a3b61(cpu); if cpu.returned_to != 0x4098da3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098db7; continue; }
  pc = 0x4098da7; continue;
 }
 0x4098da7 => {
  cpu.push(0x4098dac); f_0409d2c0(cpu); if cpu.returned_to != 0x4098dac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098db7; continue; }
  pc = 0x4098db0; continue;
 }
 0x4098db0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098db9; continue;
 }
 0x4098db7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098db9; continue;
 }
 0x4098db9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098deb; continue; }
  pc = 0x4098dbd; continue;
 }
 0x4098dbd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a5a8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098de3); cpu.call(t); if cpu.returned_to != 0x4098de3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098deb); f_0409d690(cpu); if cpu.returned_to != 0x4098deb { return; }
  pc = 0x4098deb; continue;
 }
 0x4098deb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4098df8); f_040a7e30(cpu); if cpu.returned_to != 0x4098df8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04098e20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098e20); }
 let mut pc: u32 = 0x4098e20;
 loop { match pc {
 0x4098e20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098e67; continue; }
  pc = 0x4098e40; continue;
 }
 0x4098e40 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098e4e; continue; }
  pc = 0x4098e45; continue;
 }
 0x4098e45 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098e67; continue; }
  pc = 0x4098e4e; continue;
 }
 0x4098e4e => {
  cpu.push(0x4098e53); f_040a3b61(cpu); if cpu.returned_to != 0x4098e53 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098e67; continue; }
  pc = 0x4098e57; continue;
 }
 0x4098e57 => {
  cpu.push(0x4098e5c); f_0409d2c0(cpu); if cpu.returned_to != 0x4098e5c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098e67; continue; }
  pc = 0x4098e60; continue;
 }
 0x4098e60 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098e69; continue;
 }
 0x4098e67 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098e69; continue;
 }
 0x4098e69 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098e9b; continue; }
  pc = 0x4098e6d; continue;
 }
 0x4098e6d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a58cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098e93); cpu.call(t); if cpu.returned_to != 0x4098e93 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098e9b); f_0409d690(cpu); if cpu.returned_to != 0x4098e9b { return; }
  pc = 0x4098e9b; continue;
 }
 0x4098e9b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4098ea8); f_040a7e70(cpu); if cpu.returned_to != 0x4098ea8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04098ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098ed0); }
 let mut pc: u32 = 0x4098ed0;
 loop { match pc {
 0x4098ed0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098f17; continue; }
  pc = 0x4098ef0; continue;
 }
 0x4098ef0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098efe; continue; }
  pc = 0x4098ef5; continue;
 }
 0x4098ef5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098f17; continue; }
  pc = 0x4098efe; continue;
 }
 0x4098efe => {
  cpu.push(0x4098f03); f_040a3b61(cpu); if cpu.returned_to != 0x4098f03 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098f17; continue; }
  pc = 0x4098f07; continue;
 }
 0x4098f07 => {
  cpu.push(0x4098f0c); f_0409d2c0(cpu); if cpu.returned_to != 0x4098f0c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098f17; continue; }
  pc = 0x4098f10; continue;
 }
 0x4098f10 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098f19; continue;
 }
 0x4098f17 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098f19; continue;
 }
 0x4098f19 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098f4b; continue; }
  pc = 0x4098f1d; continue;
 }
 0x4098f1d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a570u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098f43); cpu.call(t); if cpu.returned_to != 0x4098f43 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098f4b); f_0409d690(cpu); if cpu.returned_to != 0x4098f4b { return; }
  pc = 0x4098f4b; continue;
 }
 0x4098f4b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4098f58); f_040a7ed0(cpu); if cpu.returned_to != 0x4098f58 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04098f80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4098f80); }
 let mut pc: u32 = 0x4098f80;
 loop { match pc {
 0x4098f80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4098fc7; continue; }
  pc = 0x4098fa0; continue;
 }
 0x4098fa0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x4098fae; continue; }
  pc = 0x4098fa5; continue;
 }
 0x4098fa5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4098fc7; continue; }
  pc = 0x4098fae; continue;
 }
 0x4098fae => {
  cpu.push(0x4098fb3); f_040a3b61(cpu); if cpu.returned_to != 0x4098fb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4098fc7; continue; }
  pc = 0x4098fb7; continue;
 }
 0x4098fb7 => {
  cpu.push(0x4098fbc); f_0409d2c0(cpu); if cpu.returned_to != 0x4098fbc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098fc7; continue; }
  pc = 0x4098fc0; continue;
 }
 0x4098fc0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4098fc9; continue;
 }
 0x4098fc7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4098fc9; continue;
 }
 0x4098fc9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4098ffb; continue; }
  pc = 0x4098fcd; continue;
 }
 0x4098fcd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a554u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4098ff3); cpu.call(t); if cpu.returned_to != 0x4098ff3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4098ffb); f_0409d690(cpu); if cpu.returned_to != 0x4098ffb { return; }
  pc = 0x4098ffb; continue;
 }
 0x4098ffb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4099008); f_040a7f80(cpu); if cpu.returned_to != 0x4099008 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04099030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4099030); }
 let mut pc: u32 = 0x4099030;
 loop { match pc {
 0x4099030 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099077; continue; }
  pc = 0x4099050; continue;
 }
 0x4099050 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409905e; continue; }
  pc = 0x4099055; continue;
 }
 0x4099055 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099077; continue; }
  pc = 0x409905e; continue;
 }
 0x409905e => {
  cpu.push(0x4099063); f_040a3b61(cpu); if cpu.returned_to != 0x4099063 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099077; continue; }
  pc = 0x4099067; continue;
 }
 0x4099067 => {
  cpu.push(0x409906c); f_0409d2c0(cpu); if cpu.returned_to != 0x409906c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099077; continue; }
  pc = 0x4099070; continue;
 }
 0x4099070 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4099079; continue;
 }
 0x4099077 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4099079; continue;
 }
 0x4099079 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40990c7; continue; }
  pc = 0x409907d; continue;
 }
 0x409907d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a530u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40990bf); cpu.call(t); if cpu.returned_to != 0x40990bf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40990c7); f_0409d690(cpu); if cpu.returned_to != 0x40990c7 { return; }
  pc = 0x40990c7; continue;
 }
 0x40990c7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40990e4); f_040a7fc0(cpu); if cpu.returned_to != 0x40990e4 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x4099132; continue; }
  pc = 0x40990f3; continue;
 }
 0x40990f3 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4099105; continue; }
  pc = 0x40990fc; continue;
 }
 0x40990fc => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099132; continue; }
  pc = 0x4099105; continue;
 }
 0x4099105 => {
  cpu.push(0x409910a); f_040a3b61(cpu); if cpu.returned_to != 0x409910a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099132; continue; }
  pc = 0x409910e; continue;
 }
 0x409910e => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4099128); f_040a3df3(cpu); if cpu.returned_to != 0x4099128 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4099132); f_0409d640(cpu); if cpu.returned_to != 0x4099132 { return; }
  pc = 0x4099132; continue;
 }
 0x4099132 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04099150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4099150); }
 let mut pc: u32 = 0x4099150;
 loop { match pc {
 0x4099150 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099197; continue; }
  pc = 0x4099170; continue;
 }
 0x4099170 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409917e; continue; }
  pc = 0x4099175; continue;
 }
 0x4099175 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099197; continue; }
  pc = 0x409917e; continue;
 }
 0x409917e => {
  cpu.push(0x4099183); f_040a3b61(cpu); if cpu.returned_to != 0x4099183 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099197; continue; }
  pc = 0x4099187; continue;
 }
 0x4099187 => {
  cpu.push(0x409918c); f_0409d2c0(cpu); if cpu.returned_to != 0x409918c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099197; continue; }
  pc = 0x4099190; continue;
 }
 0x4099190 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4099199; continue;
 }
 0x4099197 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4099199; continue;
 }
 0x4099199 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40991cb; continue; }
  pc = 0x409919d; continue;
 }
 0x409919d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a50cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x40991c3); cpu.call(t); if cpu.returned_to != 0x40991c3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40991cb); f_0409d690(cpu); if cpu.returned_to != 0x40991cb { return; }
  pc = 0x40991cb; continue;
 }
 0x40991cb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x40991d8); f_040a8410(cpu); if cpu.returned_to != 0x40991d8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_04099200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4099200); }
 let mut pc: u32 = 0x4099200;
 loop { match pc {
 0x4099200 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099248; continue; }
  pc = 0x4099221; continue;
 }
 0x4099221 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409922f; continue; }
  pc = 0x4099226; continue;
 }
 0x4099226 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099248; continue; }
  pc = 0x409922f; continue;
 }
 0x409922f => {
  cpu.push(0x4099234); f_040a3b61(cpu); if cpu.returned_to != 0x4099234 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099248; continue; }
  pc = 0x4099238; continue;
 }
 0x4099238 => {
  cpu.push(0x409923d); f_0409d2c0(cpu); if cpu.returned_to != 0x409923d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099248; continue; }
  pc = 0x4099241; continue;
 }
 0x4099241 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409924a; continue;
 }
 0x4099248 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409924a; continue;
 }
 0x409924a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409927c; continue; }
  pc = 0x409924e; continue;
 }
 0x409924e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a4e4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099274); cpu.call(t); if cpu.returned_to != 0x4099274 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409927c); f_0409d690(cpu); if cpu.returned_to != 0x409927c { return; }
  pc = 0x409927c; continue;
 }
 0x409927c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4099289); f_040a8430(cpu); if cpu.returned_to != 0x4099289 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40992d7; continue; }
  pc = 0x4099298; continue;
 }
 0x4099298 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x40992aa; continue; }
  pc = 0x40992a1; continue;
 }
 0x40992a1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40992d7; continue; }
  pc = 0x40992aa; continue;
 }
 0x40992aa => {
  cpu.push(0x40992af); f_040a3b61(cpu); if cpu.returned_to != 0x40992af { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40992d7; continue; }
  pc = 0x40992b3; continue;
 }
 0x40992b3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x40992cd); f_040a3df3(cpu); if cpu.returned_to != 0x40992cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40992d7); f_0409d5f0(cpu); if cpu.returned_to != 0x40992d7 { return; }
  pc = 0x40992d7; continue;
 }
 0x40992d7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_04099300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x4099300); }
 let mut pc: u32 = 0x4099300;
 loop { match pc {
 0x4099300 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099347; continue; }
  pc = 0x4099320; continue;
 }
 0x4099320 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409932e; continue; }
  pc = 0x4099325; continue;
 }
 0x4099325 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099347; continue; }
  pc = 0x409932e; continue;
 }
 0x409932e => {
  cpu.push(0x4099333); f_040a3b61(cpu); if cpu.returned_to != 0x4099333 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099347; continue; }
  pc = 0x4099337; continue;
 }
 0x4099337 => {
  cpu.push(0x409933c); f_0409d2c0(cpu); if cpu.returned_to != 0x409933c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099347; continue; }
  pc = 0x4099340; continue;
 }
 0x4099340 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4099349; continue;
 }
 0x4099347 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4099349; continue;
 }
 0x4099349 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409937b; continue; }
  pc = 0x409934d; continue;
 }
 0x409934d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a4c0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099373); cpu.call(t); if cpu.returned_to != 0x4099373 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409937b); f_0409d690(cpu); if cpu.returned_to != 0x409937b { return; }
  pc = 0x409937b; continue;
 }
 0x409937b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4099388); f_040a86a0(cpu); if cpu.returned_to != 0x4099388 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040993b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40993b0); }
 let mut pc: u32 = 0x40993b0;
 loop { match pc {
 0x40993b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40993f8; continue; }
  pc = 0x40993d1; continue;
 }
 0x40993d1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40993df; continue; }
  pc = 0x40993d6; continue;
 }
 0x40993d6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40993f8; continue; }
  pc = 0x40993df; continue;
 }
 0x40993df => {
  cpu.push(0x40993e4); f_040a3b61(cpu); if cpu.returned_to != 0x40993e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40993f8; continue; }
  pc = 0x40993e8; continue;
 }
 0x40993e8 => {
  cpu.push(0x40993ed); f_0409d2c0(cpu); if cpu.returned_to != 0x40993ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40993f8; continue; }
  pc = 0x40993f1; continue;
 }
 0x40993f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40993fa; continue;
 }
 0x40993f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40993fa; continue;
 }
 0x40993fa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409943a; continue; }
  pc = 0x40993fe; continue;
 }
 0x40993fe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a498u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099432); cpu.call(t); if cpu.returned_to != 0x4099432 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409943a); f_0409d690(cpu); if cpu.returned_to != 0x409943a { return; }
  pc = 0x409943a; continue;
 }
 0x409943a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409944f); f_040a8780(cpu); if cpu.returned_to != 0x409944f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409949c; continue; }
  pc = 0x409945d; continue;
 }
 0x409945d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409946f; continue; }
  pc = 0x4099466; continue;
 }
 0x4099466 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409949c; continue; }
  pc = 0x409946f; continue;
 }
 0x409946f => {
  cpu.push(0x4099474); f_040a3b61(cpu); if cpu.returned_to != 0x4099474 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409949c; continue; }
  pc = 0x4099478; continue;
 }
 0x4099478 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4099492); f_040a3df3(cpu); if cpu.returned_to != 0x4099492 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409949c); f_0409d5f0(cpu); if cpu.returned_to != 0x409949c { return; }
  pc = 0x409949c; continue;
 }
 0x409949c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_040994c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40994c0); }
 let mut pc: u32 = 0x40994c0;
 loop { match pc {
 0x40994c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099508; continue; }
  pc = 0x40994e1; continue;
 }
 0x40994e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40994ef; continue; }
  pc = 0x40994e6; continue;
 }
 0x40994e6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099508; continue; }
  pc = 0x40994ef; continue;
 }
 0x40994ef => {
  cpu.push(0x40994f4); f_040a3b61(cpu); if cpu.returned_to != 0x40994f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099508; continue; }
  pc = 0x40994f8; continue;
 }
 0x40994f8 => {
  cpu.push(0x40994fd); f_0409d2c0(cpu); if cpu.returned_to != 0x40994fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099508; continue; }
  pc = 0x4099501; continue;
 }
 0x4099501 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409950a; continue;
 }
 0x4099508 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409950a; continue;
 }
 0x409950a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409953c; continue; }
  pc = 0x409950e; continue;
 }
 0x409950e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a47cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099534); cpu.call(t); if cpu.returned_to != 0x4099534 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409953c); f_0409d690(cpu); if cpu.returned_to != 0x409953c { return; }
  pc = 0x409953c; continue;
 }
 0x409953c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4099549); f_040a86c0(cpu); if cpu.returned_to != 0x4099549 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4099597; continue; }
  pc = 0x4099558; continue;
 }
 0x4099558 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409956a; continue; }
  pc = 0x4099561; continue;
 }
 0x4099561 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099597; continue; }
  pc = 0x409956a; continue;
 }
 0x409956a => {
  cpu.push(0x409956f); f_040a3b61(cpu); if cpu.returned_to != 0x409956f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099597; continue; }
  pc = 0x4099573; continue;
 }
 0x4099573 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409958d); f_040a3df3(cpu); if cpu.returned_to != 0x409958d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4099597); f_0409d640(cpu); if cpu.returned_to != 0x4099597 { return; }
  pc = 0x4099597; continue;
 }
 0x4099597 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_040995c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40995c0); }
 let mut pc: u32 = 0x40995c0;
 loop { match pc {
 0x40995c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099608; continue; }
  pc = 0x40995e1; continue;
 }
 0x40995e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40995ef; continue; }
  pc = 0x40995e6; continue;
 }
 0x40995e6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099608; continue; }
  pc = 0x40995ef; continue;
 }
 0x40995ef => {
  cpu.push(0x40995f4); f_040a3b61(cpu); if cpu.returned_to != 0x40995f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099608; continue; }
  pc = 0x40995f8; continue;
 }
 0x40995f8 => {
  cpu.push(0x40995fd); f_0409d2c0(cpu); if cpu.returned_to != 0x40995fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099608; continue; }
  pc = 0x4099601; continue;
 }
 0x4099601 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409960a; continue;
 }
 0x4099608 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409960a; continue;
 }
 0x409960a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409963c; continue; }
  pc = 0x409960e; continue;
 }
 0x409960e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a45cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099634); cpu.call(t); if cpu.returned_to != 0x4099634 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409963c); f_0409d690(cpu); if cpu.returned_to != 0x409963c { return; }
  pc = 0x409963c; continue;
 }
 0x409963c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x4099649); f_040a5480(cpu); if cpu.returned_to != 0x4099649 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x4099697; continue; }
  pc = 0x4099658; continue;
 }
 0x4099658 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409966a; continue; }
  pc = 0x4099661; continue;
 }
 0x4099661 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099697; continue; }
  pc = 0x409966a; continue;
 }
 0x409966a => {
  cpu.push(0x409966f); f_040a3b61(cpu); if cpu.returned_to != 0x409966f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099697; continue; }
  pc = 0x4099673; continue;
 }
 0x4099673 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409968d); f_040a3df3(cpu); if cpu.returned_to != 0x409968d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4099697); f_0409d640(cpu); if cpu.returned_to != 0x4099697 { return; }
  pc = 0x4099697; continue;
 }
 0x4099697 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_040996c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40996c0); }
 let mut pc: u32 = 0x40996c0;
 loop { match pc {
 0x40996c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099708; continue; }
  pc = 0x40996e1; continue;
 }
 0x40996e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40996ef; continue; }
  pc = 0x40996e6; continue;
 }
 0x40996e6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099708; continue; }
  pc = 0x40996ef; continue;
 }
 0x40996ef => {
  cpu.push(0x40996f4); f_040a3b61(cpu); if cpu.returned_to != 0x40996f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099708; continue; }
  pc = 0x40996f8; continue;
 }
 0x40996f8 => {
  cpu.push(0x40996fd); f_0409d2c0(cpu); if cpu.returned_to != 0x40996fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099708; continue; }
  pc = 0x4099701; continue;
 }
 0x4099701 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409970a; continue;
 }
 0x4099708 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409970a; continue;
 }
 0x409970a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409974a; continue; }
  pc = 0x409970e; continue;
 }
 0x409970e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a43cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099742); cpu.call(t); if cpu.returned_to != 0x4099742 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409974a); f_0409d690(cpu); if cpu.returned_to != 0x409974a { return; }
  pc = 0x409974a; continue;
 }
 0x409974a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409975f); f_040a4f80(cpu); if cpu.returned_to != 0x409975f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40997ac; continue; }
  pc = 0x409976d; continue;
 }
 0x409976d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409977f; continue; }
  pc = 0x4099776; continue;
 }
 0x4099776 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40997ac; continue; }
  pc = 0x409977f; continue;
 }
 0x409977f => {
  cpu.push(0x4099784); f_040a3b61(cpu); if cpu.returned_to != 0x4099784 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40997ac; continue; }
  pc = 0x4099788; continue;
 }
 0x4099788 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x40997a2); f_040a3df3(cpu); if cpu.returned_to != 0x40997a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40997ac); f_0409d640(cpu); if cpu.returned_to != 0x40997ac { return; }
  pc = 0x40997ac; continue;
 }
 0x40997ac => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_040997d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40997d0); }
 let mut pc: u32 = 0x40997d0;
 loop { match pc {
 0x40997d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x4099817; continue; }
  pc = 0x40997f0; continue;
 }
 0x40997f0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40997fe; continue; }
  pc = 0x40997f5; continue;
 }
 0x40997f5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099817; continue; }
  pc = 0x40997fe; continue;
 }
 0x40997fe => {
  cpu.push(0x4099803); f_040a3b61(cpu); if cpu.returned_to != 0x4099803 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099817; continue; }
  pc = 0x4099807; continue;
 }
 0x4099807 => {
  cpu.push(0x409980c); f_0409d2c0(cpu); if cpu.returned_to != 0x409980c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099817; continue; }
  pc = 0x4099810; continue;
 }
 0x4099810 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x4099819; continue;
 }
 0x4099817 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x4099819; continue;
 }
 0x4099819 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099867; continue; }
  pc = 0x409981d; continue;
 }
 0x409981d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a414u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409985f); cpu.call(t); if cpu.returned_to != 0x409985f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4099867); f_0409d690(cpu); if cpu.returned_to != 0x4099867 { return; }
  pc = 0x4099867; continue;
 }
 0x4099867 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4099884); f_040a4fa0(cpu); if cpu.returned_to != 0x4099884 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040999b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40999b0); }
 let mut pc: u32 = 0x40999b0;
 loop { match pc {
 0x40999b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40999f8; continue; }
  pc = 0x40999d1; continue;
 }
 0x40999d1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x40999df; continue; }
  pc = 0x40999d6; continue;
 }
 0x40999d6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x40999f8; continue; }
  pc = 0x40999df; continue;
 }
 0x40999df => {
  cpu.push(0x40999e4); f_040a3b61(cpu); if cpu.returned_to != 0x40999e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40999f8; continue; }
  pc = 0x40999e8; continue;
 }
 0x40999e8 => {
  cpu.push(0x40999ed); f_0409d2c0(cpu); if cpu.returned_to != 0x40999ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40999f8; continue; }
  pc = 0x40999f1; continue;
 }
 0x40999f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40999fa; continue;
 }
 0x40999f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40999fa; continue;
 }
 0x40999fa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099a3a; continue; }
  pc = 0x40999fe; continue;
 }
 0x40999fe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a3c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x4099a32); cpu.call(t); if cpu.returned_to != 0x4099a32 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x4099a3a); f_0409d690(cpu); if cpu.returned_to != 0x4099a3a { return; }
  pc = 0x4099a3a; continue;
 }
 0x4099a3a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x4099a4f); f_040a4fc0(cpu); if cpu.returned_to != 0x4099a4f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x4099a9c; continue; }
  pc = 0x4099a5d; continue;
 }
 0x4099a5d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x4099a6f; continue; }
  pc = 0x4099a66; continue;
 }
 0x4099a66 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x4099a9c; continue; }
  pc = 0x4099a6f; continue;
 }
 0x4099a6f => {
  cpu.push(0x4099a74); f_040a3b61(cpu); if cpu.returned_to != 0x4099a74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x4099a9c; continue; }
  pc = 0x4099a78; continue;
 }
 0x4099a78 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x4099a92); f_040a3df3(cpu); if cpu.returned_to != 0x4099a92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x4099a9c); f_0409d5f0(cpu); if cpu.returned_to != 0x4099a9c { return; }
  pc = 0x4099a9c; continue;
 }
 0x4099a9c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_0409a670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409a670); }
 let mut pc: u32 = 0x409a670;
 loop { match pc {
 0x409a670 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409a6b7; continue; }
  pc = 0x409a690; continue;
 }
 0x409a690 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409a69e; continue; }
  pc = 0x409a695; continue;
 }
 0x409a695 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409a6b7; continue; }
  pc = 0x409a69e; continue;
 }
 0x409a69e => {
  cpu.push(0x409a6a3); f_040a3b61(cpu); if cpu.returned_to != 0x409a6a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409a6b7; continue; }
  pc = 0x409a6a7; continue;
 }
 0x409a6a7 => {
  cpu.push(0x409a6ac); f_0409d2c0(cpu); if cpu.returned_to != 0x409a6ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409a6b7; continue; }
  pc = 0x409a6b0; continue;
 }
 0x409a6b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409a6b9; continue;
 }
 0x409a6b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409a6b9; continue;
 }
 0x409a6b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409a6f9; continue; }
  pc = 0x409a6bd; continue;
 }
 0x409a6bd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a1b4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409a6f1); cpu.call(t); if cpu.returned_to != 0x409a6f1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409a6f9); f_0409d690(cpu); if cpu.returned_to != 0x409a6f9 { return; }
  pc = 0x409a6f9; continue;
 }
 0x409a6f9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409a70e); f_040a4f40(cpu); if cpu.returned_to != 0x409a70e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409a730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409a730); }
 let mut pc: u32 = 0x409a730;
 loop { match pc {
 0x409a730 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409a778; continue; }
  pc = 0x409a751; continue;
 }
 0x409a751 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409a75f; continue; }
  pc = 0x409a756; continue;
 }
 0x409a756 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409a778; continue; }
  pc = 0x409a75f; continue;
 }
 0x409a75f => {
  cpu.push(0x409a764); f_040a3b61(cpu); if cpu.returned_to != 0x409a764 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409a778; continue; }
  pc = 0x409a768; continue;
 }
 0x409a768 => {
  cpu.push(0x409a76d); f_0409d2c0(cpu); if cpu.returned_to != 0x409a76d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409a778; continue; }
  pc = 0x409a771; continue;
 }
 0x409a771 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409a77a; continue;
 }
 0x409a778 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409a77a; continue;
 }
 0x409a77a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409a7ac; continue; }
  pc = 0x409a77e; continue;
 }
 0x409a77e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a194u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409a7a4); cpu.call(t); if cpu.returned_to != 0x409a7a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409a7ac); f_0409d690(cpu); if cpu.returned_to != 0x409a7ac { return; }
  pc = 0x409a7ac; continue;
 }
 0x409a7ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409a7b9); f_040a51b0(cpu); if cpu.returned_to != 0x409a7b9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409a807; continue; }
  pc = 0x409a7c8; continue;
 }
 0x409a7c8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409a7da; continue; }
  pc = 0x409a7d1; continue;
 }
 0x409a7d1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409a807; continue; }
  pc = 0x409a7da; continue;
 }
 0x409a7da => {
  cpu.push(0x409a7df); f_040a3b61(cpu); if cpu.returned_to != 0x409a7df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409a807; continue; }
  pc = 0x409a7e3; continue;
 }
 0x409a7e3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409a7fd); f_040a3df3(cpu); if cpu.returned_to != 0x409a7fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409a807); f_0409d640(cpu); if cpu.returned_to != 0x409a807 { return; }
  pc = 0x409a807; continue;
 }
 0x409a807 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_0409ab30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ab30); }
 let mut pc: u32 = 0x409ab30;
 loop { match pc {
 0x409ab30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409ab78; continue; }
  pc = 0x409ab51; continue;
 }
 0x409ab51 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409ab5f; continue; }
  pc = 0x409ab56; continue;
 }
 0x409ab56 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ab78; continue; }
  pc = 0x409ab5f; continue;
 }
 0x409ab5f => {
  cpu.push(0x409ab64); f_040a3b61(cpu); if cpu.returned_to != 0x409ab64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ab78; continue; }
  pc = 0x409ab68; continue;
 }
 0x409ab68 => {
  cpu.push(0x409ab6d); f_0409d2c0(cpu); if cpu.returned_to != 0x409ab6d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ab78; continue; }
  pc = 0x409ab71; continue;
 }
 0x409ab71 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409ab7a; continue;
 }
 0x409ab78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409ab7a; continue;
 }
 0x409ab7a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409abac; continue; }
  pc = 0x409ab7e; continue;
 }
 0x409ab7e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a120u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409aba4); cpu.call(t); if cpu.returned_to != 0x409aba4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409abac); f_0409d690(cpu); if cpu.returned_to != 0x409abac { return; }
  pc = 0x409abac; continue;
 }
 0x409abac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409abb9); f_040a52f0(cpu); if cpu.returned_to != 0x409abb9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409ac07; continue; }
  pc = 0x409abc8; continue;
 }
 0x409abc8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409abda; continue; }
  pc = 0x409abd1; continue;
 }
 0x409abd1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ac07; continue; }
  pc = 0x409abda; continue;
 }
 0x409abda => {
  cpu.push(0x409abdf); f_040a3b61(cpu); if cpu.returned_to != 0x409abdf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ac07; continue; }
  pc = 0x409abe3; continue;
 }
 0x409abe3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409abfd); f_040a3df3(cpu); if cpu.returned_to != 0x409abfd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409ac07); f_0409d640(cpu); if cpu.returned_to != 0x409ac07 { return; }
  pc = 0x409ac07; continue;
 }
 0x409ac07 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409ac30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ac30); }
 let mut pc: u32 = 0x409ac30;
 loop { match pc {
 0x409ac30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409ac77; continue; }
  pc = 0x409ac50; continue;
 }
 0x409ac50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409ac5e; continue; }
  pc = 0x409ac55; continue;
 }
 0x409ac55 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ac77; continue; }
  pc = 0x409ac5e; continue;
 }
 0x409ac5e => {
  cpu.push(0x409ac63); f_040a3b61(cpu); if cpu.returned_to != 0x409ac63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ac77; continue; }
  pc = 0x409ac67; continue;
 }
 0x409ac67 => {
  cpu.push(0x409ac6c); f_0409d2c0(cpu); if cpu.returned_to != 0x409ac6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ac77; continue; }
  pc = 0x409ac70; continue;
 }
 0x409ac70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409ac79; continue;
 }
 0x409ac77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409ac79; continue;
 }
 0x409ac79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409acc7; continue; }
  pc = 0x409ac7d; continue;
 }
 0x409ac7d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a0f4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409acbf); cpu.call(t); if cpu.returned_to != 0x409acbf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409acc7); f_0409d690(cpu); if cpu.returned_to != 0x409acc7 { return; }
  pc = 0x409acc7; continue;
 }
 0x409acc7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ace4); f_040a5180(cpu); if cpu.returned_to != 0x409ace4 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409ad00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ad00); }
 let mut pc: u32 = 0x409ad00;
 loop { match pc {
 0x409ad00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409ad47; continue; }
  pc = 0x409ad20; continue;
 }
 0x409ad20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409ad2e; continue; }
  pc = 0x409ad25; continue;
 }
 0x409ad25 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ad47; continue; }
  pc = 0x409ad2e; continue;
 }
 0x409ad2e => {
  cpu.push(0x409ad33); f_040a3b61(cpu); if cpu.returned_to != 0x409ad33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ad47; continue; }
  pc = 0x409ad37; continue;
 }
 0x409ad37 => {
  cpu.push(0x409ad3c); f_0409d2c0(cpu); if cpu.returned_to != 0x409ad3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ad47; continue; }
  pc = 0x409ad40; continue;
 }
 0x409ad40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409ad49; continue;
 }
 0x409ad47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409ad49; continue;
 }
 0x409ad49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ad89; continue; }
  pc = 0x409ad4d; continue;
 }
 0x409ad4d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a0ccu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409ad81); cpu.call(t); if cpu.returned_to != 0x409ad81 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ad89); f_0409d690(cpu); if cpu.returned_to != 0x409ad89 { return; }
  pc = 0x409ad89; continue;
 }
 0x409ad89 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409ad9e); f_040a51a0(cpu); if cpu.returned_to != 0x409ad9e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409adc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409adc0); }
 let mut pc: u32 = 0x409adc0;
 loop { match pc {
 0x409adc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409ae07; continue; }
  pc = 0x409ade0; continue;
 }
 0x409ade0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409adee; continue; }
  pc = 0x409ade5; continue;
 }
 0x409ade5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ae07; continue; }
  pc = 0x409adee; continue;
 }
 0x409adee => {
  cpu.push(0x409adf3); f_040a3b61(cpu); if cpu.returned_to != 0x409adf3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ae07; continue; }
  pc = 0x409adf7; continue;
 }
 0x409adf7 => {
  cpu.push(0x409adfc); f_0409d2c0(cpu); if cpu.returned_to != 0x409adfc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ae07; continue; }
  pc = 0x409ae00; continue;
 }
 0x409ae00 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409ae09; continue;
 }
 0x409ae07 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409ae09; continue;
 }
 0x409ae09 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ae49; continue; }
  pc = 0x409ae0d; continue;
 }
 0x409ae0d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a0acu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409ae41); cpu.call(t); if cpu.returned_to != 0x409ae41 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ae49); f_0409d690(cpu); if cpu.returned_to != 0x409ae49 { return; }
  pc = 0x409ae49; continue;
 }
 0x409ae49 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409ae5e); f_040a5210(cpu); if cpu.returned_to != 0x409ae5e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409ae80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ae80); }
 let mut pc: u32 = 0x409ae80;
 loop { match pc {
 0x409ae80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409aec7; continue; }
  pc = 0x409aea0; continue;
 }
 0x409aea0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409aeae; continue; }
  pc = 0x409aea5; continue;
 }
 0x409aea5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409aec7; continue; }
  pc = 0x409aeae; continue;
 }
 0x409aeae => {
  cpu.push(0x409aeb3); f_040a3b61(cpu); if cpu.returned_to != 0x409aeb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409aec7; continue; }
  pc = 0x409aeb7; continue;
 }
 0x409aeb7 => {
  cpu.push(0x409aebc); f_0409d2c0(cpu); if cpu.returned_to != 0x409aebc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409aec7; continue; }
  pc = 0x409aec0; continue;
 }
 0x409aec0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409aec9; continue;
 }
 0x409aec7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409aec9; continue;
 }
 0x409aec9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409af09; continue; }
  pc = 0x409aecd; continue;
 }
 0x409aecd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a088u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409af01); cpu.call(t); if cpu.returned_to != 0x409af01 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409af09); f_0409d690(cpu); if cpu.returned_to != 0x409af09 { return; }
  pc = 0x409af09; continue;
 }
 0x409af09 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409af1e); f_040a5240(cpu); if cpu.returned_to != 0x409af1e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409af40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409af40); }
 let mut pc: u32 = 0x409af40;
 loop { match pc {
 0x409af40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409af87; continue; }
  pc = 0x409af60; continue;
 }
 0x409af60 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409af6e; continue; }
  pc = 0x409af65; continue;
 }
 0x409af65 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409af87; continue; }
  pc = 0x409af6e; continue;
 }
 0x409af6e => {
  cpu.push(0x409af73); f_040a3b61(cpu); if cpu.returned_to != 0x409af73 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409af87; continue; }
  pc = 0x409af77; continue;
 }
 0x409af77 => {
  cpu.push(0x409af7c); f_0409d2c0(cpu); if cpu.returned_to != 0x409af7c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409af87; continue; }
  pc = 0x409af80; continue;
 }
 0x409af80 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409af89; continue;
 }
 0x409af87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409af89; continue;
 }
 0x409af89 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409afc9; continue; }
  pc = 0x409af8d; continue;
 }
 0x409af8d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a060u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409afc1); cpu.call(t); if cpu.returned_to != 0x409afc1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409afc9); f_0409d690(cpu); if cpu.returned_to != 0x409afc9 { return; }
  pc = 0x409afc9; continue;
 }
 0x409afc9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409afde); f_040a5270(cpu); if cpu.returned_to != 0x409afde { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b000); }
 let mut pc: u32 = 0x409b000;
 loop { match pc {
 0x409b000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b047; continue; }
  pc = 0x409b020; continue;
 }
 0x409b020 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b02e; continue; }
  pc = 0x409b025; continue;
 }
 0x409b025 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b047; continue; }
  pc = 0x409b02e; continue;
 }
 0x409b02e => {
  cpu.push(0x409b033); f_040a3b61(cpu); if cpu.returned_to != 0x409b033 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b047; continue; }
  pc = 0x409b037; continue;
 }
 0x409b037 => {
  cpu.push(0x409b03c); f_0409d2c0(cpu); if cpu.returned_to != 0x409b03c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b047; continue; }
  pc = 0x409b040; continue;
 }
 0x409b040 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b049; continue;
 }
 0x409b047 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b049; continue;
 }
 0x409b049 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b07b; continue; }
  pc = 0x409b04d; continue;
 }
 0x409b04d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a048u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b073); cpu.call(t); if cpu.returned_to != 0x409b073 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b07b); f_0409d690(cpu); if cpu.returned_to != 0x409b07b { return; }
  pc = 0x409b07b; continue;
 }
 0x409b07b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b088); f_040a5020(cpu); if cpu.returned_to != 0x409b088 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409b0b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b0b0); }
 let mut pc: u32 = 0x409b0b0;
 loop { match pc {
 0x409b0b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b0f7; continue; }
  pc = 0x409b0d0; continue;
 }
 0x409b0d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b0de; continue; }
  pc = 0x409b0d5; continue;
 }
 0x409b0d5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b0f7; continue; }
  pc = 0x409b0de; continue;
 }
 0x409b0de => {
  cpu.push(0x409b0e3); f_040a3b61(cpu); if cpu.returned_to != 0x409b0e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b0f7; continue; }
  pc = 0x409b0e7; continue;
 }
 0x409b0e7 => {
  cpu.push(0x409b0ec); f_0409d2c0(cpu); if cpu.returned_to != 0x409b0ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b0f7; continue; }
  pc = 0x409b0f0; continue;
 }
 0x409b0f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b0f9; continue;
 }
 0x409b0f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b0f9; continue;
 }
 0x409b0f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b12b; continue; }
  pc = 0x409b0fd; continue;
 }
 0x409b0fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x414a02cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b123); cpu.call(t); if cpu.returned_to != 0x409b123 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b12b); f_0409d690(cpu); if cpu.returned_to != 0x409b12b { return; }
  pc = 0x409b12b; continue;
 }
 0x409b12b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b138); f_040a5070(cpu); if cpu.returned_to != 0x409b138 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b210); }
 let mut pc: u32 = 0x409b210;
 loop { match pc {
 0x409b210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b257; continue; }
  pc = 0x409b230; continue;
 }
 0x409b230 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b23e; continue; }
  pc = 0x409b235; continue;
 }
 0x409b235 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b257; continue; }
  pc = 0x409b23e; continue;
 }
 0x409b23e => {
  cpu.push(0x409b243); f_040a3b61(cpu); if cpu.returned_to != 0x409b243 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b257; continue; }
  pc = 0x409b247; continue;
 }
 0x409b247 => {
  cpu.push(0x409b24c); f_0409d2c0(cpu); if cpu.returned_to != 0x409b24c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b257; continue; }
  pc = 0x409b250; continue;
 }
 0x409b250 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b259; continue;
 }
 0x409b257 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b259; continue;
 }
 0x409b259 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b28b; continue; }
  pc = 0x409b25d; continue;
 }
 0x409b25d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149ff8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b283); cpu.call(t); if cpu.returned_to != 0x409b283 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b28b); f_0409d690(cpu); if cpu.returned_to != 0x409b28b { return; }
  pc = 0x409b28b; continue;
 }
 0x409b28b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b298); f_040a50f0(cpu); if cpu.returned_to != 0x409b298 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409b2c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b2c0); }
 let mut pc: u32 = 0x409b2c0;
 loop { match pc {
 0x409b2c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b307; continue; }
  pc = 0x409b2e0; continue;
 }
 0x409b2e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b2ee; continue; }
  pc = 0x409b2e5; continue;
 }
 0x409b2e5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b307; continue; }
  pc = 0x409b2ee; continue;
 }
 0x409b2ee => {
  cpu.push(0x409b2f3); f_040a3b61(cpu); if cpu.returned_to != 0x409b2f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b307; continue; }
  pc = 0x409b2f7; continue;
 }
 0x409b2f7 => {
  cpu.push(0x409b2fc); f_0409d2c0(cpu); if cpu.returned_to != 0x409b2fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b307; continue; }
  pc = 0x409b300; continue;
 }
 0x409b300 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b309; continue;
 }
 0x409b307 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b309; continue;
 }
 0x409b309 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b357; continue; }
  pc = 0x409b30d; continue;
 }
 0x409b30d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149fd4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b34f); cpu.call(t); if cpu.returned_to != 0x409b34f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b357); f_0409d690(cpu); if cpu.returned_to != 0x409b357 { return; }
  pc = 0x409b357; continue;
 }
 0x409b357 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b374); f_040a5290(cpu); if cpu.returned_to != 0x409b374 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b390); }
 let mut pc: u32 = 0x409b390;
 loop { match pc {
 0x409b390 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b3d7; continue; }
  pc = 0x409b3b0; continue;
 }
 0x409b3b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b3be; continue; }
  pc = 0x409b3b5; continue;
 }
 0x409b3b5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b3d7; continue; }
  pc = 0x409b3be; continue;
 }
 0x409b3be => {
  cpu.push(0x409b3c3); f_040a3b61(cpu); if cpu.returned_to != 0x409b3c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b3d7; continue; }
  pc = 0x409b3c7; continue;
 }
 0x409b3c7 => {
  cpu.push(0x409b3cc); f_0409d2c0(cpu); if cpu.returned_to != 0x409b3cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b3d7; continue; }
  pc = 0x409b3d0; continue;
 }
 0x409b3d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b3d9; continue;
 }
 0x409b3d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b3d9; continue;
 }
 0x409b3d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b427; continue; }
  pc = 0x409b3dd; continue;
 }
 0x409b3dd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149facu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b41f); cpu.call(t); if cpu.returned_to != 0x409b41f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b427); f_0409d690(cpu); if cpu.returned_to != 0x409b427 { return; }
  pc = 0x409b427; continue;
 }
 0x409b427 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b444); f_040a52c0(cpu); if cpu.returned_to != 0x409b444 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b460); }
 let mut pc: u32 = 0x409b460;
 loop { match pc {
 0x409b460 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b4a7; continue; }
  pc = 0x409b480; continue;
 }
 0x409b480 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b48e; continue; }
  pc = 0x409b485; continue;
 }
 0x409b485 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b4a7; continue; }
  pc = 0x409b48e; continue;
 }
 0x409b48e => {
  cpu.push(0x409b493); f_040a3b61(cpu); if cpu.returned_to != 0x409b493 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b4a7; continue; }
  pc = 0x409b497; continue;
 }
 0x409b497 => {
  cpu.push(0x409b49c); f_0409d2c0(cpu); if cpu.returned_to != 0x409b49c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b4a7; continue; }
  pc = 0x409b4a0; continue;
 }
 0x409b4a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b4a9; continue;
 }
 0x409b4a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b4a9; continue;
 }
 0x409b4a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b4f7; continue; }
  pc = 0x409b4ad; continue;
 }
 0x409b4ad => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149f88u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b4ef); cpu.call(t); if cpu.returned_to != 0x409b4ef { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b4f7); f_0409d690(cpu); if cpu.returned_to != 0x409b4f7 { return; }
  pc = 0x409b4f7; continue;
 }
 0x409b4f7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b514); f_040a6ab0(cpu); if cpu.returned_to != 0x409b514 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409b562; continue; }
  pc = 0x409b523; continue;
 }
 0x409b523 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409b535; continue; }
  pc = 0x409b52c; continue;
 }
 0x409b52c => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b562; continue; }
  pc = 0x409b535; continue;
 }
 0x409b535 => {
  cpu.push(0x409b53a); f_040a3b61(cpu); if cpu.returned_to != 0x409b53a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b562; continue; }
  pc = 0x409b53e; continue;
 }
 0x409b53e => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b558); f_040a3df3(cpu); if cpu.returned_to != 0x409b558 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409b562); f_0409d640(cpu); if cpu.returned_to != 0x409b562 { return; }
  pc = 0x409b562; continue;
 }
 0x409b562 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b580); }
 let mut pc: u32 = 0x409b580;
 loop { match pc {
 0x409b580 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b5c7; continue; }
  pc = 0x409b5a0; continue;
 }
 0x409b5a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b5ae; continue; }
  pc = 0x409b5a5; continue;
 }
 0x409b5a5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b5c7; continue; }
  pc = 0x409b5ae; continue;
 }
 0x409b5ae => {
  cpu.push(0x409b5b3); f_040a3b61(cpu); if cpu.returned_to != 0x409b5b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b5c7; continue; }
  pc = 0x409b5b7; continue;
 }
 0x409b5b7 => {
  cpu.push(0x409b5bc); f_0409d2c0(cpu); if cpu.returned_to != 0x409b5bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b5c7; continue; }
  pc = 0x409b5c0; continue;
 }
 0x409b5c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b5c9; continue;
 }
 0x409b5c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b5c9; continue;
 }
 0x409b5c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b5fb; continue; }
  pc = 0x409b5cd; continue;
 }
 0x409b5cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149f70u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b5f3); cpu.call(t); if cpu.returned_to != 0x409b5f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b5fb); f_0409d690(cpu); if cpu.returned_to != 0x409b5fb { return; }
  pc = 0x409b5fb; continue;
 }
 0x409b5fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b608); f_040a5300(cpu); if cpu.returned_to != 0x409b608 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b630); }
 let mut pc: u32 = 0x409b630;
 loop { match pc {
 0x409b630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b677; continue; }
  pc = 0x409b650; continue;
 }
 0x409b650 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b65e; continue; }
  pc = 0x409b655; continue;
 }
 0x409b655 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b677; continue; }
  pc = 0x409b65e; continue;
 }
 0x409b65e => {
  cpu.push(0x409b663); f_040a3b61(cpu); if cpu.returned_to != 0x409b663 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b677; continue; }
  pc = 0x409b667; continue;
 }
 0x409b667 => {
  cpu.push(0x409b66c); f_0409d2c0(cpu); if cpu.returned_to != 0x409b66c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b677; continue; }
  pc = 0x409b670; continue;
 }
 0x409b670 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b679; continue;
 }
 0x409b677 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b679; continue;
 }
 0x409b679 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b6ab; continue; }
  pc = 0x409b67d; continue;
 }
 0x409b67d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149f4cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b6a3); cpu.call(t); if cpu.returned_to != 0x409b6a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b6ab); f_0409d690(cpu); if cpu.returned_to != 0x409b6ab { return; }
  pc = 0x409b6ab; continue;
 }
 0x409b6ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b6b8); f_040a53f0(cpu); if cpu.returned_to != 0x409b6b8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409b6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b6e0); }
 let mut pc: u32 = 0x409b6e0;
 loop { match pc {
 0x409b6e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b727; continue; }
  pc = 0x409b700; continue;
 }
 0x409b700 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b70e; continue; }
  pc = 0x409b705; continue;
 }
 0x409b705 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b727; continue; }
  pc = 0x409b70e; continue;
 }
 0x409b70e => {
  cpu.push(0x409b713); f_040a3b61(cpu); if cpu.returned_to != 0x409b713 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b727; continue; }
  pc = 0x409b717; continue;
 }
 0x409b717 => {
  cpu.push(0x409b71c); f_0409d2c0(cpu); if cpu.returned_to != 0x409b71c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b727; continue; }
  pc = 0x409b720; continue;
 }
 0x409b720 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b729; continue;
 }
 0x409b727 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b729; continue;
 }
 0x409b729 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b777; continue; }
  pc = 0x409b72d; continue;
 }
 0x409b72d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149f20u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b76f); cpu.call(t); if cpu.returned_to != 0x409b76f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b777); f_0409d690(cpu); if cpu.returned_to != 0x409b777 { return; }
  pc = 0x409b777; continue;
 }
 0x409b777 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b794); f_040a6c00(cpu); if cpu.returned_to != 0x409b794 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409b7e2; continue; }
  pc = 0x409b7a3; continue;
 }
 0x409b7a3 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409b7b5; continue; }
  pc = 0x409b7ac; continue;
 }
 0x409b7ac => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b7e2; continue; }
  pc = 0x409b7b5; continue;
 }
 0x409b7b5 => {
  cpu.push(0x409b7ba); f_040a3b61(cpu); if cpu.returned_to != 0x409b7ba { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b7e2; continue; }
  pc = 0x409b7be; continue;
 }
 0x409b7be => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b7d8); f_040a3df3(cpu); if cpu.returned_to != 0x409b7d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409b7e2); f_0409d5f0(cpu); if cpu.returned_to != 0x409b7e2 { return; }
  pc = 0x409b7e2; continue;
 }
 0x409b7e2 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_0409b800(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b800); }
 let mut pc: u32 = 0x409b800;
 loop { match pc {
 0x409b800 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b848; continue; }
  pc = 0x409b821; continue;
 }
 0x409b821 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b82f; continue; }
  pc = 0x409b826; continue;
 }
 0x409b826 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b848; continue; }
  pc = 0x409b82f; continue;
 }
 0x409b82f => {
  cpu.push(0x409b834); f_040a3b61(cpu); if cpu.returned_to != 0x409b834 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b848; continue; }
  pc = 0x409b838; continue;
 }
 0x409b838 => {
  cpu.push(0x409b83d); f_0409d2c0(cpu); if cpu.returned_to != 0x409b83d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b848; continue; }
  pc = 0x409b841; continue;
 }
 0x409b841 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b84a; continue;
 }
 0x409b848 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b84a; continue;
 }
 0x409b84a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b87c; continue; }
  pc = 0x409b84e; continue;
 }
 0x409b84e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149efcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b874); cpu.call(t); if cpu.returned_to != 0x409b874 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b87c); f_0409d690(cpu); if cpu.returned_to != 0x409b87c { return; }
  pc = 0x409b87c; continue;
 }
 0x409b87c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b889); f_040a5400(cpu); if cpu.returned_to != 0x409b889 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409b8d7; continue; }
  pc = 0x409b898; continue;
 }
 0x409b898 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409b8aa; continue; }
  pc = 0x409b8a1; continue;
 }
 0x409b8a1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b8d7; continue; }
  pc = 0x409b8aa; continue;
 }
 0x409b8aa => {
  cpu.push(0x409b8af); f_040a3b61(cpu); if cpu.returned_to != 0x409b8af { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b8d7; continue; }
  pc = 0x409b8b3; continue;
 }
 0x409b8b3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409b8cd); f_040a3df3(cpu); if cpu.returned_to != 0x409b8cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409b8d7); f_0409d5f0(cpu); if cpu.returned_to != 0x409b8d7 { return; }
  pc = 0x409b8d7; continue;
 }
 0x409b8d7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_0409b900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b900); }
 let mut pc: u32 = 0x409b900;
 loop { match pc {
 0x409b900 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b947; continue; }
  pc = 0x409b920; continue;
 }
 0x409b920 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b92e; continue; }
  pc = 0x409b925; continue;
 }
 0x409b925 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b947; continue; }
  pc = 0x409b92e; continue;
 }
 0x409b92e => {
  cpu.push(0x409b933); f_040a3b61(cpu); if cpu.returned_to != 0x409b933 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b947; continue; }
  pc = 0x409b937; continue;
 }
 0x409b937 => {
  cpu.push(0x409b93c); f_0409d2c0(cpu); if cpu.returned_to != 0x409b93c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b947; continue; }
  pc = 0x409b940; continue;
 }
 0x409b940 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b949; continue;
 }
 0x409b947 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b949; continue;
 }
 0x409b949 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b97b; continue; }
  pc = 0x409b94d; continue;
 }
 0x409b94d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149ed8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409b973); cpu.call(t); if cpu.returned_to != 0x409b973 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409b97b); f_0409d690(cpu); if cpu.returned_to != 0x409b97b { return; }
  pc = 0x409b97b; continue;
 }
 0x409b97b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409b988); f_040a54b0(cpu); if cpu.returned_to != 0x409b988 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409b9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409b9b0); }
 let mut pc: u32 = 0x409b9b0;
 loop { match pc {
 0x409b9b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409b9f8; continue; }
  pc = 0x409b9d1; continue;
 }
 0x409b9d1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409b9df; continue; }
  pc = 0x409b9d6; continue;
 }
 0x409b9d6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409b9f8; continue; }
  pc = 0x409b9df; continue;
 }
 0x409b9df => {
  cpu.push(0x409b9e4); f_040a3b61(cpu); if cpu.returned_to != 0x409b9e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409b9f8; continue; }
  pc = 0x409b9e8; continue;
 }
 0x409b9e8 => {
  cpu.push(0x409b9ed); f_0409d2c0(cpu); if cpu.returned_to != 0x409b9ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409b9f8; continue; }
  pc = 0x409b9f1; continue;
 }
 0x409b9f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409b9fa; continue;
 }
 0x409b9f8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409b9fa; continue;
 }
 0x409b9fa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ba3a; continue; }
  pc = 0x409b9fe; continue;
 }
 0x409b9fe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149eb0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409ba32); cpu.call(t); if cpu.returned_to != 0x409ba32 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ba3a); f_0409d690(cpu); if cpu.returned_to != 0x409ba3a { return; }
  pc = 0x409ba3a; continue;
 }
 0x409ba3a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409ba4f); f_040a5590(cpu); if cpu.returned_to != 0x409ba4f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ba9c; continue; }
  pc = 0x409ba5d; continue;
 }
 0x409ba5d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409ba6f; continue; }
  pc = 0x409ba66; continue;
 }
 0x409ba66 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ba9c; continue; }
  pc = 0x409ba6f; continue;
 }
 0x409ba6f => {
  cpu.push(0x409ba74); f_040a3b61(cpu); if cpu.returned_to != 0x409ba74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ba9c; continue; }
  pc = 0x409ba78; continue;
 }
 0x409ba78 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409ba92); f_040a3df3(cpu); if cpu.returned_to != 0x409ba92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409ba9c); f_0409d5f0(cpu); if cpu.returned_to != 0x409ba9c { return; }
  pc = 0x409ba9c; continue;
 }
 0x409ba9c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn o_0409bac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409bac0); }
 let mut pc: u32 = 0x409bac0;
 loop { match pc {
 0x409bac0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bb08; continue; }
  pc = 0x409bae1; continue;
 }
 0x409bae1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409baef; continue; }
  pc = 0x409bae6; continue;
 }
 0x409bae6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bb08; continue; }
  pc = 0x409baef; continue;
 }
 0x409baef => {
  cpu.push(0x409baf4); f_040a3b61(cpu); if cpu.returned_to != 0x409baf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bb08; continue; }
  pc = 0x409baf8; continue;
 }
 0x409baf8 => {
  cpu.push(0x409bafd); f_0409d2c0(cpu); if cpu.returned_to != 0x409bafd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bb08; continue; }
  pc = 0x409bb01; continue;
 }
 0x409bb01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bb0a; continue;
 }
 0x409bb08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bb0a; continue;
 }
 0x409bb0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bb3c; continue; }
  pc = 0x409bb0e; continue;
 }
 0x409bb0e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149e94u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409bb34); cpu.call(t); if cpu.returned_to != 0x409bb34 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409bb3c); f_0409d690(cpu); if cpu.returned_to != 0x409bb3c { return; }
  pc = 0x409bb3c; continue;
 }
 0x409bb3c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409bb49); f_040a54d0(cpu); if cpu.returned_to != 0x409bb49 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409bb97; continue; }
  pc = 0x409bb58; continue;
 }
 0x409bb58 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409bb6a; continue; }
  pc = 0x409bb61; continue;
 }
 0x409bb61 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bb97; continue; }
  pc = 0x409bb6a; continue;
 }
 0x409bb6a => {
  cpu.push(0x409bb6f); f_040a3b61(cpu); if cpu.returned_to != 0x409bb6f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bb97; continue; }
  pc = 0x409bb73; continue;
 }
 0x409bb73 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409bb8d); f_040a3df3(cpu); if cpu.returned_to != 0x409bb8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409bb97); f_0409d640(cpu); if cpu.returned_to != 0x409bb97 { return; }
  pc = 0x409bb97; continue;
 }
 0x409bb97 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409bbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409bbc0); }
 let mut pc: u32 = 0x409bbc0;
 loop { match pc {
 0x409bbc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bc07; continue; }
  pc = 0x409bbe0; continue;
 }
 0x409bbe0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409bbee; continue; }
  pc = 0x409bbe5; continue;
 }
 0x409bbe5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bc07; continue; }
  pc = 0x409bbee; continue;
 }
 0x409bbee => {
  cpu.push(0x409bbf3); f_040a3b61(cpu); if cpu.returned_to != 0x409bbf3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bc07; continue; }
  pc = 0x409bbf7; continue;
 }
 0x409bbf7 => {
  cpu.push(0x409bbfc); f_0409d2c0(cpu); if cpu.returned_to != 0x409bbfc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bc07; continue; }
  pc = 0x409bc00; continue;
 }
 0x409bc00 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bc09; continue;
 }
 0x409bc07 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bc09; continue;
 }
 0x409bc09 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bc3b; continue; }
  pc = 0x409bc0d; continue;
 }
 0x409bc0d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149e74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409bc33); cpu.call(t); if cpu.returned_to != 0x409bc33 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409bc3b); f_0409d690(cpu); if cpu.returned_to != 0x409bc3b { return; }
  pc = 0x409bc3b; continue;
 }
 0x409bc3b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409bc48); f_040a41c0(cpu); if cpu.returned_to != 0x409bc48 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409bc70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409bc70); }
 let mut pc: u32 = 0x409bc70;
 loop { match pc {
 0x409bc70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bcb8; continue; }
  pc = 0x409bc91; continue;
 }
 0x409bc91 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409bc9f; continue; }
  pc = 0x409bc96; continue;
 }
 0x409bc96 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bcb8; continue; }
  pc = 0x409bc9f; continue;
 }
 0x409bc9f => {
  cpu.push(0x409bca4); f_040a3b61(cpu); if cpu.returned_to != 0x409bca4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bcb8; continue; }
  pc = 0x409bca8; continue;
 }
 0x409bca8 => {
  cpu.push(0x409bcad); f_0409d2c0(cpu); if cpu.returned_to != 0x409bcad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bcb8; continue; }
  pc = 0x409bcb1; continue;
 }
 0x409bcb1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bcba; continue;
 }
 0x409bcb8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bcba; continue;
 }
 0x409bcba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bcfa; continue; }
  pc = 0x409bcbe; continue;
 }
 0x409bcbe => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149e54u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409bcf2); cpu.call(t); if cpu.returned_to != 0x409bcf2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409bcfa); f_0409d690(cpu); if cpu.returned_to != 0x409bcfa { return; }
  pc = 0x409bcfa; continue;
 }
 0x409bcfa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409bd0f); f_040a4260(cpu); if cpu.returned_to != 0x409bd0f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bd5c; continue; }
  pc = 0x409bd1d; continue;
 }
 0x409bd1d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409bd2f; continue; }
  pc = 0x409bd26; continue;
 }
 0x409bd26 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bd5c; continue; }
  pc = 0x409bd2f; continue;
 }
 0x409bd2f => {
  cpu.push(0x409bd34); f_040a3b61(cpu); if cpu.returned_to != 0x409bd34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bd5c; continue; }
  pc = 0x409bd38; continue;
 }
 0x409bd38 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409bd52); f_040a3df3(cpu); if cpu.returned_to != 0x409bd52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409bd5c); f_0409d5f0(cpu); if cpu.returned_to != 0x409bd5c { return; }
  pc = 0x409bd5c; continue;
 }
 0x409bd5c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409bd80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409bd80); }
 let mut pc: u32 = 0x409bd80;
 loop { match pc {
 0x409bd80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bdc8; continue; }
  pc = 0x409bda1; continue;
 }
 0x409bda1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409bdaf; continue; }
  pc = 0x409bda6; continue;
 }
 0x409bda6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bdc8; continue; }
  pc = 0x409bdaf; continue;
 }
 0x409bdaf => {
  cpu.push(0x409bdb4); f_040a3b61(cpu); if cpu.returned_to != 0x409bdb4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bdc8; continue; }
  pc = 0x409bdb8; continue;
 }
 0x409bdb8 => {
  cpu.push(0x409bdbd); f_0409d2c0(cpu); if cpu.returned_to != 0x409bdbd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bdc8; continue; }
  pc = 0x409bdc1; continue;
 }
 0x409bdc1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bdca; continue;
 }
 0x409bdc8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bdca; continue;
 }
 0x409bdca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bdfc; continue; }
  pc = 0x409bdce; continue;
 }
 0x409bdce => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149e34u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409bdf4); cpu.call(t); if cpu.returned_to != 0x409bdf4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409bdfc); f_0409d690(cpu); if cpu.returned_to != 0x409bdfc { return; }
  pc = 0x409bdfc; continue;
 }
 0x409bdfc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409be09); f_040a4110(cpu); if cpu.returned_to != 0x409be09 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409be57; continue; }
  pc = 0x409be18; continue;
 }
 0x409be18 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409be2a; continue; }
  pc = 0x409be21; continue;
 }
 0x409be21 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409be57; continue; }
  pc = 0x409be2a; continue;
 }
 0x409be2a => {
  cpu.push(0x409be2f); f_040a3b61(cpu); if cpu.returned_to != 0x409be2f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409be57; continue; }
  pc = 0x409be33; continue;
 }
 0x409be33 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409be4d); f_040a3df3(cpu); if cpu.returned_to != 0x409be4d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409be57); f_0409d5f0(cpu); if cpu.returned_to != 0x409be57 { return; }
  pc = 0x409be57; continue;
 }
 0x409be57 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409be80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409be80); }
 let mut pc: u32 = 0x409be80;
 loop { match pc {
 0x409be80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bec0; continue; }
  pc = 0x409be99; continue;
 }
 0x409be99 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409bea7; continue; }
  pc = 0x409be9e; continue;
 }
 0x409be9e => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bec0; continue; }
  pc = 0x409bea7; continue;
 }
 0x409bea7 => {
  cpu.push(0x409beac); f_040a3b61(cpu); if cpu.returned_to != 0x409beac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bec0; continue; }
  pc = 0x409beb0; continue;
 }
 0x409beb0 => {
  cpu.push(0x409beb5); f_0409d2c0(cpu); if cpu.returned_to != 0x409beb5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bec0; continue; }
  pc = 0x409beb9; continue;
 }
 0x409beb9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bec2; continue;
 }
 0x409bec0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bec2; continue;
 }
 0x409bec2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409beec; continue; }
  pc = 0x409bec6; continue;
 }
 0x409bec6 => {
  { let v: u32 = 0x4149e18u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149e18u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409bee9); f_040a3df3(cpu); if cpu.returned_to != 0x409bee9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x409beec; continue;
 }
 0x409beec => {
  cpu.push(0x409bef1); f_040a3beb(cpu); if cpu.returned_to != 0x409bef1 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42c538cu32)); cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409bf00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409bf00); }
 let mut pc: u32 = 0x409bf00;
 loop { match pc {
 0x409bf00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bf47; continue; }
  pc = 0x409bf20; continue;
 }
 0x409bf20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409bf2e; continue; }
  pc = 0x409bf25; continue;
 }
 0x409bf25 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bf47; continue; }
  pc = 0x409bf2e; continue;
 }
 0x409bf2e => {
  cpu.push(0x409bf33); f_040a3b61(cpu); if cpu.returned_to != 0x409bf33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bf47; continue; }
  pc = 0x409bf37; continue;
 }
 0x409bf37 => {
  cpu.push(0x409bf3c); f_0409d2c0(cpu); if cpu.returned_to != 0x409bf3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bf47; continue; }
  pc = 0x409bf40; continue;
 }
 0x409bf40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bf49; continue;
 }
 0x409bf47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bf49; continue;
 }
 0x409bf49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bf7b; continue; }
  pc = 0x409bf4d; continue;
 }
 0x409bf4d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149df8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409bf73); cpu.call(t); if cpu.returned_to != 0x409bf73 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409bf7b); f_0409d690(cpu); if cpu.returned_to != 0x409bf7b { return; }
  pc = 0x409bf7b; continue;
 }
 0x409bf7b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409bf88); f_040a3bca(cpu); if cpu.returned_to != 0x409bf88 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409bfb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409bfb0); }
 let mut pc: u32 = 0x409bfb0;
 loop { match pc {
 0x409bfb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409bff0; continue; }
  pc = 0x409bfc9; continue;
 }
 0x409bfc9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409bfd7; continue; }
  pc = 0x409bfce; continue;
 }
 0x409bfce => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409bff0; continue; }
  pc = 0x409bfd7; continue;
 }
 0x409bfd7 => {
  cpu.push(0x409bfdc); f_040a3b61(cpu); if cpu.returned_to != 0x409bfdc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409bff0; continue; }
  pc = 0x409bfe0; continue;
 }
 0x409bfe0 => {
  cpu.push(0x409bfe5); f_0409d2c0(cpu); if cpu.returned_to != 0x409bfe5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409bff0; continue; }
  pc = 0x409bfe9; continue;
 }
 0x409bfe9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409bff2; continue;
 }
 0x409bff0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409bff2; continue;
 }
 0x409bff2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c01c; continue; }
  pc = 0x409bff6; continue;
 }
 0x409bff6 => {
  { let v: u32 = 0x4149de0u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149de0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409c019); f_040a3df3(cpu); if cpu.returned_to != 0x409c019 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x409c01c; continue;
 }
 0x409c01c => {
  cpu.push(0x409c021); f_040a3c7f(cpu); if cpu.returned_to != 0x409c021 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42c538cu32)); cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c030); }
 let mut pc: u32 = 0x409c030;
 loop { match pc {
 0x409c030 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c077; continue; }
  pc = 0x409c050; continue;
 }
 0x409c050 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c05e; continue; }
  pc = 0x409c055; continue;
 }
 0x409c055 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c077; continue; }
  pc = 0x409c05e; continue;
 }
 0x409c05e => {
  cpu.push(0x409c063); f_040a3b61(cpu); if cpu.returned_to != 0x409c063 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c077; continue; }
  pc = 0x409c067; continue;
 }
 0x409c067 => {
  cpu.push(0x409c06c); f_0409d2c0(cpu); if cpu.returned_to != 0x409c06c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c077; continue; }
  pc = 0x409c070; continue;
 }
 0x409c070 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c079; continue;
 }
 0x409c077 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c079; continue;
 }
 0x409c079 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c0ab; continue; }
  pc = 0x409c07d; continue;
 }
 0x409c07d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149dc8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c0a3); cpu.call(t); if cpu.returned_to != 0x409c0a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c0ab); f_0409d690(cpu); if cpu.returned_to != 0x409c0ab { return; }
  pc = 0x409c0ab; continue;
 }
 0x409c0ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409c0b8); f_040a3c55(cpu); if cpu.returned_to != 0x409c0b8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c0e0); }
 let mut pc: u32 = 0x409c0e0;
 loop { match pc {
 0x409c0e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c120; continue; }
  pc = 0x409c0f9; continue;
 }
 0x409c0f9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c107; continue; }
  pc = 0x409c0fe; continue;
 }
 0x409c0fe => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c120; continue; }
  pc = 0x409c107; continue;
 }
 0x409c107 => {
  cpu.push(0x409c10c); f_040a3b61(cpu); if cpu.returned_to != 0x409c10c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c120; continue; }
  pc = 0x409c110; continue;
 }
 0x409c110 => {
  cpu.push(0x409c115); f_0409d2c0(cpu); if cpu.returned_to != 0x409c115 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c120; continue; }
  pc = 0x409c119; continue;
 }
 0x409c119 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c122; continue;
 }
 0x409c120 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c122; continue;
 }
 0x409c122 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c14c; continue; }
  pc = 0x409c126; continue;
 }
 0x409c126 => {
  { let v: u32 = 0x4149dacu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149dacu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409c149); f_040a3df3(cpu); if cpu.returned_to != 0x409c149 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x409c14c; continue;
 }
 0x409c14c => {
  cpu.push(0x409c151); f_040a3c35(cpu); if cpu.returned_to != 0x409c151 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x42c538cu32)); cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c160); }
 let mut pc: u32 = 0x409c160;
 loop { match pc {
 0x409c160 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c1a7; continue; }
  pc = 0x409c180; continue;
 }
 0x409c180 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c18e; continue; }
  pc = 0x409c185; continue;
 }
 0x409c185 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c1a7; continue; }
  pc = 0x409c18e; continue;
 }
 0x409c18e => {
  cpu.push(0x409c193); f_040a3b61(cpu); if cpu.returned_to != 0x409c193 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c1a7; continue; }
  pc = 0x409c197; continue;
 }
 0x409c197 => {
  cpu.push(0x409c19c); f_0409d2c0(cpu); if cpu.returned_to != 0x409c19c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c1a7; continue; }
  pc = 0x409c1a0; continue;
 }
 0x409c1a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c1a9; continue;
 }
 0x409c1a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c1a9; continue;
 }
 0x409c1a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c1db; continue; }
  pc = 0x409c1ad; continue;
 }
 0x409c1ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149d94u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c1d3); cpu.call(t); if cpu.returned_to != 0x409c1d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c1db); f_0409d690(cpu); if cpu.returned_to != 0x409c1db { return; }
  pc = 0x409c1db; continue;
 }
 0x409c1db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409c1e8); f_040a3c0b(cpu); if cpu.returned_to != 0x409c1e8 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c210); }
 let mut pc: u32 = 0x409c210;
 loop { match pc {
 0x409c210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c252; continue; }
  pc = 0x409c22b; continue;
 }
 0x409c22b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c239; continue; }
  pc = 0x409c230; continue;
 }
 0x409c230 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c252; continue; }
  pc = 0x409c239; continue;
 }
 0x409c239 => {
  cpu.push(0x409c23e); f_040a3b61(cpu); if cpu.returned_to != 0x409c23e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c252; continue; }
  pc = 0x409c242; continue;
 }
 0x409c242 => {
  cpu.push(0x409c247); f_0409d2c0(cpu); if cpu.returned_to != 0x409c247 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c252; continue; }
  pc = 0x409c24b; continue;
 }
 0x409c24b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c254; continue;
 }
 0x409c252 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c254; continue;
 }
 0x409c254 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c27e; continue; }
  pc = 0x409c258; continue;
 }
 0x409c258 => {
  { let v: u32 = 0x4149d78u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149d78u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409c27b); f_040a3df3(cpu); if cpu.returned_to != 0x409c27b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x409c27e; continue;
 }
 0x409c27e => {
  cpu.push(0x409c283); f_040a3d35(cpu); if cpu.returned_to != 0x409c283 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409c2ce; continue; }
  pc = 0x409c28f; continue;
 }
 0x409c28f => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409c2a1; continue; }
  pc = 0x409c298; continue;
 }
 0x409c298 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c2ce; continue; }
  pc = 0x409c2a1; continue;
 }
 0x409c2a1 => {
  cpu.push(0x409c2a6); f_040a3b61(cpu); if cpu.returned_to != 0x409c2a6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c2ce; continue; }
  pc = 0x409c2aa; continue;
 }
 0x409c2aa => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409c2c4); f_040a3df3(cpu); if cpu.returned_to != 0x409c2c4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409c2ce); f_0409d640(cpu); if cpu.returned_to != 0x409c2ce { return; }
  pc = 0x409c2ce; continue;
 }
 0x409c2ce => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c3b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c3b0); }
 let mut pc: u32 = 0x409c3b0;
 loop { match pc {
 0x409c3b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c3f7; continue; }
  pc = 0x409c3d0; continue;
 }
 0x409c3d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c3de; continue; }
  pc = 0x409c3d5; continue;
 }
 0x409c3d5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c3f7; continue; }
  pc = 0x409c3de; continue;
 }
 0x409c3de => {
  cpu.push(0x409c3e3); f_040a3b61(cpu); if cpu.returned_to != 0x409c3e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c3f7; continue; }
  pc = 0x409c3e7; continue;
 }
 0x409c3e7 => {
  cpu.push(0x409c3ec); f_0409d2c0(cpu); if cpu.returned_to != 0x409c3ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c3f7; continue; }
  pc = 0x409c3f0; continue;
 }
 0x409c3f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c3f9; continue;
 }
 0x409c3f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c3f9; continue;
 }
 0x409c3f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c439; continue; }
  pc = 0x409c3fd; continue;
 }
 0x409c3fd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149d34u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c431); cpu.call(t); if cpu.returned_to != 0x409c431 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c439); f_0409d690(cpu); if cpu.returned_to != 0x409c439 { return; }
  pc = 0x409c439; continue;
 }
 0x409c439 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409c44e); f_040a3ccf(cpu); if cpu.returned_to != 0x409c44e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c470); }
 let mut pc: u32 = 0x409c470;
 loop { match pc {
 0x409c470 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c4b7; continue; }
  pc = 0x409c490; continue;
 }
 0x409c490 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c49e; continue; }
  pc = 0x409c495; continue;
 }
 0x409c495 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c4b7; continue; }
  pc = 0x409c49e; continue;
 }
 0x409c49e => {
  cpu.push(0x409c4a3); f_040a3b61(cpu); if cpu.returned_to != 0x409c4a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c4b7; continue; }
  pc = 0x409c4a7; continue;
 }
 0x409c4a7 => {
  cpu.push(0x409c4ac); f_0409d2c0(cpu); if cpu.returned_to != 0x409c4ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c4b7; continue; }
  pc = 0x409c4b0; continue;
 }
 0x409c4b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c4b9; continue;
 }
 0x409c4b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c4b9; continue;
 }
 0x409c4b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c4f9; continue; }
  pc = 0x409c4bd; continue;
 }
 0x409c4bd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149d14u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c4f1); cpu.call(t); if cpu.returned_to != 0x409c4f1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c4f9); f_0409d690(cpu); if cpu.returned_to != 0x409c4f9 { return; }
  pc = 0x409c4f9; continue;
 }
 0x409c4f9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409c50e); f_040a3c9f(cpu); if cpu.returned_to != 0x409c50e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409c530(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c530); }
 let mut pc: u32 = 0x409c530;
 loop { match pc {
 0x409c530 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c578; continue; }
  pc = 0x409c551; continue;
 }
 0x409c551 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c55f; continue; }
  pc = 0x409c556; continue;
 }
 0x409c556 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c578; continue; }
  pc = 0x409c55f; continue;
 }
 0x409c55f => {
  cpu.push(0x409c564); f_040a3b61(cpu); if cpu.returned_to != 0x409c564 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c578; continue; }
  pc = 0x409c568; continue;
 }
 0x409c568 => {
  cpu.push(0x409c56d); f_0409d2c0(cpu); if cpu.returned_to != 0x409c56d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c578; continue; }
  pc = 0x409c571; continue;
 }
 0x409c571 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c57a; continue;
 }
 0x409c578 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c57a; continue;
 }
 0x409c57a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c5ba; continue; }
  pc = 0x409c57e; continue;
 }
 0x409c57e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149cf8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c5b2); cpu.call(t); if cpu.returned_to != 0x409c5b2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c5ba); f_0409d690(cpu); if cpu.returned_to != 0x409c5ba { return; }
  pc = 0x409c5ba; continue;
 }
 0x409c5ba => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409c5cf); f_040a3ba8(cpu); if cpu.returned_to != 0x409c5cf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c61c; continue; }
  pc = 0x409c5dd; continue;
 }
 0x409c5dd => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409c5ef; continue; }
  pc = 0x409c5e6; continue;
 }
 0x409c5e6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c61c; continue; }
  pc = 0x409c5ef; continue;
 }
 0x409c5ef => {
  cpu.push(0x409c5f4); f_040a3b61(cpu); if cpu.returned_to != 0x409c5f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c61c; continue; }
  pc = 0x409c5f8; continue;
 }
 0x409c5f8 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409c612); f_040a3df3(cpu); if cpu.returned_to != 0x409c612 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409c61c); f_0409d640(cpu); if cpu.returned_to != 0x409c61c { return; }
  pc = 0x409c61c; continue;
 }
 0x409c61c => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409c640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c640); }
 let mut pc: u32 = 0x409c640;
 loop { match pc {
 0x409c640 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c688; continue; }
  pc = 0x409c661; continue;
 }
 0x409c661 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c66f; continue; }
  pc = 0x409c666; continue;
 }
 0x409c666 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c688; continue; }
  pc = 0x409c66f; continue;
 }
 0x409c66f => {
  cpu.push(0x409c674); f_040a3b61(cpu); if cpu.returned_to != 0x409c674 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c688; continue; }
  pc = 0x409c678; continue;
 }
 0x409c678 => {
  cpu.push(0x409c67d); f_0409d2c0(cpu); if cpu.returned_to != 0x409c67d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c688; continue; }
  pc = 0x409c681; continue;
 }
 0x409c681 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c68a; continue;
 }
 0x409c688 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c68a; continue;
 }
 0x409c68a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c6bc; continue; }
  pc = 0x409c68e; continue;
 }
 0x409c68e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149cdcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c6b4); cpu.call(t); if cpu.returned_to != 0x409c6b4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c6bc); f_0409d690(cpu); if cpu.returned_to != 0x409c6bc { return; }
  pc = 0x409c6bc; continue;
 }
 0x409c6bc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409c6c9); f_040a3b67(cpu); if cpu.returned_to != 0x409c6c9 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409c717; continue; }
  pc = 0x409c6d8; continue;
 }
 0x409c6d8 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409c6ea; continue; }
  pc = 0x409c6e1; continue;
 }
 0x409c6e1 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c717; continue; }
  pc = 0x409c6ea; continue;
 }
 0x409c6ea => {
  cpu.push(0x409c6ef); f_040a3b61(cpu); if cpu.returned_to != 0x409c6ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c717; continue; }
  pc = 0x409c6f3; continue;
 }
 0x409c6f3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409c70d); f_040a3df3(cpu); if cpu.returned_to != 0x409c70d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409c717); f_0409d640(cpu); if cpu.returned_to != 0x409c717 { return; }
  pc = 0x409c717; continue;
 }
 0x409c717 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409c740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409c740); }
 let mut pc: u32 = 0x409c740;
 loop { match pc {
 0x409c740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3a0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3b4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3b8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409c796; continue; }
  pc = 0x409c76f; continue;
 }
 0x409c76f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409c77d; continue; }
  pc = 0x409c774; continue;
 }
 0x409c774 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409c796; continue; }
  pc = 0x409c77d; continue;
 }
 0x409c77d => {
  cpu.push(0x409c782); f_040a3b61(cpu); if cpu.returned_to != 0x409c782 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409c796; continue; }
  pc = 0x409c786; continue;
 }
 0x409c786 => {
  cpu.push(0x409c78b); f_0409d2c0(cpu); if cpu.returned_to != 0x409c78b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c796; continue; }
  pc = 0x409c78f; continue;
 }
 0x409c78f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409c798; continue;
 }
 0x409c796 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409c798; continue;
 }
 0x409c798 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409c7d4; continue; }
  pc = 0x409c79c; continue;
 }
 0x409c79c => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149c60u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x38cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c7c7); cpu.call(t); if cpu.returned_to != 0x409c7c7 { return; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c7d4); f_0409d690(cpu); if cpu.returned_to != 0x409c7d4 { return; }
  pc = 0x409c7d4; continue;
 }
 0x409c7d4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409c7db); f_040a4570(cpu); if cpu.returned_to != 0x409c7db { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ca0d; continue; }
  pc = 0x409c7e8; continue;
 }
 0x409c7e8 => {
  { cpu.sub32(cpu.r32(0x42c5390u32), 0x0u32); }
  if cpu.zf { pc = 0x409ca55; continue; }
  pc = 0x409c7f5; continue;
 }
 0x409c7f5 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409c80b; continue; }
  pc = 0x409c7fe; continue;
 }
 0x409c7fe => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ca55; continue; }
  pc = 0x409c80b; continue;
 }
 0x409c80b => {
  cpu.push(0x409c810); f_040a3b61(cpu); if cpu.returned_to != 0x409c810 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ca55; continue; }
  pc = 0x409c818; continue;
 }
 0x409c818 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x409c832); f_040a3df3(cpu); if cpu.returned_to != 0x409c832 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x388u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149c7cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x394u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c85f); cpu.call(t); if cpu.returned_to != 0x409c85f { return; }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c86c); f_0409d690(cpu); if cpu.returned_to != 0x409c86c { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409c886); f_040a3df3(cpu); if cpu.returned_to != 0x409c886 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149c8cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3acu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c8a8); cpu.call(t); if cpu.returned_to != 0x409c8a8 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c8b0); f_0409d690(cpu); if cpu.returned_to != 0x409c8b0 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409c8ca); f_040a3df3(cpu); if cpu.returned_to != 0x409c8ca { return; }
  { let v: u32 = cpu.r32(0xfeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149c9cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x398u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c8fa); cpu.call(t); if cpu.returned_to != 0x409c8fa { return; }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c907); f_0409d690(cpu); if cpu.returned_to != 0x409c907 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c920); f_040a3df3(cpu); if cpu.returned_to != 0x409c920 { return; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149cacu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x39cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c950); cpu.call(t); if cpu.returned_to != 0x409c950 { return; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c95d); f_0409d690(cpu); if cpu.returned_to != 0x409c95d { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409c977); f_040a3df3(cpu); if cpu.returned_to != 0x409c977 { return; }
  { let v: u32 = cpu.r32(0x102u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149cbcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c9a7); cpu.call(t); if cpu.returned_to != 0x409c9a7 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409c9b4); f_0409d690(cpu); if cpu.returned_to != 0x409c9b4 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x409c9ce); f_040a3df3(cpu); if cpu.returned_to != 0x409c9ce { return; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x398u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149cccu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409c9fe); cpu.call(t); if cpu.returned_to != 0x409c9fe { return; }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ca0b); f_0409d690(cpu); if cpu.returned_to != 0x409ca0b { return; }
  pc = 0x409ca55; continue;
 }
 0x409ca0d => {
  { cpu.sub32(cpu.r32(0x42c5390u32), 0x0u32); }
  if cpu.zf { pc = 0x409ca55; continue; }
  pc = 0x409ca16; continue;
 }
 0x409ca16 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409ca28; continue; }
  pc = 0x409ca1f; continue;
 }
 0x409ca1f => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ca55; continue; }
  pc = 0x409ca28; continue;
 }
 0x409ca28 => {
  cpu.push(0x409ca2d); f_040a3b61(cpu); if cpu.returned_to != 0x409ca2d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ca55; continue; }
  pc = 0x409ca31; continue;
 }
 0x409ca31 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409ca4b); f_040a3df3(cpu); if cpu.returned_to != 0x409ca4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x409ca55); f_0409d640(cpu); if cpu.returned_to != 0x409ca55 { return; }
  pc = 0x409ca55; continue;
 }
 0x409ca55 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409ca70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ca70); }
 let mut pc: u32 = 0x409ca70;
 loop { match pc {
 0x409ca70 => {
  f_040a3b61(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ca80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ca80); }
 let mut pc: u32 = 0x409ca80;
 loop { match pc {
 0x409ca80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409cac7; continue; }
  pc = 0x409caa0; continue;
 }
 0x409caa0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409caae; continue; }
  pc = 0x409caa5; continue;
 }
 0x409caa5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409cac7; continue; }
  pc = 0x409caae; continue;
 }
 0x409caae => {
  cpu.push(0x409cab3); f_040a3b61(cpu); if cpu.returned_to != 0x409cab3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409cac7; continue; }
  pc = 0x409cab7; continue;
 }
 0x409cab7 => {
  cpu.push(0x409cabc); f_0409d2c0(cpu); if cpu.returned_to != 0x409cabc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cac7; continue; }
  pc = 0x409cac0; continue;
 }
 0x409cac0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409cac9; continue;
 }
 0x409cac7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409cac9; continue;
 }
 0x409cac9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cafb; continue; }
  pc = 0x409cacd; continue;
 }
 0x409cacd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149c50u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409caf3); cpu.call(t); if cpu.returned_to != 0x409caf3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409cafb); f_0409d690(cpu); if cpu.returned_to != 0x409cafb { return; }
  pc = 0x409cafb; continue;
 }
 0x409cafb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409cb08); f_040a3f30(cpu); if cpu.returned_to != 0x409cb08 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cb30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cb30); }
 let mut pc: u32 = 0x409cb30;
 loop { match pc {
 0x409cb30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409cb77; continue; }
  pc = 0x409cb50; continue;
 }
 0x409cb50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409cb5e; continue; }
  pc = 0x409cb55; continue;
 }
 0x409cb55 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409cb77; continue; }
  pc = 0x409cb5e; continue;
 }
 0x409cb5e => {
  cpu.push(0x409cb63); f_040a3b61(cpu); if cpu.returned_to != 0x409cb63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409cb77; continue; }
  pc = 0x409cb67; continue;
 }
 0x409cb67 => {
  cpu.push(0x409cb6c); f_0409d2c0(cpu); if cpu.returned_to != 0x409cb6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cb77; continue; }
  pc = 0x409cb70; continue;
 }
 0x409cb70 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x409cb79; continue;
 }
 0x409cb77 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x409cb79; continue;
 }
 0x409cb79 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409cbd5; continue; }
  pc = 0x409cb7d; continue;
 }
 0x409cb7d => {
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
  { let v = 0x4149c28u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409cbcd); cpu.call(t); if cpu.returned_to != 0x409cbcd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409cbd5); f_0409d690(cpu); if cpu.returned_to != 0x409cbd5 { return; }
  pc = 0x409cbd5; continue;
 }
 0x409cbd5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409cbfa); f_040a3a87(cpu); if cpu.returned_to != 0x409cbfa { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x409cc48; continue; }
  pc = 0x409cc09; continue;
 }
 0x409cc09 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409cc1b; continue; }
  pc = 0x409cc12; continue;
 }
 0x409cc12 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409cc48; continue; }
  pc = 0x409cc1b; continue;
 }
 0x409cc1b => {
  cpu.push(0x409cc20); f_040a3b61(cpu); if cpu.returned_to != 0x409cc20 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409cc48; continue; }
  pc = 0x409cc24; continue;
 }
 0x409cc24 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409cc3e); f_040a3df3(cpu); if cpu.returned_to != 0x409cc3e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409cc48); f_0409d640(cpu); if cpu.returned_to != 0x409cc48 { return; }
  pc = 0x409cc48; continue;
 }
 0x409cc48 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x42c538cu32)); cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409cc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cc60); }
 let mut pc: u32 = 0x409cc60;
 loop { match pc {
 0x409cc60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409cc6a); f_040a3a20(cpu); if cpu.returned_to != 0x409cc6a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cc70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cc70); }
 let mut pc: u32 = 0x409cc70;
 loop { match pc {
 0x409cc70 => {
  pc = 0x40a3a1c; continue;
 }
 0x40a3a1c => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cc80); }
 let mut pc: u32 = 0x409cc80;
 loop { match pc {
 0x409cc80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409ccc7; continue; }
  pc = 0x409cca0; continue;
 }
 0x409cca0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409ccae; continue; }
  pc = 0x409cca5; continue;
 }
 0x409cca5 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ccc7; continue; }
  pc = 0x409ccae; continue;
 }
 0x409ccae => {
  cpu.push(0x409ccb3); f_040a3b61(cpu); if cpu.returned_to != 0x409ccb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ccc7; continue; }
  pc = 0x409ccb7; continue;
 }
 0x409ccb7 => {
  cpu.push(0x409ccbc); f_0409d2c0(cpu); if cpu.returned_to != 0x409ccbc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ccc7; continue; }
  pc = 0x409ccc0; continue;
 }
 0x409ccc0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409ccc9; continue;
 }
 0x409ccc7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409ccc9; continue;
 }
 0x409ccc9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ccfb; continue; }
  pc = 0x409cccd; continue;
 }
 0x409cccd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149c0cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409ccf3); cpu.call(t); if cpu.returned_to != 0x409ccf3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ccfb); f_0409d690(cpu); if cpu.returned_to != 0x409ccfb { return; }
  pc = 0x409ccfb; continue;
 }
 0x409ccfb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409cd08); f_040a39d7(cpu); if cpu.returned_to != 0x409cd08 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cd30); }
 let mut pc: u32 = 0x409cd30;
 loop { match pc {
 0x409cd30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409cd77; continue; }
  pc = 0x409cd50; continue;
 }
 0x409cd50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409cd5e; continue; }
  pc = 0x409cd55; continue;
 }
 0x409cd55 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409cd77; continue; }
  pc = 0x409cd5e; continue;
 }
 0x409cd5e => {
  cpu.push(0x409cd63); f_040a3b61(cpu); if cpu.returned_to != 0x409cd63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409cd77; continue; }
  pc = 0x409cd67; continue;
 }
 0x409cd67 => {
  cpu.push(0x409cd6c); f_0409d2c0(cpu); if cpu.returned_to != 0x409cd6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cd77; continue; }
  pc = 0x409cd70; continue;
 }
 0x409cd70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409cd79; continue;
 }
 0x409cd77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409cd79; continue;
 }
 0x409cd79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cdc7; continue; }
  pc = 0x409cd7d; continue;
 }
 0x409cd7d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149becu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409cdbf); cpu.call(t); if cpu.returned_to != 0x409cdbf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409cdc7); f_0409d690(cpu); if cpu.returned_to != 0x409cdc7 { return; }
  pc = 0x409cdc7; continue;
 }
 0x409cdc7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409cde4); f_040a37c1(cpu); if cpu.returned_to != 0x409cde4 { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ce00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ce00); }
 let mut pc: u32 = 0x409ce00;
 loop { match pc {
 0x409ce00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409ce47; continue; }
  pc = 0x409ce20; continue;
 }
 0x409ce20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409ce2e; continue; }
  pc = 0x409ce25; continue;
 }
 0x409ce25 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409ce47; continue; }
  pc = 0x409ce2e; continue;
 }
 0x409ce2e => {
  cpu.push(0x409ce33); f_040a3b61(cpu); if cpu.returned_to != 0x409ce33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409ce47; continue; }
  pc = 0x409ce37; continue;
 }
 0x409ce37 => {
  cpu.push(0x409ce3c); f_0409d2c0(cpu); if cpu.returned_to != 0x409ce3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ce47; continue; }
  pc = 0x409ce40; continue;
 }
 0x409ce40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409ce49; continue;
 }
 0x409ce47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409ce49; continue;
 }
 0x409ce49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ce89; continue; }
  pc = 0x409ce4d; continue;
 }
 0x409ce4d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149bccu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409ce81); cpu.call(t); if cpu.returned_to != 0x409ce81 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409ce89); f_0409d690(cpu); if cpu.returned_to != 0x409ce89 { return; }
  pc = 0x409ce89; continue;
 }
 0x409ce89 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409ce9e); f_040a3749(cpu); if cpu.returned_to != 0x409ce9e { return; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cec0); }
 let mut pc: u32 = 0x409cec0;
 loop { match pc {
 0x409cec0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409cf08; continue; }
  pc = 0x409cee1; continue;
 }
 0x409cee1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409ceef; continue; }
  pc = 0x409cee6; continue;
 }
 0x409cee6 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409cf08; continue; }
  pc = 0x409ceef; continue;
 }
 0x409ceef => {
  cpu.push(0x409cef4); f_040a3b61(cpu); if cpu.returned_to != 0x409cef4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409cf08; continue; }
  pc = 0x409cef8; continue;
 }
 0x409cef8 => {
  cpu.push(0x409cefd); f_0409d2c0(cpu); if cpu.returned_to != 0x409cefd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cf08; continue; }
  pc = 0x409cf01; continue;
 }
 0x409cf01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409cf0a; continue;
 }
 0x409cf08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409cf0a; continue;
 }
 0x409cf0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409cf3c; continue; }
  pc = 0x409cf0e; continue;
 }
 0x409cf0e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149bb0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409cf34); cpu.call(t); if cpu.returned_to != 0x409cf34 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409cf3c); f_0409d690(cpu); if cpu.returned_to != 0x409cf3c { return; }
  pc = 0x409cf3c; continue;
 }
 0x409cf3c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409cf49); f_040a3721(cpu); if cpu.returned_to != 0x409cf49 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409cf97; continue; }
  pc = 0x409cf58; continue;
 }
 0x409cf58 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409cf6a; continue; }
  pc = 0x409cf61; continue;
 }
 0x409cf61 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409cf97; continue; }
  pc = 0x409cf6a; continue;
 }
 0x409cf6a => {
  cpu.push(0x409cf6f); f_040a3b61(cpu); if cpu.returned_to != 0x409cf6f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409cf97; continue; }
  pc = 0x409cf73; continue;
 }
 0x409cf73 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409cf8d); f_040a3df3(cpu); if cpu.returned_to != 0x409cf8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409cf97); f_0409d5f0(cpu); if cpu.returned_to != 0x409cf97 { return; }
  pc = 0x409cf97; continue;
 }
 0x409cf97 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409cfc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cfc0); }
 let mut pc: u32 = 0x409cfc0;
 loop { match pc {
 0x409cfc0 => {
  pc = 0x40a32b7; continue;
 }
 0x40a32b7 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x415bacau32)); cpu.w32(0x415bacau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cfd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409cfd0); } r_0409cfd0(cpu, 0x409cfd0); }
pub fn f_040a32b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a32b0); } r_0409cfd0(cpu, 0x40a32b0); }
fn r_0409cfd0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409cfd0 => {
  pc = 0x40a32b0; continue;
 }
 0x40a32b0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x415bacau32)); cpu.w32(0x415bacau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409cfe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409cfe0); }
 let mut pc: u32 = 0x409cfe0;
 loop { match pc {
 0x409cfe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409d028; continue; }
  pc = 0x409d001; continue;
 }
 0x409d001 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409d00f; continue; }
  pc = 0x409d006; continue;
 }
 0x409d006 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409d028; continue; }
  pc = 0x409d00f; continue;
 }
 0x409d00f => {
  cpu.push(0x409d014); f_040a3b61(cpu); if cpu.returned_to != 0x409d014 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409d028; continue; }
  pc = 0x409d018; continue;
 }
 0x409d018 => {
  cpu.push(0x409d01d); f_0409d2c0(cpu); if cpu.returned_to != 0x409d01d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d028; continue; }
  pc = 0x409d021; continue;
 }
 0x409d021 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409d02a; continue;
 }
 0x409d028 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409d02a; continue;
 }
 0x409d02a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d06a; continue; }
  pc = 0x409d02e; continue;
 }
 0x409d02e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149b94u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409d062); cpu.call(t); if cpu.returned_to != 0x409d062 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409d06a); f_0409d690(cpu); if cpu.returned_to != 0x409d06a { return; }
  pc = 0x409d06a; continue;
 }
 0x409d06a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409d07f); f_040a4550(cpu); if cpu.returned_to != 0x409d07f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d0cc; continue; }
  pc = 0x409d08d; continue;
 }
 0x409d08d => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409d09f; continue; }
  pc = 0x409d096; continue;
 }
 0x409d096 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409d0cc; continue; }
  pc = 0x409d09f; continue;
 }
 0x409d09f => {
  cpu.push(0x409d0a4); f_040a3b61(cpu); if cpu.returned_to != 0x409d0a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409d0cc; continue; }
  pc = 0x409d0a8; continue;
 }
 0x409d0a8 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409d0c2); f_040a3df3(cpu); if cpu.returned_to != 0x409d0c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409d0cc); f_0409d640(cpu); if cpu.returned_to != 0x409d0cc { return; }
  pc = 0x409d0cc; continue;
 }
 0x409d0cc => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409d0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d0f0); }
 let mut pc: u32 = 0x409d0f0;
 loop { match pc {
 0x409d0f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409d138; continue; }
  pc = 0x409d111; continue;
 }
 0x409d111 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x409d11f; continue; }
  pc = 0x409d116; continue;
 }
 0x409d116 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409d138; continue; }
  pc = 0x409d11f; continue;
 }
 0x409d11f => {
  cpu.push(0x409d124); f_040a3b61(cpu); if cpu.returned_to != 0x409d124 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409d138; continue; }
  pc = 0x409d128; continue;
 }
 0x409d128 => {
  cpu.push(0x409d12d); f_0409d2c0(cpu); if cpu.returned_to != 0x409d12d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d138; continue; }
  pc = 0x409d131; continue;
 }
 0x409d131 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409d13a; continue;
 }
 0x409d138 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409d13a; continue;
 }
 0x409d13a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d16c; continue; }
  pc = 0x409d13e; continue;
 }
 0x409d13e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149b74u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409d164); cpu.call(t); if cpu.returned_to != 0x409d164 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409d16c); f_0409d690(cpu); if cpu.returned_to != 0x409d16c { return; }
  pc = 0x409d16c; continue;
 }
 0x409d16c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x409d179); f_040a3f50(cpu); if cpu.returned_to != 0x409d179 { return; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409d1c7; continue; }
  pc = 0x409d188; continue;
 }
 0x409d188 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409d19a; continue; }
  pc = 0x409d191; continue;
 }
 0x409d191 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409d1c7; continue; }
  pc = 0x409d19a; continue;
 }
 0x409d19a => {
  cpu.push(0x409d19f); f_040a3b61(cpu); if cpu.returned_to != 0x409d19f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409d1c7; continue; }
  pc = 0x409d1a3; continue;
 }
 0x409d1a3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x415a3d4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409d1bd); f_040a3df3(cpu); if cpu.returned_to != 0x409d1bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409d1c7); f_0409d5f0(cpu); if cpu.returned_to != 0x409d1c7 { return; }
  pc = 0x409d1c7; continue;
 }
 0x409d1c7 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x42c538cu32, v); }
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
pub fn f_0409d1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d1f0); }
 let mut pc: u32 = 0x409d1f0;
 loop { match pc {
 0x409d1f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415a3ccu32), 0x0u32); }
  if cpu.zf { pc = 0x409d2b8; continue; }
  pc = 0x409d201; continue;
 }
 0x409d201 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x42c5390u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c538cu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x409d23f; continue; }
  pc = 0x409d218; continue;
 }
 0x409d218 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x409d226; continue; }
  pc = 0x409d21d; continue;
 }
 0x409d21d => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409d23f; continue; }
  pc = 0x409d226; continue;
 }
 0x409d226 => {
  cpu.push(0x409d22b); f_040a3b61(cpu); if cpu.returned_to != 0x409d22b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409d23f; continue; }
  pc = 0x409d22f; continue;
 }
 0x409d22f => {
  cpu.push(0x409d234); f_0409d2c0(cpu); if cpu.returned_to != 0x409d234 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d23f; continue; }
  pc = 0x409d238; continue;
 }
 0x409d238 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409d241; continue;
 }
 0x409d23f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409d241; continue;
 }
 0x409d241 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d26b; continue; }
  pc = 0x409d245; continue;
 }
 0x409d245 => {
  { let v: u32 = 0x4149b60u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149b60u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409d268); f_040a3df3(cpu); if cpu.returned_to != 0x409d268 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x409d26b; continue;
 }
 0x409d26b => {
  cpu.push(0x409d270); f_040a3fc0(cpu); if cpu.returned_to != 0x409d270 { return; }
  { cpu.sub32(cpu.r32(0x42c5390u32), 0x0u32); }
  if cpu.zf { pc = 0x409d2a3; continue; }
  pc = 0x409d279; continue;
 }
 0x409d279 => {
  { cpu.sub32(cpu.r32(0x42c538cu32), 0x1u32); }
  if cpu.zf { pc = 0x409d28b; continue; }
  pc = 0x409d282; continue;
 }
 0x409d282 => {
  { cpu.sub32(cpu.r32(0x42c5394u32), 0x0u32); }
  if cpu.zf { pc = 0x409d2a3; continue; }
  pc = 0x409d28b; continue;
 }
 0x409d28b => {
  cpu.push(0x409d290); f_040a3b61(cpu); if cpu.returned_to != 0x409d290 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409d2a3; continue; }
  pc = 0x409d294; continue;
 }
 0x409d294 => {
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x409d2a0); f_040a3e65(cpu); if cpu.returned_to != 0x409d2a0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x409d2a3; continue;
 }
 0x409d2a3 => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415a3ccu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
  pc = 0x409d2b8; continue;
 }
 0x409d2b8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d2c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d2c0); }
 let mut pc: u32 = 0x409d2c0;
 loop { match pc {
 0x409d2c0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  cpu.push(0x409d2ce); f_040a32be(cpu); if cpu.returned_to != 0x409d2ce { return; }
  { let v: u32 = cpu.r32(0x42c5398u32); cpu.edx = v; }
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
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x409d386; continue; }
  pc = 0x409d32e; continue;
 }
 0x409d32e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149b20u32; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x9cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409d37c); cpu.call(t); if cpu.returned_to != 0x409d37c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x409d384); f_0409d690(cpu); if cpu.returned_to != 0x409d384 { return; }
  pc = 0x409d3ac; continue;
 }
 0x409d386 => {
  { let v: u32 = 0x4149b3cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149b3cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409d3a9); f_040a3df3(cpu); if cpu.returned_to != 0x409d3a9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x409d3ac; continue;
 }
 0x409d3ac => {
  { let v: u32 = cpu.r32(0x42c538cu32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x409d3cf; continue; }
  pc = 0x409d3b7; continue;
 }
 0x409d3b7 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x4149b4cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x409d3cc); f_040a3df3(cpu); if cpu.returned_to != 0x409d3cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x409d3cf; continue;
 }
 0x409d3cf => {
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
pub fn f_0409d3e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d3e0); }
 let mut pc: u32 = 0x409d3e0;
 loop { match pc {
 0x409d3e0 => {
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
  let t = cpu.r32(0x415b918u32); cpu.push(0x409d430); cpu.call(t); if cpu.returned_to != 0x409d430 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409d44e); f_040a3df3(cpu); if cpu.returned_to != 0x409d44e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d5f0); }
 let mut pc: u32 = 0x409d5f0;
 loop { match pc {
 0x409d5f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149b10u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409d614); cpu.call(t); if cpu.returned_to != 0x409d614 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409d632); f_040a3df3(cpu); if cpu.returned_to != 0x409d632 { return; }
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
pub fn f_0409d640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d640); }
 let mut pc: u32 = 0x409d640;
 loop { match pc {
 0x409d640 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x4149b00u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x415b918u32); cpu.push(0x409d664); cpu.call(t); if cpu.returned_to != 0x409d664 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409d682); f_040a3df3(cpu); if cpu.returned_to != 0x409d682 { return; }
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
pub fn f_0409d690(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d690); }
 let mut pc: u32 = 0x409d690;
 loop { match pc {
 0x409d690 => {
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
  { let v: u32 = cpu.r32(0x42c539cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409d6b1); f_040a3df3(cpu); if cpu.returned_to != 0x409d6b1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d6c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d6c0); }
 let mut pc: u32 = 0x409d6c0;
 loop { match pc {
 0x409d6c0 => {
  { cpu.sub32(cpu.r32(0x415a3d0u32), 0x0u32); }
  if !cpu.zf { pc = 0x409d740; continue; }
  pc = 0x409d6c9; continue;
 }
 0x409d6c9 => {
  { let v = 0x40955d0u32; cpu.push(v); }
  { let v = 0x409d6c0u32; cpu.push(v); }
  cpu.push(0x409d6d8); f_040a30c0(cpu); if cpu.returned_to != 0x409d6d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c538cu32; cpu.push(v); }
  cpu.push(0x409d6e7); f_040a3000(cpu); if cpu.returned_to != 0x409d6e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5390u32; cpu.push(v); }
  cpu.push(0x409d6f6); f_040a3000(cpu); if cpu.returned_to != 0x409d6f6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5394u32; cpu.push(v); }
  cpu.push(0x409d705); f_040a3000(cpu); if cpu.returned_to != 0x409d705 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c539cu32; cpu.push(v); }
  cpu.push(0x409d714); f_040a3000(cpu); if cpu.returned_to != 0x409d714 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x415a3ccu32; cpu.push(v); }
  cpu.push(0x409d723); f_040a3000(cpu); if cpu.returned_to != 0x409d723 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x42c5398u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x409d737); f_040a3000(cpu); if cpu.returned_to != 0x409d737 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415a3d0u32, v); }
  pc = 0x409d740; continue;
 }
 0x409d740 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d741(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d741); }
 let mut pc: u32 = 0x409d741;
 loop { match pc {
 0x409d741 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42c61d0u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x409d76b; continue;
 }
 0x409d755 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), cpu.ecx); }
  if cpu.zf { pc = 0x409d769; continue; }
  pc = 0x409d75d; continue;
 }
 0x409d75d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(cpu.r8(0xdu32.wrapping_add(cpu.eax)), 0x10u8); }
  if cpu.zf { pc = 0x409d769; continue; }
  pc = 0x409d764; continue;
 }
 0x409d764 => {
  cpu.push(0x409d769); f_040accda(cpu); if cpu.returned_to != 0x409d769 { return; }
  pc = 0x409d769; continue;
 }
 0x409d769 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x409d76b; continue;
 }
 0x409d76b => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x409d755; continue; }
  pc = 0x409d76f; continue;
 }
 0x409d76f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d778(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d778); }
 let mut pc: u32 = 0x409d778;
 loop { match pc {
 0x409d778 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x409d7c1; continue; }
  pc = 0x409d781; continue;
 }
 0x409d781 => {
  if !cpu.cf { pc = 0x409d7ab; continue; }
  pc = 0x409d783; continue;
 }
 0x409d783 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x409d7ab; continue; }
  pc = 0x409d78a; continue;
 }
 0x409d78a => {
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
 0x409d7ab => {
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
  pc = 0x409d7c1; continue;
 }
 0x409d7c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d7c5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409d7c5); } r_0409d7c5(cpu, 0x409d7c5); }
pub fn f_0409d855(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409d855); } r_0409d7c5(cpu, 0x409d855); }
fn r_0409d7c5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409d7c5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x409d7cb, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x409d7d0, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x409d851; continue; }
  pc = 0x409d7e0; continue;
 }
 0x409d7e0 => {
  if cpu.cf { pc = 0x409d855; continue; }
  pc = 0x409d7e2; continue;
 }
 0x409d7e2 => {
  if cpu.zf { cpu.call(0x409d859); return; }
  pc = 0x409d7e4; continue;
 }
 0x409d7e4 => {
  if (cpu.cf || cpu.zf) { pc = 0x409d85d; continue; }
  pc = 0x409d7e6; continue;
 }
 0x409d7e6 => {
  if cpu.sf { pc = 0x409d861; continue; }
  pc = 0x409d7e8; continue;
 }
 0x409d7e8 => {
  if cpu.pf() { f_0409d7ea(cpu); return; }
  f_0409d7ea(cpu); return;
 }
 0x409d851 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.adc32(cpu.eax, 0xe852c304u32); cpu.eax = v; }
  cpu.unimplemented(0x409d857, "Hlt");
  cpu.call(0x409d858); return;
 }
 0x409d855 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409d85b); f_0409d84f(cpu); if cpu.returned_to != 0x409d85b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  pc = 0x409d85d; continue;
 }
 0x409d85d => {
  if cpu.zf { pc = 0x409d877; continue; }
  pc = 0x409d85f; continue;
 }
 0x409d85f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x41c64e6du32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3039u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x409d877; continue;
 }
 0x409d861 => {
  cpu.unimplemented(0x409d861, "Insd");
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0xc2u8; cpu.w8(0xffffff81u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esi); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x409d877; continue;
 }
 0x409d877 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d7ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d7ea); }
 let mut pc: u32 = 0x409d7ea;
 loop { match pc {
 0x409d7ea => {
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
  pc = 0x409d800; continue;
 }
 0x409d800 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x409d7c5u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x409d800; continue; }
  pc = 0x409d820; continue;
 }
 0x409d820 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x409d820; continue; }
  pc = 0x409d82a; continue;
 }
 0x409d82a => {
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
pub fn f_0409d834(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d834); }
 let mut pc: u32 = 0x409d834;
 loop { match pc {
 0x409d834 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x409d846; continue; }
  pc = 0x409d83c; continue;
 }
 0x409d83c => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x409d846; continue; }
  pc = 0x409d840; continue;
 }
 0x409d840 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x409d846; continue;
 }
 0x409d846 => {
  cpu.push(0x409d84b); f_0409d7ea(cpu); if cpu.returned_to != 0x409d84b { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d84f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d84f); }
 let mut pc: u32 = 0x409d84f;
 loop { match pc {
 0x409d84f => {
  { let v: u32 = 0x415ac74u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d879(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409d879); }
 let mut pc: u32 = 0x409d879;
 loop { match pc {
 0x409d879 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x409d881); f_0409d84f(cpu); if cpu.returned_to != 0x409d881 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409d887; continue; }
  pc = 0x409d885; continue;
 }
 0x409d885 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x409d887; continue;
 }
 0x409d887 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409d88c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409d88c); } r_0409d88c(cpu, 0x409d88c); }
pub fn f_0409d900(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409d900); } r_0409d88c(cpu, 0x409d900); }
pub fn f_0409db1d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409db1d); } r_0409d88c(cpu, 0x409db1d); }
pub fn f_040b422b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40b422b); } r_0409d88c(cpu, 0x40b422b); }
fn r_0409d88c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409d88c => {
  pc = 0x409d904; continue;
 }
 0x409d900 => {
  cpu.unimplemented(0x409d900, "Arpl");
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b820u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b80cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x415b818u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x415b843u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x415b844u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x409d986; continue; }
  pc = 0x409d948; continue;
 }
 0x409d904 => {
  {  }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b820u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b80cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x415b818u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x415b843u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x415b844u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x409d986; continue; }
  pc = 0x409d948; continue;
 }
 0x409d948 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x415b818u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffff000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b80cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x409d9be; continue;
 }
 0x409d986 => {
  { cpu.sub16((cpu.eax as u16), 0x4243u16); }
  if !cpu.zf { pc = 0x409d9c0; continue; }
  pc = 0x409d98c; continue;
 }
 0x409d98c => {
  { let v: u32 = cpu.edx; cpu.w32(0x415ac80u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x415b818u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x409d9be; continue;
 }
 0x409d9be => {
  pc = 0x409da19; continue;
 }
 0x409d9c0 => {
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xff00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x409da02; continue; }
  pc = 0x409d9ce; continue;
 }
 0x409d9ce => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x409d9dd; continue; }
  pc = 0x409d9d7; continue;
 }
 0x409d9d7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x415ac84u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x409d9dd; continue;
 }
 0x409d9dd => {
  { let v: u16 = 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x409d9f1; continue; }
  pc = 0x409d9ef; continue;
 }
 0x409d9ef => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x409d9f1; continue;
 }
 0x409d9f1 => {
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x415b818u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x409da19; continue;
 }
 0x409da02 => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x24u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x17u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x409da19; continue;
 }
 0x409da19 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x415b83au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x415b83bu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x409db61u32.wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x415b83du32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x415b841u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r16(0x415b818u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x42c6690u32; cpu.edx = v; }
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
  if cpu.zf { pc = 0x409da6e; continue; }
  pc = 0x409da6b; continue;
 }
 0x409da6b => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x409da6e; continue;
 }
 0x409da6e => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r16(0x415b841u32.wrapping_add(cpu.bases[0])); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x409da80; continue;
 }
 0x409da80 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x20202020u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x37386f6eu32); }
  if !cpu.zf { pc = 0x409da95; continue; }
  pc = 0x409da8e; continue;
 }
 0x409da8e => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x3du8); }
  if !cpu.zf { pc = 0x409da95; continue; }
  pc = 0x409da94; continue;
 }
 0x409da94 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x409da95; continue;
 }
 0x409da95 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.lods(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x409da95; continue; }
  pc = 0x409da9b; continue;
 }
 0x409da9b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x409da80; continue; }
  pc = 0x409daa0; continue;
 }
 0x409daa0 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x409daa3; continue;
 }
 0x409daa3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.movs(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x409daa3; continue; }
  pc = 0x409daa9; continue;
 }
 0x409daa9 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x415b838u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x415b81cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b808u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x42c6690u32; cpu.ecx = v; }
  { let v: u32 = 0x415ce10u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x409dae4; continue; }
  pc = 0x409dad7; continue;
 }
 0x409dad7 => {
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x409dae4; continue; }
  pc = 0x409dadf; continue;
 }
 0x409dadf => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  pc = 0x409dae4; continue;
 }
 0x409dae4 => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x42c6690u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b810u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x415b814u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.flat_ds(0x409db0f); cpu.push(0x409db14); f_040acfd2(cpu); if cpu.returned_to != 0x409db14 { return; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.flat_ds(0x409db16); cpu.push(0x409db1b); f_040acf83(cpu); if cpu.returned_to != 0x409db1b { return; }
  pc = 0x409db1b; continue;
 }
 0x409db1b => {
  pc = 0x409db40; continue;
 }
 0x409db1d => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x409d900u32; cpu.edx = v; }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.df = false; }
  pc = 0x409db31; continue;
 }
 0x409db31 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x409db31; continue; }
  pc = 0x409db36; continue;
 }
 0x409db36 => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x409db40; continue;
 }
 0x409db40 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.flat_ds(0x409db4b); cpu.push(0x409db50); f_040ad01d(cpu); if cpu.returned_to != 0x409db50 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x4cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x409db55); f_0409db58(cpu); return;
 }
 0x40b422b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x415c9ccu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40b422e); cpu.push(0x40b4234); cpu.call(t); if cpu.returned_to != 0x40b4234 { return; }
  let t = cpu.r32(0x415c9d0u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40b4234); cpu.push(0x40b423a); cpu.call(t); if cpu.returned_to != 0x40b423a { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.poll(); pc = 0x409db1b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409db58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409db58); }
 let mut pc: u32 = 0x409db58;
 loop { match pc {
 0x409db58 => {
  { let v = cpu.r16(0x409db61u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x409db60); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409db61(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409db61); } r_0409db61(cpu, 0x409db61); }
pub fn f_0409db63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409db63); } r_0409db61(cpu, 0x409db63); }
pub fn f_0409db70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409db70); } r_0409db61(cpu, 0x409db70); }
fn r_0409db61(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409db61 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x409db63; continue;
 }
 0x409db63 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ac88u32), 0x0u32); }
  if cpu.zf { pc = 0x409db96; continue; }
  pc = 0x409db7c; continue;
 }
 0x409db70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ac88u32), 0x0u32); }
  if cpu.zf { pc = 0x409db96; continue; }
  pc = 0x409db7c; continue;
 }
 0x409db7c => {
  { let v = 0x409db70u32; cpu.push(v); }
  { let v = 0x409de70u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x409db8d); f_040a3020(cpu); if cpu.returned_to != 0x409db8d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ac88u32, v); }
  pc = 0x409db96; continue;
 }
 0x409db96 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409dba0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409dba0); } r_0409dba0(cpu, 0x409dba0); }
pub fn f_0409dd40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409dd40); } r_0409dba0(cpu, 0x409dd40); }
pub fn f_0409dd58(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409dd58); } r_0409dba0(cpu, 0x409dd58); }
fn r_0409dba0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409dba0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x415a3ccu32), 0x0u32); }
  if !cpu.zf { pc = 0x409dd2f; continue; }
  pc = 0x409dbb4; continue;
 }
 0x409dbb4 => {
  { let v: u32 = 0x409de60u32; cpu.ebx = v; }
  { let v: u32 = 0x409de50u32; cpu.ecx = v; }
  { let v: u32 = 0x409de40u32; cpu.esi = v; }
  { let v: u32 = 0x409de20u32; cpu.edi = v; }
  { let v: u32 = 0x409dd58u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x415ac88u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b908u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x415b90cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x415b910u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x415b914u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x415b918u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409dc16; continue; }
  pc = 0x409dbf4; continue;
 }
 0x409dbf4 => {
  { let v = 0x409db70u32; cpu.push(v); }
  { let v = 0x409de70u32; cpu.push(v); }
  cpu.push(0x409dc03); f_040a30c0(cpu); if cpu.returned_to != 0x409dc03 { return; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415ac88u32, v); }
  cpu.push(0x409dc16); f_0409d6c0(cpu); if cpu.returned_to != 0x409dc16 { return; }
  pc = 0x409dc16; continue;
 }
 0x409dc16 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x414addcu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5390u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c5394u32, v); }
  cpu.push(0x409dc2e); f_040ad1d1(cpu); if cpu.returned_to != 0x409dc2e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x409dc4a; continue; }
  pc = 0x409dc36; continue;
 }
 0x409dc36 => {
  cpu.push(0x409dc3b); f_040a3ff0(cpu); if cpu.returned_to != 0x409dc3b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x415a3ccu32, v); }
  pc = 0x409dd2f; continue;
 }
 0x409dc4a => {
  { let v: u32 = 0x414ade8u32; cpu.eax = v; }
  cpu.push(0x409dc54); f_040ad1d1(cpu); if cpu.returned_to != 0x409dc54 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x409dc64; continue; }
  pc = 0x409dc5a; continue;
 }
 0x409dc5a => {
  { let v: u32 = 0x1u32; cpu.w32(0x42c5394u32, v); }
  pc = 0x409dc64; continue;
 }
 0x409dc64 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x409dc6a); f_040a3d72(cpu); if cpu.returned_to != 0x409dc6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c539cu32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x409dc8f; continue; }
  pc = 0x409dc77; continue;
 }
 0x409dc77 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x409dc81); f_040a3ff0(cpu); if cpu.returned_to != 0x409dc81 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415a3ccu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409dc8f => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x409dc96); f_040953a4(cpu); if cpu.returned_to != 0x409dc96 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x409dc9d); f_04095424(cpu); if cpu.returned_to != 0x409dc9d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x42c53c4u32; cpu.edi = v; }
  cpu.push(0x409dcab); f_040ad370(cpu); if cpu.returned_to != 0x409dcab { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x409dcaf; continue;
 }
 0x409dcaf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x409dcaf; continue; }
  pc = 0x409dcb9; continue;
 }
 0x409dcb9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = 0x414adf8u32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x42c53dcu32, v); }
  cpu.push(0x409dccb); f_0409d690(cpu); if cpu.returned_to != 0x409dccb { return; }
  { let v = 0x414ae4cu32; cpu.push(v); }
  cpu.push(0x409dcd5); f_0409d690(cpu); if cpu.returned_to != 0x409dcd5 { return; }
  { let v = 0x414ae88u32; cpu.push(v); }
  cpu.push(0x409dcdf); f_0409d690(cpu); if cpu.returned_to != 0x409dcdf { return; }
  { let v = 0x42c53c4u32; cpu.push(v); }
  cpu.push(0x409dce9); f_0409d690(cpu); if cpu.returned_to != 0x409dce9 { return; }
  { let v = 0x414ae98u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x409dcf8); f_0409d690(cpu); if cpu.returned_to != 0x409dcf8 { return; }
  cpu.push(0x409dcfd); f_040a3ff0(cpu); if cpu.returned_to != 0x409dcfd { return; }
  cpu.push(0x409dd02); f_040a32be(cpu); if cpu.returned_to != 0x409dd02 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5398u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x42c5390u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x42c538cu32, v); }
  cpu.push(0x409dd18); f_0409d2c0(cpu); if cpu.returned_to != 0x409dd18 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x414aef0u32; cpu.push(v); }
  pc = 0x409dd1f; continue;
 }
 0x409dd1f => {
  { let v: u32 = cpu.eax; cpu.w32(0x42c538cu32, v); }
  cpu.push(0x409dd29); f_0409d690(cpu); if cpu.returned_to != 0x409dd29 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x415a3ccu32, v); }
  pc = 0x409dd2f; continue;
 }
 0x409dd2f => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409dd40 => {
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409dd1f; continue; }
  pc = 0x409dd42; continue;
 }
 0x409dd42 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(4))) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.unimplemented(0x409dd45, "Fisttp");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x9fu8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x409dd49, "Fisttp");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xb6u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x409dd4d, "Fisttp");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xd1u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x409dd51, "Fisttp");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0xf0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x409dd55, "Fisttp");
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x56u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x409de0d; continue; }
  pc = 0x409dd74; continue;
 }
 0x409dd58 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x409de0d; continue; }
  pc = 0x409dd74; continue;
 }
 0x409dd74 => {
  cpu.poll(); pc = cpu.r32(0x409dd40u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x409dd7c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x409dd83); f_0408cef2(cpu); if cpu.returned_to != 0x409dd83 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409dd8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x409dd96); f_0408cef2(cpu); if cpu.returned_to != 0x409dd96 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409dd9f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x409ddad); f_0408cef2(cpu); if cpu.returned_to != 0x409ddad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409ddb6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x409ddc8); f_0408cef2(cpu); if cpu.returned_to != 0x409ddc8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409ddd1 => {
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
  cpu.push(0x409dde7); f_0408cef2(cpu); if cpu.returned_to != 0x409dde7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x409ddf0 => {
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
  cpu.push(0x409de0a); f_0408cef2(cpu); if cpu.returned_to != 0x409de0a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x409de0d; continue;
 }
 0x409de0d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409de20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409de20); }
 let mut pc: u32 = 0x409de20;
 loop { match pc {
 0x409de20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x409de31); f_04095444(cpu); if cpu.returned_to != 0x409de31 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409de40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409de40); }
 let mut pc: u32 = 0x409de40;
 loop { match pc {
 0x409de40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x409de49); f_040ad1d1(cpu); if cpu.returned_to != 0x409de49 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409de50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409de50); }
 let mut pc: u32 = 0x409de50;
 loop { match pc {
 0x409de50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x409de59); f_040ad14f(cpu); if cpu.returned_to != 0x409de59 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409de60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409de60); }
 let mut pc: u32 = 0x409de60;
 loop { match pc {
 0x409de60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x409de69); f_040ad06c(cpu); if cpu.returned_to != 0x409de69 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409de70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409de70); }
 let mut pc: u32 = 0x409de70;
 loop { match pc {
 0x409de70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x415ac88u32), 0x0u32); }
  if !cpu.zf { pc = 0x409de9e; continue; }
  pc = 0x409de7c; continue;
 }
 0x409de7c => {
  { let v = 0x409db70u32; cpu.push(v); }
  { let v = 0x409de70u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x409de90); f_040a30c0(cpu); if cpu.returned_to != 0x409de90 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415ac88u32, v); }
  cpu.push(0x409de9e); f_0409d6c0(cpu); if cpu.returned_to != 0x409de9e { return; }
  pc = 0x409de9e; continue;
 }
 0x409de9e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409dea2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409dea2); }
 let mut pc: u32 = 0x409dea2;
 loop { match pc {
 0x409dea2 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x415ac8cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409deaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409deaa); }
 let mut pc: u32 = 0x409deaa;
 loop { match pc {
 0x409deaa => {
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x409deb3); f_0409debd(cpu); if cpu.returned_to != 0x409deb3 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409debd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409debd); }
 let mut pc: u32 = 0x409debd;
 loop { match pc {
 0x409debd => {
  { cpu.sub32(cpu.eax, cpu.esp); }
  if !cpu.cf { pc = 0x409dece; continue; }
  pc = 0x409dec1; continue;
 }
 0x409dec1 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x415b81cu32)); }
  if (cpu.cf || cpu.zf) { pc = 0x409dece; continue; }
  pc = 0x409decd; continue;
 }
 0x409decd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409dece => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x415ac8cu32)); }
  if cpu.zf { f_0409dedb(cpu); return; }
  pc = 0x409deda; continue;
 }
 0x409deda => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409dedb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409dedb); }
 let mut pc: u32 = 0x409dedb;
 loop { match pc {
 0x409dedb => {
  { let v: u32 = 0x415ac8eu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x409deea); f_040ad382(cpu); if cpu.returned_to != 0x409deea { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_0409def0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409def0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409def0); }
 let mut pc: u32 = 0x409def0;
 loop { match pc {
 0x409def0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x42c567cu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x42c567cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409df41); f_04095444(cpu); if cpu.returned_to != 0x409df41 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.r32(0x42c567cu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409df54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409df54); }
 let mut pc: u32 = 0x409df54;
 loop { match pc {
 0x409df54 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409df85); f_04095444(cpu); if cpu.returned_to != 0x409df85 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409df94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409df94); }
 let mut pc: u32 = 0x409df94;
 loop { match pc {
 0x409df94 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409dfdc); f_04095444(cpu); if cpu.returned_to != 0x409dfdc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409dfec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409dfec); }
 let mut pc: u32 = 0x409dfec;
 loop { match pc {
 0x409dfec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409e034); f_04095444(cpu); if cpu.returned_to != 0x409e034 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e044(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e044); }
 let mut pc: u32 = 0x409e044;
 loop { match pc {
 0x409e044 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409e075); f_04095444(cpu); if cpu.returned_to != 0x409e075 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e080); }
 let mut pc: u32 = 0x409e080;
 loop { match pc {
 0x409e080 => {
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
  pc = 0x409e153; continue;
 }
 0x409e09d => {
  { let v: u32 = cpu.r32(0x42c5668u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c55ecu32); cpu.ebp = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c5660u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x42c5660u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42c5678u32); cpu.ebp = v; }
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
  { let v: u16 = cpu.r16(0x415aca2u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x12u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  cpu.push(0x409e147); f_0409df94(cpu); if cpu.returned_to != 0x409e147 { return; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1eu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x42c569au32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x409e153; continue;
 }
 0x409e153 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x415aca0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409e09d; continue; }
  pc = 0x409e163; continue;
 }
 0x409e163 => {
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
pub fn f_0409e170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e170); }
 let mut pc: u32 = 0x409e170;
 loop { match pc {
 0x409e170 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x42c5678u32); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x415aca4u32), 0x0u16); }
  if !cpu.zf { pc = 0x409e40a; continue; }
  pc = 0x409e192; continue;
 }
 0x409e192 => {
  { let v: u32 = 0x41300b5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4130000u32); cpu.ebx = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409e1a9); f_0408b85c(cpu); if cpu.returned_to != 0x409e1a9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5684u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409e1cb; continue; }
  pc = 0x409e1b2; continue;
 }
 0x409e1b2 => {
  { let v = 0x414af00u32; cpu.push(v); }
  { let v = 0x414af38u32; cpu.push(v); }
  cpu.push(0x409e1c1); f_04094528(cpu); if cpu.returned_to != 0x409e1c1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409e40f; continue;
 }
 0x409e1cb => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x4130000u32; cpu.eax = v; }
  cpu.push(0x409e1d7); f_0408ca18(cpu); if cpu.returned_to != 0x409e1d7 { return; }
  { let v: u32 = 0x4130008u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4130000u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x60u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5688u32, v); }
  pc = 0x409e1f0; continue;
 }
 0x409e1f0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x42c5694u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5690u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409e21e; continue; }
  pc = 0x409e219; continue;
 }
 0x409e219 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x409e22d; continue; }
  pc = 0x409e21e; continue;
 }
 0x409e21e => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0xcfu8); }
  if cpu.zf { pc = 0x409e22d; continue; }
  pc = 0x409e226; continue;
 }
 0x409e226 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x66u8); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x409e1f0; continue; }
  pc = 0x409e22d; continue;
 }
 0x409e22d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x42c569bu32, v); }
  { cpu.sub8((cpu.ecx as u8), 0x67u8); }
  if !cpu.zf { pc = 0x409e25b; continue; }
  pc = 0x409e238; continue;
 }
 0x409e238 => {
  { let v = 0x414af44u32; cpu.push(v); }
  { let v = 0x414af38u32; cpu.push(v); }
  cpu.push(0x409e247); f_04094528(cpu); if cpu.returned_to != 0x409e247 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409e25b => {
  { let v: u32 = 0x200u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x409e275); f_04095444(cpu); if cpu.returned_to != 0x409e275 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x42c5696u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x42c5698u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x42c5688u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x42c5688u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x409e2cf); f_04095444(cpu); if cpu.returned_to != 0x409e2cf { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x415aca0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x415aca4u32, v); }
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
  { let v: u16 = cpu.r16(0x415aca2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  cpu.push(0x409e32e); f_0408b85c(cpu); if cpu.returned_to != 0x409e32e { return; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5680u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409e360; continue; }
  pc = 0x409e33d; continue;
 }
 0x409e33d => {
  { let v = 0x414af84u32; cpu.push(v); }
  { let v = 0x414af38u32; cpu.push(v); }
  cpu.push(0x409e34c); f_04094528(cpu); if cpu.returned_to != 0x409e34c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409e360 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x415aca0u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c55ecu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5668u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c566cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5660u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5664u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1eu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x409e3c8; continue;
 }
 0x409e3b1 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x415aca2u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x12u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x409e3c8; continue;
 }
 0x409e3c8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x415aca0u32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409e3b1; continue; }
  pc = 0x409e3d5; continue;
 }
 0x409e3d5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x415aca2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5674u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x42c569bu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x42c567cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x42c566cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x42c5678u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x42c5653u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x409e40a; continue;
 }
 0x409e40a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409e40f; continue;
 }
 0x409e40f => {
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
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
pub fn f_0409e420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e420); }
 let mut pc: u32 = 0x409e420;
 loop { match pc {
 0x409e420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x415aca4u32), 0x0u16); }
  if cpu.zf { pc = 0x409e48a; continue; }
  pc = 0x409e42f; continue;
 }
 0x409e42f => {
  { let v: u32 = cpu.r32(0x42c5680u32); cpu.eax = v; }
  cpu.push(0x409e439); f_0408bee0(cpu); if cpu.returned_to != 0x409e439 { return; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  cpu.push(0x409e443); f_0408bee0(cpu); if cpu.returned_to != 0x409e443 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x42c5696u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x42c5698u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x409e47c); f_04095444(cpu); if cpu.returned_to != 0x409e47c { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x415aca4u32, v); }
  pc = 0x409e48a; continue;
 }
 0x409e48a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e490); }
 let mut pc: u32 = 0x409e490;
 loop { match pc {
 0x409e490 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409e4be); f_04095444(cpu); if cpu.returned_to != 0x409e4be { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e4cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e4cc); }
 let mut pc: u32 = 0x409e4cc;
 loop { match pc {
 0x409e4cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x42c55ecu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x42c55ecu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409e52d); f_04095444(cpu); if cpu.returned_to != 0x409e52d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.r32(0x42c55ecu32); cpu.eax = v; }
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
pub fn f_0409e548(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e548); }
 let mut pc: u32 = 0x409e548;
 loop { match pc {
 0x409e548 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x42c55ecu32), 0x0u32); }
  if cpu.zf { pc = 0x409e59c; continue; }
  pc = 0x409e556; continue;
 }
 0x409e556 => {
  { let v: u32 = cpu.r32(0x42c5684u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x42c55ecu32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x42c569bu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409e58f); f_04095444(cpu); if cpu.returned_to != 0x409e58f { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x42c5670u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x42c55ecu32, v); }
  pc = 0x409e59c; continue;
 }
 0x409e59c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e5a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e5a4); }
 let mut pc: u32 = 0x409e5a4;
 loop { match pc {
 0x409e5a4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x42c566cu32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5674u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  cpu.push(0x409e5c8); f_0408ca18(cpu); if cpu.returned_to != 0x409e5c8 { return; }
  { let v: u32 = cpu.r32(0x42c5674u32); cpu.esi = v; }
  cpu.push(0x409e5d3); f_0408f270(cpu); if cpu.returned_to != 0x409e5d3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x42c5653u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x42c566cu32); cpu.ebp = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x42c55ecu32); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x42c5664u32); cpu.esi = v; }
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
  pc = 0x409e62a; continue;
 }
 0x409e62a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1bu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409e62a; continue; }
  pc = 0x409e635; continue;
 }
 0x409e635 => {
  { let v: u32 = cpu.r32(0x42c5664u32); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x42c5674u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x42c5664u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x42c5674u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c566cu32, v); }
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
  cpu.push(0x409e69f); f_0409dfec(cpu); if cpu.returned_to != 0x409e69f { return; }
  { let v: u32 = cpu.r32(0x42c566cu32); cpu.ebp = v; }
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
pub fn f_0409e6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e6b0); }
 let mut pc: u32 = 0x409e6b0;
 loop { match pc {
 0x409e6b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x42c5678u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7fffffffu32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x409e701; continue;
 }
 0x409e6db => {
  { let v: u32 = cpu.r32(0x42c5668u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x409e6fa; continue; }
  pc = 0x409e6e8; continue;
 }
 0x409e6e8 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x409e6fa; continue; }
  pc = 0x409e6f4; continue;
 }
 0x409e6f4 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x409e6fa; continue;
 }
 0x409e6fa => {
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x409e701; continue;
 }
 0x409e701 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x415aca0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409e6db; continue; }
  pc = 0x409e70e; continue;
 }
 0x409e70e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x409e81c; continue; }
  pc = 0x409e71b; continue;
 }
 0x409e71b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x12u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x409e72f; continue; }
  pc = 0x409e72d; continue;
 }
 0x409e72d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x409e72f; continue;
 }
 0x409e72f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x409e74d; continue; }
  pc = 0x409e736; continue;
 }
 0x409e736 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5660u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x409e74d; continue;
 }
 0x409e74d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c5668u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x42c55ecu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x1eu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x42c5660u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x42c5660u32); cpu.ebx = v; }
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
  { let v: u16 = cpu.r16(0x415aca2u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5678u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x409e800); f_0409df94(cpu); if cpu.returned_to != 0x409e800 { return; }
  { let v: u32 = cpu.r32(0x42c5678u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x42c569au32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5678u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409e81c => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x42c5678u32, v); }
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
pub fn f_0409e830(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e830); }
 let mut pc: u32 = 0x409e830;
 loop { match pc {
 0x409e830 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e860; continue; }
  pc = 0x409e834; continue;
 }
 0x409e834 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x409e836; continue;
 }
 0x409e836 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x409e843; continue; }
  pc = 0x409e83a; continue;
 }
 0x409e83a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x409e836; continue; }
  pc = 0x409e843; continue;
 }
 0x409e843 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x409e84c); f_0409e867(cpu); if cpu.returned_to != 0x409e84c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e860; continue; }
  pc = 0x409e852; continue;
 }
 0x409e852 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e860; continue; }
  pc = 0x409e857; continue;
 }
 0x409e857 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e860; continue; }
  pc = 0x409e85d; continue;
 }
 0x409e85d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x409e860; continue;
 }
 0x409e860 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e867(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e867); }
 let mut pc: u32 = 0x409e867;
 loop { match pc {
 0x409e867 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e8d2; continue; }
  pc = 0x409e86b; continue;
 }
 0x409e86b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x409e877; continue; }
  pc = 0x409e86f; continue;
 }
 0x409e86f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x409e86b; continue; }
  pc = 0x409e877; continue;
 }
 0x409e877 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e8b7; continue; }
  pc = 0x409e87d; continue;
 }
 0x409e87d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e8a9; continue; }
  pc = 0x409e880; continue;
 }
 0x409e880 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x409e8a6; continue; }
  pc = 0x409e88e; continue;
 }
 0x409e88e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x409e880; continue; }
  pc = 0x409e8a3; continue;
 }
 0x409e8a3 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x409e8a6; continue;
 }
 0x409e8a6 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x409e8a9; continue;
 }
 0x409e8a9 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x409e8b7; continue;
 }
 0x409e8b7 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e8d2; continue; }
  pc = 0x409e8bd; continue;
 }
 0x409e8bd => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e8d2; continue; }
  pc = 0x409e8c5; continue;
 }
 0x409e8c5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x409e8d2; continue; }
  pc = 0x409e8cd; continue;
 }
 0x409e8cd => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x409e8d2; continue;
 }
 0x409e8d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e8e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e8e0); }
 let mut pc: u32 = 0x409e8e0;
 loop { match pc {
 0x409e8e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x409e8f5); f_0409e830(cpu); if cpu.returned_to != 0x409e8f5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e8f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e8f8); }
 let mut pc: u32 = 0x409e8f8;
 loop { match pc {
 0x409e8f8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x409e90e; continue; }
  pc = 0x409e901; continue;
 }
 0x409e901 => {
  { cpu.interrupt(0x1a); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x409e923; continue;
 }
 0x409e90e => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  if !cpu.zf { pc = 0x409e923; continue; }
  pc = 0x409e916; continue;
 }
 0x409e916 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x1a); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x409e923; continue;
 }
 0x409e923 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409e927(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e927); }
 let mut pc: u32 = 0x409e927;
 loop { match pc {
 0x409e927 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x414afb4u32; cpu.eax = v; }
  cpu.push(0x409e93c); f_040ad1d1(cpu); if cpu.returned_to != 0x409e93c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x409e949; continue; }
  pc = 0x409e942; continue;
 }
 0x409e942 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x409e980; continue;
 }
 0x409e949 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x409e954; continue; }
  pc = 0x409e94f; continue;
 }
 0x409e94f => {
  { let v: u32 = 0x414afc4u32; cpu.ebx = v; }
  pc = 0x409e954; continue;
 }
 0x409e954 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x409e963); f_040ad39f(cpu); if cpu.returned_to != 0x409e963 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x409e96f; continue; }
  pc = 0x409e968; continue;
 }
 0x409e968 => {
  { let v: u32 = 0x414afd0u32; cpu.eax = v; }
  pc = 0x409e974; continue;
 }
 0x409e96f => {
  { let v: u32 = 0x414afd4u32; cpu.eax = v; }
  pc = 0x409e974; continue;
 }
 0x409e974 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x409e97d); f_040ad72a(cpu); if cpu.returned_to != 0x409e97d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  pc = 0x409e980; continue;
 }
 0x409e980 => {
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
pub fn f_0409e987(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e987); }
 let mut pc: u32 = 0x409e987;
 loop { match pc {
 0x409e987 => {
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
pub fn f_0409e9ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e9ac); }
 let mut pc: u32 = 0x409e9ac;
 loop { match pc {
 0x409e9ac => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x415b83au32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x409e9c9; continue; }
  pc = 0x409e9bb; continue;
 }
 0x409e9bb => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x409e9c9; continue; }
  pc = 0x409e9c0; continue;
 }
 0x409e9c0 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2502u32; cpu.eax = v; }
  pc = 0x409e9d3; continue;
 }
 0x409e9c9 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x409e9d3; continue;
 }
 0x409e9d3 => {
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
pub fn f_0409e9de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409e9de); }
 let mut pc: u32 = 0x409e9de;
 loop { match pc {
 0x409e9de => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x409ea01; continue; }
  pc = 0x409e9ee; continue;
 }
 0x409e9ee => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x409ea01; continue; }
  pc = 0x409e9f3; continue;
 }
 0x409e9f3 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x409ea00); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409ea01 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x409ea0a); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ea0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea0b); }
 let mut pc: u32 = 0x409ea0b;
 loop { match pc {
 0x409ea0b => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ea0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea0d); }
 let mut pc: u32 = 0x409ea0d;
 loop { match pc {
 0x409ea0d => {
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
pub fn f_0409ea17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea17); }
 let mut pc: u32 = 0x409ea17;
 loop { match pc {
 0x409ea17 => {
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
pub fn f_0409ea1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea1f); }
 let mut pc: u32 = 0x409ea1f;
 loop { match pc {
 0x409ea1f => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ea21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea21); }
 let mut pc: u32 = 0x409ea21;
 loop { match pc {
 0x409ea21 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x409ea33); f_040ad740(cpu); if cpu.returned_to != 0x409ea33 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ea3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea3a); }
 let mut pc: u32 = 0x409ea3a;
 loop { match pc {
 0x409ea3a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x409ea55; continue; }
  pc = 0x409ea49; continue;
 }
 0x409ea49 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x409ea55; continue;
 }
 0x409ea55 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ea59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea59); }
 let mut pc: u32 = 0x409ea59;
 loop { match pc {
 0x409ea59 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x409ea3au32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x409ea79); f_0409ed88(cpu); if cpu.returned_to != 0x409ea79 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ea82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ea82); }
 let mut pc: u32 = 0x409ea82;
 loop { match pc {
 0x409ea82 => {
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
  cpu.push(0x409eaa2); f_0409ea59(cpu); if cpu.returned_to != 0x409eaa2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409eaa9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409eaa9); }
 let mut pc: u32 = 0x409eaa9;
 loop { match pc {
 0x409eaa9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x409eac4); f_040a171d(cpu); if cpu.returned_to != 0x409eac4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409eb5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409eb5a); }
 let mut pc: u32 = 0x409eb5a;
 loop { match pc {
 0x409eb5a => {
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
  pc = 0x409eb68; continue;
 }
 0x409eb68 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x409eb8c; continue; }
  pc = 0x409eb6e; continue;
 }
 0x409eb6e => {
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
  cpu.poll(); pc = 0x409eb68; continue;
 }
 0x409eb8c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x43300000u32; cpu.edi = v; }
  cpu.push(0x409eb98); f_0409eba5(cpu); if cpu.returned_to != 0x409eb98 { return; }
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
pub fn f_0409eba5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409eba5); }
 let mut pc: u32 = 0x409eba5;
 loop { match pc {
 0x409eba5 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.edx); cpu.esi = v; }
  if cpu.zf { pc = 0x409ebf5; continue; }
  pc = 0x409ebad; continue;
 }
 0x409ebad => {
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if !cpu.zf { pc = 0x409ebc9; continue; }
  pc = 0x409ebb5; continue;
 }
 0x409ebb5 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if cpu.zf { cpu.poll(); pc = 0x409ebb5; continue; }
  pc = 0x409ebc7; continue;
 }
 0x409ebc7 => {
  pc = 0x409ebed; continue;
 }
 0x409ebc9 => {
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if cpu.zf { pc = 0x409ebed; continue; }
  pc = 0x409ebd1; continue;
 }
 0x409ebd1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x409ebd1; continue; }
  pc = 0x409ebe5; continue;
 }
 0x409ebe5 => {
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x409ebed; continue;
 }
 0x409ebed => {
  { let v: u32 = cpu.edx & 0xfffffu32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edi); cpu.edx = v; }
  pc = 0x409ebf5; continue;
 }
 0x409ebf5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ebf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ebf6); }
 let mut pc: u32 = 0x409ebf6;
 loop { match pc {
 0x409ebf6 => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_0409ebfb(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ebfb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ebfb); }
 let mut pc: u32 = 0x409ebfb;
 loop { match pc {
 0x409ebfb => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x409ec03); f_040a12e1(cpu); if cpu.returned_to != 0x409ec03 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ec07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ec07); }
 let mut pc: u32 = 0x409ec07;
 loop { match pc {
 0x409ec07 => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_0409ebfb(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ec0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ec0e); }
 let mut pc: u32 = 0x409ec0e;
 loop { match pc {
 0x409ec0e => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x409ec18); f_0409ebfb(cpu); if cpu.returned_to != 0x409ec18 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ec1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ec1e); }
 let mut pc: u32 = 0x409ec1e;
 loop { match pc {
 0x409ec1e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x409ec26); f_040a12e7(cpu); if cpu.returned_to != 0x409ec26 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ec2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ec2a); }
 let mut pc: u32 = 0x409ec2a;
 loop { match pc {
 0x409ec2a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x409ec50; continue; }
  pc = 0x409ec3b; continue;
 }
 0x409ec3b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x409ec4d); f_0409ec6b(cpu); if cpu.returned_to != 0x409ec4d { return; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x409ec50; continue;
 }
 0x409ec50 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x409ec5a); f_040adb58(cpu); if cpu.returned_to != 0x409ec5a { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x409ec63); f_040adb1e(cpu); if cpu.returned_to != 0x409ec63 { return; }
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
pub fn f_0409ec6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ec6b); }
 let mut pc: u32 = 0x409ec6b;
 loop { match pc {
 0x409ec6b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x414b938u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x409ec96; continue; }
  pc = 0x409ec86; continue;
 }
 0x409ec86 => {
  { let v: u32 = cpu.r32(0x414b948u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x414b94cu32); cpu.eax = v; }
  pc = 0x409ecb3; continue;
 }
 0x409ec96 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x414b940u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x409ecb7; continue; }
  pc = 0x409eca5; continue;
 }
 0x409eca5 => {
  { let v: u32 = cpu.r32(0x414b950u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x414b954u32); cpu.eax = v; }
  pc = 0x409ecb3; continue;
 }
 0x409ecb3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x409ecb7; continue;
 }
 0x409ecb7 => {
  { let v: u32 = 0x414b9d8u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x409ecfa; continue; }
  pc = 0x409ecc0; continue;
 }
 0x409ecc0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x409ed18; continue; }
  pc = 0x409eccc; continue;
 }
 0x409eccc => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x415b8f0u32), 0x1u8); }
  if cpu.zf { pc = 0x409ecec; continue; }
  pc = 0x409ecd9; continue;
 }
 0x409ecd9 => {
  { let v = cpu.r32(0x414b9deu32); cpu.push(v); }
  { let v = cpu.r32(0x414b9dau32); cpu.push(v); }
  cpu.push(0x409ecea); f_040a2c6c(cpu); if cpu.returned_to != 0x409ecea { return; }
  pc = 0x409ecf2; continue;
 }
 0x409ecec => {
  { let v = cpu.st(0) / cpu.rf64(0x414b9dau32); cpu.set_st(0, v); }
  pc = 0x409ecf2; continue;
 }
 0x409ecf2 => {
  { let v: u32 = cpu.edx.wrapping_sub(0xd8u32); cpu.edx = v; }
  pc = 0x409ed14; continue;
 }
 0x409ecfa => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x409ed18; continue; }
  pc = 0x409ed04; continue;
 }
 0x409ed04 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x414b9dau32); cpu.set_st(0, v); }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x409ed14; continue;
 }
 0x409ed14 => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x409ed18; continue;
 }
 0x409ed18 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x409ed26; continue;
 }
 0x409ed26 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x409ed38; continue; }
  pc = 0x409ed2d; continue;
 }
 0x409ed2d => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x409ed38; continue;
 }
 0x409ed38 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409ed49; continue; }
  pc = 0x409ed3c; continue;
 }
 0x409ed3c => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x409ed26; continue; }
  pc = 0x409ed44; continue;
 }
 0x409ed44 => {
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.poll(); pc = 0x409ed26; continue;
 }
 0x409ed49 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x409ed6e; continue; }
  pc = 0x409ed4d; continue;
 }
 0x409ed4d => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x415b8f0u32), 0x1u8); }
  if cpu.zf { pc = 0x409ed69; continue; }
  pc = 0x409ed5a; continue;
 }
 0x409ed5a => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x409ed67); f_040a2c6c(cpu); if cpu.returned_to != 0x409ed67 { return; }
  pc = 0x409ed75; continue;
 }
 0x409ed69 => {
  { let v = cpu.st(0) / cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x409ed75; continue;
 }
 0x409ed6e => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x409ed75; continue;
 }
 0x409ed75 => {
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
pub fn f_0409ed88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ed88); }
 let mut pc: u32 = 0x409ed88;
 loop { match pc {
 0x409ed88 => {
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
  pc = 0x409ee5a; continue;
 }
 0x409edc7 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x409edd5); f_0409f127(cpu); if cpu.returned_to != 0x409edd5 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x115u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x409f115; continue; }
  pc = 0x409ede9; continue;
 }
 0x409ede9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x409ef24; continue; }
  pc = 0x409edfb; continue;
 }
 0x409edfb => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409ee91; continue; }
  pc = 0x409ee23; continue;
 }
 0x409ee23 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409ee41; continue; }
  pc = 0x409ee2b; continue;
 }
 0x409ee2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x409ee35; continue;
 }
 0x409ee35 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409ee5a; continue;
 }
 0x409ee41 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409ee86; continue; }
  pc = 0x409ee48; continue;
 }
 0x409ee48 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x409ee51; continue;
 }
 0x409ee51 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x409ee5a; continue;
 }
 0x409ee5a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x409f115; continue; }
  pc = 0x409ee64; continue;
 }
 0x409ee64 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x25u32); }
  if cpu.zf { cpu.poll(); pc = 0x409edc7; continue; }
  pc = 0x409ee74; continue;
 }
 0x409ee74 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409ee84); cpu.call(t); if cpu.returned_to != 0x409ee84 { return; }
  cpu.poll(); pc = 0x409ee5a; continue;
 }
 0x409ee86 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x409ee51; continue;
 }
 0x409ee91 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409eeec; continue; }
  pc = 0x409eea3; continue;
 }
 0x409eea3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409eec2; continue; }
  pc = 0x409eeab; continue;
 }
 0x409eeab => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.eax); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x409ee5a; continue;
 }
 0x409eec2 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409eee1; continue; }
  pc = 0x409eec9; continue;
 }
 0x409eec9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x409eed2; continue;
 }
 0x409eed2 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x409ee5a; continue;
 }
 0x409eee1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.poll(); pc = 0x409eed2; continue;
 }
 0x409eeec => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x409ef01; continue; }
  pc = 0x409eef2; continue;
 }
 0x409eef2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  cpu.poll(); pc = 0x409ee35; continue;
 }
 0x409ef01 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409ef16; continue; }
  pc = 0x409ef08; continue;
 }
 0x409ef08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.poll(); pc = 0x409ee51; continue;
 }
 0x409ef16 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.poll(); pc = 0x409ee51; continue;
 }
 0x409ef24 => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x409ef3b); f_0409f465(cpu); if cpu.returned_to != 0x409ef3b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x409ef87; continue; }
  pc = 0x409ef4f; continue;
 }
 0x409ef4f => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x409ef87; continue; }
  pc = 0x409ef5b; continue;
 }
 0x409ef5b => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x409ef87; continue; }
  pc = 0x409ef6e; continue;
 }
 0x409ef6e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409ef85); cpu.call(t); if cpu.returned_to != 0x409ef85 { return; }
  cpu.poll(); pc = 0x409ef5b; continue;
 }
 0x409ef87 => {
  { let v: u32 = 0x117u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x409ef8e; continue;
 }
 0x409ef8e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x409efa9; continue; }
  pc = 0x409ef94; continue;
 }
 0x409ef94 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409efa7); cpu.call(t); if cpu.returned_to != 0x409efa7 { return; }
  cpu.poll(); pc = 0x409ef8e; continue;
 }
 0x409efa9 => {
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409efd3; continue; }
  pc = 0x409efbe; continue;
 }
 0x409efbe => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409efd1); cpu.call(t); if cpu.returned_to != 0x409efd1 { return; }
  cpu.poll(); pc = 0x409efa9; continue;
 }
 0x409efd3 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x409f01b; continue; }
  pc = 0x409efe3; continue;
 }
 0x409efe3 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.zf { pc = 0x409f01b; continue; }
  pc = 0x409efef; continue;
 }
 0x409efef => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x409f01b; continue; }
  pc = 0x409f002; continue;
 }
 0x409f002 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409f019); cpu.call(t); if cpu.returned_to != 0x409f019 { return; }
  cpu.poll(); pc = 0x409efef; continue;
 }
 0x409f01b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.zf { pc = 0x409f02e; continue; }
  pc = 0x409f029; continue;
 }
 0x409f029 => {
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x409f09a; continue; }
  pc = 0x409f02e; continue;
 }
 0x409f02e => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f06f; continue; }
  pc = 0x409f03e; continue;
 }
 0x409f03e => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409f0c5; continue; }
  pc = 0x409f057; continue;
 }
 0x409f057 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409f06a); cpu.call(t); if cpu.returned_to != 0x409f06a { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x409f03e; continue;
 }
 0x409f06f => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409f0c5; continue; }
  pc = 0x409f084; continue;
 }
 0x409f084 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409f097); cpu.call(t); if cpu.returned_to != 0x409f097 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x409f06f; continue;
 }
 0x409f09a => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409f0c5; continue; }
  pc = 0x409f0af; continue;
 }
 0x409f0af => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409f0c2); cpu.call(t); if cpu.returned_to != 0x409f0c2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x409f09a; continue;
 }
 0x409f0c5 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x409ee5a; continue; }
  pc = 0x409f0d9; continue;
 }
 0x409f0d9 => {
  { cpu.sub32(cpu.r32(0x104u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x409ee5a; continue; }
  pc = 0x409f0e7; continue;
 }
 0x409f0e7 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x409ee5a; continue; }
  pc = 0x409f100; continue;
 }
 0x409f100 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x409f113); cpu.call(t); if cpu.returned_to != 0x409f113 { return; }
  cpu.poll(); pc = 0x409f0e7; continue;
 }
 0x409f115 => {
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
pub fn f_0409f127(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f127); }
 let mut pc: u32 = 0x409f127;
 loop { match pc {
 0x409f127 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x409f13b); f_0409f265(cpu); if cpu.returned_to != 0x409f13b { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x409f175; continue; }
  pc = 0x409f14f; continue;
 }
 0x409f14f => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x409f175; continue; }
  pc = 0x409f154; continue;
 }
 0x409f154 => {
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
  if (cpu.sf != cpu.of) { pc = 0x409f1a0; continue; }
  pc = 0x409f16e; continue;
 }
 0x409f16e => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x409f154; continue; }
  pc = 0x409f173; continue;
 }
 0x409f173 => {
  pc = 0x409f1a0; continue;
 }
 0x409f175 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x409f1a0; continue; }
  pc = 0x409f17e; continue;
 }
 0x409f17e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x409f19f; continue; }
  pc = 0x409f18f; continue;
 }
 0x409f18f => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8) | 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409f19f; continue;
 }
 0x409f19f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x409f1a0; continue;
 }
 0x409f1a0 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x409f1f9; continue; }
  pc = 0x409f1b0; continue;
 }
 0x409f1b0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x409f1dc; continue; }
  pc = 0x409f1c1; continue;
 }
 0x409f1c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x409f1d9; continue; }
  pc = 0x409f1d2; continue;
 }
 0x409f1d2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409f1d9; continue;
 }
 0x409f1d9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x409f1f9; continue;
 }
 0x409f1dc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x409f1f9; continue; }
  pc = 0x409f1e5; continue;
 }
 0x409f1e5 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x409f1f9; continue; }
  pc = 0x409f1ea; continue;
 }
 0x409f1ea => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x409f1dc; continue;
 }
 0x409f1f9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4eu8); }
  if cpu.cf { pc = 0x409f21e; continue; }
  pc = 0x409f203; continue;
 }
 0x409f203 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f25b; continue; }
  pc = 0x409f205; continue;
 }
 0x409f205 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x409f215; continue; }
  pc = 0x409f20a; continue;
 }
 0x409f20a => {
  if (cpu.cf || cpu.zf) { pc = 0x409f22e; continue; }
  pc = 0x409f20c; continue;
 }
 0x409f20c => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x409f22e; continue; }
  pc = 0x409f211; continue;
 }
 0x409f211 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409f215 => {
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.zf { pc = 0x409f23c; continue; }
  pc = 0x409f21a; continue;
 }
 0x409f21a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409f21e => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x409f261; continue; }
  pc = 0x409f223; continue;
 }
 0x409f223 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f251; continue; }
  pc = 0x409f225; continue;
 }
 0x409f225 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x409f242; continue; }
  pc = 0x409f22a; continue;
 }
 0x409f22a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409f22e => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x409f235; continue;
 }
 0x409f235 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409f23c => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x10u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409f25f; continue;
 }
 0x409f242 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409f251 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.poll(); pc = 0x409f235; continue;
 }
 0x409f25b => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409f25f; continue;
 }
 0x409f25f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x409f261; continue;
 }
 0x409f261 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f265(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f265); }
 let mut pc: u32 = 0x409f265;
 loop { match pc {
 0x409f265 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x409f26d; continue;
 }
 0x409f26d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x409f27c; continue; }
  pc = 0x409f276; continue;
 }
 0x409f276 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x409f2bc; continue;
 }
 0x409f27c => {
  { cpu.sub32(cpu.eax, 0x23u32); }
  if !cpu.zf { pc = 0x409f287; continue; }
  pc = 0x409f281; continue;
 }
 0x409f281 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x409f2bc; continue;
 }
 0x409f287 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x409f29c; continue; }
  pc = 0x409f28c; continue;
 }
 0x409f28c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x409f2bc; continue;
 }
 0x409f29c => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x409f2b3; continue; }
  pc = 0x409f2a1; continue;
 }
 0x409f2a1 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x409f2bc; continue; }
  pc = 0x409f2ad; continue;
 }
 0x409f2ad => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x2u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x409f2bc; continue;
 }
 0x409f2b3 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x409f2bf; continue; }
  pc = 0x409f2b8; continue;
 }
 0x409f2b8 => {
  { let v: u8 = 0x30u8; cpu.w8(0x16u32.wrapping_add(cpu.edx), v); }
  pc = 0x409f2bc; continue;
 }
 0x409f2bc => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x409f26d; continue;
 }
 0x409f2bf => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f2c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409f2c4); } r_0409f2c4(cpu, 0x409f2c4); }
pub fn f_0409f2e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409f2e8); } r_0409f2c4(cpu, 0x409f2e8); }
fn r_0409f2c4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409f2c4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409f2d2; continue;
 }
 0x409f2d2 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x409f2e3; continue; }
  pc = 0x409f2dc; continue;
 }
 0x409f2dc => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x409f2e3; continue; }
  pc = 0x409f2e0; continue;
 }
 0x409f2e0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x409f2d2; continue;
 }
 0x409f2e3 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409f2e8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x409f2f6; continue;
 }
 0x409f2f6 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x409f2e3; continue; }
  pc = 0x409f304; continue;
 }
 0x409f304 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x409f2e3; continue; }
  pc = 0x409f308; continue;
 }
 0x409f308 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x409f2f6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f30b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f30b); }
 let mut pc: u32 = 0x409f30b;
 loop { match pc {
 0x409f30b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x409f323); f_0409d834(cpu); if cpu.returned_to != 0x409f323 { return; }
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
  pc = 0x409f348; continue;
 }
 0x409f341 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x409f348; continue;
 }
 0x409f348 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x409f341; continue; }
  pc = 0x409f34c; continue;
 }
 0x409f34c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x409f34f; continue;
 }
 0x409f34f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x409f35a; continue; }
  pc = 0x409f353; continue;
 }
 0x409f353 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x409f34f; continue;
 }
 0x409f35a => {
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
pub fn f_0409f369(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f369); }
 let mut pc: u32 = 0x409f369;
 loop { match pc {
 0x409f369 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x409f385; continue; }
  pc = 0x409f37a; continue;
 }
 0x409f37a => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x409f385; continue;
 }
 0x409f385 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x409f392; continue; }
  pc = 0x409f38b; continue;
 }
 0x409f38b => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f392; continue;
 }
 0x409f392 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x409f3a5); f_0409d834(cpu); if cpu.returned_to != 0x409f3a5 { return; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x409f3a7; continue;
 }
 0x409f3a7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x409f3b4; continue; }
  pc = 0x409f3b0; continue;
 }
 0x409f3b0 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.poll(); pc = 0x409f3a7; continue;
 }
 0x409f3b4 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x409f3e6; continue; }
  pc = 0x409f3ba; continue;
 }
 0x409f3ba => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x409f3de; continue;
 }
 0x409f3c3 => {
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
  pc = 0x409f3de; continue;
 }
 0x409f3de => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x409f3c3; continue; }
  pc = 0x409f3e3; continue;
 }
 0x409f3e3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x409f3e6; continue;
 }
 0x409f3e6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f457; continue; }
  pc = 0x409f3f9; continue;
 }
 0x409f3f9 => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x409f436; continue; }
  pc = 0x409f3fd; continue;
 }
 0x409f3fd => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409f403; continue;
 }
 0x409f403 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x409f413; continue; }
  pc = 0x409f40f; continue;
 }
 0x409f40f => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x409f403; continue;
 }
 0x409f413 => {
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x409f42d; continue; }
  pc = 0x409f418; continue;
 }
 0x409f418 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x409f421; continue;
 }
 0x409f421 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x409f42d; continue; }
  pc = 0x409f42a; continue;
 }
 0x409f42a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x409f421; continue;
 }
 0x409f42d => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x409f457; continue;
 }
 0x409f436 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x409f441; continue; }
  pc = 0x409f440; continue;
 }
 0x409f440 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x409f441; continue;
 }
 0x409f441 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if cpu.zf { pc = 0x409f452; continue; }
  pc = 0x409f44a; continue;
 }
 0x409f44a => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x409f457; continue;
 }
 0x409f452 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.poll(); pc = 0x409f3f9; continue;
 }
 0x409f457 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f45e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f45e); }
 let mut pc: u32 = 0x409f45e;
 loop { match pc {
 0x409f45e => {
  let t = cpu.r32(0x415c73cu32); cpu.push(0x409f464); cpu.call(t); if cpu.returned_to != 0x409f464 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f465(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f465); }
 let mut pc: u32 = 0x409f465;
 loop { match pc {
 0x409f465 => {
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
  if cpu.cf { pc = 0x409f4ae; continue; }
  pc = 0x409f492; continue;
 }
 0x409f492 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f4ff; continue; }
  pc = 0x409f498; continue;
 }
 0x409f498 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x409f4a9; continue; }
  pc = 0x409f49d; continue;
 }
 0x409f49d => {
  if (cpu.cf || cpu.zf) { pc = 0x409f4bc; continue; }
  pc = 0x409f49f; continue;
 }
 0x409f49f => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  pc = 0x409f4a2; continue;
 }
 0x409f4a2 => {
  if cpu.zf { pc = 0x409f4bc; continue; }
  pc = 0x409f4a4; continue;
 }
 0x409f4a4 => {
  pc = 0x409f509; continue;
 }
 0x409f4a9 => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  cpu.poll(); pc = 0x409f4a2; continue;
 }
 0x409f4ae => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x409f509; continue; }
  pc = 0x409f4b3; continue;
 }
 0x409f4b3 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f4bc; continue; }
  pc = 0x409f4b5; continue;
 }
 0x409f4b5 => {
  { cpu.sub8((cpu.edx as u8), 0x64u8); }
  if cpu.zf { pc = 0x409f4ff; continue; }
  pc = 0x409f4ba; continue;
 }
 0x409f4ba => {
  pc = 0x409f509; continue;
 }
 0x409f4bc => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f4d6; continue; }
  pc = 0x409f4ca; continue;
 }
 0x409f4ca => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  pc = 0x409f4fb; continue;
 }
 0x409f4d6 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f4f1; continue; }
  pc = 0x409f4e4; continue;
 }
 0x409f4e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  pc = 0x409f4fb; continue;
 }
 0x409f4f1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x409f4fb; continue;
 }
 0x409f4fb => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f4ff; continue;
 }
 0x409f4ff => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x409f509; continue; }
  pc = 0x409f505; continue;
 }
 0x409f505 => {
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f509; continue;
 }
 0x409f509 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.cf { pc = 0x409f576; continue; }
  pc = 0x409f516; continue;
 }
 0x409f516 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f5f6; continue; }
  pc = 0x409f51c; continue;
 }
 0x409f51c => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  if cpu.cf { pc = 0x409f55d; continue; }
  pc = 0x409f521; continue;
 }
 0x409f521 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f60c; continue; }
  pc = 0x409f527; continue;
 }
 0x409f527 => {
  { cpu.sub8((cpu.edx as u8), 0x73u8); }
  if cpu.cf { pc = 0x409f54f; continue; }
  pc = 0x409f52c; continue;
 }
 0x409f52c => {
  if (cpu.cf || cpu.zf) { pc = 0x409f6a1; continue; }
  pc = 0x409f532; continue;
 }
 0x409f532 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x409f873; continue; }
  pc = 0x409f53b; continue;
 }
 0x409f53b => {
  if (cpu.cf || cpu.zf) { pc = 0x409f78d; continue; }
  pc = 0x409f541; continue;
 }
 0x409f541 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  if cpu.zf { pc = 0x409f76a; continue; }
  pc = 0x409f54a; continue;
 }
 0x409f54a => {
  pc = 0x409f873; continue;
 }
 0x409f54f => {
  { cpu.sub8((cpu.edx as u8), 0x70u8); }
  if cpu.zf { pc = 0x409f7b0; continue; }
  pc = 0x409f558; continue;
 }
 0x409f558 => {
  pc = 0x409f873; continue;
 }
 0x409f55d => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.cf { pc = 0x409f5c4; continue; }
  pc = 0x409f562; continue;
 }
 0x409f562 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f5f6; continue; }
  pc = 0x409f568; continue;
 }
 0x409f568 => {
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.zf { pc = 0x409f650; continue; }
  pc = 0x409f571; continue;
 }
 0x409f571 => {
  pc = 0x409f873; continue;
 }
 0x409f576 => {
  { cpu.sub8((cpu.edx as u8), 0x50u8); }
  if cpu.cf { pc = 0x409f5ae; continue; }
  pc = 0x409f57b; continue;
 }
 0x409f57b => {
  if (cpu.cf || cpu.zf) { pc = 0x409f7b0; continue; }
  pc = 0x409f581; continue;
 }
 0x409f581 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x409f5a0; continue; }
  pc = 0x409f586; continue;
 }
 0x409f586 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f76a; continue; }
  pc = 0x409f58c; continue;
 }
 0x409f58c => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x409f873; continue; }
  pc = 0x409f595; continue;
 }
 0x409f595 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f847; continue; }
  pc = 0x409f59b; continue;
 }
 0x409f59b => {
  pc = 0x409f650; continue;
 }
 0x409f5a0 => {
  { cpu.sub8((cpu.edx as u8), 0x53u8); }
  if cpu.zf { pc = 0x409f6a1; continue; }
  pc = 0x409f5a9; continue;
 }
 0x409f5a9 => {
  pc = 0x409f873; continue;
 }
 0x409f5ae => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x409f5bf; continue; }
  pc = 0x409f5b3; continue;
 }
 0x409f5b3 => {
  if (cpu.cf || cpu.zf) { pc = 0x409f5c4; continue; }
  pc = 0x409f5b5; continue;
 }
 0x409f5b5 => {
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  pc = 0x409f5b8; continue;
 }
 0x409f5b8 => {
  if cpu.zf { pc = 0x409f5f6; continue; }
  pc = 0x409f5ba; continue;
 }
 0x409f5ba => {
  pc = 0x409f873; continue;
 }
 0x409f5bf => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  cpu.poll(); pc = 0x409f5b8; continue;
 }
 0x409f5c4 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f5f6; continue; }
  pc = 0x409f5d2; continue;
 }
 0x409f5d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x409f5e5); f_0409f369(cpu); if cpu.returned_to != 0x409f5e5 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x409f646; continue;
 }
 0x409f5f6 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x409f601); f_0409f45e(cpu); if cpu.returned_to != 0x409f601 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x409f646; continue;
 }
 0x409f60c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f621; continue; }
  pc = 0x409f61d; continue;
 }
 0x409f61d => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x409f621; continue;
 }
 0x409f621 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x409f62a); f_040add65(cpu); if cpu.returned_to != 0x409f62a { return; }
  pc = 0x409f62a; continue;
 }
 0x409f62a => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x409f63d; continue; }
  pc = 0x409f630; continue;
 }
 0x409f630 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x409f63d; continue; }
  pc = 0x409f639; continue;
 }
 0x409f639 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409f63d; continue;
 }
 0x409f63d => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x409f646; continue;
 }
 0x409f646 => {
  cpu.push(0x409f64b); f_0409f2c4(cpu); if cpu.returned_to != 0x409f64b { return; }
  pc = 0x409f89c; continue;
 }
 0x409f650 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f671; continue; }
  pc = 0x409f65e; continue;
 }
 0x409f65e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x409f66a; continue;
 }
 0x409f66a => {
  cpu.push(0x409f66f); f_040add65(cpu); if cpu.returned_to != 0x409f66f { return; }
  cpu.poll(); pc = 0x409f62a; continue;
 }
 0x409f671 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f68e; continue; }
  pc = 0x409f67f; continue;
 }
 0x409f67f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.poll(); pc = 0x409f66a; continue;
 }
 0x409f68e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x409f69f); f_0409d834(cpu); if cpu.returned_to != 0x409f69f { return; }
  cpu.poll(); pc = 0x409f62a; continue;
 }
 0x409f6a1 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f6d2; continue; }
  pc = 0x409f6af; continue;
 }
 0x409f6af => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x409f6c6; continue; }
  pc = 0x409f6c1; continue;
 }
 0x409f6c1 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x409f6ca; continue; }
  pc = 0x409f6c6; continue;
 }
 0x409f6c6 => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  pc = 0x409f708; continue;
 }
 0x409f6ca => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x409f706; continue;
 }
 0x409f6d2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x40u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f6f2; continue; }
  pc = 0x409f6e0; continue;
 }
 0x409f6e0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x409f6ca; continue; }
  pc = 0x409f6ee; continue;
 }
 0x409f6ee => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  pc = 0x409f706; continue;
 }
 0x409f6f2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x409f6ee; continue; }
  pc = 0x409f700; continue;
 }
 0x409f700 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x409f706; continue;
 }
 0x409f706 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  pc = 0x409f708; continue;
 }
 0x409f708 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xf9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x409f742; continue; }
  pc = 0x409f720; continue;
 }
 0x409f720 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f739; continue; }
  pc = 0x409f72b; continue;
 }
 0x409f72b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x409f89c; continue;
 }
 0x409f739 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x409f89c; continue;
 }
 0x409f742 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f75e; continue; }
  pc = 0x409f74d; continue;
 }
 0x409f74d => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x409f759); f_0409f2e8(cpu); if cpu.returned_to != 0x409f759 { return; }
  pc = 0x409f89c; continue;
 }
 0x409f75e => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.poll(); pc = 0x409f646; continue;
 }
 0x409f76a => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f788; continue; }
  pc = 0x409f776; continue;
 }
 0x409f776 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x409f788; continue; }
  pc = 0x409f77a; continue;
 }
 0x409f77a => {
  { let v: u8 = 0x30u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f788; continue;
 }
 0x409f788 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x409f78d; continue;
 }
 0x409f78d => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x409f796); f_040add1b(cpu); if cpu.returned_to != 0x409f796 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { cpu.poll(); pc = 0x409f62a; continue; }
  pc = 0x409f7a4; continue;
 }
 0x409f7a4 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x409f7ab); f_0409f94e(cpu); if cpu.returned_to != 0x409f7ab { return; }
  cpu.poll(); pc = 0x409f62a; continue;
 }
 0x409f7b0 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x409f7d4; continue; }
  pc = 0x409f7b6; continue;
 }
 0x409f7b6 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f7cd; continue; }
  pc = 0x409f7c4; continue;
 }
 0x409f7c4 => {
  { let v: u32 = 0xdu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f7d4; continue;
 }
 0x409f7cd => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f7d4; continue;
 }
 0x409f7d4 => {
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
  if cpu.zf { pc = 0x409f81f; continue; }
  pc = 0x409f7f6; continue;
 }
 0x409f7f6 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x409f80f); f_0409f30b(cpu); if cpu.returned_to != 0x409f80f { return; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x409f828; continue;
 }
 0x409f81f => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x409f828; continue;
 }
 0x409f828 => {
  cpu.push(0x409f82d); f_0409f30b(cpu); if cpu.returned_to != 0x409f82d { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if !cpu.zf { cpu.poll(); pc = 0x409f63d; continue; }
  pc = 0x409f83b; continue;
 }
 0x409f83b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x409f842); f_0409f94e(cpu); if cpu.returned_to != 0x409f842 { return; }
  cpu.poll(); pc = 0x409f63d; continue;
 }
 0x409f847 => {
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
  pc = 0x409f89c; continue;
 }
 0x409f873 => {
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
  pc = 0x409f89c; continue;
 }
 0x409f89c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x409f8f1; continue; }
  pc = 0x409f8a2; continue;
 }
 0x409f8a2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x409f8b4; continue; }
  pc = 0x409f8ac; continue;
 }
 0x409f8ac => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x409f8da; continue;
 }
 0x409f8b4 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f8c8; continue; }
  pc = 0x409f8c2; continue;
 }
 0x409f8c2 => {
  { let v: u8 = 0x20u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f8da; continue;
 }
 0x409f8c8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409f8de; continue; }
  pc = 0x409f8d6; continue;
 }
 0x409f8d6 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f8da; continue;
 }
 0x409f8da => {
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f8de; continue;
 }
 0x409f8de => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x409f8ea; continue; }
  pc = 0x409f8e5; continue;
 }
 0x409f8e5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f8f1; continue;
 }
 0x409f8ea => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x409f8f1; continue;
 }
 0x409f8f1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x409f908; continue; }
  pc = 0x409f8fb; continue;
 }
 0x409f8fb => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f908; continue;
 }
 0x409f908 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x409f914; continue; }
  pc = 0x409f910; continue;
 }
 0x409f910 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x409f921; continue; }
  pc = 0x409f914; continue;
 }
 0x409f914 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if cpu.zf { pc = 0x409f921; continue; }
  pc = 0x409f91e; continue;
 }
 0x409f91e => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x409f921; continue;
 }
 0x409f921 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x409f930); f_0409f2c4(cpu); if cpu.returned_to != 0x409f930 { return; }
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
pub fn f_0409f94e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f94e); }
 let mut pc: u32 = 0x409f94e;
 loop { match pc {
 0x409f94e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x409f951; continue;
 }
 0x409f951 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x409f964; continue; }
  pc = 0x409f956; continue;
 }
 0x409f956 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x409f95f); f_04094274(cpu); if cpu.returned_to != 0x409f95f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x409f951; continue;
 }
 0x409f964 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f970); }
 let mut pc: u32 = 0x409f970;
 loop { match pc {
 0x409f970 => {
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
pub fn f_0409f990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f990); }
 let mut pc: u32 = 0x409f990;
 loop { match pc {
 0x409f990 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x409f995); cpu.call(t); if cpu.returned_to != 0x409f995 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f997(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f997); }
 let mut pc: u32 = 0x409f997;
 loop { match pc {
 0x409f997 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x409f99d); cpu.call(t); if cpu.returned_to != 0x409f99d { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409f99f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409f99f); }
 let mut pc: u32 = 0x409f99f;
 loop { match pc {
 0x409f99f => {
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
  pc = 0x409f9be; continue;
 }
 0x409f9be => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  if cpu.zf { pc = 0x409fbc6; continue; }
  pc = 0x409f9d5; continue;
 }
 0x409f9d5 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409f9fb; continue; }
  pc = 0x409f9ed; continue;
 }
 0x409f9ed => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409f9f4); f_0409fcdb(cpu); if cpu.returned_to != 0x409f9f4 { return; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x409fb86; continue;
 }
 0x409f9fb => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x409fa25; continue; }
  pc = 0x409fa00; continue;
 }
 0x409fa00 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fa07); f_0409f990(cpu); if cpu.returned_to != 0x409fa07 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x409fb85; continue; }
  pc = 0x409fa0f; continue;
 }
 0x409fa0f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x409fbc6; continue; }
  pc = 0x409fa19; continue;
 }
 0x409fa19 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x409fa20); f_0409f997(cpu); if cpu.returned_to != 0x409fa20 { return; }
  pc = 0x409fbc6; continue;
 }
 0x409fa25 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x409fa2e); f_0409fbe6(cpu); if cpu.returned_to != 0x409fa2e { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x409fa3d; continue; }
  pc = 0x409fa39; continue;
 }
 0x409fa39 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x409fa3d; continue;
 }
 0x409fa3d => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if cpu.cf { pc = 0x409fa97; continue; }
  pc = 0x409fa42; continue;
 }
 0x409fa42 => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if (cpu.cf || cpu.zf) { pc = 0x409fb23; continue; }
  pc = 0x409fa4b; continue;
 }
 0x409fa4b => {
  { cpu.sub32(cpu.ebx, 0x70u32); }
  if cpu.cf { pc = 0x409fa7d; continue; }
  pc = 0x409fa50; continue;
 }
 0x409fa50 => {
  if (cpu.cf || cpu.zf) { pc = 0x409fb17; continue; }
  pc = 0x409fa56; continue;
 }
 0x409fa56 => {
  { cpu.sub32(cpu.ebx, 0x75u32); }
  if cpu.cf { pc = 0x409fa6f; continue; }
  pc = 0x409fa5b; continue;
 }
 0x409fa5b => {
  if (cpu.cf || cpu.zf) { pc = 0x409fb0a; continue; }
  pc = 0x409fa61; continue;
 }
 0x409fa61 => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x409fb17; continue; }
  pc = 0x409fa6a; continue;
 }
 0x409fa6a => {
  pc = 0x409fb86; continue;
 }
 0x409fa6f => {
  { cpu.sub32(cpu.ebx, 0x73u32); }
  if cpu.zf { pc = 0x409fb2e; continue; }
  pc = 0x409fa78; continue;
 }
 0x409fa78 => {
  pc = 0x409fb86; continue;
 }
 0x409fa7d => {
  { cpu.sub32(cpu.ebx, 0x6eu32); }
  if cpu.cf { pc = 0x409fa8d; continue; }
  pc = 0x409fa82; continue;
 }
 0x409fa82 => {
  if (cpu.cf || cpu.zf) { pc = 0x409fb69; continue; }
  pc = 0x409fa88; continue;
 }
 0x409fa88 => {
  pc = 0x409fafe; continue;
 }
 0x409fa8d => {
  { cpu.sub32(cpu.ebx, 0x69u32); }
  if cpu.zf { pc = 0x409faf1; continue; }
  pc = 0x409fa92; continue;
 }
 0x409fa92 => {
  pc = 0x409fb86; continue;
 }
 0x409fa97 => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if cpu.cf { pc = 0x409fabd; continue; }
  pc = 0x409fa9c; continue;
 }
 0x409fa9c => {
  if (cpu.cf || cpu.zf) { pc = 0x409fb17; continue; }
  pc = 0x409faa2; continue;
 }
 0x409faa2 => {
  { cpu.sub32(cpu.ebx, 0x63u32); }
  if cpu.cf { pc = 0x409faaf; continue; }
  pc = 0x409faa7; continue;
 }
 0x409faa7 => {
  if (cpu.cf || cpu.zf) { pc = 0x409fb46; continue; }
  pc = 0x409faad; continue;
 }
 0x409faad => {
  pc = 0x409fadc; continue;
 }
 0x409faaf => {
  { cpu.sub32(cpu.ebx, 0x5bu32); }
  if cpu.zf { pc = 0x409fb39; continue; }
  pc = 0x409fab8; continue;
 }
 0x409fab8 => {
  pc = 0x409fb86; continue;
 }
 0x409fabd => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.cf { pc = 0x409face; continue; }
  pc = 0x409fac2; continue;
 }
 0x409fac2 => {
  if (cpu.cf || cpu.zf) { pc = 0x409fb23; continue; }
  pc = 0x409fac4; continue;
 }
 0x409fac4 => {
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x409fb23; continue; }
  pc = 0x409fac9; continue;
 }
 0x409fac9 => {
  pc = 0x409fb86; continue;
 }
 0x409face => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x409fb76; continue; }
  pc = 0x409fad7; continue;
 }
 0x409fad7 => {
  pc = 0x409fb86; continue;
 }
 0x409fadc => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x409fae6; continue;
 }
 0x409fae6 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x409faea; continue;
 }
 0x409faea => {
  cpu.push(0x409faef); f_040a0384(cpu); if cpu.returned_to != 0x409faef { return; }
  pc = 0x409fb4f; continue;
 }
 0x409faf1 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.poll(); pc = 0x409faea; continue;
 }
 0x409fafe => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x409fae6; continue;
 }
 0x409fb0a => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x409faea; continue;
 }
 0x409fb17 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x409fae6; continue;
 }
 0x409fb23 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fb2c); f_040a006d(cpu); if cpu.returned_to != 0x409fb2c { return; }
  pc = 0x409fb4f; continue;
 }
 0x409fb2e => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fb37); f_0409fdaf(cpu); if cpu.returned_to != 0x409fb37 { return; }
  pc = 0x409fb4f; continue;
 }
 0x409fb39 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fb44); f_0409ff73(cpu); if cpu.returned_to != 0x409fb44 { return; }
  pc = 0x409fb4f; continue;
 }
 0x409fb46 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fb4f); f_0409fd1a(cpu); if cpu.returned_to != 0x409fb4f { return; }
  pc = 0x409fb4f; continue;
 }
 0x409fb4f => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x409fbc6; continue; }
  pc = 0x409fb59; continue;
 }
 0x409fb59 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x409fb86; continue; }
  pc = 0x409fb63; continue;
 }
 0x409fb63 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x409fb86; continue;
 }
 0x409fb69 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fb74); f_0409fed2(cpu); if cpu.returned_to != 0x409fb74 { return; }
  pc = 0x409fb86; continue;
 }
 0x409fb76 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fb7d); f_0409f990(cpu); if cpu.returned_to != 0x409fb7d { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x409fa0f; continue; }
  pc = 0x409fb85; continue;
 }
 0x409fb85 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x409fb86; continue;
 }
 0x409fb86 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x409f9be; continue; }
  pc = 0x409fb90; continue;
 }
 0x409fb90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x25u32); }
  if !cpu.zf { pc = 0x409fbc6; continue; }
  pc = 0x409fb9f; continue;
 }
 0x409fb9f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x409fbac); f_0409fbe6(cpu); if cpu.returned_to != 0x409fbac { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x409fbc6; continue; }
  pc = 0x409fbbb; continue;
 }
 0x409fbbb => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x409fbc6); f_0409fed2(cpu); if cpu.returned_to != 0x409fbc6 { return; }
  pc = 0x409fbc6; continue;
 }
 0x409fbc6 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x409fbda; continue; }
  pc = 0x409fbcd; continue;
 }
 0x409fbcd => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x409fbda; continue; }
  pc = 0x409fbd3; continue;
 }
 0x409fbd3 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x409fbde; continue;
 }
 0x409fbda => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x409fbde; continue;
 }
 0x409fbde => {
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
pub fn f_0409fbe6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409fbe6); }
 let mut pc: u32 = 0x409fbe6;
 loop { match pc {
 0x409fbe6 => {
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
  if !cpu.zf { pc = 0x409fc19; continue; }
  pc = 0x409fc0f; continue;
 }
 0x409fc0f => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409fc19; continue;
 }
 0x409fc19 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409fc6b; continue; }
  pc = 0x409fc3c; continue;
 }
 0x409fc3c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x409fc3e; continue;
 }
 0x409fc3e => {
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
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x409fc3e; continue; }
  pc = 0x409fc68; continue;
 }
 0x409fc68 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x409fc6b; continue;
 }
 0x409fc6b => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x4eu32); }
  if !cpu.zf { pc = 0x409fc7f; continue; }
  pc = 0x409fc77; continue;
 }
 0x409fc77 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x409fc8f; continue;
 }
 0x409fc7f => {
  { cpu.sub32(cpu.edx, 0x46u32); }
  if !cpu.zf { pc = 0x409fc8f; continue; }
  pc = 0x409fc84; continue;
 }
 0x409fc84 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409fc8f; continue;
 }
 0x409fc8f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.cf { pc = 0x409fca9; continue; }
  pc = 0x409fc99; continue;
 }
 0x409fc99 => {
  if (cpu.cf || cpu.zf) { pc = 0x409fcb0; continue; }
  pc = 0x409fc9b; continue;
 }
 0x409fc9b => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x409fcd4; continue; }
  pc = 0x409fca0; continue;
 }
 0x409fca0 => {
  if (cpu.cf || cpu.zf) { pc = 0x409fcbd; continue; }
  pc = 0x409fca2; continue;
 }
 0x409fca2 => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x409fcbd; continue; }
  pc = 0x409fca7; continue;
 }
 0x409fca7 => {
  pc = 0x409fcd4; continue;
 }
 0x409fca9 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x409fcc9; continue; }
  pc = 0x409fcae; continue;
 }
 0x409fcae => {
  pc = 0x409fcd4; continue;
 }
 0x409fcb0 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x10u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409fcd4; continue;
 }
 0x409fcbd => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409fcd4; continue;
 }
 0x409fcc9 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409fcd4; continue;
 }
 0x409fcd4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409fcdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409fcdb); }
 let mut pc: u32 = 0x409fcdb;
 loop { match pc {
 0x409fcdb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x409fce2; continue;
 }
 0x409fce2 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409fce9); f_0409f990(cpu); if cpu.returned_to != 0x409fce9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x409fd07; continue; }
  pc = 0x409fd04; continue;
 }
 0x409fd04 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x409fce2; continue;
 }
 0x409fd07 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x409fd14; continue; }
  pc = 0x409fd0d; continue;
 }
 0x409fd0d => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x409fd14); f_0409f997(cpu); if cpu.returned_to != 0x409fd14 { return; }
  pc = 0x409fd14; continue;
 }
 0x409fd14 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409fd1a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409fd1a); } r_0409fd1a(cpu, 0x409fd1a); }
pub fn f_0409fdaf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x409fdaf); } r_0409fd1a(cpu, 0x409fdaf); }
pub fn f_040a006d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a006d); } r_0409fd1a(cpu, 0x40a006d); }
fn r_0409fd1a(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x409fd1a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x409fd61; continue; }
  pc = 0x409fd2a; continue;
 }
 0x409fd2a => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x409fd3e; continue; }
  pc = 0x409fd32; continue;
 }
 0x409fd32 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x409fd61; continue;
 }
 0x409fd3e => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x409fd53; continue; }
  pc = 0x409fd43; continue;
 }
 0x409fd43 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x409fd61; continue;
 }
 0x409fd53 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x409fd61; continue;
 }
 0x409fd61 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x409fd70; continue; }
  pc = 0x409fd6b; continue;
 }
 0x409fd6b => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x409fd70; continue;
 }
 0x409fd70 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x409fda4; continue; }
  pc = 0x409fd74; continue;
 }
 0x409fd74 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409fd7b); f_0409f990(cpu); if cpu.returned_to != 0x409fd7b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x409fda4; continue; }
  pc = 0x409fd85; continue;
 }
 0x409fd85 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x409fd70; continue; }
  pc = 0x409fd8b; continue;
 }
 0x409fd8b => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x409fd9b; continue; }
  pc = 0x409fd8f; continue;
 }
 0x409fd8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x409fd70; continue;
 }
 0x409fd9b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x409fd70; continue;
 }
 0x409fda4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x409fda9; continue;
 }
 0x409fda9 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409fdaf => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x409fdc6; continue; }
  pc = 0x409fdbf; continue;
 }
 0x409fdbf => {
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x409fdcf; continue;
 }
 0x409fdc6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x10u8); }
  { let v: u8 = 0x1u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x409fdcf; continue;
 }
 0x409fdcf => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x409fe08; continue; }
  pc = 0x409fdd6; continue;
 }
 0x409fdd6 => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x409fde6; continue; }
  pc = 0x409fdda; continue;
 }
 0x409fdda => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x409fe08; continue;
 }
 0x409fde6 => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x409fdfa; continue; }
  pc = 0x409fdea; continue;
 }
 0x409fdea => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x409fe08; continue;
 }
 0x409fdfa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x409fe08; continue;
 }
 0x409fe08 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x409fe0a; continue;
 }
 0x409fe0a => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409fe11); f_0409f990(cpu); if cpu.returned_to != 0x409fe11 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x409fe30; continue; }
  pc = 0x409fe2d; continue;
 }
 0x409fe2d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x409fe0a; continue;
 }
 0x409fe30 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if cpu.zf { pc = 0x409fe3d; continue; }
  pc = 0x409fe36; continue;
 }
 0x409fe36 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x409fea7; continue;
 }
 0x409fe3d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409fe9d; continue; }
  pc = 0x409fe4a; continue;
 }
 0x409fe4a => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x409fe75; continue; }
  pc = 0x409fe53; continue;
 }
 0x409fe53 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x409fe66; continue; }
  pc = 0x409fe5e; continue;
 }
 0x409fe5e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409fe6d; continue;
 }
 0x409fe66 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409fe6d; continue;
 }
 0x409fe6d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x409fe75; continue;
 }
 0x409fe75 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x409fe7c); f_040a059b(cpu); if cpu.returned_to != 0x409fe7c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x409fea7; continue; }
  pc = 0x409fe84; continue;
 }
 0x409fe84 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x409fe4a; continue; }
  pc = 0x409fe9d; continue;
 }
 0x409fe9d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x409fea7); f_0409f997(cpu); if cpu.returned_to != 0x409fea7 { return; }
  pc = 0x409fea7; continue;
 }
 0x409fea7 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x409fec8; continue; }
  pc = 0x409fead; continue;
 }
 0x409fead => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x409fec8; continue; }
  pc = 0x409feb1; continue;
 }
 0x409feb1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x409fec2; continue; }
  pc = 0x409febc; continue;
 }
 0x409febc => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409fec8; continue;
 }
 0x409fec2 => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409fec8; continue;
 }
 0x409fec8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x409fda9; continue;
 }
 0x40a006d => {
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
  pc = 0x40a0084; continue;
 }
 0x40a0084 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40a008b); f_0409f990(cpu); if cpu.returned_to != 0x40a008b { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x40a00ab; continue; }
  pc = 0x40a00a5; continue;
 }
 0x40a00a5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x60u32.wrapping_add(cpu.esp))); cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a0084; continue;
 }
 0x40a00ab => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a00b5; continue;
 }
 0x40a00b5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a02b5; continue; }
  pc = 0x40a00c6; continue;
 }
 0x40a00c6 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x40a00d0; continue; }
  pc = 0x40a00cb; continue;
 }
 0x40a00cb => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x40a00ee; continue; }
  pc = 0x40a00d0; continue;
 }
 0x40a00d0 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a00dd); f_040a059b(cpu); if cpu.returned_to != 0x40a00dd { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a00ee; continue;
 }
 0x40a00ee => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x40a010f; continue; }
  pc = 0x40a0106; continue;
 }
 0x40a0106 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x40a02b5; continue; }
  pc = 0x40a010f; continue;
 }
 0x40a010f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a0177; continue; }
  pc = 0x40a012f; continue;
 }
 0x40a012f => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x40a0134; continue;
 }
 0x40a0134 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x40a014e; continue; }
  pc = 0x40a013f; continue;
 }
 0x40a013f => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6au32.wrapping_add(cpu.esp)), 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6au32.wrapping_add(cpu.esp), v); }
  pc = 0x40a014e; continue;
 }
 0x40a014e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x40a0156); f_040a059b(cpu); if cpu.returned_to != 0x40a0156 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a0161; continue;
 }
 0x40a0161 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40a0134; continue; }
  pc = 0x40a0177; continue;
 }
 0x40a0177 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x40a0227; continue; }
  pc = 0x40a0180; continue;
 }
 0x40a0180 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a0189); f_040a059b(cpu); if cpu.returned_to != 0x40a0189 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a0195; continue;
 }
 0x40a0195 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x40a01b3; continue; }
  pc = 0x40a0199; continue;
 }
 0x40a0199 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a02b5; continue; }
  pc = 0x40a01b3; continue;
 }
 0x40a01b3 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x40a01b4; continue;
 }
 0x40a01b4 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a01de; continue; }
  pc = 0x40a01cc; continue;
 }
 0x40a01cc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a01d6); f_040a059b(cpu); if cpu.returned_to != 0x40a01d6 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a01b4; continue; }
  pc = 0x40a01de; continue;
 }
 0x40a01de => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x40a021e; continue; }
  pc = 0x40a01e4; continue;
 }
 0x40a01e4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a01ec; continue;
 }
 0x40a01ec => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x40a0215; continue; }
  pc = 0x40a01f7; continue;
 }
 0x40a01f7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a01ec; continue;
 }
 0x40a0215 => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a021e; continue;
 }
 0x40a021e => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a0227; continue;
 }
 0x40a0227 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x40a02b5; continue; }
  pc = 0x40a0231; continue;
 }
 0x40a0231 => {
  { cpu.sub32(cpu.ecx, 0x65u32); }
  if cpu.zf { pc = 0x40a023f; continue; }
  pc = 0x40a0236; continue;
 }
 0x40a0236 => {
  { cpu.sub32(cpu.ecx, 0x45u32); }
  if !cpu.zf { pc = 0x40a02b5; continue; }
  pc = 0x40a023f; continue;
 }
 0x40a023f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a0249); f_040a059b(cpu); if cpu.returned_to != 0x40a0249 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a0255; continue;
 }
 0x40a0255 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x40a025f; continue; }
  pc = 0x40a025a; continue;
 }
 0x40a025a => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x40a0271; continue; }
  pc = 0x40a025f; continue;
 }
 0x40a025f => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a0269); f_040a059b(cpu); if cpu.returned_to != 0x40a0269 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a0271; continue;
 }
 0x40a0271 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x40a028d; continue; }
  pc = 0x40a0289; continue;
 }
 0x40a0289 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a02b5; continue;
 }
 0x40a028d => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x40a0297); f_040a059b(cpu); if cpu.returned_to != 0x40a0297 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a02be; continue; }
  pc = 0x40a029f; continue;
 }
 0x40a029f => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40a028d; continue; }
  pc = 0x40a02b5; continue;
 }
 0x40a02b5 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a02be); f_0409f997(cpu); if cpu.returned_to != 0x40a02be { return; }
  pc = 0x40a02be; continue;
 }
 0x40a02be => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a0379; continue; }
  pc = 0x40a02c6; continue;
 }
 0x40a02c6 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a0379; continue; }
  pc = 0x40a02d8; continue;
 }
 0x40a02d8 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x40a02f1; continue; }
  pc = 0x40a02e1; continue;
 }
 0x40a02e1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x40a02fd; continue; }
  pc = 0x40a02eb; continue;
 }
 0x40a02eb => {
  { let v: u32 = cpu.neg32(cpu.r32(0x68u32.wrapping_add(cpu.esp))); cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a02fd; continue;
 }
 0x40a02f1 => {
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  let t = cpu.r32(0x415c740u32); cpu.push(0x40a02fd); cpu.call(t); if cpu.returned_to != 0x40a02fd { return; }
  pc = 0x40a02fd; continue;
 }
 0x40a02fd => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40a0315; continue; }
  pc = 0x40a0305; continue;
 }
 0x40a0305 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebx = v; cpu.set_segment(0, selector); }
  pc = 0x40a0340; continue;
 }
 0x40a0315 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x40a032e; continue; }
  pc = 0x40a031a; continue;
 }
 0x40a031a => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  pc = 0x40a0340; continue;
 }
 0x40a032e => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  pc = 0x40a0340; continue;
 }
 0x40a0340 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x40a034e; continue; }
  pc = 0x40a0348; continue;
 }
 0x40a0348 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x40a0376; continue;
 }
 0x40a034e => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x40a0358; continue; }
  pc = 0x40a0353; continue;
 }
 0x40a0353 => {
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x40a0369; continue; }
  pc = 0x40a0358; continue;
 }
 0x40a0358 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40a0379; continue;
 }
 0x40a0369 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x40a0376); f_040add80(cpu); if cpu.returned_to != 0x40a0376 { return; }
  pc = 0x40a0376; continue;
 }
 0x40a0376 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40a0379; continue;
 }
 0x40a0379 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x409fda9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409fed2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409fed2); }
 let mut pc: u32 = 0x409fed2;
 loop { match pc {
 0x409fed2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x409ff2b; continue; }
  pc = 0x409fede; continue;
 }
 0x409fede => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x409feef; continue; }
  pc = 0x409fee3; continue;
 }
 0x409fee3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x409ff14; continue;
 }
 0x409feef => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x409ff04; continue; }
  pc = 0x409fef4; continue;
 }
 0x409fef4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x409ff14; continue;
 }
 0x409ff04 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x409ff14; continue;
 }
 0x409ff14 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x409ff25; continue; }
  pc = 0x409ff1c; continue;
 }
 0x409ff1c => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x409ff25 => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x409ff2b; continue;
 }
 0x409ff2b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ff30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ff30); }
 let mut pc: u32 = 0x409ff30;
 loop { match pc {
 0x409ff30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x409ff45); f_0409e8e0(cpu); if cpu.returned_to != 0x409ff45 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ff6d; continue; }
  pc = 0x409ff4e; continue;
 }
 0x409ff4e => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x414b958u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x409ff6d; continue; }
  pc = 0x409ff67; continue;
 }
 0x409ff67 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x5du32); }
  if !cpu.zf { cpu.poll(); pc = 0x409ff4e; continue; }
  pc = 0x409ff6d; continue;
 }
 0x409ff6d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0409ff73(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x409ff73); }
 let mut pc: u32 = 0x409ff73;
 loop { match pc {
 0x409ff73 => {
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
  if cpu.zf { pc = 0x409ff9a; continue; }
  pc = 0x409ff97; continue;
 }
 0x409ff97 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x409ff9a; continue;
 }
 0x409ff9a => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x409ffa3); f_0409ff30(cpu); if cpu.returned_to != 0x409ffa3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x409ffe1; continue; }
  pc = 0x409ffad; continue;
 }
 0x409ffad => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x409ffbe; continue; }
  pc = 0x409ffb2; continue;
 }
 0x409ffb2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.esi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(0, selector); }
  pc = 0x409ffe1; continue;
 }
 0x409ffbe => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x409ffd3; continue; }
  pc = 0x409ffc3; continue;
 }
 0x409ffc3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x409ffe1; continue;
 }
 0x409ffd3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.esi = v; }
  pc = 0x409ffe1; continue;
 }
 0x409ffe1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x409ffe6; continue;
 }
 0x409ffe6 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x40a0054; continue; }
  pc = 0x409ffea; continue;
 }
 0x409ffea => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x409fff1); f_0409f990(cpu); if cpu.returned_to != 0x409fff1 { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x40a0054; continue; }
  pc = 0x409fffd; continue;
 }
 0x409fffd => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x414b958u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebp); }
  { let v: u8 = (cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.zf { pc = 0x40a0040; continue; }
  pc = 0x40a0037; continue;
 }
 0x40a0037 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x40a003e); f_0409f997(cpu); if cpu.returned_to != 0x40a003e { return; }
  pc = 0x40a0054; continue;
 }
 0x40a0040 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x409ffe6; continue; }
  pc = 0x40a004a; continue;
 }
 0x40a004a => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x409ffe6; continue;
 }
 0x40a0054 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x40a0062; continue; }
  pc = 0x40a005a; continue;
 }
 0x40a005a => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a0062; continue; }
  pc = 0x40a005e; continue;
 }
 0x40a005e => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40a0062; continue;
 }
 0x40a0062 => {
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
pub fn f_040a0384(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0384); }
 let mut pc: u32 = 0x40a0384;
 loop { match pc {
 0x40a0384 => {
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
  pc = 0x40a039e; continue;
 }
 0x40a039e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40a03a5); f_0409f990(cpu); if cpu.returned_to != 0x40a03a5 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x414b82cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a03c0; continue; }
  pc = 0x40a03bb; continue;
 }
 0x40a03bb => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a039e; continue;
 }
 0x40a03c0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a03ca; continue;
 }
 0x40a03ca => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a04f3; continue; }
  pc = 0x40a03db; continue;
 }
 0x40a03db => {
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x40a040f; continue; }
  pc = 0x40a03ec; continue;
 }
 0x40a03ec => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x40a03f6; continue; }
  pc = 0x40a03f1; continue;
 }
 0x40a03f1 => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if !cpu.zf { pc = 0x40a040f; continue; }
  pc = 0x40a03f6; continue;
 }
 0x40a03f6 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x40a0404); f_040a059b(cpu); if cpu.returned_to != 0x40a0404 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a040f; continue;
 }
 0x40a040f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x40a046b; continue; }
  pc = 0x40a0413; continue;
 }
 0x40a0413 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x40a0464; continue; }
  pc = 0x40a0418; continue;
 }
 0x40a0418 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x40a0424); f_040a059b(cpu); if cpu.returned_to != 0x40a0424 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a042f; continue;
 }
 0x40a042f => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x40a0439; continue; }
  pc = 0x40a0434; continue;
 }
 0x40a0434 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x40a045d; continue; }
  pc = 0x40a0439; continue;
 }
 0x40a0439 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  cpu.push(0x40a0443); f_040a059b(cpu); if cpu.returned_to != 0x40a0443 { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a0456; continue;
 }
 0x40a0456 => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x40a04af; continue;
 }
 0x40a045d => {
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x40a04af; continue;
 }
 0x40a0464 => {
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x40a04af; continue;
 }
 0x40a046b => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x40a04af; continue; }
  pc = 0x40a0470; continue;
 }
 0x40a0470 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x40a04af; continue; }
  pc = 0x40a0475; continue;
 }
 0x40a0475 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x40a0481); f_040a059b(cpu); if cpu.returned_to != 0x40a0481 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a048c; continue;
 }
 0x40a048c => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x40a0496; continue; }
  pc = 0x40a0491; continue;
 }
 0x40a0491 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x40a04af; continue; }
  pc = 0x40a0496; continue;
 }
 0x40a0496 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x40a04a0); f_040a059b(cpu); if cpu.returned_to != 0x40a04a0 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a04a8; continue;
 }
 0x40a04a8 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a04af; continue;
 }
 0x40a04af => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40a04b6); f_040a0574(cpu); if cpu.returned_to != 0x40a04b6 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40a04c9; continue; }
  pc = 0x40a04ba; continue;
 }
 0x40a04ba => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x40a04c7); f_040a059b(cpu); if cpu.returned_to != 0x40a04c7 { return; }
  cpu.poll(); pc = 0x40a04a8; continue;
 }
 0x40a04c9 => {
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if !cpu.zf { pc = 0x40a04f3; continue; }
  pc = 0x40a04ce; continue;
 }
 0x40a04ce => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x40a04f3; continue; }
  pc = 0x40a04d4; continue;
 }
 0x40a04d4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x40a04dc); f_040a059b(cpu); if cpu.returned_to != 0x40a04dc { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a04fc; continue; }
  pc = 0x40a04e3; continue;
 }
 0x40a04e3 => {
  cpu.push(0x40a04e8); f_040a0574(cpu); if cpu.returned_to != 0x40a04e8 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40a04f3; continue; }
  pc = 0x40a04ec; continue;
 }
 0x40a04ec => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  cpu.poll(); pc = 0x40a04d4; continue;
 }
 0x40a04f3 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40a04fc); f_0409f997(cpu); if cpu.returned_to != 0x40a04fc { return; }
  pc = 0x40a04fc; continue;
 }
 0x40a04fc => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x2du32); }
  if !cpu.zf { pc = 0x40a0505; continue; }
  pc = 0x40a0503; continue;
 }
 0x40a0503 => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x40a0505; continue;
 }
 0x40a0505 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a056a; continue; }
  pc = 0x40a0509; continue;
 }
 0x40a0509 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a056a; continue; }
  pc = 0x40a0516; continue;
 }
 0x40a0516 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x40a052b; continue; }
  pc = 0x40a051b; continue;
 }
 0x40a051b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  pc = 0x40a0556; continue;
 }
 0x40a052b => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x40a0544; continue; }
  pc = 0x40a0530; continue;
 }
 0x40a0530 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x40a0556; continue;
 }
 0x40a0544 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x40a0556; continue;
 }
 0x40a0556 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x40a0564; continue; }
  pc = 0x40a055e; continue;
 }
 0x40a055e => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40a056a; continue;
 }
 0x40a0564 => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x40a056a; continue;
 }
 0x40a056a => {
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
pub fn f_040a0574(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0574); }
 let mut pc: u32 = 0x40a0574;
 loop { match pc {
 0x40a0574 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a0582; continue; }
  pc = 0x40a0579; continue;
 }
 0x40a0579 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a0582; continue; }
  pc = 0x40a057e; continue;
 }
 0x40a057e => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a0582 => {
  cpu.push(0x40a0587); f_04095434(cpu); if cpu.returned_to != 0x40a0587 { return; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a0595; continue; }
  pc = 0x40a058c; continue;
 }
 0x40a058c => {
  { cpu.sub32(cpu.eax, 0x66u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a0595; continue; }
  pc = 0x40a0591; continue;
 }
 0x40a0591 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x57u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a0595 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a059b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a059b); }
 let mut pc: u32 = 0x40a059b;
 loop { match pc {
 0x40a059b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a05b9; continue; }
  pc = 0x40a05ac; continue;
 }
 0x40a05ac => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40a05b3); f_0409f990(cpu); if cpu.returned_to != 0x40a05b3 { return; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x2u8); }
  if cpu.zf { pc = 0x40a05be; continue; }
  pc = 0x40a05b9; continue;
 }
 0x40a05b9 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a05be; continue;
 }
 0x40a05be => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a05c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a05c4); }
 let mut pc: u32 = 0x40a05c4;
 loop { match pc {
 0x40a05c4 => {
  { let v: u32 = cpu.r32(0x415b794u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a067e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a067e); }
 let mut pc: u32 = 0x40a067e;
 loop { match pc {
 0x40a067e => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a068b); f_040addcc(cpu); if cpu.returned_to != 0x40a068b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x40a06a0); f_040ae206(cpu); if cpu.returned_to != 0x40a06a0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a06a5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a06a5); }
 let mut pc: u32 = 0x40a06a5;
 loop { match pc {
 0x40a06a5 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(24); }
  cpu.push(0x40a06ae); f_040ae366(cpu); if cpu.returned_to != 0x40a06ae { return; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xfffffff8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21u32; cpu.push(v); }
  { let v: u8 = 0x35u8; cpu.w8(0xffffffe9u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a06c8); f_040a0bb6(cpu); if cpu.returned_to != 0x40a06c8 { return; }
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
pub fn f_040a0718(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0718); }
 let mut pc: u32 = 0x40a0718;
 loop { match pc {
 0x40a0718 => {
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
  cpu.push(0x40a074a); f_040a067e(cpu); if cpu.returned_to != 0x40a074a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x40a075a; continue; }
  pc = 0x40a0754; continue;
 }
 0x40a0754 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a075a => {
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
pub fn f_040a076b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a076b); } r_040a076b(cpu, 0x40a076b); }
pub fn f_040a07d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a07d4); } r_040a076b(cpu, 0x40a07d4); }
fn r_040a076b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40a076b => {
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
  pc = 0x40a078a; continue;
 }
 0x40a078a => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a0793); f_040a067e(cpu); if cpu.returned_to != 0x40a0793 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a07d4 => {
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
  cpu.poll(); pc = 0x40a078a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a07a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a07a4); }
 let mut pc: u32 = 0x40a07a4;
 loop { match pc {
 0x40a07a4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x6u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a07c0); f_040a067e(cpu); if cpu.returned_to != 0x40a07c0 { return; }
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
pub fn f_040a07f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a07f5); }
 let mut pc: u32 = 0x40a07f5;
 loop { match pc {
 0x40a07f5 => {
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
  cpu.push(0x40a0819); f_040a067e(cpu); if cpu.returned_to != 0x40a0819 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x40a0827; continue; }
  pc = 0x40a0823; continue;
 }
 0x40a0823 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40a0880; continue;
 }
 0x40a0827 => {
  { let v: u32 = cpu.r16(0xffffffe4u32.wrapping_add(cpu.ebp)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.ebx, 0x100000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x40a0861; continue; }
  pc = 0x40a0833; continue;
 }
 0x40a0833 => {
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x9u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x8092u16; cpu.w16(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a084f); f_040a067e(cpu); if cpu.returned_to != 0x40a084f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x40a085b); f_040a0718(cpu); if cpu.returned_to != 0x40a085b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a0880; continue; }
  pc = 0x40a0861; continue;
 }
 0x40a0861 => {
  { let v: u32 = (cpu.edi as u16) as u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a086d); f_040a076b(cpu); if cpu.returned_to != 0x40a086d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a0880; continue; }
  pc = 0x40a0871; continue;
 }
 0x40a0871 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a087a); f_040a07d4(cpu); if cpu.returned_to != 0x40a087a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a0880; continue; }
  pc = 0x40a087e; continue;
 }
 0x40a087e => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40a0880; continue;
 }
 0x40a0880 => {
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
pub fn f_040a09d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a09d5); }
 let mut pc: u32 = 0x40a09d5;
 loop { match pc {
 0x40a09d5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a09e7); f_040addcc(cpu); if cpu.returned_to != 0x40a09e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  cpu.push(0x40a09f3); f_040a07a4(cpu); if cpu.returned_to != 0x40a09f3 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x40a0a07; continue; }
  pc = 0x40a09fd; continue;
 }
 0x40a09fd => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a0a07; continue; }
  pc = 0x40a0a01; continue;
 }
 0x40a0a01 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a0a07 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40a0a11); f_040a0718(cpu); if cpu.returned_to != 0x40a0a11 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a0a21; continue; }
  pc = 0x40a0a17; continue;
 }
 0x40a0a17 => {
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x40a0a37; continue; }
  pc = 0x40a0a1f; continue;
 }
 0x40a0a1f => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x40a0a21; continue;
 }
 0x40a0a21 => {
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  cpu.push(0x40a0a2f); f_040a07d4(cpu); if cpu.returned_to != 0x40a0a2f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a0a37; continue; }
  pc = 0x40a0a33; continue;
 }
 0x40a0a33 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x40a0a37; continue;
 }
 0x40a0a37 => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0a3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0a3b); }
 let mut pc: u32 = 0x40a0a3b;
 loop { match pc {
 0x40a0a3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x415b7b7u32), 0x0u32); }
  if !cpu.zf { pc = 0x40a0a5b; continue; }
  pc = 0x40a0a47; continue;
 }
 0x40a0a47 => {
  { let v = 0xfffffu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x40a0a53); f_040a0ca6(cpu); if cpu.returned_to != 0x40a0a53 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x415b7b7u32, v); }
  pc = 0x40a0a5b; continue;
 }
 0x40a0a5b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415b7b7u32); cpu.eax = v; }
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
pub fn f_040a0bb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0bb6); }
 let mut pc: u32 = 0x40a0bb6;
 loop { match pc {
 0x40a0bb6 => {
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
  cpu.push(0x40a0bd5); f_0409e8e0(cpu); if cpu.returned_to != 0x40a0bd5 { return; }
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
  cpu.push(0x40a0c16); f_040addcc(cpu); if cpu.returned_to != 0x40a0c16 { return; }
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
  cpu.push(0x40a0c4c); f_040ae206(cpu); if cpu.returned_to != 0x40a0c4c { return; }
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
pub fn f_040a0ca6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0ca6); }
 let mut pc: u32 = 0x40a0ca6;
 loop { match pc {
 0x40a0ca6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x40a0cb4); f_040a07f5(cpu); if cpu.returned_to != 0x40a0cb4 { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0d18(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0d18); }
 let mut pc: u32 = 0x40a0d18;
 loop { match pc {
 0x40a0d18 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x40ae375u32; cpu.edx = v; }
  { let v: u32 = 0x40ae4beu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415c73cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c740u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0d64); }
 let mut pc: u32 = 0x40a0d64;
 loop { match pc {
 0x40a0d64 => {
  { let v: u8 = 0x3u8; cpu.w8(0x41544dcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x415b804u32.wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x40a0d72, "Smsw");
  { let v: u16 = cpu.r16(0x415b802u32.wrapping_add(cpu.bases[3])) & 0x6u16; cpu.w16(0x415b802u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x40a0d85, "Sidt");
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x415b83cu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x40a0db6; continue; }
  pc = 0x40a0d9c; continue;
 }
 0x40a0d9c => {
  cpu.flat_ds(0x40a0d9c); cpu.push(0x40a0da1); f_040a0e2f(cpu); if cpu.returned_to != 0x40a0da1 { return; }
  { let v = cpu.r16(0x415ac86u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  cpu.flat_ds(0x40a0daf); cpu.push(0x40a0db4); f_040a0e99(cpu); if cpu.returned_to != 0x40a0db4 { return; }
  pc = 0x40a0e2e; continue;
 }
 0x40a0db6 => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x40a0dda; continue; }
  pc = 0x40a0dbf; continue;
 }
 0x40a0dbf => {
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x40ae8b4u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x40a0e2e; continue;
 }
 0x40a0dda => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x40a0e02; continue; }
  pc = 0x40a0de3; continue;
 }
 0x40a0de3 => {
  cpu.flat_ds(0x40a0de3); cpu.push(0x40a0de8); f_040a0e2f(cpu); if cpu.returned_to != 0x40a0de8 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415ac80u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x40a0dfd); cpu.push(0x40a0e00); cpu.call(t); if cpu.returned_to != 0x40a0e00 { return; }
  pc = 0x40a0e2e; continue;
 }
 0x40a0e02 => {
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x40a0e1b; continue; }
  pc = 0x40a0e0b; continue;
 }
 0x40a0e0b => {
  { let v: u16 = cpu.r16(0x415ac84u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x40a0e14); cpu.push(0x40a0e19); f_040b16f2(cpu); if cpu.returned_to != 0x40a0e19 { return; }
  pc = 0x40a0e2e; continue;
 }
 0x40a0e1b => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x40a0e20); cpu.push(0x40a0e25); f_040b16f2(cpu); if cpu.returned_to != 0x40a0e25 { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a0e2e; continue; }
  pc = 0x40a0e29; continue;
 }
 0x40a0e29 => {
  cpu.flat_ds(0x40a0e29); cpu.push(0x40a0e2e); f_040a0e4c(cpu); if cpu.returned_to != 0x40a0e2e { return; }
  pc = 0x40a0e2e; continue;
 }
 0x40a0e2e => {
  cpu.flat_ds(0x40a0e2e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0e2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0e2f); }
 let mut pc: u32 = 0x40a0e2f;
 loop { match pc {
 0x40a0e2f => {
  { let v: u32 = 0x40ae8b4u32; cpu.ecx = v; }
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
pub fn f_040a0e4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0e4c); }
 let mut pc: u32 = 0x40a0e4c;
 loop { match pc {
 0x40a0e4c => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b7fcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x415b800u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x3u8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.sf != cpu.of) { pc = 0x40a0e83; continue; }
  pc = 0x40a0e73; continue;
 }
 0x40a0e73 => {
  { let v: u32 = 0x40ae844u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x40a0e7c); cpu.push(0x40a0e81); f_040a0e99(cpu); if cpu.returned_to != 0x40a0e81 { return; }
  pc = 0x40a0e98; continue;
 }
 0x40a0e83 => {
  { let v: u32 = 0x40ae8b4u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffdu32; cpu.ecx = v; }
  {  }
  pc = 0x40a0e98; continue;
 }
 0x40a0e98 => {
  cpu.flat_ds(0x40a0e98); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0e99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0e99); }
 let mut pc: u32 = 0x40a0e99;
 loop { match pc {
 0x40a0e99 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x415b7bcu32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0x415b7bcu32) | 0x4u32; cpu.w32(0x415b7bcu32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x415b7bcu32), 0xfffffffdu32); cpu.w32(0x415b7bcu32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0f33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0f33); }
 let mut pc: u32 = 0x40a0f33;
 loop { match pc {
 0x40a0f33 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x415b7fcu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x415b800u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.sub8(cpu.r8(0x415b83au32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x40a0f5a; continue; }
  pc = 0x40a0f53; continue;
 }
 0x40a0f53 => {
  cpu.flat_ds(0x40a0f53); cpu.push(0x40a0f58); f_040a0f6e(cpu); if cpu.returned_to != 0x40a0f58 { return; }
  pc = 0x40a0f6d; continue;
 }
 0x40a0f5a => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff9u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x415b802u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  {  }
  pc = 0x40a0f6d; continue;
 }
 0x40a0f6d => {
  cpu.flat_ds(0x40a0f6d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0f6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0f6e); }
 let mut pc: u32 = 0x40a0f6e;
 loop { match pc {
 0x40a0f6e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x415b7bcu32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x415b802u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415b7bcu32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0f8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0f8f); }
 let mut pc: u32 = 0x40a0f8f;
 loop { match pc {
 0x40a0f8f => {
  {  }
  cpu.unimplemented(0x40a0f90, "Fnsave");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0f94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0f94); }
 let mut pc: u32 = 0x40a0f94;
 loop { match pc {
 0x40a0f94 => {
  cpu.unimplemented(0x40a0f94, "Frstor");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0f98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0f98); }
 let mut pc: u32 = 0x40a0f98;
 loop { match pc {
 0x40a0f98 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x41544ddu32), 0x0u8); }
  if cpu.zf { pc = 0x40a0fb9; continue; }
  pc = 0x40a0fa3; continue;
 }
 0x40a0fa3 => {
  { let v: u32 = 0x40a0f8fu32; cpu.edx = v; }
  { let v: u32 = 0x40a0f94u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415c7ccu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x415c7d0u32, v); }
  pc = 0x40a0fb9; continue;
 }
 0x40a0fb9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x415c7c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40a0fc6); f_040b1820(cpu); if cpu.returned_to != 0x40a0fc6 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0fc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0fc9); }
 let mut pc: u32 = 0x40a0fc9;
 loop { match pc {
 0x40a0fc9 => {
  { cpu.sub8(cpu.r8(0x41544ddu32), 0x0u8); }
  if !cpu.zf { f_040a0f98(cpu); return; }
  pc = 0x40a0fd2; continue;
 }
 0x40a0fd2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a0fd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a0fd3); }
 let mut pc: u32 = 0x40a0fd3;
 loop { match pc {
 0x40a0fd3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x41544dcu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a101b; continue; }
  pc = 0x40a0fde; continue;
 }
 0x40a0fde => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x41544ddu32, v); }
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
  if !cpu.zf { pc = 0x40a1006; continue; }
  pc = 0x40a0ffd; continue;
 }
 0x40a0ffd => {
  cpu.push(0x40a1002); f_040a0f98(cpu); if cpu.returned_to != 0x40a1002 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a1006; continue;
 }
 0x40a1006 => {
  { cpu.sub8(cpu.r8(0x415b838u32), 0x0u8); }
  if !cpu.zf { pc = 0x40a101b; continue; }
  pc = 0x40a100f; continue;
 }
 0x40a100f => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x41544dcu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x41544ddu32, v); }
  pc = 0x40a101b; continue;
 }
 0x40a101b => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a101d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a101d); }
 let mut pc: u32 = 0x40a101d;
 loop { match pc {
 0x40a101d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a101e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a101e); }
 let mut pc: u32 = 0x40a101e;
 loop { match pc {
 0x40a101e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x40a1031); f_0409e8e0(cpu); if cpu.returned_to != 0x40a1031 { return; }
  pc = 0x40a1052; continue;
 }
 0x40a1033 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x414b960u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x40a1052; continue;
 }
 0x40a1052 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x40a1033; continue; }
  pc = 0x40a1058; continue;
 }
 0x40a1058 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a105c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a105c); }
 let mut pc: u32 = 0x40a105c;
 loop { match pc {
 0x40a105c => {
  if !cpu.cf { pc = 0x40a106c; continue; }
  pc = 0x40a105e; continue;
 }
 0x40a105e => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x40a1069); f_040b1852(cpu); if cpu.returned_to != 0x40a1069 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x40a106e; continue;
 }
 0x40a106c => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40a106e; continue;
 }
 0x40a106e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a106f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a106f); }
 let mut pc: u32 = 0x40a106f;
 loop { match pc {
 0x40a106f => {
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
  cpu.push(0x40a108d); f_040a1093(cpu); if cpu.returned_to != 0x40a108d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a1093(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1093); }
 let mut pc: u32 = 0x40a1093;
 loop { match pc {
 0x40a1093 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x40a109f; continue;
 }
 0x40a109f => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x40a10aa; continue; }
  pc = 0x40a10a7; continue;
 }
 0x40a10a7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x40a109f; continue;
 }
 0x40a10aa => {
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
  if (cpu.sf != cpu.of) { pc = 0x40a10dd; continue; }
  pc = 0x40a10d7; continue;
 }
 0x40a10d7 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x40a10dd; continue;
 }
 0x40a10dd => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x3u8); }
  if cpu.zf { pc = 0x40a1160; continue; }
  pc = 0x40a10ea; continue;
 }
 0x40a10ea => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a1160; continue; }
  pc = 0x40a10f3; continue;
 }
 0x40a10f3 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40a10fa); f_040b18c3(cpu); if cpu.returned_to != 0x40a10fa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a1160; continue; }
  pc = 0x40a10fe; continue;
 }
 0x40a10fe => {
  { cpu.and8(cpu.r8(0x29u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x40a1128; continue; }
  pc = 0x40a1105; continue;
 }
 0x40a1105 => {
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x40a1128; continue; }
  pc = 0x40a110a; continue;
 }
 0x40a110a => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x40a111e); f_0409ebfb(cpu); if cpu.returned_to != 0x40a111e { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a1273; continue;
 }
 0x40a1128 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x40u8); }
  if cpu.zf { pc = 0x40a1160; continue; }
  pc = 0x40a112f; continue;
 }
 0x40a112f => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a1160; continue; }
  pc = 0x40a1146; continue;
 }
 0x40a1146 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x40a1150; continue;
 }
 0x40a1150 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40a115b); f_040b1852(cpu); if cpu.returned_to != 0x40a115b { return; }
  pc = 0x40a1273; continue;
 }
 0x40a1160 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x40a1202; continue; }
  pc = 0x40a1169; continue;
 }
 0x40a1169 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x40a1150; continue; }
  pc = 0x40a1170; continue;
 }
 0x40a1170 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x40a1150; continue; }
  pc = 0x40a117b; continue;
 }
 0x40a117b => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a118e; continue; }
  pc = 0x40a1189; continue;
 }
 0x40a1189 => {
  { let v: u32 = 0x180u32; cpu.eax = v; }
  pc = 0x40a118e; continue;
 }
 0x40a118e => {
  { let v: u32 = cpu.r32(0x415c7e8u32); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x40a11a3; continue; }
  pc = 0x40a119e; continue;
 }
 0x40a119e => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x40a11a3; continue;
 }
 0x40a11a3 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1150; continue; }
  pc = 0x40a11ba; continue;
 }
 0x40a11ba => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a1202; continue; }
  pc = 0x40a11c8; continue;
 }
 0x40a11c8 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1150; continue; }
  pc = 0x40a11dd; continue;
 }
 0x40a11dd => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1150; continue; }
  pc = 0x40a11fc; continue;
 }
 0x40a11fc => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x40a1202; continue;
 }
 0x40a1202 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40a1209); f_040b18ea(cpu); if cpu.returned_to != 0x40a1209 { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40a1214); f_040b18c3(cpu); if cpu.returned_to != 0x40a1214 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a121b; continue; }
  pc = 0x40a1218; continue;
 }
 0x40a1218 => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x40a121b; continue;
 }
 0x40a121b => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x40a122a; continue; }
  pc = 0x40a1227; continue;
 }
 0x40a1227 => {
  { let v: u8 = (cpu.edx as u8) | 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40a122a; continue;
 }
 0x40a122a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a1234; continue; }
  pc = 0x40a1231; continue;
 }
 0x40a1231 => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40a1234; continue;
 }
 0x40a1234 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x40a123e; continue; }
  pc = 0x40a123b; continue;
 }
 0x40a123b => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40a123e; continue;
 }
 0x40a123e => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x40a1248; continue; }
  pc = 0x40a1245; continue;
 }
 0x40a1245 => {
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x40a1248; continue;
 }
 0x40a1248 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if cpu.zf { pc = 0x40a125c; continue; }
  pc = 0x40a1255; continue;
 }
 0x40a1255 => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x40a126a; continue; }
  pc = 0x40a125a; continue;
 }
 0x40a125a => {
  pc = 0x40a1268; continue;
 }
 0x40a125c => {
  { cpu.sub32(cpu.r32(0x415aeb1u32), 0x200u32); }
  if !cpu.zf { pc = 0x40a126a; continue; }
  pc = 0x40a1268; continue;
 }
 0x40a1268 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x40a126a; continue;
 }
 0x40a126a => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x40a1271); f_040b193f(cpu); if cpu.returned_to != 0x40a1271 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x40a1273; continue;
 }
 0x40a1273 => {
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
pub fn f_040a127c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a127c); }
 let mut pc: u32 = 0x40a127c;
 loop { match pc {
 0x40a127c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x40a128a); f_040b18ea(cpu); if cpu.returned_to != 0x40a128a { return; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40a1297); f_040b193f(cpu); if cpu.returned_to != 0x40a1297 { return; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x40a12d7; continue; }
  pc = 0x40a129c; continue;
 }
 0x40a129c => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40a12bc; continue; }
  pc = 0x40a12af; continue;
 }
 0x40a12af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40a12ba); f_040b1852(cpu); if cpu.returned_to != 0x40a12ba { return; }
  pc = 0x40a12d9; continue;
 }
 0x40a12bc => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a12af; continue; }
  pc = 0x40a12d7; continue;
 }
 0x40a12d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a12d9; continue;
 }
 0x40a12d9 => {
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
pub fn f_040a12e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a12e1); }
 let mut pc: u32 = 0x40a12e1;
 loop { match pc {
 0x40a12e1 => {
  { let v: u32 = 0x42c56acu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a12e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a12e7); }
 let mut pc: u32 = 0x40a12e7;
 loop { match pc {
 0x40a12e7 => {
  { let v: u32 = 0x42c56b0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a12ed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a12ed); }
 let mut pc: u32 = 0x40a12ed;
 loop { match pc {
 0x40a12ed => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x40a12fd; continue; }
  pc = 0x40a12f1; continue;
 }
 0x40a12f1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x415c7ecu32)); }
  if (cpu.cf || cpu.zf) { f_040b1954(cpu); return; }
  pc = 0x40a12fd; continue;
 }
 0x40a12fd => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x40a1307); f_0409ebfb(cpu); if cpu.returned_to != 0x40a1307 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a130d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a130d); }
 let mut pc: u32 = 0x40a130d;
 loop { match pc {
 0x40a130d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x40a131c); f_040b18ea(cpu); if cpu.returned_to != 0x40a131c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a1338; continue; }
  pc = 0x40a1324; continue;
 }
 0x40a1324 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x40a1329; continue;
 }
 0x40a1329 => {
  cpu.push(0x40a132e); f_0409ebfb(cpu); if cpu.returned_to != 0x40a132e { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a1400; continue;
 }
 0x40a1338 => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x40a1343; continue; }
  pc = 0x40a133c; continue;
 }
 0x40a133c => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x40a1329; continue;
 }
 0x40a1343 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x40a136f; continue; }
  pc = 0x40a1347; continue;
 }
 0x40a1347 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a13fe; continue; }
  pc = 0x40a1360; continue;
 }
 0x40a1360 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x40a1365; continue;
 }
 0x40a1365 => {
  cpu.push(0x40a136a); f_040b1852(cpu); if cpu.returned_to != 0x40a136a { return; }
  pc = 0x40a1400; continue;
 }
 0x40a136f => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x40a1375; continue;
 }
 0x40a1375 => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a1399; continue; }
  pc = 0x40a1392; continue;
 }
 0x40a1392 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x40a1365; continue;
 }
 0x40a1399 => {
  if cpu.zf { pc = 0x40a13fe; continue; }
  pc = 0x40a139b; continue;
 }
 0x40a139b => {
  { let v: u32 = cpu.edi ^ cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a13db; continue;
 }
 0x40a13aa => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x1au32); }
  if !cpu.zf { pc = 0x40a13cb; continue; }
  pc = 0x40a13b3; continue;
 }
 0x40a13b3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x40a13c9); f_040a15dd(cpu); if cpu.returned_to != 0x40a13c9 { return; }
  pc = 0x40a13fe; continue;
 }
 0x40a13cb => {
  { cpu.sub32(cpu.ebx, 0xdu32); }
  if cpu.zf { pc = 0x40a13d9; continue; }
  pc = 0x40a13d0; continue;
 }
 0x40a13d0 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x40a13d9; continue;
 }
 0x40a13d9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x40a13db; continue;
 }
 0x40a13db => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x40a13aa; continue; }
  pc = 0x40a13e1; continue;
 }
 0x40a13e1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if !cpu.zf { pc = 0x40a13fe; continue; }
  pc = 0x40a13f6; continue;
 }
 0x40a13f6 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x40a1375; continue; }
  pc = 0x40a13fe; continue;
 }
 0x40a13fe => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a1400; continue;
 }
 0x40a1400 => {
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
pub fn f_040a1408(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1408); }
 let mut pc: u32 = 0x40a1408;
 loop { match pc {
 0x40a1408 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x40a1419); f_040b18ea(cpu); if cpu.returned_to != 0x40a1419 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a1434; continue; }
  pc = 0x40a1420; continue;
 }
 0x40a1420 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x40a1425; continue;
 }
 0x40a1425 => {
  cpu.push(0x40a142a); f_0409ebfb(cpu); if cpu.returned_to != 0x40a142a { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a15d6; continue;
 }
 0x40a1434 => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x40a143f; continue; }
  pc = 0x40a1438; continue;
 }
 0x40a1438 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x40a1425; continue;
 }
 0x40a143f => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x40a1476; continue; }
  pc = 0x40a1443; continue;
 }
 0x40a1443 => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a1476; continue; }
  pc = 0x40a1466; continue;
 }
 0x40a1466 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40a1471); f_040b1852(cpu); if cpu.returned_to != 0x40a1471 { return; }
  pc = 0x40a15d6; continue;
 }
 0x40a1476 => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x40a14b1; continue; }
  pc = 0x40a147c; continue;
 }
 0x40a147c => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1466; continue; }
  pc = 0x40a1496; continue;
 }
 0x40a1496 => {
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x40a15d3; continue; }
  pc = 0x40a149f; continue;
 }
 0x40a149f => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x40a14a9); f_0409ebfb(cpu); if cpu.returned_to != 0x40a14a9 { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a15d3; continue;
 }
 0x40a14b1 => {
  cpu.push(0x40a14b6); f_040b1989(cpu); if cpu.returned_to != 0x40a14b6 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if !cpu.cf { pc = 0x40a14c4; continue; }
  pc = 0x40a14bf; continue;
 }
 0x40a14bf => {
  cpu.push(0x40a14c4); f_0409dedb(cpu); if cpu.returned_to != 0x40a14c4 { return; }
  pc = 0x40a14c4; continue;
 }
 0x40a14c4 => {
  { let v: u32 = 0x200u32; cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x230u32); }
  if !cpu.cf { pc = 0x40a14d6; continue; }
  pc = 0x40a14d1; continue;
 }
 0x40a14d1 => {
  { let v: u32 = 0x80u32; cpu.edi = v; }
  pc = 0x40a14d6; continue;
 }
 0x40a14d6 => {
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
  pc = 0x40a14f2; continue;
 }
 0x40a14f2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x40a159d; continue; }
  pc = 0x40a14fe; continue;
 }
 0x40a14fe => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x40a1552; continue; }
  pc = 0x40a150a; continue;
 }
 0x40a150a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x40a1552; continue; }
  pc = 0x40a1516; continue;
 }
 0x40a1516 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1466; continue; }
  pc = 0x40a1532; continue;
 }
 0x40a1532 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x40a154a; continue; }
  pc = 0x40a1536; continue;
 }
 0x40a1536 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x40a1540); f_0409ebfb(cpu); if cpu.returned_to != 0x40a1540 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x40a15d6; continue;
 }
 0x40a154a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a1552; continue;
 }
 0x40a1552 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x40a14f2; continue; }
  pc = 0x40a1570; continue;
 }
 0x40a1570 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1466; continue; }
  pc = 0x40a158c; continue;
 }
 0x40a158c => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x40a1536; continue; }
  pc = 0x40a1590; continue;
 }
 0x40a1590 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x40a14f2; continue;
 }
 0x40a159d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a15d3; continue; }
  pc = 0x40a15a1; continue;
 }
 0x40a15a1 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1466; continue; }
  pc = 0x40a15be; continue;
 }
 0x40a15be => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40a15d3; continue; }
  pc = 0x40a15c2; continue;
 }
 0x40a15c2 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x40a15cc); f_0409ebfb(cpu); if cpu.returned_to != 0x40a15cc { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  pc = 0x40a15d6; continue;
 }
 0x40a15d3 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x40a15d6; continue;
 }
 0x40a15d6 => {
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
pub fn f_040a15dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a15dd); }
 let mut pc: u32 = 0x40a15dd;
 loop { match pc {
 0x40a15dd => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a1617; continue; }
  pc = 0x40a1606; continue;
 }
 0x40a1606 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40a1610); f_040b1852(cpu); if cpu.returned_to != 0x40a1610 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a161a; continue;
 }
 0x40a1617 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x40a161a; continue;
 }
 0x40a161a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a1620(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1620); }
 let mut pc: u32 = 0x40a1620;
 loop { match pc {
 0x40a1620 => {
  pc = 0x40b1992; continue;
 }
 0x40b1992 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40b19af; continue; }
  pc = 0x40b19a3; continue;
 }
 0x40b19a3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40b19ad); f_040b1852(cpu); if cpu.returned_to != 0x40b19ad { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40b19af => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a1625(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1625); }
 let mut pc: u32 = 0x40a1625;
 loop { match pc {
 0x40a1625 => {
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
  if (cpu.sf == cpu.of) { pc = 0x40a1654; continue; }
  pc = 0x40a1646; continue;
 }
 0x40a1646 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x40a164b; continue;
 }
 0x40a164b => {
  cpu.push(0x40a1650); f_0409ebfb(cpu); if cpu.returned_to != 0x40a1650 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a16a1; continue;
 }
 0x40a1654 => {
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
  if !cpu.zf { pc = 0x40a1698; continue; }
  pc = 0x40a1682; continue;
 }
 0x40a1682 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40a1689); f_040ad06c(cpu); if cpu.returned_to != 0x40a1689 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a1696; continue; }
  pc = 0x40a168f; continue;
 }
 0x40a168f => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.poll(); pc = 0x40a164b; continue;
 }
 0x40a1696 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x40a1698; continue;
 }
 0x40a1698 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x40a16a1); f_0408f3b4(cpu); if cpu.returned_to != 0x40a16a1 { return; }
  pc = 0x40a16a1; continue;
 }
 0x40a16a1 => {
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
pub fn f_040a16ad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a16ad); }
 let mut pc: u32 = 0x40a16ad;
 loop { match pc {
 0x40a16ad => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x3bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a16ca; continue; }
  pc = 0x40a16be; continue;
 }
 0x40a16be => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x40a16c8); f_040b1852(cpu); if cpu.returned_to != 0x40a16c8 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a16ca => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a16ce(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a16ce); }
 let mut pc: u32 = 0x40a16ce;
 loop { match pc {
 0x40a16ce => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x40a16d7); f_040945a4(cpu); if cpu.returned_to != 0x40a16d7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a16f8; continue; }
  pc = 0x40a16de; continue;
 }
 0x40a16de => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x40a16e5); f_040a16fb(cpu); if cpu.returned_to != 0x40a16e5 { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x40a16f8; continue; }
  pc = 0x40a16ea; continue;
 }
 0x40a16ea => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x40a16f6; continue; }
  pc = 0x40a16ee; continue;
 }
 0x40a16ee => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a16f6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x40a16f8; continue;
 }
 0x40a16f8 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a16fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a16fb); }
 let mut pc: u32 = 0x40a16fb;
 loop { match pc {
 0x40a16fb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x40a1709); f_040a15dd(cpu); if cpu.returned_to != 0x40a1709 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a170c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a170c); }
 let mut pc: u32 = 0x40a170c;
 loop { match pc {
 0x40a170c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.push(0x40a1718); f_040b19b3(cpu); if cpu.returned_to != 0x40a1718 { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a171d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a171d); }
 let mut pc: u32 = 0x40a171d;
 loop { match pc {
 0x40a171d => {
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
  if !cpu.zf { pc = 0x40a1740; continue; }
  pc = 0x40a1739; continue;
 }
 0x40a1739 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40a1740); f_040b1a57(cpu); if cpu.returned_to != 0x40a1740 { return; }
  pc = 0x40a1740; continue;
 }
 0x40a1740 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x40a175b; continue; }
  pc = 0x40a174a; continue;
 }
 0x40a174a => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  pc = 0x40a175b; continue;
 }
 0x40a175b => {
  { let v: u32 = 0x40a170cu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x40a1767); f_0409ed88(cpu); if cpu.returned_to != 0x40a1767 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x40a1782; continue; }
  pc = 0x40a176d; continue;
 }
 0x40a176d => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x40a1782); f_040accda(cpu); if cpu.returned_to != 0x40a1782 { return; }
  pc = 0x40a1782; continue;
 }
 0x40a1782 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x40a178d; continue; }
  pc = 0x40a1788; continue;
 }
 0x40a1788 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x40a178d; continue;
 }
 0x40a178d => {
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
pub fn f_040a179c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a179c); }
 let mut pc: u32 = 0x40a179c;
 loop { match pc {
 0x40a179c => {
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x40a17a6; continue; }
  pc = 0x40a17a1; continue;
 }
 0x40a17a1 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x40a17af; continue; }
  pc = 0x40a17a6; continue;
 }
 0x40a17a6 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x40a17ad; continue; }
  pc = 0x40a17ab; continue;
 }
 0x40a17ab => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x40a17ad; continue;
 }
 0x40a17ad => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x40a17af; continue;
 }
 0x40a17af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a17b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a17b0); }
 let mut pc: u32 = 0x40a17b0;
 loop { match pc {
 0x40a17b0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a17d4; continue; }
  pc = 0x40a17c5; continue;
 }
 0x40a17c5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x40a17d4; continue; }
  pc = 0x40a17cb; continue;
 }
 0x40a17cb => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40a17d4; continue;
 }
 0x40a17d4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a1892; continue; }
  pc = 0x40a17e0; continue;
 }
 0x40a17e0 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x40a1892; continue; }
  pc = 0x40a17e9; continue;
 }
 0x40a17e9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x40a17f4); f_040b1ace(cpu); if cpu.returned_to != 0x40a17f4 { return; }
  cpu.push(0x40a17f9); f_040a179c(cpu); if cpu.returned_to != 0x40a17f9 { return; }
  cpu.push(0x40a17fe); f_040b1b26(cpu); if cpu.returned_to != 0x40a17fe { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x40a1803; continue;
 }
 0x40a1803 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a181b; continue; }
  pc = 0x40a180b; continue;
 }
 0x40a180b => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a1803; continue; }
  pc = 0x40a181b; continue;
 }
 0x40a181b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x415c844u32), 0x0u32); }
  if cpu.zf { pc = 0x40a1839; continue; }
  pc = 0x40a1825; continue;
 }
 0x40a1825 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x40a183b; continue;
 }
 0x40a1839 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a183b; continue;
 }
 0x40a183b => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a1858; continue; }
  pc = 0x40a1847; continue;
 }
 0x40a1847 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x40a1858; continue;
 }
 0x40a1858 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a17e9; continue; }
  pc = 0x40a1860; continue;
 }
 0x40a1860 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a186f; continue; }
  pc = 0x40a1867; continue;
 }
 0x40a1867 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a186f; continue;
 }
 0x40a186f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x40a1879); f_040b1d9a(cpu); if cpu.returned_to != 0x40a1879 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x40a188b; continue; }
  pc = 0x40a1888; continue;
 }
 0x40a1888 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40a1892; continue;
 }
 0x40a188b => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a1892; continue;
 }
 0x40a1892 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x40a18a1; continue; }
  pc = 0x40a1899; continue;
 }
 0x40a1899 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a18a1; continue;
 }
 0x40a18a1 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x40a1949; continue; }
  pc = 0x40a18ac; continue;
 }
 0x40a18ac => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x40a18b7); f_040b1ace(cpu); if cpu.returned_to != 0x40a18b7 { return; }
  cpu.push(0x40a18bc); f_040a179c(cpu); if cpu.returned_to != 0x40a18bc { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x40a18cf; continue; }
  pc = 0x40a18c4; continue;
 }
 0x40a18c4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x40a18cf; continue; }
  pc = 0x40a18cd; continue;
 }
 0x40a18cd => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x40a18cf; continue;
 }
 0x40a18cf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x40a18da); f_040b1ace(cpu); if cpu.returned_to != 0x40a18da { return; }
  cpu.push(0x40a18df); f_040a179c(cpu); if cpu.returned_to != 0x40a18df { return; }
  cpu.push(0x40a18e4); f_040b1b26(cpu); if cpu.returned_to != 0x40a18e4 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x40a18e9; continue;
 }
 0x40a18e9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x40a1901; continue; }
  pc = 0x40a18f1; continue;
 }
 0x40a18f1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a18e9; continue; }
  pc = 0x40a1901; continue;
 }
 0x40a1901 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a191d; continue; }
  pc = 0x40a190b; continue;
 }
 0x40a190b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x40a191d; continue;
 }
 0x40a191d => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x415c844u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a193d; continue; }
  pc = 0x40a192a; continue;
 }
 0x40a192a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415c849u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x40a193f; continue;
 }
 0x40a193d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x40a193f; continue;
 }
 0x40a193f => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x40a18cf; continue; }
  pc = 0x40a1947; continue;
 }
 0x40a1947 => {
  pc = 0x40a1958; continue;
 }
 0x40a1949 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x40a1958; continue; }
  pc = 0x40a1956; continue;
 }
 0x40a1956 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x40a1958; continue;
 }
 0x40a1958 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x40a198e; continue; }
  pc = 0x40a1960; continue;
 }
 0x40a1960 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x40a198e; continue; }
  pc = 0x40a1965; continue;
 }
 0x40a1965 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x40a1977; continue; }
  pc = 0x40a1972; continue;
 }
 0x40a1972 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40a1977; continue;
 }
 0x40a1977 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x40a198e; continue; }
  pc = 0x40a1981; continue;
 }
 0x40a1981 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x40a1977; continue;
 }
 0x40a198e => {
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
pub fn f_040a199b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a199b); }
 let mut pc: u32 = 0x40a199b;
 loop { match pc {
 0x40a199b => {
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
  if cpu.zf { pc = 0x40a1a4d; continue; }
  pc = 0x40a1a2b; continue;
 }
 0x40a1a2b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x17u32); }
  if cpu.zf { pc = 0x40a1a4d; continue; }
  pc = 0x40a1a31; continue;
 }
 0x40a1a31 => {
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
  pc = 0x40a1a4d; continue;
 }
 0x40a1a4d => {
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
pub fn f_040a1a5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1a5e); }
 let mut pc: u32 = 0x40a1a5e;
 loop { match pc {
 0x40a1a5e => {
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
  { let v: u32 = 0x414b968u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0xf4fb7ea1u32); }
  if (cpu.sf == cpu.of) { pc = 0x40a1a92; continue; }
  pc = 0x40a1a88; continue;
 }
 0x40a1a88 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x40a1bab; continue;
 }
 0x40a1a92 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x40a1aa3; continue;
 }
 0x40a1aa3 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x40a1aad; continue; }
  pc = 0x40a1aa7; continue;
 }
 0x40a1aa7 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  cpu.poll(); pc = 0x40a1aa3; continue;
 }
 0x40a1aad => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1a88; continue; }
  pc = 0x40a1ab1; continue;
 }
 0x40a1ab1 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x40a1abc); f_040a20c2(cpu); if cpu.returned_to != 0x40a1abc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a1ac5; continue; }
  pc = 0x40a1ac0; continue;
 }
 0x40a1ac0 => {
  { let v: u32 = 0x414b982u32; cpu.ebx = v; }
  pc = 0x40a1ac5; continue;
 }
 0x40a1ac5 => {
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
  if cpu.zf { pc = 0x40a1af2; continue; }
  pc = 0x40a1af1; continue;
 }
 0x40a1af1 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a1af2; continue;
 }
 0x40a1af2 => {
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
  pc = 0x40a1b13; continue;
 }
 0x40a1b13 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40a1b20; continue; }
  pc = 0x40a1b17; continue;
 }
 0x40a1b17 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40a1b13; continue;
 }
 0x40a1b20 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x40a1b2d); f_040a1f82(cpu); if cpu.returned_to != 0x40a1b2d { return; }
  cpu.push(0x40a1b32); f_040a1c1e(cpu); if cpu.returned_to != 0x40a1b32 { return; }
  { let v: u32 = cpu.r32(0x415b894u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x40a1b48; continue; }
  pc = 0x40a1b41; continue;
 }
 0x40a1b41 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x40a1b48); f_040a2216(cpu); if cpu.returned_to != 0x40a1b48 { return; }
  pc = 0x40a1b48; continue;
 }
 0x40a1b48 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a1b54; continue; }
  pc = 0x40a1b4e; continue;
 }
 0x40a1b4e => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x415b898u32)); cpu.esi = v; }
  pc = 0x40a1b54; continue;
 }
 0x40a1b54 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40a1b61; continue; }
  pc = 0x40a1b58; continue;
 }
 0x40a1b58 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x40a1b54; continue;
 }
 0x40a1b61 => {
  { cpu.sub32(cpu.edi, 0x63deu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1a88; continue; }
  pc = 0x40a1b6d; continue;
 }
 0x40a1b6d => {
  if !cpu.zf { pc = 0x40a1b8b; continue; }
  pc = 0x40a1b6f; continue;
 }
 0x40a1b6f => {
  { let v: u32 = cpu.r32(0x415b894u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x15180u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x40a1a88; continue; }
  pc = 0x40a1b82; continue;
 }
 0x40a1b82 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x40a1ba9; continue; }
  pc = 0x40a1b86; continue;
 }
 0x40a1b86 => {
  cpu.poll(); pc = 0x40a1a88; continue;
 }
 0x40a1b8b => {
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
  pc = 0x40a1ba9; continue;
 }
 0x40a1ba9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x40a1bab; continue;
 }
 0x40a1bab => {
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
pub fn f_040a1bb2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1bb2); }
 let mut pc: u32 = 0x40a1bb2;
 loop { match pc {
 0x40a1bb2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415b8e6u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x415b8e6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x415b8e6u32, v); }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x415b8e6u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x415b8e6u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x415b8e6u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x415b8e6u32, v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x415b8e6u32), cpu.edx); cpu.w32(0x415b8e6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a1c1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1c1e); }
 let mut pc: u32 = 0x40a1c1e;
 loop { match pc {
 0x40a1c06 => {
  { let v: u8 = cpu.r8(0x415b8e6u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x40a1c16; continue; }
  pc = 0x40a1c11; continue;
 }
 0x40a1c11 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.zf { pc = 0x40a1c1d; continue; }
  pc = 0x40a1c16; continue;
 }
 0x40a1c16 => {
  { let v: u8 = cpu.or8(cpu.r8(0x415b8e6u32), 0x2u8); cpu.w8(0x415b8e6u32, v); }
  pc = 0x40a1c1d; continue;
 }
 0x40a1c1d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a1c1e => {
  { let v: u32 = 0x414afdcu32; cpu.eax = v; }
  cpu.push(0x40a1c28); f_040ad1d1(cpu); if cpu.returned_to != 0x40a1c28 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x40a1c06; continue; }
  pc = 0x40a1c2c; continue;
 }
 0x40a1c2c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x415b894u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x415b89cu32, v); }
  { let v: u32 = 0x415b8a0u32; cpu.edx = v; }
  cpu.push(0x40a1c4c); f_040a1c87(cpu); if cpu.returned_to != 0x40a1c4c { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x40a1eb4; continue; }
  pc = 0x40a1c58; continue;
 }
 0x40a1c58 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x415b8bfu32, v); }
  pc = 0x40a1f78; continue;
 }
 0x40a1eb4 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x415b8bfu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x415b894u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b89cu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a1eda); f_040a1c87(cpu); if cpu.returned_to != 0x40a1eda { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x415b894u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x415b898u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, 0x2cu32); }
  if !cpu.zf { pc = 0x40a1f03; continue; }
  pc = 0x40a1ef6; continue;
 }
 0x40a1ef6 => {
  { let v: u32 = 0x415b84cu32; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x40a1f01); f_040a1db0(cpu); if cpu.returned_to != 0x40a1f01 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x40a1f03; continue;
 }
 0x40a1f03 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if !cpu.zf { pc = 0x40a1f78; continue; }
  pc = 0x40a1f0c; continue;
 }
 0x40a1f0c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x415b870u32; cpu.edx = v; }
  cpu.push(0x40a1f19); f_040a1db0(cpu); if cpu.returned_to != 0x40a1f19 { return; }
  { let v: u32 = cpu.r32(0x415b898u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xe10u32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x415b878u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x415b898u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x415b874u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x415b898u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x415b870u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x415b878u32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x415b874u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x415b870u32, v); }
  pc = 0x40a1f78; continue;
 }
 0x40a1f78 => {
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
pub fn f_040a1c63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1c63); }
 let mut pc: u32 = 0x40a1c63;
 loop { match pc {
 0x40a1c63 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x40a1c69; continue;
 }
 0x40a1c69 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a1c82; continue; }
  pc = 0x40a1c72; continue;
 }
 0x40a1c72 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a1c82; continue; }
  pc = 0x40a1c77; continue;
 }
 0x40a1c77 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x40a1c69; continue;
 }
 0x40a1c82 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a1c87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1c87); }
 let mut pc: u32 = 0x40a1c87;
 loop { match pc {
 0x40a1c87 => {
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
  if !cpu.zf { pc = 0x40a1c9f; continue; }
  pc = 0x40a1c9e; continue;
 }
 0x40a1c9e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x40a1c9f; continue;
 }
 0x40a1c9f => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x40a1ca1; continue;
 }
 0x40a1ca1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40a1cc8; continue; }
  pc = 0x40a1ca8; continue;
 }
 0x40a1ca8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if cpu.zf { pc = 0x40a1cc8; continue; }
  pc = 0x40a1cb1; continue;
 }
 0x40a1cb1 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if cpu.zf { pc = 0x40a1cc8; continue; }
  pc = 0x40a1cb6; continue;
 }
 0x40a1cb6 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x40a1cc8; continue; }
  pc = 0x40a1cbb; continue;
 }
 0x40a1cbb => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a1cc5; continue; }
  pc = 0x40a1cc0; continue;
 }
 0x40a1cc0 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a1cc8; continue; }
  pc = 0x40a1cc5; continue;
 }
 0x40a1cc5 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = 0x40a1ca1; continue;
 }
 0x40a1cc8 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a1cdd; continue; }
  pc = 0x40a1cd5; continue;
 }
 0x40a1cd5 => {
  { let v: u32 = 0x1eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x40a1cdd; continue;
 }
 0x40a1cdd => {
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
  if !cpu.zf { pc = 0x40a1d1a; continue; }
  pc = 0x40a1d13; continue;
 }
 0x40a1d13 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x40a1d1f; continue;
 }
 0x40a1d1a => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x40a1d21; continue; }
  pc = 0x40a1d1f; continue;
 }
 0x40a1d1f => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x40a1d21; continue;
 }
 0x40a1d21 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x40a1da6; continue; }
  pc = 0x40a1d32; continue;
 }
 0x40a1d32 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a1da6; continue; }
  pc = 0x40a1d37; continue;
 }
 0x40a1d37 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x40a1d4f); f_040a1c63(cpu); if cpu.returned_to != 0x40a1d4f { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x40a1d79; continue; }
  pc = 0x40a1d5a; continue;
 }
 0x40a1d5a => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x40a1d64); f_040a1c63(cpu); if cpu.returned_to != 0x40a1d64 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x40a1d79; continue; }
  pc = 0x40a1d6f; continue;
 }
 0x40a1d6f => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x40a1d77); f_040a1c63(cpu); if cpu.returned_to != 0x40a1d77 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x40a1d79; continue;
 }
 0x40a1d79 => {
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
  if cpu.zf { pc = 0x40a1da6; continue; }
  pc = 0x40a1da2; continue;
 }
 0x40a1da2 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x40a1da6; continue;
 }
 0x40a1da6 => {
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
pub fn f_040a1db0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1db0); }
 let mut pc: u32 = 0x40a1db0;
 loop { match pc {
 0x40a1db0 => {
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
  if !cpu.zf { pc = 0x40a1dcf; continue; }
  pc = 0x40a1dc8; continue;
 }
 0x40a1dc8 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x40a1dcf; continue;
 }
 0x40a1dcf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4du32); }
  if !cpu.zf { pc = 0x40a1ddb; continue; }
  pc = 0x40a1dd8; continue;
 }
 0x40a1dd8 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x40a1ddb; continue;
 }
 0x40a1ddb => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x40a1de9); f_040a1c63(cpu); if cpu.returned_to != 0x40a1de9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a1dfa; continue; }
  pc = 0x40a1df1; continue;
 }
 0x40a1df1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a1e47; continue;
 }
 0x40a1dfa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x40a1e40; continue; }
  pc = 0x40a1e0b; continue;
 }
 0x40a1e0b => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x40a1e17); f_040a1c63(cpu); if cpu.returned_to != 0x40a1e17 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x40a1e40; continue; }
  pc = 0x40a1e2b; continue;
 }
 0x40a1e2b => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x40a1e37); f_040a1c63(cpu); if cpu.returned_to != 0x40a1e37 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a1e40; continue;
 }
 0x40a1e40 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x40a1e47; continue;
 }
 0x40a1e47 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x40a1e98; continue; }
  pc = 0x40a1e62; continue;
 }
 0x40a1e62 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x40a1e6e); f_040a1c63(cpu); if cpu.returned_to != 0x40a1e6e { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x40a1e98; continue; }
  pc = 0x40a1e79; continue;
 }
 0x40a1e79 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x40a1e83); f_040a1c63(cpu); if cpu.returned_to != 0x40a1e83 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x40a1e98; continue; }
  pc = 0x40a1e8e; continue;
 }
 0x40a1e8e => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x40a1e96); f_040a1c63(cpu); if cpu.returned_to != 0x40a1e96 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x40a1e98; continue;
 }
 0x40a1e98 => {
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
pub fn f_040a1f82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a1f82); }
 let mut pc: u32 = 0x40a1f82;
 loop { match pc {
 0x40a1f82 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xa8c0u32); }
  if !cpu.cf { pc = 0x40a1faf; continue; }
  pc = 0x40a1f93; continue;
 }
 0x40a1f93 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a1faf; continue; }
  pc = 0x40a1f97; continue;
 }
 0x40a1f97 => {
  { let v: u32 = 0x15180u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x40a1fbe; continue;
 }
 0x40a1faf => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x40a1fbe; continue;
 }
 0x40a1fbe => {
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
  if (cpu.cf || cpu.zf) { pc = 0x40a2025; continue; }
  pc = 0x40a201d; continue;
 }
 0x40a201d => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x40a2025; continue;
 }
 0x40a2025 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x40a2030); f_040a20c2(cpu); if cpu.returned_to != 0x40a2030 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x16du32); cpu.eax = v; }
  pc = 0x40a2035; continue;
 }
 0x40a2035 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x40a203e; continue; }
  pc = 0x40a2039; continue;
 }
 0x40a2039 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x40a2035; continue;
 }
 0x40a203e => {
  { let v: u32 = 0x414b968u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x40a2054); f_040a20c2(cpu); if cpu.returned_to != 0x40a2054 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a205d; continue; }
  pc = 0x40a2058; continue;
 }
 0x40a2058 => {
  { let v: u32 = 0x414b982u32; cpu.esi = v; }
  pc = 0x40a205d; continue;
 }
 0x40a205d => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x40a2075; continue; }
  pc = 0x40a2074; continue;
 }
 0x40a2074 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x40a2075; continue;
 }
 0x40a2075 => {
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
pub fn f_040a2097(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2097); }
 let mut pc: u32 = 0x40a2097;
 loop { match pc {
 0x40a2097 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  cpu.push(0x40a20b2); f_040a1f82(cpu); if cpu.returned_to != 0x40a20b2 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a20c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a20c2); }
 let mut pc: u32 = 0x40a20c2;
 loop { match pc {
 0x40a20c2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x40a20f7; continue; }
  pc = 0x40a20cc; continue;
 }
 0x40a20cc => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a20e5; continue; }
  pc = 0x40a20dc; continue;
 }
 0x40a20dc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a20e5 => {
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x40a20dc; continue; }
  pc = 0x40a20f7; continue;
 }
 0x40a20f7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a20fd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a20fd); }
 let mut pc: u32 = 0x40a20fd;
 loop { match pc {
 0x40a20fd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x40a21ca; continue; }
  pc = 0x40a2112; continue;
 }
 0x40a2112 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x40a211d); f_040a20c2(cpu); if cpu.returned_to != 0x40a211d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a2134; continue; }
  pc = 0x40a2121; continue;
 }
 0x40a2121 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x414b982u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x414b980u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x40a2145; continue;
 }
 0x40a2134 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x414b968u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x414b966u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x40a2145; continue;
 }
 0x40a2145 => {
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
  cpu.push(0x40a2179); f_040a1a5e(cpu); if cpu.returned_to != 0x40a2179 { return; }
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
  if !cpu.zf { pc = 0x40a21b8; continue; }
  pc = 0x40a2199; continue;
 }
 0x40a2199 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a21b4; continue; }
  pc = 0x40a21ac; continue;
 }
 0x40a21ac => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  pc = 0x40a21b9; continue;
 }
 0x40a21b4 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x40a21b9; continue;
 }
 0x40a21b8 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x40a21b9; continue;
 }
 0x40a21b9 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x40a21d9; continue;
 }
 0x40a21ca => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x40a21d6; continue; }
  pc = 0x40a21cf; continue;
 }
 0x40a21cf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x40a21d9; continue;
 }
 0x40a21d6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40a21d9; continue;
 }
 0x40a21d9 => {
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
pub fn f_040a21e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a21e2); }
 let mut pc: u32 = 0x40a21e2;
 loop { match pc {
 0x40a21e2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a21f6; continue; }
  pc = 0x40a21ee; continue;
 }
 0x40a21ee => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a21f6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.ecx))); }
  if (cpu.sf != cpu.of) { pc = 0x40a2211; continue; }
  pc = 0x40a21fb; continue;
 }
 0x40a21fb => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x40a2202); f_040a20fd(cpu); if cpu.returned_to != 0x40a2202 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a220d); f_040a20fd(cpu); if cpu.returned_to != 0x40a220d { return; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a21ee; continue; }
  pc = 0x40a2211; continue;
 }
 0x40a2211 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2216(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2216); }
 let mut pc: u32 = 0x40a2216;
 loop { match pc {
 0x40a2216 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x415b8e2u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x40a24ca; continue; }
  pc = 0x40a2232; continue;
 }
 0x40a2232 => {
  { let v: u32 = 0x415b870u32; cpu.edx = v; }
  { let v: u32 = 0x415b84cu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  cpu.push(0x40a2244); f_040a21e2(cpu); if cpu.returned_to != 0x40a2244 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a2258; continue; }
  pc = 0x40a224c; continue;
 }
 0x40a224c => {
  { let v: u32 = 0x415b870u32; cpu.esi = v; }
  { let v: u32 = 0x415b84cu32; cpu.ebx = v; }
  pc = 0x40a2262; continue;
 }
 0x40a2258 => {
  { let v: u32 = 0x415b84cu32; cpu.esi = v; }
  { let v: u32 = 0x415b870u32; cpu.ebx = v; }
  pc = 0x40a2262; continue;
 }
 0x40a2262 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  cpu.push(0x40a2272); f_040a20c2(cpu); if cpu.returned_to != 0x40a2272 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a228a; continue; }
  pc = 0x40a227c; continue;
 }
 0x40a227c => {
  { let v: u32 = cpu.r32(0x414b982u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x414b980u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40a2296; continue;
 }
 0x40a228a => {
  { let v: u32 = cpu.r32(0x414b968u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x414b966u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x40a2296; continue;
 }
 0x40a2296 => {
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a2366; continue; }
  pc = 0x40a22b3; continue;
 }
 0x40a22b3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a22c4; continue; }
  pc = 0x40a22ba; continue;
 }
 0x40a22ba => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x40a239f; continue;
 }
 0x40a22c4 => {
  if !cpu.zf { pc = 0x40a239f; continue; }
  pc = 0x40a22ca; continue;
 }
 0x40a22ca => {
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
  if !cpu.zf { pc = 0x40a233f; continue; }
  pc = 0x40a231a; continue;
 }
 0x40a231a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x40a239f; continue; }
  pc = 0x40a232c; continue;
 }
 0x40a232c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x40a239f; continue; }
  pc = 0x40a233d; continue;
 }
 0x40a233d => {
  pc = 0x40a239b; continue;
 }
 0x40a233f => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a239f; continue; }
  pc = 0x40a2357; continue;
 }
 0x40a2357 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a239f; continue; }
  pc = 0x40a2364; continue;
 }
 0x40a2364 => {
  pc = 0x40a239b; continue;
 }
 0x40a2366 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40a238d; continue; }
  pc = 0x40a236e; continue;
 }
 0x40a236e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x40a237b); f_040a20c2(cpu); if cpu.returned_to != 0x40a237b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a238c; continue; }
  pc = 0x40a237f; continue;
 }
 0x40a237f => {
  { let v: u32 = cpu.r32(0x414b96au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a238c; continue; }
  pc = 0x40a238b; continue;
 }
 0x40a238b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40a238c; continue;
 }
 0x40a238c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x40a238d; continue;
 }
 0x40a238d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a239f; continue; }
  pc = 0x40a2394; continue;
 }
 0x40a2394 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  if !cpu.zf { pc = 0x40a239f; continue; }
  pc = 0x40a239b; continue;
 }
 0x40a239b => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x40a239f; continue;
 }
 0x40a239f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x40a23bc; continue; }
  pc = 0x40a23a6; continue;
 }
 0x40a23a6 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a23af); f_040a24d9(cpu); if cpu.returned_to != 0x40a23af { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  pc = 0x40a23bc; continue;
 }
 0x40a23bc => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x40a23d3; continue; }
  pc = 0x40a23c0; continue;
 }
 0x40a23c0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x40a24ca; continue; }
  pc = 0x40a23cc; continue;
 }
 0x40a23cc => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x40a24ca; continue;
 }
 0x40a23d3 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x40a2476; continue; }
  pc = 0x40a23e0; continue;
 }
 0x40a23e0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a23ee; continue; }
  pc = 0x40a23e7; continue;
 }
 0x40a23e7 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x40a24af; continue;
 }
 0x40a23ee => {
  if !cpu.zf { pc = 0x40a24af; continue; }
  pc = 0x40a23f4; continue;
 }
 0x40a23f4 => {
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
  if !cpu.zf { pc = 0x40a2457; continue; }
  pc = 0x40a2438; continue;
 }
 0x40a2438 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x40a2451; continue; }
  pc = 0x40a2447; continue;
 }
 0x40a2447 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x40a24af; continue;
 }
 0x40a2451 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a24af; continue; }
  pc = 0x40a2455; continue;
 }
 0x40a2455 => {
  pc = 0x40a24aa; continue;
 }
 0x40a2457 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x40a2447; continue; }
  pc = 0x40a2470; continue;
 }
 0x40a2470 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x40a24af; continue; }
  pc = 0x40a2474; continue;
 }
 0x40a2474 => {
  pc = 0x40a24aa; continue;
 }
 0x40a2476 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x40a249d; continue; }
  pc = 0x40a247e; continue;
 }
 0x40a247e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x40a248b); f_040a20c2(cpu); if cpu.returned_to != 0x40a248b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a249c; continue; }
  pc = 0x40a248f; continue;
 }
 0x40a248f => {
  { let v: u32 = cpu.r32(0x414b96au32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x40a249c; continue; }
  pc = 0x40a249b; continue;
 }
 0x40a249b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x40a249c; continue;
 }
 0x40a249c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x40a249d; continue;
 }
 0x40a249d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a24af; continue; }
  pc = 0x40a24a4; continue;
 }
 0x40a24a4 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x40a24af; continue; }
  pc = 0x40a24aa; continue;
 }
 0x40a24aa => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x40a24af; continue;
 }
 0x40a24af => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x40a24be; continue; }
  pc = 0x40a24b3; continue;
 }
 0x40a24b3 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x40a24bc); f_040a24d9(cpu); if cpu.returned_to != 0x40a24bc { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x40a24be; continue;
 }
 0x40a24be => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x40a24ca; continue; }
  pc = 0x40a24c6; continue;
 }
 0x40a24c6 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x40a24ca; continue;
 }
 0x40a24ca => {
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
pub fn f_040a24d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a24d9); }
 let mut pc: u32 = 0x40a24d9;
 loop { match pc {
 0x40a24d9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x40a24fd; continue; }
  pc = 0x40a24e9; continue;
 }
 0x40a24e9 => {
  if !cpu.zf { pc = 0x40a2502; continue; }
  pc = 0x40a24eb; continue;
 }
 0x40a24eb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x40a24fd; continue; }
  pc = 0x40a24f5; continue;
 }
 0x40a24f5 => {
  if !cpu.zf { pc = 0x40a2502; continue; }
  pc = 0x40a24f7; continue;
 }
 0x40a24f7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x40a2502; continue; }
  pc = 0x40a24fd; continue;
 }
 0x40a24fd => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x40a2502; continue;
 }
 0x40a2502 => {
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
pub fn f_040a250c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a250c); }
 let mut pc: u32 = 0x40a250c;
 loop { match pc {
 0x40a250c => {
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
pub fn f_040a2534(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2534); }
 let mut pc: u32 = 0x40a2534;
 loop { match pc {
 0x40a2534 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x40a2577; continue; }
  pc = 0x40a2548; continue;
 }
 0x40a2548 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x40a2570; continue; }
  pc = 0x40a254c; continue;
 }
 0x40a254c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x40a2560; continue; }
  pc = 0x40a2556; continue;
 }
 0x40a2556 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a25af; continue;
 }
 0x40a2560 => {
  if (cpu.cf || cpu.zf) { pc = 0x40a2569; continue; }
  pc = 0x40a2562; continue;
 }
 0x40a2562 => {
  { let v: u32 = 0x8047u32; cpu.eax = v; }
  pc = 0x40a25a1; continue;
 }
 0x40a2569 => {
  { let v: u32 = 0x4047u32; cpu.eax = v; }
  pc = 0x40a25a1; continue;
 }
 0x40a2570 => {
  { let v: u32 = 0x2047u32; cpu.eax = v; }
  pc = 0x40a25a1; continue;
 }
 0x40a2577 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.cf { pc = 0x40a258b; continue; }
  pc = 0x40a2581; continue;
 }
 0x40a2581 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x40a25af; continue;
 }
 0x40a258b => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x40a259c; continue; }
  pc = 0x40a2595; continue;
 }
 0x40a2595 => {
  { let v: u32 = 0x8107u32; cpu.eax = v; }
  pc = 0x40a25a1; continue;
 }
 0x40a259c => {
  { let v: u32 = 0x1107u32; cpu.eax = v; }
  pc = 0x40a25a1; continue;
 }
 0x40a25a1 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x40a25ac); f_040b1e7f(cpu); if cpu.returned_to != 0x40a25ac { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x40a25af; continue;
 }
 0x40a25af => {
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
pub fn f_040a25c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a25c8); }
 let mut pc: u32 = 0x40a25c8;
 loop { match pc {
 0x40a25c8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x415b8f4u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x40a25f7; continue; }
  pc = 0x40a25df; continue;
 }
 0x40a25df => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x40a25f1; continue; }
  pc = 0x40a25e3; continue;
 }
 0x40a25e3 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x40a25f1); f_040b1f5a(cpu); if cpu.returned_to != 0x40a25f1 { return; }
  pc = 0x40a25f1; continue;
 }
 0x40a25f1 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a25f7 => {
  { let a = cpu.st(0); let b = cpu.rf64(0x415b8fcu32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x40a260c; continue; }
  pc = 0x40a2608; continue;
 }
 0x40a2608 => {
  { cpu.fpush(0.0); }
  pc = 0x40a2620; continue;
 }
 0x40a260c => {
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
  pc = 0x40a2620; continue;
 }
 0x40a2620 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a2634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x40a2634); }
 let mut pc: u32 = 0x40a2634;
 loop { match pc {
 0x40a2634 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x40a263c; continue;
 }
 0x40a263c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x40a26ce; continue; }
  pc = 0x40a2648; continue;
 }
 0x40a2648 => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x40a2657; continue; }
  pc = 0x40a2654; continue;
 }
 0x40a2654 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2657 => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x415cce8u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x40a2666; continue; }
  pc = 0x40a2663; continue;
 }
 0x40a2663 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2666 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a26d8; continue; }
  pc = 0x40a2671; continue;
 }
 0x40a2671 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x40a26d8; continue; }
  pc = 0x40a2678; continue;
 }
 0x40a2678 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x40a26b7; continue; }
  pc = 0x40a26a0; continue;
 }
 0x40a26a0 => {
  { let v = cpu.st(0) * cpu.rf32(0x415ccf8u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x415ccf8u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a26b7 => {
  { let v = cpu.st(0) * cpu.rf32(0x415ccfcu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x415ccfcu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a26ce => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x40a26db; continue; }
  pc = 0x40a26d8; continue;
 }
 0x40a26d8 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a26db => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x40a26d8; continue; }
  pc = 0x40a26e6; continue;
 }
 0x40a26e6 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x40a271c; continue; }
  pc = 0x40a270b; continue;
 }
 0x40a270b => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x40a2744; continue; }
  pc = 0x40a2712; continue;
 }
 0x40a2712 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x40a2744; continue; }
  pc = 0x40a271a; continue;
 }
 0x40a271a => {
  pc = 0x40a2724; continue;
 }
 0x40a271c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x40a2744; continue; }
  pc = 0x40a2724; continue;
 }
 0x40a2724 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x415cd00u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  cpu.poll(); pc = 0x40a263c; continue;
 }
 0x40a2744 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_040a274b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a274b); } r_040a274b(cpu, 0x40a274b); }
pub fn f_040a2755(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2755); } r_040a274b(cpu, 0x40a2755); }
pub fn f_040a275b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a275b); } r_040a274b(cpu, 0x40a275b); }
pub fn f_040a2760(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2760); } r_040a274b(cpu, 0x40a2760); }
pub fn f_040a2766(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2766); } r_040a274b(cpu, 0x40a2766); }
pub fn f_040a276b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a276b); } r_040a274b(cpu, 0x40a276b); }
pub fn f_040a2771(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2771); } r_040a274b(cpu, 0x40a2771); }
pub fn f_040a2777(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2777); } r_040a274b(cpu, 0x40a2777); }
pub fn f_040a277d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a277d); } r_040a274b(cpu, 0x40a277d); }
pub fn f_040a2783(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2783); } r_040a274b(cpu, 0x40a2783); }
pub fn f_040a279f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a279f); } r_040a274b(cpu, 0x40a279f); }
pub fn f_040a27a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a27a4); } r_040a274b(cpu, 0x40a27a4); }
pub fn f_040a27ba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a27ba); } r_040a274b(cpu, 0x40a27ba); }
pub fn f_040a27bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a27bf); } r_040a274b(cpu, 0x40a27bf); }
pub fn f_040a27db(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a27db); } r_040a274b(cpu, 0x40a27db); }
pub fn f_040a27eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a27eb); } r_040a274b(cpu, 0x40a27eb); }
pub fn f_040a27ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a27ff); } r_040a274b(cpu, 0x40a27ff); }
pub fn f_040a280f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a280f); } r_040a274b(cpu, 0x40a280f); }
pub fn f_040a282f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a282f); } r_040a274b(cpu, 0x40a282f); }
pub fn f_040a2834(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2834); } r_040a274b(cpu, 0x40a2834); }
pub fn f_040a284e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a284e); } r_040a274b(cpu, 0x40a284e); }
pub fn f_040a2853(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2853); } r_040a274b(cpu, 0x40a2853); }
pub fn f_040a2873(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2873); } r_040a274b(cpu, 0x40a2873); }
pub fn f_040a2887(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2887); } r_040a274b(cpu, 0x40a2887); }
pub fn f_040a289f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a289f); } r_040a274b(cpu, 0x40a289f); }
pub fn f_040a28b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a28b3); } r_040a274b(cpu, 0x40a28b3); }
pub fn f_040a28d3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a28d3); } r_040a274b(cpu, 0x40a28d3); }
pub fn f_040a28d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a28d8); } r_040a274b(cpu, 0x40a28d8); }
pub fn f_040a28f2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a28f2); } r_040a274b(cpu, 0x40a28f2); }
pub fn f_040a28f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a28f7); } r_040a274b(cpu, 0x40a28f7); }
pub fn f_040a2917(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2917); } r_040a274b(cpu, 0x40a2917); }
pub fn f_040a292b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a292b); } r_040a274b(cpu, 0x40a292b); }
pub fn f_040a2943(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2943); } r_040a274b(cpu, 0x40a2943); }
pub fn f_040a2957(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2957); } r_040a274b(cpu, 0x40a2957); }
pub fn f_040a2977(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2977); } r_040a274b(cpu, 0x40a2977); }
pub fn f_040a297c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a297c); } r_040a274b(cpu, 0x40a297c); }
pub fn f_040a2996(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2996); } r_040a274b(cpu, 0x40a2996); }
pub fn f_040a299b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a299b); } r_040a274b(cpu, 0x40a299b); }
pub fn f_040a29bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a29bb); } r_040a274b(cpu, 0x40a29bb); }
pub fn f_040a29cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a29cf); } r_040a274b(cpu, 0x40a29cf); }
pub fn f_040a29e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a29e7); } r_040a274b(cpu, 0x40a29e7); }
pub fn f_040a29fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a29fb); } r_040a274b(cpu, 0x40a29fb); }
pub fn f_040a2a1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a1b); } r_040a274b(cpu, 0x40a2a1b); }
pub fn f_040a2a20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a20); } r_040a274b(cpu, 0x40a2a20); }
pub fn f_040a2a3a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a3a); } r_040a274b(cpu, 0x40a2a3a); }
pub fn f_040a2a3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a3f); } r_040a274b(cpu, 0x40a2a3f); }
pub fn f_040a2a5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a5f); } r_040a274b(cpu, 0x40a2a5f); }
pub fn f_040a2a73(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a73); } r_040a274b(cpu, 0x40a2a73); }
pub fn f_040a2a8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a8b); } r_040a274b(cpu, 0x40a2a8b); }
pub fn f_040a2a9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2a9f); } r_040a274b(cpu, 0x40a2a9f); }
pub fn f_040a2abf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2abf); } r_040a274b(cpu, 0x40a2abf); }
pub fn f_040a2ac4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2ac4); } r_040a274b(cpu, 0x40a2ac4); }
pub fn f_040a2ade(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2ade); } r_040a274b(cpu, 0x40a2ade); }
pub fn f_040a2ae3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2ae3); } r_040a274b(cpu, 0x40a2ae3); }
pub fn f_040a2b03(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b03); } r_040a274b(cpu, 0x40a2b03); }
pub fn f_040a2b17(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b17); } r_040a274b(cpu, 0x40a2b17); }
pub fn f_040a2b2f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b2f); } r_040a274b(cpu, 0x40a2b2f); }
pub fn f_040a2b43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b43); } r_040a274b(cpu, 0x40a2b43); }
pub fn f_040a2b63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b63); } r_040a274b(cpu, 0x40a2b63); }
pub fn f_040a2b68(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b68); } r_040a274b(cpu, 0x40a2b68); }
pub fn f_040a2b82(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b82); } r_040a274b(cpu, 0x40a2b82); }
pub fn f_040a2b87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2b87); } r_040a274b(cpu, 0x40a2b87); }
pub fn f_040a2ba7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2ba7); } r_040a274b(cpu, 0x40a2ba7); }
pub fn f_040a2bbb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2bbb); } r_040a274b(cpu, 0x40a2bbb); }
pub fn f_040a2bd3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x40a2bd3); } r_040a274b(cpu, 0x40a2bd3); }
fn r_040a274b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x40a274b => {
  { let v: u32 = cpu.sub32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.poll(); pc = cpu.r32(0x415cd04u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x40a2755 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a275b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2760; continue;
 }
 0x40a2760 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2766 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a276b; continue;
 }
 0x40a276b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2771 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2777 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a277d => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2783 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2795); f_040a2634(cpu); if cpu.returned_to != 0x40a2795 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a279f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a27a4; continue;
 }
 0x40a27a4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a27b0); f_040a2634(cpu); if cpu.returned_to != 0x40a27b0 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a27ba => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a27bf; continue;
 }
 0x40a27bf => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a27d3); f_040a2634(cpu); if cpu.returned_to != 0x40a27d3 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a27db => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a27e7); f_040a2634(cpu); if cpu.returned_to != 0x40a27e7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a27eb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a27f7); f_040a2634(cpu); if cpu.returned_to != 0x40a27f7 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a27ff => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a280b); f_040a2634(cpu); if cpu.returned_to != 0x40a280b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a280f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2823); f_040a2634(cpu); if cpu.returned_to != 0x40a2823 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a282f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2834; continue;
 }
 0x40a2834 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2842); f_040a2634(cpu); if cpu.returned_to != 0x40a2842 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a284e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2853; continue;
 }
 0x40a2853 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2869); f_040a2634(cpu); if cpu.returned_to != 0x40a2869 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2873 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2881); f_040a2634(cpu); if cpu.returned_to != 0x40a2881 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2887 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2895); f_040a2634(cpu); if cpu.returned_to != 0x40a2895 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a289f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a28ad); f_040a2634(cpu); if cpu.returned_to != 0x40a28ad { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a28b3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a28c7); f_040a2634(cpu); if cpu.returned_to != 0x40a28c7 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a28d3 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a28d8; continue;
 }
 0x40a28d8 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a28e6); f_040a2634(cpu); if cpu.returned_to != 0x40a28e6 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a28f2 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a28f7; continue;
 }
 0x40a28f7 => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a290d); f_040a2634(cpu); if cpu.returned_to != 0x40a290d { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2917 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2925); f_040a2634(cpu); if cpu.returned_to != 0x40a2925 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a292b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2939); f_040a2634(cpu); if cpu.returned_to != 0x40a2939 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2943 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2951); f_040a2634(cpu); if cpu.returned_to != 0x40a2951 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2957 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a296b); f_040a2634(cpu); if cpu.returned_to != 0x40a296b { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2977 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a297c; continue;
 }
 0x40a297c => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a298a); f_040a2634(cpu); if cpu.returned_to != 0x40a298a { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2996 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a299b; continue;
 }
 0x40a299b => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a29b1); f_040a2634(cpu); if cpu.returned_to != 0x40a29b1 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a29bb => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a29c9); f_040a2634(cpu); if cpu.returned_to != 0x40a29c9 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a29cf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a29dd); f_040a2634(cpu); if cpu.returned_to != 0x40a29dd { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a29e7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a29f5); f_040a2634(cpu); if cpu.returned_to != 0x40a29f5 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a29fb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2a0f); f_040a2634(cpu); if cpu.returned_to != 0x40a2a0f { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2a1b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2a20; continue;
 }
 0x40a2a20 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2a2e); f_040a2634(cpu); if cpu.returned_to != 0x40a2a2e { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2a3a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2a3f; continue;
 }
 0x40a2a3f => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2a55); f_040a2634(cpu); if cpu.returned_to != 0x40a2a55 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2a5f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2a6d); f_040a2634(cpu); if cpu.returned_to != 0x40a2a6d { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2a73 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2a81); f_040a2634(cpu); if cpu.returned_to != 0x40a2a81 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2a8b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2a99); f_040a2634(cpu); if cpu.returned_to != 0x40a2a99 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2a9f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2ab3); f_040a2634(cpu); if cpu.returned_to != 0x40a2ab3 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2abf => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2ac4; continue;
 }
 0x40a2ac4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2ad2); f_040a2634(cpu); if cpu.returned_to != 0x40a2ad2 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2ade => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2ae3; continue;
 }
 0x40a2ae3 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2af9); f_040a2634(cpu); if cpu.returned_to != 0x40a2af9 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2b03 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2b11); f_040a2634(cpu); if cpu.returned_to != 0x40a2b11 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2b17 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2b25); f_040a2634(cpu); if cpu.returned_to != 0x40a2b25 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2b2f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2b3d); f_040a2634(cpu); if cpu.returned_to != 0x40a2b3d { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2b43 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2b57); f_040a2634(cpu); if cpu.returned_to != 0x40a2b57 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2b63 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2b68; continue;
 }
 0x40a2b68 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2b76); f_040a2634(cpu); if cpu.returned_to != 0x40a2b76 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2b82 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x40a2b87; continue;
 }
 0x40a2b87 => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2b9d); f_040a2634(cpu); if cpu.returned_to != 0x40a2b9d { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2ba7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2bb5); f_040a2634(cpu); if cpu.returned_to != 0x40a2bb5 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2bbb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2bc9); f_040a2634(cpu); if cpu.returned_to != 0x40a2bc9 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x40a2bd3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x40a2be1); f_040a2634(cpu); if cpu.returned_to != 0x40a2be1 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
