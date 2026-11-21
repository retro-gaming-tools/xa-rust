#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_060a42fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a42fb); }
 let mut pc: u32 = 0x60a42fb;
 loop { match pc {
 0x60a42fb => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a433c); f_060c7ee0(cpu); if cpu.returned_to != 0x60a433c { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.ebp), v); }
  { let v = (cpu.r32(0xffffffd4u32.wrapping_add(cpu.ebp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x6169414u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.push(0x60a4358); f_0609c3e2(cpu); if cpu.returned_to != 0x60a4358 { return; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x61753a0u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) / cpu.rf64(0x616941cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = (cpu.r32(0x61753a0u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.ebp)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) / cpu.rf64(0x616941cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffd8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffd8u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.fpush(v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v: u32 = 0x6255a1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a43a0); f_060c8057(cpu); if cpu.returned_to != 0x60a43a0 { return; }
  { let v: u32 = 0x6255a1cu32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a43ab); f_060c7eea(cpu); if cpu.returned_to != 0x60a43ab { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a43b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a43b3); }
 let mut pc: u32 = 0x60a43b3;
 loop { match pc {
 0x60a43b3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a43ca); f_060c7fb7(cpu); if cpu.returned_to != 0x60a43ca { return; }
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
pub fn f_060a43d1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a43d1); }
 let mut pc: u32 = 0x60a43d1;
 loop { match pc {
 0x60a43d1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60a43ee); f_060c7e81(cpu); if cpu.returned_to != 0x60a43ee { return; }
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
pub fn f_060a43f5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a43f5); }
 let mut pc: u32 = 0x60a43f5;
 loop { match pc {
 0x60a43f5 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a440a); f_060c8025(cpu); if cpu.returned_to != 0x60a440a { return; }
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
pub fn f_060a4411(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4411); }
 let mut pc: u32 = 0x60a4411;
 loop { match pc {
 0x60a4411 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x60a442e); f_060c7e81(cpu); if cpu.returned_to != 0x60a442e { return; }
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
pub fn f_060a4435(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4435); }
 let mut pc: u32 = 0x60a4435;
 loop { match pc {
 0x60a4435 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a444a); f_060c8025(cpu); if cpu.returned_to != 0x60a444a { return; }
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
pub fn f_060a4451(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4451); }
 let mut pc: u32 = 0x60a4451;
 loop { match pc {
 0x60a4451 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60a446e); f_060c7e81(cpu); if cpu.returned_to != 0x60a446e { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a447b); f_060c7e45(cpu); if cpu.returned_to != 0x60a447b { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a4482); f_060c7ea9(cpu); if cpu.returned_to != 0x60a4482 { return; }
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
pub fn f_060a4489(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4489); }
 let mut pc: u32 = 0x60a4489;
 loop { match pc {
 0x60a4489 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a44a4); f_060c7e45(cpu); if cpu.returned_to != 0x60a44a4 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60a44b3); f_060c7e81(cpu); if cpu.returned_to != 0x60a44b3 { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x60a44ba); f_060c7ea9(cpu); if cpu.returned_to != 0x60a44ba { return; }
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
pub fn f_060a44c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a44c1); }
 let mut pc: u32 = 0x60a44c1;
 loop { match pc {
 0x60a44c1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a44dc); f_060c7e45(cpu); if cpu.returned_to != 0x60a44dc { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a44e3); f_060c7ea9(cpu); if cpu.returned_to != 0x60a44e3 { return; }
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
pub fn f_060a44ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a44ea); }
 let mut pc: u32 = 0x60a44ea;
 loop { match pc {
 0x60a44ea => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a4505); f_060c7e45(cpu); if cpu.returned_to != 0x60a4505 { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x60a450c); f_060c7ea9(cpu); if cpu.returned_to != 0x60a450c { return; }
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
pub fn f_060a4513(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4513); }
 let mut pc: u32 = 0x60a4513;
 loop { match pc {
 0x60a4513 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a452e); f_060c7e45(cpu); if cpu.returned_to != 0x60a452e { return; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60a453d); f_060c7e81(cpu); if cpu.returned_to != 0x60a453d { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a4544); f_060c7ea9(cpu); if cpu.returned_to != 0x60a4544 { return; }
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
pub fn f_060a454b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a454b); }
 let mut pc: u32 = 0x60a454b;
 loop { match pc {
 0x60a454b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.push(0x60a4566); f_060c7e45(cpu); if cpu.returned_to != 0x60a4566 { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60a4575); f_060c7e81(cpu); if cpu.returned_to != 0x60a4575 { return; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x60a457c); f_060c7ea9(cpu); if cpu.returned_to != 0x60a457c { return; }
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
pub fn f_060a4583(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4583); }
 let mut pc: u32 = 0x60a4583;
 loop { match pc {
 0x60a4583 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  cpu.push(0x60a4598); f_060c7e72(cpu); if cpu.returned_to != 0x60a4598 { return; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a459f); f_060c7e77(cpu); if cpu.returned_to != 0x60a459f { return; }
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
pub fn f_060a45a6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a45a6); }
 let mut pc: u32 = 0x60a45a6;
 loop { match pc {
 0x60a45a6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a45bb); f_060c7e72(cpu); if cpu.returned_to != 0x60a45bb { return; }
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
pub fn f_060a45c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a45c2); }
 let mut pc: u32 = 0x60a45c2;
 loop { match pc {
 0x60a45c2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x60a45db); f_060c7e90(cpu); if cpu.returned_to != 0x60a45db { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a45e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a45e4); }
 let mut pc: u32 = 0x60a45e4;
 loop { match pc {
 0x60a45e4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x6255b00u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v = cpu.r32(0x625591cu32.wrapping_add(cpu.eax)); cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x60a4610); f_060c8002(cpu); if cpu.returned_to != 0x60a4610 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4619(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4619); }
 let mut pc: u32 = 0x60a4619;
 loop { match pc {
 0x60a4619 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x60a4637); f_060c7ed1(cpu); if cpu.returned_to != 0x60a4637 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4669(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4669); }
 let mut pc: u32 = 0x60a4669;
 loop { match pc {
 0x60a4669 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
  cpu.push(0x60a467c); f_060c7f0d(cpu); if cpu.returned_to != 0x60a467c { return; }
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
pub fn f_060a4683(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4683); }
 let mut pc: u32 = 0x60a4683;
 loop { match pc {
 0x60a4683 => {
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
pub fn f_060a469c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a469c); }
 let mut pc: u32 = 0x60a469c;
 loop { match pc {
 0x60a469c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  pc = 0x60a469f; continue;
 }
 0x60a469f => {
  { cpu.sub8((cpu.ebx as u8), cpu.r8(0x0u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60a46ae; continue; }
  pc = 0x60a46a3; continue;
 }
 0x60a46a3 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60a469f; continue; }
  pc = 0x60a46ac; continue;
 }
 0x60a46ac => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60a46ae; continue;
 }
 0x60a46ae => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a46b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a46b0); }
 let mut pc: u32 = 0x60a46b0;
 loop { match pc {
 0x60a46b0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a46b6; continue; }
  pc = 0x60a46b4; continue;
 }
 0x60a46b4 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60a46b6; continue;
 }
 0x60a46b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a46b8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a46b8); }
 let mut pc: u32 = 0x60a46b8;
 loop { match pc {
 0x60a46b8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x617798cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x60a46fd; continue; }
  pc = 0x60a46da; continue;
 }
 0x60a46da => {
  { let v: u32 = 0x6169424u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a46e9); f_060a3b01(cpu); if cpu.returned_to != 0x60a46e9 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a46fa); f_060a3a48(cpu); if cpu.returned_to != 0x60a46fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60a46fd; continue;
 }
 0x60a46fd => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617798cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4714(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4714); }
 let mut pc: u32 = 0x60a4714;
 loop { match pc {
 0x60a4714 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x12cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.r32(0x61741c6u32), 0x0u32); }
  if !cpu.zf { pc = 0x60a477a; continue; }
  pc = 0x60a472e; continue;
 }
 0x60a472e => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x7ef4u32; cpu.eax = v; }
  cpu.push(0x60a473d); f_0609ad7c(cpu); if cpu.returned_to != 0x60a473d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x61741c6u32, v); }
  { cpu.sub32(cpu.r32(0x61741c6u32), 0x0u32); }
  if !cpu.zf { pc = 0x60a477a; continue; }
  pc = 0x60a474b; continue;
 }
 0x60a474b => {
  { let v: u32 = 0x6169468u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a475a); f_060a3b01(cpu); if cpu.returned_to != 0x60a475a { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a476b); f_060a3a48(cpu); if cpu.returned_to != 0x60a476b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a477a => {
  { let v: u32 = 0x61694a6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x61694a7u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2cu32); cpu.ebx = v; }
  { let v: u32 = 0x61694a6u32; cpu.edx = v; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a479b); f_060b1850(cpu); if cpu.returned_to != 0x60a479b { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x61753f8u32); cpu.edx = v; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a47ae); f_060a2400(cpu); if cpu.returned_to != 0x60a47ae { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x61753f8u32, v); }
  { cpu.sub32(cpu.r32(0x61753f8u32), 0x0u32); }
  if !cpu.zf { pc = 0x60a480a; continue; }
  pc = 0x60a47bc; continue;
 }
 0x60a47bc => {
  { let v: u32 = 0x61694b4u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a47cb); f_060a3b01(cpu); if cpu.returned_to != 0x60a47cb { return; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a47db); f_060a3b20(cpu); if cpu.returned_to != 0x60a47db { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a47ea); f_060a3b20(cpu); if cpu.returned_to != 0x60a47ea { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a47fb); f_060a3a48(cpu); if cpu.returned_to != 0x60a47fb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a480a => {
  { let v: u32 = 0x61694a6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x61694dcu32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2cu32); cpu.ebx = v; }
  { let v: u32 = 0x61694a6u32; cpu.edx = v; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a482b); f_060b1850(cpu); if cpu.returned_to != 0x60a482b { return; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a483b); f_060a2350(cpu); if cpu.returned_to != 0x60a483b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if (cpu.sf == cpu.of) { pc = 0x60a4893; continue; }
  pc = 0x60a4845; continue;
 }
 0x60a4845 => {
  { let v: u32 = 0x61694e9u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4854); f_060a3b01(cpu); if cpu.returned_to != 0x60a4854 { return; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4864); f_060a3b20(cpu); if cpu.returned_to != 0x60a4864 { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4873); f_060a3b20(cpu); if cpu.returned_to != 0x60a4873 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4884); f_060a3a48(cpu); if cpu.returned_to != 0x60a4884 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a4893 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a489a; continue;
 }
 0x60a489a => {
  { let v: u32 = cpu.r32(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a48a5); f_060a3630(cpu); if cpu.returned_to != 0x60a48a5 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60a4914; continue; }
  pc = 0x60a48aa; continue;
 }
 0x60a48aa => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a48bd); f_060a29d0(cpu); if cpu.returned_to != 0x60a48bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a490f; continue; }
  pc = 0x60a48c1; continue;
 }
 0x60a48c1 => {
  { let v: u32 = 0x6169508u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a48d0); f_060a3b01(cpu); if cpu.returned_to != 0x60a48d0 { return; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a48e0); f_060a3b20(cpu); if cpu.returned_to != 0x60a48e0 { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a48ef); f_060a3b20(cpu); if cpu.returned_to != 0x60a48ef { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4900); f_060a3a48(cpu); if cpu.returned_to != 0x60a4900 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a490f => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a489a; continue;
 }
 0x60a4914 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x61741c2u32, v); }
  { cpu.sub32(cpu.r32(0x61741c2u32), 0x9c4u32); }
  if (cpu.sf != cpu.of) { pc = 0x60a4957; continue; }
  pc = 0x60a4928; continue;
 }
 0x60a4928 => {
  { let v: u32 = 0x616952au32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4937); f_060a3b01(cpu); if cpu.returned_to != 0x60a4937 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4948); f_060a3a48(cpu); if cpu.returned_to != 0x60a4948 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a4957 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a4969); f_060a2be8(cpu); if cpu.returned_to != 0x60a4969 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x2710u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x617798cu32; cpu.eax = v; }
  cpu.push(0x60a4981); f_060ae9e0(cpu); if cpu.returned_to != 0x60a4981 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4988; continue;
 }
 0x60a4988 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x61741c2u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a499d; continue; }
  pc = 0x60a4993; continue;
 }
 0x60a4993 => {
  pc = 0x60a4a2e; continue;
 }
 0x60a4998 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4988; continue;
 }
 0x60a499d => {
  { let v: u32 = 0x12u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a49b0); f_060a29d0(cpu); if cpu.returned_to != 0x60a49b0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a49ff; continue; }
  pc = 0x60a49b4; continue;
 }
 0x60a49b4 => {
  { let v: u32 = 0x6169558u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a49c3); f_060a3b01(cpu); if cpu.returned_to != 0x60a49c3 { return; }
  { let v: u32 = 0xfffffed4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a49d3); f_060a3b20(cpu); if cpu.returned_to != 0x60a49d3 { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a49e2); f_060a3b20(cpu); if cpu.returned_to != 0x60a49e2 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a49f3); f_060a3a48(cpu); if cpu.returned_to != 0x60a49f3 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a49ff => {
  { let v: u32 = cpu.r32(0xffffffe6u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x61753f8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  cpu.push(0x60a4a15); f_060a46b8(cpu); if cpu.returned_to != 0x60a4a15 { return; }
  { let v: u32 = 0xffffffd8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)), 0xdu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x61741c6u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x60a4a29); f_060a3b01(cpu); if cpu.returned_to != 0x60a4a29 { return; }
  cpu.poll(); pc = 0x60a4998; continue;
 }
 0x60a4a2e => {
  { let v: u32 = cpu.r32(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a4a39); f_060a2964(cpu); if cpu.returned_to != 0x60a4a39 { return; }
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4a40; continue;
 }
 0x60a4a40 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4a4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4a4c); }
 let mut pc: u32 = 0x60a4a4c;
 loop { match pc {
 0x60a4a4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x61694a6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x616957au32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x130u32); cpu.ebx = v; }
  { let v: u32 = 0x61694a6u32; cpu.edx = v; }
  { let v: u32 = 0xfffffef4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a4a81); f_060b1850(cpu); if cpu.returned_to != 0x60a4a81 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x625f934u32; cpu.edx = v; }
  { let v: u32 = 0xfffffef4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a4a93); f_060a2400(cpu); if cpu.returned_to != 0x60a4a93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a4ae5; continue; }
  pc = 0x60a4a97; continue;
 }
 0x60a4a97 => {
  { let v: u32 = 0x6169583u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4aa6); f_060a3b01(cpu); if cpu.returned_to != 0x60a4aa6 { return; }
  { let v: u32 = 0xfffffef4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4ab6); f_060a3b20(cpu); if cpu.returned_to != 0x60a4ab6 { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4ac5); f_060a3b20(cpu); if cpu.returned_to != 0x60a4ac5 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4ad6); f_060a3a48(cpu); if cpu.returned_to != 0x60a4ad6 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4b71; continue;
 }
 0x60a4ae5 => {
  { let v: u32 = 0x61694a6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x61695a2u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x130u32); cpu.ebx = v; }
  { let v: u32 = 0x61694a6u32; cpu.edx = v; }
  { let v: u32 = 0xfffffef4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a4b09); f_060b1850(cpu); if cpu.returned_to != 0x60a4b09 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x6262134u32; cpu.edx = v; }
  { let v: u32 = 0xfffffef4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a4b1b); f_060a2400(cpu); if cpu.returned_to != 0x60a4b1b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a4b6a; continue; }
  pc = 0x60a4b1f; continue;
 }
 0x60a4b1f => {
  { let v: u32 = 0x61695aau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4b2e); f_060a3b01(cpu); if cpu.returned_to != 0x60a4b2e { return; }
  { let v: u32 = 0xfffffef4u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4b3e); f_060a3b20(cpu); if cpu.returned_to != 0x60a4b3e { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4b4d); f_060a3b20(cpu); if cpu.returned_to != 0x60a4b4d { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4b5e); f_060a3a48(cpu); if cpu.returned_to != 0x60a4b5e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4b71; continue;
 }
 0x60a4b6a => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4b71; continue;
 }
 0x60a4b71 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4b7d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4b7d); }
 let mut pc: u32 = 0x60a4b7d;
 loop { match pc {
 0x60a4b7d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62cb9b8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x60a4ba2; continue; }
  pc = 0x60a4b98; continue;
 }
 0x60a4b98 => {
  { let v: u32 = cpu.r32(0x62cb9b8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4baa; continue;
 }
 0x60a4ba2 => {
  { let v: u32 = cpu.r32(0x62cb9b4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4baa; continue;
 }
 0x60a4baa => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4bb7; continue;
 }
 0x60a4bb7 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); }
  if (cpu.sf != cpu.of) { pc = 0x60a4bc6; continue; }
  pc = 0x60a4bbf; continue;
 }
 0x60a4bbf => {
  pc = 0x60a4bd0; continue;
 }
 0x60a4bc1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4bb7; continue;
 }
 0x60a4bc6 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  cpu.push(0x60a4bce); f_060d6c00(cpu); if cpu.returned_to != 0x60a4bce { return; }
  cpu.poll(); pc = 0x60a4bc1; continue;
 }
 0x60a4bd0 => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4bd9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4bd9); }
 let mut pc: u32 = 0x60a4bd9;
 loop { match pc {
 0x60a4bd9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x62cb9b0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4bf8; continue;
 }
 0x60a4bf8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a4c0a; continue; }
  pc = 0x60a4c03; continue;
 }
 0x60a4c03 => {
  pc = 0x60a4c20; continue;
 }
 0x60a4c05 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4bf8; continue;
 }
 0x60a4c0a => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x61753f0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60a4c05; continue;
 }
 0x60a4c20 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4c27; continue;
 }
 0x60a4c27 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a4c39; continue; }
  pc = 0x60a4c32; continue;
 }
 0x60a4c32 => {
  pc = 0x60a4c4b; continue;
 }
 0x60a4c34 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4c27; continue;
 }
 0x60a4c39 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x617540cu32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60a4c34; continue;
 }
 0x60a4c4b => {
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4c54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4c54); }
 let mut pc: u32 = 0x60a4c54;
 loop { match pc {
 0x60a4c54 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = 0x617540cu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4c84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4c84); }
 let mut pc: u32 = 0x60a4c84;
 loop { match pc {
 0x60a4c84 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x617540cu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4caa; continue;
 }
 0x60a4caa => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a4cbc; continue; }
  pc = 0x60a4cb5; continue;
 }
 0x60a4cb5 => {
  pc = 0x60a4cdc; continue;
 }
 0x60a4cb7 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4caa; continue;
 }
 0x60a4cbc => {
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x13u32); cpu.eax = v; }
  cpu.push(0x60a4cca); f_060a2100(cpu); if cpu.returned_to != 0x60a4cca { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a4cd6; continue; }
  pc = 0x60a4cce; continue;
 }
 0x60a4cce => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4cdc; continue;
 }
 0x60a4cd6 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x20u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4cb7; continue;
 }
 0x60a4cdc => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4cee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4cee); }
 let mut pc: u32 = 0x60a4cee;
 loop { match pc {
 0x60a4cee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60a4d07); f_060a4c84(cpu); if cpu.returned_to != 0x60a4d07 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60a4d12); f_060d6a65(cpu); if cpu.returned_to != 0x60a4d12 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4d1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4d1b); }
 let mut pc: u32 = 0x60a4d1b;
 loop { match pc {
 0x60a4d1b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x617540cu32; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4d41; continue;
 }
 0x60a4d41 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a4d53; continue; }
  pc = 0x60a4d4c; continue;
 }
 0x60a4d4c => {
  pc = 0x60a4d6f; continue;
 }
 0x60a4d4e => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4d41; continue;
 }
 0x60a4d53 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x60a4d69; continue; }
  pc = 0x60a4d5e; continue;
 }
 0x60a4d5e => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x13u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4d76; continue;
 }
 0x60a4d69 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)).wrapping_add(0x20u32); cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4d4e; continue;
 }
 0x60a4d6f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4d76; continue;
 }
 0x60a4d76 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4d82(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4d82); }
 let mut pc: u32 = 0x60a4d82;
 loop { match pc {
 0x60a4d82 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62cb9b8u32, v); }
  { cpu.sub32(cpu.r32(0x62cb9b8u32), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60a4e74; continue; }
  pc = 0x60a4dab; continue;
 }
 0x60a4dab => {
  { cpu.sub32(cpu.r32(0x62cb9b8u32), 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60a4de6; continue; }
  pc = 0x60a4db7; continue;
 }
 0x60a4db7 => {
  { let v: u32 = 0x61695c9u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4dc6); f_060a3b01(cpu); if cpu.returned_to != 0x60a4dc6 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4dd7); f_060a3a48(cpu); if cpu.returned_to != 0x60a4dd7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4e7b; continue;
 }
 0x60a4de6 => {
  { cpu.sub32(cpu.r32(0x61753f0u32), 0x0u32); }
  if !cpu.zf { pc = 0x60a4e3b; continue; }
  pc = 0x60a4def; continue;
 }
 0x60a4def => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62cb9b8u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  cpu.push(0x60a4e01); f_0609ad7c(cpu); if cpu.returned_to != 0x60a4e01 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x61753f0u32, v); }
  { cpu.sub32(cpu.r32(0x61753f0u32), 0x0u32); }
  if !cpu.zf { pc = 0x60a4e3b; continue; }
  pc = 0x60a4e0f; continue;
 }
 0x60a4e0f => {
  { let v: u32 = 0x6169600u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4e1e); f_060a3b01(cpu); if cpu.returned_to != 0x60a4e1e { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4e2f); f_060a3a48(cpu); if cpu.returned_to != 0x60a4e2f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4e7b; continue;
 }
 0x60a4e3b => {
  { let v: u32 = 0x0u32; cpu.w32(0x62cb9b0u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4e4c; continue;
 }
 0x60a4e4c => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b8u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a4e5e; continue; }
  pc = 0x60a4e57; continue;
 }
 0x60a4e57 => {
  pc = 0x60a4e74; continue;
 }
 0x60a4e59 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a4e4c; continue;
 }
 0x60a4e5e => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x61753f0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60a4e59; continue;
 }
 0x60a4e74 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4e7b; continue;
 }
 0x60a4e7b => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a4e87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a4e87); }
 let mut pc: u32 = 0x60a4e87;
 loop { match pc {
 0x60a4e87 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x124u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x61694a6u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169646u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x234u32); cpu.ebx = v; }
  { let v: u32 = 0x61694a6u32; cpu.edx = v; }
  { let v: u32 = 0xfffffedcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a4ec3); f_060b1850(cpu); if cpu.returned_to != 0x60a4ec3 { return; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0xfffffedcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  cpu.push(0x60a4ed3); f_060a2350(cpu); if cpu.returned_to != 0x60a4ed3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub16(cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if (cpu.sf == cpu.of) { pc = 0x60a4f2b; continue; }
  pc = 0x60a4edd; continue;
 }
 0x60a4edd => {
  { let v: u32 = 0x6169652u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4eec); f_060a3b01(cpu); if cpu.returned_to != 0x60a4eec { return; }
  { let v: u32 = 0xfffffedcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4efc); f_060a3b20(cpu); if cpu.returned_to != 0x60a4efc { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4f0b); f_060a3b20(cpu); if cpu.returned_to != 0x60a4f0b { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4f1c); f_060a3a48(cpu); if cpu.returned_to != 0x60a4f1c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5145; continue;
 }
 0x60a4f2b => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a4f3e); f_060a29d0(cpu); if cpu.returned_to != 0x60a4f3e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a4f49; continue; }
  pc = 0x60a4f42; continue;
 }
 0x60a4f42 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4f49; continue;
 }
 0x60a4f49 => {
  { cpu.sub32(cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)), 0x4e494244u32); }
  if cpu.zf { pc = 0x60a4fa0; continue; }
  pc = 0x60a4f52; continue;
 }
 0x60a4f52 => {
  { let v: u32 = 0x6169680u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4f61); f_060a3b01(cpu); if cpu.returned_to != 0x60a4f61 { return; }
  { let v: u32 = 0xfffffedcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4f71); f_060a3b20(cpu); if cpu.returned_to != 0x60a4f71 { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4f80); f_060a3b20(cpu); if cpu.returned_to != 0x60a4f80 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4f91); f_060a3a48(cpu); if cpu.returned_to != 0x60a4f91 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5145; continue;
 }
 0x60a4fa0 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0x62cb9b4u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a4fb5); f_060a29d0(cpu); if cpu.returned_to != 0x60a4fb5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a4fc0; continue; }
  pc = 0x60a4fb9; continue;
 }
 0x60a4fb9 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a4fc0; continue;
 }
 0x60a4fc0 => {
  { cpu.sub32(cpu.r32(0x62cb9b4u32), 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60a4ffb; continue; }
  pc = 0x60a4fcc; continue;
 }
 0x60a4fcc => {
  { let v: u32 = 0x61696a9u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a4fdb); f_060a3b01(cpu); if cpu.returned_to != 0x60a4fdb { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a4fec); f_060a3a48(cpu); if cpu.returned_to != 0x60a4fec { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5145; continue;
 }
 0x60a4ffb => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5002; continue;
 }
 0x60a5002 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x62cb9b4u32)); }
  if (cpu.sf != cpu.of) { pc = 0x60a5017; continue; }
  pc = 0x60a500d; continue;
 }
 0x60a500d => {
  pc = 0x60a50e1; continue;
 }
 0x60a5012 => {
  { let v: u32 = cpu.inc32(cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp))); cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60a5002; continue;
 }
 0x60a5017 => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a502a); f_060a29d0(cpu); if cpu.returned_to != 0x60a502a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a503a; continue; }
  pc = 0x60a502e; continue;
 }
 0x60a502e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a50e1; continue;
 }
 0x60a503a => {
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = 0x617540cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xdu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x13u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a5060); f_060a29d0(cpu); if cpu.returned_to != 0x60a5060 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a506b; continue; }
  pc = 0x60a5064; continue;
 }
 0x60a5064 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a506b; continue;
 }
 0x60a506b => {
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a507e); f_060a29d0(cpu); if cpu.returned_to != 0x60a507e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a5089; continue; }
  pc = 0x60a5082; continue;
 }
 0x60a5082 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5089; continue;
 }
 0x60a5089 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = 0xffffffe0u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a50a5); f_060a29d0(cpu); if cpu.returned_to != 0x60a50a5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60a50b0; continue; }
  pc = 0x60a50a9; continue;
 }
 0x60a50a9 => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a50b0; continue;
 }
 0x60a50b0 => {
  { let v: u32 = cpu.r32(0xffffffe0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x12u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60a5012; continue;
 }
 0x60a50e1 => {
  { let v: u32 = cpu.r32(0xfffffff2u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  cpu.push(0x60a50ec); f_060a2964(cpu); if cpu.returned_to != 0x60a50ec { return; }
  { cpu.sub16(cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)), 0x1u16); }
  if !cpu.zf { pc = 0x60a513e; continue; }
  pc = 0x60a50f3; continue;
 }
 0x60a50f3 => {
  { let v: u32 = 0x61696e5u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a5102); f_060a3b01(cpu); if cpu.returned_to != 0x60a5102 { return; }
  { let v: u32 = 0xfffffedcu32.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a5112); f_060a3b20(cpu); if cpu.returned_to != 0x60a5112 { return; }
  { let v: u32 = 0x61694dau32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a5121); f_060a3b20(cpu); if cpu.returned_to != 0x60a5121 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5132); f_060a3a48(cpu); if cpu.returned_to != 0x60a5132 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5145; continue;
 }
 0x60a513e => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5145; continue;
 }
 0x60a5145 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a5151(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5151); }
 let mut pc: u32 = 0x60a5151;
 loop { match pc {
 0x60a5151 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x62cb9b4u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x12cu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60a519d; continue; }
  pc = 0x60a5171; continue;
 }
 0x60a5171 => {
  { let v: u32 = 0x6169707u32; cpu.edx = v; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  cpu.push(0x60a5180); f_060a3b01(cpu); if cpu.returned_to != 0x60a5180 { return; }
  { let v: u32 = 0x6255b14u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6169465u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5191); f_060a3a48(cpu); if cpu.returned_to != 0x60a5191 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5205; continue;
 }
 0x60a519d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x62cb9b4u32)); cpu.w32(0x62cb9b4u32, v); }
  { let v: u32 = cpu.r32(0x62cb9b4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x617540cu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x13u32); cpu.eax = v; }
  cpu.push(0x60a51c6); f_060a3b01(cpu); if cpu.returned_to != 0x60a51c6 { return; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0xffffu16; cpu.w16(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x1u8; cpu.w8(0x12u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62cb9b4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5205; continue;
 }
 0x60a5205 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a5210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5210); }
 let mut pc: u32 = 0x60a5210;
 loop { match pc {
 0x60a5210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x0u32); cpu.esp = v; }
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
pub fn f_060a52b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a52b9); }
 let mut pc: u32 = 0x60a52b9;
 loop { match pc {
 0x60a52b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60a52d8); f_060a4a4c(cpu); if cpu.returned_to != 0x60a52d8 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60a52e9; continue; }
  pc = 0x60a52dd; continue;
 }
 0x60a52dd => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5394; continue;
 }
 0x60a52e9 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60a52f1); f_060a4714(cpu); if cpu.returned_to != 0x60a52f1 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60a5302; continue; }
  pc = 0x60a52f6; continue;
 }
 0x60a52f6 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5394; continue;
 }
 0x60a5302 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x338u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x61753fcu32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62cb9bcu32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60a5323); f_060a4d82(cpu); if cpu.returned_to != 0x60a5323 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60a5331; continue; }
  pc = 0x60a5328; continue;
 }
 0x60a5328 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5394; continue;
 }
 0x60a5331 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60a5339); f_060a4e87(cpu); if cpu.returned_to != 0x60a5339 { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60a5347; continue; }
  pc = 0x60a533e; continue;
 }
 0x60a533e => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5394; continue;
 }
 0x60a5347 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  cpu.push(0x60a5375); f_060d1a60(cpu); if cpu.returned_to != 0x60a5375 { return; }
  cpu.push(0x60a537a); f_060d1c12(cpu); if cpu.returned_to != 0x60a537a { return; }
  cpu.push(0x60a537f); f_060a3e7a(cpu); if cpu.returned_to != 0x60a537f { return; }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if !cpu.zf { pc = 0x60a538d; continue; }
  pc = 0x60a5384; continue;
 }
 0x60a5384 => {
  { let v: u32 = 0x0u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5394; continue;
 }
 0x60a538d => {
  { let v: u32 = 0x1u32; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60a5394; continue;
 }
 0x60a5394 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
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
pub fn f_060a53a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a53a0); }
 let mut pc: u32 = 0x60a53a0;
 loop { match pc {
 0x60a53a0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x61741c6u32), 0x0u32); }
  if cpu.zf { pc = 0x60a53bc; continue; }
  pc = 0x60a53b2; continue;
 }
 0x60a53b2 => {
  { let v: u32 = cpu.r32(0x61741c6u32); cpu.eax = v; }
  cpu.push(0x60a53bc); f_0609b400(cpu); if cpu.returned_to != 0x60a53bc { return; }
  pc = 0x60a53bc; continue;
 }
 0x60a53bc => {
  { let v: u32 = 0x0u32; cpu.w32(0x61741c6u32, v); }
  { cpu.sub32(cpu.r32(0x61753f0u32), 0x0u32); }
  if cpu.zf { pc = 0x60a53d9; continue; }
  pc = 0x60a53cf; continue;
 }
 0x60a53cf => {
  { let v: u32 = cpu.r32(0x61753f0u32); cpu.eax = v; }
  cpu.push(0x60a53d9); f_0609b400(cpu); if cpu.returned_to != 0x60a53d9 { return; }
  pc = 0x60a53d9; continue;
 }
 0x60a53d9 => {
  { let v: u32 = 0x0u32; cpu.w32(0x61753f0u32, v); }
  { cpu.sub32(cpu.r32(0x61753f8u32), 0x0u32); }
  if cpu.zf { pc = 0x60a53f6; continue; }
  pc = 0x60a53ec; continue;
 }
 0x60a53ec => {
  { let v: u32 = cpu.r32(0x61753f8u32); cpu.eax = v; }
  cpu.push(0x60a53f6); f_0609b400(cpu); if cpu.returned_to != 0x60a53f6 { return; }
  pc = 0x60a53f6; continue;
 }
 0x60a53f6 => {
  { let v: u32 = 0x0u32; cpu.w32(0x61753f8u32, v); }
  cpu.push(0x60a5405); f_060a4669(cpu); if cpu.returned_to != 0x60a5405 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5409(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5409); }
 let mut pc: u32 = 0x60a5409;
 loop { match pc {
 0x60a5409 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x60a540c; continue;
 }
 0x60a540c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60a542a; continue; }
  pc = 0x60a5427; continue;
 }
 0x60a5427 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60a540c; continue;
 }
 0x60a542a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2bu32); }
  if cpu.zf { pc = 0x60a5438; continue; }
  pc = 0x60a5433; continue;
 }
 0x60a5433 => {
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x60a5439; continue; }
  pc = 0x60a5438; continue;
 }
 0x60a5438 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60a5439; continue;
 }
 0x60a5439 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60a543b; continue;
 }
 0x60a543b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60a5465; continue; }
  pc = 0x60a5456; continue;
 }
 0x60a5456 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x30u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x60a543b; continue;
 }
 0x60a5465 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x60a5470; continue; }
  pc = 0x60a546e; continue;
 }
 0x60a546e => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x60a5470; continue;
 }
 0x60a5470 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5476(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5476); }
 let mut pc: u32 = 0x60a5476;
 loop { match pc {
 0x60a5476 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  pc = 0x60a547c; continue;
 }
 0x60a547c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60a5485; continue; }
  pc = 0x60a5480; continue;
 }
 0x60a5480 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60a5485 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x60a5499; continue; }
  pc = 0x60a5492; continue;
 }
 0x60a5492 => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60a5499; continue; }
  pc = 0x60a5497; continue;
 }
 0x60a5497 => {
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60a5499; continue;
 }
 0x60a5499 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x60a54aa; continue; }
  pc = 0x60a54a2; continue;
 }
 0x60a54a2 => {
  { cpu.sub32(cpu.ebx, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60a54aa; continue; }
  pc = 0x60a54a7; continue;
 }
 0x60a54a7 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_add(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60a54aa; continue;
 }
 0x60a54aa => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x60a54c0; continue; }
  pc = 0x60a54ae; continue;
 }
 0x60a54ae => {
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
 0x60a54c0 => {
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { cpu.poll(); pc = 0x60a5480; continue; }
  pc = 0x60a54c4; continue;
 }
 0x60a54c4 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60a547c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a54cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a54cc); }
 let mut pc: u32 = 0x60a54cc;
 loop { match pc {
 0x60a54cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60a54da); f_060b1a3b(cpu); if cpu.returned_to != 0x60a54da { return; }
  { cpu.sub32(cpu.eax, 0x1f4u32); }
  if (cpu.sf != cpu.of) { pc = 0x60a54e4; continue; }
  pc = 0x60a54e1; continue;
 }
 0x60a54e1 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60a54e4; continue;
 }
 0x60a54e4 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60a54eb); f_060b1afe(cpu); if cpu.returned_to != 0x60a54eb { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60a54f3; continue; }
  pc = 0x60a54f1; continue;
 }
 0x60a54f1 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x60a54f3; continue;
 }
 0x60a54f3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a54fb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a54fb); }
 let mut pc: u32 = 0x60a54fb;
 loop { match pc {
 0x60a54fb => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  cpu.push(0x60a5508); f_060b1cbe(cpu); if cpu.returned_to != 0x60a5508 { return; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617b8b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x60a551e); f_060b2022(cpu); if cpu.returned_to != 0x60a551e { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60a5525); f_060b22b6(cpu); if cpu.returned_to != 0x60a5525 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5545; continue; }
  pc = 0x60a5529; continue;
 }
 0x60a5529 => {
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617b8b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617b8bcu32); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x60a5545); f_060b2022(cpu); if cpu.returned_to != 0x60a5545 { return; }
  pc = 0x60a5545; continue;
 }
 0x60a5545 => {
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
pub fn f_060a554c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a554c); }
 let mut pc: u32 = 0x60a554c;
 loop { match pc {
 0x60a554c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x6255cb8u32; cpu.edx = v; }
  cpu.push(0x60a5557); f_060a54fb(cpu); if cpu.returned_to != 0x60a5557 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a555c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a555c); }
 let mut pc: u32 = 0x60a555c;
 loop { match pc {
 0x60a555c => {
  { cpu.sub32(cpu.eax, 0x41u32); }
  if (cpu.sf != cpu.of) { pc = 0x60a5569; continue; }
  pc = 0x60a5561; continue;
 }
 0x60a5561 => {
  { cpu.sub32(cpu.eax, 0x5au32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60a5569; continue; }
  pc = 0x60a5566; continue;
 }
 0x60a5566 => {
  { let v: u32 = cpu.add32(cpu.eax, 0x20u32); cpu.eax = v; }
  pc = 0x60a5569; continue;
 }
 0x60a5569 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a556c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a556c); }
 let mut pc: u32 = 0x60a556c;
 loop { match pc {
 0x60a556c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  cpu.push(0x60a557c); f_060b25ac(cpu); if cpu.returned_to != 0x60a557c { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60a5583); f_060aeac1(cpu); if cpu.returned_to != 0x60a5583 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a558c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a558c); }
 let mut pc: u32 = 0x60a558c;
 loop { match pc {
 0x60a558c => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x60a55bb; continue; }
  pc = 0x60a55a1; continue;
 }
 0x60a55a1 => {
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60a55a3; continue;
 }
 0x60a55a3 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60a55b3); f_060b25d4(cpu); if cpu.returned_to != 0x60a55b3 { return; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x8u32); cpu.esp = v; }
  pc = 0x60a56b4; continue;
 }
 0x60a55bb => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffau32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffbu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if !cpu.cf { pc = 0x60a5606; continue; }
  pc = 0x60a55cb; continue;
 }
 0x60a55cb => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { cpu.poll(); pc = 0x60a55a3; continue; }
  pc = 0x60a55d0; continue;
 }
 0x60a55d0 => {
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
  if cpu.pf() { pc = 0x60a55fd; continue; }
  pc = 0x60a55ec; continue;
 }
 0x60a55ec => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffffdu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  if cpu.zf { pc = 0x60a55fd; continue; }
  pc = 0x60a55fb; continue;
 }
 0x60a55fb => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60a55fd; continue;
 }
 0x60a55fd => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0xfffffff9u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x60a560e; continue;
 }
 0x60a5606 => {
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.zf { pc = 0x60a560e; continue; }
  pc = 0x60a5609; continue;
 }
 0x60a5609 => {
  pc = 0x60a5695; continue;
 }
 0x60a560e => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff6u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0x7ff0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16).wrapping_sub(0x3ff0u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x100u16); }
  if !cpu.cf { pc = 0x60a5695; continue; }
  pc = 0x60a5626; continue;
 }
 0x60a5626 => {
  { let v = cpu.st(1); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  {  }
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), (cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60a5640; continue; }
  pc = 0x60a5635; continue;
 }
 0x60a5635 => {
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60a563e); f_060a56bf(cpu); if cpu.returned_to != 0x60a563e { return; }
  pc = 0x60a566b; continue;
 }
 0x60a5640 => {
  { let v: u16 = cpu.inc16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60a5695; continue; }
  pc = 0x60a5644; continue;
 }
 0x60a5644 => {
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60a5695; continue; }
  pc = 0x60a564a; continue;
 }
 0x60a564a => {
  { let v: u16 = cpu.neg16((cpu.eax as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60a5652); f_060a56bf(cpu); if cpu.returned_to != 0x60a5652 { return; }
  { cpu.fpush(1.0); }
  { cpu.and8(cpu.r8(0x617b930u32), 0x1u8); }
  if !cpu.zf { pc = 0x60a5661; continue; }
  pc = 0x60a565d; continue;
 }
 0x60a565d => {
  { let v = cpu.st(0) / cpu.st(1); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x60a566b; continue;
 }
 0x60a5661 => {
  { let v: u32 = 0xfu32; cpu.eax = v; }
  cpu.push(0x60a566b); f_060b27eb(cpu); if cpu.returned_to != 0x60a566b { return; }
  pc = 0x60a566b; continue;
 }
 0x60a566b => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u16 = cpu.r16(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.zf { pc = 0x60a5691; continue; }
  pc = 0x60a567d; continue;
 }
 0x60a567d => {
  { let v: u16 = cpu.r16(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.shl16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0xffe0u16); }
  if !cpu.zf { pc = 0x60a5691; continue; }
  pc = 0x60a568a; continue;
 }
 0x60a568a => {
  { let v: u8 = 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x60a55a3; continue;
 }
 0x60a5691 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x60a56b4; continue;
 }
 0x60a5695 => {
  { cpu.fpush(std::f64::consts::LN_2); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0).abs(); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0).log2(); cpu.fpop(); cpu.set_st(0, v); }
  { let v: u8 = 0x7u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60a56a6); f_060b2668(cpu); if cpu.returned_to != 0x60a56a6 { return; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60a568a; continue; }
  pc = 0x60a56aa; continue;
 }
 0x60a56aa => {
  { let v: u8 = cpu.r8(0xfffffff9u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x60a56b2; continue; }
  pc = 0x60a56b0; continue;
 }
 0x60a56b0 => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x60a56b2; continue;
 }
 0x60a56b2 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  pc = 0x60a56b4; continue;
 }
 0x60a56b4 => {
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
pub fn f_060a56bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a56bf); }
 let mut pc: u32 = 0x60a56bf;
 loop { match pc {
 0x60a56bf => {
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60a56c8; continue; }
  pc = 0x60a56c4; continue;
 }
 0x60a56c4 => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  cpu.poll(); pc = 0x60a56bf; continue;
 }
 0x60a56c8 => {
  if !cpu.cf { pc = 0x60a56dd; continue; }
  pc = 0x60a56ca; continue;
 }
 0x60a56ca => {
  { let v = cpu.st(0); cpu.fpush(v); }
  pc = 0x60a56cc; continue;
 }
 0x60a56cc => {
  if cpu.zf { pc = 0x60a56d9; continue; }
  pc = 0x60a56ce; continue;
 }
 0x60a56ce => {
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v: u16 = cpu.shr16((cpu.eax as u16), 1u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  if !cpu.cf { pc = 0x60a56d7; continue; }
  pc = 0x60a56d5; continue;
 }
 0x60a56d5 => {
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  pc = 0x60a56d7; continue;
 }
 0x60a56d7 => {
  cpu.poll(); pc = 0x60a56cc; continue;
 }
 0x60a56d9 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x60a56e1; continue;
 }
 0x60a56dd => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  pc = 0x60a56e1; continue;
 }
 0x60a56e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5700); }
 let mut pc: u32 = 0x60a5700;
 loop { match pc {
 0x60a5700 => {
  { cpu.sub32(cpu.r32(0x617a3f4u32), 0x0u32); }
  if cpu.zf { pc = 0x60a5723; continue; }
  pc = 0x60a5709; continue;
 }
 0x60a5709 => {
  { let v = 0x60a5700u32; cpu.push(v); }
  { let v = 0x60ad7f0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60a571a); f_060b30c0(cpu); if cpu.returned_to != 0x60a571a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617a3f4u32, v); }
  pc = 0x60a5723; continue;
 }
 0x60a5723 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5730); }
 let mut pc: u32 = 0x60a5730;
 loop { match pc {
 0x60a5730 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5777; continue; }
  pc = 0x60a5750; continue;
 }
 0x60a5750 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a575e; continue; }
  pc = 0x60a5755; continue;
 }
 0x60a5755 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5777; continue; }
  pc = 0x60a575e; continue;
 }
 0x60a575e => {
  cpu.push(0x60a5763); f_060b3c01(cpu); if cpu.returned_to != 0x60a5763 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5777; continue; }
  pc = 0x60a5767; continue;
 }
 0x60a5767 => {
  cpu.push(0x60a576c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a576c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5777; continue; }
  pc = 0x60a5770; continue;
 }
 0x60a5770 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5779; continue;
 }
 0x60a5777 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5779; continue;
 }
 0x60a5779 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a57c7; continue; }
  pc = 0x60a577d; continue;
 }
 0x60a577d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a9f4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a57bf); cpu.call(t); if cpu.returned_to != 0x60a57bf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a57c7); f_060ad7c0(cpu); if cpu.returned_to != 0x60a57c7 { return; }
  pc = 0x60a57c7; continue;
 }
 0x60a57c7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a57e4); f_060bc610(cpu); if cpu.returned_to != 0x60a57e4 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60a5832; continue; }
  pc = 0x60a57f3; continue;
 }
 0x60a57f3 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a5805; continue; }
  pc = 0x60a57fc; continue;
 }
 0x60a57fc => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5832; continue; }
  pc = 0x60a5805; continue;
 }
 0x60a5805 => {
  cpu.push(0x60a580a); f_060b3c01(cpu); if cpu.returned_to != 0x60a580a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5832; continue; }
  pc = 0x60a580e; continue;
 }
 0x60a580e => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a5828); f_060b3e93(cpu); if cpu.returned_to != 0x60a5828 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a5832); f_060ad720(cpu); if cpu.returned_to != 0x60a5832 { return; }
  pc = 0x60a5832; continue;
 }
 0x60a5832 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5950); }
 let mut pc: u32 = 0x60a5950;
 loop { match pc {
 0x60a5950 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5997; continue; }
  pc = 0x60a5970; continue;
 }
 0x60a5970 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a597e; continue; }
  pc = 0x60a5975; continue;
 }
 0x60a5975 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5997; continue; }
  pc = 0x60a597e; continue;
 }
 0x60a597e => {
  cpu.push(0x60a5983); f_060b3c01(cpu); if cpu.returned_to != 0x60a5983 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5997; continue; }
  pc = 0x60a5987; continue;
 }
 0x60a5987 => {
  cpu.push(0x60a598c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a598c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5997; continue; }
  pc = 0x60a5990; continue;
 }
 0x60a5990 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5999; continue;
 }
 0x60a5997 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5999; continue;
 }
 0x60a5999 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a59cb; continue; }
  pc = 0x60a599d; continue;
 }
 0x60a599d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a9c0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a59c3); cpu.call(t); if cpu.returned_to != 0x60a59c3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a59cb); f_060ad7c0(cpu); if cpu.returned_to != 0x60a59cb { return; }
  pc = 0x60a59cb; continue;
 }
 0x60a59cb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a59d8); f_060bc6a0(cpu); if cpu.returned_to != 0x60a59d8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5a00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5a00); }
 let mut pc: u32 = 0x60a5a00;
 loop { match pc {
 0x60a5a00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5a48; continue; }
  pc = 0x60a5a21; continue;
 }
 0x60a5a21 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a5a2f; continue; }
  pc = 0x60a5a26; continue;
 }
 0x60a5a26 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5a48; continue; }
  pc = 0x60a5a2f; continue;
 }
 0x60a5a2f => {
  cpu.push(0x60a5a34); f_060b3c01(cpu); if cpu.returned_to != 0x60a5a34 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5a48; continue; }
  pc = 0x60a5a38; continue;
 }
 0x60a5a38 => {
  cpu.push(0x60a5a3d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a5a3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5a48; continue; }
  pc = 0x60a5a41; continue;
 }
 0x60a5a41 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5a4a; continue;
 }
 0x60a5a48 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5a4a; continue;
 }
 0x60a5a4a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5a8a; continue; }
  pc = 0x60a5a4e; continue;
 }
 0x60a5a4e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a9a4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a5a82); cpu.call(t); if cpu.returned_to != 0x60a5a82 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5a8a); f_060ad7c0(cpu); if cpu.returned_to != 0x60a5a8a { return; }
  pc = 0x60a5a8a; continue;
 }
 0x60a5a8a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a5a9f); f_060bc6f0(cpu); if cpu.returned_to != 0x60a5a9f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5aec; continue; }
  pc = 0x60a5aad; continue;
 }
 0x60a5aad => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a5abf; continue; }
  pc = 0x60a5ab6; continue;
 }
 0x60a5ab6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5aec; continue; }
  pc = 0x60a5abf; continue;
 }
 0x60a5abf => {
  cpu.push(0x60a5ac4); f_060b3c01(cpu); if cpu.returned_to != 0x60a5ac4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5aec; continue; }
  pc = 0x60a5ac8; continue;
 }
 0x60a5ac8 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a5ae2); f_060b3e93(cpu); if cpu.returned_to != 0x60a5ae2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a5aec); f_060ad770(cpu); if cpu.returned_to != 0x60a5aec { return; }
  pc = 0x60a5aec; continue;
 }
 0x60a5aec => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a5b10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5b10); }
 let mut pc: u32 = 0x60a5b10;
 loop { match pc {
 0x60a5b10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5b57; continue; }
  pc = 0x60a5b30; continue;
 }
 0x60a5b30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a5b3e; continue; }
  pc = 0x60a5b35; continue;
 }
 0x60a5b35 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5b57; continue; }
  pc = 0x60a5b3e; continue;
 }
 0x60a5b3e => {
  cpu.push(0x60a5b43); f_060b3c01(cpu); if cpu.returned_to != 0x60a5b43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5b57; continue; }
  pc = 0x60a5b47; continue;
 }
 0x60a5b47 => {
  cpu.push(0x60a5b4c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a5b4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5b57; continue; }
  pc = 0x60a5b50; continue;
 }
 0x60a5b50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5b59; continue;
 }
 0x60a5b57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5b59; continue;
 }
 0x60a5b59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5b8b; continue; }
  pc = 0x60a5b5d; continue;
 }
 0x60a5b5d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a988u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a5b83); cpu.call(t); if cpu.returned_to != 0x60a5b83 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5b8b); f_060ad7c0(cpu); if cpu.returned_to != 0x60a5b8b { return; }
  pc = 0x60a5b8b; continue;
 }
 0x60a5b8b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a5b98); f_060bc860(cpu); if cpu.returned_to != 0x60a5b98 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a5bc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5bc0); }
 let mut pc: u32 = 0x60a5bc0;
 loop { match pc {
 0x60a5bc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5c08; continue; }
  pc = 0x60a5be1; continue;
 }
 0x60a5be1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a5bef; continue; }
  pc = 0x60a5be6; continue;
 }
 0x60a5be6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5c08; continue; }
  pc = 0x60a5bef; continue;
 }
 0x60a5bef => {
  cpu.push(0x60a5bf4); f_060b3c01(cpu); if cpu.returned_to != 0x60a5bf4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5c08; continue; }
  pc = 0x60a5bf8; continue;
 }
 0x60a5bf8 => {
  cpu.push(0x60a5bfd); f_060ad3f0(cpu); if cpu.returned_to != 0x60a5bfd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5c08; continue; }
  pc = 0x60a5c01; continue;
 }
 0x60a5c01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5c0a; continue;
 }
 0x60a5c08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5c0a; continue;
 }
 0x60a5c0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5c3c; continue; }
  pc = 0x60a5c0e; continue;
 }
 0x60a5c0e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a970u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a5c34); cpu.call(t); if cpu.returned_to != 0x60a5c34 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5c3c); f_060ad7c0(cpu); if cpu.returned_to != 0x60a5c3c { return; }
  pc = 0x60a5c3c; continue;
 }
 0x60a5c3c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a5c49); f_060bc900(cpu); if cpu.returned_to != 0x60a5c49 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a5c97; continue; }
  pc = 0x60a5c58; continue;
 }
 0x60a5c58 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a5c6a; continue; }
  pc = 0x60a5c61; continue;
 }
 0x60a5c61 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5c97; continue; }
  pc = 0x60a5c6a; continue;
 }
 0x60a5c6a => {
  cpu.push(0x60a5c6f); f_060b3c01(cpu); if cpu.returned_to != 0x60a5c6f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5c97; continue; }
  pc = 0x60a5c73; continue;
 }
 0x60a5c73 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a5c8d); f_060b3e93(cpu); if cpu.returned_to != 0x60a5c8d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a5c97); f_060ad720(cpu); if cpu.returned_to != 0x60a5c97 { return; }
  pc = 0x60a5c97; continue;
 }
 0x60a5c97 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a5e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5e40); }
 let mut pc: u32 = 0x60a5e40;
 loop { match pc {
 0x60a5e40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5e88; continue; }
  pc = 0x60a5e61; continue;
 }
 0x60a5e61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a5e6f; continue; }
  pc = 0x60a5e66; continue;
 }
 0x60a5e66 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5e88; continue; }
  pc = 0x60a5e6f; continue;
 }
 0x60a5e6f => {
  cpu.push(0x60a5e74); f_060b3c01(cpu); if cpu.returned_to != 0x60a5e74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5e88; continue; }
  pc = 0x60a5e78; continue;
 }
 0x60a5e78 => {
  cpu.push(0x60a5e7d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a5e7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5e88; continue; }
  pc = 0x60a5e81; continue;
 }
 0x60a5e81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5e8a; continue;
 }
 0x60a5e88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5e8a; continue;
 }
 0x60a5e8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5ebc; continue; }
  pc = 0x60a5e8e; continue;
 }
 0x60a5e8e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a920u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a5eb4); cpu.call(t); if cpu.returned_to != 0x60a5eb4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5ebc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a5ebc { return; }
  pc = 0x60a5ebc; continue;
 }
 0x60a5ebc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a5ec9); f_060bb8f0(cpu); if cpu.returned_to != 0x60a5ec9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a5f17; continue; }
  pc = 0x60a5ed8; continue;
 }
 0x60a5ed8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a5eea; continue; }
  pc = 0x60a5ee1; continue;
 }
 0x60a5ee1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5f17; continue; }
  pc = 0x60a5eea; continue;
 }
 0x60a5eea => {
  cpu.push(0x60a5eef); f_060b3c01(cpu); if cpu.returned_to != 0x60a5eef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5f17; continue; }
  pc = 0x60a5ef3; continue;
 }
 0x60a5ef3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a5f0d); f_060b3e93(cpu); if cpu.returned_to != 0x60a5f0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a5f17); f_060ad770(cpu); if cpu.returned_to != 0x60a5f17 { return; }
  pc = 0x60a5f17; continue;
 }
 0x60a5f17 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a5f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a5f40); }
 let mut pc: u32 = 0x60a5f40;
 loop { match pc {
 0x60a5f40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a5f88; continue; }
  pc = 0x60a5f61; continue;
 }
 0x60a5f61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a5f6f; continue; }
  pc = 0x60a5f66; continue;
 }
 0x60a5f66 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a5f88; continue; }
  pc = 0x60a5f6f; continue;
 }
 0x60a5f6f => {
  cpu.push(0x60a5f74); f_060b3c01(cpu); if cpu.returned_to != 0x60a5f74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a5f88; continue; }
  pc = 0x60a5f78; continue;
 }
 0x60a5f78 => {
  cpu.push(0x60a5f7d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a5f7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5f88; continue; }
  pc = 0x60a5f81; continue;
 }
 0x60a5f81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a5f8a; continue;
 }
 0x60a5f88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a5f8a; continue;
 }
 0x60a5f8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a5fbc; continue; }
  pc = 0x60a5f8e; continue;
 }
 0x60a5f8e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a904u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a5fb4); cpu.call(t); if cpu.returned_to != 0x60a5fb4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a5fbc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a5fbc { return; }
  pc = 0x60a5fbc; continue;
 }
 0x60a5fbc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a5fc9); f_060bb940(cpu); if cpu.returned_to != 0x60a5fc9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a6017; continue; }
  pc = 0x60a5fd8; continue;
 }
 0x60a5fd8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a5fea; continue; }
  pc = 0x60a5fe1; continue;
 }
 0x60a5fe1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6017; continue; }
  pc = 0x60a5fea; continue;
 }
 0x60a5fea => {
  cpu.push(0x60a5fef); f_060b3c01(cpu); if cpu.returned_to != 0x60a5fef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6017; continue; }
  pc = 0x60a5ff3; continue;
 }
 0x60a5ff3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a600d); f_060b3e93(cpu); if cpu.returned_to != 0x60a600d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6017); f_060ad770(cpu); if cpu.returned_to != 0x60a6017 { return; }
  pc = 0x60a6017; continue;
 }
 0x60a6017 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a6040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6040); }
 let mut pc: u32 = 0x60a6040;
 loop { match pc {
 0x60a6040 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a6087; continue; }
  pc = 0x60a6060; continue;
 }
 0x60a6060 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a606e; continue; }
  pc = 0x60a6065; continue;
 }
 0x60a6065 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6087; continue; }
  pc = 0x60a606e; continue;
 }
 0x60a606e => {
  cpu.push(0x60a6073); f_060b3c01(cpu); if cpu.returned_to != 0x60a6073 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6087; continue; }
  pc = 0x60a6077; continue;
 }
 0x60a6077 => {
  cpu.push(0x60a607c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a607c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6087; continue; }
  pc = 0x60a6080; continue;
 }
 0x60a6080 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a6089; continue;
 }
 0x60a6087 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a6089; continue;
 }
 0x60a6089 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a60d7; continue; }
  pc = 0x60a608d; continue;
 }
 0x60a608d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a8e0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a60cf); cpu.call(t); if cpu.returned_to != 0x60a60cf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a60d7); f_060ad7c0(cpu); if cpu.returned_to != 0x60a60d7 { return; }
  pc = 0x60a60d7; continue;
 }
 0x60a60d7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a60f4); f_060bb9a0(cpu); if cpu.returned_to != 0x60a60f4 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60a6142; continue; }
  pc = 0x60a6103; continue;
 }
 0x60a6103 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a6115; continue; }
  pc = 0x60a610c; continue;
 }
 0x60a610c => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6142; continue; }
  pc = 0x60a6115; continue;
 }
 0x60a6115 => {
  cpu.push(0x60a611a); f_060b3c01(cpu); if cpu.returned_to != 0x60a611a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6142; continue; }
  pc = 0x60a611e; continue;
 }
 0x60a611e => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a6138); f_060b3e93(cpu); if cpu.returned_to != 0x60a6138 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6142); f_060ad770(cpu); if cpu.returned_to != 0x60a6142 { return; }
  pc = 0x60a6142; continue;
 }
 0x60a6142 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a6160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6160); }
 let mut pc: u32 = 0x60a6160;
 loop { match pc {
 0x60a6160 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a61a8; continue; }
  pc = 0x60a6181; continue;
 }
 0x60a6181 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a618f; continue; }
  pc = 0x60a6186; continue;
 }
 0x60a6186 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a61a8; continue; }
  pc = 0x60a618f; continue;
 }
 0x60a618f => {
  cpu.push(0x60a6194); f_060b3c01(cpu); if cpu.returned_to != 0x60a6194 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a61a8; continue; }
  pc = 0x60a6198; continue;
 }
 0x60a6198 => {
  cpu.push(0x60a619d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a619d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a61a8; continue; }
  pc = 0x60a61a1; continue;
 }
 0x60a61a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a61aa; continue;
 }
 0x60a61a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a61aa; continue;
 }
 0x60a61aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a61dc; continue; }
  pc = 0x60a61ae; continue;
 }
 0x60a61ae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a8c4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a61d4); cpu.call(t); if cpu.returned_to != 0x60a61d4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a61dc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a61dc { return; }
  pc = 0x60a61dc; continue;
 }
 0x60a61dc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a61e9); f_060bba30(cpu); if cpu.returned_to != 0x60a61e9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a6237; continue; }
  pc = 0x60a61f8; continue;
 }
 0x60a61f8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a620a; continue; }
  pc = 0x60a6201; continue;
 }
 0x60a6201 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6237; continue; }
  pc = 0x60a620a; continue;
 }
 0x60a620a => {
  cpu.push(0x60a620f); f_060b3c01(cpu); if cpu.returned_to != 0x60a620f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6237; continue; }
  pc = 0x60a6213; continue;
 }
 0x60a6213 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a622d); f_060b3e93(cpu); if cpu.returned_to != 0x60a622d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6237); f_060ad770(cpu); if cpu.returned_to != 0x60a6237 { return; }
  pc = 0x60a6237; continue;
 }
 0x60a6237 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a6260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6260); }
 let mut pc: u32 = 0x60a6260;
 loop { match pc {
 0x60a6260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a62a8; continue; }
  pc = 0x60a6281; continue;
 }
 0x60a6281 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a628f; continue; }
  pc = 0x60a6286; continue;
 }
 0x60a6286 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a62a8; continue; }
  pc = 0x60a628f; continue;
 }
 0x60a628f => {
  cpu.push(0x60a6294); f_060b3c01(cpu); if cpu.returned_to != 0x60a6294 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a62a8; continue; }
  pc = 0x60a6298; continue;
 }
 0x60a6298 => {
  cpu.push(0x60a629d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a629d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a62a8; continue; }
  pc = 0x60a62a1; continue;
 }
 0x60a62a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a62aa; continue;
 }
 0x60a62a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a62aa; continue;
 }
 0x60a62aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a62dc; continue; }
  pc = 0x60a62ae; continue;
 }
 0x60a62ae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a8a4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a62d4); cpu.call(t); if cpu.returned_to != 0x60a62d4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a62dc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a62dc { return; }
  pc = 0x60a62dc; continue;
 }
 0x60a62dc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a62e9); f_060bba60(cpu); if cpu.returned_to != 0x60a62e9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a6337; continue; }
  pc = 0x60a62f8; continue;
 }
 0x60a62f8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a630a; continue; }
  pc = 0x60a6301; continue;
 }
 0x60a6301 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6337; continue; }
  pc = 0x60a630a; continue;
 }
 0x60a630a => {
  cpu.push(0x60a630f); f_060b3c01(cpu); if cpu.returned_to != 0x60a630f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6337; continue; }
  pc = 0x60a6313; continue;
 }
 0x60a6313 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a632d); f_060b3e93(cpu); if cpu.returned_to != 0x60a632d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6337); f_060ad770(cpu); if cpu.returned_to != 0x60a6337 { return; }
  pc = 0x60a6337; continue;
 }
 0x60a6337 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a6460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6460); }
 let mut pc: u32 = 0x60a6460;
 loop { match pc {
 0x60a6460 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a64a7; continue; }
  pc = 0x60a6480; continue;
 }
 0x60a6480 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a648e; continue; }
  pc = 0x60a6485; continue;
 }
 0x60a6485 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a64a7; continue; }
  pc = 0x60a648e; continue;
 }
 0x60a648e => {
  cpu.push(0x60a6493); f_060b3c01(cpu); if cpu.returned_to != 0x60a6493 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a64a7; continue; }
  pc = 0x60a6497; continue;
 }
 0x60a6497 => {
  cpu.push(0x60a649c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a649c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a64a7; continue; }
  pc = 0x60a64a0; continue;
 }
 0x60a64a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a64a9; continue;
 }
 0x60a64a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a64a9; continue;
 }
 0x60a64a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6505; continue; }
  pc = 0x60a64ad; continue;
 }
 0x60a64ad => {
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
  { let v = 0x616a858u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a64fd); cpu.call(t); if cpu.returned_to != 0x60a64fd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a6505); f_060ad7c0(cpu); if cpu.returned_to != 0x60a6505 { return; }
  pc = 0x60a6505; continue;
 }
 0x60a6505 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60a652a); f_060bbb40(cpu); if cpu.returned_to != 0x60a652a { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a6550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6550); }
 let mut pc: u32 = 0x60a6550;
 loop { match pc {
 0x60a6550 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a6598; continue; }
  pc = 0x60a6571; continue;
 }
 0x60a6571 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a657f; continue; }
  pc = 0x60a6576; continue;
 }
 0x60a6576 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6598; continue; }
  pc = 0x60a657f; continue;
 }
 0x60a657f => {
  cpu.push(0x60a6584); f_060b3c01(cpu); if cpu.returned_to != 0x60a6584 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6598; continue; }
  pc = 0x60a6588; continue;
 }
 0x60a6588 => {
  cpu.push(0x60a658d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a658d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6598; continue; }
  pc = 0x60a6591; continue;
 }
 0x60a6591 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a659a; continue;
 }
 0x60a6598 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a659a; continue;
 }
 0x60a659a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a65cc; continue; }
  pc = 0x60a659e; continue;
 }
 0x60a659e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a83cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a65c4); cpu.call(t); if cpu.returned_to != 0x60a65c4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a65cc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a65cc { return; }
  pc = 0x60a65cc; continue;
 }
 0x60a65cc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a65d9); f_060bbba0(cpu); if cpu.returned_to != 0x60a65d9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a6627; continue; }
  pc = 0x60a65e8; continue;
 }
 0x60a65e8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a65fa; continue; }
  pc = 0x60a65f1; continue;
 }
 0x60a65f1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6627; continue; }
  pc = 0x60a65fa; continue;
 }
 0x60a65fa => {
  cpu.push(0x60a65ff); f_060b3c01(cpu); if cpu.returned_to != 0x60a65ff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6627; continue; }
  pc = 0x60a6603; continue;
 }
 0x60a6603 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a661d); f_060b3e93(cpu); if cpu.returned_to != 0x60a661d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6627); f_060ad770(cpu); if cpu.returned_to != 0x60a6627 { return; }
  pc = 0x60a6627; continue;
 }
 0x60a6627 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a6650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6650); }
 let mut pc: u32 = 0x60a6650;
 loop { match pc {
 0x60a6650 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a6698; continue; }
  pc = 0x60a6671; continue;
 }
 0x60a6671 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a667f; continue; }
  pc = 0x60a6676; continue;
 }
 0x60a6676 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6698; continue; }
  pc = 0x60a667f; continue;
 }
 0x60a667f => {
  cpu.push(0x60a6684); f_060b3c01(cpu); if cpu.returned_to != 0x60a6684 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6698; continue; }
  pc = 0x60a6688; continue;
 }
 0x60a6688 => {
  cpu.push(0x60a668d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a668d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6698; continue; }
  pc = 0x60a6691; continue;
 }
 0x60a6691 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a669a; continue;
 }
 0x60a6698 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a669a; continue;
 }
 0x60a669a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a66cc; continue; }
  pc = 0x60a669e; continue;
 }
 0x60a669e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a820u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a66c4); cpu.call(t); if cpu.returned_to != 0x60a66c4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a66cc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a66cc { return; }
  pc = 0x60a66cc; continue;
 }
 0x60a66cc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a66d9); f_060bbbc0(cpu); if cpu.returned_to != 0x60a66d9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a6727; continue; }
  pc = 0x60a66e8; continue;
 }
 0x60a66e8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a66fa; continue; }
  pc = 0x60a66f1; continue;
 }
 0x60a66f1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6727; continue; }
  pc = 0x60a66fa; continue;
 }
 0x60a66fa => {
  cpu.push(0x60a66ff); f_060b3c01(cpu); if cpu.returned_to != 0x60a66ff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6727; continue; }
  pc = 0x60a6703; continue;
 }
 0x60a6703 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a671d); f_060b3e93(cpu); if cpu.returned_to != 0x60a671d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6727); f_060ad770(cpu); if cpu.returned_to != 0x60a6727 { return; }
  pc = 0x60a6727; continue;
 }
 0x60a6727 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a6750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6750); }
 let mut pc: u32 = 0x60a6750;
 loop { match pc {
 0x60a6750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a6797; continue; }
  pc = 0x60a6770; continue;
 }
 0x60a6770 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a677e; continue; }
  pc = 0x60a6775; continue;
 }
 0x60a6775 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6797; continue; }
  pc = 0x60a677e; continue;
 }
 0x60a677e => {
  cpu.push(0x60a6783); f_060b3c01(cpu); if cpu.returned_to != 0x60a6783 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6797; continue; }
  pc = 0x60a6787; continue;
 }
 0x60a6787 => {
  cpu.push(0x60a678c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a678c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6797; continue; }
  pc = 0x60a6790; continue;
 }
 0x60a6790 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a6799; continue;
 }
 0x60a6797 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a6799; continue;
 }
 0x60a6799 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a67cb; continue; }
  pc = 0x60a679d; continue;
 }
 0x60a679d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a800u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a67c3); cpu.call(t); if cpu.returned_to != 0x60a67c3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a67cb); f_060ad7c0(cpu); if cpu.returned_to != 0x60a67cb { return; }
  pc = 0x60a67cb; continue;
 }
 0x60a67cb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a67d8); f_060bbc20(cpu); if cpu.returned_to != 0x60a67d8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a68b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a68b0); }
 let mut pc: u32 = 0x60a68b0;
 loop { match pc {
 0x60a68b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a68f7; continue; }
  pc = 0x60a68d0; continue;
 }
 0x60a68d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a68de; continue; }
  pc = 0x60a68d5; continue;
 }
 0x60a68d5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a68f7; continue; }
  pc = 0x60a68de; continue;
 }
 0x60a68de => {
  cpu.push(0x60a68e3); f_060b3c01(cpu); if cpu.returned_to != 0x60a68e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a68f7; continue; }
  pc = 0x60a68e7; continue;
 }
 0x60a68e7 => {
  cpu.push(0x60a68ec); f_060ad3f0(cpu); if cpu.returned_to != 0x60a68ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a68f7; continue; }
  pc = 0x60a68f0; continue;
 }
 0x60a68f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a68f9; continue;
 }
 0x60a68f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a68f9; continue;
 }
 0x60a68f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a692b; continue; }
  pc = 0x60a68fd; continue;
 }
 0x60a68fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a7c8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a6923); cpu.call(t); if cpu.returned_to != 0x60a6923 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a692b); f_060ad7c0(cpu); if cpu.returned_to != 0x60a692b { return; }
  pc = 0x60a692b; continue;
 }
 0x60a692b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a6938); f_060bbc60(cpu); if cpu.returned_to != 0x60a6938 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a6960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6960); }
 let mut pc: u32 = 0x60a6960;
 loop { match pc {
 0x60a6960 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a69a8; continue; }
  pc = 0x60a6981; continue;
 }
 0x60a6981 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a698f; continue; }
  pc = 0x60a6986; continue;
 }
 0x60a6986 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a69a8; continue; }
  pc = 0x60a698f; continue;
 }
 0x60a698f => {
  cpu.push(0x60a6994); f_060b3c01(cpu); if cpu.returned_to != 0x60a6994 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a69a8; continue; }
  pc = 0x60a6998; continue;
 }
 0x60a6998 => {
  cpu.push(0x60a699d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a699d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a69a8; continue; }
  pc = 0x60a69a1; continue;
 }
 0x60a69a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a69aa; continue;
 }
 0x60a69a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a69aa; continue;
 }
 0x60a69aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a69dc; continue; }
  pc = 0x60a69ae; continue;
 }
 0x60a69ae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a7b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a69d4); cpu.call(t); if cpu.returned_to != 0x60a69d4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a69dc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a69dc { return; }
  pc = 0x60a69dc; continue;
 }
 0x60a69dc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a69e9); f_060bbcb0(cpu); if cpu.returned_to != 0x60a69e9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a6a37; continue; }
  pc = 0x60a69f8; continue;
 }
 0x60a69f8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a6a0a; continue; }
  pc = 0x60a6a01; continue;
 }
 0x60a6a01 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6a37; continue; }
  pc = 0x60a6a0a; continue;
 }
 0x60a6a0a => {
  cpu.push(0x60a6a0f); f_060b3c01(cpu); if cpu.returned_to != 0x60a6a0f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6a37; continue; }
  pc = 0x60a6a13; continue;
 }
 0x60a6a13 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a6a2d); f_060b3e93(cpu); if cpu.returned_to != 0x60a6a2d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a6a37); f_060ad720(cpu); if cpu.returned_to != 0x60a6a37 { return; }
  pc = 0x60a6a37; continue;
 }
 0x60a6a37 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a6a60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6a60); }
 let mut pc: u32 = 0x60a6a60;
 loop { match pc {
 0x60a6a60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a6aa7; continue; }
  pc = 0x60a6a80; continue;
 }
 0x60a6a80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a6a8e; continue; }
  pc = 0x60a6a85; continue;
 }
 0x60a6a85 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6aa7; continue; }
  pc = 0x60a6a8e; continue;
 }
 0x60a6a8e => {
  cpu.push(0x60a6a93); f_060b3c01(cpu); if cpu.returned_to != 0x60a6a93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6aa7; continue; }
  pc = 0x60a6a97; continue;
 }
 0x60a6a97 => {
  cpu.push(0x60a6a9c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a6a9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6aa7; continue; }
  pc = 0x60a6aa0; continue;
 }
 0x60a6aa0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a6aa9; continue;
 }
 0x60a6aa7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a6aa9; continue;
 }
 0x60a6aa9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6adb; continue; }
  pc = 0x60a6aad; continue;
 }
 0x60a6aad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a788u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a6ad3); cpu.call(t); if cpu.returned_to != 0x60a6ad3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a6adb); f_060ad7c0(cpu); if cpu.returned_to != 0x60a6adb { return; }
  pc = 0x60a6adb; continue;
 }
 0x60a6adb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a6ae8); f_060bae40(cpu); if cpu.returned_to != 0x60a6ae8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a6f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6f10); }
 let mut pc: u32 = 0x60a6f10;
 loop { match pc {
 0x60a6f10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a6f57; continue; }
  pc = 0x60a6f30; continue;
 }
 0x60a6f30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a6f3e; continue; }
  pc = 0x60a6f35; continue;
 }
 0x60a6f35 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a6f57; continue; }
  pc = 0x60a6f3e; continue;
 }
 0x60a6f3e => {
  cpu.push(0x60a6f43); f_060b3c01(cpu); if cpu.returned_to != 0x60a6f43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a6f57; continue; }
  pc = 0x60a6f47; continue;
 }
 0x60a6f47 => {
  cpu.push(0x60a6f4c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a6f4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6f57; continue; }
  pc = 0x60a6f50; continue;
 }
 0x60a6f50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a6f59; continue;
 }
 0x60a6f57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a6f59; continue;
 }
 0x60a6f59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a6fa7; continue; }
  pc = 0x60a6f5d; continue;
 }
 0x60a6f5d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a6a4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a6f9f); cpu.call(t); if cpu.returned_to != 0x60a6f9f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a6fa7); f_060ad7c0(cpu); if cpu.returned_to != 0x60a6fa7 { return; }
  pc = 0x60a6fa7; continue;
 }
 0x60a6fa7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a6fc4); f_060b75d0(cpu); if cpu.returned_to != 0x60a6fc4 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a6fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a6fe0); }
 let mut pc: u32 = 0x60a6fe0;
 loop { match pc {
 0x60a6fe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a7027; continue; }
  pc = 0x60a7000; continue;
 }
 0x60a7000 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a700e; continue; }
  pc = 0x60a7005; continue;
 }
 0x60a7005 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7027; continue; }
  pc = 0x60a700e; continue;
 }
 0x60a700e => {
  cpu.push(0x60a7013); f_060b3c01(cpu); if cpu.returned_to != 0x60a7013 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7027; continue; }
  pc = 0x60a7017; continue;
 }
 0x60a7017 => {
  cpu.push(0x60a701c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a701c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7027; continue; }
  pc = 0x60a7020; continue;
 }
 0x60a7020 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a7029; continue;
 }
 0x60a7027 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a7029; continue;
 }
 0x60a7029 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7069; continue; }
  pc = 0x60a702d; continue;
 }
 0x60a702d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a684u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7061); cpu.call(t); if cpu.returned_to != 0x60a7061 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7069); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7069 { return; }
  pc = 0x60a7069; continue;
 }
 0x60a7069 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a707e); f_060b7610(cpu); if cpu.returned_to != 0x60a707e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a70a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a70a0); }
 let mut pc: u32 = 0x60a70a0;
 loop { match pc {
 0x60a70a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a70e8; continue; }
  pc = 0x60a70c1; continue;
 }
 0x60a70c1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a70cf; continue; }
  pc = 0x60a70c6; continue;
 }
 0x60a70c6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a70e8; continue; }
  pc = 0x60a70cf; continue;
 }
 0x60a70cf => {
  cpu.push(0x60a70d4); f_060b3c01(cpu); if cpu.returned_to != 0x60a70d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a70e8; continue; }
  pc = 0x60a70d8; continue;
 }
 0x60a70d8 => {
  cpu.push(0x60a70dd); f_060ad3f0(cpu); if cpu.returned_to != 0x60a70dd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a70e8; continue; }
  pc = 0x60a70e1; continue;
 }
 0x60a70e1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a70ea; continue;
 }
 0x60a70e8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a70ea; continue;
 }
 0x60a70ea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a711c; continue; }
  pc = 0x60a70ee; continue;
 }
 0x60a70ee => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a668u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7114); cpu.call(t); if cpu.returned_to != 0x60a7114 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a711c); f_060ad7c0(cpu); if cpu.returned_to != 0x60a711c { return; }
  pc = 0x60a711c; continue;
 }
 0x60a711c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a7129); f_060b7730(cpu); if cpu.returned_to != 0x60a7129 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a7177; continue; }
  pc = 0x60a7138; continue;
 }
 0x60a7138 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a714a; continue; }
  pc = 0x60a7141; continue;
 }
 0x60a7141 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7177; continue; }
  pc = 0x60a714a; continue;
 }
 0x60a714a => {
  cpu.push(0x60a714f); f_060b3c01(cpu); if cpu.returned_to != 0x60a714f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7177; continue; }
  pc = 0x60a7153; continue;
 }
 0x60a7153 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a716d); f_060b3e93(cpu); if cpu.returned_to != 0x60a716d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a7177); f_060ad770(cpu); if cpu.returned_to != 0x60a7177 { return; }
  pc = 0x60a7177; continue;
 }
 0x60a7177 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a71a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a71a0); }
 let mut pc: u32 = 0x60a71a0;
 loop { match pc {
 0x60a71a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a71e7; continue; }
  pc = 0x60a71c0; continue;
 }
 0x60a71c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a71ce; continue; }
  pc = 0x60a71c5; continue;
 }
 0x60a71c5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a71e7; continue; }
  pc = 0x60a71ce; continue;
 }
 0x60a71ce => {
  cpu.push(0x60a71d3); f_060b3c01(cpu); if cpu.returned_to != 0x60a71d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a71e7; continue; }
  pc = 0x60a71d7; continue;
 }
 0x60a71d7 => {
  cpu.push(0x60a71dc); f_060ad3f0(cpu); if cpu.returned_to != 0x60a71dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a71e7; continue; }
  pc = 0x60a71e0; continue;
 }
 0x60a71e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a71e9; continue;
 }
 0x60a71e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a71e9; continue;
 }
 0x60a71e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7229; continue; }
  pc = 0x60a71ed; continue;
 }
 0x60a71ed => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a644u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7221); cpu.call(t); if cpu.returned_to != 0x60a7221 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7229); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7229 { return; }
  pc = 0x60a7229; continue;
 }
 0x60a7229 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a723e); f_060b78b0(cpu); if cpu.returned_to != 0x60a723e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a7260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7260); }
 let mut pc: u32 = 0x60a7260;
 loop { match pc {
 0x60a7260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a72a8; continue; }
  pc = 0x60a7281; continue;
 }
 0x60a7281 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a728f; continue; }
  pc = 0x60a7286; continue;
 }
 0x60a7286 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a72a8; continue; }
  pc = 0x60a728f; continue;
 }
 0x60a728f => {
  cpu.push(0x60a7294); f_060b3c01(cpu); if cpu.returned_to != 0x60a7294 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a72a8; continue; }
  pc = 0x60a7298; continue;
 }
 0x60a7298 => {
  cpu.push(0x60a729d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a729d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a72a8; continue; }
  pc = 0x60a72a1; continue;
 }
 0x60a72a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a72aa; continue;
 }
 0x60a72a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a72aa; continue;
 }
 0x60a72aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a72ea; continue; }
  pc = 0x60a72ae; continue;
 }
 0x60a72ae => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a620u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a72e2); cpu.call(t); if cpu.returned_to != 0x60a72e2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a72ea); f_060ad7c0(cpu); if cpu.returned_to != 0x60a72ea { return; }
  pc = 0x60a72ea; continue;
 }
 0x60a72ea => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a72ff); f_060b78c0(cpu); if cpu.returned_to != 0x60a72ff { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a734c; continue; }
  pc = 0x60a730d; continue;
 }
 0x60a730d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a731f; continue; }
  pc = 0x60a7316; continue;
 }
 0x60a7316 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a734c; continue; }
  pc = 0x60a731f; continue;
 }
 0x60a731f => {
  cpu.push(0x60a7324); f_060b3c01(cpu); if cpu.returned_to != 0x60a7324 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a734c; continue; }
  pc = 0x60a7328; continue;
 }
 0x60a7328 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a7342); f_060b3e93(cpu); if cpu.returned_to != 0x60a7342 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a734c); f_060ad770(cpu); if cpu.returned_to != 0x60a734c { return; }
  pc = 0x60a734c; continue;
 }
 0x60a734c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a7370(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7370); }
 let mut pc: u32 = 0x60a7370;
 loop { match pc {
 0x60a7370 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a73b7; continue; }
  pc = 0x60a7390; continue;
 }
 0x60a7390 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a739e; continue; }
  pc = 0x60a7395; continue;
 }
 0x60a7395 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a73b7; continue; }
  pc = 0x60a739e; continue;
 }
 0x60a739e => {
  cpu.push(0x60a73a3); f_060b3c01(cpu); if cpu.returned_to != 0x60a73a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a73b7; continue; }
  pc = 0x60a73a7; continue;
 }
 0x60a73a7 => {
  cpu.push(0x60a73ac); f_060ad3f0(cpu); if cpu.returned_to != 0x60a73ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a73b7; continue; }
  pc = 0x60a73b0; continue;
 }
 0x60a73b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a73b9; continue;
 }
 0x60a73b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a73b9; continue;
 }
 0x60a73b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7407; continue; }
  pc = 0x60a73bd; continue;
 }
 0x60a73bd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a5f4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a73ff); cpu.call(t); if cpu.returned_to != 0x60a73ff { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7407); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7407 { return; }
  pc = 0x60a7407; continue;
 }
 0x60a7407 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7424); f_060b78e0(cpu); if cpu.returned_to != 0x60a7424 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a7440(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7440); }
 let mut pc: u32 = 0x60a7440;
 loop { match pc {
 0x60a7440 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a7488; continue; }
  pc = 0x60a7461; continue;
 }
 0x60a7461 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a746f; continue; }
  pc = 0x60a7466; continue;
 }
 0x60a7466 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7488; continue; }
  pc = 0x60a746f; continue;
 }
 0x60a746f => {
  cpu.push(0x60a7474); f_060b3c01(cpu); if cpu.returned_to != 0x60a7474 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7488; continue; }
  pc = 0x60a7478; continue;
 }
 0x60a7478 => {
  cpu.push(0x60a747d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a747d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7488; continue; }
  pc = 0x60a7481; continue;
 }
 0x60a7481 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a748a; continue;
 }
 0x60a7488 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a748a; continue;
 }
 0x60a748a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a74ca; continue; }
  pc = 0x60a748e; continue;
 }
 0x60a748e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a5c8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a74c2); cpu.call(t); if cpu.returned_to != 0x60a74c2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a74ca); f_060ad7c0(cpu); if cpu.returned_to != 0x60a74ca { return; }
  pc = 0x60a74ca; continue;
 }
 0x60a74ca => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a74df); f_060b7900(cpu); if cpu.returned_to != 0x60a74df { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a752c; continue; }
  pc = 0x60a74ed; continue;
 }
 0x60a74ed => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a74ff; continue; }
  pc = 0x60a74f6; continue;
 }
 0x60a74f6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a752c; continue; }
  pc = 0x60a74ff; continue;
 }
 0x60a74ff => {
  cpu.push(0x60a7504); f_060b3c01(cpu); if cpu.returned_to != 0x60a7504 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a752c; continue; }
  pc = 0x60a7508; continue;
 }
 0x60a7508 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a7522); f_060b3e93(cpu); if cpu.returned_to != 0x60a7522 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a752c); f_060ad720(cpu); if cpu.returned_to != 0x60a752c { return; }
  pc = 0x60a752c; continue;
 }
 0x60a752c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a7550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7550); }
 let mut pc: u32 = 0x60a7550;
 loop { match pc {
 0x60a7550 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a7598; continue; }
  pc = 0x60a7571; continue;
 }
 0x60a7571 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a757f; continue; }
  pc = 0x60a7576; continue;
 }
 0x60a7576 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7598; continue; }
  pc = 0x60a757f; continue;
 }
 0x60a757f => {
  cpu.push(0x60a7584); f_060b3c01(cpu); if cpu.returned_to != 0x60a7584 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7598; continue; }
  pc = 0x60a7588; continue;
 }
 0x60a7588 => {
  cpu.push(0x60a758d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a758d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7598; continue; }
  pc = 0x60a7591; continue;
 }
 0x60a7591 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a759a; continue;
 }
 0x60a7598 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a759a; continue;
 }
 0x60a759a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a75da; continue; }
  pc = 0x60a759e; continue;
 }
 0x60a759e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a59cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a75d2); cpu.call(t); if cpu.returned_to != 0x60a75d2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a75da); f_060ad7c0(cpu); if cpu.returned_to != 0x60a75da { return; }
  pc = 0x60a75da; continue;
 }
 0x60a75da => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a75ef); f_060b7920(cpu); if cpu.returned_to != 0x60a75ef { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a763c; continue; }
  pc = 0x60a75fd; continue;
 }
 0x60a75fd => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a760f; continue; }
  pc = 0x60a7606; continue;
 }
 0x60a7606 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a763c; continue; }
  pc = 0x60a760f; continue;
 }
 0x60a760f => {
  cpu.push(0x60a7614); f_060b3c01(cpu); if cpu.returned_to != 0x60a7614 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a763c; continue; }
  pc = 0x60a7618; continue;
 }
 0x60a7618 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a7632); f_060b3e93(cpu); if cpu.returned_to != 0x60a7632 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a763c); f_060ad720(cpu); if cpu.returned_to != 0x60a763c { return; }
  pc = 0x60a763c; continue;
 }
 0x60a763c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a7770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7770); }
 let mut pc: u32 = 0x60a7770;
 loop { match pc {
 0x60a7770 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a77b8; continue; }
  pc = 0x60a7791; continue;
 }
 0x60a7791 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a779f; continue; }
  pc = 0x60a7796; continue;
 }
 0x60a7796 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a77b8; continue; }
  pc = 0x60a779f; continue;
 }
 0x60a779f => {
  cpu.push(0x60a77a4); f_060b3c01(cpu); if cpu.returned_to != 0x60a77a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a77b8; continue; }
  pc = 0x60a77a8; continue;
 }
 0x60a77a8 => {
  cpu.push(0x60a77ad); f_060ad3f0(cpu); if cpu.returned_to != 0x60a77ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a77b8; continue; }
  pc = 0x60a77b1; continue;
 }
 0x60a77b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a77ba; continue;
 }
 0x60a77b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a77ba; continue;
 }
 0x60a77ba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a77fa; continue; }
  pc = 0x60a77be; continue;
 }
 0x60a77be => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a548u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a77f2); cpu.call(t); if cpu.returned_to != 0x60a77f2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a77fa); f_060ad7c0(cpu); if cpu.returned_to != 0x60a77fa { return; }
  pc = 0x60a77fa; continue;
 }
 0x60a77fa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a780f); f_060b7960(cpu); if cpu.returned_to != 0x60a780f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a785c; continue; }
  pc = 0x60a781d; continue;
 }
 0x60a781d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a782f; continue; }
  pc = 0x60a7826; continue;
 }
 0x60a7826 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a785c; continue; }
  pc = 0x60a782f; continue;
 }
 0x60a782f => {
  cpu.push(0x60a7834); f_060b3c01(cpu); if cpu.returned_to != 0x60a7834 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a785c; continue; }
  pc = 0x60a7838; continue;
 }
 0x60a7838 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a7852); f_060b3e93(cpu); if cpu.returned_to != 0x60a7852 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a785c); f_060ad720(cpu); if cpu.returned_to != 0x60a785c { return; }
  pc = 0x60a785c; continue;
 }
 0x60a785c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a7aa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7aa0); }
 let mut pc: u32 = 0x60a7aa0;
 loop { match pc {
 0x60a7aa0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a7ae7; continue; }
  pc = 0x60a7ac0; continue;
 }
 0x60a7ac0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a7ace; continue; }
  pc = 0x60a7ac5; continue;
 }
 0x60a7ac5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7ae7; continue; }
  pc = 0x60a7ace; continue;
 }
 0x60a7ace => {
  cpu.push(0x60a7ad3); f_060b3c01(cpu); if cpu.returned_to != 0x60a7ad3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7ae7; continue; }
  pc = 0x60a7ad7; continue;
 }
 0x60a7ad7 => {
  cpu.push(0x60a7adc); f_060ad3f0(cpu); if cpu.returned_to != 0x60a7adc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7ae7; continue; }
  pc = 0x60a7ae0; continue;
 }
 0x60a7ae0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a7ae9; continue;
 }
 0x60a7ae7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a7ae9; continue;
 }
 0x60a7ae9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7b29; continue; }
  pc = 0x60a7aed; continue;
 }
 0x60a7aed => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a4d4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7b21); cpu.call(t); if cpu.returned_to != 0x60a7b21 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7b29); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7b29 { return; }
  pc = 0x60a7b29; continue;
 }
 0x60a7b29 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a7b3e); f_060b79c0(cpu); if cpu.returned_to != 0x60a7b3e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a7b60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7b60); }
 let mut pc: u32 = 0x60a7b60;
 loop { match pc {
 0x60a7b60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x114u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a7bb6; continue; }
  pc = 0x60a7b8f; continue;
 }
 0x60a7b8f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a7b9d; continue; }
  pc = 0x60a7b94; continue;
 }
 0x60a7b94 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7bb6; continue; }
  pc = 0x60a7b9d; continue;
 }
 0x60a7b9d => {
  cpu.push(0x60a7ba2); f_060b3c01(cpu); if cpu.returned_to != 0x60a7ba2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7bb6; continue; }
  pc = 0x60a7ba6; continue;
 }
 0x60a7ba6 => {
  cpu.push(0x60a7bab); f_060ad3f0(cpu); if cpu.returned_to != 0x60a7bab { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7bb6; continue; }
  pc = 0x60a7baf; continue;
 }
 0x60a7baf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a7bb8; continue;
 }
 0x60a7bb6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a7bb8; continue;
 }
 0x60a7bb8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7bf8; continue; }
  pc = 0x60a7bbc; continue;
 }
 0x60a7bbc => {
  { let v: u32 = cpu.r32(0x128u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a498u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7bf0); cpu.call(t); if cpu.returned_to != 0x60a7bf0 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7bf8); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7bf8 { return; }
  pc = 0x60a7bf8; continue;
 }
 0x60a7bf8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7c07); f_060b7a50(cpu); if cpu.returned_to != 0x60a7c07 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60a7c8e; continue; }
  pc = 0x60a7c18; continue;
 }
 0x60a7c18 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a7c2a; continue; }
  pc = 0x60a7c21; continue;
 }
 0x60a7c21 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7c8e; continue; }
  pc = 0x60a7c2a; continue;
 }
 0x60a7c2a => {
  cpu.push(0x60a7c2f); f_060b3c01(cpu); if cpu.returned_to != 0x60a7c2f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7c8e; continue; }
  pc = 0x60a7c33; continue;
 }
 0x60a7c33 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a7c4d); f_060b3e93(cpu); if cpu.returned_to != 0x60a7c4d { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a4c0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7c81); cpu.call(t); if cpu.returned_to != 0x60a7c81 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7c8e); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7c8e { return; }
  pc = 0x60a7c8e; continue;
 }
 0x60a7c8e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x62d12c4u32)); cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a7ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a7ca0); }
 let mut pc: u32 = 0x60a7ca0;
 loop { match pc {
 0x60a7ca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a7ce8; continue; }
  pc = 0x60a7cc1; continue;
 }
 0x60a7cc1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a7ccf; continue; }
  pc = 0x60a7cc6; continue;
 }
 0x60a7cc6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7ce8; continue; }
  pc = 0x60a7ccf; continue;
 }
 0x60a7ccf => {
  cpu.push(0x60a7cd4); f_060b3c01(cpu); if cpu.returned_to != 0x60a7cd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7ce8; continue; }
  pc = 0x60a7cd8; continue;
 }
 0x60a7cd8 => {
  cpu.push(0x60a7cdd); f_060ad3f0(cpu); if cpu.returned_to != 0x60a7cdd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7ce8; continue; }
  pc = 0x60a7ce1; continue;
 }
 0x60a7ce1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a7cea; continue;
 }
 0x60a7ce8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a7cea; continue;
 }
 0x60a7cea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7d2a; continue; }
  pc = 0x60a7cee; continue;
 }
 0x60a7cee => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a478u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a7d22); cpu.call(t); if cpu.returned_to != 0x60a7d22 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a7d2a); f_060ad7c0(cpu); if cpu.returned_to != 0x60a7d2a { return; }
  pc = 0x60a7d2a; continue;
 }
 0x60a7d2a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a7d3f); f_060b7ad0(cpu); if cpu.returned_to != 0x60a7d3f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a7d8c; continue; }
  pc = 0x60a7d4d; continue;
 }
 0x60a7d4d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a7d5f; continue; }
  pc = 0x60a7d56; continue;
 }
 0x60a7d56 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a7d8c; continue; }
  pc = 0x60a7d5f; continue;
 }
 0x60a7d5f => {
  cpu.push(0x60a7d64); f_060b3c01(cpu); if cpu.returned_to != 0x60a7d64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a7d8c; continue; }
  pc = 0x60a7d68; continue;
 }
 0x60a7d68 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a7d82); f_060b3e93(cpu); if cpu.returned_to != 0x60a7d82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a7d8c); f_060ad770(cpu); if cpu.returned_to != 0x60a7d8c { return; }
  pc = 0x60a7d8c; continue;
 }
 0x60a7d8c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060a83b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a83b0); }
 let mut pc: u32 = 0x60a83b0;
 loop { match pc {
 0x60a83b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a83f7; continue; }
  pc = 0x60a83d0; continue;
 }
 0x60a83d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a83de; continue; }
  pc = 0x60a83d5; continue;
 }
 0x60a83d5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a83f7; continue; }
  pc = 0x60a83de; continue;
 }
 0x60a83de => {
  cpu.push(0x60a83e3); f_060b3c01(cpu); if cpu.returned_to != 0x60a83e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a83f7; continue; }
  pc = 0x60a83e7; continue;
 }
 0x60a83e7 => {
  cpu.push(0x60a83ec); f_060ad3f0(cpu); if cpu.returned_to != 0x60a83ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a83f7; continue; }
  pc = 0x60a83f0; continue;
 }
 0x60a83f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a83f9; continue;
 }
 0x60a83f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a83f9; continue;
 }
 0x60a83f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a842b; continue; }
  pc = 0x60a83fd; continue;
 }
 0x60a83fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a380u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8423); cpu.call(t); if cpu.returned_to != 0x60a8423 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a842b); f_060ad7c0(cpu); if cpu.returned_to != 0x60a842b { return; }
  pc = 0x60a842b; continue;
 }
 0x60a842b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a8438); f_060b8560(cpu); if cpu.returned_to != 0x60a8438 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a8460(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8460); }
 let mut pc: u32 = 0x60a8460;
 loop { match pc {
 0x60a8460 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a84a8; continue; }
  pc = 0x60a8481; continue;
 }
 0x60a8481 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a848f; continue; }
  pc = 0x60a8486; continue;
 }
 0x60a8486 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a84a8; continue; }
  pc = 0x60a848f; continue;
 }
 0x60a848f => {
  cpu.push(0x60a8494); f_060b3c01(cpu); if cpu.returned_to != 0x60a8494 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a84a8; continue; }
  pc = 0x60a8498; continue;
 }
 0x60a8498 => {
  cpu.push(0x60a849d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a849d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a84a8; continue; }
  pc = 0x60a84a1; continue;
 }
 0x60a84a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a84aa; continue;
 }
 0x60a84a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a84aa; continue;
 }
 0x60a84aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a84dc; continue; }
  pc = 0x60a84ae; continue;
 }
 0x60a84ae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a364u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a84d4); cpu.call(t); if cpu.returned_to != 0x60a84d4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a84dc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a84dc { return; }
  pc = 0x60a84dc; continue;
 }
 0x60a84dc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a84e9); f_060b85e0(cpu); if cpu.returned_to != 0x60a84e9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a8537; continue; }
  pc = 0x60a84f8; continue;
 }
 0x60a84f8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a850a; continue; }
  pc = 0x60a8501; continue;
 }
 0x60a8501 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8537; continue; }
  pc = 0x60a850a; continue;
 }
 0x60a850a => {
  cpu.push(0x60a850f); f_060b3c01(cpu); if cpu.returned_to != 0x60a850f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8537; continue; }
  pc = 0x60a8513; continue;
 }
 0x60a8513 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a852d); f_060b3e93(cpu); if cpu.returned_to != 0x60a852d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a8537); f_060ad770(cpu); if cpu.returned_to != 0x60a8537 { return; }
  pc = 0x60a8537; continue;
 }
 0x60a8537 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a8b40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8b40); }
 let mut pc: u32 = 0x60a8b40;
 loop { match pc {
 0x60a8b40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a8b88; continue; }
  pc = 0x60a8b61; continue;
 }
 0x60a8b61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a8b6f; continue; }
  pc = 0x60a8b66; continue;
 }
 0x60a8b66 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8b88; continue; }
  pc = 0x60a8b6f; continue;
 }
 0x60a8b6f => {
  cpu.push(0x60a8b74); f_060b3c01(cpu); if cpu.returned_to != 0x60a8b74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8b88; continue; }
  pc = 0x60a8b78; continue;
 }
 0x60a8b78 => {
  cpu.push(0x60a8b7d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a8b7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8b88; continue; }
  pc = 0x60a8b81; continue;
 }
 0x60a8b81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a8b8a; continue;
 }
 0x60a8b88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a8b8a; continue;
 }
 0x60a8b8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8bbc; continue; }
  pc = 0x60a8b8e; continue;
 }
 0x60a8b8e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a278u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8bb4); cpu.call(t); if cpu.returned_to != 0x60a8bb4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8bbc); f_060ad7c0(cpu); if cpu.returned_to != 0x60a8bbc { return; }
  pc = 0x60a8bbc; continue;
 }
 0x60a8bbc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a8bc9); f_060b7dc0(cpu); if cpu.returned_to != 0x60a8bc9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a8c17; continue; }
  pc = 0x60a8bd8; continue;
 }
 0x60a8bd8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a8bea; continue; }
  pc = 0x60a8be1; continue;
 }
 0x60a8be1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8c17; continue; }
  pc = 0x60a8bea; continue;
 }
 0x60a8bea => {
  cpu.push(0x60a8bef); f_060b3c01(cpu); if cpu.returned_to != 0x60a8bef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8c17; continue; }
  pc = 0x60a8bf3; continue;
 }
 0x60a8bf3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a8c0d); f_060b3e93(cpu); if cpu.returned_to != 0x60a8c0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a8c17); f_060ad770(cpu); if cpu.returned_to != 0x60a8c17 { return; }
  pc = 0x60a8c17; continue;
 }
 0x60a8c17 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060a8c40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8c40); }
 let mut pc: u32 = 0x60a8c40;
 loop { match pc {
 0x60a8c40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a8c87; continue; }
  pc = 0x60a8c60; continue;
 }
 0x60a8c60 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a8c6e; continue; }
  pc = 0x60a8c65; continue;
 }
 0x60a8c65 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8c87; continue; }
  pc = 0x60a8c6e; continue;
 }
 0x60a8c6e => {
  cpu.push(0x60a8c73); f_060b3c01(cpu); if cpu.returned_to != 0x60a8c73 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8c87; continue; }
  pc = 0x60a8c77; continue;
 }
 0x60a8c77 => {
  cpu.push(0x60a8c7c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a8c7c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8c87; continue; }
  pc = 0x60a8c80; continue;
 }
 0x60a8c80 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a8c89; continue;
 }
 0x60a8c87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a8c89; continue;
 }
 0x60a8c89 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8cc9; continue; }
  pc = 0x60a8c8d; continue;
 }
 0x60a8c8d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a250u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8cc1); cpu.call(t); if cpu.returned_to != 0x60a8cc1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8cc9); f_060ad7c0(cpu); if cpu.returned_to != 0x60a8cc9 { return; }
  pc = 0x60a8cc9; continue;
 }
 0x60a8cc9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a8cde); f_060b7ec0(cpu); if cpu.returned_to != 0x60a8cde { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a8d00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8d00); }
 let mut pc: u32 = 0x60a8d00;
 loop { match pc {
 0x60a8d00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a8d47; continue; }
  pc = 0x60a8d20; continue;
 }
 0x60a8d20 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a8d2e; continue; }
  pc = 0x60a8d25; continue;
 }
 0x60a8d25 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8d47; continue; }
  pc = 0x60a8d2e; continue;
 }
 0x60a8d2e => {
  cpu.push(0x60a8d33); f_060b3c01(cpu); if cpu.returned_to != 0x60a8d33 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8d47; continue; }
  pc = 0x60a8d37; continue;
 }
 0x60a8d37 => {
  cpu.push(0x60a8d3c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a8d3c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8d47; continue; }
  pc = 0x60a8d40; continue;
 }
 0x60a8d40 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a8d49; continue;
 }
 0x60a8d47 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a8d49; continue;
 }
 0x60a8d49 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8d97; continue; }
  pc = 0x60a8d4d; continue;
 }
 0x60a8d4d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a228u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8d8f); cpu.call(t); if cpu.returned_to != 0x60a8d8f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8d97); f_060ad7c0(cpu); if cpu.returned_to != 0x60a8d97 { return; }
  pc = 0x60a8d97; continue;
 }
 0x60a8d97 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8db4); f_060b7dd0(cpu); if cpu.returned_to != 0x60a8db4 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a8dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8dd0); }
 let mut pc: u32 = 0x60a8dd0;
 loop { match pc {
 0x60a8dd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a8e17; continue; }
  pc = 0x60a8df0; continue;
 }
 0x60a8df0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a8dfe; continue; }
  pc = 0x60a8df5; continue;
 }
 0x60a8df5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8e17; continue; }
  pc = 0x60a8dfe; continue;
 }
 0x60a8dfe => {
  cpu.push(0x60a8e03); f_060b3c01(cpu); if cpu.returned_to != 0x60a8e03 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8e17; continue; }
  pc = 0x60a8e07; continue;
 }
 0x60a8e07 => {
  cpu.push(0x60a8e0c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a8e0c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8e17; continue; }
  pc = 0x60a8e10; continue;
 }
 0x60a8e10 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a8e19; continue;
 }
 0x60a8e17 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a8e19; continue;
 }
 0x60a8e19 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8e67; continue; }
  pc = 0x60a8e1d; continue;
 }
 0x60a8e1d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a200u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8e5f); cpu.call(t); if cpu.returned_to != 0x60a8e5f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8e67); f_060ad7c0(cpu); if cpu.returned_to != 0x60a8e67 { return; }
  pc = 0x60a8e67; continue;
 }
 0x60a8e67 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8e84); f_060b7e50(cpu); if cpu.returned_to != 0x60a8e84 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a8ea0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8ea0); }
 let mut pc: u32 = 0x60a8ea0;
 loop { match pc {
 0x60a8ea0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a8ee7; continue; }
  pc = 0x60a8ec0; continue;
 }
 0x60a8ec0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a8ece; continue; }
  pc = 0x60a8ec5; continue;
 }
 0x60a8ec5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8ee7; continue; }
  pc = 0x60a8ece; continue;
 }
 0x60a8ece => {
  cpu.push(0x60a8ed3); f_060b3c01(cpu); if cpu.returned_to != 0x60a8ed3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8ee7; continue; }
  pc = 0x60a8ed7; continue;
 }
 0x60a8ed7 => {
  cpu.push(0x60a8edc); f_060ad3f0(cpu); if cpu.returned_to != 0x60a8edc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8ee7; continue; }
  pc = 0x60a8ee0; continue;
 }
 0x60a8ee0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a8ee9; continue;
 }
 0x60a8ee7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a8ee9; continue;
 }
 0x60a8ee9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8f1b; continue; }
  pc = 0x60a8eed; continue;
 }
 0x60a8eed => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a1e4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8f13); cpu.call(t); if cpu.returned_to != 0x60a8f13 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8f1b); f_060ad7c0(cpu); if cpu.returned_to != 0x60a8f1b { return; }
  pc = 0x60a8f1b; continue;
 }
 0x60a8f1b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a8f28); f_060b7ed0(cpu); if cpu.returned_to != 0x60a8f28 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a8f50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a8f50); }
 let mut pc: u32 = 0x60a8f50;
 loop { match pc {
 0x60a8f50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a8f97; continue; }
  pc = 0x60a8f70; continue;
 }
 0x60a8f70 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a8f7e; continue; }
  pc = 0x60a8f75; continue;
 }
 0x60a8f75 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a8f97; continue; }
  pc = 0x60a8f7e; continue;
 }
 0x60a8f7e => {
  cpu.push(0x60a8f83); f_060b3c01(cpu); if cpu.returned_to != 0x60a8f83 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a8f97; continue; }
  pc = 0x60a8f87; continue;
 }
 0x60a8f87 => {
  cpu.push(0x60a8f8c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a8f8c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8f97; continue; }
  pc = 0x60a8f90; continue;
 }
 0x60a8f90 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a8f99; continue;
 }
 0x60a8f97 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a8f99; continue;
 }
 0x60a8f99 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a8fcb; continue; }
  pc = 0x60a8f9d; continue;
 }
 0x60a8f9d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a1c8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a8fc3); cpu.call(t); if cpu.returned_to != 0x60a8fc3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a8fcb); f_060ad7c0(cpu); if cpu.returned_to != 0x60a8fcb { return; }
  pc = 0x60a8fcb; continue;
 }
 0x60a8fcb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a8fd8); f_060b7f10(cpu); if cpu.returned_to != 0x60a8fd8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a9000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9000); }
 let mut pc: u32 = 0x60a9000;
 loop { match pc {
 0x60a9000 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9047; continue; }
  pc = 0x60a9020; continue;
 }
 0x60a9020 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a902e; continue; }
  pc = 0x60a9025; continue;
 }
 0x60a9025 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9047; continue; }
  pc = 0x60a902e; continue;
 }
 0x60a902e => {
  cpu.push(0x60a9033); f_060b3c01(cpu); if cpu.returned_to != 0x60a9033 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9047; continue; }
  pc = 0x60a9037; continue;
 }
 0x60a9037 => {
  cpu.push(0x60a903c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a903c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9047; continue; }
  pc = 0x60a9040; continue;
 }
 0x60a9040 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a9049; continue;
 }
 0x60a9047 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a9049; continue;
 }
 0x60a9049 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a907b; continue; }
  pc = 0x60a904d; continue;
 }
 0x60a904d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a1acu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9073); cpu.call(t); if cpu.returned_to != 0x60a9073 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a907b); f_060ad7c0(cpu); if cpu.returned_to != 0x60a907b { return; }
  pc = 0x60a907b; continue;
 }
 0x60a907b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a9088); f_060b7f70(cpu); if cpu.returned_to != 0x60a9088 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a90b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a90b0); }
 let mut pc: u32 = 0x60a90b0;
 loop { match pc {
 0x60a90b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a90f7; continue; }
  pc = 0x60a90d0; continue;
 }
 0x60a90d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a90de; continue; }
  pc = 0x60a90d5; continue;
 }
 0x60a90d5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a90f7; continue; }
  pc = 0x60a90de; continue;
 }
 0x60a90de => {
  cpu.push(0x60a90e3); f_060b3c01(cpu); if cpu.returned_to != 0x60a90e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a90f7; continue; }
  pc = 0x60a90e7; continue;
 }
 0x60a90e7 => {
  cpu.push(0x60a90ec); f_060ad3f0(cpu); if cpu.returned_to != 0x60a90ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a90f7; continue; }
  pc = 0x60a90f0; continue;
 }
 0x60a90f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a90f9; continue;
 }
 0x60a90f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a90f9; continue;
 }
 0x60a90f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a912b; continue; }
  pc = 0x60a90fd; continue;
 }
 0x60a90fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a190u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9123); cpu.call(t); if cpu.returned_to != 0x60a9123 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a912b); f_060ad7c0(cpu); if cpu.returned_to != 0x60a912b { return; }
  pc = 0x60a912b; continue;
 }
 0x60a912b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a9138); f_060b8020(cpu); if cpu.returned_to != 0x60a9138 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a9160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9160); }
 let mut pc: u32 = 0x60a9160;
 loop { match pc {
 0x60a9160 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a91a7; continue; }
  pc = 0x60a9180; continue;
 }
 0x60a9180 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a918e; continue; }
  pc = 0x60a9185; continue;
 }
 0x60a9185 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a91a7; continue; }
  pc = 0x60a918e; continue;
 }
 0x60a918e => {
  cpu.push(0x60a9193); f_060b3c01(cpu); if cpu.returned_to != 0x60a9193 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a91a7; continue; }
  pc = 0x60a9197; continue;
 }
 0x60a9197 => {
  cpu.push(0x60a919c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a919c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a91a7; continue; }
  pc = 0x60a91a0; continue;
 }
 0x60a91a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a91a9; continue;
 }
 0x60a91a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a91a9; continue;
 }
 0x60a91a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a91f7; continue; }
  pc = 0x60a91ad; continue;
 }
 0x60a91ad => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a16cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a91ef); cpu.call(t); if cpu.returned_to != 0x60a91ef { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a91f7); f_060ad7c0(cpu); if cpu.returned_to != 0x60a91f7 { return; }
  pc = 0x60a91f7; continue;
 }
 0x60a91f7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a9214); f_060b8060(cpu); if cpu.returned_to != 0x60a9214 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60a9262; continue; }
  pc = 0x60a9223; continue;
 }
 0x60a9223 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a9235; continue; }
  pc = 0x60a922c; continue;
 }
 0x60a922c => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9262; continue; }
  pc = 0x60a9235; continue;
 }
 0x60a9235 => {
  cpu.push(0x60a923a); f_060b3c01(cpu); if cpu.returned_to != 0x60a923a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9262; continue; }
  pc = 0x60a923e; continue;
 }
 0x60a923e => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a9258); f_060b3e93(cpu); if cpu.returned_to != 0x60a9258 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a9262); f_060ad770(cpu); if cpu.returned_to != 0x60a9262 { return; }
  pc = 0x60a9262; continue;
 }
 0x60a9262 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a9280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9280); }
 let mut pc: u32 = 0x60a9280;
 loop { match pc {
 0x60a9280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a92c7; continue; }
  pc = 0x60a92a0; continue;
 }
 0x60a92a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a92ae; continue; }
  pc = 0x60a92a5; continue;
 }
 0x60a92a5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a92c7; continue; }
  pc = 0x60a92ae; continue;
 }
 0x60a92ae => {
  cpu.push(0x60a92b3); f_060b3c01(cpu); if cpu.returned_to != 0x60a92b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a92c7; continue; }
  pc = 0x60a92b7; continue;
 }
 0x60a92b7 => {
  cpu.push(0x60a92bc); f_060ad3f0(cpu); if cpu.returned_to != 0x60a92bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a92c7; continue; }
  pc = 0x60a92c0; continue;
 }
 0x60a92c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a92c9; continue;
 }
 0x60a92c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a92c9; continue;
 }
 0x60a92c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a92fb; continue; }
  pc = 0x60a92cd; continue;
 }
 0x60a92cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a148u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a92f3); cpu.call(t); if cpu.returned_to != 0x60a92f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a92fb); f_060ad7c0(cpu); if cpu.returned_to != 0x60a92fb { return; }
  pc = 0x60a92fb; continue;
 }
 0x60a92fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a9308); f_060b84b0(cpu); if cpu.returned_to != 0x60a9308 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060a9330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9330); }
 let mut pc: u32 = 0x60a9330;
 loop { match pc {
 0x60a9330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9378; continue; }
  pc = 0x60a9351; continue;
 }
 0x60a9351 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a935f; continue; }
  pc = 0x60a9356; continue;
 }
 0x60a9356 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9378; continue; }
  pc = 0x60a935f; continue;
 }
 0x60a935f => {
  cpu.push(0x60a9364); f_060b3c01(cpu); if cpu.returned_to != 0x60a9364 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9378; continue; }
  pc = 0x60a9368; continue;
 }
 0x60a9368 => {
  cpu.push(0x60a936d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a936d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9378; continue; }
  pc = 0x60a9371; continue;
 }
 0x60a9371 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a937a; continue;
 }
 0x60a9378 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a937a; continue;
 }
 0x60a937a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a93ac; continue; }
  pc = 0x60a937e; continue;
 }
 0x60a937e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a120u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a93a4); cpu.call(t); if cpu.returned_to != 0x60a93a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a93ac); f_060ad7c0(cpu); if cpu.returned_to != 0x60a93ac { return; }
  pc = 0x60a93ac; continue;
 }
 0x60a93ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a93b9); f_060b84d0(cpu); if cpu.returned_to != 0x60a93b9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a9407; continue; }
  pc = 0x60a93c8; continue;
 }
 0x60a93c8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a93da; continue; }
  pc = 0x60a93d1; continue;
 }
 0x60a93d1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9407; continue; }
  pc = 0x60a93da; continue;
 }
 0x60a93da => {
  cpu.push(0x60a93df); f_060b3c01(cpu); if cpu.returned_to != 0x60a93df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9407; continue; }
  pc = 0x60a93e3; continue;
 }
 0x60a93e3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a93fd); f_060b3e93(cpu); if cpu.returned_to != 0x60a93fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a9407); f_060ad720(cpu); if cpu.returned_to != 0x60a9407 { return; }
  pc = 0x60a9407; continue;
 }
 0x60a9407 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060a9430(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9430); }
 let mut pc: u32 = 0x60a9430;
 loop { match pc {
 0x60a9430 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9477; continue; }
  pc = 0x60a9450; continue;
 }
 0x60a9450 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a945e; continue; }
  pc = 0x60a9455; continue;
 }
 0x60a9455 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9477; continue; }
  pc = 0x60a945e; continue;
 }
 0x60a945e => {
  cpu.push(0x60a9463); f_060b3c01(cpu); if cpu.returned_to != 0x60a9463 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9477; continue; }
  pc = 0x60a9467; continue;
 }
 0x60a9467 => {
  cpu.push(0x60a946c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a946c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9477; continue; }
  pc = 0x60a9470; continue;
 }
 0x60a9470 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a9479; continue;
 }
 0x60a9477 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a9479; continue;
 }
 0x60a9479 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a94ab; continue; }
  pc = 0x60a947d; continue;
 }
 0x60a947d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a0fcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a94a3); cpu.call(t); if cpu.returned_to != 0x60a94a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a94ab); f_060ad7c0(cpu); if cpu.returned_to != 0x60a94ab { return; }
  pc = 0x60a94ab; continue;
 }
 0x60a94ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a94b8); f_060b8740(cpu); if cpu.returned_to != 0x60a94b8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a94e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a94e0); }
 let mut pc: u32 = 0x60a94e0;
 loop { match pc {
 0x60a94e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9528; continue; }
  pc = 0x60a9501; continue;
 }
 0x60a9501 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a950f; continue; }
  pc = 0x60a9506; continue;
 }
 0x60a9506 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9528; continue; }
  pc = 0x60a950f; continue;
 }
 0x60a950f => {
  cpu.push(0x60a9514); f_060b3c01(cpu); if cpu.returned_to != 0x60a9514 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9528; continue; }
  pc = 0x60a9518; continue;
 }
 0x60a9518 => {
  cpu.push(0x60a951d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a951d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9528; continue; }
  pc = 0x60a9521; continue;
 }
 0x60a9521 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a952a; continue;
 }
 0x60a9528 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a952a; continue;
 }
 0x60a952a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a956a; continue; }
  pc = 0x60a952e; continue;
 }
 0x60a952e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a0d4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9562); cpu.call(t); if cpu.returned_to != 0x60a9562 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a956a); f_060ad7c0(cpu); if cpu.returned_to != 0x60a956a { return; }
  pc = 0x60a956a; continue;
 }
 0x60a956a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a957f); f_060b8820(cpu); if cpu.returned_to != 0x60a957f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a95cc; continue; }
  pc = 0x60a958d; continue;
 }
 0x60a958d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a959f; continue; }
  pc = 0x60a9596; continue;
 }
 0x60a9596 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a95cc; continue; }
  pc = 0x60a959f; continue;
 }
 0x60a959f => {
  cpu.push(0x60a95a4); f_060b3c01(cpu); if cpu.returned_to != 0x60a95a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a95cc; continue; }
  pc = 0x60a95a8; continue;
 }
 0x60a95a8 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a95c2); f_060b3e93(cpu); if cpu.returned_to != 0x60a95c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a95cc); f_060ad720(cpu); if cpu.returned_to != 0x60a95cc { return; }
  pc = 0x60a95cc; continue;
 }
 0x60a95cc => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060a95f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a95f0); }
 let mut pc: u32 = 0x60a95f0;
 loop { match pc {
 0x60a95f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9638; continue; }
  pc = 0x60a9611; continue;
 }
 0x60a9611 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a961f; continue; }
  pc = 0x60a9616; continue;
 }
 0x60a9616 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9638; continue; }
  pc = 0x60a961f; continue;
 }
 0x60a961f => {
  cpu.push(0x60a9624); f_060b3c01(cpu); if cpu.returned_to != 0x60a9624 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9638; continue; }
  pc = 0x60a9628; continue;
 }
 0x60a9628 => {
  cpu.push(0x60a962d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a962d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9638; continue; }
  pc = 0x60a9631; continue;
 }
 0x60a9631 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a963a; continue;
 }
 0x60a9638 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a963a; continue;
 }
 0x60a963a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a966c; continue; }
  pc = 0x60a963e; continue;
 }
 0x60a963e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a0b8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9664); cpu.call(t); if cpu.returned_to != 0x60a9664 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a966c); f_060ad7c0(cpu); if cpu.returned_to != 0x60a966c { return; }
  pc = 0x60a966c; continue;
 }
 0x60a966c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a9679); f_060b8760(cpu); if cpu.returned_to != 0x60a9679 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a96c7; continue; }
  pc = 0x60a9688; continue;
 }
 0x60a9688 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a969a; continue; }
  pc = 0x60a9691; continue;
 }
 0x60a9691 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a96c7; continue; }
  pc = 0x60a969a; continue;
 }
 0x60a969a => {
  cpu.push(0x60a969f); f_060b3c01(cpu); if cpu.returned_to != 0x60a969f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a96c7; continue; }
  pc = 0x60a96a3; continue;
 }
 0x60a96a3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a96bd); f_060b3e93(cpu); if cpu.returned_to != 0x60a96bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a96c7); f_060ad770(cpu); if cpu.returned_to != 0x60a96c7 { return; }
  pc = 0x60a96c7; continue;
 }
 0x60a96c7 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a96f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a96f0); }
 let mut pc: u32 = 0x60a96f0;
 loop { match pc {
 0x60a96f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9738; continue; }
  pc = 0x60a9711; continue;
 }
 0x60a9711 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a971f; continue; }
  pc = 0x60a9716; continue;
 }
 0x60a9716 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9738; continue; }
  pc = 0x60a971f; continue;
 }
 0x60a971f => {
  cpu.push(0x60a9724); f_060b3c01(cpu); if cpu.returned_to != 0x60a9724 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9738; continue; }
  pc = 0x60a9728; continue;
 }
 0x60a9728 => {
  cpu.push(0x60a972d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a972d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9738; continue; }
  pc = 0x60a9731; continue;
 }
 0x60a9731 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a973a; continue;
 }
 0x60a9738 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a973a; continue;
 }
 0x60a973a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a976c; continue; }
  pc = 0x60a973e; continue;
 }
 0x60a973e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a098u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9764); cpu.call(t); if cpu.returned_to != 0x60a9764 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a976c); f_060ad7c0(cpu); if cpu.returned_to != 0x60a976c { return; }
  pc = 0x60a976c; continue;
 }
 0x60a976c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60a9779); f_060b5520(cpu); if cpu.returned_to != 0x60a9779 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60a97c7; continue; }
  pc = 0x60a9788; continue;
 }
 0x60a9788 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a979a; continue; }
  pc = 0x60a9791; continue;
 }
 0x60a9791 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a97c7; continue; }
  pc = 0x60a979a; continue;
 }
 0x60a979a => {
  cpu.push(0x60a979f); f_060b3c01(cpu); if cpu.returned_to != 0x60a979f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a97c7; continue; }
  pc = 0x60a97a3; continue;
 }
 0x60a97a3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60a97bd); f_060b3e93(cpu); if cpu.returned_to != 0x60a97bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a97c7); f_060ad770(cpu); if cpu.returned_to != 0x60a97c7 { return; }
  pc = 0x60a97c7; continue;
 }
 0x60a97c7 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a97f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a97f0); }
 let mut pc: u32 = 0x60a97f0;
 loop { match pc {
 0x60a97f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9838; continue; }
  pc = 0x60a9811; continue;
 }
 0x60a9811 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a981f; continue; }
  pc = 0x60a9816; continue;
 }
 0x60a9816 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9838; continue; }
  pc = 0x60a981f; continue;
 }
 0x60a981f => {
  cpu.push(0x60a9824); f_060b3c01(cpu); if cpu.returned_to != 0x60a9824 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9838; continue; }
  pc = 0x60a9828; continue;
 }
 0x60a9828 => {
  cpu.push(0x60a982d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a982d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9838; continue; }
  pc = 0x60a9831; continue;
 }
 0x60a9831 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a983a; continue;
 }
 0x60a9838 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a983a; continue;
 }
 0x60a983a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a987a; continue; }
  pc = 0x60a983e; continue;
 }
 0x60a983e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a078u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9872); cpu.call(t); if cpu.returned_to != 0x60a9872 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a987a); f_060ad7c0(cpu); if cpu.returned_to != 0x60a987a { return; }
  pc = 0x60a987a; continue;
 }
 0x60a987a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a988f); f_060b5020(cpu); if cpu.returned_to != 0x60a988f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a98dc; continue; }
  pc = 0x60a989d; continue;
 }
 0x60a989d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a98af; continue; }
  pc = 0x60a98a6; continue;
 }
 0x60a98a6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a98dc; continue; }
  pc = 0x60a98af; continue;
 }
 0x60a98af => {
  cpu.push(0x60a98b4); f_060b3c01(cpu); if cpu.returned_to != 0x60a98b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a98dc; continue; }
  pc = 0x60a98b8; continue;
 }
 0x60a98b8 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a98d2); f_060b3e93(cpu); if cpu.returned_to != 0x60a98d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a98dc); f_060ad770(cpu); if cpu.returned_to != 0x60a98dc { return; }
  pc = 0x60a98dc; continue;
 }
 0x60a98dc => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060a9900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9900); }
 let mut pc: u32 = 0x60a9900;
 loop { match pc {
 0x60a9900 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9947; continue; }
  pc = 0x60a9920; continue;
 }
 0x60a9920 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a992e; continue; }
  pc = 0x60a9925; continue;
 }
 0x60a9925 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9947; continue; }
  pc = 0x60a992e; continue;
 }
 0x60a992e => {
  cpu.push(0x60a9933); f_060b3c01(cpu); if cpu.returned_to != 0x60a9933 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9947; continue; }
  pc = 0x60a9937; continue;
 }
 0x60a9937 => {
  cpu.push(0x60a993c); f_060ad3f0(cpu); if cpu.returned_to != 0x60a993c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9947; continue; }
  pc = 0x60a9940; continue;
 }
 0x60a9940 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a9949; continue;
 }
 0x60a9947 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a9949; continue;
 }
 0x60a9949 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9997; continue; }
  pc = 0x60a994d; continue;
 }
 0x60a994d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a050u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a998f); cpu.call(t); if cpu.returned_to != 0x60a998f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a9997); f_060ad7c0(cpu); if cpu.returned_to != 0x60a9997 { return; }
  pc = 0x60a9997; continue;
 }
 0x60a9997 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a99b4); f_060b5040(cpu); if cpu.returned_to != 0x60a99b4 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060a9ae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60a9ae0); }
 let mut pc: u32 = 0x60a9ae0;
 loop { match pc {
 0x60a9ae0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60a9b28; continue; }
  pc = 0x60a9b01; continue;
 }
 0x60a9b01 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60a9b0f; continue; }
  pc = 0x60a9b06; continue;
 }
 0x60a9b06 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9b28; continue; }
  pc = 0x60a9b0f; continue;
 }
 0x60a9b0f => {
  cpu.push(0x60a9b14); f_060b3c01(cpu); if cpu.returned_to != 0x60a9b14 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9b28; continue; }
  pc = 0x60a9b18; continue;
 }
 0x60a9b18 => {
  cpu.push(0x60a9b1d); f_060ad3f0(cpu); if cpu.returned_to != 0x60a9b1d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9b28; continue; }
  pc = 0x60a9b21; continue;
 }
 0x60a9b21 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60a9b2a; continue;
 }
 0x60a9b28 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60a9b2a; continue;
 }
 0x60a9b2a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9b6a; continue; }
  pc = 0x60a9b2e; continue;
 }
 0x60a9b2e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616a000u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60a9b62); cpu.call(t); if cpu.returned_to != 0x60a9b62 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60a9b6a); f_060ad7c0(cpu); if cpu.returned_to != 0x60a9b6a { return; }
  pc = 0x60a9b6a; continue;
 }
 0x60a9b6a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60a9b7f); f_060b5060(cpu); if cpu.returned_to != 0x60a9b7f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60a9bcc; continue; }
  pc = 0x60a9b8d; continue;
 }
 0x60a9b8d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60a9b9f; continue; }
  pc = 0x60a9b96; continue;
 }
 0x60a9b96 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60a9bcc; continue; }
  pc = 0x60a9b9f; continue;
 }
 0x60a9b9f => {
  cpu.push(0x60a9ba4); f_060b3c01(cpu); if cpu.returned_to != 0x60a9ba4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60a9bcc; continue; }
  pc = 0x60a9ba8; continue;
 }
 0x60a9ba8 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60a9bc2); f_060b3e93(cpu); if cpu.returned_to != 0x60a9bc2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60a9bcc); f_060ad720(cpu); if cpu.returned_to != 0x60a9bcc { return; }
  pc = 0x60a9bcc; continue;
 }
 0x60a9bcc => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060aa7a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aa7a0); }
 let mut pc: u32 = 0x60aa7a0;
 loop { match pc {
 0x60aa7a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aa7e7; continue; }
  pc = 0x60aa7c0; continue;
 }
 0x60aa7c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aa7ce; continue; }
  pc = 0x60aa7c5; continue;
 }
 0x60aa7c5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aa7e7; continue; }
  pc = 0x60aa7ce; continue;
 }
 0x60aa7ce => {
  cpu.push(0x60aa7d3); f_060b3c01(cpu); if cpu.returned_to != 0x60aa7d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aa7e7; continue; }
  pc = 0x60aa7d7; continue;
 }
 0x60aa7d7 => {
  cpu.push(0x60aa7dc); f_060ad3f0(cpu); if cpu.returned_to != 0x60aa7dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aa7e7; continue; }
  pc = 0x60aa7e0; continue;
 }
 0x60aa7e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aa7e9; continue;
 }
 0x60aa7e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aa7e9; continue;
 }
 0x60aa7e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aa829; continue; }
  pc = 0x60aa7ed; continue;
 }
 0x60aa7ed => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169df0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aa821); cpu.call(t); if cpu.returned_to != 0x60aa821 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aa829); f_060ad7c0(cpu); if cpu.returned_to != 0x60aa829 { return; }
  pc = 0x60aa829; continue;
 }
 0x60aa829 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60aa83e); f_060b4fe0(cpu); if cpu.returned_to != 0x60aa83e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aa860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aa860); }
 let mut pc: u32 = 0x60aa860;
 loop { match pc {
 0x60aa860 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aa8a8; continue; }
  pc = 0x60aa881; continue;
 }
 0x60aa881 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aa88f; continue; }
  pc = 0x60aa886; continue;
 }
 0x60aa886 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aa8a8; continue; }
  pc = 0x60aa88f; continue;
 }
 0x60aa88f => {
  cpu.push(0x60aa894); f_060b3c01(cpu); if cpu.returned_to != 0x60aa894 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aa8a8; continue; }
  pc = 0x60aa898; continue;
 }
 0x60aa898 => {
  cpu.push(0x60aa89d); f_060ad3f0(cpu); if cpu.returned_to != 0x60aa89d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aa8a8; continue; }
  pc = 0x60aa8a1; continue;
 }
 0x60aa8a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aa8aa; continue;
 }
 0x60aa8a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aa8aa; continue;
 }
 0x60aa8aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aa8dc; continue; }
  pc = 0x60aa8ae; continue;
 }
 0x60aa8ae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169dd0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aa8d4); cpu.call(t); if cpu.returned_to != 0x60aa8d4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aa8dc); f_060ad7c0(cpu); if cpu.returned_to != 0x60aa8dc { return; }
  pc = 0x60aa8dc; continue;
 }
 0x60aa8dc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60aa8e9); f_060b5250(cpu); if cpu.returned_to != 0x60aa8e9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60aa937; continue; }
  pc = 0x60aa8f8; continue;
 }
 0x60aa8f8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60aa90a; continue; }
  pc = 0x60aa901; continue;
 }
 0x60aa901 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aa937; continue; }
  pc = 0x60aa90a; continue;
 }
 0x60aa90a => {
  cpu.push(0x60aa90f); f_060b3c01(cpu); if cpu.returned_to != 0x60aa90f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aa937; continue; }
  pc = 0x60aa913; continue;
 }
 0x60aa913 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60aa92d); f_060b3e93(cpu); if cpu.returned_to != 0x60aa92d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60aa937); f_060ad770(cpu); if cpu.returned_to != 0x60aa937 { return; }
  pc = 0x60aa937; continue;
 }
 0x60aa937 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060aac60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aac60); }
 let mut pc: u32 = 0x60aac60;
 loop { match pc {
 0x60aac60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aaca8; continue; }
  pc = 0x60aac81; continue;
 }
 0x60aac81 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aac8f; continue; }
  pc = 0x60aac86; continue;
 }
 0x60aac86 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aaca8; continue; }
  pc = 0x60aac8f; continue;
 }
 0x60aac8f => {
  cpu.push(0x60aac94); f_060b3c01(cpu); if cpu.returned_to != 0x60aac94 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aaca8; continue; }
  pc = 0x60aac98; continue;
 }
 0x60aac98 => {
  cpu.push(0x60aac9d); f_060ad3f0(cpu); if cpu.returned_to != 0x60aac9d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aaca8; continue; }
  pc = 0x60aaca1; continue;
 }
 0x60aaca1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aacaa; continue;
 }
 0x60aaca8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aacaa; continue;
 }
 0x60aacaa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aacdc; continue; }
  pc = 0x60aacae; continue;
 }
 0x60aacae => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169d5cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aacd4); cpu.call(t); if cpu.returned_to != 0x60aacd4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aacdc); f_060ad7c0(cpu); if cpu.returned_to != 0x60aacdc { return; }
  pc = 0x60aacdc; continue;
 }
 0x60aacdc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60aace9); f_060b5390(cpu); if cpu.returned_to != 0x60aace9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60aad37; continue; }
  pc = 0x60aacf8; continue;
 }
 0x60aacf8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60aad0a; continue; }
  pc = 0x60aad01; continue;
 }
 0x60aad01 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aad37; continue; }
  pc = 0x60aad0a; continue;
 }
 0x60aad0a => {
  cpu.push(0x60aad0f); f_060b3c01(cpu); if cpu.returned_to != 0x60aad0f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aad37; continue; }
  pc = 0x60aad13; continue;
 }
 0x60aad13 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60aad2d); f_060b3e93(cpu); if cpu.returned_to != 0x60aad2d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60aad37); f_060ad770(cpu); if cpu.returned_to != 0x60aad37 { return; }
  pc = 0x60aad37; continue;
 }
 0x60aad37 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060aad60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aad60); }
 let mut pc: u32 = 0x60aad60;
 loop { match pc {
 0x60aad60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aada7; continue; }
  pc = 0x60aad80; continue;
 }
 0x60aad80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aad8e; continue; }
  pc = 0x60aad85; continue;
 }
 0x60aad85 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aada7; continue; }
  pc = 0x60aad8e; continue;
 }
 0x60aad8e => {
  cpu.push(0x60aad93); f_060b3c01(cpu); if cpu.returned_to != 0x60aad93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aada7; continue; }
  pc = 0x60aad97; continue;
 }
 0x60aad97 => {
  cpu.push(0x60aad9c); f_060ad3f0(cpu); if cpu.returned_to != 0x60aad9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aada7; continue; }
  pc = 0x60aada0; continue;
 }
 0x60aada0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aada9; continue;
 }
 0x60aada7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aada9; continue;
 }
 0x60aada9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aadf7; continue; }
  pc = 0x60aadad; continue;
 }
 0x60aadad => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169d30u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aadef); cpu.call(t); if cpu.returned_to != 0x60aadef { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aadf7); f_060ad7c0(cpu); if cpu.returned_to != 0x60aadf7 { return; }
  pc = 0x60aadf7; continue;
 }
 0x60aadf7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aae14); f_060b5220(cpu); if cpu.returned_to != 0x60aae14 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060aae30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aae30); }
 let mut pc: u32 = 0x60aae30;
 loop { match pc {
 0x60aae30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aae77; continue; }
  pc = 0x60aae50; continue;
 }
 0x60aae50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aae5e; continue; }
  pc = 0x60aae55; continue;
 }
 0x60aae55 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aae77; continue; }
  pc = 0x60aae5e; continue;
 }
 0x60aae5e => {
  cpu.push(0x60aae63); f_060b3c01(cpu); if cpu.returned_to != 0x60aae63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aae77; continue; }
  pc = 0x60aae67; continue;
 }
 0x60aae67 => {
  cpu.push(0x60aae6c); f_060ad3f0(cpu); if cpu.returned_to != 0x60aae6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aae77; continue; }
  pc = 0x60aae70; continue;
 }
 0x60aae70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aae79; continue;
 }
 0x60aae77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aae79; continue;
 }
 0x60aae79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aaeb9; continue; }
  pc = 0x60aae7d; continue;
 }
 0x60aae7d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169d08u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aaeb1); cpu.call(t); if cpu.returned_to != 0x60aaeb1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aaeb9); f_060ad7c0(cpu); if cpu.returned_to != 0x60aaeb9 { return; }
  pc = 0x60aaeb9; continue;
 }
 0x60aaeb9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60aaece); f_060b5240(cpu); if cpu.returned_to != 0x60aaece { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060aaef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aaef0); }
 let mut pc: u32 = 0x60aaef0;
 loop { match pc {
 0x60aaef0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aaf37; continue; }
  pc = 0x60aaf10; continue;
 }
 0x60aaf10 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aaf1e; continue; }
  pc = 0x60aaf15; continue;
 }
 0x60aaf15 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aaf37; continue; }
  pc = 0x60aaf1e; continue;
 }
 0x60aaf1e => {
  cpu.push(0x60aaf23); f_060b3c01(cpu); if cpu.returned_to != 0x60aaf23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aaf37; continue; }
  pc = 0x60aaf27; continue;
 }
 0x60aaf27 => {
  cpu.push(0x60aaf2c); f_060ad3f0(cpu); if cpu.returned_to != 0x60aaf2c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aaf37; continue; }
  pc = 0x60aaf30; continue;
 }
 0x60aaf30 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aaf39; continue;
 }
 0x60aaf37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aaf39; continue;
 }
 0x60aaf39 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aaf79; continue; }
  pc = 0x60aaf3d; continue;
 }
 0x60aaf3d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169ce8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aaf71); cpu.call(t); if cpu.returned_to != 0x60aaf71 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aaf79); f_060ad7c0(cpu); if cpu.returned_to != 0x60aaf79 { return; }
  pc = 0x60aaf79; continue;
 }
 0x60aaf79 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60aaf8e); f_060b52b0(cpu); if cpu.returned_to != 0x60aaf8e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060aafb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aafb0); }
 let mut pc: u32 = 0x60aafb0;
 loop { match pc {
 0x60aafb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aaff7; continue; }
  pc = 0x60aafd0; continue;
 }
 0x60aafd0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aafde; continue; }
  pc = 0x60aafd5; continue;
 }
 0x60aafd5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aaff7; continue; }
  pc = 0x60aafde; continue;
 }
 0x60aafde => {
  cpu.push(0x60aafe3); f_060b3c01(cpu); if cpu.returned_to != 0x60aafe3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aaff7; continue; }
  pc = 0x60aafe7; continue;
 }
 0x60aafe7 => {
  cpu.push(0x60aafec); f_060ad3f0(cpu); if cpu.returned_to != 0x60aafec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aaff7; continue; }
  pc = 0x60aaff0; continue;
 }
 0x60aaff0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aaff9; continue;
 }
 0x60aaff7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aaff9; continue;
 }
 0x60aaff9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab039; continue; }
  pc = 0x60aaffd; continue;
 }
 0x60aaffd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169cc4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab031); cpu.call(t); if cpu.returned_to != 0x60ab031 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab039); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab039 { return; }
  pc = 0x60ab039; continue;
 }
 0x60ab039 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ab04e); f_060b52e0(cpu); if cpu.returned_to != 0x60ab04e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab070(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab070); }
 let mut pc: u32 = 0x60ab070;
 loop { match pc {
 0x60ab070 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab0b7; continue; }
  pc = 0x60ab090; continue;
 }
 0x60ab090 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab09e; continue; }
  pc = 0x60ab095; continue;
 }
 0x60ab095 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab0b7; continue; }
  pc = 0x60ab09e; continue;
 }
 0x60ab09e => {
  cpu.push(0x60ab0a3); f_060b3c01(cpu); if cpu.returned_to != 0x60ab0a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab0b7; continue; }
  pc = 0x60ab0a7; continue;
 }
 0x60ab0a7 => {
  cpu.push(0x60ab0ac); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab0ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab0b7; continue; }
  pc = 0x60ab0b0; continue;
 }
 0x60ab0b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab0b9; continue;
 }
 0x60ab0b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab0b9; continue;
 }
 0x60ab0b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab0f9; continue; }
  pc = 0x60ab0bd; continue;
 }
 0x60ab0bd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169c9cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab0f1); cpu.call(t); if cpu.returned_to != 0x60ab0f1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab0f9); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab0f9 { return; }
  pc = 0x60ab0f9; continue;
 }
 0x60ab0f9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ab10e); f_060b5310(cpu); if cpu.returned_to != 0x60ab10e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab130(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab130); }
 let mut pc: u32 = 0x60ab130;
 loop { match pc {
 0x60ab130 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab177; continue; }
  pc = 0x60ab150; continue;
 }
 0x60ab150 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab15e; continue; }
  pc = 0x60ab155; continue;
 }
 0x60ab155 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab177; continue; }
  pc = 0x60ab15e; continue;
 }
 0x60ab15e => {
  cpu.push(0x60ab163); f_060b3c01(cpu); if cpu.returned_to != 0x60ab163 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab177; continue; }
  pc = 0x60ab167; continue;
 }
 0x60ab167 => {
  cpu.push(0x60ab16c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab16c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab177; continue; }
  pc = 0x60ab170; continue;
 }
 0x60ab170 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab179; continue;
 }
 0x60ab177 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab179; continue;
 }
 0x60ab179 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab1ab; continue; }
  pc = 0x60ab17d; continue;
 }
 0x60ab17d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169c84u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab1a3); cpu.call(t); if cpu.returned_to != 0x60ab1a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab1ab); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab1ab { return; }
  pc = 0x60ab1ab; continue;
 }
 0x60ab1ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab1b8); f_060b50c0(cpu); if cpu.returned_to != 0x60ab1b8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ab1e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab1e0); }
 let mut pc: u32 = 0x60ab1e0;
 loop { match pc {
 0x60ab1e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab227; continue; }
  pc = 0x60ab200; continue;
 }
 0x60ab200 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab20e; continue; }
  pc = 0x60ab205; continue;
 }
 0x60ab205 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab227; continue; }
  pc = 0x60ab20e; continue;
 }
 0x60ab20e => {
  cpu.push(0x60ab213); f_060b3c01(cpu); if cpu.returned_to != 0x60ab213 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab227; continue; }
  pc = 0x60ab217; continue;
 }
 0x60ab217 => {
  cpu.push(0x60ab21c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab21c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab227; continue; }
  pc = 0x60ab220; continue;
 }
 0x60ab220 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab229; continue;
 }
 0x60ab227 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab229; continue;
 }
 0x60ab229 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab25b; continue; }
  pc = 0x60ab22d; continue;
 }
 0x60ab22d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169c68u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab253); cpu.call(t); if cpu.returned_to != 0x60ab253 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab25b); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab25b { return; }
  pc = 0x60ab25b; continue;
 }
 0x60ab25b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab268); f_060b5110(cpu); if cpu.returned_to != 0x60ab268 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab340); }
 let mut pc: u32 = 0x60ab340;
 loop { match pc {
 0x60ab340 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab387; continue; }
  pc = 0x60ab360; continue;
 }
 0x60ab360 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab36e; continue; }
  pc = 0x60ab365; continue;
 }
 0x60ab365 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab387; continue; }
  pc = 0x60ab36e; continue;
 }
 0x60ab36e => {
  cpu.push(0x60ab373); f_060b3c01(cpu); if cpu.returned_to != 0x60ab373 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab387; continue; }
  pc = 0x60ab377; continue;
 }
 0x60ab377 => {
  cpu.push(0x60ab37c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab37c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab387; continue; }
  pc = 0x60ab380; continue;
 }
 0x60ab380 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab389; continue;
 }
 0x60ab387 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab389; continue;
 }
 0x60ab389 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab3bb; continue; }
  pc = 0x60ab38d; continue;
 }
 0x60ab38d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169c34u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab3b3); cpu.call(t); if cpu.returned_to != 0x60ab3b3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab3bb); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab3bb { return; }
  pc = 0x60ab3bb; continue;
 }
 0x60ab3bb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab3c8); f_060b5190(cpu); if cpu.returned_to != 0x60ab3c8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ab3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab3f0); }
 let mut pc: u32 = 0x60ab3f0;
 loop { match pc {
 0x60ab3f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab437; continue; }
  pc = 0x60ab410; continue;
 }
 0x60ab410 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab41e; continue; }
  pc = 0x60ab415; continue;
 }
 0x60ab415 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab437; continue; }
  pc = 0x60ab41e; continue;
 }
 0x60ab41e => {
  cpu.push(0x60ab423); f_060b3c01(cpu); if cpu.returned_to != 0x60ab423 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab437; continue; }
  pc = 0x60ab427; continue;
 }
 0x60ab427 => {
  cpu.push(0x60ab42c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab42c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab437; continue; }
  pc = 0x60ab430; continue;
 }
 0x60ab430 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab439; continue;
 }
 0x60ab437 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab439; continue;
 }
 0x60ab439 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab487; continue; }
  pc = 0x60ab43d; continue;
 }
 0x60ab43d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169c10u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab47f); cpu.call(t); if cpu.returned_to != 0x60ab47f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab487); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab487 { return; }
  pc = 0x60ab487; continue;
 }
 0x60ab487 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab4a4); f_060b5330(cpu); if cpu.returned_to != 0x60ab4a4 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab4c0); }
 let mut pc: u32 = 0x60ab4c0;
 loop { match pc {
 0x60ab4c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab507; continue; }
  pc = 0x60ab4e0; continue;
 }
 0x60ab4e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab4ee; continue; }
  pc = 0x60ab4e5; continue;
 }
 0x60ab4e5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab507; continue; }
  pc = 0x60ab4ee; continue;
 }
 0x60ab4ee => {
  cpu.push(0x60ab4f3); f_060b3c01(cpu); if cpu.returned_to != 0x60ab4f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab507; continue; }
  pc = 0x60ab4f7; continue;
 }
 0x60ab4f7 => {
  cpu.push(0x60ab4fc); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab4fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab507; continue; }
  pc = 0x60ab500; continue;
 }
 0x60ab500 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab509; continue;
 }
 0x60ab507 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab509; continue;
 }
 0x60ab509 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab557; continue; }
  pc = 0x60ab50d; continue;
 }
 0x60ab50d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169be8u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab54f); cpu.call(t); if cpu.returned_to != 0x60ab54f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab557); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab557 { return; }
  pc = 0x60ab557; continue;
 }
 0x60ab557 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab574); f_060b5360(cpu); if cpu.returned_to != 0x60ab574 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab590); }
 let mut pc: u32 = 0x60ab590;
 loop { match pc {
 0x60ab590 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab5d7; continue; }
  pc = 0x60ab5b0; continue;
 }
 0x60ab5b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab5be; continue; }
  pc = 0x60ab5b5; continue;
 }
 0x60ab5b5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab5d7; continue; }
  pc = 0x60ab5be; continue;
 }
 0x60ab5be => {
  cpu.push(0x60ab5c3); f_060b3c01(cpu); if cpu.returned_to != 0x60ab5c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab5d7; continue; }
  pc = 0x60ab5c7; continue;
 }
 0x60ab5c7 => {
  cpu.push(0x60ab5cc); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab5cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab5d7; continue; }
  pc = 0x60ab5d0; continue;
 }
 0x60ab5d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab5d9; continue;
 }
 0x60ab5d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab5d9; continue;
 }
 0x60ab5d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab627; continue; }
  pc = 0x60ab5dd; continue;
 }
 0x60ab5dd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169bc4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab61f); cpu.call(t); if cpu.returned_to != 0x60ab61f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab627); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab627 { return; }
  pc = 0x60ab627; continue;
 }
 0x60ab627 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab644); f_060b6b50(cpu); if cpu.returned_to != 0x60ab644 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60ab692; continue; }
  pc = 0x60ab653; continue;
 }
 0x60ab653 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ab665; continue; }
  pc = 0x60ab65c; continue;
 }
 0x60ab65c => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab692; continue; }
  pc = 0x60ab665; continue;
 }
 0x60ab665 => {
  cpu.push(0x60ab66a); f_060b3c01(cpu); if cpu.returned_to != 0x60ab66a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab692; continue; }
  pc = 0x60ab66e; continue;
 }
 0x60ab66e => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab688); f_060b3e93(cpu); if cpu.returned_to != 0x60ab688 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ab692); f_060ad770(cpu); if cpu.returned_to != 0x60ab692 { return; }
  pc = 0x60ab692; continue;
 }
 0x60ab692 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab6b0); }
 let mut pc: u32 = 0x60ab6b0;
 loop { match pc {
 0x60ab6b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab6f7; continue; }
  pc = 0x60ab6d0; continue;
 }
 0x60ab6d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab6de; continue; }
  pc = 0x60ab6d5; continue;
 }
 0x60ab6d5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab6f7; continue; }
  pc = 0x60ab6de; continue;
 }
 0x60ab6de => {
  cpu.push(0x60ab6e3); f_060b3c01(cpu); if cpu.returned_to != 0x60ab6e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab6f7; continue; }
  pc = 0x60ab6e7; continue;
 }
 0x60ab6e7 => {
  cpu.push(0x60ab6ec); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab6ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab6f7; continue; }
  pc = 0x60ab6f0; continue;
 }
 0x60ab6f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab6f9; continue;
 }
 0x60ab6f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab6f9; continue;
 }
 0x60ab6f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab72b; continue; }
  pc = 0x60ab6fd; continue;
 }
 0x60ab6fd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169bacu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab723); cpu.call(t); if cpu.returned_to != 0x60ab723 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab72b); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab72b { return; }
  pc = 0x60ab72b; continue;
 }
 0x60ab72b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab738); f_060b53a0(cpu); if cpu.returned_to != 0x60ab738 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab760); }
 let mut pc: u32 = 0x60ab760;
 loop { match pc {
 0x60ab760 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab7a7; continue; }
  pc = 0x60ab780; continue;
 }
 0x60ab780 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab78e; continue; }
  pc = 0x60ab785; continue;
 }
 0x60ab785 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab7a7; continue; }
  pc = 0x60ab78e; continue;
 }
 0x60ab78e => {
  cpu.push(0x60ab793); f_060b3c01(cpu); if cpu.returned_to != 0x60ab793 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab7a7; continue; }
  pc = 0x60ab797; continue;
 }
 0x60ab797 => {
  cpu.push(0x60ab79c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab79c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab7a7; continue; }
  pc = 0x60ab7a0; continue;
 }
 0x60ab7a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab7a9; continue;
 }
 0x60ab7a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab7a9; continue;
 }
 0x60ab7a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab7db; continue; }
  pc = 0x60ab7ad; continue;
 }
 0x60ab7ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169b88u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab7d3); cpu.call(t); if cpu.returned_to != 0x60ab7d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab7db); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab7db { return; }
  pc = 0x60ab7db; continue;
 }
 0x60ab7db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab7e8); f_060b5490(cpu); if cpu.returned_to != 0x60ab7e8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ab810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab810); }
 let mut pc: u32 = 0x60ab810;
 loop { match pc {
 0x60ab810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab857; continue; }
  pc = 0x60ab830; continue;
 }
 0x60ab830 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab83e; continue; }
  pc = 0x60ab835; continue;
 }
 0x60ab835 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab857; continue; }
  pc = 0x60ab83e; continue;
 }
 0x60ab83e => {
  cpu.push(0x60ab843); f_060b3c01(cpu); if cpu.returned_to != 0x60ab843 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab857; continue; }
  pc = 0x60ab847; continue;
 }
 0x60ab847 => {
  cpu.push(0x60ab84c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab84c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab857; continue; }
  pc = 0x60ab850; continue;
 }
 0x60ab850 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab859; continue;
 }
 0x60ab857 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab859; continue;
 }
 0x60ab859 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab8a7; continue; }
  pc = 0x60ab85d; continue;
 }
 0x60ab85d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169b5cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab89f); cpu.call(t); if cpu.returned_to != 0x60ab89f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab8a7); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab8a7 { return; }
  pc = 0x60ab8a7; continue;
 }
 0x60ab8a7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab8c4); f_060b6ca0(cpu); if cpu.returned_to != 0x60ab8c4 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60ab912; continue; }
  pc = 0x60ab8d3; continue;
 }
 0x60ab8d3 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ab8e5; continue; }
  pc = 0x60ab8dc; continue;
 }
 0x60ab8dc => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab912; continue; }
  pc = 0x60ab8e5; continue;
 }
 0x60ab8e5 => {
  cpu.push(0x60ab8ea); f_060b3c01(cpu); if cpu.returned_to != 0x60ab8ea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab912; continue; }
  pc = 0x60ab8ee; continue;
 }
 0x60ab8ee => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab908); f_060b3e93(cpu); if cpu.returned_to != 0x60ab908 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ab912); f_060ad720(cpu); if cpu.returned_to != 0x60ab912 { return; }
  pc = 0x60ab912; continue;
 }
 0x60ab912 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_060ab930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ab930); }
 let mut pc: u32 = 0x60ab930;
 loop { match pc {
 0x60ab930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ab978; continue; }
  pc = 0x60ab951; continue;
 }
 0x60ab951 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ab95f; continue; }
  pc = 0x60ab956; continue;
 }
 0x60ab956 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ab978; continue; }
  pc = 0x60ab95f; continue;
 }
 0x60ab95f => {
  cpu.push(0x60ab964); f_060b3c01(cpu); if cpu.returned_to != 0x60ab964 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ab978; continue; }
  pc = 0x60ab968; continue;
 }
 0x60ab968 => {
  cpu.push(0x60ab96d); f_060ad3f0(cpu); if cpu.returned_to != 0x60ab96d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab978; continue; }
  pc = 0x60ab971; continue;
 }
 0x60ab971 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ab97a; continue;
 }
 0x60ab978 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ab97a; continue;
 }
 0x60ab97a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ab9ac; continue; }
  pc = 0x60ab97e; continue;
 }
 0x60ab97e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169b38u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ab9a4); cpu.call(t); if cpu.returned_to != 0x60ab9a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ab9ac); f_060ad7c0(cpu); if cpu.returned_to != 0x60ab9ac { return; }
  pc = 0x60ab9ac; continue;
 }
 0x60ab9ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ab9b9); f_060b54a0(cpu); if cpu.returned_to != 0x60ab9b9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60aba07; continue; }
  pc = 0x60ab9c8; continue;
 }
 0x60ab9c8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ab9da; continue; }
  pc = 0x60ab9d1; continue;
 }
 0x60ab9d1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aba07; continue; }
  pc = 0x60ab9da; continue;
 }
 0x60ab9da => {
  cpu.push(0x60ab9df); f_060b3c01(cpu); if cpu.returned_to != 0x60ab9df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aba07; continue; }
  pc = 0x60ab9e3; continue;
 }
 0x60ab9e3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ab9fd); f_060b3e93(cpu); if cpu.returned_to != 0x60ab9fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60aba07); f_060ad720(cpu); if cpu.returned_to != 0x60aba07 { return; }
  pc = 0x60aba07; continue;
 }
 0x60aba07 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060aba30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aba30); }
 let mut pc: u32 = 0x60aba30;
 loop { match pc {
 0x60aba30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60aba77; continue; }
  pc = 0x60aba50; continue;
 }
 0x60aba50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60aba5e; continue; }
  pc = 0x60aba55; continue;
 }
 0x60aba55 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60aba77; continue; }
  pc = 0x60aba5e; continue;
 }
 0x60aba5e => {
  cpu.push(0x60aba63); f_060b3c01(cpu); if cpu.returned_to != 0x60aba63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60aba77; continue; }
  pc = 0x60aba67; continue;
 }
 0x60aba67 => {
  cpu.push(0x60aba6c); f_060ad3f0(cpu); if cpu.returned_to != 0x60aba6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aba77; continue; }
  pc = 0x60aba70; continue;
 }
 0x60aba70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60aba79; continue;
 }
 0x60aba77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60aba79; continue;
 }
 0x60aba79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abaab; continue; }
  pc = 0x60aba7d; continue;
 }
 0x60aba7d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169b14u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60abaa3); cpu.call(t); if cpu.returned_to != 0x60abaa3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60abaab); f_060ad7c0(cpu); if cpu.returned_to != 0x60abaab { return; }
  pc = 0x60abaab; continue;
 }
 0x60abaab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60abab8); f_060b5550(cpu); if cpu.returned_to != 0x60abab8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060abae0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60abae0); }
 let mut pc: u32 = 0x60abae0;
 loop { match pc {
 0x60abae0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60abb28; continue; }
  pc = 0x60abb01; continue;
 }
 0x60abb01 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60abb0f; continue; }
  pc = 0x60abb06; continue;
 }
 0x60abb06 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abb28; continue; }
  pc = 0x60abb0f; continue;
 }
 0x60abb0f => {
  cpu.push(0x60abb14); f_060b3c01(cpu); if cpu.returned_to != 0x60abb14 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abb28; continue; }
  pc = 0x60abb18; continue;
 }
 0x60abb18 => {
  cpu.push(0x60abb1d); f_060ad3f0(cpu); if cpu.returned_to != 0x60abb1d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abb28; continue; }
  pc = 0x60abb21; continue;
 }
 0x60abb21 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60abb2a; continue;
 }
 0x60abb28 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60abb2a; continue;
 }
 0x60abb2a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abb6a; continue; }
  pc = 0x60abb2e; continue;
 }
 0x60abb2e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169aecu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60abb62); cpu.call(t); if cpu.returned_to != 0x60abb62 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60abb6a); f_060ad7c0(cpu); if cpu.returned_to != 0x60abb6a { return; }
  pc = 0x60abb6a; continue;
 }
 0x60abb6a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60abb7f); f_060b5630(cpu); if cpu.returned_to != 0x60abb7f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abbcc; continue; }
  pc = 0x60abb8d; continue;
 }
 0x60abb8d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60abb9f; continue; }
  pc = 0x60abb96; continue;
 }
 0x60abb96 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abbcc; continue; }
  pc = 0x60abb9f; continue;
 }
 0x60abb9f => {
  cpu.push(0x60abba4); f_060b3c01(cpu); if cpu.returned_to != 0x60abba4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abbcc; continue; }
  pc = 0x60abba8; continue;
 }
 0x60abba8 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60abbc2); f_060b3e93(cpu); if cpu.returned_to != 0x60abbc2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60abbcc); f_060ad720(cpu); if cpu.returned_to != 0x60abbcc { return; }
  pc = 0x60abbcc; continue;
 }
 0x60abbcc => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn o_060abbf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60abbf0); }
 let mut pc: u32 = 0x60abbf0;
 loop { match pc {
 0x60abbf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60abc38; continue; }
  pc = 0x60abc11; continue;
 }
 0x60abc11 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60abc1f; continue; }
  pc = 0x60abc16; continue;
 }
 0x60abc16 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abc38; continue; }
  pc = 0x60abc1f; continue;
 }
 0x60abc1f => {
  cpu.push(0x60abc24); f_060b3c01(cpu); if cpu.returned_to != 0x60abc24 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abc38; continue; }
  pc = 0x60abc28; continue;
 }
 0x60abc28 => {
  cpu.push(0x60abc2d); f_060ad3f0(cpu); if cpu.returned_to != 0x60abc2d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abc38; continue; }
  pc = 0x60abc31; continue;
 }
 0x60abc31 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60abc3a; continue;
 }
 0x60abc38 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60abc3a; continue;
 }
 0x60abc3a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abc6c; continue; }
  pc = 0x60abc3e; continue;
 }
 0x60abc3e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169ad0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60abc64); cpu.call(t); if cpu.returned_to != 0x60abc64 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60abc6c); f_060ad7c0(cpu); if cpu.returned_to != 0x60abc6c { return; }
  pc = 0x60abc6c; continue;
 }
 0x60abc6c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60abc79); f_060b5570(cpu); if cpu.returned_to != 0x60abc79 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60abcc7; continue; }
  pc = 0x60abc88; continue;
 }
 0x60abc88 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60abc9a; continue; }
  pc = 0x60abc91; continue;
 }
 0x60abc91 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abcc7; continue; }
  pc = 0x60abc9a; continue;
 }
 0x60abc9a => {
  cpu.push(0x60abc9f); f_060b3c01(cpu); if cpu.returned_to != 0x60abc9f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abcc7; continue; }
  pc = 0x60abca3; continue;
 }
 0x60abca3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60abcbd); f_060b3e93(cpu); if cpu.returned_to != 0x60abcbd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60abcc7); f_060ad770(cpu); if cpu.returned_to != 0x60abcc7 { return; }
  pc = 0x60abcc7; continue;
 }
 0x60abcc7 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060abcf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60abcf0); }
 let mut pc: u32 = 0x60abcf0;
 loop { match pc {
 0x60abcf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60abd37; continue; }
  pc = 0x60abd10; continue;
 }
 0x60abd10 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60abd1e; continue; }
  pc = 0x60abd15; continue;
 }
 0x60abd15 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abd37; continue; }
  pc = 0x60abd1e; continue;
 }
 0x60abd1e => {
  cpu.push(0x60abd23); f_060b3c01(cpu); if cpu.returned_to != 0x60abd23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abd37; continue; }
  pc = 0x60abd27; continue;
 }
 0x60abd27 => {
  cpu.push(0x60abd2c); f_060ad3f0(cpu); if cpu.returned_to != 0x60abd2c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abd37; continue; }
  pc = 0x60abd30; continue;
 }
 0x60abd30 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60abd39; continue;
 }
 0x60abd37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60abd39; continue;
 }
 0x60abd39 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abd6b; continue; }
  pc = 0x60abd3d; continue;
 }
 0x60abd3d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169ab0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60abd63); cpu.call(t); if cpu.returned_to != 0x60abd63 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60abd6b); f_060ad7c0(cpu); if cpu.returned_to != 0x60abd6b { return; }
  pc = 0x60abd6b; continue;
 }
 0x60abd6b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60abd78); f_060b4260(cpu); if cpu.returned_to != 0x60abd78 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060abda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60abda0); }
 let mut pc: u32 = 0x60abda0;
 loop { match pc {
 0x60abda0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60abde8; continue; }
  pc = 0x60abdc1; continue;
 }
 0x60abdc1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60abdcf; continue; }
  pc = 0x60abdc6; continue;
 }
 0x60abdc6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abde8; continue; }
  pc = 0x60abdcf; continue;
 }
 0x60abdcf => {
  cpu.push(0x60abdd4); f_060b3c01(cpu); if cpu.returned_to != 0x60abdd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abde8; continue; }
  pc = 0x60abdd8; continue;
 }
 0x60abdd8 => {
  cpu.push(0x60abddd); f_060ad3f0(cpu); if cpu.returned_to != 0x60abddd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abde8; continue; }
  pc = 0x60abde1; continue;
 }
 0x60abde1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60abdea; continue;
 }
 0x60abde8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60abdea; continue;
 }
 0x60abdea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abe2a; continue; }
  pc = 0x60abdee; continue;
 }
 0x60abdee => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169a90u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60abe22); cpu.call(t); if cpu.returned_to != 0x60abe22 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60abe2a); f_060ad7c0(cpu); if cpu.returned_to != 0x60abe2a { return; }
  pc = 0x60abe2a; continue;
 }
 0x60abe2a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60abe3f); f_060b4300(cpu); if cpu.returned_to != 0x60abe3f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abe8c; continue; }
  pc = 0x60abe4d; continue;
 }
 0x60abe4d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60abe5f; continue; }
  pc = 0x60abe56; continue;
 }
 0x60abe56 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abe8c; continue; }
  pc = 0x60abe5f; continue;
 }
 0x60abe5f => {
  cpu.push(0x60abe64); f_060b3c01(cpu); if cpu.returned_to != 0x60abe64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abe8c; continue; }
  pc = 0x60abe68; continue;
 }
 0x60abe68 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60abe82); f_060b3e93(cpu); if cpu.returned_to != 0x60abe82 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60abe8c); f_060ad720(cpu); if cpu.returned_to != 0x60abe8c { return; }
  pc = 0x60abe8c; continue;
 }
 0x60abe8c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060abeb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60abeb0); }
 let mut pc: u32 = 0x60abeb0;
 loop { match pc {
 0x60abeb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60abef8; continue; }
  pc = 0x60abed1; continue;
 }
 0x60abed1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60abedf; continue; }
  pc = 0x60abed6; continue;
 }
 0x60abed6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abef8; continue; }
  pc = 0x60abedf; continue;
 }
 0x60abedf => {
  cpu.push(0x60abee4); f_060b3c01(cpu); if cpu.returned_to != 0x60abee4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abef8; continue; }
  pc = 0x60abee8; continue;
 }
 0x60abee8 => {
  cpu.push(0x60abeed); f_060ad3f0(cpu); if cpu.returned_to != 0x60abeed { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abef8; continue; }
  pc = 0x60abef1; continue;
 }
 0x60abef1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60abefa; continue;
 }
 0x60abef8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60abefa; continue;
 }
 0x60abefa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abf2c; continue; }
  pc = 0x60abefe; continue;
 }
 0x60abefe => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169a70u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60abf24); cpu.call(t); if cpu.returned_to != 0x60abf24 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60abf2c); f_060ad7c0(cpu); if cpu.returned_to != 0x60abf2c { return; }
  pc = 0x60abf2c; continue;
 }
 0x60abf2c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60abf39); f_060b41b0(cpu); if cpu.returned_to != 0x60abf39 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60abf87; continue; }
  pc = 0x60abf48; continue;
 }
 0x60abf48 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60abf5a; continue; }
  pc = 0x60abf51; continue;
 }
 0x60abf51 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abf87; continue; }
  pc = 0x60abf5a; continue;
 }
 0x60abf5a => {
  cpu.push(0x60abf5f); f_060b3c01(cpu); if cpu.returned_to != 0x60abf5f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abf87; continue; }
  pc = 0x60abf63; continue;
 }
 0x60abf63 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60abf7d); f_060b3e93(cpu); if cpu.returned_to != 0x60abf7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60abf87); f_060ad720(cpu); if cpu.returned_to != 0x60abf87 { return; }
  pc = 0x60abf87; continue;
 }
 0x60abf87 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060abfb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60abfb0); }
 let mut pc: u32 = 0x60abfb0;
 loop { match pc {
 0x60abfb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60abff0; continue; }
  pc = 0x60abfc9; continue;
 }
 0x60abfc9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60abfd7; continue; }
  pc = 0x60abfce; continue;
 }
 0x60abfce => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60abff0; continue; }
  pc = 0x60abfd7; continue;
 }
 0x60abfd7 => {
  cpu.push(0x60abfdc); f_060b3c01(cpu); if cpu.returned_to != 0x60abfdc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60abff0; continue; }
  pc = 0x60abfe0; continue;
 }
 0x60abfe0 => {
  cpu.push(0x60abfe5); f_060ad3f0(cpu); if cpu.returned_to != 0x60abfe5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60abff0; continue; }
  pc = 0x60abfe9; continue;
 }
 0x60abfe9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60abff2; continue;
 }
 0x60abff0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60abff2; continue;
 }
 0x60abff2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac01c; continue; }
  pc = 0x60abff6; continue;
 }
 0x60abff6 => {
  { let v: u32 = 0x6169a54u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x6169a54u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac019); f_060b3e93(cpu); if cpu.returned_to != 0x60ac019 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x60ac01c; continue;
 }
 0x60ac01c => {
  cpu.push(0x60ac021); f_060b3c8b(cpu); if cpu.returned_to != 0x60ac021 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x62d12c4u32)); cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac030(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac030); }
 let mut pc: u32 = 0x60ac030;
 loop { match pc {
 0x60ac030 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac077; continue; }
  pc = 0x60ac050; continue;
 }
 0x60ac050 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac05e; continue; }
  pc = 0x60ac055; continue;
 }
 0x60ac055 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac077; continue; }
  pc = 0x60ac05e; continue;
 }
 0x60ac05e => {
  cpu.push(0x60ac063); f_060b3c01(cpu); if cpu.returned_to != 0x60ac063 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac077; continue; }
  pc = 0x60ac067; continue;
 }
 0x60ac067 => {
  cpu.push(0x60ac06c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac06c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac077; continue; }
  pc = 0x60ac070; continue;
 }
 0x60ac070 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac079; continue;
 }
 0x60ac077 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac079; continue;
 }
 0x60ac079 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac0ab; continue; }
  pc = 0x60ac07d; continue;
 }
 0x60ac07d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169a34u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac0a3); cpu.call(t); if cpu.returned_to != 0x60ac0a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac0ab); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac0ab { return; }
  pc = 0x60ac0ab; continue;
 }
 0x60ac0ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac0b8); f_060b3c6a(cpu); if cpu.returned_to != 0x60ac0b8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac0e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac0e0); }
 let mut pc: u32 = 0x60ac0e0;
 loop { match pc {
 0x60ac0e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac120; continue; }
  pc = 0x60ac0f9; continue;
 }
 0x60ac0f9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac107; continue; }
  pc = 0x60ac0fe; continue;
 }
 0x60ac0fe => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac120; continue; }
  pc = 0x60ac107; continue;
 }
 0x60ac107 => {
  cpu.push(0x60ac10c); f_060b3c01(cpu); if cpu.returned_to != 0x60ac10c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac120; continue; }
  pc = 0x60ac110; continue;
 }
 0x60ac110 => {
  cpu.push(0x60ac115); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac115 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac120; continue; }
  pc = 0x60ac119; continue;
 }
 0x60ac119 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac122; continue;
 }
 0x60ac120 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac122; continue;
 }
 0x60ac122 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac14c; continue; }
  pc = 0x60ac126; continue;
 }
 0x60ac126 => {
  { let v: u32 = 0x6169a1cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x6169a1cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac149); f_060b3e93(cpu); if cpu.returned_to != 0x60ac149 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x60ac14c; continue;
 }
 0x60ac14c => {
  cpu.push(0x60ac151); f_060b3d1f(cpu); if cpu.returned_to != 0x60ac151 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x62d12c4u32)); cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac160); }
 let mut pc: u32 = 0x60ac160;
 loop { match pc {
 0x60ac160 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac1a7; continue; }
  pc = 0x60ac180; continue;
 }
 0x60ac180 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac18e; continue; }
  pc = 0x60ac185; continue;
 }
 0x60ac185 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac1a7; continue; }
  pc = 0x60ac18e; continue;
 }
 0x60ac18e => {
  cpu.push(0x60ac193); f_060b3c01(cpu); if cpu.returned_to != 0x60ac193 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac1a7; continue; }
  pc = 0x60ac197; continue;
 }
 0x60ac197 => {
  cpu.push(0x60ac19c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac19c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac1a7; continue; }
  pc = 0x60ac1a0; continue;
 }
 0x60ac1a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac1a9; continue;
 }
 0x60ac1a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac1a9; continue;
 }
 0x60ac1a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac1db; continue; }
  pc = 0x60ac1ad; continue;
 }
 0x60ac1ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169a04u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac1d3); cpu.call(t); if cpu.returned_to != 0x60ac1d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac1db); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac1db { return; }
  pc = 0x60ac1db; continue;
 }
 0x60ac1db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac1e8); f_060b3cf5(cpu); if cpu.returned_to != 0x60ac1e8 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac210); }
 let mut pc: u32 = 0x60ac210;
 loop { match pc {
 0x60ac210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac250; continue; }
  pc = 0x60ac229; continue;
 }
 0x60ac229 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac237; continue; }
  pc = 0x60ac22e; continue;
 }
 0x60ac22e => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac250; continue; }
  pc = 0x60ac237; continue;
 }
 0x60ac237 => {
  cpu.push(0x60ac23c); f_060b3c01(cpu); if cpu.returned_to != 0x60ac23c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac250; continue; }
  pc = 0x60ac240; continue;
 }
 0x60ac240 => {
  cpu.push(0x60ac245); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac245 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac250; continue; }
  pc = 0x60ac249; continue;
 }
 0x60ac249 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac252; continue;
 }
 0x60ac250 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac252; continue;
 }
 0x60ac252 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac27c; continue; }
  pc = 0x60ac256; continue;
 }
 0x60ac256 => {
  { let v: u32 = 0x61699e8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x61699e8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac279); f_060b3e93(cpu); if cpu.returned_to != 0x60ac279 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x60ac27c; continue;
 }
 0x60ac27c => {
  cpu.push(0x60ac281); f_060b3cd5(cpu); if cpu.returned_to != 0x60ac281 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x62d12c4u32)); cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac290); }
 let mut pc: u32 = 0x60ac290;
 loop { match pc {
 0x60ac290 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac2d7; continue; }
  pc = 0x60ac2b0; continue;
 }
 0x60ac2b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac2be; continue; }
  pc = 0x60ac2b5; continue;
 }
 0x60ac2b5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac2d7; continue; }
  pc = 0x60ac2be; continue;
 }
 0x60ac2be => {
  cpu.push(0x60ac2c3); f_060b3c01(cpu); if cpu.returned_to != 0x60ac2c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac2d7; continue; }
  pc = 0x60ac2c7; continue;
 }
 0x60ac2c7 => {
  cpu.push(0x60ac2cc); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac2cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac2d7; continue; }
  pc = 0x60ac2d0; continue;
 }
 0x60ac2d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac2d9; continue;
 }
 0x60ac2d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac2d9; continue;
 }
 0x60ac2d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac30b; continue; }
  pc = 0x60ac2dd; continue;
 }
 0x60ac2dd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61699d0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac303); cpu.call(t); if cpu.returned_to != 0x60ac303 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac30b); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac30b { return; }
  pc = 0x60ac30b; continue;
 }
 0x60ac30b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac318); f_060b3cab(cpu); if cpu.returned_to != 0x60ac318 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac340); }
 let mut pc: u32 = 0x60ac340;
 loop { match pc {
 0x60ac340 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac382; continue; }
  pc = 0x60ac35b; continue;
 }
 0x60ac35b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac369; continue; }
  pc = 0x60ac360; continue;
 }
 0x60ac360 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac382; continue; }
  pc = 0x60ac369; continue;
 }
 0x60ac369 => {
  cpu.push(0x60ac36e); f_060b3c01(cpu); if cpu.returned_to != 0x60ac36e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac382; continue; }
  pc = 0x60ac372; continue;
 }
 0x60ac372 => {
  cpu.push(0x60ac377); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac377 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac382; continue; }
  pc = 0x60ac37b; continue;
 }
 0x60ac37b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac384; continue;
 }
 0x60ac382 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac384; continue;
 }
 0x60ac384 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac3ae; continue; }
  pc = 0x60ac388; continue;
 }
 0x60ac388 => {
  { let v: u32 = 0x61699b4u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x61699b4u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac3ab); f_060b3e93(cpu); if cpu.returned_to != 0x60ac3ab { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60ac3ae; continue;
 }
 0x60ac3ae => {
  cpu.push(0x60ac3b3); f_060b3dd5(cpu); if cpu.returned_to != 0x60ac3b3 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60ac3fe; continue; }
  pc = 0x60ac3bf; continue;
 }
 0x60ac3bf => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ac3d1; continue; }
  pc = 0x60ac3c8; continue;
 }
 0x60ac3c8 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac3fe; continue; }
  pc = 0x60ac3d1; continue;
 }
 0x60ac3d1 => {
  cpu.push(0x60ac3d6); f_060b3c01(cpu); if cpu.returned_to != 0x60ac3d6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac3fe; continue; }
  pc = 0x60ac3da; continue;
 }
 0x60ac3da => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ac3f4); f_060b3e93(cpu); if cpu.returned_to != 0x60ac3f4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ac3fe); f_060ad770(cpu); if cpu.returned_to != 0x60ac3fe { return; }
  pc = 0x60ac3fe; continue;
 }
 0x60ac3fe => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac4e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac4e0); }
 let mut pc: u32 = 0x60ac4e0;
 loop { match pc {
 0x60ac4e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac527; continue; }
  pc = 0x60ac500; continue;
 }
 0x60ac500 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac50e; continue; }
  pc = 0x60ac505; continue;
 }
 0x60ac505 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac527; continue; }
  pc = 0x60ac50e; continue;
 }
 0x60ac50e => {
  cpu.push(0x60ac513); f_060b3c01(cpu); if cpu.returned_to != 0x60ac513 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac527; continue; }
  pc = 0x60ac517; continue;
 }
 0x60ac517 => {
  cpu.push(0x60ac51c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac51c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac527; continue; }
  pc = 0x60ac520; continue;
 }
 0x60ac520 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac529; continue;
 }
 0x60ac527 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac529; continue;
 }
 0x60ac529 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac569; continue; }
  pc = 0x60ac52d; continue;
 }
 0x60ac52d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169970u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac561); cpu.call(t); if cpu.returned_to != 0x60ac561 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac569); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac569 { return; }
  pc = 0x60ac569; continue;
 }
 0x60ac569 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ac57e); f_060b3d6f(cpu); if cpu.returned_to != 0x60ac57e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac5a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac5a0); }
 let mut pc: u32 = 0x60ac5a0;
 loop { match pc {
 0x60ac5a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac5e7; continue; }
  pc = 0x60ac5c0; continue;
 }
 0x60ac5c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac5ce; continue; }
  pc = 0x60ac5c5; continue;
 }
 0x60ac5c5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac5e7; continue; }
  pc = 0x60ac5ce; continue;
 }
 0x60ac5ce => {
  cpu.push(0x60ac5d3); f_060b3c01(cpu); if cpu.returned_to != 0x60ac5d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac5e7; continue; }
  pc = 0x60ac5d7; continue;
 }
 0x60ac5d7 => {
  cpu.push(0x60ac5dc); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac5dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac5e7; continue; }
  pc = 0x60ac5e0; continue;
 }
 0x60ac5e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac5e9; continue;
 }
 0x60ac5e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac5e9; continue;
 }
 0x60ac5e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac629; continue; }
  pc = 0x60ac5ed; continue;
 }
 0x60ac5ed => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169950u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac621); cpu.call(t); if cpu.returned_to != 0x60ac621 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac629); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac629 { return; }
  pc = 0x60ac629; continue;
 }
 0x60ac629 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ac63e); f_060b3d3f(cpu); if cpu.returned_to != 0x60ac63e { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ac660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac660); }
 let mut pc: u32 = 0x60ac660;
 loop { match pc {
 0x60ac660 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac6a8; continue; }
  pc = 0x60ac681; continue;
 }
 0x60ac681 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac68f; continue; }
  pc = 0x60ac686; continue;
 }
 0x60ac686 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac6a8; continue; }
  pc = 0x60ac68f; continue;
 }
 0x60ac68f => {
  cpu.push(0x60ac694); f_060b3c01(cpu); if cpu.returned_to != 0x60ac694 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac6a8; continue; }
  pc = 0x60ac698; continue;
 }
 0x60ac698 => {
  cpu.push(0x60ac69d); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac69d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac6a8; continue; }
  pc = 0x60ac6a1; continue;
 }
 0x60ac6a1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac6aa; continue;
 }
 0x60ac6a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac6aa; continue;
 }
 0x60ac6aa => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac6ea; continue; }
  pc = 0x60ac6ae; continue;
 }
 0x60ac6ae => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169934u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac6e2); cpu.call(t); if cpu.returned_to != 0x60ac6e2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac6ea); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac6ea { return; }
  pc = 0x60ac6ea; continue;
 }
 0x60ac6ea => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ac6ff); f_060b3c48(cpu); if cpu.returned_to != 0x60ac6ff { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac74c; continue; }
  pc = 0x60ac70d; continue;
 }
 0x60ac70d => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ac71f; continue; }
  pc = 0x60ac716; continue;
 }
 0x60ac716 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac74c; continue; }
  pc = 0x60ac71f; continue;
 }
 0x60ac71f => {
  cpu.push(0x60ac724); f_060b3c01(cpu); if cpu.returned_to != 0x60ac724 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac74c; continue; }
  pc = 0x60ac728; continue;
 }
 0x60ac728 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ac742); f_060b3e93(cpu); if cpu.returned_to != 0x60ac742 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ac74c); f_060ad770(cpu); if cpu.returned_to != 0x60ac74c { return; }
  pc = 0x60ac74c; continue;
 }
 0x60ac74c => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060ac770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac770); }
 let mut pc: u32 = 0x60ac770;
 loop { match pc {
 0x60ac770 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac7b8; continue; }
  pc = 0x60ac791; continue;
 }
 0x60ac791 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac79f; continue; }
  pc = 0x60ac796; continue;
 }
 0x60ac796 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac7b8; continue; }
  pc = 0x60ac79f; continue;
 }
 0x60ac79f => {
  cpu.push(0x60ac7a4); f_060b3c01(cpu); if cpu.returned_to != 0x60ac7a4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac7b8; continue; }
  pc = 0x60ac7a8; continue;
 }
 0x60ac7a8 => {
  cpu.push(0x60ac7ad); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac7ad { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac7b8; continue; }
  pc = 0x60ac7b1; continue;
 }
 0x60ac7b1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac7ba; continue;
 }
 0x60ac7b8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac7ba; continue;
 }
 0x60ac7ba => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac7ec; continue; }
  pc = 0x60ac7be; continue;
 }
 0x60ac7be => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169918u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac7e4); cpu.call(t); if cpu.returned_to != 0x60ac7e4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac7ec); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac7ec { return; }
  pc = 0x60ac7ec; continue;
 }
 0x60ac7ec => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ac7f9); f_060b3c07(cpu); if cpu.returned_to != 0x60ac7f9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60ac847; continue; }
  pc = 0x60ac808; continue;
 }
 0x60ac808 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ac81a; continue; }
  pc = 0x60ac811; continue;
 }
 0x60ac811 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac847; continue; }
  pc = 0x60ac81a; continue;
 }
 0x60ac81a => {
  cpu.push(0x60ac81f); f_060b3c01(cpu); if cpu.returned_to != 0x60ac81f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac847; continue; }
  pc = 0x60ac823; continue;
 }
 0x60ac823 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ac83d); f_060b3e93(cpu); if cpu.returned_to != 0x60ac83d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ac847); f_060ad770(cpu); if cpu.returned_to != 0x60ac847 { return; }
  pc = 0x60ac847; continue;
 }
 0x60ac847 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060ac870(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ac870); }
 let mut pc: u32 = 0x60ac870;
 loop { match pc {
 0x60ac870 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3a0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3b4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3b8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ac8c6; continue; }
  pc = 0x60ac89f; continue;
 }
 0x60ac89f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ac8ad; continue; }
  pc = 0x60ac8a4; continue;
 }
 0x60ac8a4 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ac8c6; continue; }
  pc = 0x60ac8ad; continue;
 }
 0x60ac8ad => {
  cpu.push(0x60ac8b2); f_060b3c01(cpu); if cpu.returned_to != 0x60ac8b2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ac8c6; continue; }
  pc = 0x60ac8b6; continue;
 }
 0x60ac8b6 => {
  cpu.push(0x60ac8bb); f_060ad3f0(cpu); if cpu.returned_to != 0x60ac8bb { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac8c6; continue; }
  pc = 0x60ac8bf; continue;
 }
 0x60ac8bf => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ac8c8; continue;
 }
 0x60ac8c6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ac8c8; continue;
 }
 0x60ac8c8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ac904; continue; }
  pc = 0x60ac8cc; continue;
 }
 0x60ac8cc => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616989cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x38cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac8f7); cpu.call(t); if cpu.returned_to != 0x60ac8f7 { return; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac904); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac904 { return; }
  pc = 0x60ac904; continue;
 }
 0x60ac904 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ac90b); f_060b4610(cpu); if cpu.returned_to != 0x60ac90b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acb3d; continue; }
  pc = 0x60ac918; continue;
 }
 0x60ac918 => {
  { cpu.sub32(cpu.r32(0x62d12c8u32), 0x0u32); }
  if cpu.zf { pc = 0x60acb85; continue; }
  pc = 0x60ac925; continue;
 }
 0x60ac925 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ac93b; continue; }
  pc = 0x60ac92e; continue;
 }
 0x60ac92e => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acb85; continue; }
  pc = 0x60ac93b; continue;
 }
 0x60ac93b => {
  cpu.push(0x60ac940); f_060b3c01(cpu); if cpu.returned_to != 0x60ac940 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acb85; continue; }
  pc = 0x60ac948; continue;
 }
 0x60ac948 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60ac962); f_060b3e93(cpu); if cpu.returned_to != 0x60ac962 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x388u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61698b8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x394u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac98f); cpu.call(t); if cpu.returned_to != 0x60ac98f { return; }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac99c); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac99c { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ac9b6); f_060b3e93(cpu); if cpu.returned_to != 0x60ac9b6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61698c8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3acu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ac9d8); cpu.call(t); if cpu.returned_to != 0x60ac9d8 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ac9e0); f_060ad7c0(cpu); if cpu.returned_to != 0x60ac9e0 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ac9fa); f_060b3e93(cpu); if cpu.returned_to != 0x60ac9fa { return; }
  { let v: u32 = cpu.r32(0xfeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61698d8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x398u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aca2a); cpu.call(t); if cpu.returned_to != 0x60aca2a { return; }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aca37); f_060ad7c0(cpu); if cpu.returned_to != 0x60aca37 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aca50); f_060b3e93(cpu); if cpu.returned_to != 0x60aca50 { return; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61698e8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x39cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aca80); cpu.call(t); if cpu.returned_to != 0x60aca80 { return; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60aca8d); f_060ad7c0(cpu); if cpu.returned_to != 0x60aca8d { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60acaa7); f_060b3e93(cpu); if cpu.returned_to != 0x60acaa7 { return; }
  { let v: u32 = cpu.r32(0x102u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61698f8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60acad7); cpu.call(t); if cpu.returned_to != 0x60acad7 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acae4); f_060ad7c0(cpu); if cpu.returned_to != 0x60acae4 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60acafe); f_060b3e93(cpu); if cpu.returned_to != 0x60acafe { return; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x398u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169908u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60acb2e); cpu.call(t); if cpu.returned_to != 0x60acb2e { return; }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acb3b); f_060ad7c0(cpu); if cpu.returned_to != 0x60acb3b { return; }
  pc = 0x60acb85; continue;
 }
 0x60acb3d => {
  { cpu.sub32(cpu.r32(0x62d12c8u32), 0x0u32); }
  if cpu.zf { pc = 0x60acb85; continue; }
  pc = 0x60acb46; continue;
 }
 0x60acb46 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60acb58; continue; }
  pc = 0x60acb4f; continue;
 }
 0x60acb4f => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acb85; continue; }
  pc = 0x60acb58; continue;
 }
 0x60acb58 => {
  cpu.push(0x60acb5d); f_060b3c01(cpu); if cpu.returned_to != 0x60acb5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acb85; continue; }
  pc = 0x60acb61; continue;
 }
 0x60acb61 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60acb7b); f_060b3e93(cpu); if cpu.returned_to != 0x60acb7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60acb85); f_060ad770(cpu); if cpu.returned_to != 0x60acb85 { return; }
  pc = 0x60acb85; continue;
 }
 0x60acb85 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060acba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acba0); }
 let mut pc: u32 = 0x60acba0;
 loop { match pc {
 0x60acba0 => {
  f_060b3c01(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060acbb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acbb0); }
 let mut pc: u32 = 0x60acbb0;
 loop { match pc {
 0x60acbb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60acbf7; continue; }
  pc = 0x60acbd0; continue;
 }
 0x60acbd0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60acbde; continue; }
  pc = 0x60acbd5; continue;
 }
 0x60acbd5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acbf7; continue; }
  pc = 0x60acbde; continue;
 }
 0x60acbde => {
  cpu.push(0x60acbe3); f_060b3c01(cpu); if cpu.returned_to != 0x60acbe3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acbf7; continue; }
  pc = 0x60acbe7; continue;
 }
 0x60acbe7 => {
  cpu.push(0x60acbec); f_060ad3f0(cpu); if cpu.returned_to != 0x60acbec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acbf7; continue; }
  pc = 0x60acbf0; continue;
 }
 0x60acbf0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60acbf9; continue;
 }
 0x60acbf7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60acbf9; continue;
 }
 0x60acbf9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acc2b; continue; }
  pc = 0x60acbfd; continue;
 }
 0x60acbfd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616988cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60acc23); cpu.call(t); if cpu.returned_to != 0x60acc23 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acc2b); f_060ad7c0(cpu); if cpu.returned_to != 0x60acc2b { return; }
  pc = 0x60acc2b; continue;
 }
 0x60acc2b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60acc38); f_060b3fd0(cpu); if cpu.returned_to != 0x60acc38 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060acc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acc60); }
 let mut pc: u32 = 0x60acc60;
 loop { match pc {
 0x60acc60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60acca7; continue; }
  pc = 0x60acc80; continue;
 }
 0x60acc80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60acc8e; continue; }
  pc = 0x60acc85; continue;
 }
 0x60acc85 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acca7; continue; }
  pc = 0x60acc8e; continue;
 }
 0x60acc8e => {
  cpu.push(0x60acc93); f_060b3c01(cpu); if cpu.returned_to != 0x60acc93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acca7; continue; }
  pc = 0x60acc97; continue;
 }
 0x60acc97 => {
  cpu.push(0x60acc9c); f_060ad3f0(cpu); if cpu.returned_to != 0x60acc9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acca7; continue; }
  pc = 0x60acca0; continue;
 }
 0x60acca0 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x60acca9; continue;
 }
 0x60acca7 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60acca9; continue;
 }
 0x60acca9 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60acd05; continue; }
  pc = 0x60accad; continue;
 }
 0x60accad => {
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
  { let v = 0x6169864u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60accfd); cpu.call(t); if cpu.returned_to != 0x60accfd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acd05); f_060ad7c0(cpu); if cpu.returned_to != 0x60acd05 { return; }
  pc = 0x60acd05; continue;
 }
 0x60acd05 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60acd2a); f_060b3b27(cpu); if cpu.returned_to != 0x60acd2a { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60acd78; continue; }
  pc = 0x60acd39; continue;
 }
 0x60acd39 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60acd4b; continue; }
  pc = 0x60acd42; continue;
 }
 0x60acd42 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acd78; continue; }
  pc = 0x60acd4b; continue;
 }
 0x60acd4b => {
  cpu.push(0x60acd50); f_060b3c01(cpu); if cpu.returned_to != 0x60acd50 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acd78; continue; }
  pc = 0x60acd54; continue;
 }
 0x60acd54 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60acd6e); f_060b3e93(cpu); if cpu.returned_to != 0x60acd6e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60acd78); f_060ad770(cpu); if cpu.returned_to != 0x60acd78 { return; }
  pc = 0x60acd78; continue;
 }
 0x60acd78 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x62d12c4u32)); cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060acd90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acd90); }
 let mut pc: u32 = 0x60acd90;
 loop { match pc {
 0x60acd90 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60acd9a); f_060b3ac0(cpu); if cpu.returned_to != 0x60acd9a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060acda0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acda0); }
 let mut pc: u32 = 0x60acda0;
 loop { match pc {
 0x60acda0 => {
  pc = 0x60b3abc; continue;
 }
 0x60b3abc => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060acdb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acdb0); }
 let mut pc: u32 = 0x60acdb0;
 loop { match pc {
 0x60acdb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60acdf7; continue; }
  pc = 0x60acdd0; continue;
 }
 0x60acdd0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60acdde; continue; }
  pc = 0x60acdd5; continue;
 }
 0x60acdd5 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acdf7; continue; }
  pc = 0x60acdde; continue;
 }
 0x60acdde => {
  cpu.push(0x60acde3); f_060b3c01(cpu); if cpu.returned_to != 0x60acde3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acdf7; continue; }
  pc = 0x60acde7; continue;
 }
 0x60acde7 => {
  cpu.push(0x60acdec); f_060ad3f0(cpu); if cpu.returned_to != 0x60acdec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acdf7; continue; }
  pc = 0x60acdf0; continue;
 }
 0x60acdf0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60acdf9; continue;
 }
 0x60acdf7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60acdf9; continue;
 }
 0x60acdf9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ace2b; continue; }
  pc = 0x60acdfd; continue;
 }
 0x60acdfd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169848u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ace23); cpu.call(t); if cpu.returned_to != 0x60ace23 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ace2b); f_060ad7c0(cpu); if cpu.returned_to != 0x60ace2b { return; }
  pc = 0x60ace2b; continue;
 }
 0x60ace2b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ace38); f_060b3a77(cpu); if cpu.returned_to != 0x60ace38 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ace60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ace60); }
 let mut pc: u32 = 0x60ace60;
 loop { match pc {
 0x60ace60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60acea7; continue; }
  pc = 0x60ace80; continue;
 }
 0x60ace80 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ace8e; continue; }
  pc = 0x60ace85; continue;
 }
 0x60ace85 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acea7; continue; }
  pc = 0x60ace8e; continue;
 }
 0x60ace8e => {
  cpu.push(0x60ace93); f_060b3c01(cpu); if cpu.returned_to != 0x60ace93 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acea7; continue; }
  pc = 0x60ace97; continue;
 }
 0x60ace97 => {
  cpu.push(0x60ace9c); f_060ad3f0(cpu); if cpu.returned_to != 0x60ace9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acea7; continue; }
  pc = 0x60acea0; continue;
 }
 0x60acea0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60acea9; continue;
 }
 0x60acea7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60acea9; continue;
 }
 0x60acea9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acef7; continue; }
  pc = 0x60acead; continue;
 }
 0x60acead => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169828u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60aceef); cpu.call(t); if cpu.returned_to != 0x60aceef { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acef7); f_060ad7c0(cpu); if cpu.returned_to != 0x60acef7 { return; }
  pc = 0x60acef7; continue;
 }
 0x60acef7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acf14); f_060b3861(cpu); if cpu.returned_to != 0x60acf14 { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060acf30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acf30); }
 let mut pc: u32 = 0x60acf30;
 loop { match pc {
 0x60acf30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60acf77; continue; }
  pc = 0x60acf50; continue;
 }
 0x60acf50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60acf5e; continue; }
  pc = 0x60acf55; continue;
 }
 0x60acf55 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60acf77; continue; }
  pc = 0x60acf5e; continue;
 }
 0x60acf5e => {
  cpu.push(0x60acf63); f_060b3c01(cpu); if cpu.returned_to != 0x60acf63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60acf77; continue; }
  pc = 0x60acf67; continue;
 }
 0x60acf67 => {
  cpu.push(0x60acf6c); f_060ad3f0(cpu); if cpu.returned_to != 0x60acf6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acf77; continue; }
  pc = 0x60acf70; continue;
 }
 0x60acf70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60acf79; continue;
 }
 0x60acf77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60acf79; continue;
 }
 0x60acf79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60acfb9; continue; }
  pc = 0x60acf7d; continue;
 }
 0x60acf7d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169808u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60acfb1); cpu.call(t); if cpu.returned_to != 0x60acfb1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60acfb9); f_060ad7c0(cpu); if cpu.returned_to != 0x60acfb9 { return; }
  pc = 0x60acfb9; continue;
 }
 0x60acfb9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60acfce); f_060b37e9(cpu); if cpu.returned_to != 0x60acfce { return; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060acff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60acff0); }
 let mut pc: u32 = 0x60acff0;
 loop { match pc {
 0x60acff0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ad038; continue; }
  pc = 0x60ad011; continue;
 }
 0x60ad011 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ad01f; continue; }
  pc = 0x60ad016; continue;
 }
 0x60ad016 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad038; continue; }
  pc = 0x60ad01f; continue;
 }
 0x60ad01f => {
  cpu.push(0x60ad024); f_060b3c01(cpu); if cpu.returned_to != 0x60ad024 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad038; continue; }
  pc = 0x60ad028; continue;
 }
 0x60ad028 => {
  cpu.push(0x60ad02d); f_060ad3f0(cpu); if cpu.returned_to != 0x60ad02d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad038; continue; }
  pc = 0x60ad031; continue;
 }
 0x60ad031 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ad03a; continue;
 }
 0x60ad038 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ad03a; continue;
 }
 0x60ad03a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad06c; continue; }
  pc = 0x60ad03e; continue;
 }
 0x60ad03e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61697ecu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad064); cpu.call(t); if cpu.returned_to != 0x60ad064 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ad06c); f_060ad7c0(cpu); if cpu.returned_to != 0x60ad06c { return; }
  pc = 0x60ad06c; continue;
 }
 0x60ad06c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ad079); f_060b37c1(cpu); if cpu.returned_to != 0x60ad079 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60ad0c7; continue; }
  pc = 0x60ad088; continue;
 }
 0x60ad088 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ad09a; continue; }
  pc = 0x60ad091; continue;
 }
 0x60ad091 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad0c7; continue; }
  pc = 0x60ad09a; continue;
 }
 0x60ad09a => {
  cpu.push(0x60ad09f); f_060b3c01(cpu); if cpu.returned_to != 0x60ad09f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad0c7; continue; }
  pc = 0x60ad0a3; continue;
 }
 0x60ad0a3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ad0bd); f_060b3e93(cpu); if cpu.returned_to != 0x60ad0bd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ad0c7); f_060ad720(cpu); if cpu.returned_to != 0x60ad0c7 { return; }
  pc = 0x60ad0c7; continue;
 }
 0x60ad0c7 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060ad0f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad0f0); }
 let mut pc: u32 = 0x60ad0f0;
 loop { match pc {
 0x60ad0f0 => {
  pc = 0x60b3357; continue;
 }
 0x60b3357 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x617bb0au32)); cpu.w32(0x617bb0au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad100(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60ad100); } r_060ad100(cpu, 0x60ad100); }
pub fn f_060b3350(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3350); } r_060ad100(cpu, 0x60b3350); }
fn r_060ad100(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60ad100 => {
  pc = 0x60b3350; continue;
 }
 0x60b3350 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x617bb0au32)); cpu.w32(0x617bb0au32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad110); }
 let mut pc: u32 = 0x60ad110;
 loop { match pc {
 0x60ad110 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ad158; continue; }
  pc = 0x60ad131; continue;
 }
 0x60ad131 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ad13f; continue; }
  pc = 0x60ad136; continue;
 }
 0x60ad136 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad158; continue; }
  pc = 0x60ad13f; continue;
 }
 0x60ad13f => {
  cpu.push(0x60ad144); f_060b3c01(cpu); if cpu.returned_to != 0x60ad144 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad158; continue; }
  pc = 0x60ad148; continue;
 }
 0x60ad148 => {
  cpu.push(0x60ad14d); f_060ad3f0(cpu); if cpu.returned_to != 0x60ad14d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad158; continue; }
  pc = 0x60ad151; continue;
 }
 0x60ad151 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ad15a; continue;
 }
 0x60ad158 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ad15a; continue;
 }
 0x60ad15a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad19a; continue; }
  pc = 0x60ad15e; continue;
 }
 0x60ad15e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61697d0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad192); cpu.call(t); if cpu.returned_to != 0x60ad192 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ad19a); f_060ad7c0(cpu); if cpu.returned_to != 0x60ad19a { return; }
  pc = 0x60ad19a; continue;
 }
 0x60ad19a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ad1af); f_060b45f0(cpu); if cpu.returned_to != 0x60ad1af { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad1fc; continue; }
  pc = 0x60ad1bd; continue;
 }
 0x60ad1bd => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ad1cf; continue; }
  pc = 0x60ad1c6; continue;
 }
 0x60ad1c6 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad1fc; continue; }
  pc = 0x60ad1cf; continue;
 }
 0x60ad1cf => {
  cpu.push(0x60ad1d4); f_060b3c01(cpu); if cpu.returned_to != 0x60ad1d4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad1fc; continue; }
  pc = 0x60ad1d8; continue;
 }
 0x60ad1d8 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ad1f2); f_060b3e93(cpu); if cpu.returned_to != 0x60ad1f2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ad1fc); f_060ad770(cpu); if cpu.returned_to != 0x60ad1fc { return; }
  pc = 0x60ad1fc; continue;
 }
 0x60ad1fc => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060ad220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad220); }
 let mut pc: u32 = 0x60ad220;
 loop { match pc {
 0x60ad220 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ad268; continue; }
  pc = 0x60ad241; continue;
 }
 0x60ad241 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60ad24f; continue; }
  pc = 0x60ad246; continue;
 }
 0x60ad246 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad268; continue; }
  pc = 0x60ad24f; continue;
 }
 0x60ad24f => {
  cpu.push(0x60ad254); f_060b3c01(cpu); if cpu.returned_to != 0x60ad254 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad268; continue; }
  pc = 0x60ad258; continue;
 }
 0x60ad258 => {
  cpu.push(0x60ad25d); f_060ad3f0(cpu); if cpu.returned_to != 0x60ad25d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad268; continue; }
  pc = 0x60ad261; continue;
 }
 0x60ad261 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ad26a; continue;
 }
 0x60ad268 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ad26a; continue;
 }
 0x60ad26a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad29c; continue; }
  pc = 0x60ad26e; continue;
 }
 0x60ad26e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x61697b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad294); cpu.call(t); if cpu.returned_to != 0x60ad294 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ad29c); f_060ad7c0(cpu); if cpu.returned_to != 0x60ad29c { return; }
  pc = 0x60ad29c; continue;
 }
 0x60ad29c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ad2a9); f_060b3ff0(cpu); if cpu.returned_to != 0x60ad2a9 { return; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60ad2f7; continue; }
  pc = 0x60ad2b8; continue;
 }
 0x60ad2b8 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ad2ca; continue; }
  pc = 0x60ad2c1; continue;
 }
 0x60ad2c1 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad2f7; continue; }
  pc = 0x60ad2ca; continue;
 }
 0x60ad2ca => {
  cpu.push(0x60ad2cf); f_060b3c01(cpu); if cpu.returned_to != 0x60ad2cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad2f7; continue; }
  pc = 0x60ad2d3; continue;
 }
 0x60ad2d3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x617a3f8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ad2ed); f_060b3e93(cpu); if cpu.returned_to != 0x60ad2ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ad2f7); f_060ad720(cpu); if cpu.returned_to != 0x60ad2f7 { return; }
  pc = 0x60ad2f7; continue;
 }
 0x60ad2f7 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d12c4u32, v); }
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
pub fn f_060ad320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad320); }
 let mut pc: u32 = 0x60ad320;
 loop { match pc {
 0x60ad320 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617a3f0u32), 0x0u32); }
  if cpu.zf { pc = 0x60ad3e8; continue; }
  pc = 0x60ad331; continue;
 }
 0x60ad331 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d12c8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d12c4u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60ad36f; continue; }
  pc = 0x60ad348; continue;
 }
 0x60ad348 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x60ad356; continue; }
  pc = 0x60ad34d; continue;
 }
 0x60ad34d => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad36f; continue; }
  pc = 0x60ad356; continue;
 }
 0x60ad356 => {
  cpu.push(0x60ad35b); f_060b3c01(cpu); if cpu.returned_to != 0x60ad35b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad36f; continue; }
  pc = 0x60ad35f; continue;
 }
 0x60ad35f => {
  cpu.push(0x60ad364); f_060ad3f0(cpu); if cpu.returned_to != 0x60ad364 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad36f; continue; }
  pc = 0x60ad368; continue;
 }
 0x60ad368 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ad371; continue;
 }
 0x60ad36f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ad371; continue;
 }
 0x60ad371 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad39b; continue; }
  pc = 0x60ad375; continue;
 }
 0x60ad375 => {
  { let v: u32 = 0x616979cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x616979cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ad398); f_060b3e93(cpu); if cpu.returned_to != 0x60ad398 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60ad39b; continue;
 }
 0x60ad39b => {
  cpu.push(0x60ad3a0); f_060b4060(cpu); if cpu.returned_to != 0x60ad3a0 { return; }
  { cpu.sub32(cpu.r32(0x62d12c8u32), 0x0u32); }
  if cpu.zf { pc = 0x60ad3d3; continue; }
  pc = 0x60ad3a9; continue;
 }
 0x60ad3a9 => {
  { cpu.sub32(cpu.r32(0x62d12c4u32), 0x1u32); }
  if cpu.zf { pc = 0x60ad3bb; continue; }
  pc = 0x60ad3b2; continue;
 }
 0x60ad3b2 => {
  { cpu.sub32(cpu.r32(0x62d12ccu32), 0x0u32); }
  if cpu.zf { pc = 0x60ad3d3; continue; }
  pc = 0x60ad3bb; continue;
 }
 0x60ad3bb => {
  cpu.push(0x60ad3c0); f_060b3c01(cpu); if cpu.returned_to != 0x60ad3c0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ad3d3; continue; }
  pc = 0x60ad3c4; continue;
 }
 0x60ad3c4 => {
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ad3d0); f_060b3f05(cpu); if cpu.returned_to != 0x60ad3d0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60ad3d3; continue;
 }
 0x60ad3d3 => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617a3f0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
  pc = 0x60ad3e8; continue;
 }
 0x60ad3e8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad3f0); }
 let mut pc: u32 = 0x60ad3f0;
 loop { match pc {
 0x60ad3f0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  cpu.push(0x60ad3fe); f_060b335e(cpu); if cpu.returned_to != 0x60ad3fe { return; }
  { let v: u32 = cpu.r32(0x62d12d0u32); cpu.edx = v; }
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
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x60ad4b6; continue; }
  pc = 0x60ad45e; continue;
 }
 0x60ad45e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x616975cu32; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x9cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad4ac); cpu.call(t); if cpu.returned_to != 0x60ad4ac { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ad4b4); f_060ad7c0(cpu); if cpu.returned_to != 0x60ad4b4 { return; }
  pc = 0x60ad4dc; continue;
 }
 0x60ad4b6 => {
  { let v: u32 = 0x6169778u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x6169778u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ad4d9); f_060b3e93(cpu); if cpu.returned_to != 0x60ad4d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60ad4dc; continue;
 }
 0x60ad4dc => {
  { let v: u32 = cpu.r32(0x62d12c4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60ad4ff; continue; }
  pc = 0x60ad4e7; continue;
 }
 0x60ad4e7 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x6169788u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ad4fc); f_060b3e93(cpu); if cpu.returned_to != 0x60ad4fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60ad4ff; continue;
 }
 0x60ad4ff => {
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
pub fn f_060ad510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad510); }
 let mut pc: u32 = 0x60ad510;
 loop { match pc {
 0x60ad510 => {
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
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad560); cpu.call(t); if cpu.returned_to != 0x60ad560 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60ad57e); f_060b3e93(cpu); if cpu.returned_to != 0x60ad57e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad720); }
 let mut pc: u32 = 0x60ad720;
 loop { match pc {
 0x60ad720 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x616974cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad744); cpu.call(t); if cpu.returned_to != 0x60ad744 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60ad762); f_060b3e93(cpu); if cpu.returned_to != 0x60ad762 { return; }
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
pub fn f_060ad770(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad770); }
 let mut pc: u32 = 0x60ad770;
 loop { match pc {
 0x60ad770 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x616973cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60ad794); cpu.call(t); if cpu.returned_to != 0x60ad794 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60ad7b2); f_060b3e93(cpu); if cpu.returned_to != 0x60ad7b2 { return; }
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
pub fn f_060ad7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad7c0); }
 let mut pc: u32 = 0x60ad7c0;
 loop { match pc {
 0x60ad7c0 => {
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
  { let v: u32 = cpu.r32(0x62d12d4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60ad7e1); f_060b3e93(cpu); if cpu.returned_to != 0x60ad7e1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad7f0); }
 let mut pc: u32 = 0x60ad7f0;
 loop { match pc {
 0x60ad7f0 => {
  { cpu.sub32(cpu.r32(0x617a3f4u32), 0x0u32); }
  if !cpu.zf { pc = 0x60ad870; continue; }
  pc = 0x60ad7f9; continue;
 }
 0x60ad7f9 => {
  { let v = 0x60a5700u32; cpu.push(v); }
  { let v = 0x60ad7f0u32; cpu.push(v); }
  cpu.push(0x60ad808); f_060b3160(cpu); if cpu.returned_to != 0x60ad808 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d12c4u32; cpu.push(v); }
  cpu.push(0x60ad817); f_060b30a0(cpu); if cpu.returned_to != 0x60ad817 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d12c8u32; cpu.push(v); }
  cpu.push(0x60ad826); f_060b30a0(cpu); if cpu.returned_to != 0x60ad826 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d12ccu32; cpu.push(v); }
  cpu.push(0x60ad835); f_060b30a0(cpu); if cpu.returned_to != 0x60ad835 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d12d4u32; cpu.push(v); }
  cpu.push(0x60ad844); f_060b30a0(cpu); if cpu.returned_to != 0x60ad844 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617a3f0u32; cpu.push(v); }
  cpu.push(0x60ad853); f_060b30a0(cpu); if cpu.returned_to != 0x60ad853 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d12d0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60ad867); f_060b30a0(cpu); if cpu.returned_to != 0x60ad867 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617a3f4u32, v); }
  pc = 0x60ad870; continue;
 }
 0x60ad870 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad880); }
 let mut pc: u32 = 0x60ad880;
 loop { match pc {
 0x60ad880 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x60ad8c9; continue; }
  pc = 0x60ad889; continue;
 }
 0x60ad889 => {
  if !cpu.cf { pc = 0x60ad8b3; continue; }
  pc = 0x60ad88b; continue;
 }
 0x60ad88b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x60ad8b3; continue; }
  pc = 0x60ad892; continue;
 }
 0x60ad892 => {
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
 0x60ad8b3 => {
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
  pc = 0x60ad8c9; continue;
 }
 0x60ad8c9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad8cd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60ad8cd); } r_060ad8cd(cpu, 0x60ad8cd); }
pub fn f_060ad957(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60ad957); } r_060ad8cd(cpu, 0x60ad957); }
pub fn f_060ad95d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60ad95d); } r_060ad8cd(cpu, 0x60ad95d); }
fn r_060ad8cd(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60ad8cd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x60ad8d3, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x60ad8d8, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x60ad959; continue; }
  pc = 0x60ad8e8; continue;
 }
 0x60ad8e8 => {
  if cpu.cf { pc = 0x60ad95d; continue; }
  pc = 0x60ad8ea; continue;
 }
 0x60ad8ea => {
  if cpu.zf { cpu.call(0x60ad961); return; }
  pc = 0x60ad8ec; continue;
 }
 0x60ad8ec => {
  if (cpu.cf || cpu.zf) { pc = 0x60ad965; continue; }
  pc = 0x60ad8ee; continue;
 }
 0x60ad8ee => {
  if cpu.sf { pc = 0x60ad969; continue; }
  pc = 0x60ad8f0; continue;
 }
 0x60ad8f0 => {
  if cpu.pf() { f_060ad8f2(cpu); return; }
  f_060ad8f2(cpu); return;
 }
 0x60ad957 => {
  { let v: u32 = 0x617aff0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ad959 => {
  { cpu.scas(4, 0); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ad95d => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60ad963); f_060ad957(cpu); if cpu.returned_to != 0x60ad963 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  pc = 0x60ad965; continue;
 }
 0x60ad965 => {
  if cpu.zf { pc = 0x60ad97f; continue; }
  pc = 0x60ad967; continue;
 }
 0x60ad967 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x41c64e6du32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3039u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x60ad97f; continue;
 }
 0x60ad969 => {
  cpu.unimplemented(0x60ad969, "Insd");
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0xc2u8; cpu.w8(0xffffff81u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esi); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x60ad97f; continue;
 }
 0x60ad97f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad8f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad8f2); }
 let mut pc: u32 = 0x60ad8f2;
 loop { match pc {
 0x60ad8f2 => {
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
  pc = 0x60ad908; continue;
 }
 0x60ad908 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x60ad8cdu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60ad908; continue; }
  pc = 0x60ad928; continue;
 }
 0x60ad928 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60ad928; continue; }
  pc = 0x60ad932; continue;
 }
 0x60ad932 => {
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
pub fn f_060ad93c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad93c); }
 let mut pc: u32 = 0x60ad93c;
 loop { match pc {
 0x60ad93c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x60ad94e; continue; }
  pc = 0x60ad944; continue;
 }
 0x60ad944 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60ad94e; continue; }
  pc = 0x60ad948; continue;
 }
 0x60ad948 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60ad94e; continue;
 }
 0x60ad94e => {
  cpu.push(0x60ad953); f_060ad8f2(cpu); if cpu.returned_to != 0x60ad953 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad981(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ad981); }
 let mut pc: u32 = 0x60ad981;
 loop { match pc {
 0x60ad981 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x60ad989); f_060ad957(cpu); if cpu.returned_to != 0x60ad989 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ad98f; continue; }
  pc = 0x60ad98d; continue;
 }
 0x60ad98d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60ad98f; continue;
 }
 0x60ad98f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ad994(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60ad994); } r_060ad994(cpu, 0x60ad994); }
pub fn f_060ada08(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60ada08); } r_060ad994(cpu, 0x60ada08); }
pub fn f_060adc25(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60adc25); } r_060ad994(cpu, 0x60adc25); }
pub fn f_060c3e63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60c3e63); } r_060ad994(cpu, 0x60c3e63); }
fn r_060ad994(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60ad994 => {
  pc = 0x60ada0c; continue;
 }
 0x60ada08 => {
  cpu.unimplemented(0x60ada08, "Arpl");
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b844u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b830u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x617b83cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x617b867u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x617b868u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x60ada8e; continue; }
  pc = 0x60ada50; continue;
 }
 0x60ada0c => {
  {  }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b844u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b830u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x617b83cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x617b867u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x617b868u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x60ada8e; continue; }
  pc = 0x60ada50; continue;
 }
 0x60ada50 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x617b83cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffff000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b830u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60adac6; continue;
 }
 0x60ada8e => {
  { cpu.sub16((cpu.eax as u16), 0x4243u16); }
  if !cpu.zf { pc = 0x60adac8; continue; }
  pc = 0x60ada94; continue;
 }
 0x60ada94 => {
  { let v: u32 = cpu.edx; cpu.w32(0x617affcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x617b83cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60adac6; continue;
 }
 0x60adac6 => {
  pc = 0x60adb21; continue;
 }
 0x60adac8 => {
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xff00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60adb0a; continue; }
  pc = 0x60adad6; continue;
 }
 0x60adad6 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x60adae5; continue; }
  pc = 0x60adadf; continue;
 }
 0x60adadf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x617b000u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60adae5; continue;
 }
 0x60adae5 => {
  { let v: u16 = 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x60adaf9; continue; }
  pc = 0x60adaf7; continue;
 }
 0x60adaf7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60adaf9; continue;
 }
 0x60adaf9 => {
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x617b83cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x60adb21; continue;
 }
 0x60adb0a => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x24u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x17u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60adb21; continue;
 }
 0x60adb21 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x617b85eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x617b85fu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x60adc69u32.wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x617b861u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x617b865u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r16(0x617b83cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x62d2768u32; cpu.edx = v; }
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
  if cpu.zf { pc = 0x60adb76; continue; }
  pc = 0x60adb73; continue;
 }
 0x60adb73 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x60adb76; continue;
 }
 0x60adb76 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r16(0x617b865u32.wrapping_add(cpu.bases[0])); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x60adb88; continue;
 }
 0x60adb88 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x20202020u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x37386f6eu32); }
  if !cpu.zf { pc = 0x60adb9d; continue; }
  pc = 0x60adb96; continue;
 }
 0x60adb96 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x3du8); }
  if !cpu.zf { pc = 0x60adb9d; continue; }
  pc = 0x60adb9c; continue;
 }
 0x60adb9c => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60adb9d; continue;
 }
 0x60adb9d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.lods(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x60adb9d; continue; }
  pc = 0x60adba3; continue;
 }
 0x60adba3 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60adb88; continue; }
  pc = 0x60adba8; continue;
 }
 0x60adba8 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x60adbab; continue;
 }
 0x60adbab => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.movs(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x60adbab; continue; }
  pc = 0x60adbb1; continue;
 }
 0x60adbb1 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x617b85cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x617b840u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b82cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x62d2768u32; cpu.ecx = v; }
  { let v: u32 = 0x617cf30u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x60adbec; continue; }
  pc = 0x60adbdf; continue;
 }
 0x60adbdf => {
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60adbec; continue; }
  pc = 0x60adbe7; continue;
 }
 0x60adbe7 => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  pc = 0x60adbec; continue;
 }
 0x60adbec => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x62d2768u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x617b834u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x617b838u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.flat_ds(0x60adc17); cpu.push(0x60adc1c); f_060bcf9a(cpu); if cpu.returned_to != 0x60adc1c { return; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.flat_ds(0x60adc1e); cpu.push(0x60adc23); f_060bcf4a(cpu); if cpu.returned_to != 0x60adc23 { return; }
  pc = 0x60adc23; continue;
 }
 0x60adc23 => {
  pc = 0x60adc48; continue;
 }
 0x60adc25 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x60ada08u32; cpu.edx = v; }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.df = false; }
  pc = 0x60adc39; continue;
 }
 0x60adc39 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60adc39; continue; }
  pc = 0x60adc3e; continue;
 }
 0x60adc3e => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x60adc48; continue;
 }
 0x60adc48 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.flat_ds(0x60adc53); cpu.push(0x60adc58); f_060bcfe5(cpu); if cpu.returned_to != 0x60adc58 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x4cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x60adc5d); f_060adc60(cpu); return;
 }
 0x60c3e63 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x617ca94u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60c3e66); cpu.push(0x60c3e6c); cpu.call(t); if cpu.returned_to != 0x60c3e6c { return; }
  let t = cpu.r32(0x617ca98u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60c3e6c); cpu.push(0x60c3e72); cpu.call(t); if cpu.returned_to != 0x60c3e72 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.poll(); pc = 0x60adc23; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adc60); }
 let mut pc: u32 = 0x60adc60;
 loop { match pc {
 0x60adc60 => {
  { let v = cpu.r16(0x60adc69u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60adc68); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adc69(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60adc69); } r_060adc69(cpu, 0x60adc69); }
pub fn f_060adc6b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60adc6b); } r_060adc69(cpu, 0x60adc6b); }
pub fn f_060adc70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60adc70); } r_060adc69(cpu, 0x60adc70); }
fn r_060adc69(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60adc69 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60adc6b; continue;
 }
 0x60adc6b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617b004u32), 0x0u32); }
  if cpu.zf { pc = 0x60adc96; continue; }
  pc = 0x60adc7c; continue;
 }
 0x60adc70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617b004u32), 0x0u32); }
  if cpu.zf { pc = 0x60adc96; continue; }
  pc = 0x60adc7c; continue;
 }
 0x60adc7c => {
  { let v = 0x60adc70u32; cpu.push(v); }
  { let v = 0x60adf70u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60adc8d); f_060b30c0(cpu); if cpu.returned_to != 0x60adc8d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b004u32, v); }
  pc = 0x60adc96; continue;
 }
 0x60adc96 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adca0); }
 let mut pc: u32 = 0x60adca0;
 loop { match pc {
 0x60adca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x617a3f0u32), 0x0u32); }
  if !cpu.zf { pc = 0x60ade2f; continue; }
  pc = 0x60adcb4; continue;
 }
 0x60adcb4 => {
  { let v: u32 = 0x60adf60u32; cpu.ebx = v; }
  { let v: u32 = 0x60adf50u32; cpu.ecx = v; }
  { let v: u32 = 0x60adf40u32; cpu.esi = v; }
  { let v: u32 = 0x60adf20u32; cpu.edi = v; }
  { let v: u32 = 0x60ade58u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x617b004u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b948u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x617b94cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x617b950u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x617b954u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x617b958u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60add16; continue; }
  pc = 0x60adcf4; continue;
 }
 0x60adcf4 => {
  { let v = 0x60adc70u32; cpu.push(v); }
  { let v = 0x60adf70u32; cpu.push(v); }
  cpu.push(0x60add03); f_060b3160(cpu); if cpu.returned_to != 0x60add03 { return; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617b004u32, v); }
  cpu.push(0x60add16); f_060ad7f0(cpu); if cpu.returned_to != 0x60add16 { return; }
  pc = 0x60add16; continue;
 }
 0x60add16 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x616aa18u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d12c8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d12ccu32, v); }
  cpu.push(0x60add2e); f_060bd199(cpu); if cpu.returned_to != 0x60add2e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60add4a; continue; }
  pc = 0x60add36; continue;
 }
 0x60add36 => {
  cpu.push(0x60add3b); f_060b4090(cpu); if cpu.returned_to != 0x60add3b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x617a3f0u32, v); }
  pc = 0x60ade2f; continue;
 }
 0x60add4a => {
  { let v: u32 = 0x616aa24u32; cpu.eax = v; }
  cpu.push(0x60add54); f_060bd199(cpu); if cpu.returned_to != 0x60add54 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60add64; continue; }
  pc = 0x60add5a; continue;
 }
 0x60add5a => {
  { let v: u32 = 0x1u32; cpu.w32(0x62d12ccu32, v); }
  pc = 0x60add64; continue;
 }
 0x60add64 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60add6a); f_060b3e12(cpu); if cpu.returned_to != 0x60add6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12d4u32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60add8f; continue; }
  pc = 0x60add77; continue;
 }
 0x60add77 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60add81); f_060b4090(cpu); if cpu.returned_to != 0x60add81 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617a3f0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60add8f => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60add96); f_060a54cc(cpu); if cpu.returned_to != 0x60add96 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x60add9d); f_060a554c(cpu); if cpu.returned_to != 0x60add9d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x62d1300u32; cpu.edi = v; }
  cpu.push(0x60addab); f_060bd338(cpu); if cpu.returned_to != 0x60addab { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60addaf; continue;
 }
 0x60addaf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60addaf; continue; }
  pc = 0x60addb9; continue;
 }
 0x60addb9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = 0x616aa34u32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x62d1318u32, v); }
  cpu.push(0x60addcb); f_060ad7c0(cpu); if cpu.returned_to != 0x60addcb { return; }
  { let v = 0x616aa88u32; cpu.push(v); }
  cpu.push(0x60addd5); f_060ad7c0(cpu); if cpu.returned_to != 0x60addd5 { return; }
  { let v = 0x616aac4u32; cpu.push(v); }
  cpu.push(0x60adddf); f_060ad7c0(cpu); if cpu.returned_to != 0x60adddf { return; }
  { let v = 0x62d1300u32; cpu.push(v); }
  cpu.push(0x60adde9); f_060ad7c0(cpu); if cpu.returned_to != 0x60adde9 { return; }
  { let v = 0x616aad4u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x60addf8); f_060ad7c0(cpu); if cpu.returned_to != 0x60addf8 { return; }
  cpu.push(0x60addfd); f_060b4090(cpu); if cpu.returned_to != 0x60addfd { return; }
  cpu.push(0x60ade02); f_060b335e(cpu); if cpu.returned_to != 0x60ade02 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12d0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x62d12c4u32, v); }
  cpu.push(0x60ade18); f_060ad3f0(cpu); if cpu.returned_to != 0x60ade18 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x616ab2cu32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d12c4u32, v); }
  cpu.push(0x60ade29); f_060ad7c0(cpu); if cpu.returned_to != 0x60ade29 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x617a3f0u32, v); }
  pc = 0x60ade2f; continue;
 }
 0x60ade2f => {
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
pub fn f_060ade40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ade40); }
 let mut pc: u32 = 0x60ade40;
 loop { match pc {
 0x60ade20 => {
  { let at = 0x0u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.sub32(cpu.eax, 0xf997e806u32); cpu.eax = v; }
  cpu.call(0x60ade27); return;
 }
 0x60ade40 => {
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60ade20; continue; }
  pc = 0x60ade42; continue;
 }
 0x60ade42 => {
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.esi = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.flags() & 0xff; cpu.eax = (cpu.eax & 0xffff00ff) | (v << 8); }
  { let v = cpu.st(0) * (cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as f64); cpu.set_st(0, v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = 0xdeu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { let v: u8 = cpu.or8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.esi))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x60ade54); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ade58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ade58); }
 let mut pc: u32 = 0x60ade58;
 loop { match pc {
 0x60ade58 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60adf0d; continue; }
  pc = 0x60ade74; continue;
 }
 0x60ade74 => {
  cpu.poll(); pc = cpu.r32(0x60ade40u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x60ade7c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60ade83); f_0609c412(cpu); if cpu.returned_to != 0x60ade83 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x60ade8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60ade96); f_0609c412(cpu); if cpu.returned_to != 0x60ade96 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x60ade9f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60adead); f_0609c412(cpu); if cpu.returned_to != 0x60adead { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x60adeb6 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60adec8); f_0609c412(cpu); if cpu.returned_to != 0x60adec8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x60aded1 => {
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
  cpu.push(0x60adee7); f_0609c412(cpu); if cpu.returned_to != 0x60adee7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x60adef0 => {
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
  cpu.push(0x60adf0a); f_0609c412(cpu); if cpu.returned_to != 0x60adf0a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x60adf0d; continue;
 }
 0x60adf0d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adf20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adf20); }
 let mut pc: u32 = 0x60adf20;
 loop { match pc {
 0x60adf20 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60adf31); f_060a556c(cpu); if cpu.returned_to != 0x60adf31 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adf40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adf40); }
 let mut pc: u32 = 0x60adf40;
 loop { match pc {
 0x60adf40 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60adf49); f_060bd199(cpu); if cpu.returned_to != 0x60adf49 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adf50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adf50); }
 let mut pc: u32 = 0x60adf50;
 loop { match pc {
 0x60adf50 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60adf59); f_060bd117(cpu); if cpu.returned_to != 0x60adf59 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adf60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adf60); }
 let mut pc: u32 = 0x60adf60;
 loop { match pc {
 0x60adf60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60adf69); f_060bd034(cpu); if cpu.returned_to != 0x60adf69 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adf70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adf70); }
 let mut pc: u32 = 0x60adf70;
 loop { match pc {
 0x60adf70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617b004u32), 0x0u32); }
  if !cpu.zf { pc = 0x60adf9e; continue; }
  pc = 0x60adf7c; continue;
 }
 0x60adf7c => {
  { let v = 0x60adc70u32; cpu.push(v); }
  { let v = 0x60adf70u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60adf90); f_060b3160(cpu); if cpu.returned_to != 0x60adf90 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b004u32, v); }
  cpu.push(0x60adf9e); f_060ad7f0(cpu); if cpu.returned_to != 0x60adf9e { return; }
  pc = 0x60adf9e; continue;
 }
 0x60adf9e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adfa2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adfa2); }
 let mut pc: u32 = 0x60adfa2;
 loop { match pc {
 0x60adfa2 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x617b008u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adfaa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adfaa); }
 let mut pc: u32 = 0x60adfaa;
 loop { match pc {
 0x60adfaa => {
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x60adfb3); f_060adfbd(cpu); if cpu.returned_to != 0x60adfb3 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adfbd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adfbd); }
 let mut pc: u32 = 0x60adfbd;
 loop { match pc {
 0x60adfbd => {
  { cpu.sub32(cpu.eax, cpu.esp); }
  if !cpu.cf { pc = 0x60adfce; continue; }
  pc = 0x60adfc1; continue;
 }
 0x60adfc1 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x617b840u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x60adfce; continue; }
  pc = 0x60adfcd; continue;
 }
 0x60adfcd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60adfce => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x617b008u32)); }
  if cpu.zf { f_060adfdb(cpu); return; }
  pc = 0x60adfda; continue;
 }
 0x60adfda => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adfdb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adfdb); }
 let mut pc: u32 = 0x60adfdb;
 loop { match pc {
 0x60adfdb => {
  { let v: u32 = 0x617b00au32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x60adfea); f_060bd34a(cpu); if cpu.returned_to != 0x60adfea { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_060adff0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060adff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60adff0); }
 let mut pc: u32 = 0x60adff0;
 loop { match pc {
 0x60adff0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x62d15acu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x62d15acu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae041); f_060a556c(cpu); if cpu.returned_to != 0x60ae041 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.r32(0x62d15acu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae054(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae054); }
 let mut pc: u32 = 0x60ae054;
 loop { match pc {
 0x60ae054 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae085); f_060a556c(cpu); if cpu.returned_to != 0x60ae085 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae094(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae094); }
 let mut pc: u32 = 0x60ae094;
 loop { match pc {
 0x60ae094 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae0dc); f_060a556c(cpu); if cpu.returned_to != 0x60ae0dc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae0ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae0ec); }
 let mut pc: u32 = 0x60ae0ec;
 loop { match pc {
 0x60ae0ec => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae134); f_060a556c(cpu); if cpu.returned_to != 0x60ae134 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae144(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae144); }
 let mut pc: u32 = 0x60ae144;
 loop { match pc {
 0x60ae144 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae175); f_060a556c(cpu); if cpu.returned_to != 0x60ae175 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae180); }
 let mut pc: u32 = 0x60ae180;
 loop { match pc {
 0x60ae180 => {
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
  pc = 0x60ae253; continue;
 }
 0x60ae19d => {
  { let v: u32 = cpu.r32(0x62d1598u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d151cu32); cpu.ebp = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d1590u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x62d1590u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x62d15a8u32); cpu.ebp = v; }
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
  { let v: u16 = cpu.r16(0x617b01eu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x12u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  cpu.push(0x60ae247); f_060ae094(cpu); if cpu.returned_to != 0x60ae247 { return; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1eu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62d15cau32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60ae253; continue;
 }
 0x60ae253 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x617b01cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60ae19d; continue; }
  pc = 0x60ae263; continue;
 }
 0x60ae263 => {
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
pub fn f_060ae270(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae270); }
 let mut pc: u32 = 0x60ae270;
 loop { match pc {
 0x60ae270 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x62d15a8u32); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x617b020u32), 0x0u16); }
  if !cpu.zf { pc = 0x60ae50a; continue; }
  pc = 0x60ae292; continue;
 }
 0x60ae292 => {
  { let v: u32 = 0x61400b5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x6140000u32); cpu.ebx = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60ae2a9); f_0609ad7c(cpu); if cpu.returned_to != 0x60ae2a9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15b4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ae2cb; continue; }
  pc = 0x60ae2b2; continue;
 }
 0x60ae2b2 => {
  { let v = 0x616ab3cu32; cpu.push(v); }
  { let v = 0x616ab74u32; cpu.push(v); }
  cpu.push(0x60ae2c1); f_060a3a48(cpu); if cpu.returned_to != 0x60ae2c1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ae50f; continue;
 }
 0x60ae2cb => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x6140000u32; cpu.eax = v; }
  cpu.push(0x60ae2d7); f_0609bf38(cpu); if cpu.returned_to != 0x60ae2d7 { return; }
  { let v: u32 = 0x6140008u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x6140000u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x60u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15b8u32, v); }
  pc = 0x60ae2f0; continue;
 }
 0x60ae2f0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x62d15c4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15c0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ae31e; continue; }
  pc = 0x60ae319; continue;
 }
 0x60ae319 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x60ae32d; continue; }
  pc = 0x60ae31e; continue;
 }
 0x60ae31e => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0xcfu8); }
  if cpu.zf { pc = 0x60ae32d; continue; }
  pc = 0x60ae326; continue;
 }
 0x60ae326 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x66u8); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x60ae2f0; continue; }
  pc = 0x60ae32d; continue;
 }
 0x60ae32d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x62d15cbu32, v); }
  { cpu.sub8((cpu.ecx as u8), 0x67u8); }
  if !cpu.zf { pc = 0x60ae35b; continue; }
  pc = 0x60ae338; continue;
 }
 0x60ae338 => {
  { let v = 0x616ab80u32; cpu.push(v); }
  { let v = 0x616ab74u32; cpu.push(v); }
  cpu.push(0x60ae347); f_060a3a48(cpu); if cpu.returned_to != 0x60ae347 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ae35b => {
  { let v: u32 = 0x200u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60ae375); f_060a556c(cpu); if cpu.returned_to != 0x60ae375 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d15c6u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x62d15c8u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x62d15b8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x62d15b8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60ae3cf); f_060a556c(cpu); if cpu.returned_to != 0x60ae3cf { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x617b01cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x617b020u32, v); }
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
  { let v: u16 = cpu.r16(0x617b01eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  cpu.push(0x60ae42e); f_0609ad7c(cpu); if cpu.returned_to != 0x60ae42e { return; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15b0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ae460; continue; }
  pc = 0x60ae43d; continue;
 }
 0x60ae43d => {
  { let v = 0x616abc0u32; cpu.push(v); }
  { let v = 0x616ab74u32; cpu.push(v); }
  cpu.push(0x60ae44c); f_060a3a48(cpu); if cpu.returned_to != 0x60ae44c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ae460 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x617b01cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d151cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1598u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d159cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1590u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1594u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1eu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60ae4c8; continue;
 }
 0x60ae4b1 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x617b01eu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x12u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x60ae4c8; continue;
 }
 0x60ae4c8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x617b01cu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60ae4b1; continue; }
  pc = 0x60ae4d5; continue;
 }
 0x60ae4d5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x617b01eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x62d15cbu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15acu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x62d159cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x62d15a8u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x62d1583u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x60ae50a; continue;
 }
 0x60ae50a => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60ae50f; continue;
 }
 0x60ae50f => {
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
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
pub fn f_060ae520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae520); }
 let mut pc: u32 = 0x60ae520;
 loop { match pc {
 0x60ae520 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x617b020u32), 0x0u16); }
  if cpu.zf { pc = 0x60ae58a; continue; }
  pc = 0x60ae52f; continue;
 }
 0x60ae52f => {
  { let v: u32 = cpu.r32(0x62d15b0u32); cpu.eax = v; }
  cpu.push(0x60ae539); f_0609b400(cpu); if cpu.returned_to != 0x60ae539 { return; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  cpu.push(0x60ae543); f_0609b400(cpu); if cpu.returned_to != 0x60ae543 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x62d15c6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x62d15c8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x60ae57c); f_060a556c(cpu); if cpu.returned_to != 0x60ae57c { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x617b020u32, v); }
  pc = 0x60ae58a; continue;
 }
 0x60ae58a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae590(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae590); }
 let mut pc: u32 = 0x60ae590;
 loop { match pc {
 0x60ae590 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae5be); f_060a556c(cpu); if cpu.returned_to != 0x60ae5be { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae5cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae5cc); }
 let mut pc: u32 = 0x60ae5cc;
 loop { match pc {
 0x60ae5cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x62d151cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x62d151cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae62d); f_060a556c(cpu); if cpu.returned_to != 0x60ae62d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.r32(0x62d151cu32); cpu.eax = v; }
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
pub fn f_060ae648(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae648); }
 let mut pc: u32 = 0x60ae648;
 loop { match pc {
 0x60ae648 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x62d151cu32), 0x0u32); }
  if cpu.zf { pc = 0x60ae69c; continue; }
  pc = 0x60ae656; continue;
 }
 0x60ae656 => {
  { let v: u32 = cpu.r32(0x62d15b4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x62d151cu32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x62d15cbu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60ae68f); f_060a556c(cpu); if cpu.returned_to != 0x60ae68f { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d15a0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d151cu32, v); }
  pc = 0x60ae69c; continue;
 }
 0x60ae69c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae6a4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae6a4); }
 let mut pc: u32 = 0x60ae6a4;
 loop { match pc {
 0x60ae6a4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d159cu32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d15a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  cpu.push(0x60ae6c8); f_0609bf38(cpu); if cpu.returned_to != 0x60ae6c8 { return; }
  { let v: u32 = cpu.r32(0x62d15a4u32); cpu.esi = v; }
  cpu.push(0x60ae6d3); f_0609e790(cpu); if cpu.returned_to != 0x60ae6d3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x62d1583u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x62d159cu32); cpu.ebp = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x62d151cu32); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x62d1594u32); cpu.esi = v; }
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
  pc = 0x60ae72a; continue;
 }
 0x60ae72a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1bu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60ae72a; continue; }
  pc = 0x60ae735; continue;
 }
 0x60ae735 => {
  { let v: u32 = cpu.r32(0x62d1594u32); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x62d15a4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x62d1594u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x62d15a4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d159cu32, v); }
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
  cpu.push(0x60ae79f); f_060ae0ec(cpu); if cpu.returned_to != 0x60ae79f { return; }
  { let v: u32 = cpu.r32(0x62d159cu32); cpu.ebp = v; }
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
pub fn f_060ae7b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae7b0); }
 let mut pc: u32 = 0x60ae7b0;
 loop { match pc {
 0x60ae7b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d15a8u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7fffffffu32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60ae801; continue;
 }
 0x60ae7db => {
  { let v: u32 = cpu.r32(0x62d1598u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x60ae7fa; continue; }
  pc = 0x60ae7e8; continue;
 }
 0x60ae7e8 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x60ae7fa; continue; }
  pc = 0x60ae7f4; continue;
 }
 0x60ae7f4 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60ae7fa; continue;
 }
 0x60ae7fa => {
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60ae801; continue;
 }
 0x60ae801 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x617b01cu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60ae7db; continue; }
  pc = 0x60ae80e; continue;
 }
 0x60ae80e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60ae91c; continue; }
  pc = 0x60ae81b; continue;
 }
 0x60ae81b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x12u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60ae82f; continue; }
  pc = 0x60ae82d; continue;
 }
 0x60ae82d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60ae82f; continue;
 }
 0x60ae82f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60ae84d; continue; }
  pc = 0x60ae836; continue;
 }
 0x60ae836 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1590u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x60ae84d; continue;
 }
 0x60ae84d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1598u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d151cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x1eu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x62d1590u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d1590u32); cpu.ebx = v; }
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
  { let v: u16 = cpu.r16(0x617b01eu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d15a8u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60ae900); f_060ae094(cpu); if cpu.returned_to != 0x60ae900 { return; }
  { let v: u32 = cpu.r32(0x62d15a8u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x62d15cau32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d15a8u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ae91c => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d15a8u32, v); }
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
pub fn f_060ae930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae930); }
 let mut pc: u32 = 0x60ae930;
 loop { match pc {
 0x60ae930 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae960; continue; }
  pc = 0x60ae934; continue;
 }
 0x60ae934 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x60ae936; continue;
 }
 0x60ae936 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x60ae943; continue; }
  pc = 0x60ae93a; continue;
 }
 0x60ae93a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60ae936; continue; }
  pc = 0x60ae943; continue;
 }
 0x60ae943 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x60ae94c); f_060ae967(cpu); if cpu.returned_to != 0x60ae94c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae960; continue; }
  pc = 0x60ae952; continue;
 }
 0x60ae952 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae960; continue; }
  pc = 0x60ae957; continue;
 }
 0x60ae957 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae960; continue; }
  pc = 0x60ae95d; continue;
 }
 0x60ae95d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x60ae960; continue;
 }
 0x60ae960 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae967); }
 let mut pc: u32 = 0x60ae967;
 loop { match pc {
 0x60ae967 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae9d2; continue; }
  pc = 0x60ae96b; continue;
 }
 0x60ae96b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x60ae977; continue; }
  pc = 0x60ae96f; continue;
 }
 0x60ae96f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60ae96b; continue; }
  pc = 0x60ae977; continue;
 }
 0x60ae977 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae9b7; continue; }
  pc = 0x60ae97d; continue;
 }
 0x60ae97d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae9a9; continue; }
  pc = 0x60ae980; continue;
 }
 0x60ae980 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x60ae9a6; continue; }
  pc = 0x60ae98e; continue;
 }
 0x60ae98e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60ae980; continue; }
  pc = 0x60ae9a3; continue;
 }
 0x60ae9a3 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x60ae9a6; continue;
 }
 0x60ae9a6 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x60ae9a9; continue;
 }
 0x60ae9a9 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x60ae9b7; continue;
 }
 0x60ae9b7 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae9d2; continue; }
  pc = 0x60ae9bd; continue;
 }
 0x60ae9bd => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae9d2; continue; }
  pc = 0x60ae9c5; continue;
 }
 0x60ae9c5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60ae9d2; continue; }
  pc = 0x60ae9cd; continue;
 }
 0x60ae9cd => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x60ae9d2; continue;
 }
 0x60ae9d2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae9e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae9e0); }
 let mut pc: u32 = 0x60ae9e0;
 loop { match pc {
 0x60ae9e0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x60ae9f5); f_060ae930(cpu); if cpu.returned_to != 0x60ae9f5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ae9f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ae9f8); }
 let mut pc: u32 = 0x60ae9f8;
 loop { match pc {
 0x60ae9f8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x60aea0e; continue; }
  pc = 0x60aea01; continue;
 }
 0x60aea01 => {
  { cpu.interrupt(0x1a); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60aea23; continue;
 }
 0x60aea0e => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  if !cpu.zf { pc = 0x60aea23; continue; }
  pc = 0x60aea16; continue;
 }
 0x60aea16 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x1a); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60aea23; continue;
 }
 0x60aea23 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aea27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aea27); }
 let mut pc: u32 = 0x60aea27;
 loop { match pc {
 0x60aea27 => {
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
pub fn f_060aea4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aea4c); }
 let mut pc: u32 = 0x60aea4c;
 loop { match pc {
 0x60aea4c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x617b85eu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60aea69; continue; }
  pc = 0x60aea5b; continue;
 }
 0x60aea5b => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60aea69; continue; }
  pc = 0x60aea60; continue;
 }
 0x60aea60 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2502u32; cpu.eax = v; }
  pc = 0x60aea73; continue;
 }
 0x60aea69 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60aea73; continue;
 }
 0x60aea73 => {
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
pub fn f_060aea7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aea7e); }
 let mut pc: u32 = 0x60aea7e;
 loop { match pc {
 0x60aea7e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x60aeaa1; continue; }
  pc = 0x60aea8e; continue;
 }
 0x60aea8e => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60aeaa1; continue; }
  pc = 0x60aea93; continue;
 }
 0x60aea93 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x60aeaa0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60aeaa1 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x60aeaaa); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeaab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeaab); }
 let mut pc: u32 = 0x60aeaab;
 loop { match pc {
 0x60aeaab => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeaad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeaad); }
 let mut pc: u32 = 0x60aeaad;
 loop { match pc {
 0x60aeaad => {
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
pub fn f_060aeab7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeab7); }
 let mut pc: u32 = 0x60aeab7;
 loop { match pc {
 0x60aeab7 => {
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
pub fn f_060aeabf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeabf); }
 let mut pc: u32 = 0x60aeabf;
 loop { match pc {
 0x60aeabf => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeac1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeac1); }
 let mut pc: u32 = 0x60aeac1;
 loop { match pc {
 0x60aeac1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x60aead3); f_060bd368(cpu); if cpu.returned_to != 0x60aead3 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeada(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeada); }
 let mut pc: u32 = 0x60aeada;
 loop { match pc {
 0x60aeada => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x60aeaf5; continue; }
  pc = 0x60aeae9; continue;
 }
 0x60aeae9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x60aeaf5; continue;
 }
 0x60aeaf5 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeaf9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeaf9); }
 let mut pc: u32 = 0x60aeaf9;
 loop { match pc {
 0x60aeaf9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x60aeadau32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60aeb19); f_060aee28(cpu); if cpu.returned_to != 0x60aeb19 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeb22(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeb22); }
 let mut pc: u32 = 0x60aeb22;
 loop { match pc {
 0x60aeb22 => {
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
  cpu.push(0x60aeb42); f_060aeaf9(cpu); if cpu.returned_to != 0x60aeb42 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeb49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeb49); }
 let mut pc: u32 = 0x60aeb49;
 loop { match pc {
 0x60aeb49 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60aeb64); f_060b17b3(cpu); if cpu.returned_to != 0x60aeb64 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aebfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aebfa); }
 let mut pc: u32 = 0x60aebfa;
 loop { match pc {
 0x60aebfa => {
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
  pc = 0x60aec08; continue;
 }
 0x60aec08 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60aec2c; continue; }
  pc = 0x60aec0e; continue;
 }
 0x60aec0e => {
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
  cpu.poll(); pc = 0x60aec08; continue;
 }
 0x60aec2c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x43300000u32; cpu.edi = v; }
  cpu.push(0x60aec38); f_060aec45(cpu); if cpu.returned_to != 0x60aec38 { return; }
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
pub fn f_060aec45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aec45); }
 let mut pc: u32 = 0x60aec45;
 loop { match pc {
 0x60aec45 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.edx); cpu.esi = v; }
  if cpu.zf { pc = 0x60aec95; continue; }
  pc = 0x60aec4d; continue;
 }
 0x60aec4d => {
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if !cpu.zf { pc = 0x60aec69; continue; }
  pc = 0x60aec55; continue;
 }
 0x60aec55 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if cpu.zf { cpu.poll(); pc = 0x60aec55; continue; }
  pc = 0x60aec67; continue;
 }
 0x60aec67 => {
  pc = 0x60aec8d; continue;
 }
 0x60aec69 => {
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if cpu.zf { pc = 0x60aec8d; continue; }
  pc = 0x60aec71; continue;
 }
 0x60aec71 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60aec71; continue; }
  pc = 0x60aec85; continue;
 }
 0x60aec85 => {
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x60aec8d; continue;
 }
 0x60aec8d => {
  { let v: u32 = cpu.edx & 0xfffffu32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edi); cpu.edx = v; }
  pc = 0x60aec95; continue;
 }
 0x60aec95 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aec96(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aec96); }
 let mut pc: u32 = 0x60aec96;
 loop { match pc {
 0x60aec96 => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_060aec9b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aec9b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aec9b); }
 let mut pc: u32 = 0x60aec9b;
 loop { match pc {
 0x60aec9b => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x60aeca3); f_060b1377(cpu); if cpu.returned_to != 0x60aeca3 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aeca7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aeca7); }
 let mut pc: u32 = 0x60aeca7;
 loop { match pc {
 0x60aeca7 => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_060aec9b(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aecae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aecae); }
 let mut pc: u32 = 0x60aecae;
 loop { match pc {
 0x60aecae => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x60aecb8); f_060aec9b(cpu); if cpu.returned_to != 0x60aecb8 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aecbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aecbe); }
 let mut pc: u32 = 0x60aecbe;
 loop { match pc {
 0x60aecbe => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x60aecc6); f_060b137d(cpu); if cpu.returned_to != 0x60aecc6 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aecca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aecca); }
 let mut pc: u32 = 0x60aecca;
 loop { match pc {
 0x60aecca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x60aecf0; continue; }
  pc = 0x60aecdb; continue;
 }
 0x60aecdb => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60aeced); f_060aed0b(cpu); if cpu.returned_to != 0x60aeced { return; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60aecf0; continue;
 }
 0x60aecf0 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60aecfa); f_060bd780(cpu); if cpu.returned_to != 0x60aecfa { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60aed03); f_060bd746(cpu); if cpu.returned_to != 0x60aed03 { return; }
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
pub fn f_060aed0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aed0b); }
 let mut pc: u32 = 0x60aed0b;
 loop { match pc {
 0x60aed0b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x616b5e0u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x60aed36; continue; }
  pc = 0x60aed26; continue;
 }
 0x60aed26 => {
  { let v: u32 = cpu.r32(0x616b5f0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x616b5f4u32); cpu.eax = v; }
  pc = 0x60aed53; continue;
 }
 0x60aed36 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x616b5e8u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x60aed57; continue; }
  pc = 0x60aed45; continue;
 }
 0x60aed45 => {
  { let v: u32 = cpu.r32(0x616b5f8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x616b5fcu32); cpu.eax = v; }
  pc = 0x60aed53; continue;
 }
 0x60aed53 => {
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x60aed57; continue;
 }
 0x60aed57 => {
  { let v: u32 = 0x616b680u32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x60aed9a; continue; }
  pc = 0x60aed60; continue;
 }
 0x60aed60 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60aedb8; continue; }
  pc = 0x60aed6c; continue;
 }
 0x60aed6c => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x617b930u32), 0x1u8); }
  if cpu.zf { pc = 0x60aed8c; continue; }
  pc = 0x60aed79; continue;
 }
 0x60aed79 => {
  { let v = cpu.r32(0x616b686u32); cpu.push(v); }
  { let v = cpu.r32(0x616b682u32); cpu.push(v); }
  cpu.push(0x60aed8a); f_060b2d0c(cpu); if cpu.returned_to != 0x60aed8a { return; }
  pc = 0x60aed92; continue;
 }
 0x60aed8c => {
  { let v = cpu.st(0) / cpu.rf64(0x616b682u32); cpu.set_st(0, v); }
  pc = 0x60aed92; continue;
 }
 0x60aed92 => {
  { let v: u32 = cpu.edx.wrapping_sub(0xd8u32); cpu.edx = v; }
  pc = 0x60aedb4; continue;
 }
 0x60aed9a => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60aedb8; continue; }
  pc = 0x60aeda4; continue;
 }
 0x60aeda4 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x616b682u32); cpu.set_st(0, v); }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x60aedb4; continue;
 }
 0x60aedb4 => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x60aedb8; continue;
 }
 0x60aedb8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60aedc6; continue;
 }
 0x60aedc6 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x60aedd8; continue; }
  pc = 0x60aedcd; continue;
 }
 0x60aedcd => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x60aedd8; continue;
 }
 0x60aedd8 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60aede9; continue; }
  pc = 0x60aeddc; continue;
 }
 0x60aeddc => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x60aedc6; continue; }
  pc = 0x60aede4; continue;
 }
 0x60aede4 => {
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.poll(); pc = 0x60aedc6; continue;
 }
 0x60aede9 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x60aee0e; continue; }
  pc = 0x60aeded; continue;
 }
 0x60aeded => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x617b930u32), 0x1u8); }
  if cpu.zf { pc = 0x60aee09; continue; }
  pc = 0x60aedfa; continue;
 }
 0x60aedfa => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x60aee07); f_060b2d0c(cpu); if cpu.returned_to != 0x60aee07 { return; }
  pc = 0x60aee15; continue;
 }
 0x60aee09 => {
  { let v = cpu.st(0) / cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x60aee15; continue;
 }
 0x60aee0e => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x60aee15; continue;
 }
 0x60aee15 => {
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
pub fn f_060aee28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aee28); }
 let mut pc: u32 = 0x60aee28;
 loop { match pc {
 0x60aee28 => {
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
  pc = 0x60aeefa; continue;
 }
 0x60aee67 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x60aee75); f_060af1c7(cpu); if cpu.returned_to != 0x60aee75 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x115u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60af1b5; continue; }
  pc = 0x60aee89; continue;
 }
 0x60aee89 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x60aefc4; continue; }
  pc = 0x60aee9b; continue;
 }
 0x60aee9b => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60aef31; continue; }
  pc = 0x60aeec3; continue;
 }
 0x60aeec3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60aeee1; continue; }
  pc = 0x60aeecb; continue;
 }
 0x60aeecb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x60aeed5; continue;
 }
 0x60aeed5 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60aeefa; continue;
 }
 0x60aeee1 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60aef26; continue; }
  pc = 0x60aeee8; continue;
 }
 0x60aeee8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x60aeef1; continue;
 }
 0x60aeef1 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x60aeefa; continue;
 }
 0x60aeefa => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x60af1b5; continue; }
  pc = 0x60aef04; continue;
 }
 0x60aef04 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x25u32); }
  if cpu.zf { cpu.poll(); pc = 0x60aee67; continue; }
  pc = 0x60aef14; continue;
 }
 0x60aef14 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60aef24); cpu.call(t); if cpu.returned_to != 0x60aef24 { return; }
  cpu.poll(); pc = 0x60aeefa; continue;
 }
 0x60aef26 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x60aeef1; continue;
 }
 0x60aef31 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60aef8c; continue; }
  pc = 0x60aef43; continue;
 }
 0x60aef43 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60aef62; continue; }
  pc = 0x60aef4b; continue;
 }
 0x60aef4b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.eax); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x60aeefa; continue;
 }
 0x60aef62 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60aef81; continue; }
  pc = 0x60aef69; continue;
 }
 0x60aef69 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x60aef72; continue;
 }
 0x60aef72 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x60aeefa; continue;
 }
 0x60aef81 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.poll(); pc = 0x60aef72; continue;
 }
 0x60aef8c => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60aefa1; continue; }
  pc = 0x60aef92; continue;
 }
 0x60aef92 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  cpu.poll(); pc = 0x60aeed5; continue;
 }
 0x60aefa1 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60aefb6; continue; }
  pc = 0x60aefa8; continue;
 }
 0x60aefa8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.poll(); pc = 0x60aeef1; continue;
 }
 0x60aefb6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.poll(); pc = 0x60aeef1; continue;
 }
 0x60aefc4 => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x60aefdb); f_060af505(cpu); if cpu.returned_to != 0x60aefdb { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x60af027; continue; }
  pc = 0x60aefef; continue;
 }
 0x60aefef => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x60af027; continue; }
  pc = 0x60aeffb; continue;
 }
 0x60aeffb => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x60af027; continue; }
  pc = 0x60af00e; continue;
 }
 0x60af00e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af025); cpu.call(t); if cpu.returned_to != 0x60af025 { return; }
  cpu.poll(); pc = 0x60aeffb; continue;
 }
 0x60af027 => {
  { let v: u32 = 0x117u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x60af02e; continue;
 }
 0x60af02e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60af049; continue; }
  pc = 0x60af034; continue;
 }
 0x60af034 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af047); cpu.call(t); if cpu.returned_to != 0x60af047 { return; }
  cpu.poll(); pc = 0x60af02e; continue;
 }
 0x60af049 => {
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60af073; continue; }
  pc = 0x60af05e; continue;
 }
 0x60af05e => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af071); cpu.call(t); if cpu.returned_to != 0x60af071 { return; }
  cpu.poll(); pc = 0x60af049; continue;
 }
 0x60af073 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x60af0bb; continue; }
  pc = 0x60af083; continue;
 }
 0x60af083 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.zf { pc = 0x60af0bb; continue; }
  pc = 0x60af08f; continue;
 }
 0x60af08f => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x60af0bb; continue; }
  pc = 0x60af0a2; continue;
 }
 0x60af0a2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af0b9); cpu.call(t); if cpu.returned_to != 0x60af0b9 { return; }
  cpu.poll(); pc = 0x60af08f; continue;
 }
 0x60af0bb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.zf { pc = 0x60af0ce; continue; }
  pc = 0x60af0c9; continue;
 }
 0x60af0c9 => {
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x60af13a; continue; }
  pc = 0x60af0ce; continue;
 }
 0x60af0ce => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60af10f; continue; }
  pc = 0x60af0de; continue;
 }
 0x60af0de => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60af165; continue; }
  pc = 0x60af0f7; continue;
 }
 0x60af0f7 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af10a); cpu.call(t); if cpu.returned_to != 0x60af10a { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x60af0de; continue;
 }
 0x60af10f => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60af165; continue; }
  pc = 0x60af124; continue;
 }
 0x60af124 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af137); cpu.call(t); if cpu.returned_to != 0x60af137 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60af10f; continue;
 }
 0x60af13a => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60af165; continue; }
  pc = 0x60af14f; continue;
 }
 0x60af14f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af162); cpu.call(t); if cpu.returned_to != 0x60af162 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60af13a; continue;
 }
 0x60af165 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x60aeefa; continue; }
  pc = 0x60af179; continue;
 }
 0x60af179 => {
  { cpu.sub32(cpu.r32(0x104u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60aeefa; continue; }
  pc = 0x60af187; continue;
 }
 0x60af187 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x60aeefa; continue; }
  pc = 0x60af1a0; continue;
 }
 0x60af1a0 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x60af1b3); cpu.call(t); if cpu.returned_to != 0x60af1b3 { return; }
  cpu.poll(); pc = 0x60af187; continue;
 }
 0x60af1b5 => {
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
pub fn f_060af1c7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af1c7); }
 let mut pc: u32 = 0x60af1c7;
 loop { match pc {
 0x60af1c7 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60af1db); f_060af305(cpu); if cpu.returned_to != 0x60af1db { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x60af215; continue; }
  pc = 0x60af1ef; continue;
 }
 0x60af1ef => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60af215; continue; }
  pc = 0x60af1f4; continue;
 }
 0x60af1f4 => {
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
  if (cpu.sf != cpu.of) { pc = 0x60af240; continue; }
  pc = 0x60af20e; continue;
 }
 0x60af20e => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60af1f4; continue; }
  pc = 0x60af213; continue;
 }
 0x60af213 => {
  pc = 0x60af240; continue;
 }
 0x60af215 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x60af240; continue; }
  pc = 0x60af21e; continue;
 }
 0x60af21e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60af23f; continue; }
  pc = 0x60af22f; continue;
 }
 0x60af22f => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8) | 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60af23f; continue;
 }
 0x60af23f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60af240; continue;
 }
 0x60af240 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x60af299; continue; }
  pc = 0x60af250; continue;
 }
 0x60af250 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x60af27c; continue; }
  pc = 0x60af261; continue;
 }
 0x60af261 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60af279; continue; }
  pc = 0x60af272; continue;
 }
 0x60af272 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60af279; continue;
 }
 0x60af279 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60af299; continue;
 }
 0x60af27c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x60af299; continue; }
  pc = 0x60af285; continue;
 }
 0x60af285 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60af299; continue; }
  pc = 0x60af28a; continue;
 }
 0x60af28a => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x60af27c; continue;
 }
 0x60af299 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4eu8); }
  if cpu.cf { pc = 0x60af2be; continue; }
  pc = 0x60af2a3; continue;
 }
 0x60af2a3 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af2fb; continue; }
  pc = 0x60af2a5; continue;
 }
 0x60af2a5 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x60af2b5; continue; }
  pc = 0x60af2aa; continue;
 }
 0x60af2aa => {
  if (cpu.cf || cpu.zf) { pc = 0x60af2ce; continue; }
  pc = 0x60af2ac; continue;
 }
 0x60af2ac => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x60af2ce; continue; }
  pc = 0x60af2b1; continue;
 }
 0x60af2b1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60af2b5 => {
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.zf { pc = 0x60af2dc; continue; }
  pc = 0x60af2ba; continue;
 }
 0x60af2ba => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60af2be => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x60af301; continue; }
  pc = 0x60af2c3; continue;
 }
 0x60af2c3 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af2f1; continue; }
  pc = 0x60af2c5; continue;
 }
 0x60af2c5 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x60af2e2; continue; }
  pc = 0x60af2ca; continue;
 }
 0x60af2ca => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60af2ce => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60af2d5; continue;
 }
 0x60af2d5 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60af2dc => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x10u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60af2ff; continue;
 }
 0x60af2e2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60af2f1 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.poll(); pc = 0x60af2d5; continue;
 }
 0x60af2fb => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60af2ff; continue;
 }
 0x60af2ff => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60af301; continue;
 }
 0x60af301 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060af305(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af305); }
 let mut pc: u32 = 0x60af305;
 loop { match pc {
 0x60af305 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60af30d; continue;
 }
 0x60af30d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x60af31c; continue; }
  pc = 0x60af316; continue;
 }
 0x60af316 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60af35c; continue;
 }
 0x60af31c => {
  { cpu.sub32(cpu.eax, 0x23u32); }
  if !cpu.zf { pc = 0x60af327; continue; }
  pc = 0x60af321; continue;
 }
 0x60af321 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60af35c; continue;
 }
 0x60af327 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x60af33c; continue; }
  pc = 0x60af32c; continue;
 }
 0x60af32c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60af35c; continue;
 }
 0x60af33c => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x60af353; continue; }
  pc = 0x60af341; continue;
 }
 0x60af341 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x60af35c; continue; }
  pc = 0x60af34d; continue;
 }
 0x60af34d => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x2u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60af35c; continue;
 }
 0x60af353 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x60af35f; continue; }
  pc = 0x60af358; continue;
 }
 0x60af358 => {
  { let v: u8 = 0x30u8; cpu.w8(0x16u32.wrapping_add(cpu.edx), v); }
  pc = 0x60af35c; continue;
 }
 0x60af35c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60af30d; continue;
 }
 0x60af35f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060af364(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60af364); } r_060af364(cpu, 0x60af364); }
pub fn f_060af388(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60af388); } r_060af364(cpu, 0x60af388); }
fn r_060af364(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60af364 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60af372; continue;
 }
 0x60af372 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x60af383; continue; }
  pc = 0x60af37c; continue;
 }
 0x60af37c => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x60af383; continue; }
  pc = 0x60af380; continue;
 }
 0x60af380 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60af372; continue;
 }
 0x60af383 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60af388 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60af396; continue;
 }
 0x60af396 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x60af383; continue; }
  pc = 0x60af3a4; continue;
 }
 0x60af3a4 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x60af383; continue; }
  pc = 0x60af3a8; continue;
 }
 0x60af3a8 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60af396; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060af3ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af3ab); }
 let mut pc: u32 = 0x60af3ab;
 loop { match pc {
 0x60af3ab => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x60af3c3); f_060ad93c(cpu); if cpu.returned_to != 0x60af3c3 { return; }
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
  pc = 0x60af3e8; continue;
 }
 0x60af3e1 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x60af3e8; continue;
 }
 0x60af3e8 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x60af3e1; continue; }
  pc = 0x60af3ec; continue;
 }
 0x60af3ec => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x60af3ef; continue;
 }
 0x60af3ef => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x60af3fa; continue; }
  pc = 0x60af3f3; continue;
 }
 0x60af3f3 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x60af3ef; continue;
 }
 0x60af3fa => {
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
pub fn f_060af409(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af409); }
 let mut pc: u32 = 0x60af409;
 loop { match pc {
 0x60af409 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60af425; continue; }
  pc = 0x60af41a; continue;
 }
 0x60af41a => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60af425; continue;
 }
 0x60af425 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x60af432; continue; }
  pc = 0x60af42b; continue;
 }
 0x60af42b => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af432; continue;
 }
 0x60af432 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60af445); f_060ad93c(cpu); if cpu.returned_to != 0x60af445 { return; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x60af447; continue;
 }
 0x60af447 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x60af454; continue; }
  pc = 0x60af450; continue;
 }
 0x60af450 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.poll(); pc = 0x60af447; continue;
 }
 0x60af454 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60af486; continue; }
  pc = 0x60af45a; continue;
 }
 0x60af45a => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x60af47e; continue;
 }
 0x60af463 => {
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
  pc = 0x60af47e; continue;
 }
 0x60af47e => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60af463; continue; }
  pc = 0x60af483; continue;
 }
 0x60af483 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60af486; continue;
 }
 0x60af486 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60af4f7; continue; }
  pc = 0x60af499; continue;
 }
 0x60af499 => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x60af4d6; continue; }
  pc = 0x60af49d; continue;
 }
 0x60af49d => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60af4a3; continue;
 }
 0x60af4a3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x60af4b3; continue; }
  pc = 0x60af4af; continue;
 }
 0x60af4af => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x60af4a3; continue;
 }
 0x60af4b3 => {
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x60af4cd; continue; }
  pc = 0x60af4b8; continue;
 }
 0x60af4b8 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60af4c1; continue;
 }
 0x60af4c1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x60af4cd; continue; }
  pc = 0x60af4ca; continue;
 }
 0x60af4ca => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x60af4c1; continue;
 }
 0x60af4cd => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60af4f7; continue;
 }
 0x60af4d6 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x60af4e1; continue; }
  pc = 0x60af4e0; continue;
 }
 0x60af4e0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x60af4e1; continue;
 }
 0x60af4e1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if cpu.zf { pc = 0x60af4f2; continue; }
  pc = 0x60af4ea; continue;
 }
 0x60af4ea => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60af4f7; continue;
 }
 0x60af4f2 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.poll(); pc = 0x60af499; continue;
 }
 0x60af4f7 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060af4fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af4fe); }
 let mut pc: u32 = 0x60af4fe;
 loop { match pc {
 0x60af4fe => {
  let t = cpu.r32(0x617c77cu32); cpu.push(0x60af504); cpu.call(t); if cpu.returned_to != 0x60af504 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060af505(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af505); }
 let mut pc: u32 = 0x60af505;
 loop { match pc {
 0x60af505 => {
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
  if cpu.cf { pc = 0x60af54e; continue; }
  pc = 0x60af532; continue;
 }
 0x60af532 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af59f; continue; }
  pc = 0x60af538; continue;
 }
 0x60af538 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x60af549; continue; }
  pc = 0x60af53d; continue;
 }
 0x60af53d => {
  if (cpu.cf || cpu.zf) { pc = 0x60af55c; continue; }
  pc = 0x60af53f; continue;
 }
 0x60af53f => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  pc = 0x60af542; continue;
 }
 0x60af542 => {
  if cpu.zf { pc = 0x60af55c; continue; }
  pc = 0x60af544; continue;
 }
 0x60af544 => {
  pc = 0x60af5a9; continue;
 }
 0x60af549 => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  cpu.poll(); pc = 0x60af542; continue;
 }
 0x60af54e => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x60af5a9; continue; }
  pc = 0x60af553; continue;
 }
 0x60af553 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af55c; continue; }
  pc = 0x60af555; continue;
 }
 0x60af555 => {
  { cpu.sub8((cpu.edx as u8), 0x64u8); }
  if cpu.zf { pc = 0x60af59f; continue; }
  pc = 0x60af55a; continue;
 }
 0x60af55a => {
  pc = 0x60af5a9; continue;
 }
 0x60af55c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af576; continue; }
  pc = 0x60af56a; continue;
 }
 0x60af56a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  pc = 0x60af59b; continue;
 }
 0x60af576 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af591; continue; }
  pc = 0x60af584; continue;
 }
 0x60af584 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  pc = 0x60af59b; continue;
 }
 0x60af591 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x60af59b; continue;
 }
 0x60af59b => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af59f; continue;
 }
 0x60af59f => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x60af5a9; continue; }
  pc = 0x60af5a5; continue;
 }
 0x60af5a5 => {
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af5a9; continue;
 }
 0x60af5a9 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.cf { pc = 0x60af616; continue; }
  pc = 0x60af5b6; continue;
 }
 0x60af5b6 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af696; continue; }
  pc = 0x60af5bc; continue;
 }
 0x60af5bc => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  if cpu.cf { pc = 0x60af5fd; continue; }
  pc = 0x60af5c1; continue;
 }
 0x60af5c1 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af6ac; continue; }
  pc = 0x60af5c7; continue;
 }
 0x60af5c7 => {
  { cpu.sub8((cpu.edx as u8), 0x73u8); }
  if cpu.cf { pc = 0x60af5ef; continue; }
  pc = 0x60af5cc; continue;
 }
 0x60af5cc => {
  if (cpu.cf || cpu.zf) { pc = 0x60af741; continue; }
  pc = 0x60af5d2; continue;
 }
 0x60af5d2 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x60af913; continue; }
  pc = 0x60af5db; continue;
 }
 0x60af5db => {
  if (cpu.cf || cpu.zf) { pc = 0x60af82d; continue; }
  pc = 0x60af5e1; continue;
 }
 0x60af5e1 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  if cpu.zf { pc = 0x60af80a; continue; }
  pc = 0x60af5ea; continue;
 }
 0x60af5ea => {
  pc = 0x60af913; continue;
 }
 0x60af5ef => {
  { cpu.sub8((cpu.edx as u8), 0x70u8); }
  if cpu.zf { pc = 0x60af850; continue; }
  pc = 0x60af5f8; continue;
 }
 0x60af5f8 => {
  pc = 0x60af913; continue;
 }
 0x60af5fd => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.cf { pc = 0x60af664; continue; }
  pc = 0x60af602; continue;
 }
 0x60af602 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af696; continue; }
  pc = 0x60af608; continue;
 }
 0x60af608 => {
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.zf { pc = 0x60af6f0; continue; }
  pc = 0x60af611; continue;
 }
 0x60af611 => {
  pc = 0x60af913; continue;
 }
 0x60af616 => {
  { cpu.sub8((cpu.edx as u8), 0x50u8); }
  if cpu.cf { pc = 0x60af64e; continue; }
  pc = 0x60af61b; continue;
 }
 0x60af61b => {
  if (cpu.cf || cpu.zf) { pc = 0x60af850; continue; }
  pc = 0x60af621; continue;
 }
 0x60af621 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x60af640; continue; }
  pc = 0x60af626; continue;
 }
 0x60af626 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af80a; continue; }
  pc = 0x60af62c; continue;
 }
 0x60af62c => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x60af913; continue; }
  pc = 0x60af635; continue;
 }
 0x60af635 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af8e7; continue; }
  pc = 0x60af63b; continue;
 }
 0x60af63b => {
  pc = 0x60af6f0; continue;
 }
 0x60af640 => {
  { cpu.sub8((cpu.edx as u8), 0x53u8); }
  if cpu.zf { pc = 0x60af741; continue; }
  pc = 0x60af649; continue;
 }
 0x60af649 => {
  pc = 0x60af913; continue;
 }
 0x60af64e => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x60af65f; continue; }
  pc = 0x60af653; continue;
 }
 0x60af653 => {
  if (cpu.cf || cpu.zf) { pc = 0x60af664; continue; }
  pc = 0x60af655; continue;
 }
 0x60af655 => {
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  pc = 0x60af658; continue;
 }
 0x60af658 => {
  if cpu.zf { pc = 0x60af696; continue; }
  pc = 0x60af65a; continue;
 }
 0x60af65a => {
  pc = 0x60af913; continue;
 }
 0x60af65f => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  cpu.poll(); pc = 0x60af658; continue;
 }
 0x60af664 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af696; continue; }
  pc = 0x60af672; continue;
 }
 0x60af672 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x60af685); f_060af409(cpu); if cpu.returned_to != 0x60af685 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x60af6e6; continue;
 }
 0x60af696 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60af6a1); f_060af4fe(cpu); if cpu.returned_to != 0x60af6a1 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x60af6e6; continue;
 }
 0x60af6ac => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60af6c1; continue; }
  pc = 0x60af6bd; continue;
 }
 0x60af6bd => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x60af6c1; continue;
 }
 0x60af6c1 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60af6ca); f_060bd98d(cpu); if cpu.returned_to != 0x60af6ca { return; }
  pc = 0x60af6ca; continue;
 }
 0x60af6ca => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x60af6dd; continue; }
  pc = 0x60af6d0; continue;
 }
 0x60af6d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x60af6dd; continue; }
  pc = 0x60af6d9; continue;
 }
 0x60af6d9 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60af6dd; continue;
 }
 0x60af6dd => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60af6e6; continue;
 }
 0x60af6e6 => {
  cpu.push(0x60af6eb); f_060af364(cpu); if cpu.returned_to != 0x60af6eb { return; }
  pc = 0x60af93c; continue;
 }
 0x60af6f0 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af711; continue; }
  pc = 0x60af6fe; continue;
 }
 0x60af6fe => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60af70a; continue;
 }
 0x60af70a => {
  cpu.push(0x60af70f); f_060bd98d(cpu); if cpu.returned_to != 0x60af70f { return; }
  cpu.poll(); pc = 0x60af6ca; continue;
 }
 0x60af711 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af72e; continue; }
  pc = 0x60af71f; continue;
 }
 0x60af71f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60af70a; continue;
 }
 0x60af72e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x60af73f); f_060ad93c(cpu); if cpu.returned_to != 0x60af73f { return; }
  cpu.poll(); pc = 0x60af6ca; continue;
 }
 0x60af741 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af772; continue; }
  pc = 0x60af74f; continue;
 }
 0x60af74f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60af766; continue; }
  pc = 0x60af761; continue;
 }
 0x60af761 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x60af76a; continue; }
  pc = 0x60af766; continue;
 }
 0x60af766 => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  pc = 0x60af7a8; continue;
 }
 0x60af76a => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x60af7a6; continue;
 }
 0x60af772 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x40u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af792; continue; }
  pc = 0x60af780; continue;
 }
 0x60af780 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x60af76a; continue; }
  pc = 0x60af78e; continue;
 }
 0x60af78e => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  pc = 0x60af7a6; continue;
 }
 0x60af792 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x60af78e; continue; }
  pc = 0x60af7a0; continue;
 }
 0x60af7a0 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x60af7a6; continue;
 }
 0x60af7a6 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  pc = 0x60af7a8; continue;
 }
 0x60af7a8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xf9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x60af7e2; continue; }
  pc = 0x60af7c0; continue;
 }
 0x60af7c0 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60af7d9; continue; }
  pc = 0x60af7cb; continue;
 }
 0x60af7cb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x60af93c; continue;
 }
 0x60af7d9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x60af93c; continue;
 }
 0x60af7e2 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60af7fe; continue; }
  pc = 0x60af7ed; continue;
 }
 0x60af7ed => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x60af7f9); f_060af388(cpu); if cpu.returned_to != 0x60af7f9 { return; }
  pc = 0x60af93c; continue;
 }
 0x60af7fe => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.poll(); pc = 0x60af6e6; continue;
 }
 0x60af80a => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60af828; continue; }
  pc = 0x60af816; continue;
 }
 0x60af816 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60af828; continue; }
  pc = 0x60af81a; continue;
 }
 0x60af81a => {
  { let v: u8 = 0x30u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af828; continue;
 }
 0x60af828 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x60af82d; continue;
 }
 0x60af82d => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60af836); f_060bd943(cpu); if cpu.returned_to != 0x60af836 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60af6ca; continue; }
  pc = 0x60af844; continue;
 }
 0x60af844 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60af84b); f_060af9ee(cpu); if cpu.returned_to != 0x60af84b { return; }
  cpu.poll(); pc = 0x60af6ca; continue;
 }
 0x60af850 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x60af874; continue; }
  pc = 0x60af856; continue;
 }
 0x60af856 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af86d; continue; }
  pc = 0x60af864; continue;
 }
 0x60af864 => {
  { let v: u32 = 0xdu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af874; continue;
 }
 0x60af86d => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af874; continue;
 }
 0x60af874 => {
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
  if cpu.zf { pc = 0x60af8bf; continue; }
  pc = 0x60af896; continue;
 }
 0x60af896 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x60af8af); f_060af3ab(cpu); if cpu.returned_to != 0x60af8af { return; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x60af8c8; continue;
 }
 0x60af8bf => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x60af8c8; continue;
 }
 0x60af8c8 => {
  cpu.push(0x60af8cd); f_060af3ab(cpu); if cpu.returned_to != 0x60af8cd { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60af6dd; continue; }
  pc = 0x60af8db; continue;
 }
 0x60af8db => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60af8e2); f_060af9ee(cpu); if cpu.returned_to != 0x60af8e2 { return; }
  cpu.poll(); pc = 0x60af6dd; continue;
 }
 0x60af8e7 => {
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
  pc = 0x60af93c; continue;
 }
 0x60af913 => {
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
  pc = 0x60af93c; continue;
 }
 0x60af93c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60af991; continue; }
  pc = 0x60af942; continue;
 }
 0x60af942 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x60af954; continue; }
  pc = 0x60af94c; continue;
 }
 0x60af94c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x60af97a; continue;
 }
 0x60af954 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af968; continue; }
  pc = 0x60af962; continue;
 }
 0x60af962 => {
  { let v: u8 = 0x20u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af97a; continue;
 }
 0x60af968 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60af97e; continue; }
  pc = 0x60af976; continue;
 }
 0x60af976 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af97a; continue;
 }
 0x60af97a => {
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af97e; continue;
 }
 0x60af97e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60af98a; continue; }
  pc = 0x60af985; continue;
 }
 0x60af985 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af991; continue;
 }
 0x60af98a => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x60af991; continue;
 }
 0x60af991 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x60af9a8; continue; }
  pc = 0x60af99b; continue;
 }
 0x60af99b => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af9a8; continue;
 }
 0x60af9a8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60af9b4; continue; }
  pc = 0x60af9b0; continue;
 }
 0x60af9b0 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x60af9c1; continue; }
  pc = 0x60af9b4; continue;
 }
 0x60af9b4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if cpu.zf { pc = 0x60af9c1; continue; }
  pc = 0x60af9be; continue;
 }
 0x60af9be => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x60af9c1; continue;
 }
 0x60af9c1 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x60af9d0); f_060af364(cpu); if cpu.returned_to != 0x60af9d0 { return; }
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
pub fn f_060af9ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60af9ee); }
 let mut pc: u32 = 0x60af9ee;
 loop { match pc {
 0x60af9ee => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x60af9f1; continue;
 }
 0x60af9f1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x60afa04; continue; }
  pc = 0x60af9f6; continue;
 }
 0x60af9f6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x60af9ff); f_060a3794(cpu); if cpu.returned_to != 0x60af9ff { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x60af9f1; continue;
 }
 0x60afa04 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060afa10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60afa10); }
 let mut pc: u32 = 0x60afa10;
 loop { match pc {
 0x60afa10 => {
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
pub fn f_060afa29(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60afa29); }
 let mut pc: u32 = 0x60afa29;
 loop { match pc {
 0x60afa29 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x60afa2e); cpu.call(t); if cpu.returned_to != 0x60afa2e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060afa30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60afa30); }
 let mut pc: u32 = 0x60afa30;
 loop { match pc {
 0x60afa30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x60afa36); cpu.call(t); if cpu.returned_to != 0x60afa36 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060afa38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60afa38); }
 let mut pc: u32 = 0x60afa38;
 loop { match pc {
 0x60afa38 => {
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
  pc = 0x60afa57; continue;
 }
 0x60afa57 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  if cpu.zf { pc = 0x60afc5f; continue; }
  pc = 0x60afa6e; continue;
 }
 0x60afa6e => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60afa94; continue; }
  pc = 0x60afa86; continue;
 }
 0x60afa86 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afa8d); f_060afd74(cpu); if cpu.returned_to != 0x60afa8d { return; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x60afc1f; continue;
 }
 0x60afa94 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x60afabe; continue; }
  pc = 0x60afa99; continue;
 }
 0x60afa99 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afaa0); f_060afa29(cpu); if cpu.returned_to != 0x60afaa0 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x60afc1e; continue; }
  pc = 0x60afaa8; continue;
 }
 0x60afaa8 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x60afc5f; continue; }
  pc = 0x60afab2; continue;
 }
 0x60afab2 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x60afab9); f_060afa30(cpu); if cpu.returned_to != 0x60afab9 { return; }
  pc = 0x60afc5f; continue;
 }
 0x60afabe => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60afac7); f_060afc7f(cpu); if cpu.returned_to != 0x60afac7 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60afad6; continue; }
  pc = 0x60afad2; continue;
 }
 0x60afad2 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60afad6; continue;
 }
 0x60afad6 => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if cpu.cf { pc = 0x60afb30; continue; }
  pc = 0x60afadb; continue;
 }
 0x60afadb => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60afbbc; continue; }
  pc = 0x60afae4; continue;
 }
 0x60afae4 => {
  { cpu.sub32(cpu.ebx, 0x70u32); }
  if cpu.cf { pc = 0x60afb16; continue; }
  pc = 0x60afae9; continue;
 }
 0x60afae9 => {
  if (cpu.cf || cpu.zf) { pc = 0x60afbb0; continue; }
  pc = 0x60afaef; continue;
 }
 0x60afaef => {
  { cpu.sub32(cpu.ebx, 0x75u32); }
  if cpu.cf { pc = 0x60afb08; continue; }
  pc = 0x60afaf4; continue;
 }
 0x60afaf4 => {
  if (cpu.cf || cpu.zf) { pc = 0x60afba3; continue; }
  pc = 0x60afafa; continue;
 }
 0x60afafa => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x60afbb0; continue; }
  pc = 0x60afb03; continue;
 }
 0x60afb03 => {
  pc = 0x60afc1f; continue;
 }
 0x60afb08 => {
  { cpu.sub32(cpu.ebx, 0x73u32); }
  if cpu.zf { pc = 0x60afbc7; continue; }
  pc = 0x60afb11; continue;
 }
 0x60afb11 => {
  pc = 0x60afc1f; continue;
 }
 0x60afb16 => {
  { cpu.sub32(cpu.ebx, 0x6eu32); }
  if cpu.cf { pc = 0x60afb26; continue; }
  pc = 0x60afb1b; continue;
 }
 0x60afb1b => {
  if (cpu.cf || cpu.zf) { pc = 0x60afc02; continue; }
  pc = 0x60afb21; continue;
 }
 0x60afb21 => {
  pc = 0x60afb97; continue;
 }
 0x60afb26 => {
  { cpu.sub32(cpu.ebx, 0x69u32); }
  if cpu.zf { pc = 0x60afb8a; continue; }
  pc = 0x60afb2b; continue;
 }
 0x60afb2b => {
  pc = 0x60afc1f; continue;
 }
 0x60afb30 => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if cpu.cf { pc = 0x60afb56; continue; }
  pc = 0x60afb35; continue;
 }
 0x60afb35 => {
  if (cpu.cf || cpu.zf) { pc = 0x60afbb0; continue; }
  pc = 0x60afb3b; continue;
 }
 0x60afb3b => {
  { cpu.sub32(cpu.ebx, 0x63u32); }
  if cpu.cf { pc = 0x60afb48; continue; }
  pc = 0x60afb40; continue;
 }
 0x60afb40 => {
  if (cpu.cf || cpu.zf) { pc = 0x60afbdf; continue; }
  pc = 0x60afb46; continue;
 }
 0x60afb46 => {
  pc = 0x60afb75; continue;
 }
 0x60afb48 => {
  { cpu.sub32(cpu.ebx, 0x5bu32); }
  if cpu.zf { pc = 0x60afbd2; continue; }
  pc = 0x60afb51; continue;
 }
 0x60afb51 => {
  pc = 0x60afc1f; continue;
 }
 0x60afb56 => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.cf { pc = 0x60afb67; continue; }
  pc = 0x60afb5b; continue;
 }
 0x60afb5b => {
  if (cpu.cf || cpu.zf) { pc = 0x60afbbc; continue; }
  pc = 0x60afb5d; continue;
 }
 0x60afb5d => {
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x60afbbc; continue; }
  pc = 0x60afb62; continue;
 }
 0x60afb62 => {
  pc = 0x60afc1f; continue;
 }
 0x60afb67 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x60afc0f; continue; }
  pc = 0x60afb70; continue;
 }
 0x60afb70 => {
  pc = 0x60afc1f; continue;
 }
 0x60afb75 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x60afb7f; continue;
 }
 0x60afb7f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60afb83; continue;
 }
 0x60afb83 => {
  cpu.push(0x60afb88); f_060b041d(cpu); if cpu.returned_to != 0x60afb88 { return; }
  pc = 0x60afbe8; continue;
 }
 0x60afb8a => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.poll(); pc = 0x60afb83; continue;
 }
 0x60afb97 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x60afb7f; continue;
 }
 0x60afba3 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x60afb83; continue;
 }
 0x60afbb0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x60afb7f; continue;
 }
 0x60afbbc => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afbc5); f_060b0106(cpu); if cpu.returned_to != 0x60afbc5 { return; }
  pc = 0x60afbe8; continue;
 }
 0x60afbc7 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afbd0); f_060afe48(cpu); if cpu.returned_to != 0x60afbd0 { return; }
  pc = 0x60afbe8; continue;
 }
 0x60afbd2 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afbdd); f_060b000c(cpu); if cpu.returned_to != 0x60afbdd { return; }
  pc = 0x60afbe8; continue;
 }
 0x60afbdf => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afbe8); f_060afdb3(cpu); if cpu.returned_to != 0x60afbe8 { return; }
  pc = 0x60afbe8; continue;
 }
 0x60afbe8 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60afc5f; continue; }
  pc = 0x60afbf2; continue;
 }
 0x60afbf2 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60afc1f; continue; }
  pc = 0x60afbfc; continue;
 }
 0x60afbfc => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60afc1f; continue;
 }
 0x60afc02 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afc0d); f_060aff6b(cpu); if cpu.returned_to != 0x60afc0d { return; }
  pc = 0x60afc1f; continue;
 }
 0x60afc0f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afc16); f_060afa29(cpu); if cpu.returned_to != 0x60afc16 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x60afaa8; continue; }
  pc = 0x60afc1e; continue;
 }
 0x60afc1e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60afc1f; continue;
 }
 0x60afc1f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x60afa57; continue; }
  pc = 0x60afc29; continue;
 }
 0x60afc29 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x25u32); }
  if !cpu.zf { pc = 0x60afc5f; continue; }
  pc = 0x60afc38; continue;
 }
 0x60afc38 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x60afc45); f_060afc7f(cpu); if cpu.returned_to != 0x60afc45 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x60afc5f; continue; }
  pc = 0x60afc54; continue;
 }
 0x60afc54 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60afc5f); f_060aff6b(cpu); if cpu.returned_to != 0x60afc5f { return; }
  pc = 0x60afc5f; continue;
 }
 0x60afc5f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60afc73; continue; }
  pc = 0x60afc66; continue;
 }
 0x60afc66 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x60afc73; continue; }
  pc = 0x60afc6c; continue;
 }
 0x60afc6c => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60afc77; continue;
 }
 0x60afc73 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x60afc77; continue;
 }
 0x60afc77 => {
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
pub fn f_060afc7f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60afc7f); }
 let mut pc: u32 = 0x60afc7f;
 loop { match pc {
 0x60afc7f => {
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
  if !cpu.zf { pc = 0x60afcb2; continue; }
  pc = 0x60afca8; continue;
 }
 0x60afca8 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60afcb2; continue;
 }
 0x60afcb2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60afd04; continue; }
  pc = 0x60afcd5; continue;
 }
 0x60afcd5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60afcd7; continue;
 }
 0x60afcd7 => {
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
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x60afcd7; continue; }
  pc = 0x60afd01; continue;
 }
 0x60afd01 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60afd04; continue;
 }
 0x60afd04 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x4eu32); }
  if !cpu.zf { pc = 0x60afd18; continue; }
  pc = 0x60afd10; continue;
 }
 0x60afd10 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60afd28; continue;
 }
 0x60afd18 => {
  { cpu.sub32(cpu.edx, 0x46u32); }
  if !cpu.zf { pc = 0x60afd28; continue; }
  pc = 0x60afd1d; continue;
 }
 0x60afd1d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60afd28; continue;
 }
 0x60afd28 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.cf { pc = 0x60afd42; continue; }
  pc = 0x60afd32; continue;
 }
 0x60afd32 => {
  if (cpu.cf || cpu.zf) { pc = 0x60afd49; continue; }
  pc = 0x60afd34; continue;
 }
 0x60afd34 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x60afd6d; continue; }
  pc = 0x60afd39; continue;
 }
 0x60afd39 => {
  if (cpu.cf || cpu.zf) { pc = 0x60afd56; continue; }
  pc = 0x60afd3b; continue;
 }
 0x60afd3b => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x60afd56; continue; }
  pc = 0x60afd40; continue;
 }
 0x60afd40 => {
  pc = 0x60afd6d; continue;
 }
 0x60afd42 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x60afd62; continue; }
  pc = 0x60afd47; continue;
 }
 0x60afd47 => {
  pc = 0x60afd6d; continue;
 }
 0x60afd49 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x10u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60afd6d; continue;
 }
 0x60afd56 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60afd6d; continue;
 }
 0x60afd62 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60afd6d; continue;
 }
 0x60afd6d => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060afd74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60afd74); }
 let mut pc: u32 = 0x60afd74;
 loop { match pc {
 0x60afd74 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60afd7b; continue;
 }
 0x60afd7b => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60afd82); f_060afa29(cpu); if cpu.returned_to != 0x60afd82 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60afda0; continue; }
  pc = 0x60afd9d; continue;
 }
 0x60afd9d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60afd7b; continue;
 }
 0x60afda0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x60afdad; continue; }
  pc = 0x60afda6; continue;
 }
 0x60afda6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60afdad); f_060afa30(cpu); if cpu.returned_to != 0x60afdad { return; }
  pc = 0x60afdad; continue;
 }
 0x60afdad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060afdb3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60afdb3); } r_060afdb3(cpu, 0x60afdb3); }
pub fn f_060afe48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60afe48); } r_060afdb3(cpu, 0x60afe48); }
pub fn f_060b0106(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b0106); } r_060afdb3(cpu, 0x60b0106); }
fn r_060afdb3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60afdb3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x60afdfa; continue; }
  pc = 0x60afdc3; continue;
 }
 0x60afdc3 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60afdd7; continue; }
  pc = 0x60afdcb; continue;
 }
 0x60afdcb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x60afdfa; continue;
 }
 0x60afdd7 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x60afdec; continue; }
  pc = 0x60afddc; continue;
 }
 0x60afddc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x60afdfa; continue;
 }
 0x60afdec => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x60afdfa; continue;
 }
 0x60afdfa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60afe09; continue; }
  pc = 0x60afe04; continue;
 }
 0x60afe04 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x60afe09; continue;
 }
 0x60afe09 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60afe3d; continue; }
  pc = 0x60afe0d; continue;
 }
 0x60afe0d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60afe14); f_060afa29(cpu); if cpu.returned_to != 0x60afe14 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x60afe3d; continue; }
  pc = 0x60afe1e; continue;
 }
 0x60afe1e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x60afe09; continue; }
  pc = 0x60afe24; continue;
 }
 0x60afe24 => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x60afe34; continue; }
  pc = 0x60afe28; continue;
 }
 0x60afe28 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x60afe09; continue;
 }
 0x60afe34 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x60afe09; continue;
 }
 0x60afe3d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60afe42; continue;
 }
 0x60afe42 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60afe48 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x60afe5f; continue; }
  pc = 0x60afe58; continue;
 }
 0x60afe58 => {
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60afe68; continue;
 }
 0x60afe5f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x10u8); }
  { let v: u8 = 0x1u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60afe68; continue;
 }
 0x60afe68 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x60afea1; continue; }
  pc = 0x60afe6f; continue;
 }
 0x60afe6f => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x60afe7f; continue; }
  pc = 0x60afe73; continue;
 }
 0x60afe73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x60afea1; continue;
 }
 0x60afe7f => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x60afe93; continue; }
  pc = 0x60afe83; continue;
 }
 0x60afe83 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x60afea1; continue;
 }
 0x60afe93 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x60afea1; continue;
 }
 0x60afea1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60afea3; continue;
 }
 0x60afea3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60afeaa); f_060afa29(cpu); if cpu.returned_to != 0x60afeaa { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60afec9; continue; }
  pc = 0x60afec6; continue;
 }
 0x60afec6 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60afea3; continue;
 }
 0x60afec9 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if cpu.zf { pc = 0x60afed6; continue; }
  pc = 0x60afecf; continue;
 }
 0x60afecf => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60aff40; continue;
 }
 0x60afed6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60aff36; continue; }
  pc = 0x60afee3; continue;
 }
 0x60afee3 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60aff0e; continue; }
  pc = 0x60afeec; continue;
 }
 0x60afeec => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60afeff; continue; }
  pc = 0x60afef7; continue;
 }
 0x60afef7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60aff06; continue;
 }
 0x60afeff => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60aff06; continue;
 }
 0x60aff06 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x60aff0e; continue;
 }
 0x60aff0e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60aff15); f_060b0634(cpu); if cpu.returned_to != 0x60aff15 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60aff40; continue; }
  pc = 0x60aff1d; continue;
 }
 0x60aff1d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x60afee3; continue; }
  pc = 0x60aff36; continue;
 }
 0x60aff36 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60aff40); f_060afa30(cpu); if cpu.returned_to != 0x60aff40 { return; }
  pc = 0x60aff40; continue;
 }
 0x60aff40 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x60aff61; continue; }
  pc = 0x60aff46; continue;
 }
 0x60aff46 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60aff61; continue; }
  pc = 0x60aff4a; continue;
 }
 0x60aff4a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60aff5b; continue; }
  pc = 0x60aff55; continue;
 }
 0x60aff55 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60aff61; continue;
 }
 0x60aff5b => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60aff61; continue;
 }
 0x60aff61 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x60afe42; continue;
 }
 0x60b0106 => {
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
  pc = 0x60b011d; continue;
 }
 0x60b011d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60b0124); f_060afa29(cpu); if cpu.returned_to != 0x60b0124 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x60b0144; continue; }
  pc = 0x60b013e; continue;
 }
 0x60b013e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x60u32.wrapping_add(cpu.esp))); cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b011d; continue;
 }
 0x60b0144 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b014e; continue;
 }
 0x60b014e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b034e; continue; }
  pc = 0x60b015f; continue;
 }
 0x60b015f => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x60b0169; continue; }
  pc = 0x60b0164; continue;
 }
 0x60b0164 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x60b0187; continue; }
  pc = 0x60b0169; continue;
 }
 0x60b0169 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b0176); f_060b0634(cpu); if cpu.returned_to != 0x60b0176 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b0187; continue;
 }
 0x60b0187 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x60b01a8; continue; }
  pc = 0x60b019f; continue;
 }
 0x60b019f => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x60b034e; continue; }
  pc = 0x60b01a8; continue;
 }
 0x60b01a8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b0210; continue; }
  pc = 0x60b01c8; continue;
 }
 0x60b01c8 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x60b01cd; continue;
 }
 0x60b01cd => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x60b01e7; continue; }
  pc = 0x60b01d8; continue;
 }
 0x60b01d8 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6au32.wrapping_add(cpu.esp)), 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6au32.wrapping_add(cpu.esp), v); }
  pc = 0x60b01e7; continue;
 }
 0x60b01e7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x60b01ef); f_060b0634(cpu); if cpu.returned_to != 0x60b01ef { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b01fa; continue;
 }
 0x60b01fa => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60b01cd; continue; }
  pc = 0x60b0210; continue;
 }
 0x60b0210 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x60b02c0; continue; }
  pc = 0x60b0219; continue;
 }
 0x60b0219 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b0222); f_060b0634(cpu); if cpu.returned_to != 0x60b0222 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b022e; continue;
 }
 0x60b022e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x60b024c; continue; }
  pc = 0x60b0232; continue;
 }
 0x60b0232 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b034e; continue; }
  pc = 0x60b024c; continue;
 }
 0x60b024c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x60b024d; continue;
 }
 0x60b024d => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b0277; continue; }
  pc = 0x60b0265; continue;
 }
 0x60b0265 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b026f); f_060b0634(cpu); if cpu.returned_to != 0x60b026f { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b024d; continue; }
  pc = 0x60b0277; continue;
 }
 0x60b0277 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x60b02b7; continue; }
  pc = 0x60b027d; continue;
 }
 0x60b027d => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b0285; continue;
 }
 0x60b0285 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x60b02ae; continue; }
  pc = 0x60b0290; continue;
 }
 0x60b0290 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b0285; continue;
 }
 0x60b02ae => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b02b7; continue;
 }
 0x60b02b7 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b02c0; continue;
 }
 0x60b02c0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x60b034e; continue; }
  pc = 0x60b02ca; continue;
 }
 0x60b02ca => {
  { cpu.sub32(cpu.ecx, 0x65u32); }
  if cpu.zf { pc = 0x60b02d8; continue; }
  pc = 0x60b02cf; continue;
 }
 0x60b02cf => {
  { cpu.sub32(cpu.ecx, 0x45u32); }
  if !cpu.zf { pc = 0x60b034e; continue; }
  pc = 0x60b02d8; continue;
 }
 0x60b02d8 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b02e2); f_060b0634(cpu); if cpu.returned_to != 0x60b02e2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b02ee; continue;
 }
 0x60b02ee => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x60b02f8; continue; }
  pc = 0x60b02f3; continue;
 }
 0x60b02f3 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x60b030a; continue; }
  pc = 0x60b02f8; continue;
 }
 0x60b02f8 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b0302); f_060b0634(cpu); if cpu.returned_to != 0x60b0302 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b030a; continue;
 }
 0x60b030a => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x60b0326; continue; }
  pc = 0x60b0322; continue;
 }
 0x60b0322 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b034e; continue;
 }
 0x60b0326 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b0330); f_060b0634(cpu); if cpu.returned_to != 0x60b0330 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0357; continue; }
  pc = 0x60b0338; continue;
 }
 0x60b0338 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60b0326; continue; }
  pc = 0x60b034e; continue;
 }
 0x60b034e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60b0357); f_060afa30(cpu); if cpu.returned_to != 0x60b0357 { return; }
  pc = 0x60b0357; continue;
 }
 0x60b0357 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b0412; continue; }
  pc = 0x60b035f; continue;
 }
 0x60b035f => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60b0412; continue; }
  pc = 0x60b0371; continue;
 }
 0x60b0371 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x60b038a; continue; }
  pc = 0x60b037a; continue;
 }
 0x60b037a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x60b0396; continue; }
  pc = 0x60b0384; continue;
 }
 0x60b0384 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x68u32.wrapping_add(cpu.esp))); cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b0396; continue;
 }
 0x60b038a => {
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  let t = cpu.r32(0x617c780u32); cpu.push(0x60b0396); cpu.call(t); if cpu.returned_to != 0x60b0396 { return; }
  pc = 0x60b0396; continue;
 }
 0x60b0396 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60b03ae; continue; }
  pc = 0x60b039e; continue;
 }
 0x60b039e => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebx = v; cpu.set_segment(0, selector); }
  pc = 0x60b03d9; continue;
 }
 0x60b03ae => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x60b03c7; continue; }
  pc = 0x60b03b3; continue;
 }
 0x60b03b3 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  pc = 0x60b03d9; continue;
 }
 0x60b03c7 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  pc = 0x60b03d9; continue;
 }
 0x60b03d9 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x60b03e7; continue; }
  pc = 0x60b03e1; continue;
 }
 0x60b03e1 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x60b040f; continue;
 }
 0x60b03e7 => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x60b03f1; continue; }
  pc = 0x60b03ec; continue;
 }
 0x60b03ec => {
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x60b0402; continue; }
  pc = 0x60b03f1; continue;
 }
 0x60b03f1 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60b0412; continue;
 }
 0x60b0402 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x60b040f); f_060bd9a8(cpu); if cpu.returned_to != 0x60b040f { return; }
  pc = 0x60b040f; continue;
 }
 0x60b040f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60b0412; continue;
 }
 0x60b0412 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x60afe42; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060aff6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60aff6b); }
 let mut pc: u32 = 0x60aff6b;
 loop { match pc {
 0x60aff6b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60affc4; continue; }
  pc = 0x60aff77; continue;
 }
 0x60aff77 => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60aff88; continue; }
  pc = 0x60aff7c; continue;
 }
 0x60aff7c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x60affad; continue;
 }
 0x60aff88 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x60aff9d; continue; }
  pc = 0x60aff8d; continue;
 }
 0x60aff8d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x60affad; continue;
 }
 0x60aff9d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x60affad; continue;
 }
 0x60affad => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x60affbe; continue; }
  pc = 0x60affb5; continue;
 }
 0x60affb5 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60affbe => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60affc4; continue;
 }
 0x60affc4 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060affc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60affc9); }
 let mut pc: u32 = 0x60affc9;
 loop { match pc {
 0x60affc9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x60affde); f_060ae9e0(cpu); if cpu.returned_to != 0x60affde { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b0006; continue; }
  pc = 0x60affe7; continue;
 }
 0x60affe7 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x616b600u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b0006; continue; }
  pc = 0x60b0000; continue;
 }
 0x60b0000 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x5du32); }
  if !cpu.zf { cpu.poll(); pc = 0x60affe7; continue; }
  pc = 0x60b0006; continue;
 }
 0x60b0006 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b000c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b000c); } r_060b000c(cpu, 0x60b000c); }
pub fn f_060b0101(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b0101); } r_060b000c(cpu, 0x60b0101); }
fn r_060b000c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b000c => {
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
  if cpu.zf { pc = 0x60b0033; continue; }
  pc = 0x60b0030; continue;
 }
 0x60b0030 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b0033; continue;
 }
 0x60b0033 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x60b003c); f_060affc9(cpu); if cpu.returned_to != 0x60b003c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60b007a; continue; }
  pc = 0x60b0046; continue;
 }
 0x60b0046 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x60b0057; continue; }
  pc = 0x60b004b; continue;
 }
 0x60b004b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.esi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(0, selector); }
  pc = 0x60b007a; continue;
 }
 0x60b0057 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x60b006c; continue; }
  pc = 0x60b005c; continue;
 }
 0x60b005c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x60b007a; continue;
 }
 0x60b006c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.esi = v; }
  pc = 0x60b007a; continue;
 }
 0x60b007a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b007f; continue;
 }
 0x60b007f => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x60b00ed; continue; }
  pc = 0x60b0083; continue;
 }
 0x60b0083 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60b008a); f_060afa29(cpu); if cpu.returned_to != 0x60b008a { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x60b00ed; continue; }
  pc = 0x60b0096; continue;
 }
 0x60b0096 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x616b600u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebp); }
  { let v: u8 = (cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.zf { pc = 0x60b00d9; continue; }
  pc = 0x60b00d0; continue;
 }
 0x60b00d0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x60b00d7); f_060afa30(cpu); if cpu.returned_to != 0x60b00d7 { return; }
  pc = 0x60b00ed; continue;
 }
 0x60b00d9 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x60b007f; continue; }
  pc = 0x60b00e3; continue;
 }
 0x60b00e3 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60b007f; continue;
 }
 0x60b00ed => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x60b00fb; continue; }
  pc = 0x60b00f3; continue;
 }
 0x60b00f3 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b00fb; continue; }
  pc = 0x60b00f7; continue;
 }
 0x60b00f7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60b00fb; continue;
 }
 0x60b00fb => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  pc = 0x60b0101; continue;
 }
 0x60b0101 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b041d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b041d); }
 let mut pc: u32 = 0x60b041d;
 loop { match pc {
 0x60b041d => {
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
  pc = 0x60b0437; continue;
 }
 0x60b0437 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60b043e); f_060afa29(cpu); if cpu.returned_to != 0x60b043e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x616b4d4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b0459; continue; }
  pc = 0x60b0454; continue;
 }
 0x60b0454 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b0437; continue;
 }
 0x60b0459 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b0463; continue;
 }
 0x60b0463 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b058c; continue; }
  pc = 0x60b0474; continue;
 }
 0x60b0474 => {
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b04a8; continue; }
  pc = 0x60b0485; continue;
 }
 0x60b0485 => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x60b048f; continue; }
  pc = 0x60b048a; continue;
 }
 0x60b048a => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if !cpu.zf { pc = 0x60b04a8; continue; }
  pc = 0x60b048f; continue;
 }
 0x60b048f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60b049d); f_060b0634(cpu); if cpu.returned_to != 0x60b049d { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b04a8; continue;
 }
 0x60b04a8 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b0504; continue; }
  pc = 0x60b04ac; continue;
 }
 0x60b04ac => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x60b04fd; continue; }
  pc = 0x60b04b1; continue;
 }
 0x60b04b1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x60b04bd); f_060b0634(cpu); if cpu.returned_to != 0x60b04bd { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b04c8; continue;
 }
 0x60b04c8 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x60b04d2; continue; }
  pc = 0x60b04cd; continue;
 }
 0x60b04cd => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x60b04f6; continue; }
  pc = 0x60b04d2; continue;
 }
 0x60b04d2 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  cpu.push(0x60b04dc); f_060b0634(cpu); if cpu.returned_to != 0x60b04dc { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b04ef; continue;
 }
 0x60b04ef => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x60b0548; continue;
 }
 0x60b04f6 => {
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x60b0548; continue;
 }
 0x60b04fd => {
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x60b0548; continue;
 }
 0x60b0504 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x60b0548; continue; }
  pc = 0x60b0509; continue;
 }
 0x60b0509 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x60b0548; continue; }
  pc = 0x60b050e; continue;
 }
 0x60b050e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x60b051a); f_060b0634(cpu); if cpu.returned_to != 0x60b051a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b0525; continue;
 }
 0x60b0525 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x60b052f; continue; }
  pc = 0x60b052a; continue;
 }
 0x60b052a => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x60b0548; continue; }
  pc = 0x60b052f; continue;
 }
 0x60b052f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x60b0539); f_060b0634(cpu); if cpu.returned_to != 0x60b0539 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b0541; continue;
 }
 0x60b0541 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b0548; continue;
 }
 0x60b0548 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60b054f); f_060b060d(cpu); if cpu.returned_to != 0x60b054f { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60b0562; continue; }
  pc = 0x60b0553; continue;
 }
 0x60b0553 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x60b0560); f_060b0634(cpu); if cpu.returned_to != 0x60b0560 { return; }
  cpu.poll(); pc = 0x60b0541; continue;
 }
 0x60b0562 => {
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if !cpu.zf { pc = 0x60b058c; continue; }
  pc = 0x60b0567; continue;
 }
 0x60b0567 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x60b058c; continue; }
  pc = 0x60b056d; continue;
 }
 0x60b056d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x60b0575); f_060b0634(cpu); if cpu.returned_to != 0x60b0575 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b0595; continue; }
  pc = 0x60b057c; continue;
 }
 0x60b057c => {
  cpu.push(0x60b0581); f_060b060d(cpu); if cpu.returned_to != 0x60b0581 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60b058c; continue; }
  pc = 0x60b0585; continue;
 }
 0x60b0585 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  cpu.poll(); pc = 0x60b056d; continue;
 }
 0x60b058c => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x60b0595); f_060afa30(cpu); if cpu.returned_to != 0x60b0595 { return; }
  pc = 0x60b0595; continue;
 }
 0x60b0595 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x2du32); }
  if !cpu.zf { pc = 0x60b059e; continue; }
  pc = 0x60b059c; continue;
 }
 0x60b059c => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x60b059e; continue;
 }
 0x60b059e => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b0603; continue; }
  pc = 0x60b05a2; continue;
 }
 0x60b05a2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60b0603; continue; }
  pc = 0x60b05af; continue;
 }
 0x60b05af => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x60b05c4; continue; }
  pc = 0x60b05b4; continue;
 }
 0x60b05b4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  pc = 0x60b05ef; continue;
 }
 0x60b05c4 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x60b05dd; continue; }
  pc = 0x60b05c9; continue;
 }
 0x60b05c9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x60b05ef; continue;
 }
 0x60b05dd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x60b05ef; continue;
 }
 0x60b05ef => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x60b05fd; continue; }
  pc = 0x60b05f7; continue;
 }
 0x60b05f7 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60b0603; continue;
 }
 0x60b05fd => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x60b0603; continue;
 }
 0x60b0603 => {
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
pub fn f_060b060d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b060d); } r_060b060d(cpu, 0x60b060d); }
pub fn f_060b0611(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b0611); } r_060b060d(cpu, 0x60b0611); }
fn r_060b060d(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b060d => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b061b; continue; }
  pc = 0x60b0612; continue;
 }
 0x60b0611 => {
  { let v: u32 = cpu.r32(0x47f39f8u32.wrapping_add(cpu.ebx)) | cpu.eax; cpu.w32(0x47f39f8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b0617; continue;
 }
 0x60b0612 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b061b; continue; }
  pc = 0x60b0617; continue;
 }
 0x60b0617 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b061b => {
  cpu.push(0x60b0620); f_060a555c(cpu); if cpu.returned_to != 0x60b0620 { return; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b062e; continue; }
  pc = 0x60b0625; continue;
 }
 0x60b0625 => {
  { cpu.sub32(cpu.eax, 0x66u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b062e; continue; }
  pc = 0x60b062a; continue;
 }
 0x60b062a => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x57u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b062e => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0634(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0634); }
 let mut pc: u32 = 0x60b0634;
 loop { match pc {
 0x60b0634 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b0652; continue; }
  pc = 0x60b0645; continue;
 }
 0x60b0645 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60b064c); f_060afa29(cpu); if cpu.returned_to != 0x60b064c { return; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x2u8); }
  if cpu.zf { pc = 0x60b0657; continue; }
  pc = 0x60b0652; continue;
 }
 0x60b0652 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b0657; continue;
 }
 0x60b0657 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b065a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b065a); }
 let mut pc: u32 = 0x60b065a;
 loop { match pc {
 0x60b065a => {
  { let v: u32 = cpu.r32(0x617b7b8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0714(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0714); }
 let mut pc: u32 = 0x60b0714;
 loop { match pc {
 0x60b0714 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b0721); f_060bd9f4(cpu); if cpu.returned_to != 0x60b0721 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x60b0736); f_060bde2e(cpu); if cpu.returned_to != 0x60b0736 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b07ae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b07ae); }
 let mut pc: u32 = 0x60b07ae;
 loop { match pc {
 0x60b07ae => {
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
  cpu.push(0x60b07e0); f_060b0714(cpu); if cpu.returned_to != 0x60b07e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x60b07f0; continue; }
  pc = 0x60b07ea; continue;
 }
 0x60b07ea => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b07f0 => {
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
pub fn f_060b083a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b083a); }
 let mut pc: u32 = 0x60b083a;
 loop { match pc {
 0x60b083a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x6u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60b0856); f_060b0714(cpu); if cpu.returned_to != 0x60b0856 { return; }
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
pub fn f_060b086a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b086a); }
 let mut pc: u32 = 0x60b086a;
 loop { match pc {
 0x60b0820 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60b0829); f_060b0714(cpu); if cpu.returned_to != 0x60b0829 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b086a => {
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
  cpu.poll(); pc = 0x60b0820; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0a6b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0a6b); }
 let mut pc: u32 = 0x60b0a6b;
 loop { match pc {
 0x60b0a6b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b0a7d); f_060bd9f4(cpu); if cpu.returned_to != 0x60b0a7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  cpu.push(0x60b0a89); f_060b083a(cpu); if cpu.returned_to != 0x60b0a89 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x60b0a9d; continue; }
  pc = 0x60b0a93; continue;
 }
 0x60b0a93 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b0a9d; continue; }
  pc = 0x60b0a97; continue;
 }
 0x60b0a97 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b0a9d => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60b0aa7); f_060b07ae(cpu); if cpu.returned_to != 0x60b0aa7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b0ab7; continue; }
  pc = 0x60b0aad; continue;
 }
 0x60b0aad => {
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x60b0acd; continue; }
  pc = 0x60b0ab5; continue;
 }
 0x60b0ab5 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x60b0ab7; continue;
 }
 0x60b0ab7 => {
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  cpu.push(0x60b0ac5); f_060b086a(cpu); if cpu.returned_to != 0x60b0ac5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b0acd; continue; }
  pc = 0x60b0ac9; continue;
 }
 0x60b0ac9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x60b0acd; continue;
 }
 0x60b0acd => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0dae(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0dae); }
 let mut pc: u32 = 0x60b0dae;
 loop { match pc {
 0x60b0dae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x60bdf9du32; cpu.edx = v; }
  { let v: u32 = 0x60be0e6u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617c77cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c780u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0dfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0dfa); }
 let mut pc: u32 = 0x60b0dfa;
 loop { match pc {
 0x60b0dfa => {
  { let v: u8 = 0x3u8; cpu.w8(0x617412cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x617b828u32.wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x60b0e08, "Smsw");
  { let v: u16 = cpu.r16(0x617b826u32.wrapping_add(cpu.bases[3])) & 0x6u16; cpu.w16(0x617b826u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x60b0e1b, "Sidt");
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x617b860u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x60b0e4c; continue; }
  pc = 0x60b0e32; continue;
 }
 0x60b0e32 => {
  cpu.flat_ds(0x60b0e32); cpu.push(0x60b0e37); f_060b0ec5(cpu); if cpu.returned_to != 0x60b0e37 { return; }
  { let v = cpu.r16(0x617b002u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  cpu.flat_ds(0x60b0e45); cpu.push(0x60b0e4a); f_060b0f2f(cpu); if cpu.returned_to != 0x60b0e4a { return; }
  pc = 0x60b0ec4; continue;
 }
 0x60b0e4c => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x60b0e70; continue; }
  pc = 0x60b0e55; continue;
 }
 0x60b0e55 => {
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x60be4dcu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x60b0ec4; continue;
 }
 0x60b0e70 => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x60b0e98; continue; }
  pc = 0x60b0e79; continue;
 }
 0x60b0e79 => {
  cpu.flat_ds(0x60b0e79); cpu.push(0x60b0e7e); f_060b0ec5(cpu); if cpu.returned_to != 0x60b0e7e { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617affcu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60b0e93); cpu.push(0x60b0e96); cpu.call(t); if cpu.returned_to != 0x60b0e96 { return; }
  pc = 0x60b0ec4; continue;
 }
 0x60b0e98 => {
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x60b0eb1; continue; }
  pc = 0x60b0ea1; continue;
 }
 0x60b0ea1 => {
  { let v: u16 = cpu.r16(0x617b000u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x60b0eaa); cpu.push(0x60b0eaf); f_060c131a(cpu); if cpu.returned_to != 0x60b0eaf { return; }
  pc = 0x60b0ec4; continue;
 }
 0x60b0eb1 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x60b0eb6); cpu.push(0x60b0ebb); f_060c131a(cpu); if cpu.returned_to != 0x60b0ebb { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x60b0ec4; continue; }
  pc = 0x60b0ebf; continue;
 }
 0x60b0ebf => {
  cpu.flat_ds(0x60b0ebf); cpu.push(0x60b0ec4); f_060b0ee2(cpu); if cpu.returned_to != 0x60b0ec4 { return; }
  pc = 0x60b0ec4; continue;
 }
 0x60b0ec4 => {
  cpu.flat_ds(0x60b0ec4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0ec5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0ec5); }
 let mut pc: u32 = 0x60b0ec5;
 loop { match pc {
 0x60b0ec5 => {
  { let v: u32 = 0x60be4dcu32; cpu.ecx = v; }
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
pub fn f_060b0ee2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0ee2); }
 let mut pc: u32 = 0x60b0ee2;
 loop { match pc {
 0x60b0ee2 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b820u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x617b824u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.sf != cpu.of) { pc = 0x60b0f19; continue; }
  pc = 0x60b0f09; continue;
 }
 0x60b0f09 => {
  { let v: u32 = 0x60be46cu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60b0f12); cpu.push(0x60b0f17); f_060b0f2f(cpu); if cpu.returned_to != 0x60b0f17 { return; }
  pc = 0x60b0f2e; continue;
 }
 0x60b0f19 => {
  { let v: u32 = 0x60be4dcu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffdu32; cpu.ecx = v; }
  {  }
  pc = 0x60b0f2e; continue;
 }
 0x60b0f2e => {
  cpu.flat_ds(0x60b0f2e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0f2f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0f2f); }
 let mut pc: u32 = 0x60b0f2f;
 loop { match pc {
 0x60b0f2f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x617b7e0u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0x617b7e0u32) | 0x4u32; cpu.w32(0x617b7e0u32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x617b7e0u32), 0xfffffffdu32); cpu.w32(0x617b7e0u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b0fc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b0fc9); }
 let mut pc: u32 = 0x60b0fc9;
 loop { match pc {
 0x60b0fc9 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x617b820u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x617b824u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.sub8(cpu.r8(0x617b85eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x60b0ff0; continue; }
  pc = 0x60b0fe9; continue;
 }
 0x60b0fe9 => {
  cpu.flat_ds(0x60b0fe9); cpu.push(0x60b0fee); f_060b1004(cpu); if cpu.returned_to != 0x60b0fee { return; }
  pc = 0x60b1003; continue;
 }
 0x60b0ff0 => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff9u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x617b826u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  {  }
  pc = 0x60b1003; continue;
 }
 0x60b1003 => {
  cpu.flat_ds(0x60b1003); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1004(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1004); }
 let mut pc: u32 = 0x60b1004;
 loop { match pc {
 0x60b1004 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x617b7e0u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x617b826u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617b7e0u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1025(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1025); }
 let mut pc: u32 = 0x60b1025;
 loop { match pc {
 0x60b1025 => {
  {  }
  cpu.unimplemented(0x60b1026, "Fnsave");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b102a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b102a); }
 let mut pc: u32 = 0x60b102a;
 loop { match pc {
 0x60b102a => {
  cpu.unimplemented(0x60b102a, "Frstor");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b102e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b102e); }
 let mut pc: u32 = 0x60b102e;
 loop { match pc {
 0x60b102e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x617412du32), 0x0u8); }
  if cpu.zf { pc = 0x60b104f; continue; }
  pc = 0x60b1039; continue;
 }
 0x60b1039 => {
  { let v: u32 = 0x60b1025u32; cpu.edx = v; }
  { let v: u32 = 0x60b102au32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617c80cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c810u32, v); }
  pc = 0x60b104f; continue;
 }
 0x60b104f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x617c808u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60b105c); f_060c1448(cpu); if cpu.returned_to != 0x60b105c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b105f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b105f); }
 let mut pc: u32 = 0x60b105f;
 loop { match pc {
 0x60b105f => {
  { cpu.sub8(cpu.r8(0x617412du32), 0x0u8); }
  if !cpu.zf { f_060b102e(cpu); return; }
  pc = 0x60b1068; continue;
 }
 0x60b1068 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1069(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1069); }
 let mut pc: u32 = 0x60b1069;
 loop { match pc {
 0x60b1069 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x617412cu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b10b1; continue; }
  pc = 0x60b1074; continue;
 }
 0x60b1074 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x617412du32, v); }
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
  if !cpu.zf { pc = 0x60b109c; continue; }
  pc = 0x60b1093; continue;
 }
 0x60b1093 => {
  cpu.push(0x60b1098); f_060b102e(cpu); if cpu.returned_to != 0x60b1098 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b109c; continue;
 }
 0x60b109c => {
  { cpu.sub8(cpu.r8(0x617b85cu32), 0x0u8); }
  if !cpu.zf { pc = 0x60b10b1; continue; }
  pc = 0x60b10a5; continue;
 }
 0x60b10a5 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x617412cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x617412du32, v); }
  pc = 0x60b10b1; continue;
 }
 0x60b10b1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b10b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b10b3); }
 let mut pc: u32 = 0x60b10b3;
 loop { match pc {
 0x60b10b3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b10b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b10b4); }
 let mut pc: u32 = 0x60b10b4;
 loop { match pc {
 0x60b10b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x60b10c7); f_060ae9e0(cpu); if cpu.returned_to != 0x60b10c7 { return; }
  pc = 0x60b10e8; continue;
 }
 0x60b10c9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x616b608u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x60b10e8; continue;
 }
 0x60b10e8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60b10c9; continue; }
  pc = 0x60b10ee; continue;
 }
 0x60b10ee => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b10f2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b10f2); }
 let mut pc: u32 = 0x60b10f2;
 loop { match pc {
 0x60b10f2 => {
  if !cpu.cf { pc = 0x60b1102; continue; }
  pc = 0x60b10f4; continue;
 }
 0x60b10f4 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b10ff); f_060c147a(cpu); if cpu.returned_to != 0x60b10ff { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x60b1104; continue;
 }
 0x60b1102 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60b1104; continue;
 }
 0x60b1104 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1105(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1105); }
 let mut pc: u32 = 0x60b1105;
 loop { match pc {
 0x60b1105 => {
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
  cpu.push(0x60b1123); f_060b1129(cpu); if cpu.returned_to != 0x60b1123 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1129(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1129); }
 let mut pc: u32 = 0x60b1129;
 loop { match pc {
 0x60b1129 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x60b1135; continue;
 }
 0x60b1135 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x60b1140; continue; }
  pc = 0x60b113d; continue;
 }
 0x60b113d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x60b1135; continue;
 }
 0x60b1140 => {
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
  if (cpu.sf != cpu.of) { pc = 0x60b1173; continue; }
  pc = 0x60b116d; continue;
 }
 0x60b116d => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x60b1173; continue;
 }
 0x60b1173 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x3u8); }
  if cpu.zf { pc = 0x60b11f6; continue; }
  pc = 0x60b1180; continue;
 }
 0x60b1180 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b11f6; continue; }
  pc = 0x60b1189; continue;
 }
 0x60b1189 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60b1190); f_060c14eb(cpu); if cpu.returned_to != 0x60b1190 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b11f6; continue; }
  pc = 0x60b1194; continue;
 }
 0x60b1194 => {
  { cpu.and8(cpu.r8(0x29u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x60b11be; continue; }
  pc = 0x60b119b; continue;
 }
 0x60b119b => {
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x60b11be; continue; }
  pc = 0x60b11a0; continue;
 }
 0x60b11a0 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x60b11b4); f_060aec9b(cpu); if cpu.returned_to != 0x60b11b4 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b1309; continue;
 }
 0x60b11be => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x40u8); }
  if cpu.zf { pc = 0x60b11f6; continue; }
  pc = 0x60b11c5; continue;
 }
 0x60b11c5 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b11f6; continue; }
  pc = 0x60b11dc; continue;
 }
 0x60b11dc => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x60b11e6; continue;
 }
 0x60b11e6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60b11f1); f_060c147a(cpu); if cpu.returned_to != 0x60b11f1 { return; }
  pc = 0x60b1309; continue;
 }
 0x60b11f6 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b1298; continue; }
  pc = 0x60b11ff; continue;
 }
 0x60b11ff => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x60b11e6; continue; }
  pc = 0x60b1206; continue;
 }
 0x60b1206 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b11e6; continue; }
  pc = 0x60b1211; continue;
 }
 0x60b1211 => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b1224; continue; }
  pc = 0x60b121f; continue;
 }
 0x60b121f => {
  { let v: u32 = 0x180u32; cpu.eax = v; }
  pc = 0x60b1224; continue;
 }
 0x60b1224 => {
  { let v: u32 = cpu.r32(0x617c828u32); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x60b1239; continue; }
  pc = 0x60b1234; continue;
 }
 0x60b1234 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x60b1239; continue;
 }
 0x60b1239 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b11e6; continue; }
  pc = 0x60b1250; continue;
 }
 0x60b1250 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b1298; continue; }
  pc = 0x60b125e; continue;
 }
 0x60b125e => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b11e6; continue; }
  pc = 0x60b1273; continue;
 }
 0x60b1273 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b11e6; continue; }
  pc = 0x60b1292; continue;
 }
 0x60b1292 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x60b1298; continue;
 }
 0x60b1298 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60b129f); f_060c1512(cpu); if cpu.returned_to != 0x60b129f { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60b12aa); f_060c14eb(cpu); if cpu.returned_to != 0x60b12aa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b12b1; continue; }
  pc = 0x60b12ae; continue;
 }
 0x60b12ae => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b12b1; continue;
 }
 0x60b12b1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x60b12c0; continue; }
  pc = 0x60b12bd; continue;
 }
 0x60b12bd => {
  { let v: u8 = (cpu.edx as u8) | 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60b12c0; continue;
 }
 0x60b12c0 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b12ca; continue; }
  pc = 0x60b12c7; continue;
 }
 0x60b12c7 => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60b12ca; continue;
 }
 0x60b12ca => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x60b12d4; continue; }
  pc = 0x60b12d1; continue;
 }
 0x60b12d1 => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60b12d4; continue;
 }
 0x60b12d4 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x60b12de; continue; }
  pc = 0x60b12db; continue;
 }
 0x60b12db => {
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60b12de; continue;
 }
 0x60b12de => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if cpu.zf { pc = 0x60b12f2; continue; }
  pc = 0x60b12eb; continue;
 }
 0x60b12eb => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x60b1300; continue; }
  pc = 0x60b12f0; continue;
 }
 0x60b12f0 => {
  pc = 0x60b12fe; continue;
 }
 0x60b12f2 => {
  { cpu.sub32(cpu.r32(0x617b22du32), 0x200u32); }
  if !cpu.zf { pc = 0x60b1300; continue; }
  pc = 0x60b12fe; continue;
 }
 0x60b12fe => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x60b1300; continue;
 }
 0x60b1300 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60b1307); f_060c1567(cpu); if cpu.returned_to != 0x60b1307 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60b1309; continue;
 }
 0x60b1309 => {
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
pub fn f_060b1312(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1312); }
 let mut pc: u32 = 0x60b1312;
 loop { match pc {
 0x60b1312 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x60b1320); f_060c1512(cpu); if cpu.returned_to != 0x60b1320 { return; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60b132d); f_060c1567(cpu); if cpu.returned_to != 0x60b132d { return; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x60b136d; continue; }
  pc = 0x60b1332; continue;
 }
 0x60b1332 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60b1352; continue; }
  pc = 0x60b1345; continue;
 }
 0x60b1345 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60b1350); f_060c147a(cpu); if cpu.returned_to != 0x60b1350 { return; }
  pc = 0x60b136f; continue;
 }
 0x60b1352 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b1345; continue; }
  pc = 0x60b136d; continue;
 }
 0x60b136d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b136f; continue;
 }
 0x60b136f => {
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
pub fn f_060b1377(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1377); }
 let mut pc: u32 = 0x60b1377;
 loop { match pc {
 0x60b1377 => {
  { let v: u32 = 0x62d15dcu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b137d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b137d); }
 let mut pc: u32 = 0x60b137d;
 loop { match pc {
 0x60b137d => {
  { let v: u32 = 0x62d15e0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1383(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1383); }
 let mut pc: u32 = 0x60b1383;
 loop { match pc {
 0x60b1383 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x60b1393; continue; }
  pc = 0x60b1387; continue;
 }
 0x60b1387 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x617c82cu32)); }
  if (cpu.cf || cpu.zf) { f_060c157c(cpu); return; }
  pc = 0x60b1393; continue;
 }
 0x60b1393 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x60b139d); f_060aec9b(cpu); if cpu.returned_to != 0x60b139d { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b13a3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b13a3); }
 let mut pc: u32 = 0x60b13a3;
 loop { match pc {
 0x60b13a3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x60b13b2); f_060c1512(cpu); if cpu.returned_to != 0x60b13b2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b13ce; continue; }
  pc = 0x60b13ba; continue;
 }
 0x60b13ba => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x60b13bf; continue;
 }
 0x60b13bf => {
  cpu.push(0x60b13c4); f_060aec9b(cpu); if cpu.returned_to != 0x60b13c4 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b1496; continue;
 }
 0x60b13ce => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x60b13d9; continue; }
  pc = 0x60b13d2; continue;
 }
 0x60b13d2 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60b13bf; continue;
 }
 0x60b13d9 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x60b1405; continue; }
  pc = 0x60b13dd; continue;
 }
 0x60b13dd => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b1494; continue; }
  pc = 0x60b13f6; continue;
 }
 0x60b13f6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60b13fb; continue;
 }
 0x60b13fb => {
  cpu.push(0x60b1400); f_060c147a(cpu); if cpu.returned_to != 0x60b1400 { return; }
  pc = 0x60b1496; continue;
 }
 0x60b1405 => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x60b140b; continue;
 }
 0x60b140b => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b142f; continue; }
  pc = 0x60b1428; continue;
 }
 0x60b1428 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60b13fb; continue;
 }
 0x60b142f => {
  if cpu.zf { pc = 0x60b1494; continue; }
  pc = 0x60b1431; continue;
 }
 0x60b1431 => {
  { let v: u32 = cpu.edi ^ cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b1471; continue;
 }
 0x60b1440 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x1au32); }
  if !cpu.zf { pc = 0x60b1461; continue; }
  pc = 0x60b1449; continue;
 }
 0x60b1449 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x60b145f); f_060b1673(cpu); if cpu.returned_to != 0x60b145f { return; }
  pc = 0x60b1494; continue;
 }
 0x60b1461 => {
  { cpu.sub32(cpu.ebx, 0xdu32); }
  if cpu.zf { pc = 0x60b146f; continue; }
  pc = 0x60b1466; continue;
 }
 0x60b1466 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x60b146f; continue;
 }
 0x60b146f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x60b1471; continue;
 }
 0x60b1471 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x60b1440; continue; }
  pc = 0x60b1477; continue;
 }
 0x60b1477 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if !cpu.zf { pc = 0x60b1494; continue; }
  pc = 0x60b148c; continue;
 }
 0x60b148c => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b140b; continue; }
  pc = 0x60b1494; continue;
 }
 0x60b1494 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b1496; continue;
 }
 0x60b1496 => {
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
pub fn f_060b149e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b149e); }
 let mut pc: u32 = 0x60b149e;
 loop { match pc {
 0x60b149e => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b14af); f_060c1512(cpu); if cpu.returned_to != 0x60b14af { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b14ca; continue; }
  pc = 0x60b14b6; continue;
 }
 0x60b14b6 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x60b14bb; continue;
 }
 0x60b14bb => {
  cpu.push(0x60b14c0); f_060aec9b(cpu); if cpu.returned_to != 0x60b14c0 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b166c; continue;
 }
 0x60b14ca => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x60b14d5; continue; }
  pc = 0x60b14ce; continue;
 }
 0x60b14ce => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60b14bb; continue;
 }
 0x60b14d5 => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x60b150c; continue; }
  pc = 0x60b14d9; continue;
 }
 0x60b14d9 => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b150c; continue; }
  pc = 0x60b14fc; continue;
 }
 0x60b14fc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60b1507); f_060c147a(cpu); if cpu.returned_to != 0x60b1507 { return; }
  pc = 0x60b166c; continue;
 }
 0x60b150c => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x60b1547; continue; }
  pc = 0x60b1512; continue;
 }
 0x60b1512 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b14fc; continue; }
  pc = 0x60b152c; continue;
 }
 0x60b152c => {
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x60b1669; continue; }
  pc = 0x60b1535; continue;
 }
 0x60b1535 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x60b153f); f_060aec9b(cpu); if cpu.returned_to != 0x60b153f { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b1669; continue;
 }
 0x60b1547 => {
  cpu.push(0x60b154c); f_060c15b1(cpu); if cpu.returned_to != 0x60b154c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if !cpu.cf { pc = 0x60b155a; continue; }
  pc = 0x60b1555; continue;
 }
 0x60b1555 => {
  cpu.push(0x60b155a); f_060adfdb(cpu); if cpu.returned_to != 0x60b155a { return; }
  pc = 0x60b155a; continue;
 }
 0x60b155a => {
  { let v: u32 = 0x200u32; cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x230u32); }
  if !cpu.cf { pc = 0x60b156c; continue; }
  pc = 0x60b1567; continue;
 }
 0x60b1567 => {
  { let v: u32 = 0x80u32; cpu.edi = v; }
  pc = 0x60b156c; continue;
 }
 0x60b156c => {
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
  pc = 0x60b1588; continue;
 }
 0x60b1588 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x60b1633; continue; }
  pc = 0x60b1594; continue;
 }
 0x60b1594 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x60b15e8; continue; }
  pc = 0x60b15a0; continue;
 }
 0x60b15a0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x60b15e8; continue; }
  pc = 0x60b15ac; continue;
 }
 0x60b15ac => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b14fc; continue; }
  pc = 0x60b15c8; continue;
 }
 0x60b15c8 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x60b15e0; continue; }
  pc = 0x60b15cc; continue;
 }
 0x60b15cc => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x60b15d6); f_060aec9b(cpu); if cpu.returned_to != 0x60b15d6 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x60b166c; continue;
 }
 0x60b15e0 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b15e8; continue;
 }
 0x60b15e8 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60b1588; continue; }
  pc = 0x60b1606; continue;
 }
 0x60b1606 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b14fc; continue; }
  pc = 0x60b1622; continue;
 }
 0x60b1622 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x60b15cc; continue; }
  pc = 0x60b1626; continue;
 }
 0x60b1626 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60b1588; continue;
 }
 0x60b1633 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b1669; continue; }
  pc = 0x60b1637; continue;
 }
 0x60b1637 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b14fc; continue; }
  pc = 0x60b1654; continue;
 }
 0x60b1654 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x60b1669; continue; }
  pc = 0x60b1658; continue;
 }
 0x60b1658 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x60b1662); f_060aec9b(cpu); if cpu.returned_to != 0x60b1662 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  pc = 0x60b166c; continue;
 }
 0x60b1669 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x60b166c; continue;
 }
 0x60b166c => {
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
pub fn f_060b1673(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1673); }
 let mut pc: u32 = 0x60b1673;
 loop { match pc {
 0x60b1673 => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b16ad; continue; }
  pc = 0x60b169c; continue;
 }
 0x60b169c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60b16a6); f_060c147a(cpu); if cpu.returned_to != 0x60b16a6 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b16b0; continue;
 }
 0x60b16ad => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x60b16b0; continue;
 }
 0x60b16b0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b16b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b16b6); }
 let mut pc: u32 = 0x60b16b6;
 loop { match pc {
 0x60b16b6 => {
  pc = 0x60c15ba; continue;
 }
 0x60c15ba => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60c15d7; continue; }
  pc = 0x60c15cb; continue;
 }
 0x60c15cb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60c15d5); f_060c147a(cpu); if cpu.returned_to != 0x60c15d5 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60c15d7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b16bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b16bb); }
 let mut pc: u32 = 0x60b16bb;
 loop { match pc {
 0x60b16bb => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b16ea; continue; }
  pc = 0x60b16dc; continue;
 }
 0x60b16dc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60b16e1; continue;
 }
 0x60b16e1 => {
  cpu.push(0x60b16e6); f_060aec9b(cpu); if cpu.returned_to != 0x60b16e6 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b1737; continue;
 }
 0x60b16ea => {
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
  if !cpu.zf { pc = 0x60b172e; continue; }
  pc = 0x60b1718; continue;
 }
 0x60b1718 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60b171f); f_060bd034(cpu); if cpu.returned_to != 0x60b171f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b172c; continue; }
  pc = 0x60b1725; continue;
 }
 0x60b1725 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.poll(); pc = 0x60b16e1; continue;
 }
 0x60b172c => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x60b172e; continue;
 }
 0x60b172e => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60b1737); f_0609e8d4(cpu); if cpu.returned_to != 0x60b1737 { return; }
  pc = 0x60b1737; continue;
 }
 0x60b1737 => {
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
pub fn f_060b1743(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1743); }
 let mut pc: u32 = 0x60b1743;
 loop { match pc {
 0x60b1743 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x3bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b1760; continue; }
  pc = 0x60b1754; continue;
 }
 0x60b1754 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x60b175e); f_060c147a(cpu); if cpu.returned_to != 0x60b175e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b1760 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1764(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1764); }
 let mut pc: u32 = 0x60b1764;
 loop { match pc {
 0x60b1764 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x60b176d); f_060a3ac4(cpu); if cpu.returned_to != 0x60b176d { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b178e; continue; }
  pc = 0x60b1774; continue;
 }
 0x60b1774 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x60b177b); f_060b1791(cpu); if cpu.returned_to != 0x60b177b { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b178e; continue; }
  pc = 0x60b1780; continue;
 }
 0x60b1780 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x60b178c; continue; }
  pc = 0x60b1784; continue;
 }
 0x60b1784 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b178c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60b178e; continue;
 }
 0x60b178e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1791(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1791); }
 let mut pc: u32 = 0x60b1791;
 loop { match pc {
 0x60b1791 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x60b179f); f_060b1673(cpu); if cpu.returned_to != 0x60b179f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b17a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b17a2); }
 let mut pc: u32 = 0x60b17a2;
 loop { match pc {
 0x60b17a2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.push(0x60b17ae); f_060c15db(cpu); if cpu.returned_to != 0x60b17ae { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b17b3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b17b3); }
 let mut pc: u32 = 0x60b17b3;
 loop { match pc {
 0x60b17b3 => {
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
  if !cpu.zf { pc = 0x60b17d6; continue; }
  pc = 0x60b17cf; continue;
 }
 0x60b17cf => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60b17d6); f_060c167f(cpu); if cpu.returned_to != 0x60b17d6 { return; }
  pc = 0x60b17d6; continue;
 }
 0x60b17d6 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x60b17f1; continue; }
  pc = 0x60b17e0; continue;
 }
 0x60b17e0 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  pc = 0x60b17f1; continue;
 }
 0x60b17f1 => {
  { let v: u32 = 0x60b17a2u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60b17fd); f_060aee28(cpu); if cpu.returned_to != 0x60b17fd { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60b1818; continue; }
  pc = 0x60b1803; continue;
 }
 0x60b1803 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x60b1818); f_060c16f6(cpu); if cpu.returned_to != 0x60b1818 { return; }
  pc = 0x60b1818; continue;
 }
 0x60b1818 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x60b1823; continue; }
  pc = 0x60b181e; continue;
 }
 0x60b181e => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x60b1823; continue;
 }
 0x60b1823 => {
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
pub fn f_060b1834(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b1834); } r_060b1834(cpu, 0x60b1834); }
pub fn f_060b183c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b183c); } r_060b1834(cpu, 0x60b183c); }
fn r_060b1834(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b1834 => {
  cpu.unimplemented(0x60b1834, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x32u32.wrapping_add(cpu.ebp)), 0xf8830078u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  if cpu.zf { pc = 0x60b1846; continue; }
  pc = 0x60b1841; continue;
 }
 0x60b183c => {
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x60b1846; continue; }
  pc = 0x60b1841; continue;
 }
 0x60b1841 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x60b184f; continue; }
  pc = 0x60b1846; continue;
 }
 0x60b1846 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x60b184d; continue; }
  pc = 0x60b184b; continue;
 }
 0x60b184b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b184d; continue;
 }
 0x60b184d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x60b184f; continue;
 }
 0x60b184f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1850(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1850); }
 let mut pc: u32 = 0x60b1850;
 loop { match pc {
 0x60b1850 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b1874; continue; }
  pc = 0x60b1865; continue;
 }
 0x60b1865 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x60b1874; continue; }
  pc = 0x60b186b; continue;
 }
 0x60b186b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60b1874; continue;
 }
 0x60b1874 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b1932; continue; }
  pc = 0x60b1880; continue;
 }
 0x60b1880 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x60b1932; continue; }
  pc = 0x60b1889; continue;
 }
 0x60b1889 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x60b1894); f_060c185c(cpu); if cpu.returned_to != 0x60b1894 { return; }
  cpu.push(0x60b1899); f_060b183c(cpu); if cpu.returned_to != 0x60b1899 { return; }
  cpu.push(0x60b189e); f_060c18b4(cpu); if cpu.returned_to != 0x60b189e { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60b18a3; continue;
 }
 0x60b18a3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b18bb; continue; }
  pc = 0x60b18ab; continue;
 }
 0x60b18ab => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b18a3; continue; }
  pc = 0x60b18bb; continue;
 }
 0x60b18bb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x617c90cu32), 0x0u32); }
  if cpu.zf { pc = 0x60b18d9; continue; }
  pc = 0x60b18c5; continue;
 }
 0x60b18c5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x60b18db; continue;
 }
 0x60b18d9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b18db; continue;
 }
 0x60b18db => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b18f8; continue; }
  pc = 0x60b18e7; continue;
 }
 0x60b18e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x60b18f8; continue;
 }
 0x60b18f8 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b1889; continue; }
  pc = 0x60b1900; continue;
 }
 0x60b1900 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b190f; continue; }
  pc = 0x60b1907; continue;
 }
 0x60b1907 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b190f; continue;
 }
 0x60b190f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x60b1919); f_060c1b28(cpu); if cpu.returned_to != 0x60b1919 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x60b192b; continue; }
  pc = 0x60b1928; continue;
 }
 0x60b1928 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60b1932; continue;
 }
 0x60b192b => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b1932; continue;
 }
 0x60b1932 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b1941; continue; }
  pc = 0x60b1939; continue;
 }
 0x60b1939 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b1941; continue;
 }
 0x60b1941 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b19e9; continue; }
  pc = 0x60b194c; continue;
 }
 0x60b194c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x60b1957); f_060c185c(cpu); if cpu.returned_to != 0x60b1957 { return; }
  cpu.push(0x60b195c); f_060b183c(cpu); if cpu.returned_to != 0x60b195c { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x60b196f; continue; }
  pc = 0x60b1964; continue;
 }
 0x60b1964 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x60b196f; continue; }
  pc = 0x60b196d; continue;
 }
 0x60b196d => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x60b196f; continue;
 }
 0x60b196f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x60b197a); f_060c185c(cpu); if cpu.returned_to != 0x60b197a { return; }
  cpu.push(0x60b197f); f_060b183c(cpu); if cpu.returned_to != 0x60b197f { return; }
  cpu.push(0x60b1984); f_060c18b4(cpu); if cpu.returned_to != 0x60b1984 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x60b1989; continue;
 }
 0x60b1989 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b19a1; continue; }
  pc = 0x60b1991; continue;
 }
 0x60b1991 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b1989; continue; }
  pc = 0x60b19a1; continue;
 }
 0x60b19a1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b19bd; continue; }
  pc = 0x60b19ab; continue;
 }
 0x60b19ab => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x60b19bd; continue;
 }
 0x60b19bd => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x617c90cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b19dd; continue; }
  pc = 0x60b19ca; continue;
 }
 0x60b19ca => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617c911u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x60b19df; continue;
 }
 0x60b19dd => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b19df; continue;
 }
 0x60b19df => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b196f; continue; }
  pc = 0x60b19e7; continue;
 }
 0x60b19e7 => {
  pc = 0x60b19f8; continue;
 }
 0x60b19e9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x60b19f8; continue; }
  pc = 0x60b19f6; continue;
 }
 0x60b19f6 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x60b19f8; continue;
 }
 0x60b19f8 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b1a2e; continue; }
  pc = 0x60b1a00; continue;
 }
 0x60b1a00 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x60b1a2e; continue; }
  pc = 0x60b1a05; continue;
 }
 0x60b1a05 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x60b1a17; continue; }
  pc = 0x60b1a12; continue;
 }
 0x60b1a12 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60b1a17; continue;
 }
 0x60b1a17 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x60b1a2e; continue; }
  pc = 0x60b1a21; continue;
 }
 0x60b1a21 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b1a17; continue;
 }
 0x60b1a2e => {
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
pub fn f_060b1a3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1a3b); }
 let mut pc: u32 = 0x60b1a3b;
 loop { match pc {
 0x60b1a3b => {
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
  if cpu.zf { pc = 0x60b1aed; continue; }
  pc = 0x60b1acb; continue;
 }
 0x60b1acb => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x17u32); }
  if cpu.zf { pc = 0x60b1aed; continue; }
  pc = 0x60b1ad1; continue;
 }
 0x60b1ad1 => {
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
  pc = 0x60b1aed; continue;
 }
 0x60b1aed => {
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
pub fn f_060b1afe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1afe); }
 let mut pc: u32 = 0x60b1afe;
 loop { match pc {
 0x60b1afe => {
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
  { let v: u32 = 0x616b610u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0xf4fb7ea1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b1b32; continue; }
  pc = 0x60b1b28; continue;
 }
 0x60b1b28 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b1c4b; continue;
 }
 0x60b1b32 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x60b1b43; continue;
 }
 0x60b1b43 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x60b1b4d; continue; }
  pc = 0x60b1b47; continue;
 }
 0x60b1b47 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  cpu.poll(); pc = 0x60b1b43; continue;
 }
 0x60b1b4d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b1b28; continue; }
  pc = 0x60b1b51; continue;
 }
 0x60b1b51 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x60b1b5c); f_060b2162(cpu); if cpu.returned_to != 0x60b1b5c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b1b65; continue; }
  pc = 0x60b1b60; continue;
 }
 0x60b1b60 => {
  { let v: u32 = 0x616b62au32; cpu.ebx = v; }
  pc = 0x60b1b65; continue;
 }
 0x60b1b65 => {
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
  if cpu.zf { pc = 0x60b1b92; continue; }
  pc = 0x60b1b91; continue;
 }
 0x60b1b91 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b1b92; continue;
 }
 0x60b1b92 => {
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
  pc = 0x60b1bb3; continue;
 }
 0x60b1bb3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60b1bc0; continue; }
  pc = 0x60b1bb7; continue;
 }
 0x60b1bb7 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x60b1bb3; continue;
 }
 0x60b1bc0 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60b1bcd); f_060b2022(cpu); if cpu.returned_to != 0x60b1bcd { return; }
  cpu.push(0x60b1bd2); f_060b1cbe(cpu); if cpu.returned_to != 0x60b1bd2 { return; }
  { let v: u32 = cpu.r32(0x617b8b8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x60b1be8; continue; }
  pc = 0x60b1be1; continue;
 }
 0x60b1be1 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60b1be8); f_060b22b6(cpu); if cpu.returned_to != 0x60b1be8 { return; }
  pc = 0x60b1be8; continue;
 }
 0x60b1be8 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b1bf4; continue; }
  pc = 0x60b1bee; continue;
 }
 0x60b1bee => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x617b8bcu32)); cpu.esi = v; }
  pc = 0x60b1bf4; continue;
 }
 0x60b1bf4 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60b1c01; continue; }
  pc = 0x60b1bf8; continue;
 }
 0x60b1bf8 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x60b1bf4; continue;
 }
 0x60b1c01 => {
  { cpu.sub32(cpu.edi, 0x63deu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b1b28; continue; }
  pc = 0x60b1c0d; continue;
 }
 0x60b1c0d => {
  if !cpu.zf { pc = 0x60b1c2b; continue; }
  pc = 0x60b1c0f; continue;
 }
 0x60b1c0f => {
  { let v: u32 = cpu.r32(0x617b8b8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x15180u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b1b28; continue; }
  pc = 0x60b1c22; continue;
 }
 0x60b1c22 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60b1c49; continue; }
  pc = 0x60b1c26; continue;
 }
 0x60b1c26 => {
  cpu.poll(); pc = 0x60b1b28; continue;
 }
 0x60b1c2b => {
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
  pc = 0x60b1c49; continue;
 }
 0x60b1c49 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b1c4b; continue;
 }
 0x60b1c4b => {
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
pub fn f_060b1c52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1c52); }
 let mut pc: u32 = 0x60b1c52;
 loop { match pc {
 0x60b1c52 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617b90au32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x617b90au32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x617b90au32, v); }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617b90au32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617b90au32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x617b90au32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x617b90au32, v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x617b90au32), cpu.edx); cpu.w32(0x617b90au32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1cbe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1cbe); }
 let mut pc: u32 = 0x60b1cbe;
 loop { match pc {
 0x60b1ca6 => {
  { let v: u8 = cpu.r8(0x617b90au32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x60b1cb6; continue; }
  pc = 0x60b1cb1; continue;
 }
 0x60b1cb1 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.zf { pc = 0x60b1cbd; continue; }
  pc = 0x60b1cb6; continue;
 }
 0x60b1cb6 => {
  { let v: u8 = cpu.or8(cpu.r8(0x617b90au32), 0x2u8); cpu.w8(0x617b90au32, v); }
  pc = 0x60b1cbd; continue;
 }
 0x60b1cbd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b1cbe => {
  { let v: u32 = 0x616abf0u32; cpu.eax = v; }
  cpu.push(0x60b1cc8); f_060bd199(cpu); if cpu.returned_to != 0x60b1cc8 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x60b1ca6; continue; }
  pc = 0x60b1ccc; continue;
 }
 0x60b1ccc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x617b8b8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617b8c0u32, v); }
  { let v: u32 = 0x617b8c4u32; cpu.edx = v; }
  cpu.push(0x60b1cec); f_060b1d27(cpu); if cpu.returned_to != 0x60b1cec { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b1f54; continue; }
  pc = 0x60b1cf8; continue;
 }
 0x60b1cf8 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x617b8e3u32, v); }
  pc = 0x60b2018; continue;
 }
 0x60b1f54 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x617b8e3u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617b8b8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b8c0u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60b1f7a); f_060b1d27(cpu); if cpu.returned_to != 0x60b1f7a { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x617b8b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617b8bcu32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, 0x2cu32); }
  if !cpu.zf { pc = 0x60b1fa3; continue; }
  pc = 0x60b1f96; continue;
 }
 0x60b1f96 => {
  { let v: u32 = 0x617b870u32; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x60b1fa1); f_060b1e50(cpu); if cpu.returned_to != 0x60b1fa1 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x60b1fa3; continue;
 }
 0x60b1fa3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if !cpu.zf { pc = 0x60b2018; continue; }
  pc = 0x60b1fac; continue;
 }
 0x60b1fac => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x617b894u32; cpu.edx = v; }
  cpu.push(0x60b1fb9); f_060b1e50(cpu); if cpu.returned_to != 0x60b1fb9 { return; }
  { let v: u32 = cpu.r32(0x617b8bcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xe10u32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x617b89cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x617b8bcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x617b898u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x617b8bcu32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x617b894u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617b89cu32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x617b898u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x617b894u32, v); }
  pc = 0x60b2018; continue;
 }
 0x60b2018 => {
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
pub fn f_060b1d03(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1d03); }
 let mut pc: u32 = 0x60b1d03;
 loop { match pc {
 0x60b1d03 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60b1d09; continue;
 }
 0x60b1d09 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b1d22; continue; }
  pc = 0x60b1d12; continue;
 }
 0x60b1d12 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b1d22; continue; }
  pc = 0x60b1d17; continue;
 }
 0x60b1d17 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60b1d09; continue;
 }
 0x60b1d22 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b1d27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1d27); }
 let mut pc: u32 = 0x60b1d27;
 loop { match pc {
 0x60b1d27 => {
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
  if !cpu.zf { pc = 0x60b1d3f; continue; }
  pc = 0x60b1d3e; continue;
 }
 0x60b1d3e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x60b1d3f; continue;
 }
 0x60b1d3f => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x60b1d41; continue;
 }
 0x60b1d41 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x60b1d68; continue; }
  pc = 0x60b1d48; continue;
 }
 0x60b1d48 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if cpu.zf { pc = 0x60b1d68; continue; }
  pc = 0x60b1d51; continue;
 }
 0x60b1d51 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if cpu.zf { pc = 0x60b1d68; continue; }
  pc = 0x60b1d56; continue;
 }
 0x60b1d56 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x60b1d68; continue; }
  pc = 0x60b1d5b; continue;
 }
 0x60b1d5b => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b1d65; continue; }
  pc = 0x60b1d60; continue;
 }
 0x60b1d60 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b1d68; continue; }
  pc = 0x60b1d65; continue;
 }
 0x60b1d65 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = 0x60b1d41; continue;
 }
 0x60b1d68 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b1d7d; continue; }
  pc = 0x60b1d75; continue;
 }
 0x60b1d75 => {
  { let v: u32 = 0x1eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b1d7d; continue;
 }
 0x60b1d7d => {
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
  if !cpu.zf { pc = 0x60b1dba; continue; }
  pc = 0x60b1db3; continue;
 }
 0x60b1db3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x60b1dbf; continue;
 }
 0x60b1dba => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x60b1dc1; continue; }
  pc = 0x60b1dbf; continue;
 }
 0x60b1dbf => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x60b1dc1; continue;
 }
 0x60b1dc1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b1e46; continue; }
  pc = 0x60b1dd2; continue;
 }
 0x60b1dd2 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b1e46; continue; }
  pc = 0x60b1dd7; continue;
 }
 0x60b1dd7 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x60b1def); f_060b1d03(cpu); if cpu.returned_to != 0x60b1def { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x60b1e19; continue; }
  pc = 0x60b1dfa; continue;
 }
 0x60b1dfa => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x60b1e04); f_060b1d03(cpu); if cpu.returned_to != 0x60b1e04 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x60b1e19; continue; }
  pc = 0x60b1e0f; continue;
 }
 0x60b1e0f => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x60b1e17); f_060b1d03(cpu); if cpu.returned_to != 0x60b1e17 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x60b1e19; continue;
 }
 0x60b1e19 => {
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
  if cpu.zf { pc = 0x60b1e46; continue; }
  pc = 0x60b1e42; continue;
 }
 0x60b1e42 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b1e46; continue;
 }
 0x60b1e46 => {
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
pub fn f_060b1e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b1e50); }
 let mut pc: u32 = 0x60b1e50;
 loop { match pc {
 0x60b1e50 => {
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
  if !cpu.zf { pc = 0x60b1e6f; continue; }
  pc = 0x60b1e68; continue;
 }
 0x60b1e68 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x60b1e6f; continue;
 }
 0x60b1e6f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4du32); }
  if !cpu.zf { pc = 0x60b1e7b; continue; }
  pc = 0x60b1e78; continue;
 }
 0x60b1e78 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b1e7b; continue;
 }
 0x60b1e7b => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x60b1e89); f_060b1d03(cpu); if cpu.returned_to != 0x60b1e89 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b1e9a; continue; }
  pc = 0x60b1e91; continue;
 }
 0x60b1e91 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b1ee7; continue;
 }
 0x60b1e9a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x60b1ee0; continue; }
  pc = 0x60b1eab; continue;
 }
 0x60b1eab => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x60b1eb7); f_060b1d03(cpu); if cpu.returned_to != 0x60b1eb7 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x60b1ee0; continue; }
  pc = 0x60b1ecb; continue;
 }
 0x60b1ecb => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x60b1ed7); f_060b1d03(cpu); if cpu.returned_to != 0x60b1ed7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b1ee0; continue;
 }
 0x60b1ee0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b1ee7; continue;
 }
 0x60b1ee7 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x60b1f38; continue; }
  pc = 0x60b1f02; continue;
 }
 0x60b1f02 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x60b1f0e); f_060b1d03(cpu); if cpu.returned_to != 0x60b1f0e { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x60b1f38; continue; }
  pc = 0x60b1f19; continue;
 }
 0x60b1f19 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x60b1f23); f_060b1d03(cpu); if cpu.returned_to != 0x60b1f23 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x60b1f38; continue; }
  pc = 0x60b1f2e; continue;
 }
 0x60b1f2e => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x60b1f36); f_060b1d03(cpu); if cpu.returned_to != 0x60b1f36 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x60b1f38; continue;
 }
 0x60b1f38 => {
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
pub fn f_060b2022(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2022); }
 let mut pc: u32 = 0x60b2022;
 loop { match pc {
 0x60b2022 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xa8c0u32); }
  if !cpu.cf { pc = 0x60b204f; continue; }
  pc = 0x60b2033; continue;
 }
 0x60b2033 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b204f; continue; }
  pc = 0x60b2037; continue;
 }
 0x60b2037 => {
  { let v: u32 = 0x15180u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b205e; continue;
 }
 0x60b204f => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x60b205e; continue;
 }
 0x60b205e => {
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
  if (cpu.cf || cpu.zf) { pc = 0x60b20c5; continue; }
  pc = 0x60b20bd; continue;
 }
 0x60b20bd => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x60b20c5; continue;
 }
 0x60b20c5 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x60b20d0); f_060b2162(cpu); if cpu.returned_to != 0x60b20d0 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x16du32); cpu.eax = v; }
  pc = 0x60b20d5; continue;
 }
 0x60b20d5 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x60b20de; continue; }
  pc = 0x60b20d9; continue;
 }
 0x60b20d9 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x60b20d5; continue;
 }
 0x60b20de => {
  { let v: u32 = 0x616b610u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x60b20f4); f_060b2162(cpu); if cpu.returned_to != 0x60b20f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b20fd; continue; }
  pc = 0x60b20f8; continue;
 }
 0x60b20f8 => {
  { let v: u32 = 0x616b62au32; cpu.esi = v; }
  pc = 0x60b20fd; continue;
 }
 0x60b20fd => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x60b2115; continue; }
  pc = 0x60b2114; continue;
 }
 0x60b2114 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60b2115; continue;
 }
 0x60b2115 => {
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
pub fn f_060b2137(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2137); }
 let mut pc: u32 = 0x60b2137;
 loop { match pc {
 0x60b2137 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  cpu.push(0x60b2152); f_060b2022(cpu); if cpu.returned_to != 0x60b2152 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b2162(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2162); }
 let mut pc: u32 = 0x60b2162;
 loop { match pc {
 0x60b2162 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x60b2197; continue; }
  pc = 0x60b216c; continue;
 }
 0x60b216c => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b2185; continue; }
  pc = 0x60b217c; continue;
 }
 0x60b217c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2185 => {
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x60b217c; continue; }
  pc = 0x60b2197; continue;
 }
 0x60b2197 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b219d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b219d); }
 let mut pc: u32 = 0x60b219d;
 loop { match pc {
 0x60b219d => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60b226a; continue; }
  pc = 0x60b21b2; continue;
 }
 0x60b21b2 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x60b21bd); f_060b2162(cpu); if cpu.returned_to != 0x60b21bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b21d4; continue; }
  pc = 0x60b21c1; continue;
 }
 0x60b21c1 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x616b62au32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x616b628u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x60b21e5; continue;
 }
 0x60b21d4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x616b610u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x616b60eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x60b21e5; continue;
 }
 0x60b21e5 => {
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
  cpu.push(0x60b2219); f_060b1afe(cpu); if cpu.returned_to != 0x60b2219 { return; }
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
  if !cpu.zf { pc = 0x60b2258; continue; }
  pc = 0x60b2239; continue;
 }
 0x60b2239 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b2254; continue; }
  pc = 0x60b224c; continue;
 }
 0x60b224c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  pc = 0x60b2259; continue;
 }
 0x60b2254 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x60b2259; continue;
 }
 0x60b2258 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x60b2259; continue;
 }
 0x60b2259 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x60b2279; continue;
 }
 0x60b226a => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x60b2276; continue; }
  pc = 0x60b226f; continue;
 }
 0x60b226f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x60b2279; continue;
 }
 0x60b2276 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x60b2279; continue;
 }
 0x60b2279 => {
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
pub fn f_060b2282(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2282); }
 let mut pc: u32 = 0x60b2282;
 loop { match pc {
 0x60b2282 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b2296; continue; }
  pc = 0x60b228e; continue;
 }
 0x60b228e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2296 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.ecx))); }
  if (cpu.sf != cpu.of) { pc = 0x60b22b1; continue; }
  pc = 0x60b229b; continue;
 }
 0x60b229b => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x60b22a2); f_060b219d(cpu); if cpu.returned_to != 0x60b22a2 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60b22ad); f_060b219d(cpu); if cpu.returned_to != 0x60b22ad { return; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b228e; continue; }
  pc = 0x60b22b1; continue;
 }
 0x60b22b1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b22b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b22b6); }
 let mut pc: u32 = 0x60b22b6;
 loop { match pc {
 0x60b22b6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x617b906u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x60b256a; continue; }
  pc = 0x60b22d2; continue;
 }
 0x60b22d2 => {
  { let v: u32 = 0x617b894u32; cpu.edx = v; }
  { let v: u32 = 0x617b870u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  cpu.push(0x60b22e4); f_060b2282(cpu); if cpu.returned_to != 0x60b22e4 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b22f8; continue; }
  pc = 0x60b22ec; continue;
 }
 0x60b22ec => {
  { let v: u32 = 0x617b894u32; cpu.esi = v; }
  { let v: u32 = 0x617b870u32; cpu.ebx = v; }
  pc = 0x60b2302; continue;
 }
 0x60b22f8 => {
  { let v: u32 = 0x617b870u32; cpu.esi = v; }
  { let v: u32 = 0x617b894u32; cpu.ebx = v; }
  pc = 0x60b2302; continue;
 }
 0x60b2302 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  cpu.push(0x60b2312); f_060b2162(cpu); if cpu.returned_to != 0x60b2312 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b232a; continue; }
  pc = 0x60b231c; continue;
 }
 0x60b231c => {
  { let v: u32 = cpu.r32(0x616b62au32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x616b628u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x60b2336; continue;
 }
 0x60b232a => {
  { let v: u32 = cpu.r32(0x616b610u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x616b60eu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x60b2336; continue;
 }
 0x60b2336 => {
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b2406; continue; }
  pc = 0x60b2353; continue;
 }
 0x60b2353 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b2364; continue; }
  pc = 0x60b235a; continue;
 }
 0x60b235a => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x60b243f; continue;
 }
 0x60b2364 => {
  if !cpu.zf { pc = 0x60b243f; continue; }
  pc = 0x60b236a; continue;
 }
 0x60b236a => {
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
  if !cpu.zf { pc = 0x60b23df; continue; }
  pc = 0x60b23ba; continue;
 }
 0x60b23ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60b243f; continue; }
  pc = 0x60b23cc; continue;
 }
 0x60b23cc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x60b243f; continue; }
  pc = 0x60b23dd; continue;
 }
 0x60b23dd => {
  pc = 0x60b243b; continue;
 }
 0x60b23df => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b243f; continue; }
  pc = 0x60b23f7; continue;
 }
 0x60b23f7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b243f; continue; }
  pc = 0x60b2404; continue;
 }
 0x60b2404 => {
  pc = 0x60b243b; continue;
 }
 0x60b2406 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60b242d; continue; }
  pc = 0x60b240e; continue;
 }
 0x60b240e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x60b241b); f_060b2162(cpu); if cpu.returned_to != 0x60b241b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b242c; continue; }
  pc = 0x60b241f; continue;
 }
 0x60b241f => {
  { let v: u32 = cpu.r32(0x616b612u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b242c; continue; }
  pc = 0x60b242b; continue;
 }
 0x60b242b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60b242c; continue;
 }
 0x60b242c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x60b242d; continue;
 }
 0x60b242d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b243f; continue; }
  pc = 0x60b2434; continue;
 }
 0x60b2434 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  if !cpu.zf { pc = 0x60b243f; continue; }
  pc = 0x60b243b; continue;
 }
 0x60b243b => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b243f; continue;
 }
 0x60b243f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60b245c; continue; }
  pc = 0x60b2446; continue;
 }
 0x60b2446 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60b244f); f_060b2579(cpu); if cpu.returned_to != 0x60b244f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  pc = 0x60b245c; continue;
 }
 0x60b245c => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60b2473; continue; }
  pc = 0x60b2460; continue;
 }
 0x60b2460 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b256a; continue; }
  pc = 0x60b246c; continue;
 }
 0x60b246c => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b256a; continue;
 }
 0x60b2473 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b2516; continue; }
  pc = 0x60b2480; continue;
 }
 0x60b2480 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b248e; continue; }
  pc = 0x60b2487; continue;
 }
 0x60b2487 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b254f; continue;
 }
 0x60b248e => {
  if !cpu.zf { pc = 0x60b254f; continue; }
  pc = 0x60b2494; continue;
 }
 0x60b2494 => {
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
  if !cpu.zf { pc = 0x60b24f7; continue; }
  pc = 0x60b24d8; continue;
 }
 0x60b24d8 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x60b24f1; continue; }
  pc = 0x60b24e7; continue;
 }
 0x60b24e7 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x60b254f; continue;
 }
 0x60b24f1 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b254f; continue; }
  pc = 0x60b24f5; continue;
 }
 0x60b24f5 => {
  pc = 0x60b254a; continue;
 }
 0x60b24f7 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b24e7; continue; }
  pc = 0x60b2510; continue;
 }
 0x60b2510 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b254f; continue; }
  pc = 0x60b2514; continue;
 }
 0x60b2514 => {
  pc = 0x60b254a; continue;
 }
 0x60b2516 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60b253d; continue; }
  pc = 0x60b251e; continue;
 }
 0x60b251e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x60b252b); f_060b2162(cpu); if cpu.returned_to != 0x60b252b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b253c; continue; }
  pc = 0x60b252f; continue;
 }
 0x60b252f => {
  { let v: u32 = cpu.r32(0x616b612u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b253c; continue; }
  pc = 0x60b253b; continue;
 }
 0x60b253b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60b253c; continue;
 }
 0x60b253c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x60b253d; continue;
 }
 0x60b253d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b254f; continue; }
  pc = 0x60b2544; continue;
 }
 0x60b2544 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x60b254f; continue; }
  pc = 0x60b254a; continue;
 }
 0x60b254a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x60b254f; continue;
 }
 0x60b254f => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b255e; continue; }
  pc = 0x60b2553; continue;
 }
 0x60b2553 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60b255c); f_060b2579(cpu); if cpu.returned_to != 0x60b255c { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x60b255e; continue;
 }
 0x60b255e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b256a; continue; }
  pc = 0x60b2566; continue;
 }
 0x60b2566 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x60b256a; continue;
 }
 0x60b256a => {
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
pub fn f_060b2579(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2579); }
 let mut pc: u32 = 0x60b2579;
 loop { match pc {
 0x60b2579 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x60b259d; continue; }
  pc = 0x60b2589; continue;
 }
 0x60b2589 => {
  if !cpu.zf { pc = 0x60b25a2; continue; }
  pc = 0x60b258b; continue;
 }
 0x60b258b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x60b259d; continue; }
  pc = 0x60b2595; continue;
 }
 0x60b2595 => {
  if !cpu.zf { pc = 0x60b25a2; continue; }
  pc = 0x60b2597; continue;
 }
 0x60b2597 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x60b25a2; continue; }
  pc = 0x60b259d; continue;
 }
 0x60b259d => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x60b25a2; continue;
 }
 0x60b25a2 => {
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
pub fn f_060b25ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b25ac); }
 let mut pc: u32 = 0x60b25ac;
 loop { match pc {
 0x60b25ac => {
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
pub fn f_060b25d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b25d4); }
 let mut pc: u32 = 0x60b25d4;
 loop { match pc {
 0x60b25d4 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b2617; continue; }
  pc = 0x60b25e8; continue;
 }
 0x60b25e8 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b2610; continue; }
  pc = 0x60b25ec; continue;
 }
 0x60b25ec => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x60b2600; continue; }
  pc = 0x60b25f6; continue;
 }
 0x60b25f6 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b264f; continue;
 }
 0x60b2600 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b2609; continue; }
  pc = 0x60b2602; continue;
 }
 0x60b2602 => {
  { let v: u32 = 0x8047u32; cpu.eax = v; }
  pc = 0x60b2641; continue;
 }
 0x60b2609 => {
  { let v: u32 = 0x4047u32; cpu.eax = v; }
  pc = 0x60b2641; continue;
 }
 0x60b2610 => {
  { let v: u32 = 0x2047u32; cpu.eax = v; }
  pc = 0x60b2641; continue;
 }
 0x60b2617 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.cf { pc = 0x60b262b; continue; }
  pc = 0x60b2621; continue;
 }
 0x60b2621 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b264f; continue;
 }
 0x60b262b => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x60b263c; continue; }
  pc = 0x60b2635; continue;
 }
 0x60b2635 => {
  { let v: u32 = 0x8107u32; cpu.eax = v; }
  pc = 0x60b2641; continue;
 }
 0x60b263c => {
  { let v: u32 = 0x1107u32; cpu.eax = v; }
  pc = 0x60b2641; continue;
 }
 0x60b2641 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x60b264c); f_060c1c0d(cpu); if cpu.returned_to != 0x60b264c { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x60b264f; continue;
 }
 0x60b264f => {
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
