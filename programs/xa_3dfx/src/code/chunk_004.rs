#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010b01de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b01de); }
 let mut pc: u32 = 0x10b01de;
 loop { match pc {
 0x10b01de => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10b01f9); f_010b2e43(cpu); if cpu.returned_to != 0x10b01f9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0290); }
 let mut pc: u32 = 0x10b0290;
 loop { match pc {
 0x10b0290 => {
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
  pc = 0x10b029e; continue;
 }
 0x10b029e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b02c2; continue; }
  pc = 0x10b02a4; continue;
 }
 0x10b02a4 => {
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
  cpu.poll(); pc = 0x10b029e; continue;
 }
 0x10b02c2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x43300000u32; cpu.edi = v; }
  cpu.push(0x10b02ce); f_010b02db(cpu); if cpu.returned_to != 0x10b02ce { return; }
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
pub fn f_010b02db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b02db); }
 let mut pc: u32 = 0x10b02db;
 loop { match pc {
 0x10b02db => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.edx); cpu.esi = v; }
  if cpu.zf { pc = 0x10b032b; continue; }
  pc = 0x10b02e3; continue;
 }
 0x10b02e3 => {
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if !cpu.zf { pc = 0x10b02ff; continue; }
  pc = 0x10b02eb; continue;
 }
 0x10b02eb => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if cpu.zf { cpu.poll(); pc = 0x10b02eb; continue; }
  pc = 0x10b02fd; continue;
 }
 0x10b02fd => {
  pc = 0x10b0323; continue;
 }
 0x10b02ff => {
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if cpu.zf { pc = 0x10b0323; continue; }
  pc = 0x10b0307; continue;
 }
 0x10b0307 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b0307; continue; }
  pc = 0x10b031b; continue;
 }
 0x10b031b => {
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x10b0323; continue;
 }
 0x10b0323 => {
  { let v: u32 = cpu.edx & 0xfffffu32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edi); cpu.edx = v; }
  pc = 0x10b032b; continue;
 }
 0x10b032b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b032c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b032c); }
 let mut pc: u32 = 0x10b032c;
 loop { match pc {
 0x10b032c => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_010b0331(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0331(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0331); }
 let mut pc: u32 = 0x10b0331;
 loop { match pc {
 0x10b0331 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x10b0339); f_010b2a07(cpu); if cpu.returned_to != 0x10b0339 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b033d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b033d); }
 let mut pc: u32 = 0x10b033d;
 loop { match pc {
 0x10b033d => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_010b0331(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0344(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0344); }
 let mut pc: u32 = 0x10b0344;
 loop { match pc {
 0x10b0344 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x10b034e); f_010b0331(cpu); if cpu.returned_to != 0x10b034e { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0354(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0354); }
 let mut pc: u32 = 0x10b0354;
 loop { match pc {
 0x10b0354 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x10b035c); f_010b2a0d(cpu); if cpu.returned_to != 0x10b035c { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0360); }
 let mut pc: u32 = 0x10b0360;
 loop { match pc {
 0x10b0360 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x10b0386; continue; }
  pc = 0x10b0371; continue;
 }
 0x10b0371 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b0383); f_010b03a1(cpu); if cpu.returned_to != 0x10b0383 { return; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x10b0386; continue;
 }
 0x10b0386 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10b0390); f_010bf7bc(cpu); if cpu.returned_to != 0x10b0390 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b0399); f_010bf782(cpu); if cpu.returned_to != 0x10b0399 { return; }
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
pub fn f_010b03a1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b03a1); }
 let mut pc: u32 = 0x10b03a1;
 loop { match pc {
 0x10b03a1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x116b788u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x10b03cc; continue; }
  pc = 0x10b03bc; continue;
 }
 0x10b03bc => {
  { let v: u32 = cpu.r32(0x116b798u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x116b79cu32); cpu.eax = v; }
  pc = 0x10b03e9; continue;
 }
 0x10b03cc => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x116b790u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x10b03ed; continue; }
  pc = 0x10b03db; continue;
 }
 0x10b03db => {
  { let v: u32 = cpu.r32(0x116b7a0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x116b7a4u32); cpu.eax = v; }
  pc = 0x10b03e9; continue;
 }
 0x10b03e9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b03ed; continue;
 }
 0x10b03ed => {
  { let v: u32 = 0x116b828u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10b0430; continue; }
  pc = 0x10b03f6; continue;
 }
 0x10b03f6 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b044e; continue; }
  pc = 0x10b0402; continue;
 }
 0x10b0402 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x117bb1cu32), 0x1u8); }
  if cpu.zf { pc = 0x10b0422; continue; }
  pc = 0x10b040f; continue;
 }
 0x10b040f => {
  { let v = cpu.r32(0x116b82eu32); cpu.push(v); }
  { let v = cpu.r32(0x116b82au32); cpu.push(v); }
  cpu.push(0x10b0420); f_010b439c(cpu); if cpu.returned_to != 0x10b0420 { return; }
  pc = 0x10b0428; continue;
 }
 0x10b0422 => {
  { let v = cpu.st(0) / cpu.rf64(0x116b82au32); cpu.set_st(0, v); }
  pc = 0x10b0428; continue;
 }
 0x10b0428 => {
  { let v: u32 = cpu.edx.wrapping_sub(0xd8u32); cpu.edx = v; }
  pc = 0x10b044a; continue;
 }
 0x10b0430 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b044e; continue; }
  pc = 0x10b043a; continue;
 }
 0x10b043a => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x116b82au32); cpu.set_st(0, v); }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x10b044a; continue;
 }
 0x10b044a => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x10b044e; continue;
 }
 0x10b044e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b045c; continue;
 }
 0x10b045c => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x10b046e; continue; }
  pc = 0x10b0463; continue;
 }
 0x10b0463 => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x10b046e; continue;
 }
 0x10b046e => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b047f; continue; }
  pc = 0x10b0472; continue;
 }
 0x10b0472 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x10b045c; continue; }
  pc = 0x10b047a; continue;
 }
 0x10b047a => {
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.poll(); pc = 0x10b045c; continue;
 }
 0x10b047f => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10b04a4; continue; }
  pc = 0x10b0483; continue;
 }
 0x10b0483 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x117bb1cu32), 0x1u8); }
  if cpu.zf { pc = 0x10b049f; continue; }
  pc = 0x10b0490; continue;
 }
 0x10b0490 => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x10b049d); f_010b439c(cpu); if cpu.returned_to != 0x10b049d { return; }
  pc = 0x10b04ab; continue;
 }
 0x10b049f => {
  { let v = cpu.st(0) / cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x10b04ab; continue;
 }
 0x10b04a4 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x10b04ab; continue;
 }
 0x10b04ab => {
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
pub fn f_010b04be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b04be); }
 let mut pc: u32 = 0x10b04be;
 loop { match pc {
 0x10b04be => {
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
  pc = 0x10b0590; continue;
 }
 0x10b04fd => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x10b050b); f_010b085d(cpu); if cpu.returned_to != 0x10b050b { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x115u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b084b; continue; }
  pc = 0x10b051f; continue;
 }
 0x10b051f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x10b065a; continue; }
  pc = 0x10b0531; continue;
 }
 0x10b0531 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b05c7; continue; }
  pc = 0x10b0559; continue;
 }
 0x10b0559 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b0577; continue; }
  pc = 0x10b0561; continue;
 }
 0x10b0561 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x10b056b; continue;
 }
 0x10b056b => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b0590; continue;
 }
 0x10b0577 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b05bc; continue; }
  pc = 0x10b057e; continue;
 }
 0x10b057e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x10b0587; continue;
 }
 0x10b0587 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b0590; continue;
 }
 0x10b0590 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10b084b; continue; }
  pc = 0x10b059a; continue;
 }
 0x10b059a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x25u32); }
  if cpu.zf { cpu.poll(); pc = 0x10b04fd; continue; }
  pc = 0x10b05aa; continue;
 }
 0x10b05aa => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b05ba); cpu.call(t); if cpu.returned_to != 0x10b05ba { return; }
  cpu.poll(); pc = 0x10b0590; continue;
 }
 0x10b05bc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x10b0587; continue;
 }
 0x10b05c7 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0622; continue; }
  pc = 0x10b05d9; continue;
 }
 0x10b05d9 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b05f8; continue; }
  pc = 0x10b05e1; continue;
 }
 0x10b05e1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.eax); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x10b0590; continue;
 }
 0x10b05f8 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b0617; continue; }
  pc = 0x10b05ff; continue;
 }
 0x10b05ff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x10b0608; continue;
 }
 0x10b0608 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x10b0590; continue;
 }
 0x10b0617 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.poll(); pc = 0x10b0608; continue;
 }
 0x10b0622 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b0637; continue; }
  pc = 0x10b0628; continue;
 }
 0x10b0628 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  cpu.poll(); pc = 0x10b056b; continue;
 }
 0x10b0637 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b064c; continue; }
  pc = 0x10b063e; continue;
 }
 0x10b063e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.poll(); pc = 0x10b0587; continue;
 }
 0x10b064c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.poll(); pc = 0x10b0587; continue;
 }
 0x10b065a => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x10b0671); f_010b0b9b(cpu); if cpu.returned_to != 0x10b0671 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10b06bd; continue; }
  pc = 0x10b0685; continue;
 }
 0x10b0685 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x10b06bd; continue; }
  pc = 0x10b0691; continue;
 }
 0x10b0691 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10b06bd; continue; }
  pc = 0x10b06a4; continue;
 }
 0x10b06a4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b06bb); cpu.call(t); if cpu.returned_to != 0x10b06bb { return; }
  cpu.poll(); pc = 0x10b0691; continue;
 }
 0x10b06bd => {
  { let v: u32 = 0x117u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x10b06c4; continue;
 }
 0x10b06c4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b06df; continue; }
  pc = 0x10b06ca; continue;
 }
 0x10b06ca => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b06dd); cpu.call(t); if cpu.returned_to != 0x10b06dd { return; }
  cpu.poll(); pc = 0x10b06c4; continue;
 }
 0x10b06df => {
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b0709; continue; }
  pc = 0x10b06f4; continue;
 }
 0x10b06f4 => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b0707); cpu.call(t); if cpu.returned_to != 0x10b0707 { return; }
  cpu.poll(); pc = 0x10b06df; continue;
 }
 0x10b0709 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10b0751; continue; }
  pc = 0x10b0719; continue;
 }
 0x10b0719 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.zf { pc = 0x10b0751; continue; }
  pc = 0x10b0725; continue;
 }
 0x10b0725 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10b0751; continue; }
  pc = 0x10b0738; continue;
 }
 0x10b0738 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b074f); cpu.call(t); if cpu.returned_to != 0x10b074f { return; }
  cpu.poll(); pc = 0x10b0725; continue;
 }
 0x10b0751 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.zf { pc = 0x10b0764; continue; }
  pc = 0x10b075f; continue;
 }
 0x10b075f => {
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x10b07d0; continue; }
  pc = 0x10b0764; continue;
 }
 0x10b0764 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b07a5; continue; }
  pc = 0x10b0774; continue;
 }
 0x10b0774 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b07fb; continue; }
  pc = 0x10b078d; continue;
 }
 0x10b078d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b07a0); cpu.call(t); if cpu.returned_to != 0x10b07a0 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b0774; continue;
 }
 0x10b07a5 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b07fb; continue; }
  pc = 0x10b07ba; continue;
 }
 0x10b07ba => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b07cd); cpu.call(t); if cpu.returned_to != 0x10b07cd { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b07a5; continue;
 }
 0x10b07d0 => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b07fb; continue; }
  pc = 0x10b07e5; continue;
 }
 0x10b07e5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b07f8); cpu.call(t); if cpu.returned_to != 0x10b07f8 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b07d0; continue;
 }
 0x10b07fb => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x10b0590; continue; }
  pc = 0x10b080f; continue;
 }
 0x10b080f => {
  { cpu.sub32(cpu.r32(0x104u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b0590; continue; }
  pc = 0x10b081d; continue;
 }
 0x10b081d => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x10b0590; continue; }
  pc = 0x10b0836; continue;
 }
 0x10b0836 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x10b0849); cpu.call(t); if cpu.returned_to != 0x10b0849 { return; }
  cpu.poll(); pc = 0x10b081d; continue;
 }
 0x10b084b => {
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
pub fn f_010b085d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b085d); }
 let mut pc: u32 = 0x10b085d;
 loop { match pc {
 0x10b085d => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b0871); f_010b099b(cpu); if cpu.returned_to != 0x10b0871 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b08ab; continue; }
  pc = 0x10b0885; continue;
 }
 0x10b0885 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b08ab; continue; }
  pc = 0x10b088a; continue;
 }
 0x10b088a => {
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
  if (cpu.sf != cpu.of) { pc = 0x10b08d6; continue; }
  pc = 0x10b08a4; continue;
 }
 0x10b08a4 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b088a; continue; }
  pc = 0x10b08a9; continue;
 }
 0x10b08a9 => {
  pc = 0x10b08d6; continue;
 }
 0x10b08ab => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x10b08d6; continue; }
  pc = 0x10b08b4; continue;
 }
 0x10b08b4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b08d5; continue; }
  pc = 0x10b08c5; continue;
 }
 0x10b08c5 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8) | 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b08d5; continue;
 }
 0x10b08d5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10b08d6; continue;
 }
 0x10b08d6 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x10b092f; continue; }
  pc = 0x10b08e6; continue;
 }
 0x10b08e6 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x10b0912; continue; }
  pc = 0x10b08f7; continue;
 }
 0x10b08f7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b090f; continue; }
  pc = 0x10b0908; continue;
 }
 0x10b0908 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b090f; continue;
 }
 0x10b090f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10b092f; continue;
 }
 0x10b0912 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b092f; continue; }
  pc = 0x10b091b; continue;
 }
 0x10b091b => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b092f; continue; }
  pc = 0x10b0920; continue;
 }
 0x10b0920 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10b0912; continue;
 }
 0x10b092f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4eu8); }
  if cpu.cf { pc = 0x10b0954; continue; }
  pc = 0x10b0939; continue;
 }
 0x10b0939 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0991; continue; }
  pc = 0x10b093b; continue;
 }
 0x10b093b => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x10b094b; continue; }
  pc = 0x10b0940; continue;
 }
 0x10b0940 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0964; continue; }
  pc = 0x10b0942; continue;
 }
 0x10b0942 => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x10b0964; continue; }
  pc = 0x10b0947; continue;
 }
 0x10b0947 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b094b => {
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.zf { pc = 0x10b0972; continue; }
  pc = 0x10b0950; continue;
 }
 0x10b0950 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b0954 => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x10b0997; continue; }
  pc = 0x10b0959; continue;
 }
 0x10b0959 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0987; continue; }
  pc = 0x10b095b; continue;
 }
 0x10b095b => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x10b0978; continue; }
  pc = 0x10b0960; continue;
 }
 0x10b0960 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b0964 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10b096b; continue;
 }
 0x10b096b => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b0972 => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x10u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b0995; continue;
 }
 0x10b0978 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b0987 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.poll(); pc = 0x10b096b; continue;
 }
 0x10b0991 => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b0995; continue;
 }
 0x10b0995 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10b0997; continue;
 }
 0x10b0997 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b099b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b099b); }
 let mut pc: u32 = 0x10b099b;
 loop { match pc {
 0x10b099b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b09a3; continue;
 }
 0x10b09a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x10b09b2; continue; }
  pc = 0x10b09ac; continue;
 }
 0x10b09ac => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b09f2; continue;
 }
 0x10b09b2 => {
  { cpu.sub32(cpu.eax, 0x23u32); }
  if !cpu.zf { pc = 0x10b09bd; continue; }
  pc = 0x10b09b7; continue;
 }
 0x10b09b7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b09f2; continue;
 }
 0x10b09bd => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x10b09d2; continue; }
  pc = 0x10b09c2; continue;
 }
 0x10b09c2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b09f2; continue;
 }
 0x10b09d2 => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x10b09e9; continue; }
  pc = 0x10b09d7; continue;
 }
 0x10b09d7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10b09f2; continue; }
  pc = 0x10b09e3; continue;
 }
 0x10b09e3 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x2u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b09f2; continue;
 }
 0x10b09e9 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x10b09f5; continue; }
  pc = 0x10b09ee; continue;
 }
 0x10b09ee => {
  { let v: u8 = 0x30u8; cpu.w8(0x16u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b09f2; continue;
 }
 0x10b09f2 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b09a3; continue;
 }
 0x10b09f5 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b09fa(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b09fa); } r_010b09fa(cpu, 0x10b09fa); }
pub fn f_010b0a1e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b0a1e); } r_010b09fa(cpu, 0x10b0a1e); }
fn r_010b09fa(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b09fa => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b0a08; continue;
 }
 0x10b0a08 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x10b0a19; continue; }
  pc = 0x10b0a12; continue;
 }
 0x10b0a12 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10b0a19; continue; }
  pc = 0x10b0a16; continue;
 }
 0x10b0a16 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10b0a08; continue;
 }
 0x10b0a19 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b0a1e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b0a2c; continue;
 }
 0x10b0a2c => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x10b0a19; continue; }
  pc = 0x10b0a3a; continue;
 }
 0x10b0a3a => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x10b0a19; continue; }
  pc = 0x10b0a3e; continue;
 }
 0x10b0a3e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10b0a2c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0a41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0a41); }
 let mut pc: u32 = 0x10b0a41;
 loop { match pc {
 0x10b0a41 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x10b0a59); f_010aee0c(cpu); if cpu.returned_to != 0x10b0a59 { return; }
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
  pc = 0x10b0a7e; continue;
 }
 0x10b0a77 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10b0a7e; continue;
 }
 0x10b0a7e => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x10b0a77; continue; }
  pc = 0x10b0a82; continue;
 }
 0x10b0a82 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x10b0a85; continue;
 }
 0x10b0a85 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10b0a90; continue; }
  pc = 0x10b0a89; continue;
 }
 0x10b0a89 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x10b0a85; continue;
 }
 0x10b0a90 => {
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
pub fn f_010b0a9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0a9f); }
 let mut pc: u32 = 0x10b0a9f;
 loop { match pc {
 0x10b0a9f => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b0abb; continue; }
  pc = 0x10b0ab0; continue;
 }
 0x10b0ab0 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b0abb; continue;
 }
 0x10b0abb => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b0ac8; continue; }
  pc = 0x10b0ac1; continue;
 }
 0x10b0ac1 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b0ac8; continue;
 }
 0x10b0ac8 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b0adb); f_010aee0c(cpu); if cpu.returned_to != 0x10b0adb { return; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x10b0add; continue;
 }
 0x10b0add => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10b0aea; continue; }
  pc = 0x10b0ae6; continue;
 }
 0x10b0ae6 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.poll(); pc = 0x10b0add; continue;
 }
 0x10b0aea => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b0b1c; continue; }
  pc = 0x10b0af0; continue;
 }
 0x10b0af0 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x10b0b14; continue;
 }
 0x10b0af9 => {
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
  pc = 0x10b0b14; continue;
 }
 0x10b0b14 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b0af9; continue; }
  pc = 0x10b0b19; continue;
 }
 0x10b0b19 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b0b1c; continue;
 }
 0x10b0b1c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b0b8d; continue; }
  pc = 0x10b0b2f; continue;
 }
 0x10b0b2f => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x10b0b6c; continue; }
  pc = 0x10b0b33; continue;
 }
 0x10b0b33 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b0b39; continue;
 }
 0x10b0b39 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x10b0b49; continue; }
  pc = 0x10b0b45; continue;
 }
 0x10b0b45 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x10b0b39; continue;
 }
 0x10b0b49 => {
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x10b0b63; continue; }
  pc = 0x10b0b4e; continue;
 }
 0x10b0b4e => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b0b57; continue;
 }
 0x10b0b57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x10b0b63; continue; }
  pc = 0x10b0b60; continue;
 }
 0x10b0b60 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x10b0b57; continue;
 }
 0x10b0b63 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b0b8d; continue;
 }
 0x10b0b6c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x10b0b77; continue; }
  pc = 0x10b0b76; continue;
 }
 0x10b0b76 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10b0b77; continue;
 }
 0x10b0b77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if cpu.zf { pc = 0x10b0b88; continue; }
  pc = 0x10b0b80; continue;
 }
 0x10b0b80 => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b0b8d; continue;
 }
 0x10b0b88 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.poll(); pc = 0x10b0b2f; continue;
 }
 0x10b0b8d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0b94(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0b94); }
 let mut pc: u32 = 0x10b0b94;
 loop { match pc {
 0x10b0b94 => {
  let t = cpu.r32(0x117c95cu32); cpu.push(0x10b0b9a); cpu.call(t); if cpu.returned_to != 0x10b0b9a { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b0b9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b0b9b); }
 let mut pc: u32 = 0x10b0b9b;
 loop { match pc {
 0x10b0b9b => {
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
  if cpu.cf { pc = 0x10b0be4; continue; }
  pc = 0x10b0bc8; continue;
 }
 0x10b0bc8 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0c35; continue; }
  pc = 0x10b0bce; continue;
 }
 0x10b0bce => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x10b0bdf; continue; }
  pc = 0x10b0bd3; continue;
 }
 0x10b0bd3 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0bf2; continue; }
  pc = 0x10b0bd5; continue;
 }
 0x10b0bd5 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  pc = 0x10b0bd8; continue;
 }
 0x10b0bd8 => {
  if cpu.zf { pc = 0x10b0bf2; continue; }
  pc = 0x10b0bda; continue;
 }
 0x10b0bda => {
  pc = 0x10b0c3f; continue;
 }
 0x10b0bdf => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  cpu.poll(); pc = 0x10b0bd8; continue;
 }
 0x10b0be4 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x10b0c3f; continue; }
  pc = 0x10b0be9; continue;
 }
 0x10b0be9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0bf2; continue; }
  pc = 0x10b0beb; continue;
 }
 0x10b0beb => {
  { cpu.sub8((cpu.edx as u8), 0x64u8); }
  if cpu.zf { pc = 0x10b0c35; continue; }
  pc = 0x10b0bf0; continue;
 }
 0x10b0bf0 => {
  pc = 0x10b0c3f; continue;
 }
 0x10b0bf2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0c0c; continue; }
  pc = 0x10b0c00; continue;
 }
 0x10b0c00 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  pc = 0x10b0c31; continue;
 }
 0x10b0c0c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0c27; continue; }
  pc = 0x10b0c1a; continue;
 }
 0x10b0c1a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  pc = 0x10b0c31; continue;
 }
 0x10b0c27 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x10b0c31; continue;
 }
 0x10b0c31 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b0c35; continue;
 }
 0x10b0c35 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x10b0c3f; continue; }
  pc = 0x10b0c3b; continue;
 }
 0x10b0c3b => {
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b0c3f; continue;
 }
 0x10b0c3f => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.cf { pc = 0x10b0cac; continue; }
  pc = 0x10b0c4c; continue;
 }
 0x10b0c4c => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0d2c; continue; }
  pc = 0x10b0c52; continue;
 }
 0x10b0c52 => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  if cpu.cf { pc = 0x10b0c93; continue; }
  pc = 0x10b0c57; continue;
 }
 0x10b0c57 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0d42; continue; }
  pc = 0x10b0c5d; continue;
 }
 0x10b0c5d => {
  { cpu.sub8((cpu.edx as u8), 0x73u8); }
  if cpu.cf { pc = 0x10b0c85; continue; }
  pc = 0x10b0c62; continue;
 }
 0x10b0c62 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0dd7; continue; }
  pc = 0x10b0c68; continue;
 }
 0x10b0c68 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x10b0fa9; continue; }
  pc = 0x10b0c71; continue;
 }
 0x10b0c71 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0ec3; continue; }
  pc = 0x10b0c77; continue;
 }
 0x10b0c77 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  if cpu.zf { pc = 0x10b0ea0; continue; }
  pc = 0x10b0c80; continue;
 }
 0x10b0c80 => {
  pc = 0x10b0fa9; continue;
 }
 0x10b0c85 => {
  { cpu.sub8((cpu.edx as u8), 0x70u8); }
  if cpu.zf { pc = 0x10b0ee6; continue; }
  pc = 0x10b0c8e; continue;
 }
 0x10b0c8e => {
  pc = 0x10b0fa9; continue;
 }
 0x10b0c93 => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.cf { pc = 0x10b0cfa; continue; }
  pc = 0x10b0c98; continue;
 }
 0x10b0c98 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0d2c; continue; }
  pc = 0x10b0c9e; continue;
 }
 0x10b0c9e => {
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.zf { pc = 0x10b0d86; continue; }
  pc = 0x10b0ca7; continue;
 }
 0x10b0ca7 => {
  pc = 0x10b0fa9; continue;
 }
 0x10b0cac => {
  { cpu.sub8((cpu.edx as u8), 0x50u8); }
  if cpu.cf { pc = 0x10b0ce4; continue; }
  pc = 0x10b0cb1; continue;
 }
 0x10b0cb1 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0ee6; continue; }
  pc = 0x10b0cb7; continue;
 }
 0x10b0cb7 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x10b0cd6; continue; }
  pc = 0x10b0cbc; continue;
 }
 0x10b0cbc => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0ea0; continue; }
  pc = 0x10b0cc2; continue;
 }
 0x10b0cc2 => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x10b0fa9; continue; }
  pc = 0x10b0ccb; continue;
 }
 0x10b0ccb => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0f7d; continue; }
  pc = 0x10b0cd1; continue;
 }
 0x10b0cd1 => {
  pc = 0x10b0d86; continue;
 }
 0x10b0cd6 => {
  { cpu.sub8((cpu.edx as u8), 0x53u8); }
  if cpu.zf { pc = 0x10b0dd7; continue; }
  pc = 0x10b0cdf; continue;
 }
 0x10b0cdf => {
  pc = 0x10b0fa9; continue;
 }
 0x10b0ce4 => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x10b0cf5; continue; }
  pc = 0x10b0ce9; continue;
 }
 0x10b0ce9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b0cfa; continue; }
  pc = 0x10b0ceb; continue;
 }
 0x10b0ceb => {
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  pc = 0x10b0cee; continue;
 }
 0x10b0cee => {
  if cpu.zf { pc = 0x10b0d2c; continue; }
  pc = 0x10b0cf0; continue;
 }
 0x10b0cf0 => {
  pc = 0x10b0fa9; continue;
 }
 0x10b0cf5 => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  cpu.poll(); pc = 0x10b0cee; continue;
 }
 0x10b0cfa => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0d2c; continue; }
  pc = 0x10b0d08; continue;
 }
 0x10b0d08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x10b0d1b); f_010b0a9f(cpu); if cpu.returned_to != 0x10b0d1b { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x10b0d7c; continue;
 }
 0x10b0d2c => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b0d37); f_010b0b94(cpu); if cpu.returned_to != 0x10b0d37 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x10b0d7c; continue;
 }
 0x10b0d42 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b0d57; continue; }
  pc = 0x10b0d53; continue;
 }
 0x10b0d53 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10b0d57; continue;
 }
 0x10b0d57 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10b0d60); f_010bf9c9(cpu); if cpu.returned_to != 0x10b0d60 { return; }
  pc = 0x10b0d60; continue;
 }
 0x10b0d60 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10b0d73; continue; }
  pc = 0x10b0d66; continue;
 }
 0x10b0d66 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x10b0d73; continue; }
  pc = 0x10b0d6f; continue;
 }
 0x10b0d6f => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b0d73; continue;
 }
 0x10b0d73 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10b0d7c; continue;
 }
 0x10b0d7c => {
  cpu.push(0x10b0d81); f_010b09fa(cpu); if cpu.returned_to != 0x10b0d81 { return; }
  pc = 0x10b0fd2; continue;
 }
 0x10b0d86 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0da7; continue; }
  pc = 0x10b0d94; continue;
 }
 0x10b0d94 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b0da0; continue;
 }
 0x10b0da0 => {
  cpu.push(0x10b0da5); f_010bf9c9(cpu); if cpu.returned_to != 0x10b0da5 { return; }
  cpu.poll(); pc = 0x10b0d60; continue;
 }
 0x10b0da7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0dc4; continue; }
  pc = 0x10b0db5; continue;
 }
 0x10b0db5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10b0da0; continue;
 }
 0x10b0dc4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x10b0dd5); f_010aee0c(cpu); if cpu.returned_to != 0x10b0dd5 { return; }
  cpu.poll(); pc = 0x10b0d60; continue;
 }
 0x10b0dd7 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0e08; continue; }
  pc = 0x10b0de5; continue;
 }
 0x10b0de5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b0dfc; continue; }
  pc = 0x10b0df7; continue;
 }
 0x10b0df7 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x10b0e00; continue; }
  pc = 0x10b0dfc; continue;
 }
 0x10b0dfc => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  pc = 0x10b0e3e; continue;
 }
 0x10b0e00 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x10b0e3c; continue;
 }
 0x10b0e08 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x40u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0e28; continue; }
  pc = 0x10b0e16; continue;
 }
 0x10b0e16 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x10b0e00; continue; }
  pc = 0x10b0e24; continue;
 }
 0x10b0e24 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  pc = 0x10b0e3c; continue;
 }
 0x10b0e28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b0e24; continue; }
  pc = 0x10b0e36; continue;
 }
 0x10b0e36 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x10b0e3c; continue;
 }
 0x10b0e3c => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  pc = 0x10b0e3e; continue;
 }
 0x10b0e3e => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xf9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x10b0e78; continue; }
  pc = 0x10b0e56; continue;
 }
 0x10b0e56 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b0e6f; continue; }
  pc = 0x10b0e61; continue;
 }
 0x10b0e61 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x10b0fd2; continue;
 }
 0x10b0e6f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x10b0fd2; continue;
 }
 0x10b0e78 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b0e94; continue; }
  pc = 0x10b0e83; continue;
 }
 0x10b0e83 => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x10b0e8f); f_010b0a1e(cpu); if cpu.returned_to != 0x10b0e8f { return; }
  pc = 0x10b0fd2; continue;
 }
 0x10b0e94 => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b0d7c; continue;
 }
 0x10b0ea0 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b0ebe; continue; }
  pc = 0x10b0eac; continue;
 }
 0x10b0eac => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b0ebe; continue; }
  pc = 0x10b0eb0; continue;
 }
 0x10b0eb0 => {
  { let v: u8 = 0x30u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b0ebe; continue;
 }
 0x10b0ebe => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x10b0ec3; continue;
 }
 0x10b0ec3 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10b0ecc); f_010bf97f(cpu); if cpu.returned_to != 0x10b0ecc { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b0d60; continue; }
  pc = 0x10b0eda; continue;
 }
 0x10b0eda => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b0ee1); f_010b1084(cpu); if cpu.returned_to != 0x10b0ee1 { return; }
  cpu.poll(); pc = 0x10b0d60; continue;
 }
 0x10b0ee6 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10b0f0a; continue; }
  pc = 0x10b0eec; continue;
 }
 0x10b0eec => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0f03; continue; }
  pc = 0x10b0efa; continue;
 }
 0x10b0efa => {
  { let v: u32 = 0xdu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b0f0a; continue;
 }
 0x10b0f03 => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b0f0a; continue;
 }
 0x10b0f0a => {
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
  if cpu.zf { pc = 0x10b0f55; continue; }
  pc = 0x10b0f2c; continue;
 }
 0x10b0f2c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x10b0f45); f_010b0a41(cpu); if cpu.returned_to != 0x10b0f45 { return; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x10b0f5e; continue;
 }
 0x10b0f55 => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x10b0f5e; continue;
 }
 0x10b0f5e => {
  cpu.push(0x10b0f63); f_010b0a41(cpu); if cpu.returned_to != 0x10b0f63 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b0d73; continue; }
  pc = 0x10b0f71; continue;
 }
 0x10b0f71 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b0f78); f_010b1084(cpu); if cpu.returned_to != 0x10b0f78 { return; }
  cpu.poll(); pc = 0x10b0d73; continue;
 }
 0x10b0f7d => {
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
  pc = 0x10b0fd2; continue;
 }
 0x10b0fa9 => {
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
  pc = 0x10b0fd2; continue;
 }
 0x10b0fd2 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10b1027; continue; }
  pc = 0x10b0fd8; continue;
 }
 0x10b0fd8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x10b0fea; continue; }
  pc = 0x10b0fe2; continue;
 }
 0x10b0fe2 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x10b1010; continue;
 }
 0x10b0fea => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b0ffe; continue; }
  pc = 0x10b0ff8; continue;
 }
 0x10b0ff8 => {
  { let v: u8 = 0x20u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b1010; continue;
 }
 0x10b0ffe => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b1014; continue; }
  pc = 0x10b100c; continue;
 }
 0x10b100c => {
  { let v: u8 = 0x2bu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b1010; continue;
 }
 0x10b1010 => {
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b1014; continue;
 }
 0x10b1014 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b1020; continue; }
  pc = 0x10b101b; continue;
 }
 0x10b101b => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b1027; continue;
 }
 0x10b1020 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x10b1027; continue;
 }
 0x10b1027 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x10b103e; continue; }
  pc = 0x10b1031; continue;
 }
 0x10b1031 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b103e; continue;
 }
 0x10b103e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b104a; continue; }
  pc = 0x10b1046; continue;
 }
 0x10b1046 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x10b1057; continue; }
  pc = 0x10b104a; continue;
 }
 0x10b104a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if cpu.zf { pc = 0x10b1057; continue; }
  pc = 0x10b1054; continue;
 }
 0x10b1054 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b1057; continue;
 }
 0x10b1057 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x10b1066); f_010b09fa(cpu); if cpu.returned_to != 0x10b1066 { return; }
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
pub fn f_010b1084(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1084); }
 let mut pc: u32 = 0x10b1084;
 loop { match pc {
 0x10b1084 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10b1087; continue;
 }
 0x10b1087 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x10b109a; continue; }
  pc = 0x10b108c; continue;
 }
 0x10b108c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x10b1095); f_010a4864(cpu); if cpu.returned_to != 0x10b1095 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x10b1087; continue;
 }
 0x10b109a => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b10a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b10a0); }
 let mut pc: u32 = 0x10b10a0;
 loop { match pc {
 0x10b10a0 => {
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
pub fn f_010b10b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b10b9); }
 let mut pc: u32 = 0x10b10b9;
 loop { match pc {
 0x10b10b9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x10b10be); cpu.call(t); if cpu.returned_to != 0x10b10be { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b10c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b10c0); }
 let mut pc: u32 = 0x10b10c0;
 loop { match pc {
 0x10b10c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x10b10c6); cpu.call(t); if cpu.returned_to != 0x10b10c6 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b10c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b10c8); }
 let mut pc: u32 = 0x10b10c8;
 loop { match pc {
 0x10b10c8 => {
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
  pc = 0x10b10e7; continue;
 }
 0x10b10e7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  if cpu.zf { pc = 0x10b12ef; continue; }
  pc = 0x10b10fe; continue;
 }
 0x10b10fe => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b1124; continue; }
  pc = 0x10b1116; continue;
 }
 0x10b1116 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b111d); f_010b1404(cpu); if cpu.returned_to != 0x10b111d { return; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x10b12af; continue;
 }
 0x10b1124 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x10b114e; continue; }
  pc = 0x10b1129; continue;
 }
 0x10b1129 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b1130); f_010b10b9(cpu); if cpu.returned_to != 0x10b1130 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x10b12ae; continue; }
  pc = 0x10b1138; continue;
 }
 0x10b1138 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x10b12ef; continue; }
  pc = 0x10b1142; continue;
 }
 0x10b1142 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x10b1149); f_010b10c0(cpu); if cpu.returned_to != 0x10b1149 { return; }
  pc = 0x10b12ef; continue;
 }
 0x10b114e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b1157); f_010b130f(cpu); if cpu.returned_to != 0x10b1157 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b1166; continue; }
  pc = 0x10b1162; continue;
 }
 0x10b1162 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b1166; continue;
 }
 0x10b1166 => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if cpu.cf { pc = 0x10b11c0; continue; }
  pc = 0x10b116b; continue;
 }
 0x10b116b => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10b124c; continue; }
  pc = 0x10b1174; continue;
 }
 0x10b1174 => {
  { cpu.sub32(cpu.ebx, 0x70u32); }
  if cpu.cf { pc = 0x10b11a6; continue; }
  pc = 0x10b1179; continue;
 }
 0x10b1179 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b1240; continue; }
  pc = 0x10b117f; continue;
 }
 0x10b117f => {
  { cpu.sub32(cpu.ebx, 0x75u32); }
  if cpu.cf { pc = 0x10b1198; continue; }
  pc = 0x10b1184; continue;
 }
 0x10b1184 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b1233; continue; }
  pc = 0x10b118a; continue;
 }
 0x10b118a => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x10b1240; continue; }
  pc = 0x10b1193; continue;
 }
 0x10b1193 => {
  pc = 0x10b12af; continue;
 }
 0x10b1198 => {
  { cpu.sub32(cpu.ebx, 0x73u32); }
  if cpu.zf { pc = 0x10b1257; continue; }
  pc = 0x10b11a1; continue;
 }
 0x10b11a1 => {
  pc = 0x10b12af; continue;
 }
 0x10b11a6 => {
  { cpu.sub32(cpu.ebx, 0x6eu32); }
  if cpu.cf { pc = 0x10b11b6; continue; }
  pc = 0x10b11ab; continue;
 }
 0x10b11ab => {
  if (cpu.cf || cpu.zf) { pc = 0x10b1292; continue; }
  pc = 0x10b11b1; continue;
 }
 0x10b11b1 => {
  pc = 0x10b1227; continue;
 }
 0x10b11b6 => {
  { cpu.sub32(cpu.ebx, 0x69u32); }
  if cpu.zf { pc = 0x10b121a; continue; }
  pc = 0x10b11bb; continue;
 }
 0x10b11bb => {
  pc = 0x10b12af; continue;
 }
 0x10b11c0 => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if cpu.cf { pc = 0x10b11e6; continue; }
  pc = 0x10b11c5; continue;
 }
 0x10b11c5 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b1240; continue; }
  pc = 0x10b11cb; continue;
 }
 0x10b11cb => {
  { cpu.sub32(cpu.ebx, 0x63u32); }
  if cpu.cf { pc = 0x10b11d8; continue; }
  pc = 0x10b11d0; continue;
 }
 0x10b11d0 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b126f; continue; }
  pc = 0x10b11d6; continue;
 }
 0x10b11d6 => {
  pc = 0x10b1205; continue;
 }
 0x10b11d8 => {
  { cpu.sub32(cpu.ebx, 0x5bu32); }
  if cpu.zf { pc = 0x10b1262; continue; }
  pc = 0x10b11e1; continue;
 }
 0x10b11e1 => {
  pc = 0x10b12af; continue;
 }
 0x10b11e6 => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.cf { pc = 0x10b11f7; continue; }
  pc = 0x10b11eb; continue;
 }
 0x10b11eb => {
  if (cpu.cf || cpu.zf) { pc = 0x10b124c; continue; }
  pc = 0x10b11ed; continue;
 }
 0x10b11ed => {
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x10b124c; continue; }
  pc = 0x10b11f2; continue;
 }
 0x10b11f2 => {
  pc = 0x10b12af; continue;
 }
 0x10b11f7 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x10b129f; continue; }
  pc = 0x10b1200; continue;
 }
 0x10b1200 => {
  pc = 0x10b12af; continue;
 }
 0x10b1205 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x10b120f; continue;
 }
 0x10b120f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b1213; continue;
 }
 0x10b1213 => {
  cpu.push(0x10b1218); f_010b1aad(cpu); if cpu.returned_to != 0x10b1218 { return; }
  pc = 0x10b1278; continue;
 }
 0x10b121a => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.poll(); pc = 0x10b1213; continue;
 }
 0x10b1227 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x10b120f; continue;
 }
 0x10b1233 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x10b1213; continue;
 }
 0x10b1240 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x10b120f; continue;
 }
 0x10b124c => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b1255); f_010b1796(cpu); if cpu.returned_to != 0x10b1255 { return; }
  pc = 0x10b1278; continue;
 }
 0x10b1257 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b1260); f_010b14d8(cpu); if cpu.returned_to != 0x10b1260 { return; }
  pc = 0x10b1278; continue;
 }
 0x10b1262 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b126d); f_010b169c(cpu); if cpu.returned_to != 0x10b126d { return; }
  pc = 0x10b1278; continue;
 }
 0x10b126f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b1278); f_010b1443(cpu); if cpu.returned_to != 0x10b1278 { return; }
  pc = 0x10b1278; continue;
 }
 0x10b1278 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b12ef; continue; }
  pc = 0x10b1282; continue;
 }
 0x10b1282 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b12af; continue; }
  pc = 0x10b128c; continue;
 }
 0x10b128c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b12af; continue;
 }
 0x10b1292 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b129d); f_010b15fb(cpu); if cpu.returned_to != 0x10b129d { return; }
  pc = 0x10b12af; continue;
 }
 0x10b129f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b12a6); f_010b10b9(cpu); if cpu.returned_to != 0x10b12a6 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b1138; continue; }
  pc = 0x10b12ae; continue;
 }
 0x10b12ae => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10b12af; continue;
 }
 0x10b12af => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x10b10e7; continue; }
  pc = 0x10b12b9; continue;
 }
 0x10b12b9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x25u32); }
  if !cpu.zf { pc = 0x10b12ef; continue; }
  pc = 0x10b12c8; continue;
 }
 0x10b12c8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x10b12d5); f_010b130f(cpu); if cpu.returned_to != 0x10b12d5 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x10b12ef; continue; }
  pc = 0x10b12e4; continue;
 }
 0x10b12e4 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b12ef); f_010b15fb(cpu); if cpu.returned_to != 0x10b12ef { return; }
  pc = 0x10b12ef; continue;
 }
 0x10b12ef => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b1303; continue; }
  pc = 0x10b12f6; continue;
 }
 0x10b12f6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x10b1303; continue; }
  pc = 0x10b12fc; continue;
 }
 0x10b12fc => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b1307; continue;
 }
 0x10b1303 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x10b1307; continue;
 }
 0x10b1307 => {
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
pub fn f_010b130f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b130f); }
 let mut pc: u32 = 0x10b130f;
 loop { match pc {
 0x10b130f => {
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
  if !cpu.zf { pc = 0x10b1342; continue; }
  pc = 0x10b1338; continue;
 }
 0x10b1338 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b1342; continue;
 }
 0x10b1342 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b1394; continue; }
  pc = 0x10b1365; continue;
 }
 0x10b1365 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10b1367; continue;
 }
 0x10b1367 => {
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
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x10b1367; continue; }
  pc = 0x10b1391; continue;
 }
 0x10b1391 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b1394; continue;
 }
 0x10b1394 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x4eu32); }
  if !cpu.zf { pc = 0x10b13a8; continue; }
  pc = 0x10b13a0; continue;
 }
 0x10b13a0 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b13b8; continue;
 }
 0x10b13a8 => {
  { cpu.sub32(cpu.edx, 0x46u32); }
  if !cpu.zf { pc = 0x10b13b8; continue; }
  pc = 0x10b13ad; continue;
 }
 0x10b13ad => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b13b8; continue;
 }
 0x10b13b8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.cf { pc = 0x10b13d2; continue; }
  pc = 0x10b13c2; continue;
 }
 0x10b13c2 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b13d9; continue; }
  pc = 0x10b13c4; continue;
 }
 0x10b13c4 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x10b13fd; continue; }
  pc = 0x10b13c9; continue;
 }
 0x10b13c9 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b13e6; continue; }
  pc = 0x10b13cb; continue;
 }
 0x10b13cb => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x10b13e6; continue; }
  pc = 0x10b13d0; continue;
 }
 0x10b13d0 => {
  pc = 0x10b13fd; continue;
 }
 0x10b13d2 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x10b13f2; continue; }
  pc = 0x10b13d7; continue;
 }
 0x10b13d7 => {
  pc = 0x10b13fd; continue;
 }
 0x10b13d9 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x10u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b13fd; continue;
 }
 0x10b13e6 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b13fd; continue;
 }
 0x10b13f2 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b13fd; continue;
 }
 0x10b13fd => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1404(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1404); }
 let mut pc: u32 = 0x10b1404;
 loop { match pc {
 0x10b1404 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b140b; continue;
 }
 0x10b140b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10b1412); f_010b10b9(cpu); if cpu.returned_to != 0x10b1412 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b1430; continue; }
  pc = 0x10b142d; continue;
 }
 0x10b142d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10b140b; continue;
 }
 0x10b1430 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x10b143d; continue; }
  pc = 0x10b1436; continue;
 }
 0x10b1436 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10b143d); f_010b10c0(cpu); if cpu.returned_to != 0x10b143d { return; }
  pc = 0x10b143d; continue;
 }
 0x10b143d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1443(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b1443); } r_010b1443(cpu, 0x10b1443); }
pub fn f_010b14d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b14d8); } r_010b1443(cpu, 0x10b14d8); }
pub fn f_010b1796(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b1796); } r_010b1443(cpu, 0x10b1796); }
fn r_010b1443(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b1443 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x10b148a; continue; }
  pc = 0x10b1453; continue;
 }
 0x10b1453 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b1467; continue; }
  pc = 0x10b145b; continue;
 }
 0x10b145b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x10b148a; continue;
 }
 0x10b1467 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x10b147c; continue; }
  pc = 0x10b146c; continue;
 }
 0x10b146c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x10b148a; continue;
 }
 0x10b147c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x10b148a; continue;
 }
 0x10b148a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b1499; continue; }
  pc = 0x10b1494; continue;
 }
 0x10b1494 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10b1499; continue;
 }
 0x10b1499 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b14cd; continue; }
  pc = 0x10b149d; continue;
 }
 0x10b149d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10b14a4); f_010b10b9(cpu); if cpu.returned_to != 0x10b14a4 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x10b14cd; continue; }
  pc = 0x10b14ae; continue;
 }
 0x10b14ae => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10b1499; continue; }
  pc = 0x10b14b4; continue;
 }
 0x10b14b4 => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x10b14c4; continue; }
  pc = 0x10b14b8; continue;
 }
 0x10b14b8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x10b1499; continue;
 }
 0x10b14c4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x10b1499; continue;
 }
 0x10b14cd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b14d2; continue;
 }
 0x10b14d2 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b14d8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x10b14ef; continue; }
  pc = 0x10b14e8; continue;
 }
 0x10b14e8 => {
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b14f8; continue;
 }
 0x10b14ef => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x10u8); }
  { let v: u8 = 0x1u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b14f8; continue;
 }
 0x10b14f8 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b1531; continue; }
  pc = 0x10b14ff; continue;
 }
 0x10b14ff => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b150f; continue; }
  pc = 0x10b1503; continue;
 }
 0x10b1503 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x10b1531; continue;
 }
 0x10b150f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x10b1523; continue; }
  pc = 0x10b1513; continue;
 }
 0x10b1513 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x10b1531; continue;
 }
 0x10b1523 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x10b1531; continue;
 }
 0x10b1531 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b1533; continue;
 }
 0x10b1533 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10b153a); f_010b10b9(cpu); if cpu.returned_to != 0x10b153a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b1559; continue; }
  pc = 0x10b1556; continue;
 }
 0x10b1556 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10b1533; continue;
 }
 0x10b1559 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if cpu.zf { pc = 0x10b1566; continue; }
  pc = 0x10b155f; continue;
 }
 0x10b155f => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b15d0; continue;
 }
 0x10b1566 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b15c6; continue; }
  pc = 0x10b1573; continue;
 }
 0x10b1573 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b159e; continue; }
  pc = 0x10b157c; continue;
 }
 0x10b157c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10b158f; continue; }
  pc = 0x10b1587; continue;
 }
 0x10b1587 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1596; continue;
 }
 0x10b158f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1596; continue;
 }
 0x10b1596 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x10b159e; continue;
 }
 0x10b159e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10b15a5); f_010b1cc4(cpu); if cpu.returned_to != 0x10b15a5 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b15d0; continue; }
  pc = 0x10b15ad; continue;
 }
 0x10b15ad => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x10b1573; continue; }
  pc = 0x10b15c6; continue;
 }
 0x10b15c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10b15d0); f_010b10c0(cpu); if cpu.returned_to != 0x10b15d0 { return; }
  pc = 0x10b15d0; continue;
 }
 0x10b15d0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x10b15f1; continue; }
  pc = 0x10b15d6; continue;
 }
 0x10b15d6 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b15f1; continue; }
  pc = 0x10b15da; continue;
 }
 0x10b15da => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10b15eb; continue; }
  pc = 0x10b15e5; continue;
 }
 0x10b15e5 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b15f1; continue;
 }
 0x10b15eb => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b15f1; continue;
 }
 0x10b15f1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x10b14d2; continue;
 }
 0x10b1796 => {
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
  pc = 0x10b17ad; continue;
 }
 0x10b17ad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b17b4); f_010b10b9(cpu); if cpu.returned_to != 0x10b17b4 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x10b17d4; continue; }
  pc = 0x10b17ce; continue;
 }
 0x10b17ce => {
  { let v: u32 = cpu.inc32(cpu.r32(0x60u32.wrapping_add(cpu.esp))); cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b17ad; continue;
 }
 0x10b17d4 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b17de; continue;
 }
 0x10b17de => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b19de; continue; }
  pc = 0x10b17ef; continue;
 }
 0x10b17ef => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x10b17f9; continue; }
  pc = 0x10b17f4; continue;
 }
 0x10b17f4 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x10b1817; continue; }
  pc = 0x10b17f9; continue;
 }
 0x10b17f9 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b1806); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1806 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b1817; continue;
 }
 0x10b1817 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10b1838; continue; }
  pc = 0x10b182f; continue;
 }
 0x10b182f => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x10b19de; continue; }
  pc = 0x10b1838; continue;
 }
 0x10b1838 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b18a0; continue; }
  pc = 0x10b1858; continue;
 }
 0x10b1858 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x10b185d; continue;
 }
 0x10b185d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x10b1877; continue; }
  pc = 0x10b1868; continue;
 }
 0x10b1868 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6au32.wrapping_add(cpu.esp)), 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6au32.wrapping_add(cpu.esp), v); }
  pc = 0x10b1877; continue;
 }
 0x10b1877 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x10b187f); f_010b1cc4(cpu); if cpu.returned_to != 0x10b187f { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b188a; continue;
 }
 0x10b188a => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10b185d; continue; }
  pc = 0x10b18a0; continue;
 }
 0x10b18a0 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x10b1950; continue; }
  pc = 0x10b18a9; continue;
 }
 0x10b18a9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b18b2); f_010b1cc4(cpu); if cpu.returned_to != 0x10b18b2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b18be; continue;
 }
 0x10b18be => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x10b18dc; continue; }
  pc = 0x10b18c2; continue;
 }
 0x10b18c2 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b19de; continue; }
  pc = 0x10b18dc; continue;
 }
 0x10b18dc => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10b18dd; continue;
 }
 0x10b18dd => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b1907; continue; }
  pc = 0x10b18f5; continue;
 }
 0x10b18f5 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b18ff); f_010b1cc4(cpu); if cpu.returned_to != 0x10b18ff { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b18dd; continue; }
  pc = 0x10b1907; continue;
 }
 0x10b1907 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x10b1947; continue; }
  pc = 0x10b190d; continue;
 }
 0x10b190d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b1915; continue;
 }
 0x10b1915 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x10b193e; continue; }
  pc = 0x10b1920; continue;
 }
 0x10b1920 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b1915; continue;
 }
 0x10b193e => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b1947; continue;
 }
 0x10b1947 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b1950; continue;
 }
 0x10b1950 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x10b19de; continue; }
  pc = 0x10b195a; continue;
 }
 0x10b195a => {
  { cpu.sub32(cpu.ecx, 0x65u32); }
  if cpu.zf { pc = 0x10b1968; continue; }
  pc = 0x10b195f; continue;
 }
 0x10b195f => {
  { cpu.sub32(cpu.ecx, 0x45u32); }
  if !cpu.zf { pc = 0x10b19de; continue; }
  pc = 0x10b1968; continue;
 }
 0x10b1968 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b1972); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1972 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b197e; continue;
 }
 0x10b197e => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x10b1988; continue; }
  pc = 0x10b1983; continue;
 }
 0x10b1983 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x10b199a; continue; }
  pc = 0x10b1988; continue;
 }
 0x10b1988 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b1992); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1992 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b199a; continue;
 }
 0x10b199a => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x10b19b6; continue; }
  pc = 0x10b19b2; continue;
 }
 0x10b19b2 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b19de; continue;
 }
 0x10b19b6 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b19c0); f_010b1cc4(cpu); if cpu.returned_to != 0x10b19c0 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b19e7; continue; }
  pc = 0x10b19c8; continue;
 }
 0x10b19c8 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10b19b6; continue; }
  pc = 0x10b19de; continue;
 }
 0x10b19de => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b19e7); f_010b10c0(cpu); if cpu.returned_to != 0x10b19e7 { return; }
  pc = 0x10b19e7; continue;
 }
 0x10b19e7 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b1aa2; continue; }
  pc = 0x10b19ef; continue;
 }
 0x10b19ef => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b1aa2; continue; }
  pc = 0x10b1a01; continue;
 }
 0x10b1a01 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x10b1a1a; continue; }
  pc = 0x10b1a0a; continue;
 }
 0x10b1a0a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x10b1a26; continue; }
  pc = 0x10b1a14; continue;
 }
 0x10b1a14 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x68u32.wrapping_add(cpu.esp))); cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b1a26; continue;
 }
 0x10b1a1a => {
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  let t = cpu.r32(0x117c960u32); cpu.push(0x10b1a26); cpu.call(t); if cpu.returned_to != 0x10b1a26 { return; }
  pc = 0x10b1a26; continue;
 }
 0x10b1a26 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b1a3e; continue; }
  pc = 0x10b1a2e; continue;
 }
 0x10b1a2e => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebx = v; cpu.set_segment(0, selector); }
  pc = 0x10b1a69; continue;
 }
 0x10b1a3e => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x10b1a57; continue; }
  pc = 0x10b1a43; continue;
 }
 0x10b1a43 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  pc = 0x10b1a69; continue;
 }
 0x10b1a57 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  pc = 0x10b1a69; continue;
 }
 0x10b1a69 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x10b1a77; continue; }
  pc = 0x10b1a71; continue;
 }
 0x10b1a71 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x10b1a9f; continue;
 }
 0x10b1a77 => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x10b1a81; continue; }
  pc = 0x10b1a7c; continue;
 }
 0x10b1a7c => {
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x10b1a92; continue; }
  pc = 0x10b1a81; continue;
 }
 0x10b1a81 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1aa2; continue;
 }
 0x10b1a92 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x10b1a9f); f_010bf9e4(cpu); if cpu.returned_to != 0x10b1a9f { return; }
  pc = 0x10b1a9f; continue;
 }
 0x10b1a9f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1aa2; continue;
 }
 0x10b1aa2 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x10b14d2; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b15fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b15fb); }
 let mut pc: u32 = 0x10b15fb;
 loop { match pc {
 0x10b15fb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b1654; continue; }
  pc = 0x10b1607; continue;
 }
 0x10b1607 => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b1618; continue; }
  pc = 0x10b160c; continue;
 }
 0x10b160c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x10b163d; continue;
 }
 0x10b1618 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x10b162d; continue; }
  pc = 0x10b161d; continue;
 }
 0x10b161d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x10b163d; continue;
 }
 0x10b162d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x10b163d; continue;
 }
 0x10b163d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x10b164e; continue; }
  pc = 0x10b1645; continue;
 }
 0x10b1645 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b164e => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1654; continue;
 }
 0x10b1654 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1659(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1659); }
 let mut pc: u32 = 0x10b1659;
 loop { match pc {
 0x10b1659 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10b166e); f_010afeb0(cpu); if cpu.returned_to != 0x10b166e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b1696; continue; }
  pc = 0x10b1677; continue;
 }
 0x10b1677 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b7a8u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b1696; continue; }
  pc = 0x10b1690; continue;
 }
 0x10b1690 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x5du32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b1677; continue; }
  pc = 0x10b1696; continue;
 }
 0x10b1696 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b169c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b169c); }
 let mut pc: u32 = 0x10b169c;
 loop { match pc {
 0x10b169c => {
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
  if cpu.zf { pc = 0x10b16c3; continue; }
  pc = 0x10b16c0; continue;
 }
 0x10b16c0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b16c3; continue;
 }
 0x10b16c3 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x10b16cc); f_010b1659(cpu); if cpu.returned_to != 0x10b16cc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b170a; continue; }
  pc = 0x10b16d6; continue;
 }
 0x10b16d6 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b16e7; continue; }
  pc = 0x10b16db; continue;
 }
 0x10b16db => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.esi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(0, selector); }
  pc = 0x10b170a; continue;
 }
 0x10b16e7 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x10b16fc; continue; }
  pc = 0x10b16ec; continue;
 }
 0x10b16ec => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x10b170a; continue;
 }
 0x10b16fc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.esi = v; }
  pc = 0x10b170a; continue;
 }
 0x10b170a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b170f; continue;
 }
 0x10b170f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x10b177d; continue; }
  pc = 0x10b1713; continue;
 }
 0x10b1713 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b171a); f_010b10b9(cpu); if cpu.returned_to != 0x10b171a { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x10b177d; continue; }
  pc = 0x10b1726; continue;
 }
 0x10b1726 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x116b7a8u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebp); }
  { let v: u8 = (cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.zf { pc = 0x10b1769; continue; }
  pc = 0x10b1760; continue;
 }
 0x10b1760 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x10b1767); f_010b10c0(cpu); if cpu.returned_to != 0x10b1767 { return; }
  pc = 0x10b177d; continue;
 }
 0x10b1769 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x10b170f; continue; }
  pc = 0x10b1773; continue;
 }
 0x10b1773 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10b170f; continue;
 }
 0x10b177d => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x10b178b; continue; }
  pc = 0x10b1783; continue;
 }
 0x10b1783 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b178b; continue; }
  pc = 0x10b1787; continue;
 }
 0x10b1787 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b178b; continue;
 }
 0x10b178b => {
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
pub fn f_010b1aad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1aad); }
 let mut pc: u32 = 0x10b1aad;
 loop { match pc {
 0x10b1aad => {
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
  pc = 0x10b1ac7; continue;
 }
 0x10b1ac7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b1ace); f_010b10b9(cpu); if cpu.returned_to != 0x10b1ace { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x116b67cu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b1ae9; continue; }
  pc = 0x10b1ae4; continue;
 }
 0x10b1ae4 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b1ac7; continue;
 }
 0x10b1ae9 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1af3; continue;
 }
 0x10b1af3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b1c1c; continue; }
  pc = 0x10b1b04; continue;
 }
 0x10b1b04 => {
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b1b38; continue; }
  pc = 0x10b1b15; continue;
 }
 0x10b1b15 => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x10b1b1f; continue; }
  pc = 0x10b1b1a; continue;
 }
 0x10b1b1a => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if !cpu.zf { pc = 0x10b1b38; continue; }
  pc = 0x10b1b1f; continue;
 }
 0x10b1b1f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10b1b2d); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1b2d { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1b38; continue;
 }
 0x10b1b38 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b1b94; continue; }
  pc = 0x10b1b3c; continue;
 }
 0x10b1b3c => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x10b1b8d; continue; }
  pc = 0x10b1b41; continue;
 }
 0x10b1b41 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x10b1b4d); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1b4d { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1b58; continue;
 }
 0x10b1b58 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x10b1b62; continue; }
  pc = 0x10b1b5d; continue;
 }
 0x10b1b5d => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x10b1b86; continue; }
  pc = 0x10b1b62; continue;
 }
 0x10b1b62 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  cpu.push(0x10b1b6c); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1b6c { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1b7f; continue;
 }
 0x10b1b7f => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x10b1bd8; continue;
 }
 0x10b1b86 => {
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x10b1bd8; continue;
 }
 0x10b1b8d => {
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x10b1bd8; continue;
 }
 0x10b1b94 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x10b1bd8; continue; }
  pc = 0x10b1b99; continue;
 }
 0x10b1b99 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x10b1bd8; continue; }
  pc = 0x10b1b9e; continue;
 }
 0x10b1b9e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x10b1baa); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1baa { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1bb5; continue;
 }
 0x10b1bb5 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x10b1bbf; continue; }
  pc = 0x10b1bba; continue;
 }
 0x10b1bba => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x10b1bd8; continue; }
  pc = 0x10b1bbf; continue;
 }
 0x10b1bbf => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x10b1bc9); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1bc9 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b1bd1; continue;
 }
 0x10b1bd1 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1bd8; continue;
 }
 0x10b1bd8 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10b1bdf); f_010b1c9d(cpu); if cpu.returned_to != 0x10b1bdf { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b1bf2; continue; }
  pc = 0x10b1be3; continue;
 }
 0x10b1be3 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x10b1bf0); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1bf0 { return; }
  cpu.poll(); pc = 0x10b1bd1; continue;
 }
 0x10b1bf2 => {
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if !cpu.zf { pc = 0x10b1c1c; continue; }
  pc = 0x10b1bf7; continue;
 }
 0x10b1bf7 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x10b1c1c; continue; }
  pc = 0x10b1bfd; continue;
 }
 0x10b1bfd => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x10b1c05); f_010b1cc4(cpu); if cpu.returned_to != 0x10b1c05 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b1c25; continue; }
  pc = 0x10b1c0c; continue;
 }
 0x10b1c0c => {
  cpu.push(0x10b1c11); f_010b1c9d(cpu); if cpu.returned_to != 0x10b1c11 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b1c1c; continue; }
  pc = 0x10b1c15; continue;
 }
 0x10b1c15 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  cpu.poll(); pc = 0x10b1bfd; continue;
 }
 0x10b1c1c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10b1c25); f_010b10c0(cpu); if cpu.returned_to != 0x10b1c25 { return; }
  pc = 0x10b1c25; continue;
 }
 0x10b1c25 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x2du32); }
  if !cpu.zf { pc = 0x10b1c2e; continue; }
  pc = 0x10b1c2c; continue;
 }
 0x10b1c2c => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x10b1c2e; continue;
 }
 0x10b1c2e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b1c93; continue; }
  pc = 0x10b1c32; continue;
 }
 0x10b1c32 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b1c93; continue; }
  pc = 0x10b1c3f; continue;
 }
 0x10b1c3f => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b1c54; continue; }
  pc = 0x10b1c44; continue;
 }
 0x10b1c44 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  pc = 0x10b1c7f; continue;
 }
 0x10b1c54 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x10b1c6d; continue; }
  pc = 0x10b1c59; continue;
 }
 0x10b1c59 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x10b1c7f; continue;
 }
 0x10b1c6d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x10b1c7f; continue;
 }
 0x10b1c7f => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x10b1c8d; continue; }
  pc = 0x10b1c87; continue;
 }
 0x10b1c87 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1c93; continue;
 }
 0x10b1c8d => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x10b1c93; continue;
 }
 0x10b1c93 => {
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
pub fn f_010b1c9d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1c9d); }
 let mut pc: u32 = 0x10b1c9d;
 loop { match pc {
 0x10b1c9d => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b1cab; continue; }
  pc = 0x10b1ca2; continue;
 }
 0x10b1ca2 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b1cab; continue; }
  pc = 0x10b1ca7; continue;
 }
 0x10b1ca7 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b1cab => {
  cpu.push(0x10b1cb0); f_010a6a38(cpu); if cpu.returned_to != 0x10b1cb0 { return; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b1cbe; continue; }
  pc = 0x10b1cb5; continue;
 }
 0x10b1cb5 => {
  { cpu.sub32(cpu.eax, 0x66u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b1cbe; continue; }
  pc = 0x10b1cba; continue;
 }
 0x10b1cba => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x57u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b1cbe => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1cc4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1cc4); }
 let mut pc: u32 = 0x10b1cc4;
 loop { match pc {
 0x10b1cc4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b1ce2; continue; }
  pc = 0x10b1cd5; continue;
 }
 0x10b1cd5 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10b1cdc); f_010b10b9(cpu); if cpu.returned_to != 0x10b1cdc { return; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x2u8); }
  if cpu.zf { pc = 0x10b1ce7; continue; }
  pc = 0x10b1ce2; continue;
 }
 0x10b1ce2 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b1ce7; continue;
 }
 0x10b1ce7 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1cea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1cea); }
 let mut pc: u32 = 0x10b1cea;
 loop { match pc {
 0x10b1cea => {
  { let v: u32 = cpu.r32(0x117b9a4u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1da4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1da4); }
 let mut pc: u32 = 0x10b1da4;
 loop { match pc {
 0x10b1da4 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b1db1); f_010bfa30(cpu); if cpu.returned_to != 0x10b1db1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x10b1dc6); f_010bfe6a(cpu); if cpu.returned_to != 0x10b1dc6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b1e3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1e3e); }
 let mut pc: u32 = 0x10b1e3e;
 loop { match pc {
 0x10b1e3e => {
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
  cpu.push(0x10b1e70); f_010b1da4(cpu); if cpu.returned_to != 0x10b1e70 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10b1e80; continue; }
  pc = 0x10b1e7a; continue;
 }
 0x10b1e7a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b1e80 => {
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
pub fn f_010b1eca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1eca); }
 let mut pc: u32 = 0x10b1eca;
 loop { match pc {
 0x10b1eca => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x6u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10b1ee6); f_010b1da4(cpu); if cpu.returned_to != 0x10b1ee6 { return; }
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
pub fn f_010b1efa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b1efa); }
 let mut pc: u32 = 0x10b1efa;
 loop { match pc {
 0x10b1eb0 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10b1eb9); f_010b1da4(cpu); if cpu.returned_to != 0x10b1eb9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b1efa => {
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
  cpu.poll(); pc = 0x10b1eb0; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b20fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b20fb); }
 let mut pc: u32 = 0x10b20fb;
 loop { match pc {
 0x10b20fb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b210d); f_010bfa30(cpu); if cpu.returned_to != 0x10b210d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  cpu.push(0x10b2119); f_010b1eca(cpu); if cpu.returned_to != 0x10b2119 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x10b212d; continue; }
  pc = 0x10b2123; continue;
 }
 0x10b2123 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b212d; continue; }
  pc = 0x10b2127; continue;
 }
 0x10b2127 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b212d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b2137); f_010b1e3e(cpu); if cpu.returned_to != 0x10b2137 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b2147; continue; }
  pc = 0x10b213d; continue;
 }
 0x10b213d => {
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x10b215d; continue; }
  pc = 0x10b2145; continue;
 }
 0x10b2145 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x10b2147; continue;
 }
 0x10b2147 => {
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  cpu.push(0x10b2155); f_010b1efa(cpu); if cpu.returned_to != 0x10b2155 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b215d; continue; }
  pc = 0x10b2159; continue;
 }
 0x10b2159 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x10b215d; continue;
 }
 0x10b215d => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b243e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b243e); }
 let mut pc: u32 = 0x10b243e;
 loop { match pc {
 0x10b243e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x10bffd9u32; cpu.edx = v; }
  { let v: u32 = 0x10c0122u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117c95cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c960u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b248a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b248a); }
 let mut pc: u32 = 0x10b248a;
 loop { match pc {
 0x10b248a => {
  { let v: u8 = 0x3u8; cpu.w8(0x11742ecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x117ba14u32.wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x10b2498, "Smsw");
  { let v: u16 = cpu.r16(0x117ba12u32.wrapping_add(cpu.bases[3])) & 0x6u16; cpu.w16(0x117ba12u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x10b24ab, "Sidt");
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x117ba4cu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x10b24dc; continue; }
  pc = 0x10b24c2; continue;
 }
 0x10b24c2 => {
  cpu.flat_ds(0x10b24c2); cpu.push(0x10b24c7); f_010b2555(cpu); if cpu.returned_to != 0x10b24c7 { return; }
  { let v = cpu.r16(0x117b1e2u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  cpu.flat_ds(0x10b24d5); cpu.push(0x10b24da); f_010b25bf(cpu); if cpu.returned_to != 0x10b24da { return; }
  pc = 0x10b2554; continue;
 }
 0x10b24dc => {
  { cpu.sub8(cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x10b2500; continue; }
  pc = 0x10b24e5; continue;
 }
 0x10b24e5 => {
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x10c0518u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x10b2554; continue;
 }
 0x10b2500 => {
  { cpu.sub8(cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x10b2528; continue; }
  pc = 0x10b2509; continue;
 }
 0x10b2509 => {
  cpu.flat_ds(0x10b2509); cpu.push(0x10b250e); f_010b2555(cpu); if cpu.returned_to != 0x10b250e { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117b1dcu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10b2523); cpu.push(0x10b2526); cpu.call(t); if cpu.returned_to != 0x10b2526 { return; }
  pc = 0x10b2554; continue;
 }
 0x10b2528 => {
  { cpu.sub8(cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x10b2541; continue; }
  pc = 0x10b2531; continue;
 }
 0x10b2531 => {
  { let v: u16 = cpu.r16(0x117b1e0u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x10b253a); cpu.push(0x10b253f); f_010c3356(cpu); if cpu.returned_to != 0x10b253f { return; }
  pc = 0x10b2554; continue;
 }
 0x10b2541 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x10b2546); cpu.push(0x10b254b); f_010c3356(cpu); if cpu.returned_to != 0x10b254b { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b2554; continue; }
  pc = 0x10b254f; continue;
 }
 0x10b254f => {
  cpu.flat_ds(0x10b254f); cpu.push(0x10b2554); f_010b2572(cpu); if cpu.returned_to != 0x10b2554 { return; }
  pc = 0x10b2554; continue;
 }
 0x10b2554 => {
  cpu.flat_ds(0x10b2554); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2555(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2555); }
 let mut pc: u32 = 0x10b2555;
 loop { match pc {
 0x10b2555 => {
  { let v: u32 = 0x10c0518u32; cpu.ecx = v; }
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
pub fn f_010b2572(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2572); }
 let mut pc: u32 = 0x10b2572;
 loop { match pc {
 0x10b2572 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x117ba0cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x117ba10u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])), 0x3u8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.sf != cpu.of) { pc = 0x10b25a9; continue; }
  pc = 0x10b2599; continue;
 }
 0x10b2599 => {
  { let v: u32 = 0x10c04a8u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10b25a2); cpu.push(0x10b25a7); f_010b25bf(cpu); if cpu.returned_to != 0x10b25a7 { return; }
  pc = 0x10b25be; continue;
 }
 0x10b25a9 => {
  { let v: u32 = 0x10c0518u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffdu32; cpu.ecx = v; }
  {  }
  pc = 0x10b25be; continue;
 }
 0x10b25be => {
  cpu.flat_ds(0x10b25be); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b25bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b25bf); }
 let mut pc: u32 = 0x10b25bf;
 loop { match pc {
 0x10b25bf => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x117b9ccu32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0x117b9ccu32) | 0x4u32; cpu.w32(0x117b9ccu32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x117b9ccu32), 0xfffffffdu32); cpu.w32(0x117b9ccu32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2659(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2659); }
 let mut pc: u32 = 0x10b2659;
 loop { match pc {
 0x10b2659 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x117ba0cu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x117ba10u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.sub8(cpu.r8(0x117ba4au32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x10b2680; continue; }
  pc = 0x10b2679; continue;
 }
 0x10b2679 => {
  cpu.flat_ds(0x10b2679); cpu.push(0x10b267e); f_010b2694(cpu); if cpu.returned_to != 0x10b267e { return; }
  pc = 0x10b2693; continue;
 }
 0x10b2680 => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff9u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x117ba12u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  {  }
  pc = 0x10b2693; continue;
 }
 0x10b2693 => {
  cpu.flat_ds(0x10b2693); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2694(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2694); }
 let mut pc: u32 = 0x10b2694;
 loop { match pc {
 0x10b2694 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x117b9ccu32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x117ba12u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117b9ccu32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b26b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b26b5); }
 let mut pc: u32 = 0x10b26b5;
 loop { match pc {
 0x10b26b5 => {
  {  }
  cpu.unimplemented(0x10b26b6, "Fnsave");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b26ba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b26ba); }
 let mut pc: u32 = 0x10b26ba;
 loop { match pc {
 0x10b26ba => {
  cpu.unimplemented(0x10b26ba, "Frstor");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b26be(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b26be); }
 let mut pc: u32 = 0x10b26be;
 loop { match pc {
 0x10b26be => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x11742edu32), 0x0u8); }
  if cpu.zf { pc = 0x10b26df; continue; }
  pc = 0x10b26c9; continue;
 }
 0x10b26c9 => {
  { let v: u32 = 0x10b26b5u32; cpu.edx = v; }
  { let v: u32 = 0x10b26bau32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117c9ecu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c9f0u32, v); }
  pc = 0x10b26df; continue;
 }
 0x10b26df => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x117c9e8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b26ec); f_010c3484(cpu); if cpu.returned_to != 0x10b26ec { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b26ef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b26ef); }
 let mut pc: u32 = 0x10b26ef;
 loop { match pc {
 0x10b26ef => {
  { cpu.sub8(cpu.r8(0x11742edu32), 0x0u8); }
  if !cpu.zf { f_010b26be(cpu); return; }
  pc = 0x10b26f8; continue;
 }
 0x10b26f8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b26f9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b26f9); }
 let mut pc: u32 = 0x10b26f9;
 loop { match pc {
 0x10b26f9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x11742ecu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b2741; continue; }
  pc = 0x10b2704; continue;
 }
 0x10b2704 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x11742edu32, v); }
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
  if !cpu.zf { pc = 0x10b272c; continue; }
  pc = 0x10b2723; continue;
 }
 0x10b2723 => {
  cpu.push(0x10b2728); f_010b26be(cpu); if cpu.returned_to != 0x10b2728 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b272c; continue;
 }
 0x10b272c => {
  { cpu.sub8(cpu.r8(0x117ba48u32), 0x0u8); }
  if !cpu.zf { pc = 0x10b2741; continue; }
  pc = 0x10b2735; continue;
 }
 0x10b2735 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x11742ecu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x11742edu32, v); }
  pc = 0x10b2741; continue;
 }
 0x10b2741 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2743(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2743); }
 let mut pc: u32 = 0x10b2743;
 loop { match pc {
 0x10b2743 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2744(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2744); }
 let mut pc: u32 = 0x10b2744;
 loop { match pc {
 0x10b2744 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10b2757); f_010afeb0(cpu); if cpu.returned_to != 0x10b2757 { return; }
  pc = 0x10b2778; continue;
 }
 0x10b2759 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x116b7b0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x10b2778; continue;
 }
 0x10b2778 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2759; continue; }
  pc = 0x10b277e; continue;
 }
 0x10b277e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2782(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2782); }
 let mut pc: u32 = 0x10b2782;
 loop { match pc {
 0x10b2782 => {
  if !cpu.cf { pc = 0x10b2792; continue; }
  pc = 0x10b2784; continue;
 }
 0x10b2784 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b278f); f_010c34b6(cpu); if cpu.returned_to != 0x10b278f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x10b2794; continue;
 }
 0x10b2792 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10b2794; continue;
 }
 0x10b2794 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2795(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2795); }
 let mut pc: u32 = 0x10b2795;
 loop { match pc {
 0x10b2795 => {
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
  cpu.push(0x10b27b3); f_010b27b9(cpu); if cpu.returned_to != 0x10b27b3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b27b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b27b9); }
 let mut pc: u32 = 0x10b27b9;
 loop { match pc {
 0x10b27b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x10b27c5; continue;
 }
 0x10b27c5 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x10b27d0; continue; }
  pc = 0x10b27cd; continue;
 }
 0x10b27cd => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x10b27c5; continue;
 }
 0x10b27d0 => {
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
  if (cpu.sf != cpu.of) { pc = 0x10b2803; continue; }
  pc = 0x10b27fd; continue;
 }
 0x10b27fd => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x10b2803; continue;
 }
 0x10b2803 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x3u8); }
  if cpu.zf { pc = 0x10b2886; continue; }
  pc = 0x10b2810; continue;
 }
 0x10b2810 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b2886; continue; }
  pc = 0x10b2819; continue;
 }
 0x10b2819 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b2820); f_010c3527(cpu); if cpu.returned_to != 0x10b2820 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b2886; continue; }
  pc = 0x10b2824; continue;
 }
 0x10b2824 => {
  { cpu.and8(cpu.r8(0x29u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x10b284e; continue; }
  pc = 0x10b282b; continue;
 }
 0x10b282b => {
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x10b284e; continue; }
  pc = 0x10b2830; continue;
 }
 0x10b2830 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x10b2844); f_010b0331(cpu); if cpu.returned_to != 0x10b2844 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b2999; continue;
 }
 0x10b284e => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x40u8); }
  if cpu.zf { pc = 0x10b2886; continue; }
  pc = 0x10b2855; continue;
 }
 0x10b2855 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b2886; continue; }
  pc = 0x10b286c; continue;
 }
 0x10b286c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x10b2876; continue;
 }
 0x10b2876 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b2881); f_010c34b6(cpu); if cpu.returned_to != 0x10b2881 { return; }
  pc = 0x10b2999; continue;
 }
 0x10b2886 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b2928; continue; }
  pc = 0x10b288f; continue;
 }
 0x10b288f => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x10b2876; continue; }
  pc = 0x10b2896; continue;
 }
 0x10b2896 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2876; continue; }
  pc = 0x10b28a1; continue;
 }
 0x10b28a1 => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b28b4; continue; }
  pc = 0x10b28af; continue;
 }
 0x10b28af => {
  { let v: u32 = 0x180u32; cpu.eax = v; }
  pc = 0x10b28b4; continue;
 }
 0x10b28b4 => {
  { let v: u32 = cpu.r32(0x117ca08u32); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x10b28c9; continue; }
  pc = 0x10b28c4; continue;
 }
 0x10b28c4 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x10b28c9; continue;
 }
 0x10b28c9 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2876; continue; }
  pc = 0x10b28e0; continue;
 }
 0x10b28e0 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b2928; continue; }
  pc = 0x10b28ee; continue;
 }
 0x10b28ee => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2876; continue; }
  pc = 0x10b2903; continue;
 }
 0x10b2903 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2876; continue; }
  pc = 0x10b2922; continue;
 }
 0x10b2922 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x10b2928; continue;
 }
 0x10b2928 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b292f); f_010c354e(cpu); if cpu.returned_to != 0x10b292f { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b293a); f_010c3527(cpu); if cpu.returned_to != 0x10b293a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b2941; continue; }
  pc = 0x10b293e; continue;
 }
 0x10b293e => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b2941; continue;
 }
 0x10b2941 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x10b2950; continue; }
  pc = 0x10b294d; continue;
 }
 0x10b294d => {
  { let v: u8 = (cpu.edx as u8) | 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10b2950; continue;
 }
 0x10b2950 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b295a; continue; }
  pc = 0x10b2957; continue;
 }
 0x10b2957 => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10b295a; continue;
 }
 0x10b295a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x10b2964; continue; }
  pc = 0x10b2961; continue;
 }
 0x10b2961 => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10b2964; continue;
 }
 0x10b2964 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x10b296e; continue; }
  pc = 0x10b296b; continue;
 }
 0x10b296b => {
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10b296e; continue;
 }
 0x10b296e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if cpu.zf { pc = 0x10b2982; continue; }
  pc = 0x10b297b; continue;
 }
 0x10b297b => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x10b2990; continue; }
  pc = 0x10b2980; continue;
 }
 0x10b2980 => {
  pc = 0x10b298e; continue;
 }
 0x10b2982 => {
  { cpu.sub32(cpu.r32(0x117b419u32), 0x200u32); }
  if !cpu.zf { pc = 0x10b2990; continue; }
  pc = 0x10b298e; continue;
 }
 0x10b298e => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10b2990; continue;
 }
 0x10b2990 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10b2997); f_010c35a3(cpu); if cpu.returned_to != 0x10b2997 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10b2999; continue;
 }
 0x10b2999 => {
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
pub fn f_010b29a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b29a2); }
 let mut pc: u32 = 0x10b29a2;
 loop { match pc {
 0x10b29a2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x10b29b0); f_010c354e(cpu); if cpu.returned_to != 0x10b29b0 { return; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b29bd); f_010c35a3(cpu); if cpu.returned_to != 0x10b29bd { return; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x10b29fd; continue; }
  pc = 0x10b29c2; continue;
 }
 0x10b29c2 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b29e2; continue; }
  pc = 0x10b29d5; continue;
 }
 0x10b29d5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b29e0); f_010c34b6(cpu); if cpu.returned_to != 0x10b29e0 { return; }
  pc = 0x10b29ff; continue;
 }
 0x10b29e2 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b29d5; continue; }
  pc = 0x10b29fd; continue;
 }
 0x10b29fd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b29ff; continue;
 }
 0x10b29ff => {
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
pub fn f_010b2a07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2a07); }
 let mut pc: u32 = 0x10b2a07;
 loop { match pc {
 0x10b2a07 => {
  { let v: u32 = 0x12d185cu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2a0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2a0d); }
 let mut pc: u32 = 0x10b2a0d;
 loop { match pc {
 0x10b2a0d => {
  { let v: u32 = 0x12d1860u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2a13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2a13); }
 let mut pc: u32 = 0x10b2a13;
 loop { match pc {
 0x10b2a13 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x10b2a23; continue; }
  pc = 0x10b2a17; continue;
 }
 0x10b2a17 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x117ca0cu32)); }
  if (cpu.cf || cpu.zf) { f_010c35b8(cpu); return; }
  pc = 0x10b2a23; continue;
 }
 0x10b2a23 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x10b2a2d); f_010b0331(cpu); if cpu.returned_to != 0x10b2a2d { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2a33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2a33); }
 let mut pc: u32 = 0x10b2a33;
 loop { match pc {
 0x10b2a33 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x10b2a42); f_010c354e(cpu); if cpu.returned_to != 0x10b2a42 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b2a5e; continue; }
  pc = 0x10b2a4a; continue;
 }
 0x10b2a4a => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x10b2a4f; continue;
 }
 0x10b2a4f => {
  cpu.push(0x10b2a54); f_010b0331(cpu); if cpu.returned_to != 0x10b2a54 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b2b26; continue;
 }
 0x10b2a5e => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x10b2a69; continue; }
  pc = 0x10b2a62; continue;
 }
 0x10b2a62 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10b2a4f; continue;
 }
 0x10b2a69 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x10b2a95; continue; }
  pc = 0x10b2a6d; continue;
 }
 0x10b2a6d => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b2b24; continue; }
  pc = 0x10b2a86; continue;
 }
 0x10b2a86 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10b2a8b; continue;
 }
 0x10b2a8b => {
  cpu.push(0x10b2a90); f_010c34b6(cpu); if cpu.returned_to != 0x10b2a90 { return; }
  pc = 0x10b2b26; continue;
 }
 0x10b2a95 => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x10b2a9b; continue;
 }
 0x10b2a9b => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b2abf; continue; }
  pc = 0x10b2ab8; continue;
 }
 0x10b2ab8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x10b2a8b; continue;
 }
 0x10b2abf => {
  if cpu.zf { pc = 0x10b2b24; continue; }
  pc = 0x10b2ac1; continue;
 }
 0x10b2ac1 => {
  { let v: u32 = cpu.edi ^ cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b2b01; continue;
 }
 0x10b2ad0 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x1au32); }
  if !cpu.zf { pc = 0x10b2af1; continue; }
  pc = 0x10b2ad9; continue;
 }
 0x10b2ad9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x10b2aef); f_010b2d03(cpu); if cpu.returned_to != 0x10b2aef { return; }
  pc = 0x10b2b24; continue;
 }
 0x10b2af1 => {
  { cpu.sub32(cpu.ebx, 0xdu32); }
  if cpu.zf { pc = 0x10b2aff; continue; }
  pc = 0x10b2af6; continue;
 }
 0x10b2af6 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x10b2aff; continue;
 }
 0x10b2aff => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x10b2b01; continue;
 }
 0x10b2b01 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x10b2ad0; continue; }
  pc = 0x10b2b07; continue;
 }
 0x10b2b07 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if !cpu.zf { pc = 0x10b2b24; continue; }
  pc = 0x10b2b1c; continue;
 }
 0x10b2b1c => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2a9b; continue; }
  pc = 0x10b2b24; continue;
 }
 0x10b2b24 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b2b26; continue;
 }
 0x10b2b26 => {
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
pub fn f_010b2b2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2b2e); }
 let mut pc: u32 = 0x10b2b2e;
 loop { match pc {
 0x10b2b2e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b2b3f); f_010c354e(cpu); if cpu.returned_to != 0x10b2b3f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b2b5a; continue; }
  pc = 0x10b2b46; continue;
 }
 0x10b2b46 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x10b2b4b; continue;
 }
 0x10b2b4b => {
  cpu.push(0x10b2b50); f_010b0331(cpu); if cpu.returned_to != 0x10b2b50 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b2cfc; continue;
 }
 0x10b2b5a => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x10b2b65; continue; }
  pc = 0x10b2b5e; continue;
 }
 0x10b2b5e => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10b2b4b; continue;
 }
 0x10b2b65 => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x10b2b9c; continue; }
  pc = 0x10b2b69; continue;
 }
 0x10b2b69 => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b2b9c; continue; }
  pc = 0x10b2b8c; continue;
 }
 0x10b2b8c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b2b97); f_010c34b6(cpu); if cpu.returned_to != 0x10b2b97 { return; }
  pc = 0x10b2cfc; continue;
 }
 0x10b2b9c => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x10b2bd7; continue; }
  pc = 0x10b2ba2; continue;
 }
 0x10b2ba2 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2b8c; continue; }
  pc = 0x10b2bbc; continue;
 }
 0x10b2bbc => {
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x10b2cf9; continue; }
  pc = 0x10b2bc5; continue;
 }
 0x10b2bc5 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x10b2bcf); f_010b0331(cpu); if cpu.returned_to != 0x10b2bcf { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b2cf9; continue;
 }
 0x10b2bd7 => {
  cpu.push(0x10b2bdc); f_010c35ed(cpu); if cpu.returned_to != 0x10b2bdc { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if !cpu.cf { pc = 0x10b2bea; continue; }
  pc = 0x10b2be5; continue;
 }
 0x10b2be5 => {
  cpu.push(0x10b2bea); f_010af4ab(cpu); if cpu.returned_to != 0x10b2bea { return; }
  pc = 0x10b2bea; continue;
 }
 0x10b2bea => {
  { let v: u32 = 0x200u32; cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x230u32); }
  if !cpu.cf { pc = 0x10b2bfc; continue; }
  pc = 0x10b2bf7; continue;
 }
 0x10b2bf7 => {
  { let v: u32 = 0x80u32; cpu.edi = v; }
  pc = 0x10b2bfc; continue;
 }
 0x10b2bfc => {
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
  pc = 0x10b2c18; continue;
 }
 0x10b2c18 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x10b2cc3; continue; }
  pc = 0x10b2c24; continue;
 }
 0x10b2c24 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x10b2c78; continue; }
  pc = 0x10b2c30; continue;
 }
 0x10b2c30 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x10b2c78; continue; }
  pc = 0x10b2c3c; continue;
 }
 0x10b2c3c => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2b8c; continue; }
  pc = 0x10b2c58; continue;
 }
 0x10b2c58 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10b2c70; continue; }
  pc = 0x10b2c5c; continue;
 }
 0x10b2c5c => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x10b2c66); f_010b0331(cpu); if cpu.returned_to != 0x10b2c66 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x10b2cfc; continue;
 }
 0x10b2c70 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b2c78; continue;
 }
 0x10b2c78 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10b2c18; continue; }
  pc = 0x10b2c96; continue;
 }
 0x10b2c96 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2b8c; continue; }
  pc = 0x10b2cb2; continue;
 }
 0x10b2cb2 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2c5c; continue; }
  pc = 0x10b2cb6; continue;
 }
 0x10b2cb6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10b2c18; continue;
 }
 0x10b2cc3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b2cf9; continue; }
  pc = 0x10b2cc7; continue;
 }
 0x10b2cc7 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b2b8c; continue; }
  pc = 0x10b2ce4; continue;
 }
 0x10b2ce4 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x10b2cf9; continue; }
  pc = 0x10b2ce8; continue;
 }
 0x10b2ce8 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x10b2cf2); f_010b0331(cpu); if cpu.returned_to != 0x10b2cf2 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  pc = 0x10b2cfc; continue;
 }
 0x10b2cf9 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x10b2cfc; continue;
 }
 0x10b2cfc => {
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
pub fn f_010b2d03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2d03); }
 let mut pc: u32 = 0x10b2d03;
 loop { match pc {
 0x10b2d03 => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b2d3d; continue; }
  pc = 0x10b2d2c; continue;
 }
 0x10b2d2c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b2d36); f_010c34b6(cpu); if cpu.returned_to != 0x10b2d36 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b2d40; continue;
 }
 0x10b2d3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x10b2d40; continue;
 }
 0x10b2d40 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2d46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2d46); }
 let mut pc: u32 = 0x10b2d46;
 loop { match pc {
 0x10b2d46 => {
  pc = 0x10c35f6; continue;
 }
 0x10c35f6 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10c3613; continue; }
  pc = 0x10c3607; continue;
 }
 0x10c3607 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10c3611); f_010c34b6(cpu); if cpu.returned_to != 0x10c3611 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10c3613 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2d4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2d4b); }
 let mut pc: u32 = 0x10b2d4b;
 loop { match pc {
 0x10b2d4b => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b2d7a; continue; }
  pc = 0x10b2d6c; continue;
 }
 0x10b2d6c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10b2d71; continue;
 }
 0x10b2d71 => {
  cpu.push(0x10b2d76); f_010b0331(cpu); if cpu.returned_to != 0x10b2d76 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b2dc7; continue;
 }
 0x10b2d7a => {
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
  if !cpu.zf { pc = 0x10b2dbe; continue; }
  pc = 0x10b2da8; continue;
 }
 0x10b2da8 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10b2daf); f_010b000a(cpu); if cpu.returned_to != 0x10b2daf { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b2dbc; continue; }
  pc = 0x10b2db5; continue;
 }
 0x10b2db5 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.poll(); pc = 0x10b2d71; continue;
 }
 0x10b2dbc => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x10b2dbe; continue;
 }
 0x10b2dbe => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10b2dc7); f_0109f9a4(cpu); if cpu.returned_to != 0x10b2dc7 { return; }
  pc = 0x10b2dc7; continue;
 }
 0x10b2dc7 => {
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
pub fn f_010b2dd3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2dd3); }
 let mut pc: u32 = 0x10b2dd3;
 loop { match pc {
 0x10b2dd3 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x3bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b2df0; continue; }
  pc = 0x10b2de4; continue;
 }
 0x10b2de4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x10b2dee); f_010c34b6(cpu); if cpu.returned_to != 0x10b2dee { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b2df0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2df4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2df4); }
 let mut pc: u32 = 0x10b2df4;
 loop { match pc {
 0x10b2df4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x10b2dfd); f_010a4b94(cpu); if cpu.returned_to != 0x10b2dfd { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b2e1e; continue; }
  pc = 0x10b2e04; continue;
 }
 0x10b2e04 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x10b2e0b); f_010b2e21(cpu); if cpu.returned_to != 0x10b2e0b { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b2e1e; continue; }
  pc = 0x10b2e10; continue;
 }
 0x10b2e10 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x10b2e1c; continue; }
  pc = 0x10b2e14; continue;
 }
 0x10b2e14 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b2e1c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10b2e1e; continue;
 }
 0x10b2e1e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2e21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2e21); }
 let mut pc: u32 = 0x10b2e21;
 loop { match pc {
 0x10b2e21 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x10b2e2f); f_010b2d03(cpu); if cpu.returned_to != 0x10b2e2f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2e32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2e32); }
 let mut pc: u32 = 0x10b2e32;
 loop { match pc {
 0x10b2e32 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.push(0x10b2e3e); f_010c3617(cpu); if cpu.returned_to != 0x10b2e3e { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2e43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2e43); }
 let mut pc: u32 = 0x10b2e43;
 loop { match pc {
 0x10b2e43 => {
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
  if !cpu.zf { pc = 0x10b2e66; continue; }
  pc = 0x10b2e5f; continue;
 }
 0x10b2e5f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b2e66); f_010c36bb(cpu); if cpu.returned_to != 0x10b2e66 { return; }
  pc = 0x10b2e66; continue;
 }
 0x10b2e66 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x10b2e81; continue; }
  pc = 0x10b2e70; continue;
 }
 0x10b2e70 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  pc = 0x10b2e81; continue;
 }
 0x10b2e81 => {
  { let v: u32 = 0x10b2e32u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10b2e8d); f_010b04be(cpu); if cpu.returned_to != 0x10b2e8d { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10b2ea8; continue; }
  pc = 0x10b2e93; continue;
 }
 0x10b2e93 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x10b2ea8); f_010c3732(cpu); if cpu.returned_to != 0x10b2ea8 { return; }
  pc = 0x10b2ea8; continue;
 }
 0x10b2ea8 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x10b2eb3; continue; }
  pc = 0x10b2eae; continue;
 }
 0x10b2eae => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x10b2eb3; continue;
 }
 0x10b2eb3 => {
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
pub fn f_010b2ec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2ec2); }
 let mut pc: u32 = 0x10b2ec2;
 loop { match pc {
 0x10b2ec2 => {
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x10b2ecc; continue; }
  pc = 0x10b2ec7; continue;
 }
 0x10b2ec7 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x10b2ed5; continue; }
  pc = 0x10b2ecc; continue;
 }
 0x10b2ecc => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x10b2ed3; continue; }
  pc = 0x10b2ed1; continue;
 }
 0x10b2ed1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b2ed3; continue;
 }
 0x10b2ed3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x10b2ed5; continue;
 }
 0x10b2ed5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b2ed6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b2ed6); }
 let mut pc: u32 = 0x10b2ed6;
 loop { match pc {
 0x10b2ed6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b2efa; continue; }
  pc = 0x10b2eeb; continue;
 }
 0x10b2eeb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x10b2efa; continue; }
  pc = 0x10b2ef1; continue;
 }
 0x10b2ef1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10b2efa; continue;
 }
 0x10b2efa => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b2fb8; continue; }
  pc = 0x10b2f06; continue;
 }
 0x10b2f06 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10b2fb8; continue; }
  pc = 0x10b2f0f; continue;
 }
 0x10b2f0f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10b2f1a); f_010c380b(cpu); if cpu.returned_to != 0x10b2f1a { return; }
  cpu.push(0x10b2f1f); f_010b2ec2(cpu); if cpu.returned_to != 0x10b2f1f { return; }
  cpu.push(0x10b2f24); f_010c3863(cpu); if cpu.returned_to != 0x10b2f24 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10b2f29; continue;
 }
 0x10b2f29 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b2f41; continue; }
  pc = 0x10b2f31; continue;
 }
 0x10b2f31 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2f29; continue; }
  pc = 0x10b2f41; continue;
 }
 0x10b2f41 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x117ca64u32), 0x0u32); }
  if cpu.zf { pc = 0x10b2f5f; continue; }
  pc = 0x10b2f4b; continue;
 }
 0x10b2f4b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x117ca69u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10b2f61; continue;
 }
 0x10b2f5f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b2f61; continue;
 }
 0x10b2f61 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x117ca64u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b2f7e; continue; }
  pc = 0x10b2f6d; continue;
 }
 0x10b2f6d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x117ca69u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10b2f7e; continue;
 }
 0x10b2f7e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2f0f; continue; }
  pc = 0x10b2f86; continue;
 }
 0x10b2f86 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b2f95; continue; }
  pc = 0x10b2f8d; continue;
 }
 0x10b2f8d => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b2f95; continue;
 }
 0x10b2f95 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x10b2f9f); f_010c3ad7(cpu); if cpu.returned_to != 0x10b2f9f { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10b2fb1; continue; }
  pc = 0x10b2fae; continue;
 }
 0x10b2fae => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10b2fb8; continue;
 }
 0x10b2fb1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b2fb8; continue;
 }
 0x10b2fb8 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b2fc7; continue; }
  pc = 0x10b2fbf; continue;
 }
 0x10b2fbf => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b2fc7; continue;
 }
 0x10b2fc7 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b306f; continue; }
  pc = 0x10b2fd2; continue;
 }
 0x10b2fd2 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10b2fdd); f_010c380b(cpu); if cpu.returned_to != 0x10b2fdd { return; }
  cpu.push(0x10b2fe2); f_010b2ec2(cpu); if cpu.returned_to != 0x10b2fe2 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x10b2ff5; continue; }
  pc = 0x10b2fea; continue;
 }
 0x10b2fea => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10b2ff5; continue; }
  pc = 0x10b2ff3; continue;
 }
 0x10b2ff3 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x10b2ff5; continue;
 }
 0x10b2ff5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10b3000); f_010c380b(cpu); if cpu.returned_to != 0x10b3000 { return; }
  cpu.push(0x10b3005); f_010b2ec2(cpu); if cpu.returned_to != 0x10b3005 { return; }
  cpu.push(0x10b300a); f_010c3863(cpu); if cpu.returned_to != 0x10b300a { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x10b300f; continue;
 }
 0x10b300f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b3027; continue; }
  pc = 0x10b3017; continue;
 }
 0x10b3017 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b300f; continue; }
  pc = 0x10b3027; continue;
 }
 0x10b3027 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117ca64u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b3043; continue; }
  pc = 0x10b3031; continue;
 }
 0x10b3031 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x117ca69u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10b3043; continue;
 }
 0x10b3043 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x117ca64u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b3063; continue; }
  pc = 0x10b3050; continue;
 }
 0x10b3050 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x117ca69u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10b3065; continue;
 }
 0x10b3063 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b3065; continue;
 }
 0x10b3065 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b2ff5; continue; }
  pc = 0x10b306d; continue;
 }
 0x10b306d => {
  pc = 0x10b307e; continue;
 }
 0x10b306f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x10b307e; continue; }
  pc = 0x10b307c; continue;
 }
 0x10b307c => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x10b307e; continue;
 }
 0x10b307e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b30b4; continue; }
  pc = 0x10b3086; continue;
 }
 0x10b3086 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10b30b4; continue; }
  pc = 0x10b308b; continue;
 }
 0x10b308b => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x10b309d; continue; }
  pc = 0x10b3098; continue;
 }
 0x10b3098 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10b309d; continue;
 }
 0x10b309d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x10b30b4; continue; }
  pc = 0x10b30a7; continue;
 }
 0x10b30a7 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b309d; continue;
 }
 0x10b30b4 => {
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
pub fn f_010b30c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b30c4); } r_010b30c4(cpu, 0x10b30c4); }
pub fn f_010b30cc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b30cc); } r_010b30c4(cpu, 0x10b30cc); }
fn r_010b30c4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b30c4 => {
  cpu.unimplemented(0x10b30c4, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x32u32.wrapping_add(cpu.ebp)), 0x51530078u32); cpu.esp = v; }
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
  if cpu.zf { pc = 0x10b317e; continue; }
  pc = 0x10b315c; continue;
 }
 0x10b30cc => {
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
  if cpu.zf { pc = 0x10b317e; continue; }
  pc = 0x10b315c; continue;
 }
 0x10b315c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x17u32); }
  if cpu.zf { pc = 0x10b317e; continue; }
  pc = 0x10b3162; continue;
 }
 0x10b3162 => {
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
  pc = 0x10b317e; continue;
 }
 0x10b317e => {
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
pub fn f_010b318f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b318f); }
 let mut pc: u32 = 0x10b318f;
 loop { match pc {
 0x10b318f => {
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
  { let v: u32 = 0x116b7b8u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0xf4fb7ea1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10b31c3; continue; }
  pc = 0x10b31b9; continue;
 }
 0x10b31b9 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b32dc; continue;
 }
 0x10b31c3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x10b31d4; continue;
 }
 0x10b31d4 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x10b31de; continue; }
  pc = 0x10b31d8; continue;
 }
 0x10b31d8 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  cpu.poll(); pc = 0x10b31d4; continue;
 }
 0x10b31de => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b31b9; continue; }
  pc = 0x10b31e2; continue;
 }
 0x10b31e2 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x10b31ed); f_010b37f3(cpu); if cpu.returned_to != 0x10b31ed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b31f6; continue; }
  pc = 0x10b31f1; continue;
 }
 0x10b31f1 => {
  { let v: u32 = 0x116b7d2u32; cpu.ebx = v; }
  pc = 0x10b31f6; continue;
 }
 0x10b31f6 => {
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
  if cpu.zf { pc = 0x10b3223; continue; }
  pc = 0x10b3222; continue;
 }
 0x10b3222 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10b3223; continue;
 }
 0x10b3223 => {
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
  pc = 0x10b3244; continue;
 }
 0x10b3244 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x10b3251; continue; }
  pc = 0x10b3248; continue;
 }
 0x10b3248 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10b3244; continue;
 }
 0x10b3251 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10b325e); f_010b36b3(cpu); if cpu.returned_to != 0x10b325e { return; }
  cpu.push(0x10b3263); f_010b334f(cpu); if cpu.returned_to != 0x10b3263 { return; }
  { let v: u32 = cpu.r32(0x117baa4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x10b3279; continue; }
  pc = 0x10b3272; continue;
 }
 0x10b3272 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10b3279); f_010b3947(cpu); if cpu.returned_to != 0x10b3279 { return; }
  pc = 0x10b3279; continue;
 }
 0x10b3279 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b3285; continue; }
  pc = 0x10b327f; continue;
 }
 0x10b327f => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x117baa8u32)); cpu.esi = v; }
  pc = 0x10b3285; continue;
 }
 0x10b3285 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x10b3292; continue; }
  pc = 0x10b3289; continue;
 }
 0x10b3289 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10b3285; continue;
 }
 0x10b3292 => {
  { cpu.sub32(cpu.edi, 0x63deu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b31b9; continue; }
  pc = 0x10b329e; continue;
 }
 0x10b329e => {
  if !cpu.zf { pc = 0x10b32bc; continue; }
  pc = 0x10b32a0; continue;
 }
 0x10b32a0 => {
  { let v: u32 = cpu.r32(0x117baa4u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x15180u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b31b9; continue; }
  pc = 0x10b32b3; continue;
 }
 0x10b32b3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x10b32da; continue; }
  pc = 0x10b32b7; continue;
 }
 0x10b32b7 => {
  cpu.poll(); pc = 0x10b31b9; continue;
 }
 0x10b32bc => {
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
  pc = 0x10b32da; continue;
 }
 0x10b32da => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b32dc; continue;
 }
 0x10b32dc => {
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
pub fn f_010b32e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b32e3); }
 let mut pc: u32 = 0x10b32e3;
 loop { match pc {
 0x10b32e3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x117baf6u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x117baf6u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x117baf6u32, v); }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117baf6u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117baf6u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x117baf6u32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x117baf6u32, v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x117baf6u32), cpu.edx); cpu.w32(0x117baf6u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b334f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b334f); }
 let mut pc: u32 = 0x10b334f;
 loop { match pc {
 0x10b3337 => {
  { let v: u8 = cpu.r8(0x117baf6u32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b3347; continue; }
  pc = 0x10b3342; continue;
 }
 0x10b3342 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.zf { pc = 0x10b334e; continue; }
  pc = 0x10b3347; continue;
 }
 0x10b3347 => {
  { let v: u8 = cpu.or8(cpu.r8(0x117baf6u32), 0x2u8); cpu.w8(0x117baf6u32, v); }
  pc = 0x10b334e; continue;
 }
 0x10b334e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b334f => {
  { let v: u32 = 0x116ad68u32; cpu.eax = v; }
  cpu.push(0x10b3359); f_010be6c4(cpu); if cpu.returned_to != 0x10b3359 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x10b3337; continue; }
  pc = 0x10b335d; continue;
 }
 0x10b335d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x117baa4u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117baacu32, v); }
  { let v: u32 = 0x117bab0u32; cpu.edx = v; }
  cpu.push(0x10b337d); f_010b33b8(cpu); if cpu.returned_to != 0x10b337d { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b35e5; continue; }
  pc = 0x10b3389; continue;
 }
 0x10b3389 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x117bacfu32, v); }
  pc = 0x10b36a9; continue;
 }
 0x10b35e5 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x117bacfu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x117baa4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117baacu32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b360b); f_010b33b8(cpu); if cpu.returned_to != 0x10b360b { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x117baa4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117baa8u32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, 0x2cu32); }
  if !cpu.zf { pc = 0x10b3634; continue; }
  pc = 0x10b3627; continue;
 }
 0x10b3627 => {
  { let v: u32 = 0x117ba5cu32; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x10b3632); f_010b34e1(cpu); if cpu.returned_to != 0x10b3632 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10b3634; continue;
 }
 0x10b3634 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if !cpu.zf { pc = 0x10b36a9; continue; }
  pc = 0x10b363d; continue;
 }
 0x10b363d => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x117ba80u32; cpu.edx = v; }
  cpu.push(0x10b364a); f_010b34e1(cpu); if cpu.returned_to != 0x10b364a { return; }
  { let v: u32 = cpu.r32(0x117baa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xe10u32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x117ba88u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x117baa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x117ba84u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x117baa8u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x117ba80u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117ba88u32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x117ba84u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x117ba80u32, v); }
  pc = 0x10b36a9; continue;
 }
 0x10b36a9 => {
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
pub fn f_010b3394(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3394); }
 let mut pc: u32 = 0x10b3394;
 loop { match pc {
 0x10b3394 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10b339a; continue;
 }
 0x10b339a => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b33b3; continue; }
  pc = 0x10b33a3; continue;
 }
 0x10b33a3 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b33b3; continue; }
  pc = 0x10b33a8; continue;
 }
 0x10b33a8 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b339a; continue;
 }
 0x10b33b3 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b33b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b33b8); }
 let mut pc: u32 = 0x10b33b8;
 loop { match pc {
 0x10b33b8 => {
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
  if !cpu.zf { pc = 0x10b33d0; continue; }
  pc = 0x10b33cf; continue;
 }
 0x10b33cf => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x10b33d0; continue;
 }
 0x10b33d0 => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x10b33d2; continue;
 }
 0x10b33d2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10b33f9; continue; }
  pc = 0x10b33d9; continue;
 }
 0x10b33d9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if cpu.zf { pc = 0x10b33f9; continue; }
  pc = 0x10b33e2; continue;
 }
 0x10b33e2 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if cpu.zf { pc = 0x10b33f9; continue; }
  pc = 0x10b33e7; continue;
 }
 0x10b33e7 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x10b33f9; continue; }
  pc = 0x10b33ec; continue;
 }
 0x10b33ec => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b33f6; continue; }
  pc = 0x10b33f1; continue;
 }
 0x10b33f1 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b33f9; continue; }
  pc = 0x10b33f6; continue;
 }
 0x10b33f6 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = 0x10b33d2; continue;
 }
 0x10b33f9 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b340e; continue; }
  pc = 0x10b3406; continue;
 }
 0x10b3406 => {
  { let v: u32 = 0x1eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b340e; continue;
 }
 0x10b340e => {
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
  if !cpu.zf { pc = 0x10b344b; continue; }
  pc = 0x10b3444; continue;
 }
 0x10b3444 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x10b3450; continue;
 }
 0x10b344b => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x10b3452; continue; }
  pc = 0x10b3450; continue;
 }
 0x10b3450 => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x10b3452; continue;
 }
 0x10b3452 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b34d7; continue; }
  pc = 0x10b3463; continue;
 }
 0x10b3463 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b34d7; continue; }
  pc = 0x10b3468; continue;
 }
 0x10b3468 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10b3480); f_010b3394(cpu); if cpu.returned_to != 0x10b3480 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x10b34aa; continue; }
  pc = 0x10b348b; continue;
 }
 0x10b348b => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x10b3495); f_010b3394(cpu); if cpu.returned_to != 0x10b3495 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x10b34aa; continue; }
  pc = 0x10b34a0; continue;
 }
 0x10b34a0 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x10b34a8); f_010b3394(cpu); if cpu.returned_to != 0x10b34a8 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x10b34aa; continue;
 }
 0x10b34aa => {
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
  if cpu.zf { pc = 0x10b34d7; continue; }
  pc = 0x10b34d3; continue;
 }
 0x10b34d3 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b34d7; continue;
 }
 0x10b34d7 => {
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
pub fn f_010b34e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b34e1); }
 let mut pc: u32 = 0x10b34e1;
 loop { match pc {
 0x10b34e1 => {
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
  if !cpu.zf { pc = 0x10b3500; continue; }
  pc = 0x10b34f9; continue;
 }
 0x10b34f9 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x10b3500; continue;
 }
 0x10b3500 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4du32); }
  if !cpu.zf { pc = 0x10b350c; continue; }
  pc = 0x10b3509; continue;
 }
 0x10b3509 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b350c; continue;
 }
 0x10b350c => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x10b351a); f_010b3394(cpu); if cpu.returned_to != 0x10b351a { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b352b; continue; }
  pc = 0x10b3522; continue;
 }
 0x10b3522 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b3578; continue;
 }
 0x10b352b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x10b3571; continue; }
  pc = 0x10b353c; continue;
 }
 0x10b353c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10b3548); f_010b3394(cpu); if cpu.returned_to != 0x10b3548 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x10b3571; continue; }
  pc = 0x10b355c; continue;
 }
 0x10b355c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x10b3568); f_010b3394(cpu); if cpu.returned_to != 0x10b3568 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b3571; continue;
 }
 0x10b3571 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b3578; continue;
 }
 0x10b3578 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x10b35c9; continue; }
  pc = 0x10b3593; continue;
 }
 0x10b3593 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x10b359f); f_010b3394(cpu); if cpu.returned_to != 0x10b359f { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x10b35c9; continue; }
  pc = 0x10b35aa; continue;
 }
 0x10b35aa => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x10b35b4); f_010b3394(cpu); if cpu.returned_to != 0x10b35b4 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x10b35c9; continue; }
  pc = 0x10b35bf; continue;
 }
 0x10b35bf => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x10b35c7); f_010b3394(cpu); if cpu.returned_to != 0x10b35c7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x10b35c9; continue;
 }
 0x10b35c9 => {
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
pub fn f_010b36b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b36b3); }
 let mut pc: u32 = 0x10b36b3;
 loop { match pc {
 0x10b36b3 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xa8c0u32); }
  if !cpu.cf { pc = 0x10b36e0; continue; }
  pc = 0x10b36c4; continue;
 }
 0x10b36c4 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b36e0; continue; }
  pc = 0x10b36c8; continue;
 }
 0x10b36c8 => {
  { let v: u32 = 0x15180u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10b36ef; continue;
 }
 0x10b36e0 => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x10b36ef; continue;
 }
 0x10b36ef => {
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
  if (cpu.cf || cpu.zf) { pc = 0x10b3756; continue; }
  pc = 0x10b374e; continue;
 }
 0x10b374e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x10b3756; continue;
 }
 0x10b3756 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x10b3761); f_010b37f3(cpu); if cpu.returned_to != 0x10b3761 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x16du32); cpu.eax = v; }
  pc = 0x10b3766; continue;
 }
 0x10b3766 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x10b376f; continue; }
  pc = 0x10b376a; continue;
 }
 0x10b376a => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b3766; continue;
 }
 0x10b376f => {
  { let v: u32 = 0x116b7b8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x10b3785); f_010b37f3(cpu); if cpu.returned_to != 0x10b3785 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b378e; continue; }
  pc = 0x10b3789; continue;
 }
 0x10b3789 => {
  { let v: u32 = 0x116b7d2u32; cpu.esi = v; }
  pc = 0x10b378e; continue;
 }
 0x10b378e => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x10b37a6; continue; }
  pc = 0x10b37a5; continue;
 }
 0x10b37a5 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10b37a6; continue;
 }
 0x10b37a6 => {
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
pub fn f_010b37c8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b37c8); }
 let mut pc: u32 = 0x10b37c8;
 loop { match pc {
 0x10b37c8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  cpu.push(0x10b37e3); f_010b36b3(cpu); if cpu.returned_to != 0x10b37e3 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b37f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b37f3); }
 let mut pc: u32 = 0x10b37f3;
 loop { match pc {
 0x10b37f3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x10b3828; continue; }
  pc = 0x10b37fd; continue;
 }
 0x10b37fd => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b3816; continue; }
  pc = 0x10b380d; continue;
 }
 0x10b380d => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3816 => {
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x10b380d; continue; }
  pc = 0x10b3828; continue;
 }
 0x10b3828 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b382e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b382e); }
 let mut pc: u32 = 0x10b382e;
 loop { match pc {
 0x10b382e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b38fb; continue; }
  pc = 0x10b3843; continue;
 }
 0x10b3843 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x10b384e); f_010b37f3(cpu); if cpu.returned_to != 0x10b384e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b3865; continue; }
  pc = 0x10b3852; continue;
 }
 0x10b3852 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x116b7d2u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x116b7d0u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x10b3876; continue;
 }
 0x10b3865 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x116b7b8u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x116b7b6u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x10b3876; continue;
 }
 0x10b3876 => {
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
  cpu.push(0x10b38aa); f_010b318f(cpu); if cpu.returned_to != 0x10b38aa { return; }
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
  if !cpu.zf { pc = 0x10b38e9; continue; }
  pc = 0x10b38ca; continue;
 }
 0x10b38ca => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b38e5; continue; }
  pc = 0x10b38dd; continue;
 }
 0x10b38dd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  pc = 0x10b38ea; continue;
 }
 0x10b38e5 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x10b38ea; continue;
 }
 0x10b38e9 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x10b38ea; continue;
 }
 0x10b38ea => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x10b390a; continue;
 }
 0x10b38fb => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x10b3907; continue; }
  pc = 0x10b3900; continue;
 }
 0x10b3900 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x10b390a; continue;
 }
 0x10b3907 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10b390a; continue;
 }
 0x10b390a => {
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
pub fn f_010b3913(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3913); }
 let mut pc: u32 = 0x10b3913;
 loop { match pc {
 0x10b3913 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b3927; continue; }
  pc = 0x10b391f; continue;
 }
 0x10b391f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3927 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.ecx))); }
  if (cpu.sf != cpu.of) { pc = 0x10b3942; continue; }
  pc = 0x10b392c; continue;
 }
 0x10b392c => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x10b3933); f_010b382e(cpu); if cpu.returned_to != 0x10b3933 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b393e); f_010b382e(cpu); if cpu.returned_to != 0x10b393e { return; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b391f; continue; }
  pc = 0x10b3942; continue;
 }
 0x10b3942 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b3947(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3947); }
 let mut pc: u32 = 0x10b3947;
 loop { match pc {
 0x10b3947 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x117baf2u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10b3bfb; continue; }
  pc = 0x10b3963; continue;
 }
 0x10b3963 => {
  { let v: u32 = 0x117ba80u32; cpu.edx = v; }
  { let v: u32 = 0x117ba5cu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  cpu.push(0x10b3975); f_010b3913(cpu); if cpu.returned_to != 0x10b3975 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b3989; continue; }
  pc = 0x10b397d; continue;
 }
 0x10b397d => {
  { let v: u32 = 0x117ba80u32; cpu.esi = v; }
  { let v: u32 = 0x117ba5cu32; cpu.ebx = v; }
  pc = 0x10b3993; continue;
 }
 0x10b3989 => {
  { let v: u32 = 0x117ba5cu32; cpu.esi = v; }
  { let v: u32 = 0x117ba80u32; cpu.ebx = v; }
  pc = 0x10b3993; continue;
 }
 0x10b3993 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  cpu.push(0x10b39a3); f_010b37f3(cpu); if cpu.returned_to != 0x10b39a3 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b39bb; continue; }
  pc = 0x10b39ad; continue;
 }
 0x10b39ad => {
  { let v: u32 = cpu.r32(0x116b7d2u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x116b7d0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10b39c7; continue;
 }
 0x10b39bb => {
  { let v: u32 = cpu.r32(0x116b7b8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x116b7b6u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10b39c7; continue;
 }
 0x10b39c7 => {
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b3a97; continue; }
  pc = 0x10b39e4; continue;
 }
 0x10b39e4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b39f5; continue; }
  pc = 0x10b39eb; continue;
 }
 0x10b39eb => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10b3ad0; continue;
 }
 0x10b39f5 => {
  if !cpu.zf { pc = 0x10b3ad0; continue; }
  pc = 0x10b39fb; continue;
 }
 0x10b39fb => {
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
  if !cpu.zf { pc = 0x10b3a70; continue; }
  pc = 0x10b3a4b; continue;
 }
 0x10b3a4b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b3ad0; continue; }
  pc = 0x10b3a5d; continue;
 }
 0x10b3a5d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10b3ad0; continue; }
  pc = 0x10b3a6e; continue;
 }
 0x10b3a6e => {
  pc = 0x10b3acc; continue;
 }
 0x10b3a70 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b3ad0; continue; }
  pc = 0x10b3a88; continue;
 }
 0x10b3a88 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b3ad0; continue; }
  pc = 0x10b3a95; continue;
 }
 0x10b3a95 => {
  pc = 0x10b3acc; continue;
 }
 0x10b3a97 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10b3abe; continue; }
  pc = 0x10b3a9f; continue;
 }
 0x10b3a9f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x10b3aac); f_010b37f3(cpu); if cpu.returned_to != 0x10b3aac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b3abd; continue; }
  pc = 0x10b3ab0; continue;
 }
 0x10b3ab0 => {
  { let v: u32 = cpu.r32(0x116b7bau32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b3abd; continue; }
  pc = 0x10b3abc; continue;
 }
 0x10b3abc => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10b3abd; continue;
 }
 0x10b3abd => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x10b3abe; continue;
 }
 0x10b3abe => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b3ad0; continue; }
  pc = 0x10b3ac5; continue;
 }
 0x10b3ac5 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  if !cpu.zf { pc = 0x10b3ad0; continue; }
  pc = 0x10b3acc; continue;
 }
 0x10b3acc => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b3ad0; continue;
 }
 0x10b3ad0 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10b3aed; continue; }
  pc = 0x10b3ad7; continue;
 }
 0x10b3ad7 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b3ae0); f_010b3c0a(cpu); if cpu.returned_to != 0x10b3ae0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  pc = 0x10b3aed; continue;
 }
 0x10b3aed => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10b3b04; continue; }
  pc = 0x10b3af1; continue;
 }
 0x10b3af1 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b3bfb; continue; }
  pc = 0x10b3afd; continue;
 }
 0x10b3afd => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b3bfb; continue;
 }
 0x10b3b04 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b3ba7; continue; }
  pc = 0x10b3b11; continue;
 }
 0x10b3b11 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b3b1f; continue; }
  pc = 0x10b3b18; continue;
 }
 0x10b3b18 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b3be0; continue;
 }
 0x10b3b1f => {
  if !cpu.zf { pc = 0x10b3be0; continue; }
  pc = 0x10b3b25; continue;
 }
 0x10b3b25 => {
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
  if !cpu.zf { pc = 0x10b3b88; continue; }
  pc = 0x10b3b69; continue;
 }
 0x10b3b69 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10b3b82; continue; }
  pc = 0x10b3b78; continue;
 }
 0x10b3b78 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10b3be0; continue;
 }
 0x10b3b82 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b3be0; continue; }
  pc = 0x10b3b86; continue;
 }
 0x10b3b86 => {
  pc = 0x10b3bdb; continue;
 }
 0x10b3b88 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b3b78; continue; }
  pc = 0x10b3ba1; continue;
 }
 0x10b3ba1 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b3be0; continue; }
  pc = 0x10b3ba5; continue;
 }
 0x10b3ba5 => {
  pc = 0x10b3bdb; continue;
 }
 0x10b3ba7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10b3bce; continue; }
  pc = 0x10b3baf; continue;
 }
 0x10b3baf => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x10b3bbc); f_010b37f3(cpu); if cpu.returned_to != 0x10b3bbc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b3bcd; continue; }
  pc = 0x10b3bc0; continue;
 }
 0x10b3bc0 => {
  { let v: u32 = cpu.r32(0x116b7bau32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b3bcd; continue; }
  pc = 0x10b3bcc; continue;
 }
 0x10b3bcc => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10b3bcd; continue;
 }
 0x10b3bcd => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x10b3bce; continue;
 }
 0x10b3bce => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b3be0; continue; }
  pc = 0x10b3bd5; continue;
 }
 0x10b3bd5 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x10b3be0; continue; }
  pc = 0x10b3bdb; continue;
 }
 0x10b3bdb => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x10b3be0; continue;
 }
 0x10b3be0 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b3bef; continue; }
  pc = 0x10b3be4; continue;
 }
 0x10b3be4 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10b3bed); f_010b3c0a(cpu); if cpu.returned_to != 0x10b3bed { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x10b3bef; continue;
 }
 0x10b3bef => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b3bfb; continue; }
  pc = 0x10b3bf7; continue;
 }
 0x10b3bf7 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x10b3bfb; continue;
 }
 0x10b3bfb => {
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
pub fn f_010b3c0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3c0a); }
 let mut pc: u32 = 0x10b3c0a;
 loop { match pc {
 0x10b3c0a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x10b3c2e; continue; }
  pc = 0x10b3c1a; continue;
 }
 0x10b3c1a => {
  if !cpu.zf { pc = 0x10b3c33; continue; }
  pc = 0x10b3c1c; continue;
 }
 0x10b3c1c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10b3c2e; continue; }
  pc = 0x10b3c26; continue;
 }
 0x10b3c26 => {
  if !cpu.zf { pc = 0x10b3c33; continue; }
  pc = 0x10b3c28; continue;
 }
 0x10b3c28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x10b3c33; continue; }
  pc = 0x10b3c2e; continue;
 }
 0x10b3c2e => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x10b3c33; continue;
 }
 0x10b3c33 => {
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
pub fn f_010b3c3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3c3c); }
 let mut pc: u32 = 0x10b3c3c;
 loop { match pc {
 0x10b3c3c => {
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
pub fn f_010b3c64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3c64); }
 let mut pc: u32 = 0x10b3c64;
 loop { match pc {
 0x10b3c64 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b3ca7; continue; }
  pc = 0x10b3c78; continue;
 }
 0x10b3c78 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b3ca0; continue; }
  pc = 0x10b3c7c; continue;
 }
 0x10b3c7c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x10b3c90; continue; }
  pc = 0x10b3c86; continue;
 }
 0x10b3c86 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b3cdf; continue;
 }
 0x10b3c90 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b3c99; continue; }
  pc = 0x10b3c92; continue;
 }
 0x10b3c92 => {
  { let v: u32 = 0x8047u32; cpu.eax = v; }
  pc = 0x10b3cd1; continue;
 }
 0x10b3c99 => {
  { let v: u32 = 0x4047u32; cpu.eax = v; }
  pc = 0x10b3cd1; continue;
 }
 0x10b3ca0 => {
  { let v: u32 = 0x2047u32; cpu.eax = v; }
  pc = 0x10b3cd1; continue;
 }
 0x10b3ca7 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.cf { pc = 0x10b3cbb; continue; }
  pc = 0x10b3cb1; continue;
 }
 0x10b3cb1 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b3cdf; continue;
 }
 0x10b3cbb => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x10b3ccc; continue; }
  pc = 0x10b3cc5; continue;
 }
 0x10b3cc5 => {
  { let v: u32 = 0x8107u32; cpu.eax = v; }
  pc = 0x10b3cd1; continue;
 }
 0x10b3ccc => {
  { let v: u32 = 0x1107u32; cpu.eax = v; }
  pc = 0x10b3cd1; continue;
 }
 0x10b3cd1 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x10b3cdc); f_010c3c4b(cpu); if cpu.returned_to != 0x10b3cdc { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x10b3cdf; continue;
 }
 0x10b3cdf => {
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
pub fn f_010b3cf8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3cf8); }
 let mut pc: u32 = 0x10b3cf8;
 loop { match pc {
 0x10b3cf8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x117bb20u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x10b3d27; continue; }
  pc = 0x10b3d0f; continue;
 }
 0x10b3d0f => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x10b3d21; continue; }
  pc = 0x10b3d13; continue;
 }
 0x10b3d13 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10b3d21); f_010c3d26(cpu); if cpu.returned_to != 0x10b3d21 { return; }
  pc = 0x10b3d21; continue;
 }
 0x10b3d21 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3d27 => {
  { let a = cpu.st(0); let b = cpu.rf64(0x117bb28u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b3d3c; continue; }
  pc = 0x10b3d38; continue;
 }
 0x10b3d38 => {
  { cpu.fpush(0.0); }
  pc = 0x10b3d50; continue;
 }
 0x10b3d3c => {
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
  pc = 0x10b3d50; continue;
 }
 0x10b3d50 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b3d64(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b3d64); }
 let mut pc: u32 = 0x10b3d64;
 loop { match pc {
 0x10b3d64 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x10b3d6c; continue;
 }
 0x10b3d6c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x10b3dfe; continue; }
  pc = 0x10b3d78; continue;
 }
 0x10b3d78 => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x10b3d87; continue; }
  pc = 0x10b3d84; continue;
 }
 0x10b3d84 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3d87 => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x117d01cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x10b3d96; continue; }
  pc = 0x10b3d93; continue;
 }
 0x10b3d93 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3d96 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b3e08; continue; }
  pc = 0x10b3da1; continue;
 }
 0x10b3da1 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x10b3e08; continue; }
  pc = 0x10b3da8; continue;
 }
 0x10b3da8 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10b3de7; continue; }
  pc = 0x10b3dd0; continue;
 }
 0x10b3dd0 => {
  { let v = cpu.st(0) * cpu.rf32(0x117d02cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x117d02cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3de7 => {
  { let v = cpu.st(0) * cpu.rf32(0x117d030u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x117d030u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3dfe => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x10b3e0b; continue; }
  pc = 0x10b3e08; continue;
 }
 0x10b3e08 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3e0b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10b3e08; continue; }
  pc = 0x10b3e16; continue;
 }
 0x10b3e16 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b3e4c; continue; }
  pc = 0x10b3e3b; continue;
 }
 0x10b3e3b => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x10b3e74; continue; }
  pc = 0x10b3e42; continue;
 }
 0x10b3e42 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x10b3e74; continue; }
  pc = 0x10b3e4a; continue;
 }
 0x10b3e4a => {
  pc = 0x10b3e54; continue;
 }
 0x10b3e4c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x10b3e74; continue; }
  pc = 0x10b3e54; continue;
 }
 0x10b3e54 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x117d034u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  cpu.poll(); pc = 0x10b3d6c; continue;
 }
 0x10b3e74 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b3e7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3e7b); } r_010b3e7b(cpu, 0x10b3e7b); }
pub fn f_010b3e85(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3e85); } r_010b3e7b(cpu, 0x10b3e85); }
pub fn f_010b3e8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3e8b); } r_010b3e7b(cpu, 0x10b3e8b); }
pub fn f_010b3e90(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3e90); } r_010b3e7b(cpu, 0x10b3e90); }
pub fn f_010b3e96(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3e96); } r_010b3e7b(cpu, 0x10b3e96); }
pub fn f_010b3e9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3e9b); } r_010b3e7b(cpu, 0x10b3e9b); }
pub fn f_010b3ea1(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3ea1); } r_010b3e7b(cpu, 0x10b3ea1); }
pub fn f_010b3ea7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3ea7); } r_010b3e7b(cpu, 0x10b3ea7); }
pub fn f_010b3ead(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3ead); } r_010b3e7b(cpu, 0x10b3ead); }
pub fn f_010b3eb3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3eb3); } r_010b3e7b(cpu, 0x10b3eb3); }
pub fn f_010b3ecf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3ecf); } r_010b3e7b(cpu, 0x10b3ecf); }
pub fn f_010b3ed4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3ed4); } r_010b3e7b(cpu, 0x10b3ed4); }
pub fn f_010b3eea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3eea); } r_010b3e7b(cpu, 0x10b3eea); }
pub fn f_010b3eef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3eef); } r_010b3e7b(cpu, 0x10b3eef); }
pub fn f_010b3f0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f0b); } r_010b3e7b(cpu, 0x10b3f0b); }
pub fn f_010b3f1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f1b); } r_010b3e7b(cpu, 0x10b3f1b); }
pub fn f_010b3f2f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f2f); } r_010b3e7b(cpu, 0x10b3f2f); }
pub fn f_010b3f3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f3f); } r_010b3e7b(cpu, 0x10b3f3f); }
pub fn f_010b3f5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f5f); } r_010b3e7b(cpu, 0x10b3f5f); }
pub fn f_010b3f64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f64); } r_010b3e7b(cpu, 0x10b3f64); }
pub fn f_010b3f7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f7e); } r_010b3e7b(cpu, 0x10b3f7e); }
pub fn f_010b3f83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3f83); } r_010b3e7b(cpu, 0x10b3f83); }
pub fn f_010b3fa3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3fa3); } r_010b3e7b(cpu, 0x10b3fa3); }
pub fn f_010b3fb7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3fb7); } r_010b3e7b(cpu, 0x10b3fb7); }
pub fn f_010b3fcf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3fcf); } r_010b3e7b(cpu, 0x10b3fcf); }
pub fn f_010b3fe3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b3fe3); } r_010b3e7b(cpu, 0x10b3fe3); }
pub fn f_010b4003(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4003); } r_010b3e7b(cpu, 0x10b4003); }
pub fn f_010b4008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4008); } r_010b3e7b(cpu, 0x10b4008); }
pub fn f_010b4022(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4022); } r_010b3e7b(cpu, 0x10b4022); }
pub fn f_010b4027(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4027); } r_010b3e7b(cpu, 0x10b4027); }
pub fn f_010b4047(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4047); } r_010b3e7b(cpu, 0x10b4047); }
pub fn f_010b405b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b405b); } r_010b3e7b(cpu, 0x10b405b); }
pub fn f_010b4073(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4073); } r_010b3e7b(cpu, 0x10b4073); }
pub fn f_010b4087(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4087); } r_010b3e7b(cpu, 0x10b4087); }
pub fn f_010b40a7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b40a7); } r_010b3e7b(cpu, 0x10b40a7); }
pub fn f_010b40ac(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b40ac); } r_010b3e7b(cpu, 0x10b40ac); }
pub fn f_010b40c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b40c6); } r_010b3e7b(cpu, 0x10b40c6); }
pub fn f_010b40cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b40cb); } r_010b3e7b(cpu, 0x10b40cb); }
pub fn f_010b40eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b40eb); } r_010b3e7b(cpu, 0x10b40eb); }
pub fn f_010b40ff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b40ff); } r_010b3e7b(cpu, 0x10b40ff); }
pub fn f_010b4117(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4117); } r_010b3e7b(cpu, 0x10b4117); }
pub fn f_010b412b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b412b); } r_010b3e7b(cpu, 0x10b412b); }
pub fn f_010b414b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b414b); } r_010b3e7b(cpu, 0x10b414b); }
pub fn f_010b4150(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4150); } r_010b3e7b(cpu, 0x10b4150); }
pub fn f_010b416a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b416a); } r_010b3e7b(cpu, 0x10b416a); }
pub fn f_010b416f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b416f); } r_010b3e7b(cpu, 0x10b416f); }
pub fn f_010b418f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b418f); } r_010b3e7b(cpu, 0x10b418f); }
pub fn f_010b41a3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b41a3); } r_010b3e7b(cpu, 0x10b41a3); }
pub fn f_010b41bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b41bb); } r_010b3e7b(cpu, 0x10b41bb); }
pub fn f_010b41cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b41cf); } r_010b3e7b(cpu, 0x10b41cf); }
pub fn f_010b41ef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b41ef); } r_010b3e7b(cpu, 0x10b41ef); }
pub fn f_010b41f4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b41f4); } r_010b3e7b(cpu, 0x10b41f4); }
pub fn f_010b420e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b420e); } r_010b3e7b(cpu, 0x10b420e); }
pub fn f_010b4213(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4213); } r_010b3e7b(cpu, 0x10b4213); }
pub fn f_010b4233(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4233); } r_010b3e7b(cpu, 0x10b4233); }
pub fn f_010b4247(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4247); } r_010b3e7b(cpu, 0x10b4247); }
pub fn f_010b425f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b425f); } r_010b3e7b(cpu, 0x10b425f); }
pub fn f_010b4273(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4273); } r_010b3e7b(cpu, 0x10b4273); }
pub fn f_010b4293(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4293); } r_010b3e7b(cpu, 0x10b4293); }
pub fn f_010b4298(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4298); } r_010b3e7b(cpu, 0x10b4298); }
pub fn f_010b42b2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b42b2); } r_010b3e7b(cpu, 0x10b42b2); }
pub fn f_010b42b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b42b7); } r_010b3e7b(cpu, 0x10b42b7); }
pub fn f_010b42d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b42d7); } r_010b3e7b(cpu, 0x10b42d7); }
pub fn f_010b42eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b42eb); } r_010b3e7b(cpu, 0x10b42eb); }
pub fn f_010b4303(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4303); } r_010b3e7b(cpu, 0x10b4303); }
fn r_010b3e7b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b3e7b => {
  { let v: u32 = cpu.sub32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.poll(); pc = cpu.r32(0x117d038u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x10b3e85 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3e8b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b3e90; continue;
 }
 0x10b3e90 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3e96 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b3e9b; continue;
 }
 0x10b3e9b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3ea1 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3ea7 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3ead => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3eb3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3ec5); f_010b3d64(cpu); if cpu.returned_to != 0x10b3ec5 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3ecf => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b3ed4; continue;
 }
 0x10b3ed4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3ee0); f_010b3d64(cpu); if cpu.returned_to != 0x10b3ee0 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3eea => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b3eef; continue;
 }
 0x10b3eef => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f03); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f03 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3f0b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f17); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f17 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3f1b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f27); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f27 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3f2f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f3b); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f3b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3f3f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f53); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f53 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3f5f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b3f64; continue;
 }
 0x10b3f64 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f72); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f72 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3f7e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b3f83; continue;
 }
 0x10b3f83 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3f99); f_010b3d64(cpu); if cpu.returned_to != 0x10b3f99 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3fa3 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3fb1); f_010b3d64(cpu); if cpu.returned_to != 0x10b3fb1 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3fb7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3fc5); f_010b3d64(cpu); if cpu.returned_to != 0x10b3fc5 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3fcf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3fdd); f_010b3d64(cpu); if cpu.returned_to != 0x10b3fdd { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b3fe3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b3ff7); f_010b3d64(cpu); if cpu.returned_to != 0x10b3ff7 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4003 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b4008; continue;
 }
 0x10b4008 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4016); f_010b3d64(cpu); if cpu.returned_to != 0x10b4016 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4022 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b4027; continue;
 }
 0x10b4027 => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b403d); f_010b3d64(cpu); if cpu.returned_to != 0x10b403d { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4047 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4055); f_010b3d64(cpu); if cpu.returned_to != 0x10b4055 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b405b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4069); f_010b3d64(cpu); if cpu.returned_to != 0x10b4069 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4073 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4081); f_010b3d64(cpu); if cpu.returned_to != 0x10b4081 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4087 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b409b); f_010b3d64(cpu); if cpu.returned_to != 0x10b409b { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b40a7 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b40ac; continue;
 }
 0x10b40ac => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b40ba); f_010b3d64(cpu); if cpu.returned_to != 0x10b40ba { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b40c6 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b40cb; continue;
 }
 0x10b40cb => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b40e1); f_010b3d64(cpu); if cpu.returned_to != 0x10b40e1 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b40eb => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b40f9); f_010b3d64(cpu); if cpu.returned_to != 0x10b40f9 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b40ff => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b410d); f_010b3d64(cpu); if cpu.returned_to != 0x10b410d { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4117 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4125); f_010b3d64(cpu); if cpu.returned_to != 0x10b4125 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b412b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b413f); f_010b3d64(cpu); if cpu.returned_to != 0x10b413f { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b414b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b4150; continue;
 }
 0x10b4150 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b415e); f_010b3d64(cpu); if cpu.returned_to != 0x10b415e { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b416a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b416f; continue;
 }
 0x10b416f => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4185); f_010b3d64(cpu); if cpu.returned_to != 0x10b4185 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b418f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b419d); f_010b3d64(cpu); if cpu.returned_to != 0x10b419d { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b41a3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b41b1); f_010b3d64(cpu); if cpu.returned_to != 0x10b41b1 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b41bb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b41c9); f_010b3d64(cpu); if cpu.returned_to != 0x10b41c9 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b41cf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b41e3); f_010b3d64(cpu); if cpu.returned_to != 0x10b41e3 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b41ef => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b41f4; continue;
 }
 0x10b41f4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4202); f_010b3d64(cpu); if cpu.returned_to != 0x10b4202 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b420e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b4213; continue;
 }
 0x10b4213 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4229); f_010b3d64(cpu); if cpu.returned_to != 0x10b4229 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4233 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4241); f_010b3d64(cpu); if cpu.returned_to != 0x10b4241 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4247 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4255); f_010b3d64(cpu); if cpu.returned_to != 0x10b4255 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b425f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b426d); f_010b3d64(cpu); if cpu.returned_to != 0x10b426d { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4273 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4287); f_010b3d64(cpu); if cpu.returned_to != 0x10b4287 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4293 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b4298; continue;
 }
 0x10b4298 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b42a6); f_010b3d64(cpu); if cpu.returned_to != 0x10b42a6 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b42b2 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x10b42b7; continue;
 }
 0x10b42b7 => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b42cd); f_010b3d64(cpu); if cpu.returned_to != 0x10b42cd { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b42d7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b42e5); f_010b3d64(cpu); if cpu.returned_to != 0x10b42e5 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b42eb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b42f9); f_010b3d64(cpu); if cpu.returned_to != 0x10b42f9 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4303 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4311); f_010b3d64(cpu); if cpu.returned_to != 0x10b4311 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4317(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4317); }
 let mut pc: u32 = 0x10b4317;
 loop { match pc {
 0x10b4317 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4326); f_010b3d64(cpu); if cpu.returned_to != 0x10b4326 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b432a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b432a); }
 let mut pc: u32 = 0x10b432a;
 loop { match pc {
 0x10b432a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x10b4339); f_010b3d64(cpu); if cpu.returned_to != 0x10b4339 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b439c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b439c); }
 let mut pc: u32 = 0x10b439c;
 loop { match pc {
 0x10b439c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x10b43e0; continue; }
  pc = 0x10b43ad; continue;
 }
 0x10b43ad => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b43c3; continue; }
  pc = 0x10b43b6; continue;
 }
 0x10b43b6 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10b43bf); f_010b4317(cpu); if cpu.returned_to != 0x10b43bf { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x10b43c3 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x10b43d4); f_010b4317(cpu); if cpu.returned_to != 0x10b43d4 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x10b43e0 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b458a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b458a); }
 let mut pc: u32 = 0x10b458a;
 loop { match pc {
 0x10b458a => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
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
pub fn f_010b45a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b45a0); }
 let mut pc: u32 = 0x10b45a0;
 loop { match pc {
 0x10b45a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x117bb30u32, v); }
  cpu.push(0x10b45bb); f_010b546b(cpu); if cpu.returned_to != 0x10b45bb { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b45cb; continue; }
  pc = 0x10b45c6; continue;
 }
 0x10b45c6 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b45dc; continue; }
  pc = 0x10b45cb; continue;
 }
 0x10b45cb => {
  { let v: u32 = 0x3u32; cpu.w32(0x117bb30u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b45dc => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b45e4); f_010b558f(cpu); if cpu.returned_to != 0x10b45e4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.push(0x10b45f1); f_010b5565(cpu); if cpu.returned_to != 0x10b45f1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b45fd; continue; }
  pc = 0x10b45f8; continue;
 }
 0x10b45f8 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b460b; continue; }
  pc = 0x10b45fd; continue;
 }
 0x10b45fd => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b4606); f_010b4990(cpu); if cpu.returned_to != 0x10b4606 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b460d; continue;
 }
 0x10b460b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x10b460d; continue;
 }
 0x10b460d => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b4630; continue; }
  pc = 0x10b4615; continue;
 }
 0x10b4615 => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x117bb30u32, v); }
  cpu.push(0x10b4626); f_010b5595(cpu); if cpu.returned_to != 0x10b4626 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4630 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b463a; continue; }
  pc = 0x10b4635; continue;
 }
 0x10b4635 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b463a; continue;
 }
 0x10b463a => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b4642); f_010b54e1(cpu); if cpu.returned_to != 0x10b4642 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10b4683; continue; }
  pc = 0x10b4649; continue;
 }
 0x10b4649 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x10b4668; continue; }
  pc = 0x10b464d; continue;
 }
 0x10b464d => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b4668; continue; }
  pc = 0x10b4651; continue;
 }
 0x10b4651 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b465b); f_010b4710(cpu); if cpu.returned_to != 0x10b465b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x117bb38u32); cpu.push(0x10b4668); cpu.call(t); if cpu.returned_to != 0x10b4668 { return; }
  pc = 0x10b4668; continue;
 }
 0x10b4668 => {
  { let v: u32 = 0x5u32; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x117bb30u32, v); }
  cpu.push(0x10b4679); f_010b5595(cpu); if cpu.returned_to != 0x10b4679 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b4683 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b4689); f_010b5595(cpu); if cpu.returned_to != 0x10b4689 { return; }
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
pub fn f_010b46a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b46a0); }
 let mut pc: u32 = 0x10b46a0;
 loop { match pc {
 0x10b46a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x117bb30u32, v); }
  cpu.push(0x10b46b5); f_010b546b(cpu); if cpu.returned_to != 0x10b46b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b46c3; continue; }
  pc = 0x10b46be; continue;
 }
 0x10b46be => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b46d6; continue; }
  pc = 0x10b46c3; continue;
 }
 0x10b46c3 => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117bb30u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b46d6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b46de); f_010b558f(cpu); if cpu.returned_to != 0x10b46de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b46f2; continue; }
  pc = 0x10b46e8; continue;
 }
 0x10b46e8 => {
  { let v: u32 = 0x5u32; cpu.w32(0x117bb30u32, v); }
  pc = 0x10b46f2; continue;
 }
 0x10b46f2 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b46f8); f_010b5595(cpu); if cpu.returned_to != 0x10b46f8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4710); }
 let mut pc: u32 = 0x10b4710;
 loop { match pc {
 0x10b4710 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b4722); f_010b4750(cpu); if cpu.returned_to != 0x10b4722 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4730); }
 let mut pc: u32 = 0x10b4730;
 loop { match pc {
 0x10b4730 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b4742); f_010b47f0(cpu); if cpu.returned_to != 0x10b4742 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4750); }
 let mut pc: u32 = 0x10b4750;
 loop { match pc {
 0x10b4750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if !cpu.cf { pc = 0x10b4767; continue; }
  pc = 0x10b4763; continue;
 }
 0x10b4763 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x10b4769; continue;
 }
 0x10b4767 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x10b4769; continue;
 }
 0x10b4769 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b476d; continue; }
  pc = 0x10b476b; continue;
 }
 0x10b476b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10b476d; continue;
 }
 0x10b476d => {
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
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10b47d5); cpu.call(t); if cpu.returned_to != 0x10b47d5 { return; }
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
pub fn f_010b47f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b47f0); }
 let mut pc: u32 = 0x10b47f0;
 loop { match pc {
 0x10b47f0 => {
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
  if !cpu.cf { pc = 0x10b4829; continue; }
  pc = 0x10b4825; continue;
 }
 0x10b4825 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x10b482b; continue;
 }
 0x10b4829 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b482b; continue;
 }
 0x10b482b => {
  if (cpu.cf || cpu.zf) { pc = 0x10b4831; continue; }
  pc = 0x10b482d; continue;
 }
 0x10b482d => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x10b4833; continue;
 }
 0x10b4831 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  pc = 0x10b4833; continue;
 }
 0x10b4833 => {
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
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10b4876); cpu.call(t); if cpu.returned_to != 0x10b4876 { return; }
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
pub fn f_010b4890(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4890); }
 let mut pc: u32 = 0x10b4890;
 loop { match pc {
 0x10b4890 => {
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
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10b48bc); cpu.call(t); if cpu.returned_to != 0x10b48bc { return; }
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.esp)), 0x0u32); }
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b48d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b48d0); }
 let mut pc: u32 = 0x10b48d0;
 loop { match pc {
 0x10b48d0 => {
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
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10b48f7); cpu.call(t); if cpu.returned_to != 0x10b48f7 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10b4902; continue; }
  pc = 0x10b48fe; continue;
 }
 0x10b48fe => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10b4935; continue;
 }
 0x10b4902 => {
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
  pc = 0x10b4935; continue;
 }
 0x10b4935 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b4956; continue; }
  pc = 0x10b4939; continue;
 }
 0x10b4939 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b4953); f_010b47f0(cpu); if cpu.returned_to != 0x10b4953 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10b4956; continue;
 }
 0x10b4956 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4960); }
 let mut pc: u32 = 0x10b4960;
 loop { match pc {
 0x10b4960 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b4983; continue; }
  pc = 0x10b4969; continue;
 }
 0x10b4969 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b4976); f_010b4750(cpu); if cpu.returned_to != 0x10b4976 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x117bb38u32); cpu.push(0x10b4983); cpu.call(t); if cpu.returned_to != 0x10b4983 { return; }
  pc = 0x10b4983; continue;
 }
 0x10b4983 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4990); }
 let mut pc: u32 = 0x10b4990;
 loop { match pc {
 0x10b4990 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x117bb34u32); cpu.push(0x10b49a0); cpu.call(t); if cpu.returned_to != 0x10b49a0 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b49b8; continue; }
  pc = 0x10b49a6; continue;
 }
 0x10b49a6 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b49b0); f_010b47f0(cpu); if cpu.returned_to != 0x10b49b0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b49b8; continue;
 }
 0x10b49b8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b49c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b49c0); }
 let mut pc: u32 = 0x10b49c0;
 loop { match pc {
 0x10b49c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x117bb38u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117bb38u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b49ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b49ee); }
 let mut pc: u32 = 0x10b49ee;
 loop { match pc {
 0x10b49ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117bcd8u32), 0x0u32); }
  if !cpu.zf { pc = 0x10b4a02; continue; }
  pc = 0x10b49f8; continue;
 }
 0x10b49f8 => {
  { cpu.sub16(cpu.r16(0x117bcdcu32), 0x0u16); }
  if cpu.zf { pc = 0x10b4a0f; continue; }
  pc = 0x10b4a02; continue;
 }
 0x10b4a02 => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x117bcd8u32); cpu.push(cpu.segments[1] as u32); cpu.push(0x10b4a0d); cpu.call(t); if cpu.returned_to != 0x10b4a0d { return; }
  pc = 0x10b4a70; continue;
 }
 0x10b4a0f => {
  { let v: u32 = cpu.r32(0x117bcc8u32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  pc = 0x10b4a1b; continue;
 }
 0x10b4a1b => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b4a21; continue;
 }
 0x10b4a21 => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub32(cpu.ecx, cpu.r32(0x117bcc8u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x10b4a0f; continue; }
  pc = 0x10b4a2d; continue;
 }
 0x10b4a2d => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b4a58; continue; }
  pc = 0x10b4a34; continue;
 }
 0x10b4a34 => {
  { let v: u32 = cpu.r32(0x117bceau32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x10b4a44; continue; }
  pc = 0x10b4a3f; continue;
 }
 0x10b4a3f => {
  { let v: u32 = 0x10000u32; cpu.ebx = v; }
  pc = 0x10b4a44; continue;
 }
 0x10b4a44 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.cf { pc = 0x10b4a4a; continue; }
  pc = 0x10b4a48; continue;
 }
 0x10b4a48 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10b4a4a; continue;
 }
 0x10b4a4a => {
  { let v: u32 = 0x3e8u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = 0x1234ddu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x10b4a58; continue;
 }
 0x10b4a58 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117bceeu32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x117bcd0u32)); cpu.eax = v; }
  pc = 0x10b4a70; continue;
 }
 0x10b4a70 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x117bcccu32)); }
  if !cpu.cf { pc = 0x10b4a7d; continue; }
  pc = 0x10b4a78; continue;
 }
 0x10b4a78 => {
  { let v: u32 = cpu.r32(0x117bcccu32); cpu.eax = v; }
  pc = 0x10b4a7d; continue;
 }
 0x10b4a7d => {
  { let v: u32 = cpu.eax; cpu.w32(0x117bcccu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4a84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4a84); }
 let mut pc: u32 = 0x10b4a84;
 loop { match pc {
 0x10b4a84 => {
  { cpu.sub32(cpu.r32(0x117bcccu32), 0x0u32); }
  if cpu.zf { pc = 0x10b4aa1; continue; }
  pc = 0x10b4a8d; continue;
 }
 0x10b4a8d => {
  cpu.push(0x10b4a92); f_010b49ee(cpu); if cpu.returned_to != 0x10b4a92 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x117bcc8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x117bcd0u32, v); }
  pc = 0x10b4aa1; continue;
 }
 0x10b4aa1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4aa2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4aa2); }
 let mut pc: u32 = 0x10b4aa2;
 loop { match pc {
 0x10b4aa2 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x117bcfau32); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x117bcc8u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x117bcc8u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.r32(0x117bcf2u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b4c3d; continue; }
  pc = 0x10b4abe; continue;
 }
 0x10b4abe => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  { let v = cpu.r16(0x117bcfau32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x117bcf2u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x117bcf2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x117c70cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x117c710u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x117c70cu32; cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x117bcd8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x10b4afe; continue; }
  pc = 0x10b4af4; continue;
 }
 0x10b4af4 => {
  { cpu.sub16(cpu.r16(0x117bcdcu32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x10b4b24; continue; }
  pc = 0x10b4afe; continue;
 }
 0x10b4afe => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x117bcd8u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10b4b03); cpu.push(cpu.segments[1] as u32); cpu.push(0x10b4b09); cpu.call(t); if cpu.returned_to != 0x10b4b09 { return; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x117bcd4u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x10b4b45; continue; }
  pc = 0x10b4b17; continue;
 }
 0x10b4b17 => {
  { let v: u32 = cpu.r32(0x117bceeu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117bcd4u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x10b4b65; continue;
 }
 0x10b4b24 => {
  { let v: u32 = cpu.r32(0x117bcc8u32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x117bcd4u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x10b4b3f; continue; }
  pc = 0x10b4b32; continue;
 }
 0x10b4b32 => {
  { let v: u32 = cpu.r32(0x117bceeu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117bcd4u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x10b4b65; continue;
 }
 0x10b4b3f => {
  { let v: u32 = cpu.r32(0x117bceeu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  pc = 0x10b4b45; continue;
 }
 0x10b4b45 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x117bcd4u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x117bcd4u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xf4240u32); }
  if cpu.cf { pc = 0x10b4b65; continue; }
  pc = 0x10b4b60; continue;
 }
 0x10b4b60 => {
  { let v: u32 = 0xf4240u32; cpu.edx = v; }
  pc = 0x10b4b65; continue;
 }
 0x10b4b65 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x10b4b6a; continue;
 }
 0x10b4b6a => {
  { cpu.sub32(cpu.r32(0x117bb88u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if !cpu.zf { pc = 0x10b4b97; continue; }
  pc = 0x10b4b73; continue;
 }
 0x10b4b73 => {
  { let v: u32 = cpu.r32(0x117bbc8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  pc = 0x10b4b7b; continue;
 }
 0x10b4b7b => {
  { cpu.sub32(cpu.eax, cpu.r32(0x117bc08u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if cpu.cf { pc = 0x10b4b91; continue; }
  pc = 0x10b4b83; continue;
 }
 0x10b4b83 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x117bc08u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x117bc48u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x117bc48u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10b4b7b; continue;
 }
 0x10b4b91 => {
  { let v: u32 = cpu.eax; cpu.w32(0x117bbc8u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x10b4b97; continue;
 }
 0x10b4b97 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x10b4b6a; continue; }
  pc = 0x10b4b9f; continue;
 }
 0x10b4b9f => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  {  }
  { cpu.sub32(cpu.r32(0x117bcf6u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b4bd8; continue; }
  pc = 0x10b4bad; continue;
 }
 0x10b4bad => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x10b4bb2; continue;
 }
 0x10b4bb2 => {
  { cpu.sub32(cpu.r32(0x117bc48u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x10b4bd0; continue; }
  pc = 0x10b4bbb; continue;
 }
 0x10b4bbb => {
  { let v: u32 = cpu.dec32(cpu.r32(0x117bc48u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x117bc48u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x117bc88u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb48u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10b4bc8); cpu.push(0x10b4bce); cpu.call(t); if cpu.returned_to != 0x10b4bce { return; }
  cpu.poll(); pc = 0x10b4bb2; continue;
 }
 0x10b4bd0 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x10b4bb2; continue; }
  pc = 0x10b4bd8; continue;
 }
 0x10b4bd8 => {
  { cpu.sub32(cpu.r32(0x117bc84u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x10b4c21; continue; }
  pc = 0x10b4be1; continue;
 }
 0x10b4be1 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x117bc84u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x117bc84u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x117c70cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x117c710u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x117bcf2u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x117bcf2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x117bcdeu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x117bce2u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
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
 0x10b4c21 => {
  { let v = cpu.r16(0x117c70cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x117c710u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x117bcf2u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x117bcf2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10b4c3c); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x10b4c3d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10b4c44); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4c45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4c45); }
 let mut pc: u32 = 0x10b4c45;
 loop { match pc {
 0x10b4c45 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v: u8 = 0x34u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117bceau32, v); }
  pc = 0x10b4c58; continue;
 }
 0x10b4c58 => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b4c5e; continue;
 }
 0x10b4c5e => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x10b4c6b; continue; }
  pc = 0x10b4c6a; continue;
 }
 0x10b4c6a => {
  {  }
  pc = 0x10b4c6b; continue;
 }
 0x10b4c6b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4c6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4c6f); }
 let mut pc: u32 = 0x10b4c6f;
 loop { match pc {
 0x10b4c6f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xd68du32); }
  if !cpu.cf { pc = 0x10b4c92; continue; }
  pc = 0x10b4c81; continue;
 }
 0x10b4c81 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20bcu32; cpu.ebx = v; }
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x10b4c92; continue;
 }
 0x10b4c92 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b4c98); f_010b4c45(cpu); if cpu.returned_to != 0x10b4c98 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4c9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4c9e); }
 let mut pc: u32 = 0x10b4c9e;
 loop { match pc {
 0x10b4c9e => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x10b4ca0); cpu.push(0x10b4ca5); f_010ae5d0(cpu); if cpu.returned_to != 0x10b4ca5 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x10b4cb2; continue;
 }
 0x10b4cb2 => {
  { cpu.sub32(cpu.r32(0x117bb88u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x10b4cc7; continue; }
  pc = 0x10b4cbb; continue;
 }
 0x10b4cbb => {
  { let v: u32 = cpu.r32(0x117bc08u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x10b4cc7; continue; }
  pc = 0x10b4cc5; continue;
 }
 0x10b4cc5 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x10b4cc7; continue;
 }
 0x10b4cc7 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x10b4cb2; continue; }
  pc = 0x10b4ccf; continue;
 }
 0x10b4ccf => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x117bceeu32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x10b4cfc; continue; }
  pc = 0x10b4cd7; continue;
 }
 0x10b4cd7 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x117bceeu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x10b4cde); cpu.push(0x10b4ce3); f_010b4c6f(cpu); if cpu.returned_to != 0x10b4ce3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x117bbc8u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.flat_ds(0x10b4cf7); cpu.push(0x10b4cfc); f_010b4a84(cpu); if cpu.returned_to != 0x10b4cfc { return; }
  pc = 0x10b4cfc; continue;
 }
 0x10b4cfc => {
  cpu.flat_ds(0x10b4cfc); cpu.push(0x10b4d01); f_010ae5c0(cpu); if cpu.returned_to != 0x10b4d01 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.flat_ds(0x10b4d03); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4e1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4e1a); }
 let mut pc: u32 = 0x10b4e1a;
 loop { match pc {
 0x10b4e1a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x10b4e1f); cpu.push(0x10b4e24); f_010b4c45(cpu); if cpu.returned_to != 0x10b4e24 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117bcdeu32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x117bce2u32.wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x10b4e4d; continue; }
  pc = 0x10b4e4c; continue;
 }
 0x10b4e4c => {
  {  }
  pc = 0x10b4e4d; continue;
 }
 0x10b4e4d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x10b4e50); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4e51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4e51); }
 let mut pc: u32 = 0x10b4e51;
 loop { match pc {
 0x10b4e51 => {
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
pub fn f_010b4e79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4e79); }
 let mut pc: u32 = 0x10b4e79;
 loop { match pc {
 0x10b4e79 => {
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
pub fn f_010b4e9c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4e9c); } r_010b4e9c(cpu, 0x10b4e9c); }
pub fn f_010b4ea3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4ea3); } r_010b4e9c(cpu, 0x10b4ea3); }
pub fn f_010b4eb2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4eb2); } r_010b4e9c(cpu, 0x10b4eb2); }
pub fn f_010b4eb8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4eb8); } r_010b4e9c(cpu, 0x10b4eb8); }
pub fn f_010b4ece(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4ece); } r_010b4e9c(cpu, 0x10b4ece); }
pub fn f_010b4ed8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4ed8); } r_010b4e9c(cpu, 0x10b4ed8); }
pub fn f_010b4edf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b4edf); } r_010b4e9c(cpu, 0x10b4edf); }
fn r_010b4e9c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b4e9c => {
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
 0x10b4ea3 => {
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
 0x10b4eb2 => {
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
 0x10b4eb8 => {
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
 0x10b4ece => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x10b4ed8 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x10b4edf => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4ee6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4ee6); }
 let mut pc: u32 = 0x10b4ee6;
 loop { match pc {
 0x10b4ee6 => {
  { let v = (cpu.ebx as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.ecx; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x10b4ef0); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b4ef1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b4ef1); }
 let mut pc: u32 = 0x10b4ef1;
 loop { match pc {
 0x10b4ef1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffe4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.sub32(cpu.r32(0x117bcfeu32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b5101; continue; }
  pc = 0x10b4f08; continue;
 }
 0x10b4f08 => {
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
  cpu.unimplemented(0x10b4f62, "Lar");
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
  cpu.unimplemented(0x10b4fb3, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x117bd0cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x117bd0cu32; cpu.eax = v; }
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
  cpu.unimplemented(0x10b5016, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x10b4e9cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x10b4e9cu32; cpu.eax = v; }
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
  cpu.unimplemented(0x10b5079, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x200u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10b4eb8u32, v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b4eb2u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b4ea3u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b4ed8u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b4edfu32, v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10b4eceu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117bcfeu32, v); }
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x10b50d6; continue; }
  pc = 0x10b50d3; continue;
 }
 0x10b50d3 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x10b50d6; continue;
 }
 0x10b50d6 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x204u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117bd02u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x117bd06u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  pc = 0x10b5101; continue;
 }
 0x10b5101 => {
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
pub fn f_010b5107(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5107); }
 let mut pc: u32 = 0x10b5107;
 loop { match pc {
 0x10b5107 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b5146; continue; }
  pc = 0x10b5116; continue;
 }
 0x10b5116 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x117bcfeu32)); }
  if !cpu.zf { pc = 0x10b5146; continue; }
  pc = 0x10b511e; continue;
 }
 0x10b511e => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x10b5126; continue; }
  pc = 0x10b5123; continue;
 }
 0x10b5123 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x10b5126; continue;
 }
 0x10b5126 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117bd02u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x117bd06u32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x117bcfeu32, v); }
  pc = 0x10b5146; continue;
 }
 0x10b5146 => {
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
pub fn f_010b5150(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5150); }
 let mut pc: u32 = 0x10b5150;
 loop { match pc {
 0x10b5150 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x10b5161; continue; }
  pc = 0x10b5160; continue;
 }
 0x10b5160 => {
  {  }
  pc = 0x10b5161; continue;
 }
 0x10b5161 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5165(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5165); }
 let mut pc: u32 = 0x10b5165;
 loop { match pc {
 0x10b5165 => {
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
pub fn f_010b51b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b51b7); }
 let mut pc: u32 = 0x10b51b7;
 loop { match pc {
 0x10b51b7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffc4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x10b51c2); cpu.push(0x10b51c7); f_010ae5d0(cpu); if cpu.returned_to != 0x10b51c7 { return; }
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
  if cpu.zf { pc = 0x10b5226; continue; }
  pc = 0x10b5206; continue;
 }
 0x10b5206 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b5226; continue;
 }
 0x10b5226 => {
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
  if cpu.zf { pc = 0x10b527f; continue; }
  pc = 0x10b5250; continue;
 }
 0x10b5250 => {
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
  pc = 0x10b527f; continue;
 }
 0x10b527f => {
  { let v: u32 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x10b5284); cpu.push(0x10b5289); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5289 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x10b5290); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5291(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5291); }
 let mut pc: u32 = 0x10b5291;
 loop { match pc {
 0x10b5291 => {
  { let v: u32 = cpu.r32(0x117bcf2u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5297(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5297); }
 let mut pc: u32 = 0x10b5297;
 loop { match pc {
 0x10b5297 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b529f); f_010ae5d0(cpu); if cpu.returned_to != 0x10b529f { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x10b52a4; continue;
 }
 0x10b52a4 => {
  { cpu.sub32(cpu.r32(0x117bb88u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x10b52bc; continue; }
  pc = 0x10b52ad; continue;
 }
 0x10b52ad => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x10b52a4; continue; }
  pc = 0x10b52b5; continue;
 }
 0x10b52b5 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b52cf; continue;
 }
 0x10b52bc => {
  { let v: u32 = 0x1u32; cpu.w32(0x117bb88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117bb48u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b52cf; continue;
 }
 0x10b52cf => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b52d5); f_010ae5c0(cpu); if cpu.returned_to != 0x10b52d5 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b52d8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b52d8); }
 let mut pc: u32 = 0x10b52d8;
 loop { match pc {
 0x10b52d8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b52e0); f_010ae5d0(cpu); if cpu.returned_to != 0x10b52e0 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b52f1; continue; }
  pc = 0x10b52e8; continue;
 }
 0x10b52e8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let a = cpu.r32(0x117bc88u32.wrapping_add(cpu.ecx)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x117bc88u32.wrapping_add(cpu.ecx), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x10b52f1; continue;
 }
 0x10b52f1 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b52f7); f_010ae5c0(cpu); if cpu.returned_to != 0x10b52f7 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b52fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b52fa); }
 let mut pc: u32 = 0x10b52fa;
 loop { match pc {
 0x10b52fa => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b5302); f_010ae5d0(cpu); if cpu.returned_to != 0x10b5302 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b5314; continue; }
  pc = 0x10b530a; continue;
 }
 0x10b530a => {
  { let v: u32 = 0x0u32; cpu.w32(0x117bb88u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b5314; continue;
 }
 0x10b5314 => {
  cpu.push(0x10b5319); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5319 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b531b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b531b); }
 let mut pc: u32 = 0x10b531b;
 loop { match pc {
 0x10b531b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b5321); f_010ae5d0(cpu); if cpu.returned_to != 0x10b5321 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x10b5326; continue;
 }
 0x10b5326 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b532c); f_010ad500(cpu); if cpu.returned_to != 0x10b532c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b5326; continue; }
  pc = 0x10b5334; continue;
 }
 0x10b5334 => {
  cpu.push(0x10b5339); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5339 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b533b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b533b); }
 let mut pc: u32 = 0x10b533b;
 loop { match pc {
 0x10b533b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b5343); f_010ae5d0(cpu); if cpu.returned_to != 0x10b5343 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b535e; continue; }
  pc = 0x10b534b; continue;
 }
 0x10b534b => {
  { cpu.sub32(cpu.r32(0x117bb88u32.wrapping_add(cpu.ecx)), 0x1u32); }
  if !cpu.zf { pc = 0x10b535e; continue; }
  pc = 0x10b5354; continue;
 }
 0x10b5354 => {
  { let v: u32 = 0x2u32; cpu.w32(0x117bb88u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b535e; continue;
 }
 0x10b535e => {
  cpu.push(0x10b5363); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5363 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5365(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5365); }
 let mut pc: u32 = 0x10b5365;
 loop { match pc {
 0x10b5365 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b536b); f_010ae5d0(cpu); if cpu.returned_to != 0x10b536b { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x10b5370; continue;
 }
 0x10b5370 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b5376); f_010ad760(cpu); if cpu.returned_to != 0x10b5376 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b5370; continue; }
  pc = 0x10b537e; continue;
 }
 0x10b537e => {
  cpu.push(0x10b5383); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5383 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5385(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5385); }
 let mut pc: u32 = 0x10b5385;
 loop { match pc {
 0x10b5385 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b538d); f_010ae5d0(cpu); if cpu.returned_to != 0x10b538d { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b53a8; continue; }
  pc = 0x10b5395; continue;
 }
 0x10b5395 => {
  { cpu.sub32(cpu.r32(0x117bb88u32.wrapping_add(cpu.ecx)), 0x2u32); }
  if !cpu.zf { pc = 0x10b53a8; continue; }
  pc = 0x10b539e; continue;
 }
 0x10b539e => {
  { let v: u32 = 0x1u32; cpu.w32(0x117bb88u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b53a8; continue;
 }
 0x10b53a8 => {
  cpu.push(0x10b53ad); f_010ae5c0(cpu); if cpu.returned_to != 0x10b53ad { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b53af(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b53af); }
 let mut pc: u32 = 0x10b53af;
 loop { match pc {
 0x10b53af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b53b5); f_010ae5d0(cpu); if cpu.returned_to != 0x10b53b5 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x10b53ba; continue;
 }
 0x10b53ba => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b53c0); f_010ad630(cpu); if cpu.returned_to != 0x10b53c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b53ba; continue; }
  pc = 0x10b53c8; continue;
 }
 0x10b53c8 => {
  cpu.push(0x10b53cd); f_010ae5c0(cpu); if cpu.returned_to != 0x10b53cd { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b53cf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b53cf); }
 let mut pc: u32 = 0x10b53cf;
 loop { match pc {
 0x10b53cf => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b53da); f_010ae5d0(cpu); if cpu.returned_to != 0x10b53da { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117bc08u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x117bbc8u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x10b53f5); f_010b4c9e(cpu); if cpu.returned_to != 0x10b53f5 { return; }
  cpu.push(0x10b53fa); f_010ae5c0(cpu); if cpu.returned_to != 0x10b53fa { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b53ff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b53ff); }
 let mut pc: u32 = 0x10b53ff;
 loop { match pc {
 0x10b53ff => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b5407); f_010ae5d0(cpu); if cpu.returned_to != 0x10b5407 { return; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x10b541f); f_010ada70(cpu); if cpu.returned_to != 0x10b541f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x10b5427); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5427 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5429(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5429); }
 let mut pc: u32 = 0x10b5429;
 loop { match pc {
 0x10b5429 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x10b5431); f_010ae5d0(cpu); if cpu.returned_to != 0x10b5431 { return; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b543e; continue; }
  pc = 0x10b5437; continue;
 }
 0x10b5437 => {
  { let v: u32 = 0xd68du32; cpu.eax = v; }
  pc = 0x10b5452; continue;
 }
 0x10b543e => {
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x2e9cu32; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v = cpu.ecx; cpu.div32(v); }
  pc = 0x10b5452; continue;
 }
 0x10b5452 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x10b545b); f_010ada70(cpu); if cpu.returned_to != 0x10b545b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x10b5463); f_010ae5c0(cpu); if cpu.returned_to != 0x10b5463 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5465(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5465); }
 let mut pc: u32 = 0x10b5465;
 loop { match pc {
 0x10b5465 => {
  { let v: u32 = cpu.r32(0x117bceau32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b546b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b546b); }
 let mut pc: u32 = 0x10b546b;
 loop { match pc {
 0x10b546b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x10b5480; continue; }
  pc = 0x10b547c; continue;
 }
 0x10b547c => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10b5480; continue;
 }
 0x10b5480 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5487(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5487); }
 let mut pc: u32 = 0x10b5487;
 loop { match pc {
 0x10b5487 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x10b549b; continue; }
  pc = 0x10b5497; continue;
 }
 0x10b5497 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10b549b; continue;
 }
 0x10b549b => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b54a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b54a2); }
 let mut pc: u32 = 0x10b54a2;
 loop { match pc {
 0x10b54a2 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x10b54cd; continue; }
  pc = 0x10b54b3; continue;
 }
 0x10b54b3 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x10b54c0; continue; }
  pc = 0x10b54b9; continue;
 }
 0x10b54b9 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10b54dc; continue;
 }
 0x10b54c0 => {
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { cpu.poll(); pc = 0x10b54b9; continue; }
  pc = 0x10b54c8; continue;
 }
 0x10b54c8 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  pc = 0x10b54dc; continue;
 }
 0x10b54cd => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10b54dc; continue;
 }
 0x10b54dc => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b54e1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b54e1); }
 let mut pc: u32 = 0x10b54e1;
 loop { match pc {
 0x10b54e1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x10b54ea; continue;
 }
 0x10b54ea => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x10b54fc; continue; }
  pc = 0x10b54f7; continue;
 }
 0x10b54f7 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x10b54fc; continue;
 }
 0x10b54fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x10b5519; continue; }
  pc = 0x10b5507; continue;
 }
 0x10b5507 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b5519; continue; }
  pc = 0x10b550e; continue;
 }
 0x10b550e => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x10b5519; continue; }
  pc = 0x10b5514; continue;
 }
 0x10b5514 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x10b54ea; continue; }
  pc = 0x10b5519; continue;
 }
 0x10b5519 => {
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
pub fn f_010b5523(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5523); }
 let mut pc: u32 = 0x10b5523;
 loop { match pc {
 0x10b5523 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x10b552c; continue;
 }
 0x10b552c => {
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x10b553e; continue; }
  pc = 0x10b5539; continue;
 }
 0x10b5539 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x10b553e; continue;
 }
 0x10b553e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x10b555b; continue; }
  pc = 0x10b5549; continue;
 }
 0x10b5549 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x10b555b; continue; }
  pc = 0x10b5550; continue;
 }
 0x10b5550 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x10b555b; continue; }
  pc = 0x10b5556; continue;
 }
 0x10b5556 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x10b552c; continue; }
  pc = 0x10b555b; continue;
 }
 0x10b555b => {
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
pub fn f_010b5565(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b5565); } r_010b5565(cpu, 0x10b5565); }
pub fn f_010b558f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b558f); } r_010b5565(cpu, 0x10b558f); }
fn r_010b5565(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b5565 => {
  { let v: u16 = 0x4200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x10b5569; continue;
 }
 0x10b5569 => {
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
 0x10b558f => {
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x10b5569; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5595(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5595); }
 let mut pc: u32 = 0x10b5595;
 loop { match pc {
 0x10b5595 => {
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
pub fn f_010b55a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b55a3); }
 let mut pc: u32 = 0x10b55a3;
 loop { match pc {
 0x10b55a3 => {
  { let v = 0x117c714u32; cpu.push(v); }
  { let v = 0x117bb48u32; cpu.push(v); }
  cpu.push(0x10b55b2); f_010b47f0(cpu); if cpu.returned_to != 0x10b55b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x10b55a3u32; cpu.push(v); }
  { let v = 0x10b49e0u32; cpu.push(v); }
  cpu.push(0x10b55c4); f_010b47f0(cpu); if cpu.returned_to != 0x10b55c4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b55d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b55d0); }
 let mut pc: u32 = 0x10b55d0;
 loop { match pc {
 0x10b55d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c818u32), 0x0u32); }
  if cpu.zf { pc = 0x10b5653; continue; }
  pc = 0x10b55dc; continue;
 }
 0x10b55dc => {
  { let v = 0x10b55d0u32; cpu.push(v); }
  { let v = 0x10b62c0u32; cpu.push(v); }
  cpu.push(0x10b55eb); f_010b4750(cpu); if cpu.returned_to != 0x10b55eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x12d1fe0u32; cpu.push(v); }
  cpu.push(0x10b55fa); f_010b4710(cpu); if cpu.returned_to != 0x10b55fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x12d2120u32; cpu.push(v); }
  cpu.push(0x10b5609); f_010b4710(cpu); if cpu.returned_to != 0x10b5609 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x12d2020u32; cpu.push(v); }
  cpu.push(0x10b561b); f_010b4710(cpu); if cpu.returned_to != 0x10b561b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x12d2170u32; cpu.push(v); }
  cpu.push(0x10b562a); f_010b4710(cpu); if cpu.returned_to != 0x10b562a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c814u32; cpu.push(v); }
  cpu.push(0x10b5639); f_010b4710(cpu); if cpu.returned_to != 0x10b5639 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d2188u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10b564a); f_010b4710(cpu); if cpu.returned_to != 0x10b564a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c818u32, v); }
  pc = 0x10b5653; continue;
 }
 0x10b5653 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5660); }
 let mut pc: u32 = 0x10b5660;
 loop { match pc {
 0x10b5660 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  cpu.push(0x10b566d); f_010b49ee(cpu); if cpu.returned_to != 0x10b566d { return; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x10b566f; continue;
 }
 0x10b566f => {
  cpu.push(0x10b5674); f_010b49ee(cpu); if cpu.returned_to != 0x10b5674 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x10b566f; continue; }
  pc = 0x10b5678; continue;
 }
 0x10b5678 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5680); }
 let mut pc: u32 = 0x10b5680;
 loop { match pc {
 0x10b5680 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10b5695; continue; }
  pc = 0x10b568b; continue;
 }
 0x10b568b => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x117c714u32, v); }
  pc = 0x10b56db; continue;
 }
 0x10b5695 => {
  { let v: u32 = 0x117c714u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b569c; continue;
 }
 0x10b569c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b569c; continue; }
  pc = 0x10b56a6; continue;
 }
 0x10b56a6 => {
  { let v: u32 = 0x117c714u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b56db; continue; }
  pc = 0x10b56bb; continue;
 }
 0x10b56bb => {
  { let v: u8 = cpu.r8(0x117c713u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if cpu.zf { pc = 0x10b56db; continue; }
  pc = 0x10b56c6; continue;
 }
 0x10b56c6 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3au8); }
  if cpu.zf { pc = 0x10b56db; continue; }
  pc = 0x10b56cb; continue;
 }
 0x10b56cb => {
  { let v: u8 = 0x5cu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x117c714u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x117c715u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b56db; continue;
 }
 0x10b56db => {
  { let v: u32 = 0x117c714u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b56f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b56f0); }
 let mut pc: u32 = 0x10b56f0;
 loop { match pc {
 0x10b56f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  pc = 0x10b56f6; continue;
 }
 0x10b56f6 => {
  { let v: u32 = cpu.r32(0x12d1fe0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b5709; continue; }
  pc = 0x10b5700; continue;
 }
 0x10b5700 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b5706); f_010ad1c0(cpu); if cpu.returned_to != 0x10b5706 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b5709; continue;
 }
 0x10b5709 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xfffffffcu32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b56f6; continue; }
  pc = 0x10b5711; continue;
 }
 0x10b5711 => {
  cpu.push(0x10b5716); f_010ad480(cpu); if cpu.returned_to != 0x10b5716 { return; }
  cpu.push(0x10b571b); f_010b4e1a(cpu); if cpu.returned_to != 0x10b571b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5720); }
 let mut pc: u32 = 0x10b5720;
 loop { match pc {
 0x10b4d04 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x117bcfau32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x117bcfau32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x10b4d1a); cpu.push(0x10b4d1f); f_010b55a3(cpu); if cpu.returned_to != 0x10b4d1f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x117bcf2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x117bcf6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x117bceeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x117bcfeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x117bb88u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x117bbc8u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x117bc08u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x117bc48u32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = 0x1684u32; cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = (cpu.edi as u16); cpu.set_segment(0, v as u16); }
  { cpu.interrupt(0x2f); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x117bcd8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x117bcdau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x117bcdcu32.wrapping_add(cpu.bases[3]), v); }
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
  { let v: u32 = cpu.ebx; cpu.w32(0x117bcdeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x117bce2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x117bce6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x10b4aa2u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x2u32; cpu.w32(0x117bbc4u32.wrapping_add(cpu.bases[3]), v); }
  { let v = 0xd68du32; cpu.push(v); }
  { let v = 0x3cu32; cpu.push(v); }
  cpu.flat_ds(0x10b4e01); cpu.push(0x10b4e06); f_010ada70(cpu); if cpu.returned_to != 0x10b4e06 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x10b4e14; continue; }
  pc = 0x10b4e13; continue;
 }
 0x10b4e13 => {
  {  }
  pc = 0x10b4e14; continue;
 }
 0x10b4e14 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x10b4e19); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b5720 => {
  cpu.flat_ds(0x10b5720); cpu.push(0x10b5725); f_010b62c0(cpu); if cpu.returned_to != 0x10b5725 { return; }
  { let v = 0xc8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x10b572c); cpu.push(0x10b5731); f_010ae5e0(cpu); if cpu.returned_to != 0x10b5731 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.flat_ds(0x10b5738); cpu.push(0x10b573d); f_010ae5e0(cpu); if cpu.returned_to != 0x10b573d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.flat_ds(0x10b5747); cpu.push(0x10b574c); f_010ae5e0(cpu); if cpu.returned_to != 0x10b574c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.flat_ds(0x10b5753); cpu.push(0x10b5758); f_010ae5e0(cpu); if cpu.returned_to != 0x10b5758 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.flat_ds(0x10b575f); cpu.push(0x10b5764); f_010ae5e0(cpu); if cpu.returned_to != 0x10b5764 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  cpu.flat_ds(0x10b576b); cpu.push(0x10b5770); f_010ae5e0(cpu); if cpu.returned_to != 0x10b5770 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x28fu32; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  cpu.flat_ds(0x10b577a); cpu.push(0x10b577f); f_010ae5e0(cpu); if cpu.returned_to != 0x10b577f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  cpu.flat_ds(0x10b5786); cpu.push(0x10b578b); f_010ae5e0(cpu); if cpu.returned_to != 0x10b578b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.flat_ds(0x10b5792); cpu.push(0x10b5797); f_010ae5e0(cpu); if cpu.returned_to != 0x10b5797 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  cpu.flat_ds(0x10b579e); cpu.push(0x10b57a3); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57a3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  cpu.flat_ds(0x10b57aa); cpu.push(0x10b57af); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x78u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.flat_ds(0x10b57b6); cpu.push(0x10b57bb); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.flat_ds(0x10b57c2); cpu.push(0x10b57c7); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xdu32; cpu.push(v); }
  cpu.flat_ds(0x10b57ce); cpu.push(0x10b57d3); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57d3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xeu32; cpu.push(v); }
  cpu.flat_ds(0x10b57da); cpu.push(0x10b57df); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57df { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  cpu.flat_ds(0x10b57e6); cpu.push(0x10b57eb); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.flat_ds(0x10b57f2); cpu.push(0x10b57f7); f_010ae5e0(cpu); if cpu.returned_to != 0x10b57f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.flat_ds(0x10b57fe); cpu.push(0x10b5803); f_010ae5e0(cpu); if cpu.returned_to != 0x10b5803 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.flat_ds(0x10b580a); cpu.push(0x10b580f); f_010ae5e0(cpu); if cpu.returned_to != 0x10b580f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x13u32; cpu.push(v); }
  cpu.flat_ds(0x10b5816); cpu.push(0x10b581b); f_010ae5e0(cpu); if cpu.returned_to != 0x10b581b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b5820; continue;
 }
 0x10b5820 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d1fdcu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5820; continue; }
  pc = 0x10b5830; continue;
 }
 0x10b5830 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x12d2020u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10b4d04; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5840); }
 let mut pc: u32 = 0x10b5840;
 loop { match pc {
 0x10b5840 => {
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
  if cpu.zf { pc = 0x10b58de; continue; }
  pc = 0x10b5864; continue;
 }
 0x10b5864 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb3cu32); cpu.push(0x10b586b); cpu.call(t); if cpu.returned_to != 0x10b586b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b58de; continue; }
  pc = 0x10b5871; continue;
 }
 0x10b5871 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b588b; continue; }
  pc = 0x10b5886; continue;
 }
 0x10b5886 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x10b588b; continue;
 }
 0x10b588b => {
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
  cpu.push(0x10b58b8); f_010ae130(cpu); if cpu.returned_to != 0x10b58b8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b58c7; continue; }
  pc = 0x10b58c0; continue;
 }
 0x10b58c0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b58c7 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x12d218cu32; cpu.edi = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x12d218cu32; cpu.eax = v; }
  { cpu.movs(4, true); }
  pc = 0x10b58de; continue;
 }
 0x10b58de => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b58f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b58f0); }
 let mut pc: u32 = 0x10b58f0;
 loop { match pc {
 0x10b58f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b5908; continue; }
  pc = 0x10b58fe; continue;
 }
 0x10b58fe => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x10b5905); cpu.call(t); if cpu.returned_to != 0x10b5905 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b5908; continue;
 }
 0x10b5908 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b5919; continue; }
  pc = 0x10b5910; continue;
 }
 0x10b5910 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b5916); f_010ad500(cpu); if cpu.returned_to != 0x10b5916 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b5919; continue;
 }
 0x10b5919 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b5942; continue; }
  pc = 0x10b591f; continue;
 }
 0x10b591f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b5930; continue; }
  pc = 0x10b5927; continue;
 }
 0x10b5927 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b592d); f_010ae280(cpu); if cpu.returned_to != 0x10b592d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b5930; continue;
 }
 0x10b5930 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b593f); f_010ae130(cpu); if cpu.returned_to != 0x10b593f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10b5942; continue;
 }
 0x10b5942 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x10b5950); f_010ae400(cpu); if cpu.returned_to != 0x10b5950 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b5963); f_010b4890(cpu); if cpu.returned_to != 0x10b5963 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b596c); f_010b4960(cpu); if cpu.returned_to != 0x10b596c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b5971; continue;
 }
 0x10b5971 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x12d1fe0u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10b5981; continue; }
  pc = 0x10b5979; continue;
 }
 0x10b5979 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x12d1fe0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b5981; continue;
 }
 0x10b5981 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5971; continue; }
  pc = 0x10b5989; continue;
 }
 0x10b5989 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5990); }
 let mut pc: u32 = 0x10b5990;
 loop { match pc {
 0x10b5990 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2cu32; cpu.push(v); }
  cpu.push(0x10b599f); f_010b4990(cpu); if cpu.returned_to != 0x10b599f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b59c5; continue; }
  pc = 0x10b59a8; continue;
 }
 0x10b59a8 => {
  { let v: u32 = 0x116adccu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b59b4; continue;
 }
 0x10b59b4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b59b4; continue; }
  pc = 0x10b59be; continue;
 }
 0x10b59be => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b59c5 => {
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
  cpu.push(0x10b59df); f_010b48d0(cpu); if cpu.returned_to != 0x10b59df { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b5a0c; continue; }
  pc = 0x10b59e6; continue;
 }
 0x10b59e6 => {
  { let v: u32 = 0x116adf8u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b59f2; continue;
 }
 0x10b59f2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b59f2; continue; }
  pc = 0x10b59fc; continue;
 }
 0x10b59fc => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b5a02); f_010b4960(cpu); if cpu.returned_to != 0x10b5a02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b5a0c => {
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
  { let v: u32 = 0x116ae14u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b5a39; continue;
 }
 0x10b5a39 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5a47; continue; }
  pc = 0x10b5a41; continue;
 }
 0x10b5a41 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5a47; continue; }
  pc = 0x10b5a45; continue;
 }
 0x10b5a45 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5a47; continue;
 }
 0x10b5a47 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5a54; continue; }
  pc = 0x10b5a4c; continue;
 }
 0x10b5a4c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5a54; continue; }
  pc = 0x10b5a51; continue;
 }
 0x10b5a51 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b5a54; continue;
 }
 0x10b5a54 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b5a63; continue; }
  pc = 0x10b5a58; continue;
 }
 0x10b5a58 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b5a63; continue; }
  pc = 0x10b5a5c; continue;
 }
 0x10b5a5c => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b5a63; continue; }
  pc = 0x10b5a5f; continue;
 }
 0x10b5a5f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b5a39; continue;
 }
 0x10b5a63 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b5a6a; continue; }
  pc = 0x10b5a68; continue;
 }
 0x10b5a68 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5a6a; continue;
 }
 0x10b5a6a => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b5a7a; continue; }
  pc = 0x10b5a6e; continue;
 }
 0x10b5a6e => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b5afe; continue;
 }
 0x10b5a7a => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x116ae1cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  pc = 0x10b5a87; continue;
 }
 0x10b5a87 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5a95; continue; }
  pc = 0x10b5a8f; continue;
 }
 0x10b5a8f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5a95; continue; }
  pc = 0x10b5a93; continue;
 }
 0x10b5a93 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5a95; continue;
 }
 0x10b5a95 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5aa2; continue; }
  pc = 0x10b5a9a; continue;
 }
 0x10b5a9a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5aa2; continue; }
  pc = 0x10b5a9f; continue;
 }
 0x10b5a9f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b5aa2; continue;
 }
 0x10b5aa2 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b5ab1; continue; }
  pc = 0x10b5aa6; continue;
 }
 0x10b5aa6 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b5ab1; continue; }
  pc = 0x10b5aaa; continue;
 }
 0x10b5aaa => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b5ab1; continue; }
  pc = 0x10b5aad; continue;
 }
 0x10b5aad => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b5a87; continue;
 }
 0x10b5ab1 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b5ab8; continue; }
  pc = 0x10b5ab6; continue;
 }
 0x10b5ab6 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5ab8; continue;
 }
 0x10b5ab8 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b5af7; continue; }
  pc = 0x10b5abc; continue;
 }
 0x10b5abc => {
  { let v: u32 = 0x116ae24u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b5ac8; continue;
 }
 0x10b5ac8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5ac8; continue; }
  pc = 0x10b5ad2; continue;
 }
 0x10b5ad2 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b5ae4); f_010b4890(cpu); if cpu.returned_to != 0x10b5ae4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b5aed); f_010b4960(cpu); if cpu.returned_to != 0x10b5aed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b5af7 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b5afe; continue;
 }
 0x10b5afe => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b5b02; continue;
 }
 0x10b5b02 => {
  { cpu.sub32(cpu.r32(0x12d1fe0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10b5b1a; continue; }
  pc = 0x10b5b0b; continue;
 }
 0x10b5b0b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x12d1fe0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x32u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b5b23; continue;
 }
 0x10b5b1a => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { cpu.poll(); pc = 0x10b5b02; continue; }
  pc = 0x10b5b23; continue;
 }
 0x10b5b23 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x10b5b62; continue; }
  pc = 0x10b5b28; continue;
 }
 0x10b5b28 => {
  { let v: u32 = 0x116ae3cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b5b34; continue;
 }
 0x10b5b34 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5b34; continue; }
  pc = 0x10b5b3e; continue;
 }
 0x10b5b3e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b5b4f); f_010b4890(cpu); if cpu.returned_to != 0x10b5b4f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b5b58); f_010b4960(cpu); if cpu.returned_to != 0x10b5b58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b5b62 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x66u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10b5b80); f_010ae4c0(cpu); if cpu.returned_to != 0x10b5b80 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x36u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x34u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x10b5ba0); f_010ae400(cpu); if cpu.returned_to != 0x10b5ba0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x300u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10b5bc0); f_010ae130(cpu); if cpu.returned_to != 0x10b5bc0 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b5bdb; continue; }
  pc = 0x10b5bcf; continue;
 }
 0x10b5bcf => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b5c67; continue;
 }
 0x10b5bdb => {
  { let v = 0x10b6260u32; cpu.push(v); }
  cpu.push(0x10b5be5); f_010adc40(cpu); if cpu.returned_to != 0x10b5be5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b5c3b; continue; }
  pc = 0x10b5bf0; continue;
 }
 0x10b5bf0 => {
  { let v: u32 = 0x116ae54u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b5bfc; continue;
 }
 0x10b5bfc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5bfc; continue; }
  pc = 0x10b5c06; continue;
 }
 0x10b5c06 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x10b5c14); f_010ae400(cpu); if cpu.returned_to != 0x10b5c14 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b5c28); f_010b4890(cpu); if cpu.returned_to != 0x10b5c28 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b5c31); f_010b4960(cpu); if cpu.returned_to != 0x10b5c31 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b5c3b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b5c42); f_010adb30(cpu); if cpu.returned_to != 0x10b5c42 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b5c58); f_010ad9b0(cpu); if cpu.returned_to != 0x10b5c58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b5c64); f_010ad760(cpu); if cpu.returned_to != 0x10b5c64 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b5c67; continue;
 }
 0x10b5c67 => {
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
pub fn f_010b5c70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5c70); }
 let mut pc: u32 = 0x10b5c70;
 loop { match pc {
 0x10b5c70 => {
  { let v: u32 = 0x12d2020u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5c80); }
 let mut pc: u32 = 0x10b5c80;
 loop { match pc {
 0x10b5c80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d2120u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d2120u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b5ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b5ca0); }
 let mut pc: u32 = 0x10b5ca0;
 loop { match pc {
 0x10b5ca0 => {
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
  cpu.push(0x10b5d02); f_010b546b(cpu); if cpu.returned_to != 0x10b5d02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b5d7a; continue; }
  pc = 0x10b5d11; continue;
 }
 0x10b5d11 => {
  { cpu.sub8(cpu.r8(0x117c714u32), 0x0u8); }
  if !cpu.zf { pc = 0x10b5d21; continue; }
  pc = 0x10b5d1a; continue;
 }
 0x10b5d1a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b612d; continue;
 }
 0x10b5d21 => {
  { let v: u32 = 0x117c714u32; cpu.esi = v; }
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b5d2f; continue;
 }
 0x10b5d2f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5d2f; continue; }
  pc = 0x10b5d39; continue;
 }
 0x10b5d39 => {
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10b5d4f; continue;
 }
 0x10b5d4f => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5d4f; continue; }
  pc = 0x10b5d55; continue;
 }
 0x10b5d55 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x228u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b5d64); f_010b546b(cpu); if cpu.returned_to != 0x10b5d64 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b5d7a; continue; }
  pc = 0x10b5d73; continue;
 }
 0x10b5d73 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b612d; continue;
 }
 0x10b5d7a => {
  { let v: u32 = 0x10cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b5d8f; continue;
 }
 0x10b5d8f => {
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10b5dae); f_010b6140(cpu); if cpu.returned_to != 0x10b5dae { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b60ba; continue; }
  pc = 0x10b5db6; continue;
 }
 0x10b5db6 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10b5de8; continue; }
  pc = 0x10b5dd0; continue;
 }
 0x10b5dd0 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x10b5de8; continue; }
  pc = 0x10b5dda; continue;
 }
 0x10b5dda => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x325u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b5dd0; continue; }
  pc = 0x10b5de8; continue;
 }
 0x10b5de8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10b5df7; continue;
 }
 0x10b5dec => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x10b5e0e; continue; }
  pc = 0x10b5df6; continue;
 }
 0x10b5df6 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10b5df7; continue;
 }
 0x10b5df7 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x10b5dec; continue; }
  pc = 0x10b5e0e; continue;
 }
 0x10b5e0e => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x10b5e24; continue;
 }
 0x10b5e19 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x10b5e3b; continue; }
  pc = 0x10b5e23; continue;
 }
 0x10b5e23 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10b5e24; continue;
 }
 0x10b5e24 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x10b5e19; continue; }
  pc = 0x10b5e3b; continue;
 }
 0x10b5e3b => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x10b5e52; continue;
 }
 0x10b5e47 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x10b5e69; continue; }
  pc = 0x10b5e51; continue;
 }
 0x10b5e51 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10b5e52; continue;
 }
 0x10b5e52 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x10b5e47; continue; }
  pc = 0x10b5e69; continue;
 }
 0x10b5e69 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { cpu.poll(); pc = 0x10b5d8f; continue; }
  pc = 0x10b5e8d; continue;
 }
 0x10b5e8d => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x3bu8); }
  if cpu.zf { cpu.poll(); pc = 0x10b5d8f; continue; }
  pc = 0x10b5e99; continue;
 }
 0x10b5e99 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x116ad80u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b5ea5; continue;
 }
 0x10b5ea5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5eb3; continue; }
  pc = 0x10b5ead; continue;
 }
 0x10b5ead => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5eb3; continue; }
  pc = 0x10b5eb1; continue;
 }
 0x10b5eb1 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5eb3; continue;
 }
 0x10b5eb3 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5ec0; continue; }
  pc = 0x10b5eb8; continue;
 }
 0x10b5eb8 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5ec0; continue; }
  pc = 0x10b5ebd; continue;
 }
 0x10b5ebd => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b5ec0; continue;
 }
 0x10b5ec0 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b5ecf; continue; }
  pc = 0x10b5ec4; continue;
 }
 0x10b5ec4 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b5ecf; continue; }
  pc = 0x10b5ec8; continue;
 }
 0x10b5ec8 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b5ecf; continue; }
  pc = 0x10b5ecb; continue;
 }
 0x10b5ecb => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b5ea5; continue;
 }
 0x10b5ecf => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b5ed6; continue; }
  pc = 0x10b5ed4; continue;
 }
 0x10b5ed4 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5ed6; continue;
 }
 0x10b5ed6 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b5ef4; continue; }
  pc = 0x10b5eda; continue;
 }
 0x10b5eda => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b5ee5; continue;
 }
 0x10b5ee5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5ee5; continue; }
  pc = 0x10b5eef; continue;
 }
 0x10b5eef => {
  cpu.poll(); pc = 0x10b5d8f; continue;
 }
 0x10b5ef4 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x116ad88u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b5f00; continue;
 }
 0x10b5f00 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5f0e; continue; }
  pc = 0x10b5f08; continue;
 }
 0x10b5f08 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5f0e; continue; }
  pc = 0x10b5f0c; continue;
 }
 0x10b5f0c => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5f0e; continue;
 }
 0x10b5f0e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5f1b; continue; }
  pc = 0x10b5f13; continue;
 }
 0x10b5f13 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5f1b; continue; }
  pc = 0x10b5f18; continue;
 }
 0x10b5f18 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b5f1b; continue;
 }
 0x10b5f1b => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b5f2a; continue; }
  pc = 0x10b5f1f; continue;
 }
 0x10b5f1f => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b5f2a; continue; }
  pc = 0x10b5f23; continue;
 }
 0x10b5f23 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b5f2a; continue; }
  pc = 0x10b5f26; continue;
 }
 0x10b5f26 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b5f00; continue;
 }
 0x10b5f2a => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b5f31; continue; }
  pc = 0x10b5f2f; continue;
 }
 0x10b5f2f => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5f31; continue;
 }
 0x10b5f31 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b5f4a; continue; }
  pc = 0x10b5f35; continue;
 }
 0x10b5f35 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b5f3b; continue;
 }
 0x10b5f3b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5f3b; continue; }
  pc = 0x10b5f45; continue;
 }
 0x10b5f45 => {
  cpu.poll(); pc = 0x10b5d8f; continue;
 }
 0x10b5f4a => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x116ad90u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b5f56; continue;
 }
 0x10b5f56 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5f64; continue; }
  pc = 0x10b5f5e; continue;
 }
 0x10b5f5e => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5f64; continue; }
  pc = 0x10b5f62; continue;
 }
 0x10b5f62 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5f64; continue;
 }
 0x10b5f64 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5f71; continue; }
  pc = 0x10b5f69; continue;
 }
 0x10b5f69 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5f71; continue; }
  pc = 0x10b5f6e; continue;
 }
 0x10b5f6e => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b5f71; continue;
 }
 0x10b5f71 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b5f80; continue; }
  pc = 0x10b5f75; continue;
 }
 0x10b5f75 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b5f80; continue; }
  pc = 0x10b5f79; continue;
 }
 0x10b5f79 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b5f80; continue; }
  pc = 0x10b5f7c; continue;
 }
 0x10b5f7c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b5f56; continue;
 }
 0x10b5f80 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b5f87; continue; }
  pc = 0x10b5f85; continue;
 }
 0x10b5f85 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5f87; continue;
 }
 0x10b5f87 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b5fa5; continue; }
  pc = 0x10b5f8b; continue;
 }
 0x10b5f8b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b5f95); f_010b61c0(cpu); if cpu.returned_to != 0x10b5f95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x100u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b5d8f; continue;
 }
 0x10b5fa5 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116ad98u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b5fb1; continue;
 }
 0x10b5fb1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5fbf; continue; }
  pc = 0x10b5fb9; continue;
 }
 0x10b5fb9 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5fbf; continue; }
  pc = 0x10b5fbd; continue;
 }
 0x10b5fbd => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5fbf; continue;
 }
 0x10b5fbf => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b5fcc; continue; }
  pc = 0x10b5fc4; continue;
 }
 0x10b5fc4 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b5fcc; continue; }
  pc = 0x10b5fc9; continue;
 }
 0x10b5fc9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b5fcc; continue;
 }
 0x10b5fcc => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b5fdb; continue; }
  pc = 0x10b5fd0; continue;
 }
 0x10b5fd0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b5fdb; continue; }
  pc = 0x10b5fd4; continue;
 }
 0x10b5fd4 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b5fdb; continue; }
  pc = 0x10b5fd7; continue;
 }
 0x10b5fd7 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b5fb1; continue;
 }
 0x10b5fdb => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b5fe2; continue; }
  pc = 0x10b5fe0; continue;
 }
 0x10b5fe0 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b5fe2; continue;
 }
 0x10b5fe2 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b6000; continue; }
  pc = 0x10b5fe6; continue;
 }
 0x10b5fe6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b5ff0); f_010b61c0(cpu); if cpu.returned_to != 0x10b5ff0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x102u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b5d8f; continue;
 }
 0x10b6000 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x116ad9cu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b600c; continue;
 }
 0x10b600c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b601a; continue; }
  pc = 0x10b6014; continue;
 }
 0x10b6014 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b601a; continue; }
  pc = 0x10b6018; continue;
 }
 0x10b6018 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b601a; continue;
 }
 0x10b601a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b6027; continue; }
  pc = 0x10b601f; continue;
 }
 0x10b601f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b6027; continue; }
  pc = 0x10b6024; continue;
 }
 0x10b6024 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b6027; continue;
 }
 0x10b6027 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b6036; continue; }
  pc = 0x10b602b; continue;
 }
 0x10b602b => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b6036; continue; }
  pc = 0x10b602f; continue;
 }
 0x10b602f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b6036; continue; }
  pc = 0x10b6032; continue;
 }
 0x10b6032 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b600c; continue;
 }
 0x10b6036 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b603d; continue; }
  pc = 0x10b603b; continue;
 }
 0x10b603b => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b603d; continue;
 }
 0x10b603d => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b605b; continue; }
  pc = 0x10b6041; continue;
 }
 0x10b6041 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b604b); f_010b61c0(cpu); if cpu.returned_to != 0x10b604b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x104u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b5d8f; continue;
 }
 0x10b605b => {
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { let v: u32 = 0x116ada8u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b6067; continue;
 }
 0x10b6067 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b6075; continue; }
  pc = 0x10b606f; continue;
 }
 0x10b606f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b6075; continue; }
  pc = 0x10b6073; continue;
 }
 0x10b6073 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b6075; continue;
 }
 0x10b6075 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b6082; continue; }
  pc = 0x10b607a; continue;
 }
 0x10b607a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b6082; continue; }
  pc = 0x10b607f; continue;
 }
 0x10b607f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b6082; continue;
 }
 0x10b6082 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b6091; continue; }
  pc = 0x10b6086; continue;
 }
 0x10b6086 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b6091; continue; }
  pc = 0x10b608a; continue;
 }
 0x10b608a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b6091; continue; }
  pc = 0x10b608d; continue;
 }
 0x10b608d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b6067; continue;
 }
 0x10b6091 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b6098; continue; }
  pc = 0x10b6096; continue;
 }
 0x10b6096 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b6098; continue;
 }
 0x10b6098 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10b5d8f; continue; }
  pc = 0x10b60a0; continue;
 }
 0x10b60a0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b60aa); f_010b61c0(cpu); if cpu.returned_to != 0x10b60aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x106u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x10b5d8f; continue;
 }
 0x10b60ba => {
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b60c7); f_010b5595(cpu); if cpu.returned_to != 0x10b60c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b60f3; continue; }
  pc = 0x10b60e1; continue;
 }
 0x10b60e1 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b6118; continue; }
  pc = 0x10b60f3; continue;
 }
 0x10b60f3 => {
  { let v: u32 = 0x116adb4u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b60ff; continue;
 }
 0x10b60ff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b60ff; continue; }
  pc = 0x10b6109; continue;
 }
 0x10b6109 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x378u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6118 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10b612d; continue;
 }
 0x10b612d => {
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
pub fn f_010b6140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6140); }
 let mut pc: u32 = 0x10b6140;
 loop { match pc {
 0x10b6140 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x10b61a7; continue; }
  pc = 0x10b6155; continue;
 }
 0x10b6155 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x10b617f; continue; }
  pc = 0x10b615b; continue;
 }
 0x10b615b => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6175); f_010b54e1(cpu); if cpu.returned_to != 0x10b6175 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b61a7; continue; }
  pc = 0x10b617f; continue;
 }
 0x10b617f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xdu8); }
  if cpu.zf { pc = 0x10b61a0; continue; }
  pc = 0x10b6190; continue;
 }
 0x10b6190 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if !cpu.zf { pc = 0x10b619c; continue; }
  pc = 0x10b6196; continue;
 }
 0x10b6196 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b61a7; continue;
 }
 0x10b619c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x10b61a0; continue;
 }
 0x10b61a0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x10b6155; continue; }
  pc = 0x10b61a7; continue;
 }
 0x10b61a7 => {
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
pub fn f_010b61c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b61c0); }
 let mut pc: u32 = 0x10b61c0;
 loop { match pc {
 0x10b61c0 => {
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
  pc = 0x10b6228; continue;
 }
 0x10b61e1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b61f1; continue; }
  pc = 0x10b61e9; continue;
 }
 0x10b61e9 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b61f1; continue; }
  pc = 0x10b61ee; continue;
 }
 0x10b61ee => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10b61f1; continue;
 }
 0x10b61f1 => {
  { cpu.sub8((cpu.edx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x10b61fb; continue; }
  pc = 0x10b61f6; continue;
 }
 0x10b61f6 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b6226; continue;
 }
 0x10b61fb => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x10b6226; continue; }
  pc = 0x10b6200; continue;
 }
 0x10b6200 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b6222; continue; }
  pc = 0x10b6206; continue;
 }
 0x10b6206 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x116ad6cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10b621d; continue; }
  pc = 0x10b620e; continue;
 }
 0x10b620e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b6222; continue;
 }
 0x10b621d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b6206; continue; }
  pc = 0x10b6222; continue;
 }
 0x10b6222 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x10b623c; continue; }
  pc = 0x10b6226; continue;
 }
 0x10b6226 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x10b6228; continue;
 }
 0x10b6228 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x10b61e1; continue; }
  pc = 0x10b623c; continue;
 }
 0x10b623c => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b624d; continue; }
  pc = 0x10b6244; continue;
 }
 0x10b6244 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10b624d; continue;
 }
 0x10b624d => {
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
pub fn f_010b6260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6260); }
 let mut pc: u32 = 0x10b6260;
 loop { match pc {
 0x10b6260 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c814u32), 0x0u32); }
  if !cpu.zf { pc = 0x10b62ac; continue; }
  pc = 0x10b626b; continue;
 }
 0x10b626b => {
  { let v: u32 = 0x1u32; cpu.w32(0x117c814u32, v); }
  cpu.push(0x10b627a); f_010ae270(cpu); if cpu.returned_to != 0x10b627a { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d2188u32, v); }
  cpu.push(0x10b6292); f_010ae130(cpu); if cpu.returned_to != 0x10b6292 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d2188u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x10b62a3); f_010ae260(cpu); if cpu.returned_to != 0x10b62a3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x117c814u32, v); }
  pc = 0x10b62ac; continue;
 }
 0x10b62ac => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b62c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b62c0); }
 let mut pc: u32 = 0x10b62c0;
 loop { match pc {
 0x10b62c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c818u32), 0x0u32); }
  if !cpu.zf { pc = 0x10b6346; continue; }
  pc = 0x10b62cc; continue;
 }
 0x10b62cc => {
  { let v = 0x10b55d0u32; cpu.push(v); }
  { let v = 0x10b62c0u32; cpu.push(v); }
  cpu.push(0x10b62db); f_010b47f0(cpu); if cpu.returned_to != 0x10b62db { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x12d1fe0u32; cpu.push(v); }
  cpu.push(0x10b62ea); f_010b4730(cpu); if cpu.returned_to != 0x10b62ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x12d2120u32; cpu.push(v); }
  cpu.push(0x10b62f9); f_010b4730(cpu); if cpu.returned_to != 0x10b62f9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x12d2020u32; cpu.push(v); }
  cpu.push(0x10b630b); f_010b4730(cpu); if cpu.returned_to != 0x10b630b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x12d2170u32; cpu.push(v); }
  cpu.push(0x10b631a); f_010b4730(cpu); if cpu.returned_to != 0x10b631a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c814u32; cpu.push(v); }
  cpu.push(0x10b6329); f_010b4730(cpu); if cpu.returned_to != 0x10b6329 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d2188u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10b633d); f_010b4730(cpu); if cpu.returned_to != 0x10b633d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c818u32, v); }
  pc = 0x10b6346; continue;
 }
 0x10b6346 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6350); }
 let mut pc: u32 = 0x10b6350;
 loop { match pc {
 0x10b6350 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c8a0u32), 0x0u32); }
  if cpu.zf { pc = 0x10b6404; continue; }
  pc = 0x10b6360; continue;
 }
 0x10b6360 => {
  { let v = 0x10b6350u32; cpu.push(v); }
  { let v = 0x10b80d0u32; cpu.push(v); }
  cpu.push(0x10b636f); f_010b4750(cpu); if cpu.returned_to != 0x10b636f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x117c81cu32; cpu.push(v); }
  cpu.push(0x10b6381); f_010b4710(cpu); if cpu.returned_to != 0x10b6381 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c89cu32; cpu.push(v); }
  cpu.push(0x10b6390); f_010b4710(cpu); if cpu.returned_to != 0x10b6390 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21b8u32; cpu.push(v); }
  cpu.push(0x10b639f); f_010b4710(cpu); if cpu.returned_to != 0x10b639f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x12d21a4u32; cpu.push(v); }
  cpu.push(0x10b63ae); f_010b4710(cpu); if cpu.returned_to != 0x10b63ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21c0u32; cpu.push(v); }
  cpu.push(0x10b63bd); f_010b4710(cpu); if cpu.returned_to != 0x10b63bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21c4u32; cpu.push(v); }
  cpu.push(0x10b63cc); f_010b4710(cpu); if cpu.returned_to != 0x10b63cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21b4u32; cpu.push(v); }
  cpu.push(0x10b63db); f_010b4710(cpu); if cpu.returned_to != 0x10b63db { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21bcu32; cpu.push(v); }
  cpu.push(0x10b63ea); f_010b4710(cpu); if cpu.returned_to != 0x10b63ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21b0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10b63fb); f_010b4710(cpu); if cpu.returned_to != 0x10b63fb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c8a0u32, v); }
  pc = 0x10b6404; continue;
 }
 0x10b6404 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6410); }
 let mut pc: u32 = 0x10b6410;
 loop { match pc {
 0x10b6410 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b6426; continue; }
  pc = 0x10b6421; continue;
 }
 0x10b6421 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b6426; continue;
 }
 0x10b6426 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b642f; continue; }
  pc = 0x10b642a; continue;
 }
 0x10b642a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b642f; continue;
 }
 0x10b642f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10b643f; continue; }
  pc = 0x10b6436; continue;
 }
 0x10b6436 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b643f; continue;
 }
 0x10b643f => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b644c; continue; }
  pc = 0x10b6447; continue;
 }
 0x10b6447 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b644c; continue;
 }
 0x10b644c => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b65b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b65b0); } r_010b65b0(cpu, 0x10b65b0); }
pub fn f_010b65be(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b65be); } r_010b65b0(cpu, 0x10b65be); }
fn r_010b65b0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b65b0 => {
  cpu.unimplemented(0x10b65b0, "Frstor");
  { let v: u32 = cpu.esp.wrapping_add(cpu.esp); cpu.esp = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u8 = cpu.port_in(0x65u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  pc = 0x10b6623; continue;
 }
 0x10b65be => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b65f0; continue; }
  pc = 0x10b65d5; continue;
 }
 0x10b65d5 => {
  cpu.poll(); pc = cpu.r32(0x10b65b0u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x10b65dd => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x10b65f0; continue;
 }
 0x10b65e4 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  pc = 0x10b65f0; continue;
 }
 0x10b65eb => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x10b65f0; continue;
 }
 0x10b65f0 => {
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
  { let v: u32 = cpu.r32(0x12d2138u32); cpu.edx = v; }
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
  if cpu.zf { pc = 0x10b6647; continue; }
  pc = 0x10b6632; continue;
 }
 0x10b6623 => {
  cpu.unimplemented(0x10b6623, "Reservednop");
  { let v: u32 = cpu.sar32(cpu.eax, 15u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x10b6647; continue; }
  pc = 0x10b6632; continue;
 }
 0x10b6632 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x10b6647; continue; }
  pc = 0x10b663d; continue;
 }
 0x10b663d => {
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x10b6647; continue; }
  pc = 0x10b6644; continue;
 }
 0x10b6644 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x10b6647; continue;
 }
 0x10b6647 => {
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
pub fn f_010b6660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6660); }
 let mut pc: u32 = 0x10b6660;
 loop { match pc {
 0x10b6660 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6670); }
 let mut pc: u32 = 0x10b6670;
 loop { match pc {
 0x10b6670 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b66ab; continue; }
  pc = 0x10b6687; continue;
 }
 0x10b6687 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b6689; continue;
 }
 0x10b6689 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10b669d; continue; }
  pc = 0x10b6694; continue;
 }
 0x10b6694 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b669a); f_010b7c40(cpu); if cpu.returned_to != 0x10b669a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b669d; continue;
 }
 0x10b669d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8a8u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b6689; continue; }
  pc = 0x10b66ab; continue;
 }
 0x10b66ab => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b66b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b66b0); }
 let mut pc: u32 = 0x10b66b0;
 loop { match pc {
 0x10b66b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b66bb; continue; }
  pc = 0x10b66b8; continue;
 }
 0x10b66b8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b66bb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x868u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b66d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b66d0); }
 let mut pc: u32 = 0x10b66d0;
 loop { match pc {
 0x10b66d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b66eb; continue; }
  pc = 0x10b66d8; continue;
 }
 0x10b66d8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x868u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b66eb; continue;
 }
 0x10b66eb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b66f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b66f0); }
 let mut pc: u32 = 0x10b66f0;
 loop { match pc {
 0x10b66f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b670b; continue; }
  pc = 0x10b66f9; continue;
 }
 0x10b66f9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10b670b; continue;
 }
 0x10b670b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6750); }
 let mut pc: u32 = 0x10b6750;
 loop { match pc {
 0x10b6750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b6796; continue; }
  pc = 0x10b675a; continue;
 }
 0x10b675a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10b6796; continue; }
  pc = 0x10b6762; continue;
 }
 0x10b6762 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x10b6796; continue; }
  pc = 0x10b6767; continue;
 }
 0x10b6767 => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b6786; continue; }
  pc = 0x10b677f; continue;
 }
 0x10b677f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x10b6786); cpu.call(t); if cpu.returned_to != 0x10b6786 { return; }
  pc = 0x10b6786; continue;
 }
 0x10b6786 => {
  { cpu.sub32(cpu.r32(0x864u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b6796; continue; }
  pc = 0x10b678f; continue;
 }
 0x10b678f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x10b6796); cpu.call(t); if cpu.returned_to != 0x10b6796 { return; }
  pc = 0x10b6796; continue;
 }
 0x10b6796 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b67a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b67a0); }
 let mut pc: u32 = 0x10b67a0;
 loop { match pc {
 0x10b67a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b67f4; continue; }
  pc = 0x10b67ad; continue;
 }
 0x10b67ad => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x8u32); }
  if !cpu.zf { pc = 0x10b67f4; continue; }
  pc = 0x10b67b3; continue;
 }
 0x10b67b3 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b67f4; continue; }
  pc = 0x10b67c4; continue;
 }
 0x10b67c4 => {
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
  cpu.push(0x10b67ea); f_010ae130(cpu); if cpu.returned_to != 0x10b67ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b67f4; continue;
 }
 0x10b67f4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6820); }
 let mut pc: u32 = 0x10b6820;
 loop { match pc {
 0x10b6820 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b689a; continue; }
  pc = 0x10b682e; continue;
 }
 0x10b682e => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10b689a; continue; }
  pc = 0x10b6834; continue;
 }
 0x10b6834 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b689a; continue; }
  pc = 0x10b6842; continue;
 }
 0x10b6842 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b689a; continue; }
  pc = 0x10b6848; continue;
 }
 0x10b6848 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10b689a; continue; }
  pc = 0x10b686a; continue;
 }
 0x10b686a => {
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
  cpu.push(0x10b6890); f_010ae130(cpu); if cpu.returned_to != 0x10b6890 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b689a; continue;
 }
 0x10b689a => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b68b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b68b0); }
 let mut pc: u32 = 0x10b68b0;
 loop { match pc {
 0x10b68b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b68c6; continue; }
  pc = 0x10b68b8; continue;
 }
 0x10b68b8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b68c6; continue;
 }
 0x10b68c6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b68d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b68d0); }
 let mut pc: u32 = 0x10b68d0;
 loop { match pc {
 0x10b68d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b68df; continue; }
  pc = 0x10b68d8; continue;
 }
 0x10b68d8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b68df; continue;
 }
 0x10b68df => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b68e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b68e0); }
 let mut pc: u32 = 0x10b68e0;
 loop { match pc {
 0x10b68e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b68ee; continue; }
  pc = 0x10b68e8; continue;
 }
 0x10b68e8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b68ee => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6940); }
 let mut pc: u32 = 0x10b6940;
 loop { match pc {
 0x10b6940 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b695d; continue; }
  pc = 0x10b694c; continue;
 }
 0x10b694c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x54u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10b695d; continue; }
  pc = 0x10b6951; continue;
 }
 0x10b6951 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b695a); f_010b7c40(cpu); if cpu.returned_to != 0x10b695a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b695d; continue;
 }
 0x10b695d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6960); }
 let mut pc: u32 = 0x10b6960;
 loop { match pc {
 0x10b6960 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6969; continue; }
  pc = 0x10b6968; continue;
 }
 0x10b6968 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6969 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6970); }
 let mut pc: u32 = 0x10b6970;
 loop { match pc {
 0x10b6970 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b698d; continue; }
  pc = 0x10b697c; continue;
 }
 0x10b697c => {
  { cpu.sub32(cpu.edx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10b698d; continue; }
  pc = 0x10b6981; continue;
 }
 0x10b6981 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b698a); f_010b7c40(cpu); if cpu.returned_to != 0x10b698a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b698d; continue;
 }
 0x10b698d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6990); }
 let mut pc: u32 = 0x10b6990;
 loop { match pc {
 0x10b6990 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6999; continue; }
  pc = 0x10b6998; continue;
 }
 0x10b6998 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6999 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b69a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b69a0); }
 let mut pc: u32 = 0x10b69a0;
 loop { match pc {
 0x10b69a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b69af; continue; }
  pc = 0x10b69a8; continue;
 }
 0x10b69a8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10b69af; continue;
 }
 0x10b69af => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b69b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b69b0); }
 let mut pc: u32 = 0x10b69b0;
 loop { match pc {
 0x10b69b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b69b9; continue; }
  pc = 0x10b69b8; continue;
 }
 0x10b69b8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b69b9 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b69c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b69c0); }
 let mut pc: u32 = 0x10b69c0;
 loop { match pc {
 0x10b69c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b69ee; continue; }
  pc = 0x10b69cd; continue;
 }
 0x10b69cd => {
  { cpu.sub32(cpu.edx, cpu.r32(0x44u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10b69db; continue; }
  pc = 0x10b69d2; continue;
 }
 0x10b69d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x48u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10b69ee; continue; }
  pc = 0x10b69db; continue;
 }
 0x10b69db => {
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b69eb); f_010b7c40(cpu); if cpu.returned_to != 0x10b69eb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b69ee; continue;
 }
 0x10b69ee => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b69f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b69f0); }
 let mut pc: u32 = 0x10b69f0;
 loop { match pc {
 0x10b69f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6a14; continue; }
  pc = 0x10b69f8; continue;
 }
 0x10b69f8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b6a14; continue;
 }
 0x10b6a14 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6a20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6a20); }
 let mut pc: u32 = 0x10b6a20;
 loop { match pc {
 0x10b6a20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6a29; continue; }
  pc = 0x10b6a28; continue;
 }
 0x10b6a28 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6a29 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6a30); }
 let mut pc: u32 = 0x10b6a30;
 loop { match pc {
 0x10b6a30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6b14; continue; }
  pc = 0x10b6a3d; continue;
 }
 0x10b6a3d => {
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
  { let v: u32 = cpu.r32(0x12d2134u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x2b11u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b6ad3; continue; }
  pc = 0x10b6ace; continue;
 }
 0x10b6ace => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x10b6adc; continue; }
  pc = 0x10b6ad3; continue;
 }
 0x10b6ad3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b6ae3; continue;
 }
 0x10b6adc => {
  { let v: u32 = 0x40u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b6ae3; continue;
 }
 0x10b6ae3 => {
  { let v: u32 = 0x0u32; cpu.w32(0x85cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x860u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b6b11); f_010b7c40(cpu); if cpu.returned_to != 0x10b6b11 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b6b14; continue;
 }
 0x10b6b14 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6b20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6b20); }
 let mut pc: u32 = 0x10b6b20;
 loop { match pc {
 0x10b6b20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6b2f; continue; }
  pc = 0x10b6b28; continue;
 }
 0x10b6b28 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b6b2f; continue;
 }
 0x10b6b2f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6b30); }
 let mut pc: u32 = 0x10b6b30;
 loop { match pc {
 0x10b6b30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x10b6b3c); f_010ae5d0(cpu); if cpu.returned_to != 0x10b6b3c { return; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b6b5c; continue; }
  pc = 0x10b6b45; continue;
 }
 0x10b6b45 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x10b6b48; continue;
 }
 0x10b6b48 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x10b6b5c; continue; }
  pc = 0x10b6b4e; continue;
 }
 0x10b6b4e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b6b48; continue; }
  pc = 0x10b6b5c; continue;
 }
 0x10b6b5c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x64u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x10b6b82; continue; }
  pc = 0x10b6b61; continue;
 }
 0x10b6b61 => {
  { let v: u32 = 0x116afd0u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6b6d; continue;
 }
 0x10b6b6d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6b6d; continue; }
  pc = 0x10b6b77; continue;
 }
 0x10b6b77 => {
  cpu.push(0x10b6b7c); f_010ae5c0(cpu); if cpu.returned_to != 0x10b6b7c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6b82 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b6b93); f_010acb80(cpu); if cpu.returned_to != 0x10b6b93 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x10b6b9b); f_010ae5c0(cpu); if cpu.returned_to != 0x10b6b9b { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6bb0); }
 let mut pc: u32 = 0x10b6bb0;
 loop { match pc {
 0x10b6bb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b6bd9; continue; }
  pc = 0x10b6bc1; continue;
 }
 0x10b6bc1 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x10b6bc4; continue;
 }
 0x10b6bc4 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x4u32); }
  if !cpu.zf { pc = 0x10b6bcb; continue; }
  pc = 0x10b6bca; continue;
 }
 0x10b6bca => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x10b6bcb; continue;
 }
 0x10b6bcb => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b6bc4; continue; }
  pc = 0x10b6bd9; continue;
 }
 0x10b6bd9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6be0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6be0); }
 let mut pc: u32 = 0x10b6be0;
 loop { match pc {
 0x10b6be0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b6bed); f_010ad1c0(cpu); if cpu.returned_to != 0x10b6bed { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6c00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6c00); }
 let mut pc: u32 = 0x10b6c00;
 loop { match pc {
 0x10b6c00 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x116afa8u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6c17); f_010add40(cpu); if cpu.returned_to != 0x10b6c17 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6c3b; continue; }
  pc = 0x10b6c1e; continue;
 }
 0x10b6c1e => {
  { let v: u32 = 0x116afb0u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6c2a; continue;
 }
 0x10b6c2a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6c2a; continue; }
  pc = 0x10b6c34; continue;
 }
 0x10b6c34 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10b6c70; continue;
 }
 0x10b6c3b => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6c50); f_010acfb0(cpu); if cpu.returned_to != 0x10b6c50 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6c6e; continue; }
  pc = 0x10b6c60; continue;
 }
 0x10b6c60 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6c6e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b6c70; continue;
 }
 0x10b6c70 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6c80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6c80); }
 let mut pc: u32 = 0x10b6c80;
 loop { match pc {
 0x10b6c80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b6c91); f_010ad270(cpu); if cpu.returned_to != 0x10b6c91 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6cb9; continue; }
  pc = 0x10b6c9a; continue;
 }
 0x10b6c9a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6ca5); f_010b6da0(cpu); if cpu.returned_to != 0x10b6ca5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6cb7; continue; }
  pc = 0x10b6cae; continue;
 }
 0x10b6cae => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b6cb4); f_010ad1c0(cpu); if cpu.returned_to != 0x10b6cb4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b6cb7; continue;
 }
 0x10b6cb7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b6cb9; continue;
 }
 0x10b6cb9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6cc0); }
 let mut pc: u32 = 0x10b6cc0;
 loop { match pc {
 0x10b6cc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b6cd8); f_010b45a0(cpu); if cpu.returned_to != 0x10b6cd8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6d46; continue; }
  pc = 0x10b6ce1; continue;
 }
 0x10b6ce1 => {
  { cpu.sub8(cpu.r8(0x117c714u32), 0x0u8); }
  if !cpu.zf { pc = 0x10b6d07; continue; }
  pc = 0x10b6cea; continue;
 }
 0x10b6cea => {
  { let v: u32 = 0x116af90u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6cf6; continue;
 }
 0x10b6cf6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6cf6; continue; }
  pc = 0x10b6d00; continue;
 }
 0x10b6d00 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b6d92; continue;
 }
 0x10b6d07 => {
  { let v: u32 = 0x117c714u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6d10; continue;
 }
 0x10b6d10 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6d10; continue; }
  pc = 0x10b6d1a; continue;
 }
 0x10b6d1a => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10b6d2b; continue;
 }
 0x10b6d2b => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6d2b; continue; }
  pc = 0x10b6d31; continue;
 }
 0x10b6d31 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6d3d); f_010b45a0(cpu); if cpu.returned_to != 0x10b6d3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x10b6cea; continue; }
  pc = 0x10b6d46; continue;
 }
 0x10b6d46 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6d52); f_010ad270(cpu); if cpu.returned_to != 0x10b6d52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10b6d5d); f_010b4960(cpu); if cpu.returned_to != 0x10b6d5d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10b6d70; continue; }
  pc = 0x10b6d64; continue;
 }
 0x10b6d64 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6d70 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b6d7e); f_010b6da0(cpu); if cpu.returned_to != 0x10b6d7e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6d90; continue; }
  pc = 0x10b6d87; continue;
 }
 0x10b6d87 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b6d8d); f_010ad1c0(cpu); if cpu.returned_to != 0x10b6d8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b6d90; continue;
 }
 0x10b6d90 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b6d92; continue;
 }
 0x10b6d92 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b6da0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b6da0); }
 let mut pc: u32 = 0x10b6da0;
 loop { match pc {
 0x10b6da0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x10b6dac); f_010b80d0(cpu); if cpu.returned_to != 0x10b6dac { return; }
  { let v = 0x90u32; cpu.push(v); }
  cpu.push(0x10b6db6); f_010b4990(cpu); if cpu.returned_to != 0x10b6db6 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b6ddc; continue; }
  pc = 0x10b6dbf; continue;
 }
 0x10b6dbf => {
  { let v: u32 = 0x116ae98u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6dcb; continue;
 }
 0x10b6dcb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6dcb; continue; }
  pc = 0x10b6dd5; continue;
 }
 0x10b6dd5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b73d2; continue;
 }
 0x10b6ddc => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116aec0u32; cpu.esi = v; }
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
  if cpu.zf { pc = 0x10b6e33; continue; }
  pc = 0x10b6e08; continue;
 }
 0x10b6e08 => {
  { let v: u32 = 0x116aec8u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6e16; continue;
 }
 0x10b6e16 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6e16; continue; }
  pc = 0x10b6e20; continue;
 }
 0x10b6e20 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b6e26); f_010b4960(cpu); if cpu.returned_to != 0x10b6e26 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b6e33 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6e44); f_010ae130(cpu); if cpu.returned_to != 0x10b6e44 { return; }
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
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x10b6f03; continue; }
  pc = 0x10b6ecd; continue;
 }
 0x10b6ecd => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b6ee6); f_010b7480(cpu); if cpu.returned_to != 0x10b6ee6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6f03; continue; }
  pc = 0x10b6eed; continue;
 }
 0x10b6eed => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x10b6f03; continue;
 }
 0x10b6f03 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b6f4c; continue; }
  pc = 0x10b6f0a; continue;
 }
 0x10b6f0a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b6f13); f_010ad380(cpu); if cpu.returned_to != 0x10b6f13 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6f4c; continue; }
  pc = 0x10b6f1c; continue;
 }
 0x10b6f1c => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x10b6f31); f_010b7480(cpu); if cpu.returned_to != 0x10b6f31 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6f4c; continue; }
  pc = 0x10b6f38; continue;
 }
 0x10b6f38 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x10b6f4c; continue;
 }
 0x10b6f4c => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10b6fe7; continue; }
  pc = 0x10b6f58; continue;
 }
 0x10b6f58 => {
  { cpu.sub32(cpu.r32(0x12d2164u32), 0x1u32); }
  if !cpu.zf { pc = 0x10b6fe7; continue; }
  pc = 0x10b6f65; continue;
 }
 0x10b6f65 => {
  { let v: u32 = cpu.edi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b6fd3; continue;
 }
 0x10b6f6f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10b6f9b; continue; }
  pc = 0x10b6f8d; continue;
 }
 0x10b6f8d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x10b6f9b; continue;
 }
 0x10b6f9b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b6fa2); f_010b7480(cpu); if cpu.returned_to != 0x10b6fa2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b6fbf; continue; }
  pc = 0x10b6fa9; continue;
 }
 0x10b6fa9 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x10b6fe7; continue;
 }
 0x10b6fbf => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x18u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b6fd3; continue;
 }
 0x10b6fd3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b6f6f; continue; }
  pc = 0x10b6fe7; continue;
 }
 0x10b6fe7 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10b7017; continue; }
  pc = 0x10b6fee; continue;
 }
 0x10b6fee => {
  { let v: u32 = 0x116aee0u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b6ffa; continue;
 }
 0x10b6ffa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b6ffa; continue; }
  pc = 0x10b7004; continue;
 }
 0x10b7004 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b700a); f_010b4960(cpu); if cpu.returned_to != 0x10b700a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b7017 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { cpu.sub32(cpu.r32(0x12d2168u32), 0x0u32); }
  if cpu.zf { pc = 0x10b7035; continue; }
  pc = 0x10b702e; continue;
 }
 0x10b702e => {
  { let v: u32 = 0x2000u32; cpu.ebx = v; }
  pc = 0x10b703b; continue;
 }
 0x10b7035 => {
  { let v: u32 = cpu.r32(0x12d2128u32); cpu.ebx = v; }
  pc = 0x10b703b; continue;
 }
 0x10b703b => {
  { let v: u32 = 0xfu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x10b7054; continue;
 }
 0x10b7054 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b7069); f_010b48d0(cpu); if cpu.returned_to != 0x10b7069 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b7099; continue; }
  pc = 0x10b7070; continue;
 }
 0x10b7070 => {
  { let v: u32 = 0x116af04u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b707c; continue;
 }
 0x10b707c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b707c; continue; }
  pc = 0x10b7086; continue;
 }
 0x10b7086 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b708c); f_010b4960(cpu); if cpu.returned_to != 0x10b708c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b7099 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b70b1; continue; }
  pc = 0x10b70a0; continue;
 }
 0x10b70a0 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b70ae); f_010b4890(cpu); if cpu.returned_to != 0x10b70ae { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10b70b1; continue;
 }
 0x10b70b1 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x10b7054; continue; }
  pc = 0x10b70e5; continue;
 }
 0x10b70e5 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b70eb); f_010b75b0(cpu); if cpu.returned_to != 0x10b70eb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b7119; continue; }
  pc = 0x10b70f2; continue;
 }
 0x10b70f2 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b7103); f_010b4890(cpu); if cpu.returned_to != 0x10b7103 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b710c); f_010b4960(cpu); if cpu.returned_to != 0x10b710c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b7119 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b712b); f_010ae130(cpu); if cpu.returned_to != 0x10b712b { return; }
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
  cpu.push(0x10b7168); f_010b4990(cpu); if cpu.returned_to != 0x10b7168 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b71e3; continue; }
  pc = 0x10b7172; continue;
 }
 0x10b7172 => {
  { let v: u32 = 0x116af24u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b717e; continue;
 }
 0x10b717e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b717e; continue; }
  pc = 0x10b7188; continue;
 }
 0x10b7188 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b719d; continue; }
  pc = 0x10b7194; continue;
 }
 0x10b7194 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b719a); f_010ae280(cpu); if cpu.returned_to != 0x10b719a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b719d; continue;
 }
 0x10b719d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b71af); f_010ae130(cpu); if cpu.returned_to != 0x10b71af { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b71cd); f_010b4890(cpu); if cpu.returned_to != 0x10b71cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b71d6); f_010b4960(cpu); if cpu.returned_to != 0x10b71d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b71e3 => {
  { let v: u32 = cpu.r32(0x12d2130u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b71f7); f_010b4990(cpu); if cpu.returned_to != 0x10b71f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b7282; continue; }
  pc = 0x10b7205; continue;
 }
 0x10b7205 => {
  { let v: u32 = 0x116af48u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b7211; continue;
 }
 0x10b7211 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b7211; continue; }
  pc = 0x10b721b; continue;
 }
 0x10b721b => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b7230; continue; }
  pc = 0x10b7227; continue;
 }
 0x10b7227 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b722d); f_010ae280(cpu); if cpu.returned_to != 0x10b722d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b7230; continue;
 }
 0x10b7230 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b7242); f_010ae130(cpu); if cpu.returned_to != 0x10b7242 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b7260); f_010b4890(cpu); if cpu.returned_to != 0x10b7260 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b726c); f_010b4960(cpu); if cpu.returned_to != 0x10b726c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b7275); f_010b4960(cpu); if cpu.returned_to != 0x10b7275 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b7282 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b72d7; continue; }
  pc = 0x10b728f; continue;
 }
 0x10b728f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x116af70u32; cpu.esi = v; }
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b728f; continue; }
  pc = 0x10b72d7; continue;
 }
 0x10b72d7 => {
  { let v = 0x10b7a30u32; cpu.push(v); }
  cpu.push(0x10b72e1); f_010adc40(cpu); if cpu.returned_to != 0x10b72e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b7379; continue; }
  pc = 0x10b72f0; continue;
 }
 0x10b72f0 => {
  { let v: u32 = 0x116af78u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b72fc; continue;
 }
 0x10b72fc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b72fc; continue; }
  pc = 0x10b7306; continue;
 }
 0x10b7306 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b731b; continue; }
  pc = 0x10b7312; continue;
 }
 0x10b7312 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b7318); f_010ae280(cpu); if cpu.returned_to != 0x10b7318 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b731b; continue;
 }
 0x10b731b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b732d); f_010ae130(cpu); if cpu.returned_to != 0x10b732d { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b734b); f_010b4890(cpu); if cpu.returned_to != 0x10b734b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b7357); f_010b4960(cpu); if cpu.returned_to != 0x10b7357 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b7363); f_010b4960(cpu); if cpu.returned_to != 0x10b7363 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b736c); f_010b4960(cpu); if cpu.returned_to != 0x10b736c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b7379 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b7380); f_010adb30(cpu); if cpu.returned_to != 0x10b7380 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d2120u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b7392); f_010ad9b0(cpu); if cpu.returned_to != 0x10b7392 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b739e); f_010ad760(cpu); if cpu.returned_to != 0x10b739e { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x10b73e0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b73b7); f_010c5dc6(cpu); if cpu.returned_to != 0x10b73b7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b73c2); f_010c5d33(cpu); if cpu.returned_to != 0x10b73c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b73cd); f_010c5d33(cpu); if cpu.returned_to != 0x10b73cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x10b73d2; continue;
 }
 0x10b73d2 => {
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
pub fn f_010b73e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b73e0); }
 let mut pc: u32 = 0x10b73e0;
 loop { match pc {
 0x10b73e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b7436; continue; }
  pc = 0x10b73ef; continue;
 }
 0x10b73ef => {
  if cpu.zf { pc = 0x10b740d; continue; }
  pc = 0x10b73f1; continue;
 }
 0x10b73f1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b7403); f_010ae130(cpu); if cpu.returned_to != 0x10b7403 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b740d; continue;
 }
 0x10b740d => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b7413); f_010c5dc6(cpu); if cpu.returned_to != 0x10b7413 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b741e); f_010c5d33(cpu); if cpu.returned_to != 0x10b741e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b7429); f_010c5d33(cpu); if cpu.returned_to != 0x10b7429 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x10b7433); f_010ae080(cpu); if cpu.returned_to != 0x10b7433 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b7436; continue;
 }
 0x10b7436 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b743f); f_010ad500(cpu); if cpu.returned_to != 0x10b743f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b7453); f_010b4890(cpu); if cpu.returned_to != 0x10b7453 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b745f); f_010b4960(cpu); if cpu.returned_to != 0x10b745f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b746b); f_010b4960(cpu); if cpu.returned_to != 0x10b746b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b7474); f_010b4960(cpu); if cpu.returned_to != 0x10b7474 { return; }
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
pub fn f_010b7480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b7480); }
 let mut pc: u32 = 0x10b7480;
 loop { match pc {
 0x10b7480 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x10b7522; continue; }
  pc = 0x10b74a9; continue;
 }
 0x10b74a9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b74ce; continue; }
  pc = 0x10b74c7; continue;
 }
 0x10b74c7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b74ce; continue;
 }
 0x10b74ce => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b74e1; continue; }
  pc = 0x10b74d8; continue;
 }
 0x10b74d8 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b74e1; continue;
 }
 0x10b74e1 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b74f5; continue; }
  pc = 0x10b74ec; continue;
 }
 0x10b74ec => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b74f5; continue;
 }
 0x10b74f5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b7509; continue; }
  pc = 0x10b7500; continue;
 }
 0x10b7500 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b7509; continue;
 }
 0x10b7509 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b750b; continue;
 }
 0x10b750b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x10b751a; continue; }
  pc = 0x10b7512; continue;
 }
 0x10b7512 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x10b751a; continue;
 }
 0x10b751a => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10b750b; continue; }
  pc = 0x10b7522; continue;
 }
 0x10b7522 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x12d2144u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10b7559; continue; }
  pc = 0x10b753e; continue;
 }
 0x10b753e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x10b7573; continue;
 }
 0x10b7559 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x10b7573; continue;
 }
 0x10b7573 => {
  cpu.push(0x10b7578); f_010ae130(cpu); if cpu.returned_to != 0x10b7578 { return; }
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
pub fn f_010b7590(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b7590); } r_010b7590(cpu, 0x10b7590); }
pub fn f_010b75a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b75a0); } r_010b7590(cpu, 0x10b75a0); }
pub fn f_010b75b0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b75b0); } r_010b7590(cpu, 0x10b75b0); }
fn r_010b7590(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b7590 => {
  cpu.unimplemented(0x10b7590, "Arpl");
  { let v: u32 = cpu.add32(cpu.r32(0xa7010b76u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0xa7010b76u32.wrapping_add(cpu.ebp), v); }
  if (cpu.cf || cpu.zf) { pc = 0x10b75a6; continue; }
  pc = 0x10b759b; continue;
 }
 0x10b759b => {
  { let v: u32 = cpu.add32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.cf || cpu.zf) { pc = 0x10b75aa; continue; }
  pc = 0x10b759f; continue;
 }
 0x10b759f => {
  { let v: u32 = cpu.add32(cpu.r32(0x4d010b78u32), cpu.ebp); cpu.w32(0x4d010b78u32, v); }
  if cpu.sf { pc = 0x10b75b2; continue; }
  pc = 0x10b75a7; continue;
 }
 0x10b75a0 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x4d010b78u32); cpu.eax = v; }
  if cpu.sf { pc = 0x10b75b2; continue; }
  pc = 0x10b75a7; continue;
 }
 0x10b75a6 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x56010b78u32); }
  if cpu.sf { pc = 0x10b75ba; continue; }
  pc = 0x10b75af; continue;
 }
 0x10b75a7 => {
  { let v: u32 = cpu.add32(cpu.r32(0x56010b78u32), cpu.edi); cpu.w32(0x56010b78u32, v); }
  if cpu.sf { pc = 0x10b75ba; continue; }
  pc = 0x10b75af; continue;
 }
 0x10b75aa => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  if cpu.sf { pc = 0x10b75ba; continue; }
  pc = 0x10b75af; continue;
 }
 0x10b75af => {
  { let v: u32 = cpu.r32(0x56u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.edx); cpu.w32(0x56u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b75b2; continue;
 }
 0x10b75b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x10b75b2; continue;
 }
 0x10b75b2 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b7643; continue; }
  pc = 0x10b75ca; continue;
 }
 0x10b75ba => {
  { cpu.sub8(cpu.r8(0x44895846u32.wrapping_add(cpu.ebx)), (cpu.ecx as u8)); }
  { let v: u8 = (cpu.eax as u8) & 0x1cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b7643; continue; }
  pc = 0x10b75ca; continue;
 }
 0x10b75ca => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b7602; continue; }
  pc = 0x10b75d3; continue;
 }
 0x10b75d3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b75d5; continue;
 }
 0x10b75d5 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8a4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x4u32); }
  if !cpu.zf { pc = 0x10b75f5; continue; }
  pc = 0x10b75ee; continue;
 }
 0x10b75ee => {
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10b75f5; continue;
 }
 0x10b75f5 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b75d5; continue; }
  pc = 0x10b7602; continue;
 }
 0x10b7602 => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b7624; continue; }
  pc = 0x10b7608; continue;
 }
 0x10b7608 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b761a); f_010ae130(cpu); if cpu.returned_to != 0x10b761a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7624; continue;
 }
 0x10b7624 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b762a); f_010c5dc6(cpu); if cpu.returned_to != 0x10b762a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b7635); f_010c5d33(cpu); if cpu.returned_to != 0x10b7635 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b7640); f_010c5d33(cpu); if cpu.returned_to != 0x10b7640 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10b7643; continue;
 }
 0x10b7643 => {
  { let v: u32 = cpu.r32(0x12d213cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d2140u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b76e9; continue; }
  pc = 0x10b765b; continue;
 }
 0x10b765b => {
  cpu.poll(); pc = cpu.r32(0x10b7590u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x10b7663 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b76e9; continue;
 }
 0x10b7685 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b76e9; continue;
 }
 0x10b76a7 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b76e9; continue;
 }
 0x10b76c9 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10b76e9; continue;
 }
 0x10b76e9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b76ee; continue;
 }
 0x10b76ee => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x10b76fc; continue; }
  pc = 0x10b76f7; continue;
 }
 0x10b76f7 => {
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7704; continue;
 }
 0x10b76fc => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b76ee; continue; }
  pc = 0x10b7704; continue;
 }
 0x10b7704 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x12d2124u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x10b772d; continue; }
  pc = 0x10b7724; continue;
 }
 0x10b7724 => {
  if (cpu.cf || cpu.zf) { pc = 0x10b774f; continue; }
  pc = 0x10b7726; continue;
 }
 0x10b7726 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x10b776d; continue; }
  pc = 0x10b772b; continue;
 }
 0x10b772b => {
  pc = 0x10b778b; continue;
 }
 0x10b772d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b778b; continue; }
  pc = 0x10b7731; continue;
 }
 0x10b7731 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x10b781a; continue;
 }
 0x10b774f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x10b781a; continue;
 }
 0x10b776d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x10b781a; continue;
 }
 0x10b778b => {
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
  { let v: u32 = cpu.r32(0x12d2124u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b77f0; continue;
 }
 0x10b77f0 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b77fb; continue; }
  pc = 0x10b77f9; continue;
 }
 0x10b77f9 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10b77fb; continue;
 }
 0x10b77fb => {
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b780e; continue; }
  pc = 0x10b77ff; continue;
 }
 0x10b77ff => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b780c; continue; }
  pc = 0x10b780a; continue;
 }
 0x10b780a => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10b780c; continue;
 }
 0x10b780c => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x10b780e; continue;
 }
 0x10b780e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b77f0; continue; }
  pc = 0x10b7817; continue;
 }
 0x10b7817 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  pc = 0x10b781a; continue;
 }
 0x10b781a => {
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b7864; continue; }
  pc = 0x10b7825; continue;
 }
 0x10b7825 => {
  cpu.poll(); pc = cpu.r32(0x10b75a0u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x10b782d => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7864; continue;
 }
 0x10b783d => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7864; continue;
 }
 0x10b784d => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b785d; continue;
 }
 0x10b7856 => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b785d; continue;
 }
 0x10b785d => {
  { let v: u32 = 0x2u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7864; continue;
 }
 0x10b7864 => {
  { let v: u32 = cpu.r32(0x12d212cu32); cpu.eax = v; }
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
  { let v: u32 = cpu.r32(0x12d2168u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b78e9; continue; }
  pc = 0x10b78b5; continue;
 }
 0x10b78b5 => {
  { cpu.sub32(cpu.ebp, 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b78c2; continue; }
  pc = 0x10b78bd; continue;
 }
 0x10b78bd => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x10b78c2; continue;
 }
 0x10b78c2 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b78e9; continue; }
  pc = 0x10b78cc; continue;
 }
 0x10b78cc => {
  { let v: u32 = 0x116ae6cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b78d8; continue;
 }
 0x10b78d8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b78d8; continue; }
  pc = 0x10b78e2; continue;
 }
 0x10b78e2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b7a27; continue;
 }
 0x10b78e9 => {
  { let v: u32 = 0x7fffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edi = v; }
  pc = 0x10b7915; continue;
 }
 0x10b78f5 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b7900; continue; }
  pc = 0x10b78fe; continue;
 }
 0x10b78fe => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10b7900; continue;
 }
 0x10b7900 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b7913; continue; }
  pc = 0x10b7904; continue;
 }
 0x10b7904 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b790f; continue; }
  pc = 0x10b790d; continue;
 }
 0x10b790d => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10b790f; continue;
 }
 0x10b790f => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x10b7913; continue;
 }
 0x10b7913 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  pc = 0x10b7915; continue;
 }
 0x10b7915 => {
  { let v: u32 = cpu.r32(0x12d2128u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b78f5; continue; }
  pc = 0x10b7927; continue;
 }
 0x10b7927 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x10b7935; continue; }
  pc = 0x10b7932; continue;
 }
 0x10b7932 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7935; continue;
 }
 0x10b7935 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x14u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { pc = 0x10b793d; continue; }
  pc = 0x10b793a; continue;
 }
 0x10b793a => {
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b793d; continue;
 }
 0x10b793d => {
  { let v: u32 = cpu.r32(0x12d2168u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b795a; continue; }
  pc = 0x10b794a; continue;
 }
 0x10b794a => {
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xff000u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 12u32); cpu.eax = v; }
  pc = 0x10b795a; continue;
 }
 0x10b795a => {
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
  if cpu.zf { pc = 0x10b7a22; continue; }
  pc = 0x10b79a6; continue;
 }
 0x10b79a6 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b79cc; continue; }
  pc = 0x10b79af; continue;
 }
 0x10b79af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b79b1; continue;
 }
 0x10b79b1 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8a4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b79b1; continue; }
  pc = 0x10b79cc; continue;
 }
 0x10b79cc => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b79d2); f_010c5dc6(cpu); if cpu.returned_to != 0x10b79d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b79dd); f_010c5d33(cpu); if cpu.returned_to != 0x10b79dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b79e8); f_010c5d33(cpu); if cpu.returned_to != 0x10b79e8 { return; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10b7a22; continue; }
  pc = 0x10b79f2; continue;
 }
 0x10b79f2 => {
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
  cpu.push(0x10b7a18); f_010ae130(cpu); if cpu.returned_to != 0x10b7a18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b7a22; continue;
 }
 0x10b7a22 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10b7a27; continue;
 }
 0x10b7a27 => {
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
pub fn f_010b7a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b7a30); }
 let mut pc: u32 = 0x10b7a30;
 loop { match pc {
 0x10b7a30 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x117c89cu32), 0x0u32); }
  if !cpu.zf { pc = 0x10b7c2d; continue; }
  pc = 0x10b7a46; continue;
 }
 0x10b7a46 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d2148u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c89cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21b8u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b7a6e; continue; }
  pc = 0x10b7a64; continue;
 }
 0x10b7a64 => {
  cpu.push(0x10b7a69); f_010ae270(cpu); if cpu.returned_to != 0x10b7a69 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21c0u32, v); }
  pc = 0x10b7a6e; continue;
 }
 0x10b7a6e => {
  { let v: u32 = cpu.r32(0x12d21b8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x10b7aab; continue; }
  pc = 0x10b7a80; continue;
 }
 0x10b7a80 => {
  { cpu.sub32(cpu.r32(0x12d2148u32), 0x0u32); }
  if cpu.zf { pc = 0x10b7a98; continue; }
  pc = 0x10b7a89; continue;
 }
 0x10b7a89 => {
  { let v: u32 = cpu.r32(0x12d21c0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b7a95); f_010ae260(cpu); if cpu.returned_to != 0x10b7a95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b7a98; continue;
 }
 0x10b7a98 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c89cu32, v); }
  pc = 0x10b7c2d; continue;
 }
 0x10b7aab => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d21bcu32, v); }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b7ac1; continue; }
  pc = 0x10b7abc; continue;
 }
 0x10b7abc => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10b7afa; continue; }
  pc = 0x10b7ac1; continue;
 }
 0x10b7ac1 => {
  { let v: u32 = cpu.r32(0x12d2148u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21b4u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b7ae0; continue; }
  pc = 0x10b7ad1; continue;
 }
 0x10b7ad1 => {
  { let v: u32 = cpu.r32(0x12d21c0u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b7add); f_010ae260(cpu); if cpu.returned_to != 0x10b7add { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b7ae0; continue;
 }
 0x10b7ae0 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x117c89cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21b4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x10b7afa => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b7b03); f_010c5dc6(cpu); if cpu.returned_to != 0x10b7b03 { return; }
  { let v: u32 = cpu.r32(0x12d21b8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d21c4u32, v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21b4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21b0u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b7b61; continue; }
  pc = 0x10b7b28; continue;
 }
 0x10b7b28 => {
  { let v: u32 = cpu.r32(0x12d21b0u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x10b7b3e; continue; }
  pc = 0x10b7b33; continue;
 }
 0x10b7b33 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x12d21c4u32)); cpu.w32(0x12d21c4u32, v); }
  cpu.push(0x10b7b3e); f_010b7f30(cpu); if cpu.returned_to != 0x10b7b3e { return; }
  pc = 0x10b7b3e; continue;
 }
 0x10b7b3e => {
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x12d21b0u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21b4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x12d21b0u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x10b7b28; continue; }
  pc = 0x10b7b61; continue;
 }
 0x10b7b61 => {
  { let v: u32 = cpu.r32(0x12d21b8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21c4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d21bcu32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b7b7f); f_010c5d33(cpu); if cpu.returned_to != 0x10b7b7f { return; }
  { let v: u32 = cpu.r32(0x12d21b8u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x10b7bc1; continue; }
  pc = 0x10b7b8f; continue;
 }
 0x10b7b8f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x12d21a4u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x12d21bcu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = 0x405u32; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x12d21a8u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x12d21aau32, v); }
  cpu.push(0x10b7bbe); f_010ae130(cpu); if cpu.returned_to != 0x10b7bbe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10b7bc1; continue;
 }
 0x10b7bc1 => {
  { let v: u32 = cpu.r32(0x12d21b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b7bdd; continue; }
  pc = 0x10b7bd4; continue;
 }
 0x10b7bd4 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b7c07; continue;
 }
 0x10b7bdd => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x10b7c07; continue; }
  pc = 0x10b7beb; continue;
 }
 0x10b7beb => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b7c07; continue; }
  pc = 0x10b7bf1; continue;
 }
 0x10b7bf1 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b7c01); f_010ae130(cpu); if cpu.returned_to != 0x10b7c01 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b7c07; continue;
 }
 0x10b7c07 => {
  { cpu.sub32(cpu.r32(0x12d2148u32), 0x0u32); }
  if cpu.zf { pc = 0x10b7c1f; continue; }
  pc = 0x10b7c10; continue;
 }
 0x10b7c10 => {
  { let v: u32 = cpu.r32(0x12d21c0u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b7c1c); f_010ae260(cpu); if cpu.returned_to != 0x10b7c1c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b7c1f; continue;
 }
 0x10b7c1f => {
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x12d21b4u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x117c89cu32, v); }
  pc = 0x10b7c2d; continue;
 }
 0x10b7c2d => {
  { let v: u32 = cpu.esi; cpu.w32(0x12d21b4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b7c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b7c40); }
 let mut pc: u32 = 0x10b7c40;
 loop { match pc {
 0x10b7c40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b7c5c; continue; }
  pc = 0x10b7c53; continue;
 }
 0x10b7c53 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b7c67; continue;
 }
 0x10b7c5c => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10b7c67; continue; }
  pc = 0x10b7c60; continue;
 }
 0x10b7c60 => {
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b7c67; continue;
 }
 0x10b7c67 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b7c78; continue; }
  pc = 0x10b7c6f; continue;
 }
 0x10b7c6f => {
  { let v: u32 = 0x7fu32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b7c83; continue;
 }
 0x10b7c78 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x10b7c83; continue; }
  pc = 0x10b7c7c; continue;
 }
 0x10b7c7c => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b7c83; continue;
 }
 0x10b7c83 => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b7cb2; continue; }
  pc = 0x10b7cac; continue;
 }
 0x10b7cac => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b7cb2; continue;
 }
 0x10b7cb2 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b7cc1; continue; }
  pc = 0x10b7cb9; continue;
 }
 0x10b7cb9 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b7cc1; continue;
 }
 0x10b7cc1 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x10b7cce; continue; }
  pc = 0x10b7cc9; continue;
 }
 0x10b7cc9 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x10b7d17; continue; }
  pc = 0x10b7cce; continue;
 }
 0x10b7cce => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x117c81cu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x117c81cu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x458u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b7f27; continue;
 }
 0x10b7d17 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b7d26; continue; }
  pc = 0x10b7d1f; continue;
 }
 0x10b7d1f => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b7d26; continue;
 }
 0x10b7d26 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x10b7d54; continue; }
  pc = 0x10b7d31; continue;
 }
 0x10b7d31 => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x10b7d54; continue; }
  pc = 0x10b7d36; continue;
 }
 0x10b7d36 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b7d43; continue; }
  pc = 0x10b7d3a; continue;
 }
 0x10b7d3a => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x10b7e8c; continue; }
  pc = 0x10b7d43; continue;
 }
 0x10b7d43 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x10b7d54; continue; }
  pc = 0x10b7d4b; continue;
 }
 0x10b7d4b => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x10b7e8c; continue; }
  pc = 0x10b7d54; continue;
 }
 0x10b7d54 => {
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x458u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x117c81cu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x117c81cu32.wrapping_add(cpu.edi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x48u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x10b7e2f; continue; }
  pc = 0x10b7d8c; continue;
 }
 0x10b7d8c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0x0u32); cpu.ecx = v; }
  pc = 0x10b7d9d; continue;
 }
 0x10b7d9d => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b7d9d; continue; }
  pc = 0x10b7dd3; continue;
 }
 0x10b7dd3 => {
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
  pc = 0x10b7df7; continue;
 }
 0x10b7df7 => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b7f27; continue; }
  pc = 0x10b7e2d; continue;
 }
 0x10b7e2d => {
  cpu.poll(); pc = 0x10b7df7; continue;
 }
 0x10b7e2f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  pc = 0x10b7e50; continue;
 }
 0x10b7e50 => {
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
  if (cpu.sf == cpu.of) { pc = 0x10b7f27; continue; }
  pc = 0x10b7e8a; continue;
 }
 0x10b7e8a => {
  cpu.poll(); pc = 0x10b7e50; continue;
 }
 0x10b7e8c => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10b7ef7; continue; }
  pc = 0x10b7e9e; continue;
 }
 0x10b7e9e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x0u32); cpu.edx = v; }
  pc = 0x10b7ea9; continue;
 }
 0x10b7ea9 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b7ea9; continue; }
  pc = 0x10b7ec4; continue;
 }
 0x10b7ec4 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x10b7ede; continue;
 }
 0x10b7ede => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x100u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x10b7f27; continue; }
  pc = 0x10b7ef5; continue;
 }
 0x10b7ef5 => {
  cpu.poll(); pc = 0x10b7ede; continue;
 }
 0x10b7ef7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x10b7f0c; continue;
 }
 0x10b7f0c => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b7f0c; continue; }
  pc = 0x10b7f27; continue;
 }
 0x10b7f27 => {
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
pub fn f_010b7f30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b7f30); }
 let mut pc: u32 = 0x10b7f30;
 loop { match pc {
 0x10b7f30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x10b8046; continue; }
  pc = 0x10b7f42; continue;
 }
 0x10b7f42 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x10b7f44; continue;
 }
 0x10b7f44 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x10b8046; continue; }
  pc = 0x10b7f50; continue;
 }
 0x10b7f50 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10b7f7e; continue; }
  pc = 0x10b7f68; continue;
 }
 0x10b7f68 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b7f7e; continue; }
  pc = 0x10b7f70; continue;
 }
 0x10b7f70 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.eax))); }
  if !cpu.cf { pc = 0x10b7f7e; continue; }
  pc = 0x10b7f75; continue;
 }
 0x10b7f75 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b7f7e; continue;
 }
 0x10b7f7e => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10b8024; continue; }
  pc = 0x10b7f86; continue;
 }
 0x10b7f86 => {
  { cpu.sub32(cpu.r32(0x860u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b7f96; continue; }
  pc = 0x10b7f8f; continue;
 }
 0x10b7f8f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x10b7f96); cpu.call(t); if cpu.returned_to != 0x10b7f96 { return; }
  pc = 0x10b7f96; continue;
 }
 0x10b7f96 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x10b7fba; continue; }
  pc = 0x10b7f9c; continue;
 }
 0x10b7f9c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x10b7f44; continue; }
  pc = 0x10b7fb2; continue;
 }
 0x10b7fb2 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x10b7f44; continue;
 }
 0x10b7fba => {
  { cpu.sub32(cpu.r32(0x858u32.wrapping_add(cpu.esi)), 0x2u32); }
  if !cpu.zf { pc = 0x10b7ffe; continue; }
  pc = 0x10b7fc3; continue;
 }
 0x10b7fc3 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10b7ffe; continue; }
  pc = 0x10b7fd8; continue;
 }
 0x10b7fd8 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10b7fe7; continue; }
  pc = 0x10b7fde; continue;
 }
 0x10b7fde => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b8046; continue;
 }
 0x10b7fe7 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x10b7ff6; continue; }
  pc = 0x10b7fed; continue;
 }
 0x10b7fed => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b8046; continue;
 }
 0x10b7ff6 => {
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x10b7f44; continue;
 }
 0x10b7ffe => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b8046; continue; }
  pc = 0x10b8016; continue;
 }
 0x10b8016 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x10b801d); cpu.call(t); if cpu.returned_to != 0x10b801d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b8024 => {
  { cpu.sub32(cpu.r32(0x85cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b8034; continue; }
  pc = 0x10b802d; continue;
 }
 0x10b802d => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x85cu32.wrapping_add(cpu.esi)); cpu.push(0x10b8034); cpu.call(t); if cpu.returned_to != 0x10b8034 { return; }
  pc = 0x10b8034; continue;
 }
 0x10b8034 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b803c); f_010c5bd0(cpu); if cpu.returned_to != 0x10b803c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.poll(); pc = 0x10b7f44; continue;
 }
 0x10b8046 => {
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
pub fn f_010b8050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8050); }
 let mut pc: u32 = 0x10b8050;
 loop { match pc {
 0x10b8050 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b8077; continue; }
  pc = 0x10b805b; continue;
 }
 0x10b805b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b806d); f_010ae130(cpu); if cpu.returned_to != 0x10b806d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b8077; continue;
 }
 0x10b8077 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b80d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b80d0); }
 let mut pc: u32 = 0x10b80d0;
 loop { match pc {
 0x10b80d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c8a0u32), 0x0u32); }
  if !cpu.zf { pc = 0x10b818c; continue; }
  pc = 0x10b80e0; continue;
 }
 0x10b80e0 => {
  { let v = 0x10b6350u32; cpu.push(v); }
  { let v = 0x10b80d0u32; cpu.push(v); }
  cpu.push(0x10b80ef); f_010b47f0(cpu); if cpu.returned_to != 0x10b80ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x117c81cu32; cpu.push(v); }
  cpu.push(0x10b8101); f_010b4730(cpu); if cpu.returned_to != 0x10b8101 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c89cu32; cpu.push(v); }
  cpu.push(0x10b8110); f_010b4730(cpu); if cpu.returned_to != 0x10b8110 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21b8u32; cpu.push(v); }
  cpu.push(0x10b811f); f_010b4730(cpu); if cpu.returned_to != 0x10b811f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x12d21a4u32; cpu.push(v); }
  cpu.push(0x10b812e); f_010b4730(cpu); if cpu.returned_to != 0x10b812e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21c0u32; cpu.push(v); }
  cpu.push(0x10b813d); f_010b4730(cpu); if cpu.returned_to != 0x10b813d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21c4u32; cpu.push(v); }
  cpu.push(0x10b814c); f_010b4730(cpu); if cpu.returned_to != 0x10b814c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21b4u32; cpu.push(v); }
  cpu.push(0x10b815b); f_010b4730(cpu); if cpu.returned_to != 0x10b815b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21bcu32; cpu.push(v); }
  cpu.push(0x10b816a); f_010b4730(cpu); if cpu.returned_to != 0x10b816a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21b0u32; cpu.push(v); }
  cpu.push(0x10b8179); f_010b4730(cpu); if cpu.returned_to != 0x10b8179 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10b8186); f_010c5df2(cpu); if cpu.returned_to != 0x10b8186 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c8a0u32, v); }
  pc = 0x10b818c; continue;
 }
 0x10b818c => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8190); }
 let mut pc: u32 = 0x10b8190;
 loop { match pc {
 0x10b8190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c8a4u32), 0x0u32); }
  if cpu.zf { pc = 0x10b81b6; continue; }
  pc = 0x10b819c; continue;
 }
 0x10b819c => {
  { let v = 0x10b8190u32; cpu.push(v); }
  { let v = 0x10b8970u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10b81ad); f_010b4750(cpu); if cpu.returned_to != 0x10b81ad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c8a4u32, v); }
  pc = 0x10b81b6; continue;
 }
 0x10b81b6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b81c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b81c0); }
 let mut pc: u32 = 0x10b81c0;
 loop { match pc {
 0x10b81c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b81db; continue; }
  pc = 0x10b81c9; continue;
 }
 0x10b81c9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x888u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x888u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10b81db; continue;
 }
 0x10b81db => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b81e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b81e0); }
 let mut pc: u32 = 0x10b81e0;
 loop { match pc {
 0x10b81e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b81f5; continue; }
  pc = 0x10b81ef; continue;
 }
 0x10b81ef => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b81f5 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x116aff8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b8201; continue;
 }
 0x10b8201 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b820f; continue; }
  pc = 0x10b8209; continue;
 }
 0x10b8209 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b820f; continue; }
  pc = 0x10b820d; continue;
 }
 0x10b820d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b820f; continue;
 }
 0x10b820f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b821c; continue; }
  pc = 0x10b8214; continue;
 }
 0x10b8214 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b821c; continue; }
  pc = 0x10b8219; continue;
 }
 0x10b8219 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b821c; continue;
 }
 0x10b821c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b822b; continue; }
  pc = 0x10b8220; continue;
 }
 0x10b8220 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b822b; continue; }
  pc = 0x10b8224; continue;
 }
 0x10b8224 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b822b; continue; }
  pc = 0x10b8227; continue;
 }
 0x10b8227 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b8201; continue;
 }
 0x10b822b => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b8232; continue; }
  pc = 0x10b8230; continue;
 }
 0x10b8230 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b8232; continue;
 }
 0x10b8232 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b823a; continue; }
  pc = 0x10b8236; continue;
 }
 0x10b8236 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b829d; continue;
 }
 0x10b823a => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b004u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x10b8247; continue;
 }
 0x10b8247 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8255; continue; }
  pc = 0x10b824f; continue;
 }
 0x10b824f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8255; continue; }
  pc = 0x10b8253; continue;
 }
 0x10b8253 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b8255; continue;
 }
 0x10b8255 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8262; continue; }
  pc = 0x10b825a; continue;
 }
 0x10b825a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8262; continue; }
  pc = 0x10b825f; continue;
 }
 0x10b825f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b8262; continue;
 }
 0x10b8262 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b8271; continue; }
  pc = 0x10b8266; continue;
 }
 0x10b8266 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b8271; continue; }
  pc = 0x10b826a; continue;
 }
 0x10b826a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b8271; continue; }
  pc = 0x10b826d; continue;
 }
 0x10b826d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b8247; continue;
 }
 0x10b8271 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b8278; continue; }
  pc = 0x10b8276; continue;
 }
 0x10b8276 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b8278; continue;
 }
 0x10b8278 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b8298; continue; }
  pc = 0x10b827c; continue;
 }
 0x10b827c => {
  { let v: u32 = 0x116b054u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b8288; continue;
 }
 0x10b8288 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b8288; continue; }
  pc = 0x10b8292; continue;
 }
 0x10b8292 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b8298 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10b829d; continue;
 }
 0x10b829d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x10b82a8; continue; }
  pc = 0x10b82a1; continue;
 }
 0x10b82a1 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10b82e5; continue; }
  pc = 0x10b82a6; continue;
 }
 0x10b82a6 => {
  pc = 0x10b82f9; continue;
 }
 0x10b82a8 => {
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
  cpu.push(0x10b82e3); f_010b86a8(cpu); if cpu.returned_to != 0x10b82e3 { return; }
  pc = 0x10b82f6; continue;
 }
 0x10b82e5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b82f6); f_010b8520(cpu); if cpu.returned_to != 0x10b82f6 { return; }
  pc = 0x10b82f6; continue;
 }
 0x10b82f6 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10b82f9; continue;
 }
 0x10b82f9 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.ebx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b831e; continue; }
  pc = 0x10b8302; continue;
 }
 0x10b8302 => {
  { let v: u32 = 0x116b07cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b830e; continue;
 }
 0x10b830e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b830e; continue; }
  pc = 0x10b8318; continue;
 }
 0x10b8318 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b831e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8330); }
 let mut pc: u32 = 0x10b8330;
 loop { match pc {
 0x10b8330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x117c8a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10b835e; continue; }
  pc = 0x10b8341; continue;
 }
 0x10b8341 => {
  { let v = 0x10b8190u32; cpu.push(v); }
  { let v = 0x10b8970u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10b8355); f_010b47f0(cpu); if cpu.returned_to != 0x10b8355 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c8a4u32, v); }
  pc = 0x10b835e; continue;
 }
 0x10b835e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x10b8369; continue; }
  pc = 0x10b8362; continue;
 }
 0x10b8362 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b8369 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x116aff8u32; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x10b8375; continue;
 }
 0x10b8375 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8383; continue; }
  pc = 0x10b837d; continue;
 }
 0x10b837d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8383; continue; }
  pc = 0x10b8381; continue;
 }
 0x10b8381 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b8383; continue;
 }
 0x10b8383 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8390; continue; }
  pc = 0x10b8388; continue;
 }
 0x10b8388 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8390; continue; }
  pc = 0x10b838d; continue;
 }
 0x10b838d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b8390; continue;
 }
 0x10b8390 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b839f; continue; }
  pc = 0x10b8394; continue;
 }
 0x10b8394 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b839f; continue; }
  pc = 0x10b8398; continue;
 }
 0x10b8398 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b839f; continue; }
  pc = 0x10b839b; continue;
 }
 0x10b839b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b8375; continue;
 }
 0x10b839f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b83a6; continue; }
  pc = 0x10b83a4; continue;
 }
 0x10b83a4 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b83a6; continue;
 }
 0x10b83a6 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b83ae; continue; }
  pc = 0x10b83aa; continue;
 }
 0x10b83aa => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b8412; continue;
 }
 0x10b83ae => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b004u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  pc = 0x10b83bb; continue;
 }
 0x10b83bb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b83c9; continue; }
  pc = 0x10b83c3; continue;
 }
 0x10b83c3 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b83c9; continue; }
  pc = 0x10b83c7; continue;
 }
 0x10b83c7 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b83c9; continue;
 }
 0x10b83c9 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b83d6; continue; }
  pc = 0x10b83ce; continue;
 }
 0x10b83ce => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b83d6; continue; }
  pc = 0x10b83d3; continue;
 }
 0x10b83d3 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b83d6; continue;
 }
 0x10b83d6 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b83e5; continue; }
  pc = 0x10b83da; continue;
 }
 0x10b83da => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b83e5; continue; }
  pc = 0x10b83de; continue;
 }
 0x10b83de => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b83e5; continue; }
  pc = 0x10b83e1; continue;
 }
 0x10b83e1 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b83bb; continue;
 }
 0x10b83e5 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b83ec; continue; }
  pc = 0x10b83ea; continue;
 }
 0x10b83ea => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b83ec; continue;
 }
 0x10b83ec => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b840d; continue; }
  pc = 0x10b83f0; continue;
 }
 0x10b83f0 => {
  { let v: u32 = 0x116b00cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b83fc; continue;
 }
 0x10b83fc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b83fc; continue; }
  pc = 0x10b8406; continue;
 }
 0x10b8406 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b840d => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10b8412; continue;
 }
 0x10b8412 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10b841c); f_010ace00(cpu); if cpu.returned_to != 0x10b841c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b84da; continue; }
  pc = 0x10b842b; continue;
 }
 0x10b842b => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b8431); f_010acb80(cpu); if cpu.returned_to != 0x10b8431 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x888u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x10b8449; continue; }
  pc = 0x10b8442; continue;
 }
 0x10b8442 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x10b8494; continue; }
  pc = 0x10b8447; continue;
 }
 0x10b8447 => {
  pc = 0x10b84b2; continue;
 }
 0x10b8449 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x898u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x10b8660u32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b8487); f_010aafb0(cpu); if cpu.returned_to != 0x10b8487 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8492); f_010b86a8(cpu); if cpu.returned_to != 0x10b8492 { return; }
  pc = 0x10b84af; continue;
 }
 0x10b8494 => {
  { let v = 0x10b84e0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b84a5); f_010aafb0(cpu); if cpu.returned_to != 0x10b84a5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b84af); f_010b8520(cpu); if cpu.returned_to != 0x10b84af { return; }
  pc = 0x10b84af; continue;
 }
 0x10b84af => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10b84b2; continue;
 }
 0x10b84b2 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b84d8; continue; }
  pc = 0x10b84bb; continue;
 }
 0x10b84bb => {
  { let v: u32 = 0x116b034u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b84c7; continue;
 }
 0x10b84c7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b84c7; continue; }
  pc = 0x10b84d1; continue;
 }
 0x10b84d1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b84d8 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b84da; continue;
 }
 0x10b84da => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b84e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b84e0); }
 let mut pc: u32 = 0x10b84e0;
 loop { match pc {
 0x10b84e0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b84f5; continue; }
  pc = 0x10b84ee; continue;
 }
 0x10b84ee => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x10b84f5); cpu.call(t); if cpu.returned_to != 0x10b84f5 { return; }
  pc = 0x10b84f5; continue;
 }
 0x10b84f5 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b8507; continue; }
  pc = 0x10b84fe; continue;
 }
 0x10b84fe => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8504); f_010acc30(cpu); if cpu.returned_to != 0x10b8504 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b8507; continue;
 }
 0x10b8507 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8520); }
 let mut pc: u32 = 0x10b8520;
 loop { match pc {
 0x10b8520 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x116afe8u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x10b8534; continue;
 }
 0x10b8534 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b853d; continue;
 }
 0x10b853d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b854b; continue; }
  pc = 0x10b8545; continue;
 }
 0x10b8545 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b854b; continue; }
  pc = 0x10b8549; continue;
 }
 0x10b8549 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b854b; continue;
 }
 0x10b854b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8558; continue; }
  pc = 0x10b8550; continue;
 }
 0x10b8550 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8558; continue; }
  pc = 0x10b8555; continue;
 }
 0x10b8555 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b8558; continue;
 }
 0x10b8558 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b8567; continue; }
  pc = 0x10b855c; continue;
 }
 0x10b855c => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b8567; continue; }
  pc = 0x10b8560; continue;
 }
 0x10b8560 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b8567; continue; }
  pc = 0x10b8563; continue;
 }
 0x10b8563 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b853d; continue;
 }
 0x10b8567 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b856e; continue; }
  pc = 0x10b856c; continue;
 }
 0x10b856c => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b856e; continue;
 }
 0x10b856e => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b8584; continue; }
  pc = 0x10b8572; continue;
 }
 0x10b8572 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b8534; continue;
 }
 0x10b8584 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x10b8598; continue; }
  pc = 0x10b858b; continue;
 }
 0x10b858b => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x10b8598; continue; }
  pc = 0x10b8592; continue;
 }
 0x10b8592 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x10b85d2; continue;
 }
 0x10b8598 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x10b85ac; continue; }
  pc = 0x10b859f; continue;
 }
 0x10b859f => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x10b85ac; continue; }
  pc = 0x10b85a6; continue;
 }
 0x10b85a6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x10b85d2; continue;
 }
 0x10b85ac => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x10b85c0; continue; }
  pc = 0x10b85b3; continue;
 }
 0x10b85b3 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x10b85c0; continue; }
  pc = 0x10b85ba; continue;
 }
 0x10b85ba => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x10b85d2; continue;
 }
 0x10b85c0 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x10b85db; continue; }
  pc = 0x10b85c7; continue;
 }
 0x10b85c7 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x10b85db; continue; }
  pc = 0x10b85ce; continue;
 }
 0x10b85ce => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x10b85d2; continue;
 }
 0x10b85d2 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b85d8); f_010ac8c0(cpu); if cpu.returned_to != 0x10b85d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10b85db; continue;
 }
 0x10b85db => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b85e5); f_010ac540(cpu); if cpu.returned_to != 0x10b85e5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x10b85ef; continue;
 }
 0x10b85ef => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116aff0u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b85fb; continue;
 }
 0x10b85fb => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8609; continue; }
  pc = 0x10b8603; continue;
 }
 0x10b8603 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8609; continue; }
  pc = 0x10b8607; continue;
 }
 0x10b8607 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b8609; continue;
 }
 0x10b8609 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b8616; continue; }
  pc = 0x10b860e; continue;
 }
 0x10b860e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b8616; continue; }
  pc = 0x10b8613; continue;
 }
 0x10b8613 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b8616; continue;
 }
 0x10b8616 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b8625; continue; }
  pc = 0x10b861a; continue;
 }
 0x10b861a => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b8625; continue; }
  pc = 0x10b861e; continue;
 }
 0x10b861e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10b8625; continue; }
  pc = 0x10b8621; continue;
 }
 0x10b8621 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b85fb; continue;
 }
 0x10b8625 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b862c; continue; }
  pc = 0x10b862a; continue;
 }
 0x10b862a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b862c; continue;
 }
 0x10b862c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10b8642; continue; }
  pc = 0x10b8630; continue;
 }
 0x10b8630 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x10b85ef; continue;
 }
 0x10b8642 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b8650); f_010ac990(cpu); if cpu.returned_to != 0x10b8650 { return; }
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
pub fn f_010b8660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8660); }
 let mut pc: u32 = 0x10b8660;
 loop { match pc {
 0x10b8660 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b866c); f_010b86a8(cpu); if cpu.returned_to != 0x10b866c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8680); }
 let mut pc: u32 = 0x10b8680;
 loop { match pc {
 0x10b8680 => {
  { let v: u32 = cpu.rol32(cpu.r32(0x86e1010bu32.wrapping_add(cpu.esi)), (cpu.ecx & 0xff)); cpu.w32(0x86e1010bu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x88f7010bu32.wrapping_add(cpu.eax)), 0x874a010bu32); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.and32(cpu.r32(0x8781010bu32.wrapping_add(cpu.eax)), 0x8798010bu32); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(2955);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b86a8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b86a8); }
 let mut pc: u32 = 0x10b86a8;
 loop { match pc {
 0x10b86a8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10b86b6; continue;
 }
 0x10b86b6 => {
  { let v: u32 = cpu.r32(0x88cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b88f7; continue; }
  pc = 0x10b86c6; continue;
 }
 0x10b86c6 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x10b8680u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x10b86d3 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b86d9); f_010b8920(cpu); if cpu.returned_to != 0x10b86d9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b86e1 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b88f7; continue; }
  pc = 0x10b86ee; continue;
 }
 0x10b86ee => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b86f4); f_010b8960(cpu); if cpu.returned_to != 0x10b86f4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8705); f_010ac990(cpu); if cpu.returned_to != 0x10b8705 { return; }
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
  cpu.push(0x10b8724); f_010ac540(cpu); if cpu.returned_to != 0x10b8724 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8731); f_010ac8c0(cpu); if cpu.returned_to != 0x10b8731 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b88f2; continue; }
  pc = 0x10b873c; continue;
 }
 0x10b873c => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8742); f_010ac810(cpu); if cpu.returned_to != 0x10b8742 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b88f2; continue;
 }
 0x10b874a => {
  { let v: u32 = cpu.r32(0x898u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b88f7; continue; }
  pc = 0x10b8759; continue;
 }
 0x10b8759 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x10b8772; continue; }
  pc = 0x10b8763; continue;
 }
 0x10b8763 => {
  { let v: u32 = 0x1u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b88f7; continue;
 }
 0x10b8772 => {
  { let v: u32 = 0x0u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b88f7; continue;
 }
 0x10b8781 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x890u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b88f7; continue;
 }
 0x10b8798 => {
  { let v: u32 = cpu.r32(0x894u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffu32); }
  if cpu.zf { pc = 0x10b87b7; continue; }
  pc = 0x10b87a6; continue;
 }
 0x10b87a6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b88f7; continue; }
  pc = 0x10b87b7; continue;
 }
 0x10b87b7 => {
  { let v: u32 = cpu.r32(0x890u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x10b88f7; continue;
 }
 0x10b87c2 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b88f7; continue; }
  pc = 0x10b87cf; continue;
 }
 0x10b87cf => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x10b87f5; continue; }
  pc = 0x10b87d5; continue;
 }
 0x10b87d5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b87df); f_010ac8c0(cpu); if cpu.returned_to != 0x10b87df { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7a12000u32; cpu.eax = v; }
  pc = 0x10b8813; continue;
 }
 0x10b87f5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b87ff); f_010ac8c0(cpu); if cpu.returned_to != 0x10b87ff { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xf424000u32; cpu.eax = v; }
  pc = 0x10b8813; continue;
 }
 0x10b8813 => {
  { let v = cpu.edi; cpu.div32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b881f); f_010ac540(cpu); if cpu.returned_to != 0x10b881f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8832); f_010b8960(cpu); if cpu.returned_to != 0x10b8832 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8843); f_010ac990(cpu); if cpu.returned_to != 0x10b8843 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b88f2; continue; }
  pc = 0x10b884e; continue;
 }
 0x10b884e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8854); f_010ac810(cpu); if cpu.returned_to != 0x10b8854 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b88f2; continue;
 }
 0x10b885c => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10b88f7; continue; }
  pc = 0x10b8869; continue;
 }
 0x10b8869 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b886f); f_010b8960(cpu); if cpu.returned_to != 0x10b886f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b8880); f_010ac990(cpu); if cpu.returned_to != 0x10b8880 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b888d); f_010ac540(cpu); if cpu.returned_to != 0x10b888d { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x10b88a5; continue; }
  pc = 0x10b8898; continue;
 }
 0x10b8898 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x10b88a5; continue; }
  pc = 0x10b889f; continue;
 }
 0x10b889f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x10b88dc; continue;
 }
 0x10b88a5 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x10b88b8; continue; }
  pc = 0x10b88ab; continue;
 }
 0x10b88ab => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x10b88b8; continue; }
  pc = 0x10b88b2; continue;
 }
 0x10b88b2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x10b88dc; continue;
 }
 0x10b88b8 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x10b88cb; continue; }
  pc = 0x10b88be; continue;
 }
 0x10b88be => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x10b88cb; continue; }
  pc = 0x10b88c5; continue;
 }
 0x10b88c5 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x10b88dc; continue;
 }
 0x10b88cb => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x10b88e5; continue; }
  pc = 0x10b88d1; continue;
 }
 0x10b88d1 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x10b88e5; continue; }
  pc = 0x10b88d8; continue;
 }
 0x10b88d8 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x10b88dc; continue;
 }
 0x10b88dc => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b88e2); f_010ac8c0(cpu); if cpu.returned_to != 0x10b88e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10b88e5; continue;
 }
 0x10b88e5 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b88f2; continue; }
  pc = 0x10b88e9; continue;
 }
 0x10b88e9 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b88ef); f_010ac810(cpu); if cpu.returned_to != 0x10b88ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b88f2; continue;
 }
 0x10b88f2 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x10b88f7; continue;
 }
 0x10b88f7 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b88fd); f_010b8960(cpu); if cpu.returned_to != 0x10b88fd { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x10b86b6; continue; }
  pc = 0x10b8913; continue;
 }
 0x10b8913 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8920); }
 let mut pc: u32 = 0x10b8920;
 loop { match pc {
 0x10b8920 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10b8936; continue; }
  pc = 0x10b892f; continue;
 }
 0x10b892f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x10b8936); cpu.call(t); if cpu.returned_to != 0x10b8936 { return; }
  pc = 0x10b8936; continue;
 }
 0x10b8936 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b8948; continue; }
  pc = 0x10b893f; continue;
 }
 0x10b893f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8945); f_010acc30(cpu); if cpu.returned_to != 0x10b8945 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b8948; continue;
 }
 0x10b8948 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8960); }
 let mut pc: u32 = 0x10b8960;
 loop { match pc {
 0x10b8960 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8970); }
 let mut pc: u32 = 0x10b8970;
 loop { match pc {
 0x10b8970 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c8a4u32), 0x0u32); }
  if !cpu.zf { pc = 0x10b8999; continue; }
  pc = 0x10b897c; continue;
 }
 0x10b897c => {
  { let v = 0x10b8190u32; cpu.push(v); }
  { let v = 0x10b8970u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10b8990); f_010b47f0(cpu); if cpu.returned_to != 0x10b8990 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c8a4u32, v); }
  pc = 0x10b8999; continue;
 }
 0x10b8999 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b89a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b89a0); }
 let mut pc: u32 = 0x10b89a0;
 loop { match pc {
 0x10b89a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c92cu32), 0x0u32); }
  if cpu.zf { pc = 0x10b8a9f; continue; }
  pc = 0x10b89b0; continue;
 }
 0x10b89b0 => {
  { let v = 0x10b89a0u32; cpu.push(v); }
  { let v = 0x10bc180u32; cpu.push(v); }
  cpu.push(0x10b89bf); f_010b4750(cpu); if cpu.returned_to != 0x10b89bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x117c8a8u32; cpu.push(v); }
  cpu.push(0x10b89d1); f_010b4710(cpu); if cpu.returned_to != 0x10b89d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c928u32; cpu.push(v); }
  cpu.push(0x10b89e0); f_010b4710(cpu); if cpu.returned_to != 0x10b89e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21dcu32; cpu.push(v); }
  cpu.push(0x10b89ef); f_010b4710(cpu); if cpu.returned_to != 0x10b89ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21e0u32; cpu.push(v); }
  cpu.push(0x10b89fe); f_010b4710(cpu); if cpu.returned_to != 0x10b89fe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21e8u32; cpu.push(v); }
  cpu.push(0x10b8a0d); f_010b4710(cpu); if cpu.returned_to != 0x10b8a0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21f8u32; cpu.push(v); }
  cpu.push(0x10b8a1c); f_010b4710(cpu); if cpu.returned_to != 0x10b8a1c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21ecu32; cpu.push(v); }
  cpu.push(0x10b8a2b); f_010b4710(cpu); if cpu.returned_to != 0x10b8a2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21f0u32; cpu.push(v); }
  cpu.push(0x10b8a3a); f_010b4710(cpu); if cpu.returned_to != 0x10b8a3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21d8u32; cpu.push(v); }
  cpu.push(0x10b8a49); f_010b4710(cpu); if cpu.returned_to != 0x10b8a49 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21f4u32; cpu.push(v); }
  cpu.push(0x10b8a58); f_010b4710(cpu); if cpu.returned_to != 0x10b8a58 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21d4u32; cpu.push(v); }
  cpu.push(0x10b8a67); f_010b4710(cpu); if cpu.returned_to != 0x10b8a67 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21d0u32; cpu.push(v); }
  cpu.push(0x10b8a76); f_010b4710(cpu); if cpu.returned_to != 0x10b8a76 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21c8u32; cpu.push(v); }
  cpu.push(0x10b8a85); f_010b4710(cpu); if cpu.returned_to != 0x10b8a85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21ccu32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10b8a96); f_010b4710(cpu); if cpu.returned_to != 0x10b8a96 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c92cu32, v); }
  pc = 0x10b8a9f; continue;
 }
 0x10b8a9f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8ab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8ab0); }
 let mut pc: u32 = 0x10b8ab0;
 loop { match pc {
 0x10b8ab0 => {
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
  cpu.push(0x10b8ad4); f_010bbe50(cpu); if cpu.returned_to != 0x10b8ad4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x10b8ae8); f_010bc0d0(cpu); if cpu.returned_to != 0x10b8ae8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10b8afa; continue; }
  pc = 0x10b8af3; continue;
 }
 0x10b8af3 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x10b8afc; continue;
 }
 0x10b8afa => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x10b8afc; continue;
 }
 0x10b8afc => {
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
  cpu.push(0x10b8b2e); f_010bc0d0(cpu); if cpu.returned_to != 0x10b8b2e { return; }
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
pub fn f_010b8c60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8c60); }
 let mut pc: u32 = 0x10b8c60;
 loop { match pc {
 0x10b8c60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b8c91; continue; }
  pc = 0x10b8c6d; continue;
 }
 0x10b8c6d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x10b8c91; continue; }
  pc = 0x10b8c89; continue;
 }
 0x10b8c89 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x10b8c91; continue; }
  pc = 0x10b8c8e; continue;
 }
 0x10b8c8e => {
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.edx), v); }
  pc = 0x10b8c91; continue;
 }
 0x10b8c91 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8ca0); }
 let mut pc: u32 = 0x10b8ca0;
 loop { match pc {
 0x10b8ca0 => {
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
  if !cpu.zf { pc = 0x10b8daa; continue; }
  pc = 0x10b8cc5; continue;
 }
 0x10b8cc5 => {
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
  cpu.push(0x10b8cee); f_010bbf50(cpu); if cpu.returned_to != 0x10b8cee { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b8d78; continue; }
  pc = 0x10b8d05; continue;
 }
 0x10b8d05 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b8d0e; continue;
 }
 0x10b8d0e => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x10b8d59; continue; }
  pc = 0x10b8d14; continue;
 }
 0x10b8d14 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x10b8d19; continue;
 }
 0x10b8d19 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b8d52; continue; }
  pc = 0x10b8d24; continue;
 }
 0x10b8d24 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x10b8d52; continue; }
  pc = 0x10b8d2f; continue;
 }
 0x10b8d2f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8d45); f_010bb4a0(cpu); if cpu.returned_to != 0x10b8d45 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b8d52; continue;
 }
 0x10b8d52 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10b8d19; continue; }
  pc = 0x10b8d59; continue;
 }
 0x10b8d59 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10b8d0e; continue; }
  pc = 0x10b8d78; continue;
 }
 0x10b8d78 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b8da3; continue; }
  pc = 0x10b8d8f; continue;
 }
 0x10b8d8f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x10b8da3; continue; }
  pc = 0x10b8d99; continue;
 }
 0x10b8d99 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8da0); f_010bb100(cpu); if cpu.returned_to != 0x10b8da0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10b8da3; continue;
 }
 0x10b8da3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b8daa; continue;
 }
 0x10b8daa => {
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
pub fn f_010b8dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8dc0); }
 let mut pc: u32 = 0x10b8dc0;
 loop { match pc {
 0x10b8dc0 => {
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
  pc = 0x10b8de9; continue;
 }
 0x10b8de9 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x10b8e08; continue; }
  pc = 0x10b8dee; continue;
 }
 0x10b8dee => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.zf { pc = 0x10b8e08; continue; }
  pc = 0x10b8df5; continue;
 }
 0x10b8df5 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x10b8e08; continue; }
  pc = 0x10b8dfa; continue;
 }
 0x10b8dfa => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b8e08; continue; }
  pc = 0x10b8e04; continue;
 }
 0x10b8e04 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b8e08; continue;
 }
 0x10b8e08 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b8de9; continue; }
  pc = 0x10b8e11; continue;
 }
 0x10b8e11 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b8e48; continue; }
  pc = 0x10b8e16; continue;
 }
 0x10b8e16 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x10b8e27; continue;
 }
 0x10b8e27 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x10b8e3f; continue; }
  pc = 0x10b8e2c; continue;
 }
 0x10b8e2c => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x24u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10b8e3f; continue; }
  pc = 0x10b8e31; continue;
 }
 0x10b8e31 => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b8e3f; continue; }
  pc = 0x10b8e3b; continue;
 }
 0x10b8e3b => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x10b8e3f; continue;
 }
 0x10b8e3f => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10b8e27; continue; }
  pc = 0x10b8e48; continue;
 }
 0x10b8e48 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b8e5f; continue; }
  pc = 0x10b8e4d; continue;
 }
 0x10b8e4d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b8f30; continue;
 }
 0x10b8e5f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8e72); f_010bbf50(cpu); if cpu.returned_to != 0x10b8e72 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b8ef6; continue; }
  pc = 0x10b8e87; continue;
 }
 0x10b8e87 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10b8e90; continue;
 }
 0x10b8e90 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x10b8ed7; continue; }
  pc = 0x10b8e96; continue;
 }
 0x10b8e96 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x10b8e9b; continue;
 }
 0x10b8e9b => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10b8ed0; continue; }
  pc = 0x10b8ea6; continue;
 }
 0x10b8ea6 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x10b8ed0; continue; }
  pc = 0x10b8eaf; continue;
 }
 0x10b8eaf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10b8ec3); f_010bb4a0(cpu); if cpu.returned_to != 0x10b8ec3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b8ed0; continue;
 }
 0x10b8ed0 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10b8e9b; continue; }
  pc = 0x10b8ed7; continue;
 }
 0x10b8ed7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b8e90; continue; }
  pc = 0x10b8ef6; continue;
 }
 0x10b8ef6 => {
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
  pc = 0x10b8f30; continue;
 }
 0x10b8f30 => {
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
pub fn f_010b8f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8f40); }
 let mut pc: u32 = 0x10b8f40;
 loop { match pc {
 0x10b8f40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b8f4f; continue; }
  pc = 0x10b8f48; continue;
 }
 0x10b8f48 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10b8f4f; continue;
 }
 0x10b8f4f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8f50); }
 let mut pc: u32 = 0x10b8f50;
 loop { match pc {
 0x10b8f50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b8f5b; continue; }
  pc = 0x10b8f58; continue;
 }
 0x10b8f58 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b8f5b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6dcu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8f70); }
 let mut pc: u32 = 0x10b8f70;
 loop { match pc {
 0x10b8f70 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b8f8b; continue; }
  pc = 0x10b8f78; continue;
 }
 0x10b8f78 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6dcu32.wrapping_add(cpu.eax), v); }
  pc = 0x10b8f8b; continue;
 }
 0x10b8f8b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b8f90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8f90); }
 let mut pc: u32 = 0x10b8f90;
 loop { match pc {
 0x10b8f90 => {
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
pub fn f_010b8fb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8fb0); }
 let mut pc: u32 = 0x10b8fb0;
 loop { match pc {
 0x10b8fb0 => {
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
pub fn f_010b8ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b8ff0); }
 let mut pc: u32 = 0x10b8ff0;
 loop { match pc {
 0x10b8ff0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b9005; continue; }
  pc = 0x10b8ff9; continue;
 }
 0x10b8ff9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10b9005; continue;
 }
 0x10b9005 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9050); }
 let mut pc: u32 = 0x10b9050;
 loop { match pc {
 0x10b9050 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10b90d7; continue; }
  pc = 0x10b9064; continue;
 }
 0x10b9064 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b90d7; continue; }
  pc = 0x10b906f; continue;
 }
 0x10b906f => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b9093; continue; }
  pc = 0x10b9080; continue;
 }
 0x10b9080 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x10b9082; continue;
 }
 0x10b9082 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.zf { pc = 0x10b9093; continue; }
  pc = 0x10b908b; continue;
 }
 0x10b908b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b9082; continue; }
  pc = 0x10b9093; continue;
 }
 0x10b9093 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x10b90d7; continue; }
  pc = 0x10b9097; continue;
 }
 0x10b9097 => {
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
  { let v: u32 = cpu.r32(0x12d215cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10b90d7; continue; }
  pc = 0x10b90c1; continue;
 }
 0x10b90c1 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edi); cpu.edx = v; }
  pc = 0x10b90c6; continue;
 }
 0x10b90c6 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b90c6; continue; }
  pc = 0x10b90d7; continue;
 }
 0x10b90d7 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b90e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b90e0); }
 let mut pc: u32 = 0x10b90e0;
 loop { match pc {
 0x10b90e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b914d; continue; }
  pc = 0x10b90ec; continue;
 }
 0x10b90ec => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d2158u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b912d; continue; }
  pc = 0x10b910b; continue;
 }
 0x10b910b => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10b9122; continue; }
  pc = 0x10b9117; continue;
 }
 0x10b9117 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x68u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x10b9122; continue; }
  pc = 0x10b911f; continue;
 }
 0x10b911f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10b9122; continue;
 }
 0x10b9122 => {
  { let v: u32 = cpu.r32(0x12d2158u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b910b; continue; }
  pc = 0x10b912d; continue;
 }
 0x10b912d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x10b9133; continue; }
  pc = 0x10b9131; continue;
 }
 0x10b9131 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b9133; continue;
 }
 0x10b9133 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b913d; continue; }
  pc = 0x10b913b; continue;
 }
 0x10b913b => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b913d; continue;
 }
 0x10b913d => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b9147; continue; }
  pc = 0x10b9145; continue;
 }
 0x10b9145 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10b9147; continue;
 }
 0x10b9147 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b914d; continue;
 }
 0x10b914d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
