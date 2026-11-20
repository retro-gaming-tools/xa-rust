#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_010b9160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9160); }
 let mut pc: u32 = 0x10b9160;
 loop { match pc {
 0x10b9160 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b916d; continue; }
  pc = 0x10b9169; continue;
 }
 0x10b9169 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b916d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x40u32); cpu.ecx = v; }
  pc = 0x10b9179; continue;
 }
 0x10b9179 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10b9182; continue; }
  pc = 0x10b9181; continue;
 }
 0x10b9181 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x10b9182; continue;
 }
 0x10b9182 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9179; continue; }
  pc = 0x10b9189; continue;
 }
 0x10b9189 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9450); }
 let mut pc: u32 = 0x10b9450;
 loop { match pc {
 0x10b9450 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9459; continue; }
  pc = 0x10b9458; continue;
 }
 0x10b9458 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9459 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9460); }
 let mut pc: u32 = 0x10b9460;
 loop { match pc {
 0x10b9460 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b94d9; continue; }
  pc = 0x10b946f; continue;
 }
 0x10b946f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x10b9490; continue; }
  pc = 0x10b9486; continue;
 }
 0x10b9486 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9490 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b9499; continue; }
  pc = 0x10b9494; continue;
 }
 0x10b9494 => {
  { let v: u32 = cpu.esi; cpu.w32(0x3cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b94c6; continue;
 }
 0x10b9499 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b94b3; continue; }
  pc = 0x10b94b1; continue;
 }
 0x10b94b1 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10b94b3; continue;
 }
 0x10b94b3 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b94c6; continue;
 }
 0x10b94c6 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b94cc); f_010bafe0(cpu); if cpu.returned_to != 0x10b94cc { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b94d9; continue;
 }
 0x10b94d9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b94e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b94e0); }
 let mut pc: u32 = 0x10b94e0;
 loop { match pc {
 0x10b94e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b9541; continue; }
  pc = 0x10b94ee; continue;
 }
 0x10b94ee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x10b953b; continue; }
  pc = 0x10b9505; continue;
 }
 0x10b9505 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b950e; continue; }
  pc = 0x10b9509; continue;
 }
 0x10b9509 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b953b; continue;
 }
 0x10b950e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x10b9528; continue; }
  pc = 0x10b9526; continue;
 }
 0x10b9526 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x10b9528; continue;
 }
 0x10b9528 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b953b; continue;
 }
 0x10b953b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b9541; continue;
 }
 0x10b9541 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9550); }
 let mut pc: u32 = 0x10b9550;
 loop { match pc {
 0x10b9550 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b955f; continue; }
  pc = 0x10b9558; continue;
 }
 0x10b9558 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b955f; continue;
 }
 0x10b955f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9560); }
 let mut pc: u32 = 0x10b9560;
 loop { match pc {
 0x10b9560 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b9592; continue; }
  pc = 0x10b956a; continue;
 }
 0x10b956a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x10b9592; continue; }
  pc = 0x10b9572; continue;
 }
 0x10b9572 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x10b9592; continue; }
  pc = 0x10b9577; continue;
 }
 0x10b9577 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b957d); f_010aa4d0(cpu); if cpu.returned_to != 0x10b957d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10b9592; continue; }
  pc = 0x10b958e; continue;
 }
 0x10b958e => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x10b9592); cpu.call(t); if cpu.returned_to != 0x10b9592 { return; }
  pc = 0x10b9592; continue;
 }
 0x10b9592 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b95a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b95a0); }
 let mut pc: u32 = 0x10b95a0;
 loop { match pc {
 0x10b95a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10b95fb; continue; }
  pc = 0x10b95ac; continue;
 }
 0x10b95ac => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x8u32); }
  if !cpu.zf { pc = 0x10b95fb; continue; }
  pc = 0x10b95b2; continue;
 }
 0x10b95b2 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10b95b9; continue;
 }
 0x10b95b9 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b95d9; continue; }
  pc = 0x10b95c2; continue;
 }
 0x10b95c2 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10b95c8); f_010a8570(cpu); if cpu.returned_to != 0x10b95c8 { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10b95d3; continue; }
  pc = 0x10b95d1; continue;
 }
 0x10b95d1 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10b95d3; continue;
 }
 0x10b95d3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.esi), v); }
  pc = 0x10b95d9; continue;
 }
 0x10b95d9 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b95b9; continue; }
  pc = 0x10b95e2; continue;
 }
 0x10b95e2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x10b95e4; continue;
 }
 0x10b95e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b95eb); f_010bb100(cpu); if cpu.returned_to != 0x10b95eb { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b95e4; continue; }
  pc = 0x10b95f4; continue;
 }
 0x10b95f4 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10b95fb; continue;
 }
 0x10b95fb => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9600); }
 let mut pc: u32 = 0x10b9600;
 loop { match pc {
 0x10b9600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b969c; continue; }
  pc = 0x10b9610; continue;
 }
 0x10b9610 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.zf { pc = 0x10b969c; continue; }
  pc = 0x10b961a; continue;
 }
 0x10b961a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x40u32); cpu.ebp = v; }
  cpu.push(0x10b962c); f_010bb3e0(cpu); if cpu.returned_to != 0x10b962c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x10b9632; continue;
 }
 0x10b9632 => {
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b9655; continue; }
  pc = 0x10b9643; continue;
 }
 0x10b9643 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b9652); f_010bbf50(cpu); if cpu.returned_to != 0x10b9652 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10b9655; continue;
 }
 0x10b9655 => {
  { cpu.sub32(cpu.r32(0x1d8u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b9666; continue; }
  pc = 0x10b965e; continue;
 }
 0x10b965e => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x10b9666; continue;
 }
 0x10b9666 => {
  { cpu.sub32(cpu.r32(0x258u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b9681; continue; }
  pc = 0x10b966f; continue;
 }
 0x10b966f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b967e); f_010bbf50(cpu); if cpu.returned_to != 0x10b967e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10b9681; continue;
 }
 0x10b9681 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10b9695; continue; }
  pc = 0x10b968a; continue;
 }
 0x10b968a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10b9692); f_010a84b0(cpu); if cpu.returned_to != 0x10b9692 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x10b9695; continue;
 }
 0x10b9695 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9632; continue; }
  pc = 0x10b969c; continue;
 }
 0x10b969c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b96b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b96b0); }
 let mut pc: u32 = 0x10b96b0;
 loop { match pc {
 0x10b96b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b96e1; continue; }
  pc = 0x10b96b9; continue;
 }
 0x10b96b9 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if cpu.zf { pc = 0x10b96e1; continue; }
  pc = 0x10b96bf; continue;
 }
 0x10b96bf => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b96c5); f_010aa4d0(cpu); if cpu.returned_to != 0x10b96c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b96ce); f_010bb040(cpu); if cpu.returned_to != 0x10b96ce { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b96e1; continue;
 }
 0x10b96e1 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b96f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b96f0); }
 let mut pc: u32 = 0x10b96f0;
 loop { match pc {
 0x10b96f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10b9706; continue; }
  pc = 0x10b96ff; continue;
 }
 0x10b96ff => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b9b07; continue;
 }
 0x10b9706 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10b971c); f_010bbcc0(cpu); if cpu.returned_to != 0x10b971c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9743; continue; }
  pc = 0x10b9723; continue;
 }
 0x10b9723 => {
  { let v: u32 = 0x116b27cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b972f; continue;
 }
 0x10b972f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b972f; continue; }
  pc = 0x10b9739; continue;
 }
 0x10b9739 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9743 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xffu32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0xff00u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 24u32); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff000000u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 24u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ebp); }
  if !cpu.cf { pc = 0x10b9842; continue; }
  pc = 0x10b97a0; continue;
 }
 0x10b97a0 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b294u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b97b6; continue; }
  pc = 0x10b97b4; continue;
 }
 0x10b97b4 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b97b6; continue;
 }
 0x10b97b6 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b97c1; continue; }
  pc = 0x10b97ba; continue;
 }
 0x10b97ba => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x10b97c1; continue;
 }
 0x10b97c1 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b29cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b97d7; continue; }
  pc = 0x10b97d5; continue;
 }
 0x10b97d5 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b97d7; continue;
 }
 0x10b97d7 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b97e2; continue; }
  pc = 0x10b97db; continue;
 }
 0x10b97db => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b97e2; continue;
 }
 0x10b97e2 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b2a4u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b97f8; continue; }
  pc = 0x10b97f6; continue;
 }
 0x10b97f6 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b97f8; continue;
 }
 0x10b97f8 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b9803; continue; }
  pc = 0x10b97fc; continue;
 }
 0x10b97fc => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b9803; continue;
 }
 0x10b9803 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi & 0xffu32; cpu.esi = v; }
  { let v: u32 = cpu.ebx & 0xff00u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 24u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x10b97a0; continue; }
  pc = 0x10b9842; continue;
 }
 0x10b9842 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x10b986c; continue; }
  pc = 0x10b984c; continue;
 }
 0x10b984c => {
  { let v: u32 = 0x116b27cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9858; continue;
 }
 0x10b9858 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9858; continue; }
  pc = 0x10b9862; continue;
 }
 0x10b9862 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b986c => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10b989e); f_010bb040(cpu); if cpu.returned_to != 0x10b989e { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x12d2154u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x64u32; cpu.w32(0x4cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x64u32; cpu.w32(0x50u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10b9b02; continue; }
  pc = 0x10b98f5; continue;
 }
 0x10b98f5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b98fe); f_010bbe10(cpu); if cpu.returned_to != 0x10b98fe { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10b9912; continue; }
  pc = 0x10b990b; continue;
 }
 0x10b990b => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x10b9926; continue;
 }
 0x10b9912 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b9920); f_010bbe10(cpu); if cpu.returned_to != 0x10b9920 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x10b9926; continue;
 }
 0x10b9926 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x12d21fcu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1a8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = 0x12d21fcu32; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10b99fa; continue; }
  pc = 0x10b995a; continue;
 }
 0x10b995a => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10b995e; continue;
 }
 0x10b995e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.cf { pc = 0x10b99fa; continue; }
  pc = 0x10b996c; continue;
 }
 0x10b996c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x1a8u32.wrapping_add(cpu.eax)); cpu.push(0x10b9995); cpu.call(t); if cpu.returned_to != 0x10b9995 { return; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b99a3; continue; }
  pc = 0x10b999c; continue;
 }
 0x10b999c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.poll(); pc = 0x10b995e; continue;
 }
 0x10b99a3 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x12d21fcu32); cpu.eax = v; }
  pc = 0x10b99bf; continue;
 }
 0x10b99b3 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  pc = 0x10b99bf; continue;
 }
 0x10b99bf => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x10b99b3; continue; }
  pc = 0x10b99c9; continue;
 }
 0x10b99c9 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.cf { pc = 0x10b99ed; continue; }
  pc = 0x10b99d8; continue;
 }
 0x10b99d8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10b995e; continue;
 }
 0x10b99ed => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x10b995e; continue;
 }
 0x10b99fa => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x10b9b02; continue; }
  pc = 0x10b9a05; continue;
 }
 0x10b9a05 => {
  cpu.push(0x10b9a0a); f_010ae070(cpu); if cpu.returned_to != 0x10b9a0a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b9a31; continue; }
  pc = 0x10b9a0e; continue;
 }
 0x10b9a0e => {
  { let v: u32 = 0x116b2acu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9a1a; continue;
 }
 0x10b9a1a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9a1a; continue; }
  pc = 0x10b9a24; continue;
 }
 0x10b9a24 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9a31 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b9a87; continue; }
  pc = 0x10b9a4f; continue;
 }
 0x10b9a4f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b9a70); f_010ae130(cpu); if cpu.returned_to != 0x10b9a70 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b9a87; continue;
 }
 0x10b9a87 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = 0x503u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b9ac8); f_010ae130(cpu); if cpu.returned_to != 0x10b9ac8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x10b9b02; continue; }
  pc = 0x10b9add; continue;
 }
 0x10b9add => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.push(0x10b9af5); f_010b9b10(cpu); if cpu.returned_to != 0x10b9af5 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9b02 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10b9b07; continue;
 }
 0x10b9b07 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9b10); }
 let mut pc: u32 = 0x10b9b10;
 loop { match pc {
 0x10b9b10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = 0x12d2020u32; cpu.push(v); }
  { let v = 0x116b248u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb44u32); cpu.push(0x10b9b34); cpu.call(t); if cpu.returned_to != 0x10b9b34 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9b40); }
 let mut pc: u32 = 0x10b9b40;
 loop { match pc {
 0x10b9b40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b9b59; continue; }
  pc = 0x10b9b49; continue;
 }
 0x10b9b49 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b9b4f); f_010aa4d0(cpu); if cpu.returned_to != 0x10b9b4f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10b9b59; continue;
 }
 0x10b9b59 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9b60); }
 let mut pc: u32 = 0x10b9b60;
 loop { match pc {
 0x10b9b60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x10b9b6c); f_010ae5d0(cpu); if cpu.returned_to != 0x10b9b6c { return; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10b9b8c; continue; }
  pc = 0x10b9b75; continue;
 }
 0x10b9b75 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x10b9b78; continue;
 }
 0x10b9b78 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x10b9b8c; continue; }
  pc = 0x10b9b7e; continue;
 }
 0x10b9b7e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10b9b78; continue; }
  pc = 0x10b9b8c; continue;
 }
 0x10b9b8c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x20u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x10b9bb2; continue; }
  pc = 0x10b9b91; continue;
 }
 0x10b9b91 => {
  { let v: u32 = 0x116b22cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9b9d; continue;
 }
 0x10b9b9d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9b9d; continue; }
  pc = 0x10b9ba7; continue;
 }
 0x10b9ba7 => {
  cpu.push(0x10b9bac); f_010ae5c0(cpu); if cpu.returned_to != 0x10b9bac { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9bb2 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b9bd5); f_010bb040(cpu); if cpu.returned_to != 0x10b9bd5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10b9be4); f_010ae5c0(cpu); if cpu.returned_to != 0x10b9be4 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9bf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9bf0); }
 let mut pc: u32 = 0x10b9bf0;
 loop { match pc {
 0x10b9bf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x117c8a8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9bfe; continue;
 }
 0x10b9bfe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9bfe; continue; }
  pc = 0x10b9c08; continue;
 }
 0x10b9c08 => {
  { let v: u32 = 0x117c8a8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10b9c61; continue; }
  pc = 0x10b9c20; continue;
 }
 0x10b9c20 => {
  { let v: u8 = cpu.r8(0x117c8a8u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x10b9c4d; continue; }
  pc = 0x10b9c2b; continue;
 }
 0x10b9c2b => {
  { let v: u32 = 0x117c8a8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.zf { pc = 0x10b9c61; continue; }
  pc = 0x10b9c41; continue;
 }
 0x10b9c41 => {
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x117c8a8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9c4d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x10b9c5e; continue; }
  pc = 0x10b9c52; continue;
 }
 0x10b9c52 => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x117c8a8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9c5e => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10b9c20; continue; }
  pc = 0x10b9c61; continue;
 }
 0x10b9c61 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9c70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9c70); }
 let mut pc: u32 = 0x10b9c70;
 loop { match pc {
 0x10b9c70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x112u32); }
  if cpu.cf { pc = 0x10b9d14; continue; }
  pc = 0x10b9c8a; continue;
 }
 0x10b9c8a => {
  { let v: u32 = cpu.edx.wrapping_add(0xbau32); cpu.edx = v; }
  { let v: u32 = 0x116b1f0u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b9c97; continue;
 }
 0x10b9c97 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9ca5; continue; }
  pc = 0x10b9c9f; continue;
 }
 0x10b9c9f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9ca5; continue; }
  pc = 0x10b9ca3; continue;
 }
 0x10b9ca3 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9ca5; continue;
 }
 0x10b9ca5 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9cb2; continue; }
  pc = 0x10b9caa; continue;
 }
 0x10b9caa => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9cb2; continue; }
  pc = 0x10b9caf; continue;
 }
 0x10b9caf => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b9cb2; continue;
 }
 0x10b9cb2 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b9cbe; continue; }
  pc = 0x10b9cb6; continue;
 }
 0x10b9cb6 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b9cbe; continue; }
  pc = 0x10b9cba; continue;
 }
 0x10b9cba => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b9c97; continue;
 }
 0x10b9cbe => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b9cc5; continue; }
  pc = 0x10b9cc3; continue;
 }
 0x10b9cc3 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9cc5; continue;
 }
 0x10b9cc5 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b9cd2; continue; }
  pc = 0x10b9cc9; continue;
 }
 0x10b9cc9 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9cd2 => {
  { let v: u32 = 0x116b204u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b9cd9; continue;
 }
 0x10b9cd9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9ce7; continue; }
  pc = 0x10b9ce1; continue;
 }
 0x10b9ce1 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9ce7; continue; }
  pc = 0x10b9ce5; continue;
 }
 0x10b9ce5 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9ce7; continue;
 }
 0x10b9ce7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9cf4; continue; }
  pc = 0x10b9cec; continue;
 }
 0x10b9cec => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9cf4; continue; }
  pc = 0x10b9cf1; continue;
 }
 0x10b9cf1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b9cf4; continue;
 }
 0x10b9cf4 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b9d00; continue; }
  pc = 0x10b9cf8; continue;
 }
 0x10b9cf8 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b9d00; continue; }
  pc = 0x10b9cfc; continue;
 }
 0x10b9cfc => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b9cd9; continue;
 }
 0x10b9d00 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b9d07; continue; }
  pc = 0x10b9d05; continue;
 }
 0x10b9d05 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9d07; continue;
 }
 0x10b9d07 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b9d14; continue; }
  pc = 0x10b9d0b; continue;
 }
 0x10b9d0b => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9d14 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  if cpu.zf { pc = 0x10b9dca; continue; }
  pc = 0x10b9d31; continue;
 }
 0x10b9d31 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x10b9dca; continue; }
  pc = 0x10b9d3a; continue;
 }
 0x10b9d3a => {
  { let v: u32 = 0x116b220u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b9d41; continue;
 }
 0x10b9d41 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9d4f; continue; }
  pc = 0x10b9d49; continue;
 }
 0x10b9d49 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9d4f; continue; }
  pc = 0x10b9d4d; continue;
 }
 0x10b9d4d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9d4f; continue;
 }
 0x10b9d4f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9d5c; continue; }
  pc = 0x10b9d54; continue;
 }
 0x10b9d54 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9d5c; continue; }
  pc = 0x10b9d59; continue;
 }
 0x10b9d59 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b9d5c; continue;
 }
 0x10b9d5c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b9d68; continue; }
  pc = 0x10b9d60; continue;
 }
 0x10b9d60 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b9d68; continue; }
  pc = 0x10b9d64; continue;
 }
 0x10b9d64 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b9d41; continue;
 }
 0x10b9d68 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b9d6f; continue; }
  pc = 0x10b9d6d; continue;
 }
 0x10b9d6d => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9d6f; continue;
 }
 0x10b9d6f => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b9d88; continue; }
  pc = 0x10b9d73; continue;
 }
 0x10b9d73 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x5000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9dca; continue; }
  pc = 0x10b9d7f; continue;
 }
 0x10b9d7f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9d88 => {
  { let v: u32 = 0x116b224u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x10b9d8f; continue;
 }
 0x10b9d8f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9d9d; continue; }
  pc = 0x10b9d97; continue;
 }
 0x10b9d97 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9d9d; continue; }
  pc = 0x10b9d9b; continue;
 }
 0x10b9d9b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9d9d; continue;
 }
 0x10b9d9d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10b9daa; continue; }
  pc = 0x10b9da2; continue;
 }
 0x10b9da2 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10b9daa; continue; }
  pc = 0x10b9da7; continue;
 }
 0x10b9da7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10b9daa; continue;
 }
 0x10b9daa => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10b9db6; continue; }
  pc = 0x10b9dae; continue;
 }
 0x10b9dae => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10b9db6; continue; }
  pc = 0x10b9db2; continue;
 }
 0x10b9db2 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10b9d8f; continue;
 }
 0x10b9db6 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10b9dbd; continue; }
  pc = 0x10b9dbb; continue;
 }
 0x10b9dbb => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10b9dbd; continue;
 }
 0x10b9dbd => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10b9dca; continue; }
  pc = 0x10b9dc1; continue;
 }
 0x10b9dc1 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9dca => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9dd0); }
 let mut pc: u32 = 0x10b9dd0;
 loop { match pc {
 0x10b9dd0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10b9ddd); f_010ad1c0(cpu); if cpu.returned_to != 0x10b9ddd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9df0); }
 let mut pc: u32 = 0x10b9df0;
 loop { match pc {
 0x10b9df0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x116b1c8u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b9e07); f_010add40(cpu); if cpu.returned_to != 0x10b9e07 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9e2b; continue; }
  pc = 0x10b9e0e; continue;
 }
 0x10b9e0e => {
  { let v: u32 = 0x116b1d0u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9e1a; continue;
 }
 0x10b9e1a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9e1a; continue; }
  pc = 0x10b9e24; continue;
 }
 0x10b9e24 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10b9e60; continue;
 }
 0x10b9e2b => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b9e40); f_010aa9b0(cpu); if cpu.returned_to != 0x10b9e40 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9e5e; continue; }
  pc = 0x10b9e50; continue;
 }
 0x10b9e50 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10b9e5e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10b9e60; continue;
 }
 0x10b9e60 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9e70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10b9e70); }
 let mut pc: u32 = 0x10b9e70;
 loop { match pc {
 0x10b9e70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b9e81); f_010ad270(cpu); if cpu.returned_to != 0x10b9e81 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10b9ea9; continue; }
  pc = 0x10b9e8a; continue;
 }
 0x10b9e8a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b9e95); f_010ba000(cpu); if cpu.returned_to != 0x10b9e95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9ea7; continue; }
  pc = 0x10b9e9e; continue;
 }
 0x10b9e9e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b9ea4); f_010ad1c0(cpu); if cpu.returned_to != 0x10b9ea4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10b9ea7; continue;
 }
 0x10b9ea7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10b9ea9; continue;
 }
 0x10b9ea9 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010b9eb0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b9eb0); } r_010b9eb0(cpu, 0x10b9eb0); }
pub fn f_010b9ee8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10b9ee8); } r_010b9eb0(cpu, 0x10b9ee8); }
fn r_010b9eb0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10b9eb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x180u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x194u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x108u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x10b9ed4); f_010b45a0(cpu); if cpu.returned_to != 0x10b9ed4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9f92; continue; }
  pc = 0x10b9ee1; continue;
 }
 0x10b9ee1 => {
  { cpu.sub8(cpu.r8(0x117c714u32), 0x0u8); }
  pc = 0x10b9ee8; continue;
 }
 0x10b9ee8 => {
  if cpu.zf { pc = 0x10b9f7a; continue; }
  pc = 0x10b9eee; continue;
 }
 0x10b9eee => {
  { let v: u32 = 0x117c714u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9ef7; continue;
 }
 0x10b9ef7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9ef7; continue; }
  pc = 0x10b9f01; continue;
 }
 0x10b9f01 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10b9f12; continue;
 }
 0x10b9f12 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9f12; continue; }
  pc = 0x10b9f18; continue;
 }
 0x10b9f18 => {
  { let v: u32 = 0x117c8a8u32; cpu.esi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9f26; continue;
 }
 0x10b9f26 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9f26; continue; }
  pc = 0x10b9f30; continue;
 }
 0x10b9f30 => {
  { let v: u32 = 0x117c714u32; cpu.esi = v; }
  { let v: u32 = 0x117c8a8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9f3c; continue;
 }
 0x10b9f3c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9f3c; continue; }
  pc = 0x10b9f46; continue;
 }
 0x10b9f46 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x117c8a8u32; cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10b9f5f; continue;
 }
 0x10b9f5f => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9f5f; continue; }
  pc = 0x10b9f65; continue;
 }
 0x10b9f65 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b9f71); f_010b45a0(cpu); if cpu.returned_to != 0x10b9f71 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9f92; continue; }
  pc = 0x10b9f7a; continue;
 }
 0x10b9f7a => {
  { let v: u32 = 0x116b1b0u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9f86; continue;
 }
 0x10b9f86 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9f86; continue; }
  pc = 0x10b9f90; continue;
 }
 0x10b9f90 => {
  pc = 0x10b9fd0; continue;
 }
 0x10b9f92 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10b9f9e); f_010ad270(cpu); if cpu.returned_to != 0x10b9f9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10b9fa9); f_010b4960(cpu); if cpu.returned_to != 0x10b9fa9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10b9fd0; continue; }
  pc = 0x10b9fb0; continue;
 }
 0x10b9fb0 => {
  { let v: u32 = cpu.r32(0x198u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b9fbe); f_010ba000(cpu); if cpu.returned_to != 0x10b9fbe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10b9fd0; continue; }
  pc = 0x10b9fc7; continue;
 }
 0x10b9fc7 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10b9fcd); f_010ad1c0(cpu); if cpu.returned_to != 0x10b9fcd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10b9fd0; continue;
 }
 0x10b9fd0 => {
  { cpu.sub8(cpu.r8(0x100u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x10b9ff2; continue; }
  pc = 0x10b9fda; continue;
 }
 0x10b9fda => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x117c8a8u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10b9fe8; continue;
 }
 0x10b9fe8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10b9fe8; continue; }
  pc = 0x10b9ff2; continue;
 }
 0x10b9ff2 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x180u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ba000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ba000); }
 let mut pc: u32 = 0x10ba000;
 loop { match pc {
 0x10ba000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x10ba00c); f_010bc180(cpu); if cpu.returned_to != 0x10ba00c { return; }
  { let v = 0x1d8u32; cpu.push(v); }
  cpu.push(0x10ba016); f_010b4990(cpu); if cpu.returned_to != 0x10ba016 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ba03c; continue; }
  pc = 0x10ba01f; continue;
 }
 0x10ba01f => {
  { let v: u32 = 0x116b0d4u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba02b; continue;
 }
 0x10ba02b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba02b; continue; }
  pc = 0x10ba035; continue;
 }
 0x10ba035 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ba6ca; continue;
 }
 0x10ba03c => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b0fcu32; cpu.esi = v; }
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
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x10ba094; continue; }
  pc = 0x10ba069; continue;
 }
 0x10ba069 => {
  { let v: u32 = 0x116b104u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba077; continue;
 }
 0x10ba077 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba077; continue; }
  pc = 0x10ba081; continue;
 }
 0x10ba081 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10ba087); f_010b4960(cpu); if cpu.returned_to != 0x10ba087 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10ba094 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ba0a6); f_010ae130(cpu); if cpu.returned_to != 0x10ba0a6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  if cpu.zf { pc = 0x10ba13e; continue; }
  pc = 0x10ba0ef; continue;
 }
 0x10ba0ef => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10ba13e; continue; }
  pc = 0x10ba0f4; continue;
 }
 0x10ba0f4 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x117bb3cu32); cpu.push(0x10ba0fb); cpu.call(t); if cpu.returned_to != 0x10ba0fb { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ba13e; continue; }
  pc = 0x10ba101; continue;
 }
 0x10ba101 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x10ba13e; continue; }
  pc = 0x10ba106; continue;
 }
 0x10ba106 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10ba120; continue; }
  pc = 0x10ba11b; continue;
 }
 0x10ba11b => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x10ba120; continue;
 }
 0x10ba120 => {
  { let v: u32 = cpu.r32(0x12d21d0u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x10ba13e; continue;
 }
 0x10ba13e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x10ba196; continue; }
  pc = 0x10ba157; continue;
 }
 0x10ba157 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x10ba196; continue; }
  pc = 0x10ba15c; continue;
 }
 0x10ba15c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x117c8a8u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba16c; continue;
 }
 0x10ba16c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba16c; continue; }
  pc = 0x10ba176; continue;
 }
 0x10ba176 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x10ba18e; continue;
 }
 0x10ba18e => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba18e; continue; }
  pc = 0x10ba194; continue;
 }
 0x10ba194 => {
  pc = 0x10ba1a0; continue;
 }
 0x10ba196 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x10ba1a0; continue;
 }
 0x10ba1a0 => {
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
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
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10ba207; continue; }
  pc = 0x10ba1d3; continue;
 }
 0x10ba1d3 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba1ea); f_010ba740(cpu); if cpu.returned_to != 0x10ba1ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ba207; continue; }
  pc = 0x10ba1f1; continue;
 }
 0x10ba1f1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x10ba207; continue;
 }
 0x10ba207 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10ba250; continue; }
  pc = 0x10ba20e; continue;
 }
 0x10ba20e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10ba217); f_010ad380(cpu); if cpu.returned_to != 0x10ba217 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ba250; continue; }
  pc = 0x10ba220; continue;
 }
 0x10ba220 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x10ba235); f_010ba740(cpu); if cpu.returned_to != 0x10ba235 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ba250; continue; }
  pc = 0x10ba23c; continue;
 }
 0x10ba23c => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x10ba250; continue;
 }
 0x10ba250 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10ba2eb; continue; }
  pc = 0x10ba25c; continue;
 }
 0x10ba25c => {
  { cpu.sub32(cpu.r32(0x12d2164u32), 0x1u32); }
  if !cpu.zf { pc = 0x10ba2eb; continue; }
  pc = 0x10ba269; continue;
 }
 0x10ba269 => {
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  pc = 0x10ba2d7; continue;
 }
 0x10ba273 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10ba29f; continue; }
  pc = 0x10ba291; continue;
 }
 0x10ba291 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x10ba29f; continue;
 }
 0x10ba29f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba2a6); f_010ba740(cpu); if cpu.returned_to != 0x10ba2a6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10ba2c3; continue; }
  pc = 0x10ba2ad; continue;
 }
 0x10ba2ad => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x10ba2eb; continue;
 }
 0x10ba2c3 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x10ba2d7; continue;
 }
 0x10ba2d7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x10ba273; continue; }
  pc = 0x10ba2eb; continue;
 }
 0x10ba2eb => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10ba31b; continue; }
  pc = 0x10ba2f2; continue;
 }
 0x10ba2f2 => {
  { let v: u32 = 0x116b11cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba2fe; continue;
 }
 0x10ba2fe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba2fe; continue; }
  pc = 0x10ba308; continue;
 }
 0x10ba308 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba30e); f_010b4960(cpu); if cpu.returned_to != 0x10ba30e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10ba31b => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x12d2170u32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ba33d); f_010ae130(cpu); if cpu.returned_to != 0x10ba33d { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x501u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ba35f); f_010ae130(cpu); if cpu.returned_to != 0x10ba35f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x10ba3b2; continue; }
  pc = 0x10ba36a; continue;
 }
 0x10ba36a => {
  { let v: u32 = 0x116b13cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba376; continue;
 }
 0x10ba376 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba376; continue; }
  pc = 0x10ba380; continue;
 }
 0x10ba380 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ba392); f_010ae130(cpu); if cpu.returned_to != 0x10ba392 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10ba3a5); f_010b4960(cpu); if cpu.returned_to != 0x10ba3a5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10ba3b2 => {
  { let v: u32 = cpu.r32(0x12d2150u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ba3d1); f_010b4990(cpu); if cpu.returned_to != 0x10ba3d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10ba423; continue; }
  pc = 0x10ba3db; continue;
 }
 0x10ba3db => {
  { let v: u32 = 0x116b168u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba3e7; continue;
 }
 0x10ba3e7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba3e7; continue; }
  pc = 0x10ba3f1; continue;
 }
 0x10ba3f1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ba403); f_010ae130(cpu); if cpu.returned_to != 0x10ba403 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10ba416); f_010b4960(cpu); if cpu.returned_to != 0x10ba416 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10ba423 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10ba477; continue; }
  pc = 0x10ba430; continue;
 }
 0x10ba430 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10ba432; continue;
 }
 0x10ba432 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x116b190u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.edx); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10ba432; continue; }
  pc = 0x10ba477; continue;
 }
 0x10ba477 => {
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d214cu32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1a8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x7fu32; cpu.w32(0x1b4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x40u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ba4cc; continue;
 }
 0x10ba4cc => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xa0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xe0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x120u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x160u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba4cc; continue; }
  pc = 0x10ba4f1; continue;
 }
 0x10ba4f1 => {
  { let v = 0x10ba820u32; cpu.push(v); }
  cpu.push(0x10ba4fb); f_010adc40(cpu); if cpu.returned_to != 0x10ba4fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10ba55a; continue; }
  pc = 0x10ba506; continue;
 }
 0x10ba506 => {
  { let v: u32 = 0x116b198u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10ba512; continue;
 }
 0x10ba512 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba512; continue; }
  pc = 0x10ba51c; continue;
 }
 0x10ba51c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ba52e); f_010ae130(cpu); if cpu.returned_to != 0x10ba52e { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ba544); f_010b4960(cpu); if cpu.returned_to != 0x10ba544 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba54d); f_010b4960(cpu); if cpu.returned_to != 0x10ba54d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10ba55a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10ba561); f_010adb30(cpu); if cpu.returned_to != 0x10ba561 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x10ba6e0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x10ba578; continue;
 }
 0x10ba578 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba587); f_010bbf50(cpu); if cpu.returned_to != 0x10ba587 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba597); f_010bbf50(cpu); if cpu.returned_to != 0x10ba597 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5a8); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5a8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x70u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5b5); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5c2); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d2154u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5d4); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5d4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5e2); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5f0); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba5fd); f_010bbf50(cpu); if cpu.returned_to != 0x10ba5fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = 0x5bu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba60b); f_010bbf50(cpu); if cpu.returned_to != 0x10ba60b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x5du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba618); f_010bbf50(cpu); if cpu.returned_to != 0x10ba618 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba625); f_010bbf50(cpu); if cpu.returned_to != 0x10ba625 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x65u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba632); f_010bbf50(cpu); if cpu.returned_to != 0x10ba632 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba63f); f_010bbf50(cpu); if cpu.returned_to != 0x10ba63f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d2160u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba652); f_010bbf50(cpu); if cpu.returned_to != 0x10ba652 { return; }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x10ba68e; continue; }
  pc = 0x10ba65f; continue;
 }
 0x10ba65f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ba67f); f_010ae130(cpu); if cpu.returned_to != 0x10ba67f { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10ba68e; continue;
 }
 0x10ba68e => {
  { cpu.and16((cpu.esi as u16), 0x3u16); }
  if !cpu.zf { pc = 0x10ba69f; continue; }
  pc = 0x10ba695; continue;
 }
 0x10ba695 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10ba69c); f_010ae080(cpu); if cpu.returned_to != 0x10ba69c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10ba69f; continue;
 }
 0x10ba69f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10ba578; continue; }
  pc = 0x10ba6a9; continue;
 }
 0x10ba6a9 => {
  { let v: u32 = cpu.r32(0x12d214cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10ba6b9); f_010ad9b0(cpu); if cpu.returned_to != 0x10ba6b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10ba6c5); f_010ad760(cpu); if cpu.returned_to != 0x10ba6c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x10ba6ca; continue;
 }
 0x10ba6ca => {
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
pub fn f_010ba6e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ba6e0); }
 let mut pc: u32 = 0x10ba6e0;
 loop { match pc {
 0x10ba6e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10ba70f; continue; }
  pc = 0x10ba6f1; continue;
 }
 0x10ba6f1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10ba6f3; continue;
 }
 0x10ba6f3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x10ba6ff); f_010aa370(cpu); if cpu.returned_to != 0x10ba6ff { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10ba6f3; continue; }
  pc = 0x10ba70f; continue;
 }
 0x10ba70f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10ba718); f_010ad500(cpu); if cpu.returned_to != 0x10ba718 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10ba724); f_010b4960(cpu); if cpu.returned_to != 0x10ba724 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10ba72d); f_010b4960(cpu); if cpu.returned_to != 0x10ba72d { return; }
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
pub fn f_010ba740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ba740); }
 let mut pc: u32 = 0x10ba740;
 loop { match pc {
 0x10ba740 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub16(cpu.r16(0x10u32.wrapping_add(cpu.eax)), 0x0u16); }
  if (cpu.cf || cpu.zf) { pc = 0x10ba7e7; continue; }
  pc = 0x10ba769; continue;
 }
 0x10ba769 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10ba790; continue; }
  pc = 0x10ba789; continue;
 }
 0x10ba789 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x10ba790; continue;
 }
 0x10ba790 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10ba7a4; continue; }
  pc = 0x10ba79b; continue;
 }
 0x10ba79b => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x10ba7a4; continue;
 }
 0x10ba7a4 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10ba7b9; continue; }
  pc = 0x10ba7b0; continue;
 }
 0x10ba7b0 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10ba7b9; continue;
 }
 0x10ba7b9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10ba7ce; continue; }
  pc = 0x10ba7c5; continue;
 }
 0x10ba7c5 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x10ba7ce; continue;
 }
 0x10ba7ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10ba7d0; continue;
 }
 0x10ba7d0 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x10ba7df; continue; }
  pc = 0x10ba7d7; continue;
 }
 0x10ba7d7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x10ba7df; continue;
 }
 0x10ba7df => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba7d0; continue; }
  pc = 0x10ba7e7; continue;
 }
 0x10ba7e7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10ba80d); f_010ae130(cpu); if cpu.returned_to != 0x10ba80d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010ba820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10ba820); }
 let mut pc: u32 = 0x10ba820;
 loop { match pc {
 0x10ba820 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x10bafca; continue; }
  pc = 0x10ba839; continue;
 }
 0x10ba839 => {
  { cpu.sub32(cpu.r32(0x117c928u32), 0x0u32); }
  if !cpu.zf { pc = 0x10bafca; continue; }
  pc = 0x10ba846; continue;
 }
 0x10ba846 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117c928u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d21e8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21dcu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10baf75; continue; }
  pc = 0x10ba86a; continue;
 }
 0x10ba86a => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x10baf51; continue; }
  pc = 0x10ba879; continue;
 }
 0x10ba879 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d21f8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10ba893; continue;
 }
 0x10ba893 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x64u32); }
  if (cpu.sf != cpu.of) { pc = 0x10bae89; continue; }
  pc = 0x10ba8a4; continue;
 }
 0x10ba8a4 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10ba94b; continue; }
  pc = 0x10ba8b8; continue;
 }
 0x10ba8b8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21e0u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x10ba8c5; continue;
 }
 0x10ba8c5 => {
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10ba935; continue; }
  pc = 0x10ba8dd; continue;
 }
 0x10ba8dd => {
  { let v: u32 = cpu.r32(0x65cu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x65cu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10ba935; continue; }
  pc = 0x10ba8ef; continue;
 }
 0x10ba8ef => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10ba90e); f_010bb4a0(cpu); if cpu.returned_to != 0x10ba90e { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x10ba94b; continue; }
  pc = 0x10ba935; continue;
 }
 0x10ba935 => {
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d21e0u32, v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10ba8c5; continue; }
  pc = 0x10ba94b; continue;
 }
 0x10ba94b => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x10badfc; continue; }
  pc = 0x10ba95f; continue;
 }
 0x10ba95f => {
  { cpu.sub32(cpu.r32(0x12d21f8u32), 0x0u32); }
  if !cpu.zf { pc = 0x10badda; continue; }
  pc = 0x10ba96c; continue;
 }
 0x10ba96c => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21f4u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.cf { pc = 0x10badda; continue; }
  pc = 0x10ba98a; continue;
 }
 0x10ba98a => {
  { cpu.sub32(cpu.eax, 0xf7u32); }
  if cpu.cf { pc = 0x10ba9a3; continue; }
  pc = 0x10ba991; continue;
 }
 0x10ba991 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bab8f; continue; }
  pc = 0x10ba997; continue;
 }
 0x10ba997 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { pc = 0x10ba9b3; continue; }
  pc = 0x10ba99e; continue;
 }
 0x10ba99e => {
  pc = 0x10bac2e; continue;
 }
 0x10ba9a3 => {
  { cpu.sub32(cpu.eax, 0xf0u32); }
  if cpu.zf { pc = 0x10bab8f; continue; }
  pc = 0x10ba9ae; continue;
 }
 0x10ba9ae => {
  pc = 0x10bac2e; continue;
 }
 0x10ba9b3 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x18u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21d4u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10ba9cb); f_010bbe50(cpu); if cpu.returned_to != 0x10ba9cb { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21d0u32, v); }
  { let v: u32 = cpu.r32(0x12d21d4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x51u32); }
  if cpu.cf { pc = 0x10ba9f1; continue; }
  pc = 0x10ba9dd; continue;
 }
 0x10ba9dd => {
  if (cpu.cf || cpu.zf) { pc = 0x10baa94; continue; }
  pc = 0x10ba9e3; continue;
 }
 0x10ba9e3 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if cpu.zf { pc = 0x10baadb; continue; }
  pc = 0x10ba9ec; continue;
 }
 0x10ba9ec => {
  pc = 0x10bab7c; continue;
 }
 0x10ba9f1 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x10bab7c; continue; }
  pc = 0x10ba9fa; continue;
 }
 0x10ba9fa => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d21f8u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10baa1a; continue; }
  pc = 0x10baa11; continue;
 }
 0x10baa11 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10baa58; continue; }
  pc = 0x10baa1a; continue;
 }
 0x10baa1a => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10bab7c; continue; }
  pc = 0x10baa48; continue;
 }
 0x10baa48 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x10bab79; continue;
 }
 0x10baa58 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10baa61); f_010aa4d0(cpu); if cpu.returned_to != 0x10baa61 { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10bab7c; continue; }
  pc = 0x10baa7b; continue;
 }
 0x10baa7b => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x10baa81); cpu.call(t); if cpu.returned_to != 0x10baa81 { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10ba95f; continue;
 }
 0x10baa94 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x12d21f0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10ba95f; continue;
 }
 0x10baadb => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21f0u32, v); }
  { let v: u32 = 0xf42400u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d214cu32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x12d21f0u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21ecu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10bab36; continue; }
  pc = 0x10bab22; continue;
 }
 0x10bab22 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21f0u32, v); }
  { let v: u8 = cpu.r8(0x12d21f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x10bab3e; continue;
 }
 0x10bab36 => {
  { let v: u8 = cpu.r8(0x12d21f0u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x10bab3e; continue;
 }
 0x10bab3e => {
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bab7c; continue; }
  pc = 0x10bab6d; continue;
 }
 0x10bab6d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x10bab79; continue;
 }
 0x10bab79 => {
  let t = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.push(0x10bab7c); cpu.call(t); if cpu.returned_to != 0x10bab7c { return; }
  pc = 0x10bab7c; continue;
 }
 0x10bab7c => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10ba95f; continue;
 }
 0x10bab8f => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = 0x12d21c8u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21c8u32, v); }
  cpu.push(0x10baba7); f_010bbe50(cpu); if cpu.returned_to != 0x10baba7 { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12d21c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21d0u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21d0u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x10babd5); f_010bc0d0(cpu); if cpu.returned_to != 0x10babd5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10babe7; continue; }
  pc = 0x10babe0; continue;
 }
 0x10babe0 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x10babe9; continue;
 }
 0x10babe7 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x10babe9; continue;
 }
 0x10babe9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x10bac13); f_010bc0d0(cpu); if cpu.returned_to != 0x10bac13 { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21d0u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x10ba95f; continue;
 }
 0x10bac2e => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d21ccu32, v); }
  { let v: u32 = cpu.r32(0x12d21f4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x12d21d8u32, v); }
  { let v: u32 = cpu.r32(0x12d21f4u32) & 0xf0u32; cpu.w32(0x12d21f4u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bac75); f_010bb4a0(cpu); if cpu.returned_to != 0x10bac75 { return; }
  { let v: u32 = cpu.r32(0x12d21f4u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if cpu.zf { pc = 0x10bad07; continue; }
  pc = 0x10bac8a; continue;
 }
 0x10bac8a => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d21ccu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba95f; continue; }
  pc = 0x10baca0; continue;
 }
 0x10baca0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x10bacd0; continue; }
  pc = 0x10bacb3; continue;
 }
 0x10bacb3 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bace9; continue; }
  pc = 0x10bacb5; continue;
 }
 0x10bacb5 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x10bacc7; continue; }
  pc = 0x10bacbc; continue;
 }
 0x10bacbc => {
  if (cpu.cf || cpu.zf) { pc = 0x10bacf0; continue; }
  pc = 0x10bacbe; continue;
 }
 0x10bacbe => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x10bace9; continue; }
  pc = 0x10bacc5; continue;
 }
 0x10bacc5 => {
  pc = 0x10bacf7; continue;
 }
 0x10bacc7 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x10bacf0; continue; }
  pc = 0x10bacce; continue;
 }
 0x10bacce => {
  pc = 0x10bacf7; continue;
 }
 0x10bacd0 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x10bace2; continue; }
  pc = 0x10bacd7; continue;
 }
 0x10bacd7 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bace9; continue; }
  pc = 0x10bacd9; continue;
 }
 0x10bacd9 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x10bace9; continue; }
  pc = 0x10bace0; continue;
 }
 0x10bace0 => {
  pc = 0x10bacf7; continue;
 }
 0x10bace2 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x10bacf7; continue; }
  pc = 0x10bace9; continue;
 }
 0x10bace9 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x10bacf9; continue;
 }
 0x10bacf0 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x10bacf9; continue;
 }
 0x10bacf7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bacf9; continue;
 }
 0x10bacf9 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10ba95f; continue;
 }
 0x10bad07 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d21e0u32, v); }
  pc = 0x10bad17; continue;
 }
 0x10bad17 => {
  { cpu.sub32(cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffffffffu32); }
  if cpu.zf { pc = 0x10bad38; continue; }
  pc = 0x10bad21; continue;
 }
 0x10bad21 => {
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x12d21e0u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bad17; continue; }
  pc = 0x10bad38; continue;
 }
 0x10bad38 => {
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if !cpu.zf { pc = 0x10bad81; continue; }
  pc = 0x10bad43; continue;
 }
 0x10bad43 => {
  { let v: u32 = 0x116b0b4u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10bad4f; continue;
 }
 0x10bad4f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10bad4f; continue; }
  pc = 0x10bad59; continue;
 }
 0x10bad59 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  cpu.push(0x10bad67); f_010aa4d0(cpu); if cpu.returned_to != 0x10bad67 { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x117c928u32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bafca; continue;
 }
 0x10bad81 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x558u32.wrapping_add(cpu.eax))); cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d21d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10badbf); f_010bbe50(cpu); if cpu.returned_to != 0x10badbf { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x65cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x10ba95f; continue;
 }
 0x10badda => {
  { cpu.sub32(cpu.r32(0x12d21f8u32), 0x0u32); }
  if !cpu.zf { pc = 0x10badfc; continue; }
  pc = 0x10bade3; continue;
 }
 0x10bade3 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x10badfc; continue;
 }
 0x10badfc => {
  { cpu.sub32(cpu.r32(0x12d21f8u32), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba893; continue; }
  pc = 0x10bae09; continue;
 }
 0x10bae09 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10ba893; continue; }
  pc = 0x10bae24; continue;
 }
 0x10bae24 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x10bae47; continue; }
  pc = 0x10bae39; continue;
 }
 0x10bae39 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bae47; continue;
 }
 0x10bae47 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x10ba893; continue; }
  pc = 0x10bae56; continue;
 }
 0x10bae56 => {
  { let v = 0x12d21e4u32; cpu.push(v); }
  { let v = 0x12d21e0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bae66); f_010a9030(cpu); if cpu.returned_to != 0x10bae66 { return; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d21e0u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d21e4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x10bae84); cpu.call(t); if cpu.returned_to != 0x10bae84 { return; }
  cpu.poll(); pc = 0x10ba893; continue;
 }
 0x10bae89 => {
  { cpu.sub32(cpu.r32(0x12d21f8u32), 0x0u32); }
  if !cpu.zf { pc = 0x10baf51; continue; }
  pc = 0x10bae96; continue;
 }
 0x10bae96 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10baefb; continue; }
  pc = 0x10bae9e; continue;
 }
 0x10bae9e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x10baeac; continue;
 }
 0x10baeac => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x10baee7; continue; }
  pc = 0x10baebb; continue;
 }
 0x10baebb => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10baed4; continue; }
  pc = 0x10baecc; continue;
 }
 0x10baecc => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10baeda; continue;
 }
 0x10baed4 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10baeda; continue;
 }
 0x10baeda => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x10baeac; continue; }
  pc = 0x10baee7; continue;
 }
 0x10baee7 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x38u32.wrapping_add(cpu.eax)), 0x7u8); }
  if !cpu.zf { pc = 0x10baefb; continue; }
  pc = 0x10baef2; continue;
 }
 0x10baef2 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10baef8); f_010bafe0(cpu); if cpu.returned_to != 0x10baef8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10baefb; continue;
 }
 0x10baefb => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10baf51; continue; }
  pc = 0x10baf08; continue;
 }
 0x10baf08 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x10baf16; continue;
 }
 0x10baf16 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x10baf51; continue; }
  pc = 0x10baf25; continue;
 }
 0x10baf25 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10baf3e; continue; }
  pc = 0x10baf36; continue;
 }
 0x10baf36 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10baf44; continue;
 }
 0x10baf3e => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x10baf44; continue;
 }
 0x10baf44 => {
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x10baf16; continue; }
  pc = 0x10baf51; continue;
 }
 0x10baf51 => {
  { let v: u32 = cpu.r32(0x12d21e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x12d21dcu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x71cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x12d21e8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x12d21dcu32, v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10ba86a; continue; }
  pc = 0x10baf75; continue;
 }
 0x10baf75 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bafc2; continue; }
  pc = 0x10baf82; continue;
 }
 0x10baf82 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bafa7); f_010ae130(cpu); if cpu.returned_to != 0x10bafa7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bafc2; continue;
 }
 0x10bafc2 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x117c928u32, v); }
  pc = 0x10bafca; continue;
 }
 0x10bafca => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bafe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bafe0); }
 let mut pc: u32 = 0x10bafe0;
 loop { match pc {
 0x10bafe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x10bafeb; continue;
 }
 0x10bafeb => {
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb009; continue; }
  pc = 0x10baff6; continue;
 }
 0x10baff6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bb006); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb006 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb009; continue;
 }
 0x10bb009 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bafeb; continue; }
  pc = 0x10bb012; continue;
 }
 0x10bb012 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bb020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb020); }
 let mut pc: u32 = 0x10bb020;
 loop { match pc {
 0x10bb020 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bb02b); f_010bb040(cpu); if cpu.returned_to != 0x10bb02b { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bb040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb040); }
 let mut pc: u32 = 0x10bb040;
 loop { match pc {
 0x10bb040 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bb04b; continue;
 }
 0x10bb04b => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bb04b; continue; }
  pc = 0x10bb05a; continue;
 }
 0x10bb05a => {
  { let v: u32 = 0x480u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xd8u32.wrapping_add(cpu.esi); cpu.edi = v; }
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
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x10bb085; continue;
 }
 0x10bb085 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10bb085; continue; }
  pc = 0x10bb096; continue;
 }
 0x10bb096 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x10bb09e; continue;
 }
 0x10bb09e => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x10bb09e; continue; }
  pc = 0x10bb0af; continue;
 }
 0x10bb0af => {
  { let v: u32 = 0x0u32; cpu.w32(0x558u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x64u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x6cu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32; cpu.w32(0x68u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x7a1200u32; cpu.w32(0x74u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bb100(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb100); }
 let mut pc: u32 = 0x10bb100;
 loop { match pc {
 0x10bb100 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x298u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb133; continue; }
  pc = 0x10bb120; continue;
 }
 0x10bb120 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb130); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb130 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb133; continue;
 }
 0x10bb133 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xd8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb15a; continue; }
  pc = 0x10bb147; continue;
 }
 0x10bb147 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb157); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb157 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb15a; continue;
 }
 0x10bb15a => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb186; continue; }
  pc = 0x10bb16e; continue;
 }
 0x10bb16e => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb183); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb183 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb186; continue;
 }
 0x10bb186 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb1ad; continue; }
  pc = 0x10bb19a; continue;
 }
 0x10bb19a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb1aa); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb1aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb1ad; continue;
 }
 0x10bb1ad => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb1d4; continue; }
  pc = 0x10bb1c1; continue;
 }
 0x10bb1c1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6fu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb1d1); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb1d1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb1d4; continue;
 }
 0x10bb1d4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x258u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb1fb; continue; }
  pc = 0x10bb1e8; continue;
 }
 0x10bb1e8 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb1f8); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb1f8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb1fb; continue;
 }
 0x10bb1fb => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x358u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb222; continue; }
  pc = 0x10bb20f; continue;
 }
 0x10bb20f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb21f); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb21f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb222; continue;
 }
 0x10bb222 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb249; continue; }
  pc = 0x10bb236; continue;
 }
 0x10bb236 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb246); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb246 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb249; continue;
 }
 0x10bb249 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb270; continue; }
  pc = 0x10bb25d; continue;
 }
 0x10bb25d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb26d); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb26d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb270; continue;
 }
 0x10bb270 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x418u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb297; continue; }
  pc = 0x10bb284; continue;
 }
 0x10bb284 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xbu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb294); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb294 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb297; continue;
 }
 0x10bb297 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb2be; continue; }
  pc = 0x10bb2ab; continue;
 }
 0x10bb2ab => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb2bb); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb2bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb2be; continue;
 }
 0x10bb2be => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x498u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb2e5; continue; }
  pc = 0x10bb2d2; continue;
 }
 0x10bb2d2 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb2e2); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb2e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb2e5; continue;
 }
 0x10bb2e5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb30c; continue; }
  pc = 0x10bb2f9; continue;
 }
 0x10bb2f9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5du32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb309); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb309 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb30c; continue;
 }
 0x10bb30c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb332; continue; }
  pc = 0x10bb320; continue;
 }
 0x10bb320 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb32f); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb32f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  pc = 0x10bb332; continue;
 }
 0x10bb332 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bb340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb340); }
 let mut pc: u32 = 0x10bb340;
 loop { match pc {
 0x10bb340 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x10bb357; continue;
 }
 0x10bb357 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x10bb382; continue; }
  pc = 0x10bb361; continue;
 }
 0x10bb361 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb375); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb375 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bb382; continue;
 }
 0x10bb382 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x10bb357; continue; }
  pc = 0x10bb389; continue;
 }
 0x10bb389 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bb3cd; continue; }
  pc = 0x10bb395; continue;
 }
 0x10bb395 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bb3b6); f_010ae130(cpu); if cpu.returned_to != 0x10bb3b6 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bb3cd; continue;
 }
 0x10bb3cd => {
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
pub fn f_010bb3e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb3e0); }
 let mut pc: u32 = 0x10bb3e0;
 loop { match pc {
 0x10bb3e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  pc = 0x10bb3f5; continue;
 }
 0x10bb3f5 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb422; continue; }
  pc = 0x10bb400; continue;
 }
 0x10bb400 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x10bb415); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb415 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bb422; continue;
 }
 0x10bb422 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x10bb3f5; continue; }
  pc = 0x10bb429; continue;
 }
 0x10bb429 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x558u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bb477; continue; }
  pc = 0x10bb43f; continue;
 }
 0x10bb43f => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bb460); f_010ae130(cpu); if cpu.returned_to != 0x10bb460 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bb477; continue;
 }
 0x10bb477 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10bb48e; continue; }
  pc = 0x10bb47b; continue;
 }
 0x10bb47b => {
  cpu.push(0x10bb480); f_010ae070(cpu); if cpu.returned_to != 0x10bb480 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bb48e; continue; }
  pc = 0x10bb484; continue;
 }
 0x10bb484 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x10bb48b); f_010ae080(cpu); if cpu.returned_to != 0x10bb48b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10bb48e; continue;
 }
 0x10bb48e => {
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
pub fn f_010bb4a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb4a0); }
 let mut pc: u32 = 0x10bb4a0;
 loop { match pc {
 0x10bb4a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xf0u32; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, 0xb0u32); }
  if cpu.zf { pc = 0x10bb4f4; continue; }
  pc = 0x10bb4e4; continue;
 }
 0x10bb4e4 => {
  { cpu.sub32(cpu.edx, 0xc0u32); }
  if cpu.zf { pc = 0x10bb4f4; continue; }
  pc = 0x10bb4ec; continue;
 }
 0x10bb4ec => {
  { cpu.sub32(cpu.edx, 0xe0u32); }
  if !cpu.zf { pc = 0x10bb510; continue; }
  pc = 0x10bb4f4; continue;
 }
 0x10bb4f4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xd8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bb50d); f_010bb910(cpu); if cpu.returned_to != 0x10bb50d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10bb510; continue;
 }
 0x10bb510 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xb0u32); }
  if !cpu.zf { pc = 0x10bb647; continue; }
  pc = 0x10bb51e; continue;
 }
 0x10bb51e => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10bb545; continue; }
  pc = 0x10bb525; continue;
 }
 0x10bb525 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2d8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb545; continue; }
  pc = 0x10bb539; continue;
 }
 0x10bb539 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2d8u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bb545; continue;
 }
 0x10bb545 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, 0x6fu32); }
  if cpu.cf { pc = 0x10bb59c; continue; }
  pc = 0x10bb564; continue;
 }
 0x10bb564 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bb7d0; continue; }
  pc = 0x10bb56a; continue;
 }
 0x10bb56a => {
  { cpu.sub32(cpu.ebp, 0x75u32); }
  if cpu.cf { pc = 0x10bb58c; continue; }
  pc = 0x10bb56f; continue;
 }
 0x10bb56f => {
  if (cpu.cf || cpu.zf) { pc = 0x10bb749; continue; }
  pc = 0x10bb575; continue;
 }
 0x10bb575 => {
  { cpu.sub32(cpu.ebp, 0x76u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10bb6b8; continue; }
  pc = 0x10bb57e; continue;
 }
 0x10bb57e => {
  { cpu.sub32(cpu.ebp, 0x77u32); }
  if cpu.zf { pc = 0x10bb6f8; continue; }
  pc = 0x10bb587; continue;
 }
 0x10bb587 => {
  pc = 0x10bb647; continue;
 }
 0x10bb58c => {
  { cpu.sub32(cpu.ebp, 0x73u32); }
  if cpu.cf { pc = 0x10bb647; continue; }
  pc = 0x10bb595; continue;
 }
 0x10bb595 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bb5ca; continue; }
  pc = 0x10bb597; continue;
 }
 0x10bb597 => {
  pc = 0x10bb714; continue;
 }
 0x10bb59c => {
  { cpu.sub32(cpu.ebp, 0x6cu32); }
  if cpu.cf { pc = 0x10bb5b1; continue; }
  pc = 0x10bb5a1; continue;
 }
 0x10bb5a1 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bb5e3; continue; }
  pc = 0x10bb5a3; continue;
 }
 0x10bb5a3 => {
  { cpu.sub32(cpu.ebp, 0x6du32); }
  if (cpu.cf || cpu.zf) { pc = 0x10bb7be; continue; }
  pc = 0x10bb5ac; continue;
 }
 0x10bb5ac => {
  pc = 0x10bb7fd; continue;
 }
 0x10bb5b1 => {
  { cpu.sub32(cpu.ebp, 0x6u32); }
  if cpu.cf { pc = 0x10bb647; continue; }
  pc = 0x10bb5ba; continue;
 }
 0x10bb5ba => {
  if (cpu.cf || cpu.zf) { pc = 0x10bb605; continue; }
  pc = 0x10bb5bc; continue;
 }
 0x10bb5bc => {
  { cpu.sub32(cpu.ebp, 0x7u32); }
  if cpu.zf { pc = 0x10bb68b; continue; }
  pc = 0x10bb5c5; continue;
 }
 0x10bb5c5 => {
  pc = 0x10bb647; continue;
 }
 0x10bb5ca => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x10bb647; continue;
 }
 0x10bb5e3 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10bb647; continue; }
  pc = 0x10bb5ed; continue;
 }
 0x10bb5ed => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.push(0x10bb5f7); cpu.call(t); if cpu.returned_to != 0x10bb5f7 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x10bb647; continue;
 }
 0x10bb605 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb618); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb618 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x65u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb62e); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb62e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x26u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb644); f_010bb4a0(cpu); if cpu.returned_to != 0x10bb644 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x10bb647; continue;
 }
 0x10bb647 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x10bb663; continue; }
  pc = 0x10bb65a; continue;
 }
 0x10bb65a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x64u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb663; continue;
 }
 0x10bb663 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x90u32); }
  if !cpu.zf { pc = 0x10bb882; continue; }
  pc = 0x10bb680; continue;
 }
 0x10bb680 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bb890; continue;
 }
 0x10bb68b => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x1b4u32.wrapping_add(cpu.ebx))); cpu.edx = v; }
  { let v: u32 = 0x3f01u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bb6b0; continue; }
  pc = 0x10bb6ab; continue;
 }
 0x10bb6ab => {
  { let v: u32 = 0x7fu32; cpu.edi = v; }
  pc = 0x10bb6b0; continue;
 }
 0x10bb6b0 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10bb647; continue; }
  pc = 0x10bb6b4; continue;
 }
 0x10bb6b4 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.poll(); pc = 0x10bb647; continue;
 }
 0x10bb6b8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x64u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb6e3; continue;
 }
 0x10bb6e3 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.push(0x10bb6f0); cpu.call(t); if cpu.returned_to != 0x10bb6f0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb6f8 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb702; continue;
 }
 0x10bb702 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.push(0x10bb70c); cpu.call(t); if cpu.returned_to != 0x10bb70c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb714 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bb718; continue;
 }
 0x10bb718 => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb72a; continue; }
  pc = 0x10bb721; continue;
 }
 0x10bb721 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bb718; continue; }
  pc = 0x10bb72a; continue;
 }
 0x10bb72a => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb733; continue;
 }
 0x10bb733 => {
  { let v: u32 = cpu.edi; cpu.w32(0x88u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x78u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb749 => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10bb8ff; continue; }
  pc = 0x10bb752; continue;
 }
 0x10bb752 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x10bb75a; continue;
 }
 0x10bb75a => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x10bb76b; continue; }
  pc = 0x10bb763; continue;
 }
 0x10bb763 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10bb75a; continue; }
  pc = 0x10bb76b; continue;
 }
 0x10bb76b => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb774; continue;
 }
 0x10bb774 => {
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10bb791; continue; }
  pc = 0x10bb783; continue;
 }
 0x10bb783 => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb791 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10bb7ac; continue; }
  pc = 0x10bb79e; continue;
 }
 0x10bb79e => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb7ac => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb7be => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb7c5); f_010a8f70(cpu); if cpu.returned_to != 0x10bb7c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb7d0 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb7da; continue;
 }
 0x10bb7da => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x10bb7ee; continue; }
  pc = 0x10bb7df; continue;
 }
 0x10bb7df => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb7ee => {
  { let v: u32 = 0x2u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb7fd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x10bb845; continue; }
  pc = 0x10bb807; continue;
 }
 0x10bb807 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb811; continue;
 }
 0x10bb811 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bb81b); f_010a8570(cpu); if cpu.returned_to != 0x10bb81b { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb828; continue;
 }
 0x10bb828 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb830); f_010a83e0(cpu); if cpu.returned_to != 0x10bb830 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 2u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.esp))); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x60u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb845 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb851; continue;
 }
 0x10bb851 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb85c); f_010bb340(cpu); if cpu.returned_to != 0x10bb85c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bb86c); f_010a84b0(cpu); if cpu.returned_to != 0x10bb86c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bb877); f_010a83e0(cpu); if cpu.returned_to != 0x10bb877 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb882 => {
  { cpu.sub32(cpu.edx, 0x80u32); }
  if !cpu.zf { pc = 0x10bb890; continue; }
  pc = 0x10bb88a; continue;
 }
 0x10bb88a => {
  { let v: u32 = cpu.dec32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bb890; continue;
 }
 0x10bb890 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xe4u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if !cpu.zf { pc = 0x10bb8bd; continue; }
  pc = 0x10bb8af; continue;
 }
 0x10bb8af => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x218u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x10bb8ff; continue; }
  pc = 0x10bb8bd; continue;
 }
 0x10bb8bd => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1a4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x10bb8e5; continue; }
  pc = 0x10bb8ca; continue;
 }
 0x10bb8ca => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x1a4u32.wrapping_add(cpu.eax)); cpu.push(0x10bb8e1); cpu.call(t); if cpu.returned_to != 0x10bb8e1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bb8ff; continue; }
  pc = 0x10bb8e5; continue;
 }
 0x10bb8e5 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10bb8fc); f_010bbf50(cpu); if cpu.returned_to != 0x10bb8fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10bb8ff; continue;
 }
 0x10bb8ff => {
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bb910(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bb910); }
 let mut pc: u32 = 0x10bb910;
 loop { match pc {
 0x10bb910 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xf0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, 0xc0u32); }
  if cpu.cf { pc = 0x10bb94d; continue; }
  pc = 0x10bb93f; continue;
 }
 0x10bb93f => {
  if (cpu.cf || cpu.zf) { pc = 0x10bb959; continue; }
  pc = 0x10bb941; continue;
 }
 0x10bb941 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x10bb964; continue; }
  pc = 0x10bb949; continue;
 }
 0x10bb949 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb94d => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x10bb97a; continue; }
  pc = 0x10bb955; continue;
 }
 0x10bb955 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb959 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb964 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb97a => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x10bb9cb; continue; }
  pc = 0x10bb97f; continue;
 }
 0x10bb97f => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbad7; continue; }
  pc = 0x10bb985; continue;
 }
 0x10bb985 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x10bb9b5; continue; }
  pc = 0x10bb98a; continue;
 }
 0x10bb98a => {
  if (cpu.cf || cpu.zf) { pc = 0x10bba46; continue; }
  pc = 0x10bb990; continue;
 }
 0x10bb990 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x10bb9a8; continue; }
  pc = 0x10bb995; continue;
 }
 0x10bb995 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bba62; continue; }
  pc = 0x10bb99b; continue;
 }
 0x10bb99b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x10bba70; continue; }
  pc = 0x10bb9a4; continue;
 }
 0x10bb9a4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb9a8 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x10bba54; continue; }
  pc = 0x10bb9b1; continue;
 }
 0x10bb9b1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb9b5 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x10bb9be; continue; }
  pc = 0x10bb9ba; continue;
 }
 0x10bb9ba => {
  if (cpu.cf || cpu.zf) { pc = 0x10bba1c; continue; }
  pc = 0x10bb9bc; continue;
 }
 0x10bb9bc => {
  pc = 0x10bba2a; continue;
 }
 0x10bb9be => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x10bba38; continue; }
  pc = 0x10bb9c7; continue;
 }
 0x10bb9c7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb9cb => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x10bb9fb; continue; }
  pc = 0x10bb9d0; continue;
 }
 0x10bb9d0 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bba9c; continue; }
  pc = 0x10bb9d6; continue;
 }
 0x10bb9d6 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x10bb9ee; continue; }
  pc = 0x10bb9db; continue;
 }
 0x10bb9db => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbaba; continue; }
  pc = 0x10bb9e1; continue;
 }
 0x10bb9e1 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x10bbac9; continue; }
  pc = 0x10bb9ea; continue;
 }
 0x10bb9ea => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb9ee => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x10bbaab; continue; }
  pc = 0x10bb9f7; continue;
 }
 0x10bb9f7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bb9fb => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x10bba13; continue; }
  pc = 0x10bba00; continue;
 }
 0x10bba00 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbae5; continue; }
  pc = 0x10bba06; continue;
 }
 0x10bba06 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x10bba8d; continue; }
  pc = 0x10bba0f; continue;
 }
 0x10bba0f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba13 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10bba7e; continue; }
  pc = 0x10bba18; continue;
 }
 0x10bba18 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba1c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba2a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba38 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba46 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba54 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba62 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba70 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba7e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba8d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bba9c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbaab => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbaba => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbac9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbad7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbae5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bbb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bbb00); }
 let mut pc: u32 = 0x10bbb00;
 loop { match pc {
 0x10bbb00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.cf { pc = 0x10bbb39; continue; }
  pc = 0x10bbb2a; continue;
 }
 0x10bbb2a => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbb46; continue; }
  pc = 0x10bbb2c; continue;
 }
 0x10bbb2c => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x10bbb4b; continue; }
  pc = 0x10bbb34; continue;
 }
 0x10bbb34 => {
  pc = 0x10bbcaa; continue;
 }
 0x10bbb39 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.zf { pc = 0x10bbb5c; continue; }
  pc = 0x10bbb41; continue;
 }
 0x10bbb41 => {
  pc = 0x10bbcaa; continue;
 }
 0x10bbb46 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbb4b => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbb5c => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x10bbbc0; continue; }
  pc = 0x10bbb61; continue;
 }
 0x10bbb61 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbc97; continue; }
  pc = 0x10bbb67; continue;
 }
 0x10bbb67 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x10bbb9f; continue; }
  pc = 0x10bbb6c; continue;
 }
 0x10bbb6c => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbc38; continue; }
  pc = 0x10bbb72; continue;
 }
 0x10bbb72 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x10bbb8e; continue; }
  pc = 0x10bbb77; continue;
 }
 0x10bbb77 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbc4a; continue; }
  pc = 0x10bbb7d; continue;
 }
 0x10bbb7d => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x10bbc53; continue; }
  pc = 0x10bbb86; continue;
 }
 0x10bbb86 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbb8e => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x10bbc41; continue; }
  pc = 0x10bbb97; continue;
 }
 0x10bbb97 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbb9f => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x10bbbaf; continue; }
  pc = 0x10bbba4; continue;
 }
 0x10bbba4 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbc1d; continue; }
  pc = 0x10bbbaa; continue;
 }
 0x10bbbaa => {
  pc = 0x10bbc26; continue;
 }
 0x10bbbaf => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x10bbc2f; continue; }
  pc = 0x10bbbb8; continue;
 }
 0x10bbbb8 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbbc0 => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x10bbbf8; continue; }
  pc = 0x10bbbc5; continue;
 }
 0x10bbbc5 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbc70; continue; }
  pc = 0x10bbbcb; continue;
 }
 0x10bbbcb => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x10bbbe7; continue; }
  pc = 0x10bbbd0; continue;
 }
 0x10bbbd0 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbc84; continue; }
  pc = 0x10bbbd6; continue;
 }
 0x10bbbd6 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x10bbc8e; continue; }
  pc = 0x10bbbdf; continue;
 }
 0x10bbbdf => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbbe7 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x10bbc7a; continue; }
  pc = 0x10bbbf0; continue;
 }
 0x10bbbf0 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbbf8 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x10bbc10; continue; }
  pc = 0x10bbbfd; continue;
 }
 0x10bbbfd => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbca0; continue; }
  pc = 0x10bbc03; continue;
 }
 0x10bbc03 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x10bbc66; continue; }
  pc = 0x10bbc08; continue;
 }
 0x10bbc08 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc10 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10bbc5c; continue; }
  pc = 0x10bbc15; continue;
 }
 0x10bbc15 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc1d => {
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc26 => {
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc2f => {
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc38 => {
  { let v: u32 = cpu.r32(0x180u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc41 => {
  { let v: u32 = cpu.r32(0x1c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc4a => {
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc53 => {
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc5c => {
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc66 => {
  { let v: u32 = cpu.r32(0x2c0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc70 => {
  { let v: u32 = cpu.r32(0x300u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc7a => {
  { let v: u32 = cpu.r32(0x340u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc84 => {
  { let v: u32 = cpu.r32(0x380u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc8e => {
  { let v: u32 = cpu.r32(0x3c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbc97 => {
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbca0 => {
  { let v: u32 = cpu.r32(0x440u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbcaa => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bbcc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bbcc0); }
 let mut pc: u32 = 0x10bbcc0;
 loop { match pc {
 0x10bbcc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x10bbcd1; continue;
 }
 0x10bbcd1 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x116b09cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10bbce9; continue; }
  pc = 0x10bbce7; continue;
 }
 0x10bbce7 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10bbce9; continue;
 }
 0x10bbce9 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10bbd0b; continue; }
  pc = 0x10bbced; continue;
 }
 0x10bbced => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b0a4u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10bbd03; continue; }
  pc = 0x10bbd01; continue;
 }
 0x10bbd01 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10bbd03; continue;
 }
 0x10bbd03 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10bbdfe; continue; }
  pc = 0x10bbd0b; continue;
 }
 0x10bbd0b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp & 0xffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi & 0xff00u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 24u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xff0000u32; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x116b0acu32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10bbd57; continue; }
  pc = 0x10bbd55; continue;
 }
 0x10bbd55 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10bbd57; continue;
 }
 0x10bbd57 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x10bbcd1; continue; }
  pc = 0x10bbd5f; continue;
 }
 0x10bbd5f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b09cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10bbd75; continue; }
  pc = 0x10bbd73; continue;
 }
 0x10bbd73 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10bbd75; continue;
 }
 0x10bbd75 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10bbd88; continue; }
  pc = 0x10bbd79; continue;
 }
 0x10bbd79 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10bbdfe; continue; }
  pc = 0x10bbd81; continue;
 }
 0x10bbd81 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10bbe00; continue;
 }
 0x10bbd88 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x10bbdfe; continue; }
  pc = 0x10bbd95; continue;
 }
 0x10bbd95 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b0acu32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10bbdac; continue; }
  pc = 0x10bbdaa; continue;
 }
 0x10bbdaa => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10bbdac; continue;
 }
 0x10bbdac => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10bbdc0; continue; }
  pc = 0x10bbdb0; continue;
 }
 0x10bbdb0 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x10bbdc0; continue; }
  pc = 0x10bbdb6; continue;
 }
 0x10bbdb6 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bbdc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp & 0xffu32; cpu.ebp = v; }
  { let v: u32 = cpu.edi & 0xff00u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ebp, 24u32); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xff0000u32; cpu.edi = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if cpu.cf { cpu.poll(); pc = 0x10bbd95; continue; }
  pc = 0x10bbdfe; continue;
 }
 0x10bbdfe => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bbe00; continue;
 }
 0x10bbe00 => {
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
pub fn f_010bbe10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bbe10); }
 let mut pc: u32 = 0x10bbe10;
 loop { match pc {
 0x10bbe10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xff00u32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xff0000u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xff000000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 8u32); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 24u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bbe50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bbe50); }
 let mut pc: u32 = 0x10bbe50;
 loop { match pc {
 0x10bbe50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x10bbe5e; continue;
 }
 0x10bbe5e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi & 0x7fu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx | cpu.esi; cpu.ecx = v; }
  { cpu.and8((cpu.ebx as u8), 0x80u8); }
  if cpu.zf { pc = 0x10bbe79; continue; }
  pc = 0x10bbe76; continue;
 }
 0x10bbe76 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x10bbe5e; continue; }
  pc = 0x10bbe79; continue;
 }
 0x10bbe79 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bbe80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bbe80); }
 let mut pc: u32 = 0x10bbe80;
 loop { match pc {
 0x10bbe80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bbed0; continue; }
  pc = 0x10bbe98; continue;
 }
 0x10bbe98 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bbeb9); f_010ae130(cpu); if cpu.returned_to != 0x10bbeb9 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bbed0; continue;
 }
 0x10bbed0 => {
  { cpu.sub32(cpu.edi, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10bbedf; continue; }
  pc = 0x10bbed8; continue;
 }
 0x10bbed8 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x10bbee1; continue;
 }
 0x10bbedf => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x10bbee1; continue;
 }
 0x10bbee1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x100u32); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bbf45; continue; }
  pc = 0x10bbf0d; continue;
 }
 0x10bbf0d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bbf2e); f_010ae130(cpu); if cpu.returned_to != 0x10bbf2e { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bbf45; continue;
 }
 0x10bbf45 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bbf50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bbf50); }
 let mut pc: u32 = 0x10bbf50;
 loop { match pc {
 0x10bbf50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12d216cu32); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10bbf90; continue; }
  pc = 0x10bbf72; continue;
 }
 0x10bbf72 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x10bbf8b; continue; }
  pc = 0x10bbf80; continue;
 }
 0x10bbf80 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if !cpu.zf { pc = 0x10bbf90; continue; }
  pc = 0x10bbf87; continue;
 }
 0x10bbf87 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10bbf90; continue; }
  pc = 0x10bbf8b; continue;
 }
 0x10bbf8b => {
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x10bbf90; continue;
 }
 0x10bbf90 => {
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  pc = 0x10bbfa8; continue;
 }
 0x10bbfa8 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bc0c2; continue; }
  pc = 0x10bbfb2; continue;
 }
 0x10bbfb2 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.cf { pc = 0x10bbfda; continue; }
  pc = 0x10bbfba; continue;
 }
 0x10bbfba => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbff6; continue; }
  pc = 0x10bbfbc; continue;
 }
 0x10bbfbc => {
  { cpu.sub32(cpu.esi, 0xd0u32); }
  if cpu.cf { pc = 0x10bbfd0; continue; }
  pc = 0x10bbfc4; continue;
 }
 0x10bbfc4 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbffd; continue; }
  pc = 0x10bbfc6; continue;
 }
 0x10bbfc6 => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x10bbff6; continue; }
  pc = 0x10bbfce; continue;
 }
 0x10bbfce => {
  pc = 0x10bc004; continue;
 }
 0x10bbfd0 => {
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.zf { pc = 0x10bbffd; continue; }
  pc = 0x10bbfd8; continue;
 }
 0x10bbfd8 => {
  pc = 0x10bc004; continue;
 }
 0x10bbfda => {
  { cpu.sub32(cpu.esi, 0x90u32); }
  if cpu.cf { pc = 0x10bbfee; continue; }
  pc = 0x10bbfe2; continue;
 }
 0x10bbfe2 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bbff6; continue; }
  pc = 0x10bbfe4; continue;
 }
 0x10bbfe4 => {
  { cpu.sub32(cpu.esi, 0xa0u32); }
  if cpu.zf { pc = 0x10bbff6; continue; }
  pc = 0x10bbfec; continue;
 }
 0x10bbfec => {
  pc = 0x10bc004; continue;
 }
 0x10bbfee => {
  { cpu.sub32(cpu.esi, 0x80u32); }
  if !cpu.zf { pc = 0x10bc004; continue; }
  pc = 0x10bbff6; continue;
 }
 0x10bbff6 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x10bc006; continue;
 }
 0x10bbffd => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x10bc006; continue;
 }
 0x10bc004 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bc006; continue;
 }
 0x10bc006 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x10bc058; continue; }
  pc = 0x10bc017; continue;
 }
 0x10bc017 => {
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bc058; continue; }
  pc = 0x10bc020; continue;
 }
 0x10bc020 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bc041); f_010ae130(cpu); if cpu.returned_to != 0x10bc041 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bc058; continue;
 }
 0x10bc058 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x10bc0b7; continue; }
  pc = 0x10bc099; continue;
 }
 0x10bc099 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bc0b7; continue;
 }
 0x10bc0b7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx))); cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10bbfa8; continue;
 }
 0x10bc0c2 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bc0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bc0d0); }
 let mut pc: u32 = 0x10bc0d0;
 loop { match pc {
 0x10bc0d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bc119; continue; }
  pc = 0x10bc0e1; continue;
 }
 0x10bc0e1 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bc102); f_010ae130(cpu); if cpu.returned_to != 0x10bc102 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bc119; continue;
 }
 0x10bc119 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bc120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bc120); }
 let mut pc: u32 = 0x10bc120;
 loop { match pc {
 0x10bc120 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x10bc14f; continue; }
  pc = 0x10bc130; continue;
 }
 0x10bc130 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bc169; continue; }
  pc = 0x10bc132; continue;
 }
 0x10bc132 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x10bc145; continue; }
  pc = 0x10bc139; continue;
 }
 0x10bc139 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bc16f; continue; }
  pc = 0x10bc13b; continue;
 }
 0x10bc13b => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x10bc169; continue; }
  pc = 0x10bc142; continue;
 }
 0x10bc142 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bc145 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x10bc16f; continue; }
  pc = 0x10bc14c; continue;
 }
 0x10bc14c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bc14f => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x10bc162; continue; }
  pc = 0x10bc156; continue;
 }
 0x10bc156 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bc169; continue; }
  pc = 0x10bc158; continue;
 }
 0x10bc158 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x10bc169; continue; }
  pc = 0x10bc15f; continue;
 }
 0x10bc15f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bc162 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x10bc175; continue; }
  pc = 0x10bc169; continue;
 }
 0x10bc169 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bc16f => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bc175 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bc180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bc180); }
 let mut pc: u32 = 0x10bc180;
 loop { match pc {
 0x10bc180 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c92cu32), 0x0u32); }
  if !cpu.zf { pc = 0x10bc291; continue; }
  pc = 0x10bc190; continue;
 }
 0x10bc190 => {
  { let v = 0x10b89a0u32; cpu.push(v); }
  { let v = 0x10bc180u32; cpu.push(v); }
  cpu.push(0x10bc19f); f_010b47f0(cpu); if cpu.returned_to != 0x10bc19f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x117c8a8u32; cpu.push(v); }
  cpu.push(0x10bc1b1); f_010b4730(cpu); if cpu.returned_to != 0x10bc1b1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c928u32; cpu.push(v); }
  cpu.push(0x10bc1c0); f_010b4730(cpu); if cpu.returned_to != 0x10bc1c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21dcu32; cpu.push(v); }
  cpu.push(0x10bc1cf); f_010b4730(cpu); if cpu.returned_to != 0x10bc1cf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21e0u32; cpu.push(v); }
  cpu.push(0x10bc1de); f_010b4730(cpu); if cpu.returned_to != 0x10bc1de { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21e4u32; cpu.push(v); }
  cpu.push(0x10bc1ed); f_010b4730(cpu); if cpu.returned_to != 0x10bc1ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21e8u32; cpu.push(v); }
  cpu.push(0x10bc1fc); f_010b4730(cpu); if cpu.returned_to != 0x10bc1fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21f8u32; cpu.push(v); }
  cpu.push(0x10bc20b); f_010b4730(cpu); if cpu.returned_to != 0x10bc20b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21ecu32; cpu.push(v); }
  cpu.push(0x10bc21a); f_010b4730(cpu); if cpu.returned_to != 0x10bc21a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21f0u32; cpu.push(v); }
  cpu.push(0x10bc229); f_010b4730(cpu); if cpu.returned_to != 0x10bc229 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21d8u32; cpu.push(v); }
  cpu.push(0x10bc238); f_010b4730(cpu); if cpu.returned_to != 0x10bc238 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21f4u32; cpu.push(v); }
  cpu.push(0x10bc247); f_010b4730(cpu); if cpu.returned_to != 0x10bc247 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21d4u32; cpu.push(v); }
  cpu.push(0x10bc256); f_010b4730(cpu); if cpu.returned_to != 0x10bc256 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21d0u32; cpu.push(v); }
  cpu.push(0x10bc265); f_010b4730(cpu); if cpu.returned_to != 0x10bc265 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21c8u32; cpu.push(v); }
  cpu.push(0x10bc274); f_010b4730(cpu); if cpu.returned_to != 0x10bc274 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d21ccu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10bc288); f_010b4730(cpu); if cpu.returned_to != 0x10bc288 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c92cu32, v); }
  pc = 0x10bc291; continue;
 }
 0x10bc291 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bc2a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bc2a0); } r_010bc2a0(cpu, 0x10bc2a0); }
pub fn f_010bc4a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bc4a0); } r_010bc2a0(cpu, 0x10bc4a0); }
fn r_010bc2a0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10bc2a0 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x17000000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1e000000u32).wrapping_add((cpu.ebx as u8)); cpu.w8(0x1e000000u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x41000000u32).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x41000000u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1))).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10bc32a; continue; }
  pc = 0x10bc32a; continue;
 }
 0x10bc32a => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.pf() { pc = 0x10bc32e; continue; }
  pc = 0x10bc32e; continue;
 }
 0x10bc32e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add(0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x92000000u32.wrapping_add(cpu.edx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x92000000u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xa4000000u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ebx as u8)); cpu.w8(0xa4000000u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xb9000000u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xb9000000u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add((cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(((cpu.edx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x15000001u32).wrapping_add((cpu.eax as u8)); cpu.w8(0x15000001u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x37000001u32).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x37000001u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ecx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x1u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xd2000001u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0xd2000001u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)).wrapping_add(cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.ecx as u8)); cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10bc39c, "Outsb");
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xba000002u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0xba000002u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x3u32.wrapping_add(cpu.eax)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x3u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8).wrapping_add((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))).wrapping_add((cpu.edx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u8 = cpu.r8(0xdc000004u32.wrapping_add(cpu.esi)).wrapping_add((cpu.edx as u8)); cpu.w8(0xdc000004u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.eax >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x5740000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(5); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) & (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if (cpu.sf == cpu.of) { pc = 0x10bc3e8; continue; }
  pc = 0x10bc3e2; continue;
 }
 0x10bc3e2 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x10bc3ec; continue; }
  pc = 0x10bc3e6; continue;
 }
 0x10bc3e6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bc3e8; continue;
 }
 0x10bc3e8 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bc3ec; continue;
 }
 0x10bc3ec => {
  { let v: u8 = 0x7u8; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xa9000008u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0xb8000009u32).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.w8(0xb8000009u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)) | cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1))).wrapping_add((cpu.ecx as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ecx.wrapping_mul(1)), v); }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(((cpu.ecx >> 8) as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x4000000bu32.wrapping_add(cpu.eax)).wrapping_add((cpu.edx as u8)); cpu.w8(0x4000000bu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax | 0xe910000u32; cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.unimplemented(0x10bc41c, "Outsd");
  cpu.unimplemented(0x10bc41d, "Sldt");
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = cpu.adc8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x11u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x11u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x13u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x13u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), 0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.add8((cpu.ebx as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x17200000u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.sbb8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(((cpu.eax >> 8) as u8), ((cpu.edx >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.sbb32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x2300001bu32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x2300001bu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.sbb32(cpu.eax, 0x1ede0000u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u8 = (cpu.eax as u8) & cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0xe3000024u32.wrapping_add(cpu.esi)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0xe3000024u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ebp ^ cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { cpu.cmps(1, 0); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x2eu32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)).wrapping_add(((cpu.edx >> 8) as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10bc4a8; continue;
 }
 0x10bc4a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c93cu32), 0x0u32); }
  if cpu.zf { pc = 0x10bc4c6; continue; }
  pc = 0x10bc4ac; continue;
 }
 0x10bc4a8 => {
  { let v: u32 = cpu.add32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.eax); cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x10bc4c6; continue; }
  pc = 0x10bc4ac; continue;
 }
 0x10bc4ac => {
  { let v = 0x10bc4a0u32; cpu.push(v); }
  { let v = 0x10bcf10u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10bc4bd); f_010b4750(cpu); if cpu.returned_to != 0x10bc4bd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c93cu32, v); }
  pc = 0x10bc4c6; continue;
 }
 0x10bc4c6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bc4d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bc4d0); }
 let mut pc: u32 = 0x10bc4d0;
 loop { match pc {
 0x10bc4d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10bc52e; continue; }
  pc = 0x10bc4dc; continue;
 }
 0x10bc4dc => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bc507; continue; }
  pc = 0x10bc4e8; continue;
 }
 0x10bc4e8 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x10bc4ea; continue;
 }
 0x10bc4ea => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x10bc4fa); f_010acc30(cpu); if cpu.returned_to != 0x10bc4fa { return; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bc4ea; continue; }
  pc = 0x10bc507; continue;
 }
 0x10bc507 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bc513); f_010a8a20(cpu); if cpu.returned_to != 0x10bc513 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bc522); f_010a8910(cpu); if cpu.returned_to != 0x10bc522 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bc52b); f_010b4960(cpu); if cpu.returned_to != 0x10bc52b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10bc52e; continue;
 }
 0x10bc52e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bc740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bc740); }
 let mut pc: u32 = 0x10bc740;
 loop { match pc {
 0x10bc740 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xf0u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xfu32; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1b8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0xb0u32); }
  if cpu.zf { pc = 0x10bc776; continue; }
  pc = 0x10bc766; continue;
 }
 0x10bc766 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.zf { pc = 0x10bc776; continue; }
  pc = 0x10bc76e; continue;
 }
 0x10bc76e => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if !cpu.zf { pc = 0x10bc78d; continue; }
  pc = 0x10bc776; continue;
 }
 0x10bc776 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bc78a); f_010bcd40(cpu); if cpu.returned_to != 0x10bc78a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x10bc78d; continue;
 }
 0x10bc78d => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if !cpu.zf { pc = 0x10bc7a1; continue; }
  pc = 0x10bc795; continue;
 }
 0x10bc795 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x10bc7a1; continue; }
  pc = 0x10bc79c; continue;
 }
 0x10bc79c => {
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x10bc7a1; continue;
 }
 0x10bc7a1 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0xb0u32); }
  if cpu.cf { pc = 0x10bc7d7; continue; }
  pc = 0x10bc7b2; continue;
 }
 0x10bc7b2 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bc7fa; continue; }
  pc = 0x10bc7b4; continue;
 }
 0x10bc7b4 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.cf { pc = 0x10bcb58; continue; }
  pc = 0x10bc7c0; continue;
 }
 0x10bc7c0 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bc918; continue; }
  pc = 0x10bc7c6; continue;
 }
 0x10bc7c6 => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if cpu.zf { pc = 0x10bc938; continue; }
  pc = 0x10bc7d2; continue;
 }
 0x10bc7d2 => {
  pc = 0x10bcb58; continue;
 }
 0x10bc7d7 => {
  { cpu.sub32(cpu.ebx, 0x80u32); }
  if cpu.cf { pc = 0x10bcb58; continue; }
  pc = 0x10bc7e3; continue;
 }
 0x10bc7e3 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bcaf5; continue; }
  pc = 0x10bc7e9; continue;
 }
 0x10bc7e9 => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if cpu.zf { pc = 0x10bc983; continue; }
  pc = 0x10bc7f5; continue;
 }
 0x10bc7f5 => {
  pc = 0x10bcb58; continue;
 }
 0x10bc7fa => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7u32); }
  if cpu.cf { pc = 0x10bc824; continue; }
  pc = 0x10bc803; continue;
 }
 0x10bc803 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bc831; continue; }
  pc = 0x10bc805; continue;
 }
 0x10bc805 => {
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x10bcb58; continue; }
  pc = 0x10bc80e; continue;
 }
 0x10bc80e => {
  { cpu.sub32(cpu.edx, 0xbu32); }
  if (cpu.cf || cpu.zf) { pc = 0x10bc831; continue; }
  pc = 0x10bc813; continue;
 }
 0x10bc813 => {
  { cpu.sub32(cpu.edx, 0x7bu32); }
  if cpu.zf { pc = 0x10bc8c7; continue; }
  pc = 0x10bc81c; continue;
 }
 0x10bc81c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x10bc824 => {
  { cpu.sub32(cpu.edx, 0x6u32); }
  if cpu.zf { pc = 0x10bc87c; continue; }
  pc = 0x10bc829; continue;
 }
 0x10bc829 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x10bc831 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc841; continue;
 }
 0x10bc841 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x10bc843; continue;
 }
 0x10bc843 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x10bc868; continue; }
  pc = 0x10bc84b; continue;
 }
 0x10bc84b => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bc857); f_010ac130(cpu); if cpu.returned_to != 0x10bc857 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10bc868; continue; }
  pc = 0x10bc85f; continue;
 }
 0x10bc85f => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bc868); f_010bcca0(cpu); if cpu.returned_to != 0x10bc868 { return; }
  pc = 0x10bc868; continue;
 }
 0x10bc868 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc87a; continue;
 }
 0x10bc87a => {
  cpu.poll(); pc = 0x10bc843; continue;
 }
 0x10bc87c => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc88c; continue;
 }
 0x10bc88c => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x10bc88e; continue;
 }
 0x10bc88e => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x10bc8b3; continue; }
  pc = 0x10bc896; continue;
 }
 0x10bc896 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bc8a2); f_010ac130(cpu); if cpu.returned_to != 0x10bc8a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10bc8b3; continue; }
  pc = 0x10bc8aa; continue;
 }
 0x10bc8aa => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bc8b3); f_010bcb90(cpu); if cpu.returned_to != 0x10bc8b3 { return; }
  pc = 0x10bc8b3; continue;
 }
 0x10bc8b3 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc8c5; continue;
 }
 0x10bc8c5 => {
  cpu.poll(); pc = 0x10bc88e; continue;
 }
 0x10bc8c7 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc8d7; continue;
 }
 0x10bc8d7 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x10bc8d9; continue;
 }
 0x10bc8d9 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x10bc904; continue; }
  pc = 0x10bc8e1; continue;
 }
 0x10bc8e1 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bc8ed); f_010ac130(cpu); if cpu.returned_to != 0x10bc8ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10bc904; continue; }
  pc = 0x10bc8f5; continue;
 }
 0x10bc8f5 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bc901); f_010ac600(cpu); if cpu.returned_to != 0x10bc901 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10bc904; continue;
 }
 0x10bc904 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc916; continue;
 }
 0x10bc916 => {
  cpu.poll(); pc = 0x10bc8d9; continue;
 }
 0x10bc918 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x10bc92a); f_010bcd10(cpu); if cpu.returned_to != 0x10bc92a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x10bc938 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc948; continue;
 }
 0x10bc948 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x10bc94a; continue;
 }
 0x10bc94a => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x10bc96f; continue; }
  pc = 0x10bc952; continue;
 }
 0x10bc952 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bc95e); f_010ac130(cpu); if cpu.returned_to != 0x10bc95e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10bc96f; continue; }
  pc = 0x10bc966; continue;
 }
 0x10bc966 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bc96f); f_010bcb90(cpu); if cpu.returned_to != 0x10bc96f { return; }
  pc = 0x10bc96f; continue;
 }
 0x10bc96f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bc981; continue;
 }
 0x10bc981 => {
  cpu.poll(); pc = 0x10bc94a; continue;
 }
 0x10bc983 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if !cpu.zf { pc = 0x10bc99f; continue; }
  pc = 0x10bc988; continue;
 }
 0x10bc988 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x10bc999); f_010bcd10(cpu); if cpu.returned_to != 0x10bc999 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bc99f; continue;
 }
 0x10bc99f => {
  { cpu.sub32(cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x10bcb58; continue; }
  pc = 0x10bc9ad; continue;
 }
 0x10bc9ad => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bc9dd; continue; }
  pc = 0x10bc9b9; continue;
 }
 0x10bc9b9 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x10bc9bb; continue;
 }
 0x10bc9bb => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bc9c7); f_010ac130(cpu); if cpu.returned_to != 0x10bc9c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x10bc9dd; continue; }
  pc = 0x10bc9cf; continue;
 }
 0x10bc9cf => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bc9bb; continue; }
  pc = 0x10bc9dd; continue;
 }
 0x10bc9dd => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x514u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x10bca20; continue; }
  pc = 0x10bc9e5; continue;
 }
 0x10bc9e5 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bca10; continue; }
  pc = 0x10bc9f2; continue;
 }
 0x10bc9f2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x10bc9f4; continue;
 }
 0x10bc9f4 => {
  { let v: u32 = cpu.r32(0x658u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { pc = 0x10bca02; continue; }
  pc = 0x10bc9fe; continue;
 }
 0x10bc9fe => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x10bca02; continue;
 }
 0x10bca02 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bc9f4; continue; }
  pc = 0x10bca10; continue;
 }
 0x10bca10 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10bca1d); f_010ac600(cpu); if cpu.returned_to != 0x10bca1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10bca20; continue;
 }
 0x10bca20 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x518u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x558u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x598u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5d8u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x618u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x698u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x698u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x658u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  cpu.push(0x10bca82); f_010acb80(cpu); if cpu.returned_to != 0x10bca82 { return; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10bcaa1); f_010ac8c0(cpu); if cpu.returned_to != 0x10bcaa1 { return; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bcac5); f_010ac990(cpu); if cpu.returned_to != 0x10bcac5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bcad1); f_010bcb90(cpu); if cpu.returned_to != 0x10bcad1 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bcada); f_010bcca0(cpu); if cpu.returned_to != 0x10bcada { return; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10bcae7); f_010ac810(cpu); if cpu.returned_to != 0x10bcae7 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x10bcaf5 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x10bcb58; continue; }
  pc = 0x10bcafa; continue;
 }
 0x10bcafa => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcb58; continue; }
  pc = 0x10bcb06; continue;
 }
 0x10bcb06 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x10bcb08; continue;
 }
 0x10bcb08 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x10bcb4a; continue; }
  pc = 0x10bcb10; continue;
 }
 0x10bcb10 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x558u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x10bcb4a; continue; }
  pc = 0x10bcb1c; continue;
 }
 0x10bcb1c => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bcb28); f_010ac130(cpu); if cpu.returned_to != 0x10bcb28 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x10bcb4a; continue; }
  pc = 0x10bcb30; continue;
 }
 0x10bcb30 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bcb3c); f_010ac600(cpu); if cpu.returned_to != 0x10bcb3c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x10bcb4a => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bcb08; continue; }
  pc = 0x10bcb58; continue;
 }
 0x10bcb58 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcb60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcb60); }
 let mut pc: u32 = 0x10bcb60;
 loop { match pc {
 0x10bcb60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1b8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x10bcb7a); f_010bcd10(cpu); if cpu.returned_to != 0x10bcb7a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcb90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcb90); }
 let mut pc: u32 = 0x10bcb90;
 loop { match pc {
 0x10bcb90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x598u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5d8u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.shl32(cpu.edx, 7u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x2000u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcbe6; continue; }
  pc = 0x10bcbda; continue;
 }
 0x10bcbda => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1fffu32); cpu.eax = v; }
  pc = 0x10bcbfc; continue;
 }
 0x10bcbe6 => {
  if (cpu.sf == cpu.of) { pc = 0x10bcbf8; continue; }
  pc = 0x10bcbe8; continue;
 }
 0x10bcbe8 => {
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x10bcbfc; continue;
 }
 0x10bcbf8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10bcbfc; continue;
 }
 0x10bcbfc => {
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x454u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x10bcc0e; continue; }
  pc = 0x10bcc0c; continue;
 }
 0x10bcc0c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10bcc0e; continue;
 }
 0x10bcc0e => {
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x10bcc18; continue; }
  pc = 0x10bcc13; continue;
 }
 0x10bcc13 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x10bcc18; continue;
 }
 0x10bcc18 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10bc2a0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10bc2a0u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10bc2a0u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 13u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 13u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x10bcc69; continue; }
  pc = 0x10bcc52; continue;
 }
 0x10bcc52 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10bcc70; continue;
 }
 0x10bcc69 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x10bcc70; continue;
 }
 0x10bcc70 => {
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bcc89); f_010ac540(cpu); if cpu.returned_to != 0x10bcc89 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
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
pub fn f_010bcca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcca0); }
 let mut pc: u32 = 0x10bcca0;
 loop { match pc {
 0x10bcca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2d4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x354u32.wrapping_add(cpu.esi))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x618u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bcced); f_010ac480(cpu); if cpu.returned_to != 0x10bcced { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x314u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bcd03); f_010ac3c0(cpu); if cpu.returned_to != 0x10bcd03 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
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
pub fn f_010bcd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcd10); }
 let mut pc: u32 = 0x10bcd10;
 loop { match pc {
 0x10bcd10 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x10bcd16; continue;
 }
 0x10bcd16 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bcd32; continue; }
  pc = 0x10bcd1d; continue;
 }
 0x10bcd1d => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x10bcd26; continue; }
  pc = 0x10bcd21; continue;
 }
 0x10bcd21 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x10bcd34; continue; }
  pc = 0x10bcd26; continue;
 }
 0x10bcd26 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x200u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x10bcd16; continue; }
  pc = 0x10bcd32; continue;
 }
 0x10bcd32 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10bcd34; continue;
 }
 0x10bcd34 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcd40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcd40); }
 let mut pc: u32 = 0x10bcd40;
 loop { match pc {
 0x10bcd40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.ecx & 0xfu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.edi & 0xf0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.sub32(cpu.edi, 0xc0u32); }
  if cpu.cf { pc = 0x10bcd7d; continue; }
  pc = 0x10bcd6f; continue;
 }
 0x10bcd6f => {
  if (cpu.cf || cpu.zf) { pc = 0x10bcd89; continue; }
  pc = 0x10bcd71; continue;
 }
 0x10bcd71 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x10bcd94; continue; }
  pc = 0x10bcd79; continue;
 }
 0x10bcd79 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcd7d => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x10bcdaa; continue; }
  pc = 0x10bcd85; continue;
 }
 0x10bcd85 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcd89 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcd94 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcdaa => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.cf { pc = 0x10bcdfb; continue; }
  pc = 0x10bcdaf; continue;
 }
 0x10bcdaf => {
  if (cpu.cf || cpu.zf) { pc = 0x10bcee1; continue; }
  pc = 0x10bcdb5; continue;
 }
 0x10bcdb5 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x10bcde5; continue; }
  pc = 0x10bcdba; continue;
 }
 0x10bcdba => {
  if (cpu.cf || cpu.zf) { pc = 0x10bce5f; continue; }
  pc = 0x10bcdc0; continue;
 }
 0x10bcdc0 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x10bcdd8; continue; }
  pc = 0x10bcdc5; continue;
 }
 0x10bcdc5 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bce7b; continue; }
  pc = 0x10bcdcb; continue;
 }
 0x10bcdcb => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x10bce89; continue; }
  pc = 0x10bcdd4; continue;
 }
 0x10bcdd4 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcdd8 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x10bce6d; continue; }
  pc = 0x10bcde1; continue;
 }
 0x10bcde1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcde5 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x10bcdee; continue; }
  pc = 0x10bcdea; continue;
 }
 0x10bcdea => {
  if (cpu.cf || cpu.zf) { pc = 0x10bce43; continue; }
  pc = 0x10bcdec; continue;
 }
 0x10bcdec => {
  pc = 0x10bce51; continue;
 }
 0x10bcdee => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.zf { pc = 0x10bceef; continue; }
  pc = 0x10bcdf7; continue;
 }
 0x10bcdf7 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcdfb => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.cf { pc = 0x10bce2b; continue; }
  pc = 0x10bce00; continue;
 }
 0x10bce00 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bcea6; continue; }
  pc = 0x10bce06; continue;
 }
 0x10bce06 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.cf { pc = 0x10bce1e; continue; }
  pc = 0x10bce0b; continue;
 }
 0x10bce0b => {
  if (cpu.cf || cpu.zf) { pc = 0x10bcec4; continue; }
  pc = 0x10bce11; continue;
 }
 0x10bce11 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x10bced3; continue; }
  pc = 0x10bce1a; continue;
 }
 0x10bce1a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce1e => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.zf { pc = 0x10bceb5; continue; }
  pc = 0x10bce27; continue;
 }
 0x10bce27 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce2b => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x10bcf08; continue; }
  pc = 0x10bce34; continue;
 }
 0x10bce34 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bce97; continue; }
  pc = 0x10bce36; continue;
 }
 0x10bce36 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.zf { pc = 0x10bcefd; continue; }
  pc = 0x10bce3f; continue;
 }
 0x10bce3f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce43 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce51 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce5f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce6d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce7b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce89 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bce97 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcea6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bceb5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcec4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bced3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcee1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bceef => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bcefd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  pc = 0x10bcf08; continue;
 }
 0x10bcf08 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcf10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcf10); }
 let mut pc: u32 = 0x10bcf10;
 loop { match pc {
 0x10bcf10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c93cu32), 0x0u32); }
  if !cpu.zf { pc = 0x10bcf39; continue; }
  pc = 0x10bcf1c; continue;
 }
 0x10bcf1c => {
  { let v = 0x10bc4a0u32; cpu.push(v); }
  { let v = 0x10bcf10u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10bcf30); f_010b47f0(cpu); if cpu.returned_to != 0x10bcf30 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c93cu32, v); }
  pc = 0x10bcf39; continue;
 }
 0x10bcf39 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcf40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcf40); }
 let mut pc: u32 = 0x10bcf40;
 loop { match pc {
 0x10bcf40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bcf4b); f_010a7b20(cpu); if cpu.returned_to != 0x10bcf4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x10bcf6b; continue; }
  pc = 0x10bcf53; continue;
 }
 0x10bcf53 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10bcf68); f_010a7510(cpu); if cpu.returned_to != 0x10bcf68 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10bcf6b; continue;
 }
 0x10bcf6b => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bcf71); f_010a7b20(cpu); if cpu.returned_to != 0x10bcf71 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcf80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcf80); }
 let mut pc: u32 = 0x10bcf80;
 loop { match pc {
 0x10bcf80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bcf8c); f_010a7b20(cpu); if cpu.returned_to != 0x10bcf8c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10bcfc0; continue; }
  pc = 0x10bcf94; continue;
 }
 0x10bcf94 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bcf9a); f_010a7730(cpu); if cpu.returned_to != 0x10bcf9a { return; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10bcfc0); f_010bdb50(cpu); if cpu.returned_to != 0x10bcfc0 { return; }
  pc = 0x10bcfc0; continue;
 }
 0x10bcfc0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bcfc6); f_010a7b20(cpu); if cpu.returned_to != 0x10bcfc6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bcfd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bcfd0); }
 let mut pc: u32 = 0x10bcfd0;
 loop { match pc {
 0x10bcfd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bcfdc); f_010a7b20(cpu); if cpu.returned_to != 0x10bcfdc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd01c; continue; }
  pc = 0x10bcfe3; continue;
 }
 0x10bcfe3 => {
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10bd003); f_010bdb50(cpu); if cpu.returned_to != 0x10bd003 { return; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10bd01c); f_010bdb50(cpu); if cpu.returned_to != 0x10bd01c { return; }
  pc = 0x10bd01c; continue;
 }
 0x10bd01c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bd022); f_010a7b20(cpu); if cpu.returned_to != 0x10bd022 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd030); }
 let mut pc: u32 = 0x10bd030;
 loop { match pc {
 0x10bd030 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bd042); f_010a7b20(cpu); if cpu.returned_to != 0x10bd042 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd0af; continue; }
  pc = 0x10bd049; continue;
 }
 0x10bd049 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x10bd067; continue; }
  pc = 0x10bd04e; continue;
 }
 0x10bd04e => {
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10bd067); f_010bdb50(cpu); if cpu.returned_to != 0x10bd067 { return; }
  pc = 0x10bd067; continue;
 }
 0x10bd067 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if cpu.zf { pc = 0x10bd0af; continue; }
  pc = 0x10bd06f; continue;
 }
 0x10bd06f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x84u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10bd093); f_010bd9e0(cpu); if cpu.returned_to != 0x10bd093 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10bd09d); f_010bd9e0(cpu); if cpu.returned_to != 0x10bd09d { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10bd0af); f_010bdb50(cpu); if cpu.returned_to != 0x10bd0af { return; }
  pc = 0x10bd0af; continue;
 }
 0x10bd0af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bd0b5); f_010a7b20(cpu); if cpu.returned_to != 0x10bd0b5 { return; }
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
pub fn f_010bd0c0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bd0c0); } r_010bd0c0(cpu, 0x10bd0c0); }
pub fn f_010bd0f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10bd0f0); } r_010bd0c0(cpu, 0x10bd0f0); }
fn r_010bd0c0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10bd0c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bd0ca); f_010a7b20(cpu); if cpu.returned_to != 0x10bd0ca { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10bd0e2; continue; }
  pc = 0x10bd0d2; continue;
 }
 0x10bd0d2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10bd580; continue;
 }
 0x10bd0e2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd0f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bd0fa); f_010a7b20(cpu); if cpu.returned_to != 0x10bd0fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x10bd10f; continue; }
  pc = 0x10bd102; continue;
 }
 0x10bd102 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x10bd580; continue;
 }
 0x10bd10f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd580 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u16 = 0xbu16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x10bd5d9); f_010bdb50(cpu); if cpu.returned_to != 0x10bd5d9 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd672; continue; }
  pc = 0x10bd5e1; continue;
 }
 0x10bd5e1 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x10bd646; continue; }
  pc = 0x10bd5e5; continue;
 }
 0x10bd5e5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r8(0xau32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x10bd672; continue;
 }
 0x10bd646 => {
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10bd672; continue;
 }
 0x10bd672 => {
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
pub fn f_010bd120(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd120); }
 let mut pc: u32 = 0x10bd120;
 loop { match pc {
 0x10bd120 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bd131); f_010a7b20(cpu); if cpu.returned_to != 0x10bd131 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd1c5; continue; }
  pc = 0x10bd13c; continue;
 }
 0x10bd13c => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bd146); f_010a7a20(cpu); if cpu.returned_to != 0x10bd146 { return; }
  { let v: u32 = cpu.r32(0x117c948u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.ebx | cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x10bd1c3; continue; }
  pc = 0x10bd168; continue;
 }
 0x10bd168 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x10bd17f); f_010a7930(cpu); if cpu.returned_to != 0x10bd17f { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x117c944u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx | cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117c944u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 24u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx | cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x10bd168; continue; }
  pc = 0x10bd1c3; continue;
 }
 0x10bd1c3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10bd1c5; continue;
 }
 0x10bd1c5 => {
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
pub fn f_010bd1d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd1d0); }
 let mut pc: u32 = 0x10bd1d0;
 loop { match pc {
 0x10bd1d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bd1ea); f_010a7b20(cpu); if cpu.returned_to != 0x10bd1ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd1fb; continue; }
  pc = 0x10bd1f1; continue;
 }
 0x10bd1f1 => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.cf { pc = 0x10bd1fb; continue; }
  pc = 0x10bd1f6; continue;
 }
 0x10bd1f6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.ebx))); }
  if (cpu.cf || cpu.zf) { pc = 0x10bd215; continue; }
  pc = 0x10bd1fb; continue;
 }
 0x10bd1fb => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10bd205; continue; }
  pc = 0x10bd1ff; continue;
 }
 0x10bd1ff => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10bd205; continue;
 }
 0x10bd205 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10bd22b; continue; }
  pc = 0x10bd209; continue;
 }
 0x10bd209 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd215 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10bd21f; continue; }
  pc = 0x10bd219; continue;
 }
 0x10bd219 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x10bd21f; continue;
 }
 0x10bd21f => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x10bd22b; continue; }
  pc = 0x10bd223; continue;
 }
 0x10bd223 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bd22b; continue;
 }
 0x10bd22b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd230(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd230); }
 let mut pc: u32 = 0x10bd230;
 loop { match pc {
 0x10bd230 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bd23a); f_010a7b20(cpu); if cpu.returned_to != 0x10bd23a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bd242; continue; }
  pc = 0x10bd241; continue;
 }
 0x10bd241 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd242 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd250); }
 let mut pc: u32 = 0x10bd250;
 loop { match pc {
 0x10bd250 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bd2a9; continue; }
  pc = 0x10bd259; continue;
 }
 0x10bd259 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x10bd260); f_010bd3c0(cpu); if cpu.returned_to != 0x10bd260 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd29b; continue; }
  pc = 0x10bd266; continue;
 }
 0x10bd266 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10bd281; continue; }
  pc = 0x10bd26c; continue;
 }
 0x10bd26c => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x10bd276); f_010bd880(cpu); if cpu.returned_to != 0x10bd276 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd29b; continue; }
  pc = 0x10bd27a; continue;
 }
 0x10bd27a => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bd281; continue;
 }
 0x10bd281 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x10bd28a; continue; }
  pc = 0x10bd286; continue;
 }
 0x10bd286 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd28a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd29b => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bd2a9; continue;
 }
 0x10bd2a9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd2b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd2b0); }
 let mut pc: u32 = 0x10bd2b0;
 loop { match pc {
 0x10bd2b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bd2cc; continue; }
  pc = 0x10bd2b9; continue;
 }
 0x10bd2b9 => {
  { let v: u32 = 0x5u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x10bd2c5); f_010bd680(cpu); if cpu.returned_to != 0x10bd2c5 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bd2cc; continue;
 }
 0x10bd2cc => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd2f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd2f0); }
 let mut pc: u32 = 0x10bd2f0;
 loop { match pc {
 0x10bd2f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10bd33a; continue; }
  pc = 0x10bd2fa; continue;
 }
 0x10bd2fa => {
  { let v: u32 = cpu.r32(0x117c940u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bd32d; continue; }
  pc = 0x10bd304; continue;
 }
 0x10bd304 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x117c940u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x10bd32d; continue; }
  pc = 0x10bd311; continue;
 }
 0x10bd311 => {
  { let v: u32 = cpu.r32(0x12d2404u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bd32a); f_010b4890(cpu); if cpu.returned_to != 0x10bd32a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x10bd32d; continue;
 }
 0x10bd32d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bd337); f_010b4960(cpu); if cpu.returned_to != 0x10bd337 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10bd33a; continue;
 }
 0x10bd33a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd340); }
 let mut pc: u32 = 0x10bd340;
 loop { match pc {
 0x10bd340 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0x1a8u32; cpu.push(v); }
  cpu.push(0x10bd352); f_010b4990(cpu); if cpu.returned_to != 0x10bd352 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd3aa; continue; }
  pc = 0x10bd35b; continue;
 }
 0x10bd35b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10bd362); f_010bd730(cpu); if cpu.returned_to != 0x10bd362 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if !cpu.zf { pc = 0x10bd378; continue; }
  pc = 0x10bd36b; continue;
 }
 0x10bd36b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bd371); f_010b4960(cpu); if cpu.returned_to != 0x10bd371 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x10bd3aa; continue;
 }
 0x10bd378 => {
  { let v: u32 = 0x1a8u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = cpu.and8((cpu.ebx as u8), 0x3u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x10bd3aa; continue;
 }
 0x10bd3aa => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd3c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd3c0); }
 let mut pc: u32 = 0x10bd3c0;
 loop { match pc {
 0x10bd3c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u16 = 0x5u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x10bd411); f_010bdb50(cpu); if cpu.returned_to != 0x10bd411 { return; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x10bd41f; continue; }
  pc = 0x10bd419; continue;
 }
 0x10bd419 => {
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x10bd423; continue; }
  pc = 0x10bd41f; continue;
 }
 0x10bd41f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10bd42f; continue;
 }
 0x10bd423 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10bd42f; continue;
 }
 0x10bd42f => {
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
pub fn f_010bd440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd440); }
 let mut pc: u32 = 0x10bd440;
 loop { match pc {
 0x10bd440 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x10bd45a); f_010bdb50(cpu); if cpu.returned_to != 0x10bd45a { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd680); }
 let mut pc: u32 = 0x10bd680;
 loop { match pc {
 0x10bd680 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bd6d5); f_010bdb50(cpu); if cpu.returned_to != 0x10bd6d5 { return; }
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
pub fn f_010bd730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd730); }
 let mut pc: u32 = 0x10bd730;
 loop { match pc {
 0x10bd730 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xa4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
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
  { let v: u32 = 0x1500u32; cpu.edx = v; }
  { let v: u32 = 0x300u32; cpu.ecx = v; }
  { let v: u32 = 0x2fu32; cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x54u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x74u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3au32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10bd7a9); cpu.call(t); if cpu.returned_to != 0x10bd7a9 { return; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { pc = 0x10bd7b9; continue; }
  pc = 0x10bd7af; continue;
 }
 0x10bd7af => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x10bd86c; continue;
 }
 0x10bd7b9 => {
  { cpu.sub32(cpu.r32(0x117c940u32), 0x0u32); }
  if !cpu.zf { pc = 0x10bd7f7; continue; }
  pc = 0x10bd7c2; continue;
 }
 0x10bd7c2 => {
  { let v = 0x12d2404u32; cpu.push(v); }
  { let v = 0x12d23fcu32; cpu.push(v); }
  { let v = 0x12d2400u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x10bd7d8); f_010b48d0(cpu); if cpu.returned_to != 0x10bd7d8 { return; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x10bd7f7; continue; }
  pc = 0x10bd7e5; continue;
 }
 0x10bd7e5 => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0xa4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bd7f7 => {
  { let v: u32 = 0x150du32; cpu.ecx = v; }
  { let v: u32 = 0x300u32; cpu.ebp = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.eax = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x2fu32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x38u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x90u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x3au32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10bd852); cpu.call(t); if cpu.returned_to != 0x10bd852 { return; }
  { let v: u32 = cpu.r32(0x117c940u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c940u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x10bd86c; continue;
 }
 0x10bd86c => {
  { let v: u32 = cpu.add32(cpu.esp, 0xa4u32); cpu.esp = v; }
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
pub fn f_010bd880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd880); }
 let mut pc: u32 = 0x10bd880;
 loop { match pc {
 0x10bd880 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x12d2400u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x21u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10bd8e1); f_010bdb50(cpu); if cpu.returned_to != 0x10bd8e1 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd9a3; continue; }
  pc = 0x10bd8e9; continue;
 }
 0x10bd8e9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xbu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10bd970; continue;
 }
 0x10bd911 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x12d23fcu32); cpu.ebx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x10bd943); f_010bdb50(cpu); if cpu.returned_to != 0x10bd943 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bd9a3; continue; }
  pc = 0x10bd947; continue;
 }
 0x10bd947 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10bd959); f_010bdab0(cpu); if cpu.returned_to != 0x10bd959 { return; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x10bd970; continue;
 }
 0x10bd970 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x10bd911; continue; }
  pc = 0x10bd97d; continue;
 }
 0x10bd97d => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x10bd992); f_010bdab0(cpu); if cpu.returned_to != 0x10bd992 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x10bd9a5; continue;
 }
 0x10bd9a3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bd9a5; continue;
 }
 0x10bd9a5 => {
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
pub fn f_010bd9b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd9b0); }
 let mut pc: u32 = 0x10bd9b0;
 loop { match pc {
 0x10bd9b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bd9e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bd9e0); }
 let mut pc: u32 = 0x10bd9e0;
 loop { match pc {
 0x10bd9e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x3cu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xea60u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = 0xffffff6au32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bda50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bda50); }
 let mut pc: u32 = 0x10bda50;
 loop { match pc {
 0x10bda50 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 5u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 3u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdab0); }
 let mut pc: u32 = 0x10bdab0;
 loop { match pc {
 0x10bdab0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 5u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 3u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x4bu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdb20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdb20); }
 let mut pc: u32 = 0x10bdb20;
 loop { match pc {
 0x10bdb20 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x10bdb41); f_010bdb50(cpu); if cpu.returned_to != 0x10bdb41 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdb50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdb50); }
 let mut pc: u32 = 0x10bdb50;
 loop { match pc {
 0x10bdb50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x70u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x6cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x32u32; cpu.ecx = v; }
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
  { let v: u32 = 0x1510u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v: u32 = 0x2fu32; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0xffffu16; cpu.w16(0x3u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x54u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x54u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x3au32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10bdbf2); cpu.call(t); if cpu.returned_to != 0x10bdbf2 { return; }
  { cpu.and8(cpu.r8(0x4u32.wrapping_add(cpu.esi)), 0x80u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x74u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdc10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdc10); }
 let mut pc: u32 = 0x10bdc10;
 loop { match pc {
 0x10bdc10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = 0x300u32; cpu.ebx = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x20u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = 0x0u16; cpu.w16(0x2eu32.wrapping_add(cpu.edx), v); }
  let t = cpu.r32(0x117bb40u32); cpu.push(0x10bdc4a); cpu.call(t); if cpu.returned_to != 0x10bdc4a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdc50); }
 let mut pc: u32 = 0x10bdc50;
 loop { match pc {
 0x10bdc50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c958u32), 0x0u32); }
  if cpu.zf { pc = 0x10bdc76; continue; }
  pc = 0x10bdc5c; continue;
 }
 0x10bdc5c => {
  { let v = 0x10bdc50u32; cpu.push(v); }
  { let v = 0x10be3b0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x10bdc6d); f_010b4750(cpu); if cpu.returned_to != 0x10bdc6d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c958u32, v); }
  pc = 0x10bdc76; continue;
 }
 0x10bdc76 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdc80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdc80); }
 let mut pc: u32 = 0x10bdc80;
 loop { match pc {
 0x10bdc80 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bdc8e; continue; }
  pc = 0x10bdc88; continue;
 }
 0x10bdc88 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bdc8e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdca0); }
 let mut pc: u32 = 0x10bdca0;
 loop { match pc {
 0x10bdca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdcac); f_010a7090(cpu); if cpu.returned_to != 0x10bdcac { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bdd21; continue; }
  pc = 0x10bdcbb; continue;
 }
 0x10bdcbb => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bdcc6); f_010a6ed0(cpu); if cpu.returned_to != 0x10bdcc6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bdcdb; continue; }
  pc = 0x10bdccd; continue;
 }
 0x10bdccd => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x10bdcd3); f_010a6fe0(cpu); if cpu.returned_to != 0x10bdcd3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bdcdb => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10bdd1f; continue; }
  pc = 0x10bdce2; continue;
 }
 0x10bdce2 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x10bdd1f; continue; }
  pc = 0x10bdce9; continue;
 }
 0x10bdce9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x10bdd1f; continue; }
  pc = 0x10bdcf1; continue;
 }
 0x10bdcf1 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bdcf8; continue; }
  pc = 0x10bdcf3; continue;
 }
 0x10bdcf3 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  pc = 0x10bdd1d; continue;
 }
 0x10bdcf8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdd01); f_010aa010(cpu); if cpu.returned_to != 0x10bdd01 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { cpu.poll(); pc = 0x10bdcf8; continue; }
  pc = 0x10bdd09; continue;
 }
 0x10bdd09 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bdd0e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bdd17); f_010ac130(cpu); if cpu.returned_to != 0x10bdd17 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  pc = 0x10bdd1d; continue;
 }
 0x10bdd1d => {
  if cpu.zf { cpu.poll(); pc = 0x10bdd0e; continue; }
  pc = 0x10bdd1f; continue;
 }
 0x10bdd1f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x10bdd21; continue;
 }
 0x10bdd21 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdd30); }
 let mut pc: u32 = 0x10bdd30;
 loop { match pc {
 0x10bdd30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bdd73; continue; }
  pc = 0x10bdd39; continue;
 }
 0x10bdd39 => {
  cpu.push(0x10bdd3e); f_010ae5d0(cpu); if cpu.returned_to != 0x10bdd3e { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bdd6e; continue; }
  pc = 0x10bdd45; continue;
 }
 0x10bdd45 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x10bdd67; continue; }
  pc = 0x10bdd4d; continue;
 }
 0x10bdd4d => {
  if (cpu.cf || cpu.zf) { pc = 0x10bdd56; continue; }
  pc = 0x10bdd4f; continue;
 }
 0x10bdd4f => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x10bdd5e; continue; }
  pc = 0x10bdd54; continue;
 }
 0x10bdd54 => {
  pc = 0x10bdd67; continue;
 }
 0x10bdd56 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdd5c); f_010aa370(cpu); if cpu.returned_to != 0x10bdd5c { return; }
  pc = 0x10bdd64; continue;
 }
 0x10bdd5e => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdd64); f_010ac600(cpu); if cpu.returned_to != 0x10bdd64 { return; }
  pc = 0x10bdd64; continue;
 }
 0x10bdd64 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10bdd67; continue;
 }
 0x10bdd67 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bdd6e; continue;
 }
 0x10bdd6e => {
  cpu.push(0x10bdd73); f_010ae5c0(cpu); if cpu.returned_to != 0x10bdd73 { return; }
  pc = 0x10bdd73; continue;
 }
 0x10bdd73 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdd80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdd80); }
 let mut pc: u32 = 0x10bdd80;
 loop { match pc {
 0x10bdd80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bdee2; continue; }
  pc = 0x10bdd90; continue;
 }
 0x10bdd90 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x10bdd9f; continue; }
  pc = 0x10bdd96; continue;
 }
 0x10bdd96 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bdd9c); f_010a6e20(cpu); if cpu.returned_to != 0x10bdd9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10bdd9f; continue;
 }
 0x10bdd9f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x10bdee2; continue; }
  pc = 0x10bddab; continue;
 }
 0x10bddab => {
  if (cpu.cf || cpu.zf) { pc = 0x10bddbb; continue; }
  pc = 0x10bddad; continue;
 }
 0x10bddad => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x10bde49; continue; }
  pc = 0x10bddb6; continue;
 }
 0x10bddb6 => {
  pc = 0x10bdee2; continue;
 }
 0x10bddbb => {
  { let v: u32 = cpu.r32(0x12d240cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bddc7); f_010aa800(cpu); if cpu.returned_to != 0x10bddc7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bdee4; continue; }
  pc = 0x10bddd5; continue;
 }
 0x10bddd5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bdde0); f_010aa630(cpu); if cpu.returned_to != 0x10bdde0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x10bddfb; continue; }
  pc = 0x10bdde8; continue;
 }
 0x10bdde8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bddf1); f_010aa750(cpu); if cpu.returned_to != 0x10bddf1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bddfb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10bde07); f_010a8840(cpu); if cpu.returned_to != 0x10bde07 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x10be370u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bde18); f_010a8c40(cpu); if cpu.returned_to != 0x10bde18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bde29); f_010aa110(cpu); if cpu.returned_to != 0x10bde29 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10bde3c); f_010aa580(cpu); if cpu.returned_to != 0x10bde3c { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bde49 => {
  { let v: u32 = cpu.r32(0x12d2408u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bde55); f_010ace00(cpu); if cpu.returned_to != 0x10bde55 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bdee4; continue; }
  pc = 0x10bde63; continue;
 }
 0x10bde63 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bde69); f_010acb80(cpu); if cpu.returned_to != 0x10bde69 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bde7a); f_010aca60(cpu); if cpu.returned_to != 0x10bde7a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bde94; continue; }
  pc = 0x10bde81; continue;
 }
 0x10bde81 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10bde8a); f_010acc30(cpu); if cpu.returned_to != 0x10bde8a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bde94 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdea0); f_010aadd0(cpu); if cpu.returned_to != 0x10bdea0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x10be330u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10bdeb1); f_010aafb0(cpu); if cpu.returned_to != 0x10bdeb1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bdec2); f_010ac300(cpu); if cpu.returned_to != 0x10bdec2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x10bded5); f_010ac810(cpu); if cpu.returned_to != 0x10bded5 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bdee2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10bdee4; continue;
 }
 0x10bdee4 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdef0); }
 let mut pc: u32 = 0x10bdef0;
 loop { match pc {
 0x10bdef0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x10bdf8d; continue; }
  pc = 0x10bdefe; continue;
 }
 0x10bdefe => {
  cpu.push(0x10bdf03); f_010ae5d0(cpu); if cpu.returned_to != 0x10bdf03 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x10bdf33; continue; }
  pc = 0x10bdf0a; continue;
 }
 0x10bdf0a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x10bdf2c; continue; }
  pc = 0x10bdf12; continue;
 }
 0x10bdf12 => {
  if (cpu.cf || cpu.zf) { pc = 0x10bdf1b; continue; }
  pc = 0x10bdf14; continue;
 }
 0x10bdf14 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x10bdf23; continue; }
  pc = 0x10bdf19; continue;
 }
 0x10bdf19 => {
  pc = 0x10bdf2c; continue;
 }
 0x10bdf1b => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdf21); f_010aa370(cpu); if cpu.returned_to != 0x10bdf21 { return; }
  pc = 0x10bdf29; continue;
 }
 0x10bdf23 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdf29); f_010ac600(cpu); if cpu.returned_to != 0x10bdf29 { return; }
  pc = 0x10bdf29; continue;
 }
 0x10bdf29 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x10bdf2c; continue;
 }
 0x10bdf2c => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bdf33; continue;
 }
 0x10bdf33 => {
  cpu.push(0x10bdf38); f_010ae5c0(cpu); if cpu.returned_to != 0x10bdf38 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10bdf4d; continue; }
  pc = 0x10bdf3e; continue;
 }
 0x10bdf3e => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x10bdf44); f_010b4960(cpu); if cpu.returned_to != 0x10bdf44 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10bdf4d; continue;
 }
 0x10bdf4d => {
  { let v: u32 = cpu.r32(0x117c954u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x10bdf60; continue; }
  pc = 0x10bdf56; continue;
 }
 0x10bdf56 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117c954u32, v); }
  pc = 0x10bdf84; continue;
 }
 0x10bdf60 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bdf84; continue; }
  pc = 0x10bdf64; continue;
 }
 0x10bdf64 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x10bdf7d; continue; }
  pc = 0x10bdf6b; continue;
 }
 0x10bdf6b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bdf77); f_010b4960(cpu); if cpu.returned_to != 0x10bdf77 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10bdf7d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x10bdf64; continue; }
  pc = 0x10bdf84; continue;
 }
 0x10bdf84 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10bdf8a); f_010b4960(cpu); if cpu.returned_to != 0x10bdf8a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x10bdf8d; continue;
 }
 0x10bdf8d => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bdf90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bdf90); }
 let mut pc: u32 = 0x10bdf90;
 loop { match pc {
 0x10bdf90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x10bdf9e); f_010b4990(cpu); if cpu.returned_to != 0x10bdf9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bdfc7; continue; }
  pc = 0x10bdfaa; continue;
 }
 0x10bdfaa => {
  { let v: u32 = 0x116b318u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10bdfb6; continue;
 }
 0x10bdfb6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10bdfb6; continue; }
  pc = 0x10bdfc0; continue;
 }
 0x10bdfc0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10be210; continue;
 }
 0x10bdfc7 => {
  { let v: u32 = 0x14u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
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
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10bdff5); f_010b45a0(cpu); if cpu.returned_to != 0x10bdff5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10be028; continue; }
  pc = 0x10bdfff; continue;
 }
 0x10bdfff => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x10be005); f_010a6fe0(cpu); if cpu.returned_to != 0x10be005 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x116b328u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10be014; continue;
 }
 0x10be014 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10be014; continue; }
  pc = 0x10be01e; continue;
 }
 0x10be01e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10be028 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x116b338u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x10be034; continue;
 }
 0x10be034 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be042; continue; }
  pc = 0x10be03c; continue;
 }
 0x10be03c => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be042; continue; }
  pc = 0x10be040; continue;
 }
 0x10be040 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be042; continue;
 }
 0x10be042 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be04f; continue; }
  pc = 0x10be047; continue;
 }
 0x10be047 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be04f; continue; }
  pc = 0x10be04c; continue;
 }
 0x10be04c => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10be04f; continue;
 }
 0x10be04f => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10be05e; continue; }
  pc = 0x10be053; continue;
 }
 0x10be053 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10be05e; continue; }
  pc = 0x10be057; continue;
 }
 0x10be057 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10be05e; continue; }
  pc = 0x10be05a; continue;
 }
 0x10be05a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10be034; continue;
 }
 0x10be05e => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10be065; continue; }
  pc = 0x10be063; continue;
 }
 0x10be063 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be065; continue;
 }
 0x10be065 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10be075; continue; }
  pc = 0x10be069; continue;
 }
 0x10be069 => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10be180; continue;
 }
 0x10be075 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x116b344u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x10be085; continue;
 }
 0x10be085 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be093; continue; }
  pc = 0x10be08d; continue;
 }
 0x10be08d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be093; continue; }
  pc = 0x10be091; continue;
 }
 0x10be091 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be093; continue;
 }
 0x10be093 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be0a0; continue; }
  pc = 0x10be098; continue;
 }
 0x10be098 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be0a0; continue; }
  pc = 0x10be09d; continue;
 }
 0x10be09d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10be0a0; continue;
 }
 0x10be0a0 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10be0af; continue; }
  pc = 0x10be0a4; continue;
 }
 0x10be0a4 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10be0af; continue; }
  pc = 0x10be0a8; continue;
 }
 0x10be0a8 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10be0af; continue; }
  pc = 0x10be0ab; continue;
 }
 0x10be0ab => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10be085; continue;
 }
 0x10be0af => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10be0b6; continue; }
  pc = 0x10be0b4; continue;
 }
 0x10be0b4 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be0b6; continue;
 }
 0x10be0b6 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10be0c6; continue; }
  pc = 0x10be0ba; continue;
 }
 0x10be0ba => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10be180; continue;
 }
 0x10be0c6 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x116b34cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x10be0d3; continue;
 }
 0x10be0d3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be0e1; continue; }
  pc = 0x10be0db; continue;
 }
 0x10be0db => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be0e1; continue; }
  pc = 0x10be0df; continue;
 }
 0x10be0df => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be0e1; continue;
 }
 0x10be0e1 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be0ee; continue; }
  pc = 0x10be0e6; continue;
 }
 0x10be0e6 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be0ee; continue; }
  pc = 0x10be0eb; continue;
 }
 0x10be0eb => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10be0ee; continue;
 }
 0x10be0ee => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10be0fd; continue; }
  pc = 0x10be0f2; continue;
 }
 0x10be0f2 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10be0fd; continue; }
  pc = 0x10be0f6; continue;
 }
 0x10be0f6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10be0fd; continue; }
  pc = 0x10be0f9; continue;
 }
 0x10be0f9 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10be0d3; continue;
 }
 0x10be0fd => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10be104; continue; }
  pc = 0x10be102; continue;
 }
 0x10be102 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be104; continue;
 }
 0x10be104 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x10be14d; continue; }
  pc = 0x10be108; continue;
 }
 0x10be108 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x116b354u32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x10be118; continue;
 }
 0x10be118 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be126; continue; }
  pc = 0x10be120; continue;
 }
 0x10be120 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be126; continue; }
  pc = 0x10be124; continue;
 }
 0x10be124 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be126; continue;
 }
 0x10be126 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x10be133; continue; }
  pc = 0x10be12b; continue;
 }
 0x10be12b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be133; continue; }
  pc = 0x10be130; continue;
 }
 0x10be130 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x10be133; continue;
 }
 0x10be133 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x10be142; continue; }
  pc = 0x10be137; continue;
 }
 0x10be137 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x10be142; continue; }
  pc = 0x10be13b; continue;
 }
 0x10be13b => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x10be142; continue; }
  pc = 0x10be13e; continue;
 }
 0x10be13e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x10be118; continue;
 }
 0x10be142 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x10be149; continue; }
  pc = 0x10be147; continue;
 }
 0x10be147 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be149; continue;
 }
 0x10be149 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x10be179; continue; }
  pc = 0x10be14d; continue;
 }
 0x10be14d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10be156); f_010a6fe0(cpu); if cpu.returned_to != 0x10be156 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x116b35cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10be165; continue;
 }
 0x10be165 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10be165; continue; }
  pc = 0x10be16f; continue;
 }
 0x10be16f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10be179 => {
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10be180; continue;
 }
 0x10be180 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x10be1bb; continue; }
  pc = 0x10be189; continue;
 }
 0x10be189 => {
  { cpu.sub32(cpu.r32(0x12d240cu32), 0x0u32); }
  if !cpu.zf { pc = 0x10be1bb; continue; }
  pc = 0x10be192; continue;
 }
 0x10be192 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x10be198); f_010a6fe0(cpu); if cpu.returned_to != 0x10be198 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x116b370u32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10be1a7; continue;
 }
 0x10be1a7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10be1a7; continue; }
  pc = 0x10be1b1; continue;
 }
 0x10be1b1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10be1bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x10be1f9; continue; }
  pc = 0x10be1c4; continue;
 }
 0x10be1c4 => {
  { cpu.sub32(cpu.r32(0x12d2408u32), 0x0u32); }
  if !cpu.zf { pc = 0x10be1f9; continue; }
  pc = 0x10be1cd; continue;
 }
 0x10be1cd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x10be1d6); f_010a6fe0(cpu); if cpu.returned_to != 0x10be1d6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x116b38cu32; cpu.esi = v; }
  { let v: u32 = 0x12d2020u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x10be1e5; continue;
 }
 0x10be1e5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10be1e5; continue; }
  pc = 0x10be1ef; continue;
 }
 0x10be1ef => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x10be1f9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x117c954u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x10be210; continue;
 }
 0x10be210 => {
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
pub fn f_010be220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be220); }
 let mut pc: u32 = 0x10be220;
 loop { match pc {
 0x10be220 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x12d240cu32), 0x0u32); }
  if !cpu.zf { pc = 0x10be233; continue; }
  pc = 0x10be22a; continue;
 }
 0x10be22a => {
  { cpu.sub32(cpu.r32(0x12d2408u32), 0x0u32); }
  if cpu.zf { pc = 0x10be25b; continue; }
  pc = 0x10be233; continue;
 }
 0x10be233 => {
  { let v: u32 = cpu.r32(0x117c954u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x10be248; continue; }
  pc = 0x10be23d; continue;
 }
 0x10be23d => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x10be243); f_010a6fe0(cpu); if cpu.returned_to != 0x10be243 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  cpu.poll(); pc = 0x10be233; continue;
 }
 0x10be248 => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  cpu.push(0x10be24f); f_010ae7f0(cpu); if cpu.returned_to != 0x10be24f { return; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d240cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x12d2408u32, v); }
  pc = 0x10be25b; continue;
 }
 0x10be25b => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be330); }
 let mut pc: u32 = 0x10be330;
 loop { match pc {
 0x10be330 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10be33c); f_010aacc0(cpu); if cpu.returned_to != 0x10be33c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10be34e); f_010aacc0(cpu); if cpu.returned_to != 0x10be34e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10be35e); f_010acc30(cpu); if cpu.returned_to != 0x10be35e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be370); }
 let mut pc: u32 = 0x10be370;
 loop { match pc {
 0x10be370 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x10be37c); f_010a8730(cpu); if cpu.returned_to != 0x10be37c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10be38e); f_010a8730(cpu); if cpu.returned_to != 0x10be38e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x10be39e); f_010aa750(cpu); if cpu.returned_to != 0x10be39e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be3b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be3b0); }
 let mut pc: u32 = 0x10be3b0;
 loop { match pc {
 0x10be3b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x117c958u32), 0x0u32); }
  if !cpu.zf { pc = 0x10be406; continue; }
  pc = 0x10be3bc; continue;
 }
 0x10be3bc => {
  { let v = 0x10bdc50u32; cpu.push(v); }
  { let v = 0x10be3b0u32; cpu.push(v); }
  cpu.push(0x10be3cb); f_010b47f0(cpu); if cpu.returned_to != 0x10be3cb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d240cu32; cpu.push(v); }
  cpu.push(0x10be3da); f_010b4730(cpu); if cpu.returned_to != 0x10be3da { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x12d2408u32; cpu.push(v); }
  cpu.push(0x10be3e9); f_010b4730(cpu); if cpu.returned_to != 0x10be3e9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x117c954u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x10be3fd); f_010b4730(cpu); if cpu.returned_to != 0x10be3fd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x117c958u32, v); }
  pc = 0x10be406; continue;
 }
 0x10be406 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be40a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be40a); }
 let mut pc: u32 = 0x10be40a;
 loop { match pc {
 0x10be40a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117ba20u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x10be421); f_010be4cb(cpu); if cpu.returned_to != 0x10be421 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x117ba20u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x10be443); f_010b000a(cpu); if cpu.returned_to != 0x10be443 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10be4a4; continue; }
  pc = 0x10be449; continue;
 }
 0x10be449 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x117ba20u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117b1d8u32, v); }
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
  { let v: u32 = cpu.r32(0x117b1d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x117ba24u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x10be48b); f_010be4cb(cpu); if cpu.returned_to != 0x10be48b { return; }
  { let v: u32 = cpu.r32(0x117b1d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117b1d4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x10be4ae; continue;
 }
 0x10be4a4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x117b1d8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x117b1d4u32, v); }
  pc = 0x10be4ae; continue;
 }
 0x10be4ae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d24b8u32, v); }
  { let v: u32 = cpu.r32(0x117b1d8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x12d24bcu32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
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
pub fn f_010be4cb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be4cb); }
 let mut pc: u32 = 0x10be4cb;
 loop { match pc {
 0x10be4cb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12d24c0u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x10be4e2; continue;
 }
 0x10be4e2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.zf { pc = 0x10be4f0; continue; }
  pc = 0x10be4eb; continue;
 }
 0x10be4eb => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if !cpu.zf { pc = 0x10be4f3; continue; }
  pc = 0x10be4f0; continue;
 }
 0x10be4f0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10be4e2; continue;
 }
 0x10be4f3 => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x10be5c5; continue; }
  pc = 0x10be4fb; continue;
 }
 0x10be4fb => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if !cpu.zf { pc = 0x10be505; continue; }
  pc = 0x10be502; continue;
 }
 0x10be502 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10be505; continue;
 }
 0x10be505 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x10be50a; continue;
 }
 0x10be50a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x10be533; continue; }
  pc = 0x10be513; continue;
 }
 0x10be513 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x10be592; continue; }
  pc = 0x10be51c; continue;
 }
 0x10be51c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10be533; continue; }
  pc = 0x10be520; continue;
 }
 0x10be520 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x10be529; continue; }
  pc = 0x10be524; continue;
 }
 0x10be524 => {
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10be526; continue;
 }
 0x10be526 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x10be50a; continue;
 }
 0x10be529 => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x10be533; continue; }
  pc = 0x10be52e; continue;
 }
 0x10be52e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x10be50a; continue;
 }
 0x10be533 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.zf { pc = 0x10be541; continue; }
  pc = 0x10be53c; continue;
 }
 0x10be53c => {
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if !cpu.zf { pc = 0x10be545; continue; }
  pc = 0x10be541; continue;
 }
 0x10be541 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10be592; continue; }
  pc = 0x10be545; continue;
 }
 0x10be545 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x10be592; continue; }
  pc = 0x10be54b; continue;
 }
 0x10be54b => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  if !cpu.zf { pc = 0x10be583; continue; }
  pc = 0x10be554; continue;
 }
 0x10be554 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x10be564; continue; }
  pc = 0x10be558; continue;
 }
 0x10be558 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  pc = 0x10be560; continue;
 }
 0x10be560 => {
  if !cpu.zf { pc = 0x10be583; continue; }
  pc = 0x10be562; continue;
 }
 0x10be562 => {
  pc = 0x10be582; continue;
 }
 0x10be564 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x10be578; continue; }
  pc = 0x10be569; continue;
 }
 0x10be569 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if cpu.zf { pc = 0x10be582; continue; }
  pc = 0x10be573; continue;
 }
 0x10be573 => {
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  cpu.poll(); pc = 0x10be560; continue;
 }
 0x10be578 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x10be583; continue; }
  pc = 0x10be582; continue;
 }
 0x10be582 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x10be583; continue;
 }
 0x10be583 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x10be526; continue; }
  pc = 0x10be587; continue;
 }
 0x10be587 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x10be50a; continue;
 }
 0x10be592 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x10be5b8; continue; }
  pc = 0x10be596; continue;
 }
 0x10be596 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x10be5af; continue; }
  pc = 0x10be5ab; continue;
 }
 0x10be5ab => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10be5c5; continue;
 }
 0x10be5af => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x10be4e2; continue;
 }
 0x10be5b8 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x10be4f0; continue; }
  pc = 0x10be5c5; continue;
 }
 0x10be5c5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x12d24c0u32, v); }
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
pub fn f_010be5da(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be5da); }
 let mut pc: u32 = 0x10be5da;
 loop { match pc {
 0x10be5da => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x117ba34u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x10be5ef); f_010c35ed(cpu); if cpu.returned_to != 0x10be5ef { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x10be5fb; continue; }
  pc = 0x10be5f3; continue;
 }
 0x10be5f3 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x10be5fd; continue;
 }
 0x10be5fb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10be5fd; continue;
 }
 0x10be5fd => {
  { let v: u32 = cpu.r32(0x117ba34u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x117ba38u32, v); }
  cpu.push(0x10be60f); f_010c5e21(cpu); if cpu.returned_to != 0x10be60f { return; }
  { let v: u32 = cpu.r32(0x12d24bcu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x12d24b8u32); cpu.eax = v; }
  cpu.push(0x10be61f); f_0107eb58(cpu); if cpu.returned_to != 0x10be61f { return; }
  cpu.push(0x10be624); f_010c5e8b(cpu); if cpu.returned_to != 0x10be624 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be62a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be62a); }
 let mut pc: u32 = 0x10be62a;
 loop { match pc {
 0x10be62a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x10be631; continue;
 }
 0x10be631 => {
  { let v: u32 = 0x117cfceu32; cpu.esi = v; }
  { let v: u32 = 0x117d00au32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be641; continue;
 }
 0x10be641 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10be659; continue; }
  pc = 0x10be645; continue;
 }
 0x10be645 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x10be654; continue; }
  pc = 0x10be64a; continue;
 }
 0x10be64a => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be654; continue; }
  pc = 0x10be64f; continue;
 }
 0x10be64f => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be654; continue;
 }
 0x10be654 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10be641; continue;
 }
 0x10be659 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x10be66f; continue; }
  pc = 0x10be65d; continue;
 }
 0x10be65d => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10be66a; continue; }
  pc = 0x10be664; continue;
 }
 0x10be664 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x10be667); cpu.push(0x10be669); cpu.call(t); if cpu.returned_to != 0x10be669 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x10be66a; continue;
 }
 0x10be66a => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10be631; continue;
 }
 0x10be66f => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10be674); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be675(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be675); }
 let mut pc: u32 = 0x10be675;
 loop { match pc {
 0x10be675 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x10be67d; continue;
 }
 0x10be67d => {
  { let v: u32 = 0x117d00au32; cpu.esi = v; }
  { let v: u32 = 0x117d01cu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be68d; continue;
 }
 0x10be68d => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x10be6a5; continue; }
  pc = 0x10be691; continue;
 }
 0x10be691 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x10be6a0; continue; }
  pc = 0x10be696; continue;
 }
 0x10be696 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be6a0; continue; }
  pc = 0x10be69b; continue;
 }
 0x10be69b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x10be6a0; continue;
 }
 0x10be6a0 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x10be68d; continue;
 }
 0x10be6a5 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x10be6bf; continue; }
  pc = 0x10be6a9; continue;
 }
 0x10be6a9 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x10be6ba; continue; }
  pc = 0x10be6ad; continue;
 }
 0x10be6ad => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x10be6ba; continue; }
  pc = 0x10be6b4; continue;
 }
 0x10be6b4 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x10be6b7); cpu.push(0x10be6b9); cpu.call(t); if cpu.returned_to != 0x10be6b9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x10be6ba; continue;
 }
 0x10be6ba => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x10be67d; continue;
 }
 0x10be6bf => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10be6c3); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be6c4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10be6c4); } r_010be6c4(cpu, 0x10be6c4); }
pub fn f_010be6e8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10be6e8); } r_010be6c4(cpu, 0x10be6e8); }
fn r_010be6c4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10be6c4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x117cc7cu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x10be71c; continue; }
  pc = 0x10be6d6; continue;
 }
 0x10be6d6 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10be71c; continue; }
  pc = 0x10be6da; continue;
 }
 0x10be6da => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  pc = 0x10be6e8; continue;
 }
 0x10be6e8 => {
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x10be716; continue;
 }
 0x10be6f0 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x10be6fb); f_010a5a93(cpu); if cpu.returned_to != 0x10be6fb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10be713; continue; }
  pc = 0x10be6ff; continue;
 }
 0x10be6ff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if !cpu.zf { pc = 0x10be713; continue; }
  pc = 0x10be70c; continue;
 }
 0x10be70c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x10be71e; continue;
 }
 0x10be713 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x10be716; continue;
 }
 0x10be716 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x10be6f0; continue; }
  pc = 0x10be71c; continue;
 }
 0x10be71c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x10be71e; continue;
 }
 0x10be71e => {
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
pub fn f_010be725(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be725); }
 let mut pc: u32 = 0x10be725;
 loop { match pc {
 0x10be725 => {
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
pub fn f_010be758(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be758); }
 let mut pc: u32 = 0x10be758;
 loop { match pc {
 0x10be758 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b810u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x116b817u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x116b81eu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x3u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116b7ecu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x116b7f8u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = cpu.r8(0x116b804u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x20u8; cpu.w8(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x8u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x10be7b5); f_010be725(cpu); if cpu.returned_to != 0x10be7b5 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x8u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x10be7c3; continue; }
  pc = 0x10be7bf; continue;
 }
 0x10be7bf => {
  { let v: u8 = 0x20u8; cpu.w8(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x10be7c3; continue;
 }
 0x10be7c3 => {
  { let v: u32 = 0xbu32; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0xau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x10be7d6); f_010be725(cpu); if cpu.returned_to != 0x10be7d6 { return; }
  { let v: u32 = 0xeu32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0xdu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x10be7e9); f_010be725(cpu); if cpu.returned_to != 0x10be7e9 { return; }
  { let v: u32 = 0x11u32; cpu.edx = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  cpu.push(0x10be7fb); f_010be725(cpu); if cpu.returned_to != 0x10be7fb { return; }
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
  cpu.push(0x10be823); f_010be725(cpu); if cpu.returned_to != 0x10be823 { return; }
  { let v: u32 = 0x16u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.push(0x10be833); f_010be725(cpu); if cpu.returned_to != 0x10be833 { return; }
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
pub fn f_010be863(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be863); }
 let mut pc: u32 = 0x10be863;
 loop { match pc {
 0x10be863 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x12d2410u32; cpu.edx = v; }
  cpu.push(0x10be86e); f_010be758(cpu); if cpu.returned_to != 0x10be86e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be875(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be875); }
 let mut pc: u32 = 0x10be875;
 loop { match pc {
 0x10be875 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x10be882); f_010c5eba(cpu); if cpu.returned_to != 0x10be882 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10be88f; continue; }
  pc = 0x10be886; continue;
 }
 0x10be886 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x10be88f); f_010af0f5(cpu); if cpu.returned_to != 0x10be88f { return; }
  pc = 0x10be88f; continue;
 }
 0x10be88f => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be892(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be892); }
 let mut pc: u32 = 0x10be892;
 loop { match pc {
 0x10be892 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x10be89e; continue; }
  pc = 0x10be899; continue;
 }
 0x10be899 => {
  { let v: u8 = 0x2fu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x10be8ac; continue;
 }
 0x10be89e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x37u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x10be8ac; continue;
 }
 0x10be8ac => {
  { let v: u8 = 0x63u8; cpu.w8(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be8b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be8b8); }
 let mut pc: u32 = 0x10be8b8;
 loop { match pc {
 0x10be8b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(44); }
  { let v: u32 = 0xffffffd4u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  cpu.push(0x10be8cb); f_010a3272(cpu); if cpu.returned_to != 0x10be8cb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be8d7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10be8d7); } r_010be8d7(cpu, 0x10be8d7); }
pub fn f_010be900(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x10be900); } r_010be8d7(cpu, 0x10be900); }
fn r_010be8d7(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x10be8d7 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(4); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.push(0x10be8f1); f_010c6027(cpu); if cpu.returned_to != 0x10be8f1 { return; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x10be901); f_010c607a(cpu); if cpu.returned_to != 0x10be901 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x10be900 => {
  { let v: u8 = cpu.add8(cpu.r8(0x5fc914c4u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x5fc914c4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010be90a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10be90a); }
 let mut pc: u32 = 0x10be90a;
 loop { match pc {
 0x10be90a => {
  { let v = 0xe4u32; cpu.push(v); }
  cpu.flat_ds(0x10be90f); cpu.push(0x10be914); f_010af47a(cpu); if cpu.returned_to != 0x10be914 { return; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(176); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x82u32); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x6eu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x7eu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.cf { pc = 0x10be946; continue; }
  pc = 0x10be932; continue;
 }
 0x10be932 => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.flat_ds(0x10be937); cpu.push(0x10be93c); f_010b0331(cpu); if cpu.returned_to != 0x10be93c { return; }
  pc = 0x10be93c; continue;
 }
 0x10be93c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x10bec12; continue;
 }
 0x10be946 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x52u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x56u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x5au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x5eu32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10be959); cpu.push(0x10be95e); f_010c5eef(cpu); if cpu.returned_to != 0x10be95e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x10bec12; continue; }
  pc = 0x10be96a; continue;
 }
 0x10be96a => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x10be96c); cpu.push(0x10be971); f_010a4c24(cpu); if cpu.returned_to != 0x10be971 { return; }
  { let v: u32 = 0x9au32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.flat_ds(0x10be979); cpu.push(0x10be97e); f_010b000a(cpu); if cpu.returned_to != 0x10be97e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x66u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10be9af; continue; }
  pc = 0x10be985; continue;
 }
 0x10be985 => {
  { let v: u32 = 0x3u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xfcu8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10be98b); cpu.push(0x10be990); f_010c35ed(cpu); if cpu.returned_to != 0x10be990 { return; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.cf { pc = 0x10be99c; continue; }
  pc = 0x10be994; continue;
 }
 0x10be994 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x10be99e; continue;
 }
 0x10be99c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10be99e; continue;
 }
 0x10be99e => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10be9b2; continue; }
  pc = 0x10be9a5; continue;
 }
 0x10be9a5 => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10be9a8); cpu.push(0x10be9ad); f_010b00ed(cpu); if cpu.returned_to != 0x10be9ad { return; }
  cpu.poll(); pc = 0x10be93c; continue;
 }
 0x10be9af => {
  { let v: u32 = cpu.eax; cpu.w32(0x7au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10be9b2; continue;
 }
 0x10be9b2 => {
  { let v: u32 = 0x42u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = 0x4au32.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0x46u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x3eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x93u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x7au32.wrapping_add(cpu.ebp))); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.flat_ds(0x10be9cb); cpu.push(0x10be9d0); f_010c619a(cpu); if cpu.returned_to != 0x10be9d0 { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x117c9ecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10be9d3); cpu.push(0x10be9d9); cpu.call(t); if cpu.returned_to != 0x10be9d9 { return; }
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10be9dc); cpu.push(0x10be9e1); f_010b000a(cpu); if cpu.returned_to != 0x10be9e1 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bea31; continue; }
  pc = 0x10be9e8; continue;
 }
 0x10be9e8 => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.flat_ds(0x10be9f1); cpu.push(0x10be9f6); f_010c35ed(cpu); if cpu.returned_to != 0x10be9f6 { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x10bea08; continue; }
  pc = 0x10be9fa; continue;
 }
 0x10be9fa => {
  { let v: u32 = cpu.r32(0x52u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x3u32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfcu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x10bea0a; continue;
 }
 0x10bea08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x10bea0a; continue;
 }
 0x10bea0a => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10bea34; continue; }
  pc = 0x10bea11; continue;
 }
 0x10bea11 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.flat_ds(0x10bea16); cpu.push(0x10bea1b); f_010b0331(cpu); if cpu.returned_to != 0x10bea1b { return; }
  { let v: u32 = 0xau32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x10bea27); cpu.push(0x10bea2c); f_010b0354(cpu); if cpu.returned_to != 0x10bea2c { return; }
  pc = 0x10bebee; continue;
 }
 0x10bea31 => {
  { let v: u32 = cpu.eax; cpu.w32(0x72u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bea34; continue;
 }
 0x10bea34 => {
  { let v: u32 = cpu.r8(0x117ba53u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (cpu.sf != cpu.of) { pc = 0x10bea57; continue; }
  pc = 0x10bea40; continue;
 }
 0x10bea40 => {
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x10bea57; continue; }
  pc = 0x10bea48; continue;
 }
 0x10bea48 => {
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x10bea57; continue; }
  pc = 0x10bea50; continue;
 }
 0x10bea50 => {
  { let v: u32 = 0x116b3acu32; cpu.w32(0x4au32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bea57; continue;
 }
 0x10bea57 => {
  { let v = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x3eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4au32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x46u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10bea66); cpu.push(0x10bea6b); f_010b2ed6(cpu); if cpu.returned_to != 0x10bea6b { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x10bea70); cpu.push(0x10bea75); f_010b0331(cpu); if cpu.returned_to != 0x10bea75 { return; }
  { let v: u32 = cpu.r32(0x42u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x10beac7; continue; }
  pc = 0x10bea7d; continue;
 }
 0x10bea7d => {
  { let v: u32 = 0x116b3afu32; cpu.edx = v; }
  cpu.flat_ds(0x10bea82); cpu.push(0x10bea87); f_010a2e87(cpu); if cpu.returned_to != 0x10bea87 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10beaa7; continue; }
  pc = 0x10bea8b; continue;
 }
 0x10bea8b => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x10bea95); cpu.push(0x10bea9a); f_010be8b8(cpu); if cpu.returned_to != 0x10bea9a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x10beb8b; continue; }
  pc = 0x10beaa2; continue;
 }
 0x10beaa2 => {
  pc = 0x10bebee; continue;
 }
 0x10beaa7 => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x10beaaf); cpu.push(0x10beab4); f_010b0331(cpu); if cpu.returned_to != 0x10beab4 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10beabd); cpu.push(0x10beac2); f_010be8d7(cpu); if cpu.returned_to != 0x10beac2 { return; }
  pc = 0x10bebeb; continue;
 }
 0x10beac7 => {
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x10beacd); cpu.push(0x10bead2); f_010a4c24(cpu); if cpu.returned_to != 0x10bead2 { return; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6au32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10beae9; continue; }
  pc = 0x10beadd; continue;
 }
 0x10beadd => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.flat_ds(0x10beae2); cpu.push(0x10beae7); f_010b0331(cpu); if cpu.returned_to != 0x10beae7 { return; }
  pc = 0x10beb12; continue;
 }
 0x10beae9 => {
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0x116b3b4u32; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x10beafc); cpu.push(0x10beb01); f_010b0331(cpu); if cpu.returned_to != 0x10beb01 { return; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10beb0a); cpu.push(0x10beb0f); f_010be8d7(cpu); if cpu.returned_to != 0x10beb0f { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10beb12; continue;
 }
 0x10beb12 => {
  cpu.flat_ds(0x10beb12); cpu.push(0x10beb17); f_010b2a07(cpu); if cpu.returned_to != 0x10beb17 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x10beb2a; continue; }
  pc = 0x10beb1c; continue;
 }
 0x10beb1c => {
  cpu.flat_ds(0x10beb1c); cpu.push(0x10beb21); f_010b2a07(cpu); if cpu.returned_to != 0x10beb21 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x10bebee; continue; }
  pc = 0x10beb2a; continue;
 }
 0x10beb2a => {
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x10beb35); cpu.push(0x10beb3a); f_010b0331(cpu); if cpu.returned_to != 0x10beb3a { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x116b3b9u32; cpu.esi = v; }
  { let v = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  { let v: u32 = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  cpu.flat_ds(0x10beb4c); cpu.push(0x10beb51); f_010be8d7(cpu); if cpu.returned_to != 0x10beb51 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x10beb54); cpu.push(0x10beb59); f_010b2a07(cpu); if cpu.returned_to != 0x10beb59 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x1u32); }
  if cpu.zf { pc = 0x10beb6c; continue; }
  pc = 0x10beb5e; continue;
 }
 0x10beb5e => {
  cpu.flat_ds(0x10beb5e); cpu.push(0x10beb63); f_010b2a07(cpu); if cpu.returned_to != 0x10beb63 { return; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])), 0x9u32); }
  if !cpu.zf { pc = 0x10bebee; continue; }
  pc = 0x10beb6c; continue;
 }
 0x10beb6c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6au32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  cpu.flat_ds(0x10beb71); cpu.push(0x10beb76); f_010b0331(cpu); if cpu.returned_to != 0x10beb76 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x116b3afu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.movs(4, false); }
  { cpu.movs(1, false); }
  cpu.flat_ds(0x10beb82); cpu.push(0x10beb87); f_010be8b8(cpu); if cpu.returned_to != 0x10beb87 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x10bebee; continue; }
  pc = 0x10beb8b; continue;
 }
 0x10beb8b => {
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x6eu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  cpu.flat_ds(0x10beb99); cpu.push(0x10beb9e); f_010b00ed(cpu); if cpu.returned_to != 0x10beb9e { return; }
  { let v: u32 = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x5eu32.wrapping_add(cpu.ebp), v); }
  cpu.flat_ds(0x10beba8); cpu.push(0x10bebad); f_010c6027(cpu); if cpu.returned_to != 0x10bebad { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.r32(0x72u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = cpu.r8(0x7eu32.wrapping_add(cpu.ebp)) as u32; cpu.edx = v; }
  { let v = cpu.r32(0x7au32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v: u32 = 0x4eu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.flat_ds(0x10bebbc); cpu.push(0x10bebc1); f_010be892(cpu); if cpu.returned_to != 0x10bebc1 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x7eu32.wrapping_add(cpu.ebp)), 0x0u8); }
  if cpu.zf { pc = 0x10bebcf; continue; }
  pc = 0x10bebc8; continue;
 }
 0x10bebc8 => {
  { let v: u32 = 0x116b3c6u32; cpu.eax = v; }
  pc = 0x10bebd4; continue;
 }
 0x10bebcf => {
  { let v: u32 = 0x116b3cau32; cpu.eax = v; }
  pc = 0x10bebd4; continue;
 }
 0x10bebd4 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x116b3beu32; cpu.eax = v; }
  cpu.flat_ds(0x10bebda); cpu.push(0x10bebdf); f_010be6c4(cpu); if cpu.returned_to != 0x10bebdf { return; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0xffffffceu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.flat_ds(0x10bebe3); cpu.push(0x10bebe8); f_010c6275(cpu); if cpu.returned_to != 0x10bebe8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x10bebeb; continue;
 }
 0x10bebeb => {
  { let v: u32 = cpu.eax; cpu.w32(0x76u32.wrapping_add(cpu.ebp), v); }
  pc = 0x10bebee; continue;
 }
 0x10bebee => {
  { let v: u32 = cpu.r32(0x62u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10bebf1); cpu.push(0x10bebf6); f_010b00ed(cpu); if cpu.returned_to != 0x10bebf6 { return; }
  { let v: u32 = cpu.r32(0x66u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10bebf9); cpu.push(0x10bebfe); f_010b00ed(cpu); if cpu.returned_to != 0x10bebfe { return; }
  { let v: u32 = cpu.r32(0x5eu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.flat_ds(0x10bec01); cpu.push(0x10bec06); f_010b00ed(cpu); if cpu.returned_to != 0x10bec06 { return; }
  { let v: u32 = 0xffffffd2u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  let t = cpu.r32(0x117c9f0u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x10bec09); cpu.push(0x10bec0f); cpu.call(t); if cpu.returned_to != 0x10bec0f { return; }
  { let v: u32 = cpu.r32(0x76u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x10bec12; continue;
 }
 0x10bec12 => {
  { let v: u32 = 0x82u32.wrapping_add(cpu.ebp); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x10bec1c); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bec1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bec1d); }
 let mut pc: u32 = 0x10bec1d;
 loop { match pc {
 0x10bec1d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x10bec30); f_010c6293(cpu); if cpu.returned_to != 0x10bec30 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bec34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bec34); }
 let mut pc: u32 = 0x10bec34;
 loop { match pc {
 0x10bec34 => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.flat_ds(0x10bec39); cpu.push(0x10bec3e); f_010bec6c(cpu); if cpu.returned_to != 0x10bec3e { return; }
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
  cpu.flat_ds(0x10bec6b); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bec6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bec6c); }
 let mut pc: u32 = 0x10bec6c;
 loop { match pc {
 0x10bec6c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.esi.wrapping_mul(2)); cpu.esi = v; }
  { let v: u32 = 0x10bed12u32.wrapping_add(cpu.esi); cpu.eax = v; }
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
pub fn f_010becde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10becde); }
 let mut pc: u32 = 0x10becde;
 loop { match pc {
 0x10becde => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x10bed12u32.wrapping_add(cpu.eax); cpu.eax = v; }
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
pub fn f_010bed12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed12); }
 let mut pc: u32 = 0x10bed12;
 loop { match pc {
 0x10bed12 => {
  { cpu.interrupt(0x0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed15); }
 let mut pc: u32 = 0x10bed15;
 loop { match pc {
 0x10bed15 => {
  { cpu.interrupt(0x1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed1e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed1e); }
 let mut pc: u32 = 0x10bed1e;
 loop { match pc {
 0x10bed1e => {
  { cpu.interrupt(0x4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed21(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed21); }
 let mut pc: u32 = 0x10bed21;
 loop { match pc {
 0x10bed21 => {
  { cpu.interrupt(0x5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed24); }
 let mut pc: u32 = 0x10bed24;
 loop { match pc {
 0x10bed24 => {
  { cpu.interrupt(0x6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed27); }
 let mut pc: u32 = 0x10bed27;
 loop { match pc {
 0x10bed27 => {
  { cpu.interrupt(0x7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed2a); }
 let mut pc: u32 = 0x10bed2a;
 loop { match pc {
 0x10bed2a => {
  { cpu.interrupt(0x8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed2d); }
 let mut pc: u32 = 0x10bed2d;
 loop { match pc {
 0x10bed2d => {
  { cpu.interrupt(0x9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed30); }
 let mut pc: u32 = 0x10bed30;
 loop { match pc {
 0x10bed30 => {
  { cpu.interrupt(0xa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed33(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed33); }
 let mut pc: u32 = 0x10bed33;
 loop { match pc {
 0x10bed33 => {
  { cpu.interrupt(0xb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed36(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed36); }
 let mut pc: u32 = 0x10bed36;
 loop { match pc {
 0x10bed36 => {
  { cpu.interrupt(0xc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed39); }
 let mut pc: u32 = 0x10bed39;
 loop { match pc {
 0x10bed39 => {
  { cpu.interrupt(0xd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed3c); }
 let mut pc: u32 = 0x10bed3c;
 loop { match pc {
 0x10bed3c => {
  { cpu.interrupt(0xe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed3f); }
 let mut pc: u32 = 0x10bed3f;
 loop { match pc {
 0x10bed3f => {
  { cpu.interrupt(0xf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed42(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed42); }
 let mut pc: u32 = 0x10bed42;
 loop { match pc {
 0x10bed42 => {
  { cpu.interrupt(0x10); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed45); }
 let mut pc: u32 = 0x10bed45;
 loop { match pc {
 0x10bed45 => {
  { cpu.interrupt(0x11); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed48); }
 let mut pc: u32 = 0x10bed48;
 loop { match pc {
 0x10bed48 => {
  { cpu.interrupt(0x12); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed4b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed4b); }
 let mut pc: u32 = 0x10bed4b;
 loop { match pc {
 0x10bed4b => {
  { cpu.interrupt(0x13); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed4e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed4e); }
 let mut pc: u32 = 0x10bed4e;
 loop { match pc {
 0x10bed4e => {
  { cpu.interrupt(0x14); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed51); }
 let mut pc: u32 = 0x10bed51;
 loop { match pc {
 0x10bed51 => {
  { cpu.interrupt(0x15); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed54); }
 let mut pc: u32 = 0x10bed54;
 loop { match pc {
 0x10bed54 => {
  { cpu.interrupt(0x16); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed57(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed57); }
 let mut pc: u32 = 0x10bed57;
 loop { match pc {
 0x10bed57 => {
  { cpu.interrupt(0x17); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed5a); }
 let mut pc: u32 = 0x10bed5a;
 loop { match pc {
 0x10bed5a => {
  { cpu.interrupt(0x18); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed5d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed5d); }
 let mut pc: u32 = 0x10bed5d;
 loop { match pc {
 0x10bed5d => {
  { cpu.interrupt(0x19); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed60); }
 let mut pc: u32 = 0x10bed60;
 loop { match pc {
 0x10bed60 => {
  { cpu.interrupt(0x1a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed63(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed63); }
 let mut pc: u32 = 0x10bed63;
 loop { match pc {
 0x10bed63 => {
  { cpu.interrupt(0x1b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed66(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed66); }
 let mut pc: u32 = 0x10bed66;
 loop { match pc {
 0x10bed66 => {
  { cpu.interrupt(0x1c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed69); }
 let mut pc: u32 = 0x10bed69;
 loop { match pc {
 0x10bed69 => {
  { cpu.interrupt(0x1d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed6c); }
 let mut pc: u32 = 0x10bed6c;
 loop { match pc {
 0x10bed6c => {
  { cpu.interrupt(0x1e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed6f); }
 let mut pc: u32 = 0x10bed6f;
 loop { match pc {
 0x10bed6f => {
  { cpu.interrupt(0x1f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed72(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed72); }
 let mut pc: u32 = 0x10bed72;
 loop { match pc {
 0x10bed72 => {
  { cpu.interrupt(0x20); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed75(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed75); }
 let mut pc: u32 = 0x10bed75;
 loop { match pc {
 0x10bed75 => {
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed78); }
 let mut pc: u32 = 0x10bed78;
 loop { match pc {
 0x10bed78 => {
  { cpu.interrupt(0x22); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed7b); }
 let mut pc: u32 = 0x10bed7b;
 loop { match pc {
 0x10bed7b => {
  { cpu.interrupt(0x23); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed7e); }
 let mut pc: u32 = 0x10bed7e;
 loop { match pc {
 0x10bed7e => {
  { cpu.interrupt(0x24); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed81(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed81); }
 let mut pc: u32 = 0x10bed81;
 loop { match pc {
 0x10bed81 => {
  { cpu.interrupt(0x25); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed84); }
 let mut pc: u32 = 0x10bed84;
 loop { match pc {
 0x10bed84 => {
  { cpu.interrupt(0x26); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed87); }
 let mut pc: u32 = 0x10bed87;
 loop { match pc {
 0x10bed87 => {
  { cpu.interrupt(0x27); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed8a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed8a); }
 let mut pc: u32 = 0x10bed8a;
 loop { match pc {
 0x10bed8a => {
  { cpu.interrupt(0x28); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed8d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed8d); }
 let mut pc: u32 = 0x10bed8d;
 loop { match pc {
 0x10bed8d => {
  { cpu.interrupt(0x29); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed90); }
 let mut pc: u32 = 0x10bed90;
 loop { match pc {
 0x10bed90 => {
  { cpu.interrupt(0x2a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed93); }
 let mut pc: u32 = 0x10bed93;
 loop { match pc {
 0x10bed93 => {
  { cpu.interrupt(0x2b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed96); }
 let mut pc: u32 = 0x10bed96;
 loop { match pc {
 0x10bed96 => {
  { cpu.interrupt(0x2c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed99); }
 let mut pc: u32 = 0x10bed99;
 loop { match pc {
 0x10bed99 => {
  { cpu.interrupt(0x2d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed9c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed9c); }
 let mut pc: u32 = 0x10bed9c;
 loop { match pc {
 0x10bed9c => {
  { cpu.interrupt(0x2e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bed9f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bed9f); }
 let mut pc: u32 = 0x10bed9f;
 loop { match pc {
 0x10bed9f => {
  { cpu.interrupt(0x2f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beda2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beda2); }
 let mut pc: u32 = 0x10beda2;
 loop { match pc {
 0x10beda2 => {
  { cpu.interrupt(0x30); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beda5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beda5); }
 let mut pc: u32 = 0x10beda5;
 loop { match pc {
 0x10beda5 => {
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beda8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beda8); }
 let mut pc: u32 = 0x10beda8;
 loop { match pc {
 0x10beda8 => {
  { cpu.interrupt(0x32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedab); }
 let mut pc: u32 = 0x10bedab;
 loop { match pc {
 0x10bedab => {
  { cpu.interrupt(0x33); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedae); }
 let mut pc: u32 = 0x10bedae;
 loop { match pc {
 0x10bedae => {
  { cpu.interrupt(0x34); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedb1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedb1); }
 let mut pc: u32 = 0x10bedb1;
 loop { match pc {
 0x10bedb1 => {
  { cpu.interrupt(0x35); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedb4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedb4); }
 let mut pc: u32 = 0x10bedb4;
 loop { match pc {
 0x10bedb4 => {
  { cpu.interrupt(0x36); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedb7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedb7); }
 let mut pc: u32 = 0x10bedb7;
 loop { match pc {
 0x10bedb7 => {
  { cpu.interrupt(0x37); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedba); }
 let mut pc: u32 = 0x10bedba;
 loop { match pc {
 0x10bedba => {
  { cpu.interrupt(0x38); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedbd); }
 let mut pc: u32 = 0x10bedbd;
 loop { match pc {
 0x10bedbd => {
  { cpu.interrupt(0x39); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedc0); }
 let mut pc: u32 = 0x10bedc0;
 loop { match pc {
 0x10bedc0 => {
  { cpu.interrupt(0x3a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedc3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedc3); }
 let mut pc: u32 = 0x10bedc3;
 loop { match pc {
 0x10bedc3 => {
  { cpu.interrupt(0x3b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedc6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedc6); }
 let mut pc: u32 = 0x10bedc6;
 loop { match pc {
 0x10bedc6 => {
  { cpu.interrupt(0x3c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedc9); }
 let mut pc: u32 = 0x10bedc9;
 loop { match pc {
 0x10bedc9 => {
  { cpu.interrupt(0x3d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedcc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedcc); }
 let mut pc: u32 = 0x10bedcc;
 loop { match pc {
 0x10bedcc => {
  { cpu.interrupt(0x3e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedcf); }
 let mut pc: u32 = 0x10bedcf;
 loop { match pc {
 0x10bedcf => {
  { cpu.interrupt(0x3f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedd2); }
 let mut pc: u32 = 0x10bedd2;
 loop { match pc {
 0x10bedd2 => {
  { cpu.interrupt(0x40); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedd5); }
 let mut pc: u32 = 0x10bedd5;
 loop { match pc {
 0x10bedd5 => {
  { cpu.interrupt(0x41); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedd8); }
 let mut pc: u32 = 0x10bedd8;
 loop { match pc {
 0x10bedd8 => {
  { cpu.interrupt(0x42); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beddb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beddb); }
 let mut pc: u32 = 0x10beddb;
 loop { match pc {
 0x10beddb => {
  { cpu.interrupt(0x43); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedde(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedde); }
 let mut pc: u32 = 0x10bedde;
 loop { match pc {
 0x10bedde => {
  { cpu.interrupt(0x44); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bede1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bede1); }
 let mut pc: u32 = 0x10bede1;
 loop { match pc {
 0x10bede1 => {
  { cpu.interrupt(0x45); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bede4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bede4); }
 let mut pc: u32 = 0x10bede4;
 loop { match pc {
 0x10bede4 => {
  { cpu.interrupt(0x46); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bede7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bede7); }
 let mut pc: u32 = 0x10bede7;
 loop { match pc {
 0x10bede7 => {
  { cpu.interrupt(0x47); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedea); }
 let mut pc: u32 = 0x10bedea;
 loop { match pc {
 0x10bedea => {
  { cpu.interrupt(0x48); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beded(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beded); }
 let mut pc: u32 = 0x10beded;
 loop { match pc {
 0x10beded => {
  { cpu.interrupt(0x49); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedf0); }
 let mut pc: u32 = 0x10bedf0;
 loop { match pc {
 0x10bedf0 => {
  { cpu.interrupt(0x4a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedf3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedf3); }
 let mut pc: u32 = 0x10bedf3;
 loop { match pc {
 0x10bedf3 => {
  { cpu.interrupt(0x4b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedf6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedf6); }
 let mut pc: u32 = 0x10bedf6;
 loop { match pc {
 0x10bedf6 => {
  { cpu.interrupt(0x4c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedf9); }
 let mut pc: u32 = 0x10bedf9;
 loop { match pc {
 0x10bedf9 => {
  { cpu.interrupt(0x4d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedfc); }
 let mut pc: u32 = 0x10bedfc;
 loop { match pc {
 0x10bedfc => {
  { cpu.interrupt(0x4e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bedff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bedff); }
 let mut pc: u32 = 0x10bedff;
 loop { match pc {
 0x10bedff => {
  { cpu.interrupt(0x4f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee02(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee02); }
 let mut pc: u32 = 0x10bee02;
 loop { match pc {
 0x10bee02 => {
  { cpu.interrupt(0x50); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee05); }
 let mut pc: u32 = 0x10bee05;
 loop { match pc {
 0x10bee05 => {
  { cpu.interrupt(0x51); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee08); }
 let mut pc: u32 = 0x10bee08;
 loop { match pc {
 0x10bee08 => {
  { cpu.interrupt(0x52); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee0b); }
 let mut pc: u32 = 0x10bee0b;
 loop { match pc {
 0x10bee0b => {
  { cpu.interrupt(0x53); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee0e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee0e); }
 let mut pc: u32 = 0x10bee0e;
 loop { match pc {
 0x10bee0e => {
  { cpu.interrupt(0x54); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee11); }
 let mut pc: u32 = 0x10bee11;
 loop { match pc {
 0x10bee11 => {
  { cpu.interrupt(0x55); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee14); }
 let mut pc: u32 = 0x10bee14;
 loop { match pc {
 0x10bee14 => {
  { cpu.interrupt(0x56); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee17(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee17); }
 let mut pc: u32 = 0x10bee17;
 loop { match pc {
 0x10bee17 => {
  { cpu.interrupt(0x57); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee1a); }
 let mut pc: u32 = 0x10bee1a;
 loop { match pc {
 0x10bee1a => {
  { cpu.interrupt(0x58); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee1d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee1d); }
 let mut pc: u32 = 0x10bee1d;
 loop { match pc {
 0x10bee1d => {
  { cpu.interrupt(0x59); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee20); }
 let mut pc: u32 = 0x10bee20;
 loop { match pc {
 0x10bee20 => {
  { cpu.interrupt(0x5a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee23(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee23); }
 let mut pc: u32 = 0x10bee23;
 loop { match pc {
 0x10bee23 => {
  { cpu.interrupt(0x5b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee26); }
 let mut pc: u32 = 0x10bee26;
 loop { match pc {
 0x10bee26 => {
  { cpu.interrupt(0x5c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee29); }
 let mut pc: u32 = 0x10bee29;
 loop { match pc {
 0x10bee29 => {
  { cpu.interrupt(0x5d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee2c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee2c); }
 let mut pc: u32 = 0x10bee2c;
 loop { match pc {
 0x10bee2c => {
  { cpu.interrupt(0x5e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee2f); }
 let mut pc: u32 = 0x10bee2f;
 loop { match pc {
 0x10bee2f => {
  { cpu.interrupt(0x5f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee32(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee32); }
 let mut pc: u32 = 0x10bee32;
 loop { match pc {
 0x10bee32 => {
  { cpu.interrupt(0x60); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee35); }
 let mut pc: u32 = 0x10bee35;
 loop { match pc {
 0x10bee35 => {
  { cpu.interrupt(0x61); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee38); }
 let mut pc: u32 = 0x10bee38;
 loop { match pc {
 0x10bee38 => {
  { cpu.interrupt(0x62); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee3b); }
 let mut pc: u32 = 0x10bee3b;
 loop { match pc {
 0x10bee3b => {
  { cpu.interrupt(0x63); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee3e); }
 let mut pc: u32 = 0x10bee3e;
 loop { match pc {
 0x10bee3e => {
  { cpu.interrupt(0x64); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee41(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee41); }
 let mut pc: u32 = 0x10bee41;
 loop { match pc {
 0x10bee41 => {
  { cpu.interrupt(0x65); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee44(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee44); }
 let mut pc: u32 = 0x10bee44;
 loop { match pc {
 0x10bee44 => {
  { cpu.interrupt(0x66); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee47); }
 let mut pc: u32 = 0x10bee47;
 loop { match pc {
 0x10bee47 => {
  { cpu.interrupt(0x67); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee4a); }
 let mut pc: u32 = 0x10bee4a;
 loop { match pc {
 0x10bee4a => {
  { cpu.interrupt(0x68); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee4d); }
 let mut pc: u32 = 0x10bee4d;
 loop { match pc {
 0x10bee4d => {
  { cpu.interrupt(0x69); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee50); }
 let mut pc: u32 = 0x10bee50;
 loop { match pc {
 0x10bee50 => {
  { cpu.interrupt(0x6a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee53(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee53); }
 let mut pc: u32 = 0x10bee53;
 loop { match pc {
 0x10bee53 => {
  { cpu.interrupt(0x6b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee56(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee56); }
 let mut pc: u32 = 0x10bee56;
 loop { match pc {
 0x10bee56 => {
  { cpu.interrupt(0x6c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee59(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee59); }
 let mut pc: u32 = 0x10bee59;
 loop { match pc {
 0x10bee59 => {
  { cpu.interrupt(0x6d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee5c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee5c); }
 let mut pc: u32 = 0x10bee5c;
 loop { match pc {
 0x10bee5c => {
  { cpu.interrupt(0x6e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee5f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee5f); }
 let mut pc: u32 = 0x10bee5f;
 loop { match pc {
 0x10bee5f => {
  { cpu.interrupt(0x6f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee62); }
 let mut pc: u32 = 0x10bee62;
 loop { match pc {
 0x10bee62 => {
  { cpu.interrupt(0x70); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee65); }
 let mut pc: u32 = 0x10bee65;
 loop { match pc {
 0x10bee65 => {
  { cpu.interrupt(0x71); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee68); }
 let mut pc: u32 = 0x10bee68;
 loop { match pc {
 0x10bee68 => {
  { cpu.interrupt(0x72); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee6b); }
 let mut pc: u32 = 0x10bee6b;
 loop { match pc {
 0x10bee6b => {
  { cpu.interrupt(0x73); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee6e); }
 let mut pc: u32 = 0x10bee6e;
 loop { match pc {
 0x10bee6e => {
  { cpu.interrupt(0x74); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee71(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee71); }
 let mut pc: u32 = 0x10bee71;
 loop { match pc {
 0x10bee71 => {
  { cpu.interrupt(0x75); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee74); }
 let mut pc: u32 = 0x10bee74;
 loop { match pc {
 0x10bee74 => {
  { cpu.interrupt(0x76); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee77); }
 let mut pc: u32 = 0x10bee77;
 loop { match pc {
 0x10bee77 => {
  { cpu.interrupt(0x77); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee7a); }
 let mut pc: u32 = 0x10bee7a;
 loop { match pc {
 0x10bee7a => {
  { cpu.interrupt(0x78); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee7d); }
 let mut pc: u32 = 0x10bee7d;
 loop { match pc {
 0x10bee7d => {
  { cpu.interrupt(0x79); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee80); }
 let mut pc: u32 = 0x10bee80;
 loop { match pc {
 0x10bee80 => {
  { cpu.interrupt(0x7a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee83); }
 let mut pc: u32 = 0x10bee83;
 loop { match pc {
 0x10bee83 => {
  { cpu.interrupt(0x7b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee86); }
 let mut pc: u32 = 0x10bee86;
 loop { match pc {
 0x10bee86 => {
  { cpu.interrupt(0x7c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee89); }
 let mut pc: u32 = 0x10bee89;
 loop { match pc {
 0x10bee89 => {
  { cpu.interrupt(0x7d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee8c); }
 let mut pc: u32 = 0x10bee8c;
 loop { match pc {
 0x10bee8c => {
  { cpu.interrupt(0x7e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee8f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee8f); }
 let mut pc: u32 = 0x10bee8f;
 loop { match pc {
 0x10bee8f => {
  { cpu.interrupt(0x7f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee92(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee92); }
 let mut pc: u32 = 0x10bee92;
 loop { match pc {
 0x10bee92 => {
  { cpu.interrupt(0x80); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee95(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee95); }
 let mut pc: u32 = 0x10bee95;
 loop { match pc {
 0x10bee95 => {
  { cpu.interrupt(0x81); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee98(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee98); }
 let mut pc: u32 = 0x10bee98;
 loop { match pc {
 0x10bee98 => {
  { cpu.interrupt(0x82); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee9b); }
 let mut pc: u32 = 0x10bee9b;
 loop { match pc {
 0x10bee9b => {
  { cpu.interrupt(0x83); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bee9e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bee9e); }
 let mut pc: u32 = 0x10bee9e;
 loop { match pc {
 0x10bee9e => {
  { cpu.interrupt(0x84); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beea1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beea1); }
 let mut pc: u32 = 0x10beea1;
 loop { match pc {
 0x10beea1 => {
  { cpu.interrupt(0x85); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beea4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beea4); }
 let mut pc: u32 = 0x10beea4;
 loop { match pc {
 0x10beea4 => {
  { cpu.interrupt(0x86); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beea7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beea7); }
 let mut pc: u32 = 0x10beea7;
 loop { match pc {
 0x10beea7 => {
  { cpu.interrupt(0x87); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeaa); }
 let mut pc: u32 = 0x10beeaa;
 loop { match pc {
 0x10beeaa => {
  { cpu.interrupt(0x88); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beead(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beead); }
 let mut pc: u32 = 0x10beead;
 loop { match pc {
 0x10beead => {
  { cpu.interrupt(0x89); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeb0); }
 let mut pc: u32 = 0x10beeb0;
 loop { match pc {
 0x10beeb0 => {
  { cpu.interrupt(0x8a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeb3); }
 let mut pc: u32 = 0x10beeb3;
 loop { match pc {
 0x10beeb3 => {
  { cpu.interrupt(0x8b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeb6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeb6); }
 let mut pc: u32 = 0x10beeb6;
 loop { match pc {
 0x10beeb6 => {
  { cpu.interrupt(0x8c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeb9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeb9); }
 let mut pc: u32 = 0x10beeb9;
 loop { match pc {
 0x10beeb9 => {
  { cpu.interrupt(0x8d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beebc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beebc); }
 let mut pc: u32 = 0x10beebc;
 loop { match pc {
 0x10beebc => {
  { cpu.interrupt(0x8e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beebf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beebf); }
 let mut pc: u32 = 0x10beebf;
 loop { match pc {
 0x10beebf => {
  { cpu.interrupt(0x8f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beec2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beec2); }
 let mut pc: u32 = 0x10beec2;
 loop { match pc {
 0x10beec2 => {
  { cpu.interrupt(0x90); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beec5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beec5); }
 let mut pc: u32 = 0x10beec5;
 loop { match pc {
 0x10beec5 => {
  { cpu.interrupt(0x91); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beec8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beec8); }
 let mut pc: u32 = 0x10beec8;
 loop { match pc {
 0x10beec8 => {
  { cpu.interrupt(0x92); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beecb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beecb); }
 let mut pc: u32 = 0x10beecb;
 loop { match pc {
 0x10beecb => {
  { cpu.interrupt(0x93); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beece(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beece); }
 let mut pc: u32 = 0x10beece;
 loop { match pc {
 0x10beece => {
  { cpu.interrupt(0x94); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beed1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beed1); }
 let mut pc: u32 = 0x10beed1;
 loop { match pc {
 0x10beed1 => {
  { cpu.interrupt(0x95); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beed4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beed4); }
 let mut pc: u32 = 0x10beed4;
 loop { match pc {
 0x10beed4 => {
  { cpu.interrupt(0x96); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beed7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beed7); }
 let mut pc: u32 = 0x10beed7;
 loop { match pc {
 0x10beed7 => {
  { cpu.interrupt(0x97); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeda(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeda); }
 let mut pc: u32 = 0x10beeda;
 loop { match pc {
 0x10beeda => {
  { cpu.interrupt(0x98); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beedd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beedd); }
 let mut pc: u32 = 0x10beedd;
 loop { match pc {
 0x10beedd => {
  { cpu.interrupt(0x99); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beee0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beee0); }
 let mut pc: u32 = 0x10beee0;
 loop { match pc {
 0x10beee0 => {
  { cpu.interrupt(0x9a); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beee3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beee3); }
 let mut pc: u32 = 0x10beee3;
 loop { match pc {
 0x10beee3 => {
  { cpu.interrupt(0x9b); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beee6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beee6); }
 let mut pc: u32 = 0x10beee6;
 loop { match pc {
 0x10beee6 => {
  { cpu.interrupt(0x9c); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beee9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beee9); }
 let mut pc: u32 = 0x10beee9;
 loop { match pc {
 0x10beee9 => {
  { cpu.interrupt(0x9d); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeec); }
 let mut pc: u32 = 0x10beeec;
 loop { match pc {
 0x10beeec => {
  { cpu.interrupt(0x9e); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beeef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beeef); }
 let mut pc: u32 = 0x10beeef;
 loop { match pc {
 0x10beeef => {
  { cpu.interrupt(0x9f); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beef2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beef2); }
 let mut pc: u32 = 0x10beef2;
 loop { match pc {
 0x10beef2 => {
  { cpu.interrupt(0xa0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beef5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beef5); }
 let mut pc: u32 = 0x10beef5;
 loop { match pc {
 0x10beef5 => {
  { cpu.interrupt(0xa1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beef8); }
 let mut pc: u32 = 0x10beef8;
 loop { match pc {
 0x10beef8 => {
  { cpu.interrupt(0xa2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beefb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beefb); }
 let mut pc: u32 = 0x10beefb;
 loop { match pc {
 0x10beefb => {
  { cpu.interrupt(0xa3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010beefe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10beefe); }
 let mut pc: u32 = 0x10beefe;
 loop { match pc {
 0x10beefe => {
  { cpu.interrupt(0xa4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef01); }
 let mut pc: u32 = 0x10bef01;
 loop { match pc {
 0x10bef01 => {
  { cpu.interrupt(0xa5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef04); }
 let mut pc: u32 = 0x10bef04;
 loop { match pc {
 0x10bef04 => {
  { cpu.interrupt(0xa6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef07); }
 let mut pc: u32 = 0x10bef07;
 loop { match pc {
 0x10bef07 => {
  { cpu.interrupt(0xa7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef0a); }
 let mut pc: u32 = 0x10bef0a;
 loop { match pc {
 0x10bef0a => {
  { cpu.interrupt(0xa8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef0d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef0d); }
 let mut pc: u32 = 0x10bef0d;
 loop { match pc {
 0x10bef0d => {
  { cpu.interrupt(0xa9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef10); }
 let mut pc: u32 = 0x10bef10;
 loop { match pc {
 0x10bef10 => {
  { cpu.interrupt(0xaa); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef13); }
 let mut pc: u32 = 0x10bef13;
 loop { match pc {
 0x10bef13 => {
  { cpu.interrupt(0xab); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef16(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef16); }
 let mut pc: u32 = 0x10bef16;
 loop { match pc {
 0x10bef16 => {
  { cpu.interrupt(0xac); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef19(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef19); }
 let mut pc: u32 = 0x10bef19;
 loop { match pc {
 0x10bef19 => {
  { cpu.interrupt(0xad); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef1c); }
 let mut pc: u32 = 0x10bef1c;
 loop { match pc {
 0x10bef1c => {
  { cpu.interrupt(0xae); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef1f); }
 let mut pc: u32 = 0x10bef1f;
 loop { match pc {
 0x10bef1f => {
  { cpu.interrupt(0xaf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef22); }
 let mut pc: u32 = 0x10bef22;
 loop { match pc {
 0x10bef22 => {
  { cpu.interrupt(0xb0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef25(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef25); }
 let mut pc: u32 = 0x10bef25;
 loop { match pc {
 0x10bef25 => {
  { cpu.interrupt(0xb1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef28); }
 let mut pc: u32 = 0x10bef28;
 loop { match pc {
 0x10bef28 => {
  { cpu.interrupt(0xb2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef2b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef2b); }
 let mut pc: u32 = 0x10bef2b;
 loop { match pc {
 0x10bef2b => {
  { cpu.interrupt(0xb3); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef2e); }
 let mut pc: u32 = 0x10bef2e;
 loop { match pc {
 0x10bef2e => {
  { cpu.interrupt(0xb4); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef31(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef31); }
 let mut pc: u32 = 0x10bef31;
 loop { match pc {
 0x10bef31 => {
  { cpu.interrupt(0xb5); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef34); }
 let mut pc: u32 = 0x10bef34;
 loop { match pc {
 0x10bef34 => {
  { cpu.interrupt(0xb6); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef37); }
 let mut pc: u32 = 0x10bef37;
 loop { match pc {
 0x10bef37 => {
  { cpu.interrupt(0xb7); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef3a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef3a); }
 let mut pc: u32 = 0x10bef3a;
 loop { match pc {
 0x10bef3a => {
  { cpu.interrupt(0xb8); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef3d); }
 let mut pc: u32 = 0x10bef3d;
 loop { match pc {
 0x10bef3d => {
  { cpu.interrupt(0xb9); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef40); }
 let mut pc: u32 = 0x10bef40;
 loop { match pc {
 0x10bef40 => {
  { cpu.interrupt(0xba); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef43(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef43); }
 let mut pc: u32 = 0x10bef43;
 loop { match pc {
 0x10bef43 => {
  { cpu.interrupt(0xbb); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef46(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef46); }
 let mut pc: u32 = 0x10bef46;
 loop { match pc {
 0x10bef46 => {
  { cpu.interrupt(0xbc); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef49); }
 let mut pc: u32 = 0x10bef49;
 loop { match pc {
 0x10bef49 => {
  { cpu.interrupt(0xbd); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef4c); }
 let mut pc: u32 = 0x10bef4c;
 loop { match pc {
 0x10bef4c => {
  { cpu.interrupt(0xbe); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef4f); }
 let mut pc: u32 = 0x10bef4f;
 loop { match pc {
 0x10bef4f => {
  { cpu.interrupt(0xbf); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef52); }
 let mut pc: u32 = 0x10bef52;
 loop { match pc {
 0x10bef52 => {
  { cpu.interrupt(0xc0); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef55(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef55); }
 let mut pc: u32 = 0x10bef55;
 loop { match pc {
 0x10bef55 => {
  { cpu.interrupt(0xc1); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_010bef58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x10bef58); }
 let mut pc: u32 = 0x10bef58;
 loop { match pc {
 0x10bef58 => {
  { cpu.interrupt(0xc2); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
