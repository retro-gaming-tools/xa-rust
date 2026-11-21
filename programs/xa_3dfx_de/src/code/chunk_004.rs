#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_060b2668(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2668); }
 let mut pc: u32 = 0x60b2668;
 loop { match pc {
 0x60b2668 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x617b934u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x60b2697; continue; }
  pc = 0x60b267f; continue;
 }
 0x60b267f => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x60b2691; continue; }
  pc = 0x60b2683; continue;
 }
 0x60b2683 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60b2691); f_060c1ce8(cpu); if cpu.returned_to != 0x60b2691 { return; }
  pc = 0x60b2691; continue;
 }
 0x60b2691 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2697 => {
  { let a = cpu.st(0); let b = cpu.rf64(0x617b93cu32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b26ac; continue; }
  pc = 0x60b26a8; continue;
 }
 0x60b26a8 => {
  { cpu.fpush(0.0); }
  pc = 0x60b26c0; continue;
 }
 0x60b26ac => {
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
  pc = 0x60b26c0; continue;
 }
 0x60b26c0 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b26d4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b26d4); }
 let mut pc: u32 = 0x60b26d4;
 loop { match pc {
 0x60b26d4 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x60b26dc; continue;
 }
 0x60b26dc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x60b276e; continue; }
  pc = 0x60b26e8; continue;
 }
 0x60b26e8 => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x60b26f7; continue; }
  pc = 0x60b26f4; continue;
 }
 0x60b26f4 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b26f7 => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x617ce14u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x60b2706; continue; }
  pc = 0x60b2703; continue;
 }
 0x60b2703 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2706 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b2778; continue; }
  pc = 0x60b2711; continue;
 }
 0x60b2711 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x60b2778; continue; }
  pc = 0x60b2718; continue;
 }
 0x60b2718 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60b2757; continue; }
  pc = 0x60b2740; continue;
 }
 0x60b2740 => {
  { let v = cpu.st(0) * cpu.rf32(0x617ce24u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x617ce24u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2757 => {
  { let v = cpu.st(0) * cpu.rf32(0x617ce28u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x617ce28u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b276e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x60b277b; continue; }
  pc = 0x60b2778; continue;
 }
 0x60b2778 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b277b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60b2778; continue; }
  pc = 0x60b2786; continue;
 }
 0x60b2786 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b27bc; continue; }
  pc = 0x60b27ab; continue;
 }
 0x60b27ab => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x60b27e4; continue; }
  pc = 0x60b27b2; continue;
 }
 0x60b27b2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x60b27e4; continue; }
  pc = 0x60b27ba; continue;
 }
 0x60b27ba => {
  pc = 0x60b27c4; continue;
 }
 0x60b27bc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x60b27e4; continue; }
  pc = 0x60b27c4; continue;
 }
 0x60b27c4 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x617ce2cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  cpu.poll(); pc = 0x60b26dc; continue;
 }
 0x60b27e4 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b27eb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b27eb); } r_060b27eb(cpu, 0x60b27eb); }
pub fn f_060b27f5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b27f5); } r_060b27eb(cpu, 0x60b27f5); }
pub fn f_060b27fb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b27fb); } r_060b27eb(cpu, 0x60b27fb); }
pub fn f_060b2800(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2800); } r_060b27eb(cpu, 0x60b2800); }
pub fn f_060b2806(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2806); } r_060b27eb(cpu, 0x60b2806); }
pub fn f_060b280b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b280b); } r_060b27eb(cpu, 0x60b280b); }
pub fn f_060b2811(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2811); } r_060b27eb(cpu, 0x60b2811); }
pub fn f_060b2817(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2817); } r_060b27eb(cpu, 0x60b2817); }
pub fn f_060b281d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b281d); } r_060b27eb(cpu, 0x60b281d); }
pub fn f_060b2823(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2823); } r_060b27eb(cpu, 0x60b2823); }
pub fn f_060b283f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b283f); } r_060b27eb(cpu, 0x60b283f); }
pub fn f_060b2844(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2844); } r_060b27eb(cpu, 0x60b2844); }
pub fn f_060b285a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b285a); } r_060b27eb(cpu, 0x60b285a); }
pub fn f_060b285f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b285f); } r_060b27eb(cpu, 0x60b285f); }
pub fn f_060b287b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b287b); } r_060b27eb(cpu, 0x60b287b); }
pub fn f_060b288b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b288b); } r_060b27eb(cpu, 0x60b288b); }
pub fn f_060b289f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b289f); } r_060b27eb(cpu, 0x60b289f); }
pub fn f_060b28af(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b28af); } r_060b27eb(cpu, 0x60b28af); }
pub fn f_060b28cf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b28cf); } r_060b27eb(cpu, 0x60b28cf); }
pub fn f_060b28d4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b28d4); } r_060b27eb(cpu, 0x60b28d4); }
pub fn f_060b28ee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b28ee); } r_060b27eb(cpu, 0x60b28ee); }
pub fn f_060b28f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b28f3); } r_060b27eb(cpu, 0x60b28f3); }
pub fn f_060b2913(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2913); } r_060b27eb(cpu, 0x60b2913); }
pub fn f_060b2927(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2927); } r_060b27eb(cpu, 0x60b2927); }
pub fn f_060b293f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b293f); } r_060b27eb(cpu, 0x60b293f); }
pub fn f_060b2953(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2953); } r_060b27eb(cpu, 0x60b2953); }
pub fn f_060b2973(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2973); } r_060b27eb(cpu, 0x60b2973); }
pub fn f_060b2978(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2978); } r_060b27eb(cpu, 0x60b2978); }
pub fn f_060b2992(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2992); } r_060b27eb(cpu, 0x60b2992); }
pub fn f_060b2997(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2997); } r_060b27eb(cpu, 0x60b2997); }
pub fn f_060b29b7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b29b7); } r_060b27eb(cpu, 0x60b29b7); }
pub fn f_060b29cb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b29cb); } r_060b27eb(cpu, 0x60b29cb); }
pub fn f_060b29e3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b29e3); } r_060b27eb(cpu, 0x60b29e3); }
pub fn f_060b29f7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b29f7); } r_060b27eb(cpu, 0x60b29f7); }
pub fn f_060b2a17(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a17); } r_060b27eb(cpu, 0x60b2a17); }
pub fn f_060b2a1c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a1c); } r_060b27eb(cpu, 0x60b2a1c); }
pub fn f_060b2a36(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a36); } r_060b27eb(cpu, 0x60b2a36); }
pub fn f_060b2a3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a3b); } r_060b27eb(cpu, 0x60b2a3b); }
pub fn f_060b2a5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a5b); } r_060b27eb(cpu, 0x60b2a5b); }
pub fn f_060b2a6f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a6f); } r_060b27eb(cpu, 0x60b2a6f); }
pub fn f_060b2a87(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a87); } r_060b27eb(cpu, 0x60b2a87); }
pub fn f_060b2a9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2a9b); } r_060b27eb(cpu, 0x60b2a9b); }
pub fn f_060b2abb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2abb); } r_060b27eb(cpu, 0x60b2abb); }
pub fn f_060b2ac0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2ac0); } r_060b27eb(cpu, 0x60b2ac0); }
pub fn f_060b2ada(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2ada); } r_060b27eb(cpu, 0x60b2ada); }
pub fn f_060b2adf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2adf); } r_060b27eb(cpu, 0x60b2adf); }
pub fn f_060b2aff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2aff); } r_060b27eb(cpu, 0x60b2aff); }
pub fn f_060b2b13(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b13); } r_060b27eb(cpu, 0x60b2b13); }
pub fn f_060b2b2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b2b); } r_060b27eb(cpu, 0x60b2b2b); }
pub fn f_060b2b3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b3f); } r_060b27eb(cpu, 0x60b2b3f); }
pub fn f_060b2b5f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b5f); } r_060b27eb(cpu, 0x60b2b5f); }
pub fn f_060b2b64(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b64); } r_060b27eb(cpu, 0x60b2b64); }
pub fn f_060b2b7e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b7e); } r_060b27eb(cpu, 0x60b2b7e); }
pub fn f_060b2b83(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2b83); } r_060b27eb(cpu, 0x60b2b83); }
pub fn f_060b2ba3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2ba3); } r_060b27eb(cpu, 0x60b2ba3); }
pub fn f_060b2bb7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2bb7); } r_060b27eb(cpu, 0x60b2bb7); }
pub fn f_060b2bcf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2bcf); } r_060b27eb(cpu, 0x60b2bcf); }
pub fn f_060b2be3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2be3); } r_060b27eb(cpu, 0x60b2be3); }
pub fn f_060b2c03(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c03); } r_060b27eb(cpu, 0x60b2c03); }
pub fn f_060b2c08(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c08); } r_060b27eb(cpu, 0x60b2c08); }
pub fn f_060b2c22(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c22); } r_060b27eb(cpu, 0x60b2c22); }
pub fn f_060b2c27(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c27); } r_060b27eb(cpu, 0x60b2c27); }
pub fn f_060b2c47(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c47); } r_060b27eb(cpu, 0x60b2c47); }
pub fn f_060b2c5b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c5b); } r_060b27eb(cpu, 0x60b2c5b); }
pub fn f_060b2c73(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b2c73); } r_060b27eb(cpu, 0x60b2c73); }
fn r_060b27eb(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b27eb => {
  { let v: u32 = cpu.sub32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.poll(); pc = cpu.r32(0x617ce30u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x60b27f5 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b27fb => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2800; continue;
 }
 0x60b2800 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2806 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b280b; continue;
 }
 0x60b280b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2811 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2817 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b281d => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2823 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2835); f_060b26d4(cpu); if cpu.returned_to != 0x60b2835 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b283f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2844; continue;
 }
 0x60b2844 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2850); f_060b26d4(cpu); if cpu.returned_to != 0x60b2850 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b285a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b285f; continue;
 }
 0x60b285f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2873); f_060b26d4(cpu); if cpu.returned_to != 0x60b2873 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b287b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2887); f_060b26d4(cpu); if cpu.returned_to != 0x60b2887 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b288b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2897); f_060b26d4(cpu); if cpu.returned_to != 0x60b2897 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b289f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b28ab); f_060b26d4(cpu); if cpu.returned_to != 0x60b28ab { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b28af => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b28c3); f_060b26d4(cpu); if cpu.returned_to != 0x60b28c3 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b28cf => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b28d4; continue;
 }
 0x60b28d4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b28e2); f_060b26d4(cpu); if cpu.returned_to != 0x60b28e2 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b28ee => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b28f3; continue;
 }
 0x60b28f3 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2909); f_060b26d4(cpu); if cpu.returned_to != 0x60b2909 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2913 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2921); f_060b26d4(cpu); if cpu.returned_to != 0x60b2921 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2927 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2935); f_060b26d4(cpu); if cpu.returned_to != 0x60b2935 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b293f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b294d); f_060b26d4(cpu); if cpu.returned_to != 0x60b294d { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2953 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2967); f_060b26d4(cpu); if cpu.returned_to != 0x60b2967 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2973 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2978; continue;
 }
 0x60b2978 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2986); f_060b26d4(cpu); if cpu.returned_to != 0x60b2986 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2992 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2997; continue;
 }
 0x60b2997 => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b29ad); f_060b26d4(cpu); if cpu.returned_to != 0x60b29ad { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b29b7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b29c5); f_060b26d4(cpu); if cpu.returned_to != 0x60b29c5 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b29cb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b29d9); f_060b26d4(cpu); if cpu.returned_to != 0x60b29d9 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b29e3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b29f1); f_060b26d4(cpu); if cpu.returned_to != 0x60b29f1 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b29f7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2a0b); f_060b26d4(cpu); if cpu.returned_to != 0x60b2a0b { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2a17 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2a1c; continue;
 }
 0x60b2a1c => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2a2a); f_060b26d4(cpu); if cpu.returned_to != 0x60b2a2a { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2a36 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2a3b; continue;
 }
 0x60b2a3b => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2a51); f_060b26d4(cpu); if cpu.returned_to != 0x60b2a51 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2a5b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2a69); f_060b26d4(cpu); if cpu.returned_to != 0x60b2a69 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2a6f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2a7d); f_060b26d4(cpu); if cpu.returned_to != 0x60b2a7d { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2a87 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2a95); f_060b26d4(cpu); if cpu.returned_to != 0x60b2a95 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2a9b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2aaf); f_060b26d4(cpu); if cpu.returned_to != 0x60b2aaf { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2abb => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2ac0; continue;
 }
 0x60b2ac0 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2ace); f_060b26d4(cpu); if cpu.returned_to != 0x60b2ace { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2ada => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2adf; continue;
 }
 0x60b2adf => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2af5); f_060b26d4(cpu); if cpu.returned_to != 0x60b2af5 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2aff => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2b0d); f_060b26d4(cpu); if cpu.returned_to != 0x60b2b0d { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2b13 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2b21); f_060b26d4(cpu); if cpu.returned_to != 0x60b2b21 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2b2b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2b39); f_060b26d4(cpu); if cpu.returned_to != 0x60b2b39 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2b3f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2b53); f_060b26d4(cpu); if cpu.returned_to != 0x60b2b53 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2b5f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2b64; continue;
 }
 0x60b2b64 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2b72); f_060b26d4(cpu); if cpu.returned_to != 0x60b2b72 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2b7e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2b83; continue;
 }
 0x60b2b83 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2b99); f_060b26d4(cpu); if cpu.returned_to != 0x60b2b99 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2ba3 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2bb1); f_060b26d4(cpu); if cpu.returned_to != 0x60b2bb1 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2bb7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2bc5); f_060b26d4(cpu); if cpu.returned_to != 0x60b2bc5 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2bcf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2bdd); f_060b26d4(cpu); if cpu.returned_to != 0x60b2bdd { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2be3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2bf7); f_060b26d4(cpu); if cpu.returned_to != 0x60b2bf7 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2c03 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2c08; continue;
 }
 0x60b2c08 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2c16); f_060b26d4(cpu); if cpu.returned_to != 0x60b2c16 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2c22 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x60b2c27; continue;
 }
 0x60b2c27 => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2c3d); f_060b26d4(cpu); if cpu.returned_to != 0x60b2c3d { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2c47 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2c55); f_060b26d4(cpu); if cpu.returned_to != 0x60b2c55 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2c5b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2c69); f_060b26d4(cpu); if cpu.returned_to != 0x60b2c69 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2c73 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2c81); f_060b26d4(cpu); if cpu.returned_to != 0x60b2c81 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b2c87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2c87); }
 let mut pc: u32 = 0x60b2c87;
 loop { match pc {
 0x60b2c87 => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2c96); f_060b26d4(cpu); if cpu.returned_to != 0x60b2c96 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b2c9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2c9a); }
 let mut pc: u32 = 0x60b2c9a;
 loop { match pc {
 0x60b2c9a => {
  { let v: u32 = cpu.esp.wrapping_sub(0x2cu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x60b2ca9); f_060b26d4(cpu); if cpu.returned_to != 0x60b2ca9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b2d0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2d0c); }
 let mut pc: u32 = 0x60b2d0c;
 loop { match pc {
 0x60b2d0c => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7ff00000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7ff00000u32); }
  if cpu.zf { pc = 0x60b2d50; continue; }
  pc = 0x60b2d1d; continue;
 }
 0x60b2d1d => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b2d33; continue; }
  pc = 0x60b2d26; continue;
 }
 0x60b2d26 => {
  { let v = cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x60b2d2f); f_060b2c87(cpu); if cpu.returned_to != 0x60b2d2f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x60b2d33 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf64(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x60b2d44); f_060b2c87(cpu); if cpu.returned_to != 0x60b2d44 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 0x60b2d50 => {
  { let v = cpu.st(0) / cpu.rf64(0x8u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b2d58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2d58); }
 let mut pc: u32 = 0x60b2d58;
 loop { match pc {
 0x60b2d58 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7f800000u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x7f800000u32); }
  if cpu.zf { pc = 0x60b2d9c; continue; }
  pc = 0x60b2d69; continue;
 }
 0x60b2d69 => {
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v: u32 = cpu.and32(cpu.eax, 0x3800u32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b2d7f; continue; }
  pc = 0x60b2d72; continue;
 }
 0x60b2d72 => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x60b2d7b); f_060b2c9a(cpu); if cpu.returned_to != 0x60b2d7b { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x60b2d7f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  cpu.push(0x60b2d90); f_060b2c9a(cpu); if cpu.returned_to != 0x60b2d90 { return; }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x60b2d9c => {
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esp)) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b2f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b2f10); }
 let mut pc: u32 = 0x60b2f10;
 loop { match pc {
 0x60b2f10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617b944u32, v); }
  cpu.push(0x60b2f2b); f_060b3ddb(cpu); if cpu.returned_to != 0x60b2f2b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b2f3b; continue; }
  pc = 0x60b2f36; continue;
 }
 0x60b2f36 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b2f4c; continue; }
  pc = 0x60b2f3b; continue;
 }
 0x60b2f3b => {
  { let v: u32 = 0x3u32; cpu.w32(0x617b944u32, v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2f4c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b2f54); f_060b3eff(cpu); if cpu.returned_to != 0x60b2f54 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  cpu.push(0x60b2f61); f_060b3ed5(cpu); if cpu.returned_to != 0x60b2f61 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b2f6d; continue; }
  pc = 0x60b2f68; continue;
 }
 0x60b2f68 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b2f7b; continue; }
  pc = 0x60b2f6d; continue;
 }
 0x60b2f6d => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b2f76); f_060b3300(cpu); if cpu.returned_to != 0x60b2f76 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b2f7d; continue;
 }
 0x60b2f7b => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x60b2f7d; continue;
 }
 0x60b2f7d => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b2fa0; continue; }
  pc = 0x60b2f85; continue;
 }
 0x60b2f85 => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x617b944u32, v); }
  cpu.push(0x60b2f96); f_060b3f05(cpu); if cpu.returned_to != 0x60b2f96 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2fa0 => {
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b2faa; continue; }
  pc = 0x60b2fa5; continue;
 }
 0x60b2fa5 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b2faa; continue;
 }
 0x60b2faa => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b2fb2); f_060b3e51(cpu); if cpu.returned_to != 0x60b2fb2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x60b2ff3; continue; }
  pc = 0x60b2fb9; continue;
 }
 0x60b2fb9 => {
  { cpu.sub32(cpu.ebp, cpu.ebx); }
  if cpu.zf { pc = 0x60b2fd8; continue; }
  pc = 0x60b2fbd; continue;
 }
 0x60b2fbd => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b2fd8; continue; }
  pc = 0x60b2fc1; continue;
 }
 0x60b2fc1 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b2fcb); f_060b3080(cpu); if cpu.returned_to != 0x60b2fcb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x617b94cu32); cpu.push(0x60b2fd8); cpu.call(t); if cpu.returned_to != 0x60b2fd8 { return; }
  pc = 0x60b2fd8; continue;
 }
 0x60b2fd8 => {
  { let v: u32 = 0x5u32; cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x617b944u32, v); }
  cpu.push(0x60b2fe9); f_060b3f05(cpu); if cpu.returned_to != 0x60b2fe9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b2ff3 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b2ff9); f_060b3f05(cpu); if cpu.returned_to != 0x60b2ff9 { return; }
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
pub fn f_060b3010(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3010); }
 let mut pc: u32 = 0x60b3010;
 loop { match pc {
 0x60b3010 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617b944u32, v); }
  cpu.push(0x60b3025); f_060b3ddb(cpu); if cpu.returned_to != 0x60b3025 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b3033; continue; }
  pc = 0x60b302e; continue;
 }
 0x60b302e => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b3046; continue; }
  pc = 0x60b3033; continue;
 }
 0x60b3033 => {
  { let v: u32 = 0x3u32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617b944u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b3046 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b304e); f_060b3eff(cpu); if cpu.returned_to != 0x60b304e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b3062; continue; }
  pc = 0x60b3058; continue;
 }
 0x60b3058 => {
  { let v: u32 = 0x5u32; cpu.w32(0x617b944u32, v); }
  pc = 0x60b3062; continue;
 }
 0x60b3062 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3068); f_060b3f05(cpu); if cpu.returned_to != 0x60b3068 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3080); }
 let mut pc: u32 = 0x60b3080;
 loop { match pc {
 0x60b3080 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b3092); f_060b30c0(cpu); if cpu.returned_to != 0x60b3092 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b30a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b30a0); }
 let mut pc: u32 = 0x60b30a0;
 loop { match pc {
 0x60b30a0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b30b2); f_060b3160(cpu); if cpu.returned_to != 0x60b30b2 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b30c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b30c0); }
 let mut pc: u32 = 0x60b30c0;
 loop { match pc {
 0x60b30c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if !cpu.cf { pc = 0x60b30d7; continue; }
  pc = 0x60b30d3; continue;
 }
 0x60b30d3 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x60b30d9; continue;
 }
 0x60b30d7 => {
  { let v: u32 = cpu.eax; cpu.esi = v; }
  pc = 0x60b30d9; continue;
 }
 0x60b30d9 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b30dd; continue; }
  pc = 0x60b30db; continue;
 }
 0x60b30db => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60b30dd; continue;
 }
 0x60b30dd => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60b3145); cpu.call(t); if cpu.returned_to != 0x60b3145 { return; }
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
pub fn f_060b3160(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3160); }
 let mut pc: u32 = 0x60b3160;
 loop { match pc {
 0x60b3160 => {
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
  if !cpu.cf { pc = 0x60b3199; continue; }
  pc = 0x60b3195; continue;
 }
 0x60b3195 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x60b319b; continue;
 }
 0x60b3199 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b319b; continue;
 }
 0x60b319b => {
  if (cpu.cf || cpu.zf) { pc = 0x60b31a1; continue; }
  pc = 0x60b319d; continue;
 }
 0x60b319d => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x60b31a3; continue;
 }
 0x60b31a1 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  pc = 0x60b31a3; continue;
 }
 0x60b31a3 => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60b31e6); cpu.call(t); if cpu.returned_to != 0x60b31e6 { return; }
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
pub fn f_060b3200(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3200); }
 let mut pc: u32 = 0x60b3200;
 loop { match pc {
 0x60b3200 => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60b322c); cpu.call(t); if cpu.returned_to != 0x60b322c { return; }
  { cpu.sub32(cpu.r32(0x40u32.wrapping_add(cpu.esp)), 0x0u32); }
  { cpu.sub32(cpu.r32(0x44u32.wrapping_add(cpu.esp)), 0x0u32); }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3240); }
 let mut pc: u32 = 0x60b3240;
 loop { match pc {
 0x60b3240 => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60b3267); cpu.call(t); if cpu.returned_to != 0x60b3267 { return; }
  { cpu.sub32(cpu.r32(0x34u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60b3272; continue; }
  pc = 0x60b326e; continue;
 }
 0x60b326e => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60b32a5; continue;
 }
 0x60b3272 => {
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
  pc = 0x60b32a5; continue;
 }
 0x60b32a5 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b32c6; continue; }
  pc = 0x60b32a9; continue;
 }
 0x60b32a9 => {
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b32c3); f_060b3160(cpu); if cpu.returned_to != 0x60b32c3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60b32c6; continue;
 }
 0x60b32c6 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b32d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b32d0); }
 let mut pc: u32 = 0x60b32d0;
 loop { match pc {
 0x60b32d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b32f3; continue; }
  pc = 0x60b32d9; continue;
 }
 0x60b32d9 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b32e6); f_060b30c0(cpu); if cpu.returned_to != 0x60b32e6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x617b94cu32); cpu.push(0x60b32f3); cpu.call(t); if cpu.returned_to != 0x60b32f3 { return; }
  pc = 0x60b32f3; continue;
 }
 0x60b32f3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3300); }
 let mut pc: u32 = 0x60b3300;
 loop { match pc {
 0x60b3300 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x617b948u32); cpu.push(0x60b3310); cpu.call(t); if cpu.returned_to != 0x60b3310 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b3328; continue; }
  pc = 0x60b3316; continue;
 }
 0x60b3316 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b3320); f_060b3160(cpu); if cpu.returned_to != 0x60b3320 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b3328; continue;
 }
 0x60b3328 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3330); }
 let mut pc: u32 = 0x60b3330;
 loop { match pc {
 0x60b3330 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x617b94cu32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617b94cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b335e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b335e); }
 let mut pc: u32 = 0x60b335e;
 loop { match pc {
 0x60b335e => {
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617baecu32), 0x0u32); }
  if !cpu.zf { pc = 0x60b3372; continue; }
  pc = 0x60b3368; continue;
 }
 0x60b3368 => {
  { cpu.sub16(cpu.r16(0x617baf0u32), 0x0u16); }
  if cpu.zf { pc = 0x60b337f; continue; }
  pc = 0x60b3372; continue;
 }
 0x60b3372 => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x617baecu32); cpu.push(cpu.segments[1] as u32); cpu.push(0x60b337d); cpu.call(t); if cpu.returned_to != 0x60b337d { return; }
  pc = 0x60b33e0; continue;
 }
 0x60b337f => {
  { let v: u32 = cpu.r32(0x617badcu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  pc = 0x60b338b; continue;
 }
 0x60b338b => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b3391; continue;
 }
 0x60b3391 => {
  { let v: u8 = cpu.port_in(0x40u16, 1) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { cpu.sub32(cpu.ecx, cpu.r32(0x617badcu32)); }
  if !cpu.zf { cpu.poll(); pc = 0x60b337f; continue; }
  pc = 0x60b339d; continue;
 }
 0x60b339d => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b33c8; continue; }
  pc = 0x60b33a4; continue;
 }
 0x60b33a4 => {
  { let v: u32 = cpu.r32(0x617bafeu32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x0u32); }
  if !cpu.zf { pc = 0x60b33b4; continue; }
  pc = 0x60b33af; continue;
 }
 0x60b33af => {
  { let v: u32 = 0x10000u32; cpu.ebx = v; }
  pc = 0x60b33b4; continue;
 }
 0x60b33b4 => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  if !cpu.cf { pc = 0x60b33ba; continue; }
  pc = 0x60b33b8; continue;
 }
 0x60b33b8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60b33ba; continue;
 }
 0x60b33ba => {
  { let v: u32 = 0x3e8u32; cpu.eax = v; }
  { let v = cpu.ebx; cpu.mul32(v); }
  { let v: u32 = 0x1234ddu32; cpu.ebx = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x60b33c8; continue;
 }
 0x60b33c8 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617bb02u32); cpu.eax = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x617bae4u32)); cpu.eax = v; }
  pc = 0x60b33e0; continue;
 }
 0x60b33e0 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x617bae0u32)); }
  if !cpu.cf { pc = 0x60b33ed; continue; }
  pc = 0x60b33e8; continue;
 }
 0x60b33e8 => {
  { let v: u32 = cpu.r32(0x617bae0u32); cpu.eax = v; }
  pc = 0x60b33ed; continue;
 }
 0x60b33ed => {
  { let v: u32 = cpu.eax; cpu.w32(0x617bae0u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b33f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b33f4); }
 let mut pc: u32 = 0x60b33f4;
 loop { match pc {
 0x60b33f4 => {
  { cpu.sub32(cpu.r32(0x617bae0u32), 0x0u32); }
  if cpu.zf { pc = 0x60b3411; continue; }
  pc = 0x60b33fd; continue;
 }
 0x60b33fd => {
  cpu.push(0x60b3402); f_060b335e(cpu); if cpu.returned_to != 0x60b3402 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x617badcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x617bae4u32, v); }
  pc = 0x60b3411; continue;
 }
 0x60b3411 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3412(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3412); }
 let mut pc: u32 = 0x60b3412;
 loop { match pc {
 0x60b3412 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x617bb0eu32); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x617badcu32.wrapping_add(cpu.bases[3]))); cpu.w32(0x617badcu32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.r32(0x617bb06u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b35ad; continue; }
  pc = 0x60b342e; continue;
 }
 0x60b342e => {
  { cpu.pushad(); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.df = false; }
  { let v = cpu.r16(0x617bb0eu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.inc32(cpu.r32(0x617bb06u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x617bb06u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x617c520u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x617c524u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = (cpu.eax as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x617c520u32; cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x617baecu32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60b346e; continue; }
  pc = 0x60b3464; continue;
 }
 0x60b3464 => {
  { cpu.sub16(cpu.r16(0x617baf0u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if cpu.zf { pc = 0x60b3494; continue; }
  pc = 0x60b346e; continue;
 }
 0x60b346e => {
  { let v: u32 = 0x101u32; cpu.eax = v; }
  let t = cpu.r32(0x617baecu32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60b3473); cpu.push(cpu.segments[1] as u32); cpu.push(0x60b3479); cpu.call(t); if cpu.returned_to != 0x60b3479 { return; }
  { let v: u32 = 0x3e8u32; cpu.ecx = v; }
  { cpu.sub32(cpu.r32(0x617bae8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60b34b5; continue; }
  pc = 0x60b3487; continue;
 }
 0x60b3487 => {
  { let v: u32 = cpu.r32(0x617bb02u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617bae8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60b34d5; continue;
 }
 0x60b3494 => {
  { let v: u32 = cpu.r32(0x617badcu32.wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x617bae8u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if !cpu.zf { pc = 0x60b34af; continue; }
  pc = 0x60b34a2; continue;
 }
 0x60b34a2 => {
  { let v: u32 = cpu.r32(0x617bb02u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617bae8u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x60b34d5; continue;
 }
 0x60b34af => {
  { let v: u32 = cpu.r32(0x617bb02u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  pc = 0x60b34b5; continue;
 }
 0x60b34b5 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x617bae8u32.wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x617bae8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xf4240u32); }
  if cpu.cf { pc = 0x60b34d5; continue; }
  pc = 0x60b34d0; continue;
 }
 0x60b34d0 => {
  { let v: u32 = 0xf4240u32; cpu.edx = v; }
  pc = 0x60b34d5; continue;
 }
 0x60b34d5 => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x60b34da; continue;
 }
 0x60b34da => {
  { cpu.sub32(cpu.r32(0x617b99cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x2u32); }
  if !cpu.zf { pc = 0x60b3507; continue; }
  pc = 0x60b34e3; continue;
 }
 0x60b34e3 => {
  { let v: u32 = cpu.r32(0x617b9dcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  pc = 0x60b34eb; continue;
 }
 0x60b34eb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x617ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); }
  if cpu.cf { pc = 0x60b3501; continue; }
  pc = 0x60b34f3; continue;
 }
 0x60b34f3 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x617ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x617ba5cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x617ba5cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60b34eb; continue;
 }
 0x60b3501 => {
  { let v: u32 = cpu.eax; cpu.w32(0x617b9dcu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  pc = 0x60b3507; continue;
 }
 0x60b3507 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x60b34da; continue; }
  pc = 0x60b350f; continue;
 }
 0x60b350f => {
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  {  }
  { cpu.sub32(cpu.r32(0x617bb0au32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b3548; continue; }
  pc = 0x60b351d; continue;
 }
 0x60b351d => {
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x60b3522; continue;
 }
 0x60b3522 => {
  { cpu.sub32(cpu.r32(0x617ba5cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x60b3540; continue; }
  pc = 0x60b352b; continue;
 }
 0x60b352b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x617ba5cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]))); cpu.w32(0x617ba5cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x617ba9cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b95cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x60b3538); cpu.push(0x60b353e); cpu.call(t); if cpu.returned_to != 0x60b353e { return; }
  cpu.poll(); pc = 0x60b3522; continue;
 }
 0x60b3540 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x60b3522; continue; }
  pc = 0x60b3548; continue;
 }
 0x60b3548 => {
  { cpu.sub32(cpu.r32(0x617ba98u32.wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x60b3591; continue; }
  pc = 0x60b3551; continue;
 }
 0x60b3551 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x617ba98u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x617ba98u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x617c520u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x617c524u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x617bb06u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x617bb06u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x617baf2u32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  { let v: u32 = cpu.r16(0x617baf6u32.wrapping_add(cpu.bases[3])) as u32; cpu.eax = v; }
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
 0x60b3591 => {
  { let v = cpu.r16(0x617c520u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x617c524u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x617bb06u32.wrapping_add(cpu.bases[3]))); cpu.w32(0x617bb06u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(4, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.popad(); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60b35ac); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x60b35ad => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60b35b4); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b35b5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b35b5); }
 let mut pc: u32 = 0x60b35b5;
 loop { match pc {
 0x60b35b5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v: u8 = 0x34u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x43u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617bafeu32, v); }
  pc = 0x60b35c8; continue;
 }
 0x60b35c8 => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b35ce; continue;
 }
 0x60b35ce => {
  { cpu.port_out(0x40u16, 1, (cpu.eax as u8) as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x60b35db; continue; }
  pc = 0x60b35da; continue;
 }
 0x60b35da => {
  {  }
  pc = 0x60b35db; continue;
 }
 0x60b35db => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b35df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b35df); }
 let mut pc: u32 = 0x60b35df;
 loop { match pc {
 0x60b35df => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0xd68du32); }
  if !cpu.cf { pc = 0x60b3602; continue; }
  pc = 0x60b35f1; continue;
 }
 0x60b35f1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x20bcu32; cpu.ebx = v; }
  { let v: u32 = 0x2710u32; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.mul32(v); }
  { let v = cpu.ebx; cpu.div32(v); }
  pc = 0x60b3602; continue;
 }
 0x60b3602 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b3608); f_060b35b5(cpu); if cpu.returned_to != 0x60b3608 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b360e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b360e); }
 let mut pc: u32 = 0x60b360e;
 loop { match pc {
 0x60b360e => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x60b3610); cpu.push(0x60b3615); f_060ad100(cpu); if cpu.returned_to != 0x60b3615 { return; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.edi = v; }
  pc = 0x60b3622; continue;
 }
 0x60b3622 => {
  { cpu.sub32(cpu.r32(0x617b99cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), 0x0u32); }
  if cpu.zf { pc = 0x60b3637; continue; }
  pc = 0x60b362b; continue;
 }
 0x60b362b => {
  { let v: u32 = cpu.r32(0x617ba1cu32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.cf { pc = 0x60b3637; continue; }
  pc = 0x60b3635; continue;
 }
 0x60b3635 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x60b3637; continue;
 }
 0x60b3637 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if cpu.cf { cpu.poll(); pc = 0x60b3622; continue; }
  pc = 0x60b363f; continue;
 }
 0x60b363f => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x617bb02u32.wrapping_add(cpu.bases[3]))); }
  if cpu.zf { pc = 0x60b366c; continue; }
  pc = 0x60b3647; continue;
 }
 0x60b3647 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x617bb02u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.flat_ds(0x60b364e); cpu.push(0x60b3653); f_060b35df(cpu); if cpu.returned_to != 0x60b3653 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x617b9dcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  cpu.flat_ds(0x60b3667); cpu.push(0x60b366c); f_060b33f4(cpu); if cpu.returned_to != 0x60b366c { return; }
  pc = 0x60b366c; continue;
 }
 0x60b366c => {
  cpu.flat_ds(0x60b366c); cpu.push(0x60b3671); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3671 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.flat_ds(0x60b3673); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b378a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b378a); }
 let mut pc: u32 = 0x60b378a;
 loop { match pc {
 0x60b378a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x60b378f); cpu.push(0x60b3794); f_060b35b5(cpu); if cpu.returned_to != 0x60b3794 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617baf2u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.r16(0x617baf6u32.wrapping_add(cpu.bases[3])) as u32; cpu.ebx = v; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x60b37bd; continue; }
  pc = 0x60b37bc; continue;
 }
 0x60b37bc => {
  {  }
  pc = 0x60b37bd; continue;
 }
 0x60b37bd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60b37c0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b37c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b37c1); }
 let mut pc: u32 = 0x60b37c1;
 loop { match pc {
 0x60b37c1 => {
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
pub fn f_060b37e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b37e9); }
 let mut pc: u32 = 0x60b37e9;
 loop { match pc {
 0x60b37e9 => {
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
pub fn f_060b380c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b380c); } r_060b380c(cpu, 0x60b380c); }
pub fn f_060b3813(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3813); } r_060b380c(cpu, 0x60b3813); }
pub fn f_060b3822(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3822); } r_060b380c(cpu, 0x60b3822); }
pub fn f_060b3828(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3828); } r_060b380c(cpu, 0x60b3828); }
pub fn f_060b383e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b383e); } r_060b380c(cpu, 0x60b383e); }
pub fn f_060b3848(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3848); } r_060b380c(cpu, 0x60b3848); }
pub fn f_060b384f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b384f); } r_060b380c(cpu, 0x60b384f); }
fn r_060b380c(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b380c => {
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
 0x60b3813 => {
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
 0x60b3822 => {
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
 0x60b3828 => {
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
 0x60b383e => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.esp.wrapping_sub(0x6u32); cpu.esp = v; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffeu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x60b3848 => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u16 = 0x9999u16; cpu.w16(0xfffffffcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 0x60b384f => {
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.pop(); cpu.pop(); cpu.esp = cpu.esp.wrapping_add(0); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3856(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3856); }
 let mut pc: u32 = 0x60b3856;
 loop { match pc {
 0x60b3856 => {
  { let v = (cpu.ebx as u16); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.ecx; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x60b3860); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3861(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3861); }
 let mut pc: u32 = 0x60b3861;
 loop { match pc {
 0x60b3861 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffe4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.sub32(cpu.r32(0x617bb12u32), 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b3a71; continue; }
  pc = 0x60b3878; continue;
 }
 0x60b3878 => {
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
  cpu.unimplemented(0x60b38d2, "Lar");
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
  cpu.unimplemented(0x60b3923, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x617bb20u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x617bb20u32; cpu.eax = v; }
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
  cpu.unimplemented(0x60b3986, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x60b380cu32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff0u32; cpu.ecx = v; }
  { let v: u32 = 0x60b380cu32; cpu.eax = v; }
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
  cpu.unimplemented(0x60b39e9, "Lar");
  { let v: u32 = cpu.shr32(cpu.ecx, 8u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ecx as u8) | 0x8u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0xc0u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.r32(0xffffffecu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x200u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60b3828u32, v); }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x60b3822u32, v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x60b3813u32, v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x60b3848u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x60b384fu32, v); }
  { let v: u32 = 0x4au32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x60b383eu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617bb12u32, v); }
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x60b3a46; continue; }
  pc = 0x60b3a43; continue;
 }
 0x60b3a43 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x60b3a46; continue;
 }
 0x60b3a46 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x204u32; cpu.eax = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617bb16u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x617bb1au32, v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xffffffe4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  pc = 0x60b3a71; continue;
 }
 0x60b3a71 => {
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
pub fn f_060b3a77(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3a77); }
 let mut pc: u32 = 0x60b3a77;
 loop { match pc {
 0x60b3a77 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b3ab6; continue; }
  pc = 0x60b3a86; continue;
 }
 0x60b3a86 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x617bb12u32)); }
  if !cpu.zf { pc = 0x60b3ab6; continue; }
  pc = 0x60b3a8e; continue;
 }
 0x60b3a8e => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if cpu.cf { pc = 0x60b3a96; continue; }
  pc = 0x60b3a93; continue;
 }
 0x60b3a93 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x60u32); cpu.ebx = v; }
  pc = 0x60b3a96; continue;
 }
 0x60b3a96 => {
  { let v: u32 = cpu.add32(cpu.ebx, 0x8u32); cpu.ebx = v; }
  { let v: u32 = 0x205u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617bb16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x617bb1au32); cpu.edx = v; }
  { cpu.interrupt(0x31); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x617bb12u32, v); }
  pc = 0x60b3ab6; continue;
 }
 0x60b3ab6 => {
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
pub fn f_060b3ac0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3ac0); }
 let mut pc: u32 = 0x60b3ac0;
 loop { match pc {
 0x60b3ac0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x60b3ad1; continue; }
  pc = 0x60b3ad0; continue;
 }
 0x60b3ad0 => {
  {  }
  pc = 0x60b3ad1; continue;
 }
 0x60b3ad1 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3ad5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3ad5); }
 let mut pc: u32 = 0x60b3ad5;
 loop { match pc {
 0x60b3ad5 => {
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
pub fn f_060b3b27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3b27); }
 let mut pc: u32 = 0x60b3b27;
 loop { match pc {
 0x60b3b27 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xffffffc4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.flat_ds(0x60b3b32); cpu.push(0x60b3b37); f_060ad100(cpu); if cpu.returned_to != 0x60b3b37 { return; }
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
  if cpu.zf { pc = 0x60b3b96; continue; }
  pc = 0x60b3b76; continue;
 }
 0x60b3b76 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.esi as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b3b96; continue;
 }
 0x60b3b96 => {
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
  if cpu.zf { pc = 0x60b3bef; continue; }
  pc = 0x60b3bc0; continue;
 }
 0x60b3bc0 => {
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
  pc = 0x60b3bef; continue;
 }
 0x60b3bef => {
  { let v: u32 = cpu.r16(0xffffffe2u32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.flat_ds(0x60b3bf4); cpu.push(0x60b3bf9); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3bf9 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.flat_ds(0x60b3c00); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3c01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3c01); }
 let mut pc: u32 = 0x60b3c01;
 loop { match pc {
 0x60b3c01 => {
  { let v: u32 = cpu.r32(0x617bb06u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3c07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3c07); }
 let mut pc: u32 = 0x60b3c07;
 loop { match pc {
 0x60b3c07 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3c0f); f_060ad100(cpu); if cpu.returned_to != 0x60b3c0f { return; }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  pc = 0x60b3c14; continue;
 }
 0x60b3c14 => {
  { cpu.sub32(cpu.r32(0x617b99cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x60b3c2c; continue; }
  pc = 0x60b3c1d; continue;
 }
 0x60b3c1d => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3cu32); }
  if cpu.cf { cpu.poll(); pc = 0x60b3c14; continue; }
  pc = 0x60b3c25; continue;
 }
 0x60b3c25 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b3c3f; continue;
 }
 0x60b3c2c => {
  { let v: u32 = 0x1u32; cpu.w32(0x617b99cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617b95cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b3c3f; continue;
 }
 0x60b3c3f => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b3c45); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3c45 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3c48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3c48); }
 let mut pc: u32 = 0x60b3c48;
 loop { match pc {
 0x60b3c48 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3c50); f_060ad100(cpu); if cpu.returned_to != 0x60b3c50 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b3c61; continue; }
  pc = 0x60b3c58; continue;
 }
 0x60b3c58 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let a = cpu.r32(0x617ba9cu32.wrapping_add(cpu.ecx)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x617ba9cu32.wrapping_add(cpu.ecx), v); } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x60b3c61; continue;
 }
 0x60b3c61 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b3c67); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3c67 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3c6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3c6a); }
 let mut pc: u32 = 0x60b3c6a;
 loop { match pc {
 0x60b3c6a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3c72); f_060ad100(cpu); if cpu.returned_to != 0x60b3c72 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b3c84; continue; }
  pc = 0x60b3c7a; continue;
 }
 0x60b3c7a => {
  { let v: u32 = 0x0u32; cpu.w32(0x617b99cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b3c84; continue;
 }
 0x60b3c84 => {
  cpu.push(0x60b3c89); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3c89 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3c8b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3c8b); }
 let mut pc: u32 = 0x60b3c8b;
 loop { match pc {
 0x60b3c8b => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3c91); f_060ad100(cpu); if cpu.returned_to != 0x60b3c91 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x60b3c96; continue;
 }
 0x60b3c96 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3c9c); f_060ac030(cpu); if cpu.returned_to != 0x60b3c9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b3c96; continue; }
  pc = 0x60b3ca4; continue;
 }
 0x60b3ca4 => {
  cpu.push(0x60b3ca9); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3ca9 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3cab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3cab); }
 let mut pc: u32 = 0x60b3cab;
 loop { match pc {
 0x60b3cab => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3cb3); f_060ad100(cpu); if cpu.returned_to != 0x60b3cb3 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b3cce; continue; }
  pc = 0x60b3cbb; continue;
 }
 0x60b3cbb => {
  { cpu.sub32(cpu.r32(0x617b99cu32.wrapping_add(cpu.ecx)), 0x1u32); }
  if !cpu.zf { pc = 0x60b3cce; continue; }
  pc = 0x60b3cc4; continue;
 }
 0x60b3cc4 => {
  { let v: u32 = 0x2u32; cpu.w32(0x617b99cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b3cce; continue;
 }
 0x60b3cce => {
  cpu.push(0x60b3cd3); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3cd3 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3cd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3cd5); }
 let mut pc: u32 = 0x60b3cd5;
 loop { match pc {
 0x60b3cd5 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3cdb); f_060ad100(cpu); if cpu.returned_to != 0x60b3cdb { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x60b3ce0; continue;
 }
 0x60b3ce0 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3ce6); f_060ac290(cpu); if cpu.returned_to != 0x60b3ce6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b3ce0; continue; }
  pc = 0x60b3cee; continue;
 }
 0x60b3cee => {
  cpu.push(0x60b3cf3); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3cf3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3cf5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3cf5); }
 let mut pc: u32 = 0x60b3cf5;
 loop { match pc {
 0x60b3cf5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3cfd); f_060ad100(cpu); if cpu.returned_to != 0x60b3cfd { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b3d18; continue; }
  pc = 0x60b3d05; continue;
 }
 0x60b3d05 => {
  { cpu.sub32(cpu.r32(0x617b99cu32.wrapping_add(cpu.ecx)), 0x2u32); }
  if !cpu.zf { pc = 0x60b3d18; continue; }
  pc = 0x60b3d0e; continue;
 }
 0x60b3d0e => {
  { let v: u32 = 0x1u32; cpu.w32(0x617b99cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b3d18; continue;
 }
 0x60b3d18 => {
  cpu.push(0x60b3d1d); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3d1d { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3d1f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3d1f); }
 let mut pc: u32 = 0x60b3d1f;
 loop { match pc {
 0x60b3d1f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3d25); f_060ad100(cpu); if cpu.returned_to != 0x60b3d25 { return; }
  { let v: u32 = 0x38u32; cpu.esi = v; }
  pc = 0x60b3d2a; continue;
 }
 0x60b3d2a => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b3d30); f_060ac160(cpu); if cpu.returned_to != 0x60b3d30 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.esi, 0x4u32); cpu.esi = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b3d2a; continue; }
  pc = 0x60b3d38; continue;
 }
 0x60b3d38 => {
  cpu.push(0x60b3d3d); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3d3d { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3d3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3d3f); }
 let mut pc: u32 = 0x60b3d3f;
 loop { match pc {
 0x60b3d3f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b3d4a); f_060ad100(cpu); if cpu.returned_to != 0x60b3d4a { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617ba1cu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x617b9dcu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x60b3d65); f_060b360e(cpu); if cpu.returned_to != 0x60b3d65 { return; }
  cpu.push(0x60b3d6a); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3d6a { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3d6f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3d6f); }
 let mut pc: u32 = 0x60b3d6f;
 loop { match pc {
 0x60b3d6f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3d77); f_060ad100(cpu); if cpu.returned_to != 0x60b3d77 { return; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.div32(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x60b3d8f); f_060ac5a0(cpu); if cpu.returned_to != 0x60b3d8f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x60b3d97); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3d97 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3d99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3d99); }
 let mut pc: u32 = 0x60b3d99;
 loop { match pc {
 0x60b3d99 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  cpu.push(0x60b3da1); f_060ad100(cpu); if cpu.returned_to != 0x60b3da1 { return; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b3dae; continue; }
  pc = 0x60b3da7; continue;
 }
 0x60b3da7 => {
  { let v: u32 = 0xd68du32; cpu.eax = v; }
  pc = 0x60b3dc2; continue;
 }
 0x60b3dae => {
  { let v: u32 = 0x2710u32; cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.edx = v; }
  { let v: u32 = 0x2e9cu32; cpu.ecx = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.mul32(v); }
  { let v = cpu.ecx; cpu.div32(v); }
  pc = 0x60b3dc2; continue;
 }
 0x60b3dc2 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x60b3dcb); f_060ac5a0(cpu); if cpu.returned_to != 0x60b3dcb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.push(0x60b3dd3); f_060ad0f0(cpu); if cpu.returned_to != 0x60b3dd3 { return; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3dd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3dd5); }
 let mut pc: u32 = 0x60b3dd5;
 loop { match pc {
 0x60b3dd5 => {
  { let v: u32 = cpu.r32(0x617bafeu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3ddb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3ddb); }
 let mut pc: u32 = 0x60b3ddb;
 loop { match pc {
 0x60b3ddb => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3du8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x60b3df0; continue; }
  pc = 0x60b3dec; continue;
 }
 0x60b3dec => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60b3df0; continue;
 }
 0x60b3df0 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3df7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3df7); }
 let mut pc: u32 = 0x60b3df7;
 loop { match pc {
 0x60b3df7 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x60b3e0b; continue; }
  pc = 0x60b3e07; continue;
 }
 0x60b3e07 => {
  { let v: u16 = 0xffffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60b3e0b; continue;
 }
 0x60b3e0b => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3e12(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3e12); }
 let mut pc: u32 = 0x60b3e12;
 loop { match pc {
 0x60b3e12 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { cpu.interrupt(0x21); }
  if !cpu.cf { pc = 0x60b3e3d; continue; }
  pc = 0x60b3e23; continue;
 }
 0x60b3e23 => {
  { cpu.sub16((cpu.eax as u16), 0x2u16); }
  if cpu.zf { pc = 0x60b3e30; continue; }
  pc = 0x60b3e29; continue;
 }
 0x60b3e29 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x60b3e4c; continue;
 }
 0x60b3e30 => {
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { cpu.interrupt(0x21); }
  if cpu.cf { cpu.poll(); pc = 0x60b3e29; continue; }
  pc = 0x60b3e38; continue;
 }
 0x60b3e38 => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  pc = 0x60b3e4c; continue;
 }
 0x60b3e3d => {
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x60b3e4c; continue;
 }
 0x60b3e4c => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3e51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3e51); }
 let mut pc: u32 = 0x60b3e51;
 loop { match pc {
 0x60b3e51 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x60b3e5a; continue;
 }
 0x60b3e5a => {
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x60b3e6c; continue; }
  pc = 0x60b3e67; continue;
 }
 0x60b3e67 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x60b3e6c; continue;
 }
 0x60b3e6c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x60b3e89; continue; }
  pc = 0x60b3e77; continue;
 }
 0x60b3e77 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b3e89; continue; }
  pc = 0x60b3e7e; continue;
 }
 0x60b3e7e => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x60b3e89; continue; }
  pc = 0x60b3e84; continue;
 }
 0x60b3e84 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x60b3e5a; continue; }
  pc = 0x60b3e89; continue;
 }
 0x60b3e89 => {
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
pub fn f_060b3e93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3e93); }
 let mut pc: u32 = 0x60b3e93;
 loop { match pc {
 0x60b3e93 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  pc = 0x60b3e9c; continue;
 }
 0x60b3e9c => {
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if (cpu.cf || cpu.zf) { pc = 0x60b3eae; continue; }
  pc = 0x60b3ea9; continue;
 }
 0x60b3ea9 => {
  { let v: u32 = 0xf000u32; cpu.ecx = v; }
  pc = 0x60b3eae; continue;
 }
 0x60b3eae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  if cpu.cf { pc = 0x60b3ecb; continue; }
  pc = 0x60b3eb9; continue;
 }
 0x60b3eb9 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x60b3ecb; continue; }
  pc = 0x60b3ec0; continue;
 }
 0x60b3ec0 => {
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.cf { pc = 0x60b3ecb; continue; }
  pc = 0x60b3ec6; continue;
 }
 0x60b3ec6 => {
  { let v: u32 = cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebp)), cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  if !cpu.zf { cpu.poll(); pc = 0x60b3e9c; continue; }
  pc = 0x60b3ecb; continue;
 }
 0x60b3ecb => {
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
pub fn f_060b3ed5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3ed5); } r_060b3ed5(cpu, 0x60b3ed5); }
pub fn f_060b3eff(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b3eff); } r_060b3ed5(cpu, 0x60b3eff); }
fn r_060b3ed5(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b3ed5 => {
  { let v: u16 = 0x4200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x60b3ed9; continue;
 }
 0x60b3ed9 => {
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
 0x60b3eff => {
  { let v: u16 = 0x4202u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x60b3ed9; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3f05(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3f05); }
 let mut pc: u32 = 0x60b3f05;
 loop { match pc {
 0x60b3f05 => {
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
pub fn f_060b3f13(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3f13); }
 let mut pc: u32 = 0x60b3f13;
 loop { match pc {
 0x60b3f13 => {
  { let v = 0x617c528u32; cpu.push(v); }
  { let v = 0x617b95cu32; cpu.push(v); }
  cpu.push(0x60b3f22); f_060b3160(cpu); if cpu.returned_to != 0x60b3f22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x60b3f13u32; cpu.push(v); }
  { let v = 0x60b3350u32; cpu.push(v); }
  cpu.push(0x60b3f34); f_060b3160(cpu); if cpu.returned_to != 0x60b3f34 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3f40); }
 let mut pc: u32 = 0x60b3f40;
 loop { match pc {
 0x60b3f40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c62cu32), 0x0u32); }
  if cpu.zf { pc = 0x60b3fc3; continue; }
  pc = 0x60b3f4c; continue;
 }
 0x60b3f4c => {
  { let v = 0x60b3f40u32; cpu.push(v); }
  { let v = 0x60b4c30u32; cpu.push(v); }
  cpu.push(0x60b3f5b); f_060b30c0(cpu); if cpu.returned_to != 0x60b3f5b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x62d1d60u32; cpu.push(v); }
  cpu.push(0x60b3f6a); f_060b3080(cpu); if cpu.returned_to != 0x60b3f6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x62d1ea0u32; cpu.push(v); }
  cpu.push(0x60b3f79); f_060b3080(cpu); if cpu.returned_to != 0x60b3f79 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x62d1da0u32; cpu.push(v); }
  cpu.push(0x60b3f8b); f_060b3080(cpu); if cpu.returned_to != 0x60b3f8b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x62d1ef0u32; cpu.push(v); }
  cpu.push(0x60b3f9a); f_060b3080(cpu); if cpu.returned_to != 0x60b3f9a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c628u32; cpu.push(v); }
  cpu.push(0x60b3fa9); f_060b3080(cpu); if cpu.returned_to != 0x60b3fa9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f08u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60b3fba); f_060b3080(cpu); if cpu.returned_to != 0x60b3fba { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c62cu32, v); }
  pc = 0x60b3fc3; continue;
 }
 0x60b3fc3 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3fd0); }
 let mut pc: u32 = 0x60b3fd0;
 loop { match pc {
 0x60b3fd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  cpu.push(0x60b3fdd); f_060b335e(cpu); if cpu.returned_to != 0x60b3fdd { return; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  pc = 0x60b3fdf; continue;
 }
 0x60b3fdf => {
  cpu.push(0x60b3fe4); f_060b335e(cpu); if cpu.returned_to != 0x60b3fe4 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x60b3fdf; continue; }
  pc = 0x60b3fe8; continue;
 }
 0x60b3fe8 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b3ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b3ff0); }
 let mut pc: u32 = 0x60b3ff0;
 loop { match pc {
 0x60b3ff0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60b4005; continue; }
  pc = 0x60b3ffb; continue;
 }
 0x60b3ffb => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x617c528u32, v); }
  pc = 0x60b404b; continue;
 }
 0x60b4005 => {
  { let v: u32 = 0x617c528u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b400c; continue;
 }
 0x60b400c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b400c; continue; }
  pc = 0x60b4016; continue;
 }
 0x60b4016 => {
  { let v: u32 = 0x617c528u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b404b; continue; }
  pc = 0x60b402b; continue;
 }
 0x60b402b => {
  { let v: u8 = cpu.r8(0x617c527u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if cpu.zf { pc = 0x60b404b; continue; }
  pc = 0x60b4036; continue;
 }
 0x60b4036 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x3au8); }
  if cpu.zf { pc = 0x60b404b; continue; }
  pc = 0x60b403b; continue;
 }
 0x60b403b => {
  { let v: u8 = 0x5cu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.ebx as u8), (cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x617c528u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x617c529u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b404b; continue;
 }
 0x60b404b => {
  { let v: u32 = 0x617c528u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4060); }
 let mut pc: u32 = 0x60b4060;
 loop { match pc {
 0x60b4060 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  pc = 0x60b4066; continue;
 }
 0x60b4066 => {
  { let v: u32 = cpu.r32(0x62d1d60u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b4079; continue; }
  pc = 0x60b4070; continue;
 }
 0x60b4070 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b4076); f_060abcf0(cpu); if cpu.returned_to != 0x60b4076 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b4079; continue;
 }
 0x60b4079 => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xfffffffcu32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4066; continue; }
  pc = 0x60b4081; continue;
 }
 0x60b4081 => {
  cpu.push(0x60b4086); f_060abfb0(cpu); if cpu.returned_to != 0x60b4086 { return; }
  cpu.push(0x60b408b); f_060b378a(cpu); if cpu.returned_to != 0x60b408b { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4090(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4090); }
 let mut pc: u32 = 0x60b4090;
 loop { match pc {
 0x60b3674 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.flags(); cpu.push(v); }
  {  }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { cpu.df = false; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x617bb0eu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.r16(0x617bb0eu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x60b368a); cpu.push(0x60b368f); f_060b3f13(cpu); if cpu.returned_to != 0x60b368f { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x617bb06u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x617bb0au32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x617bb02u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x617bb12u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0x617b99cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x617b9dcu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x617ba1cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { let v: u32 = 0x617ba5cu32; cpu.edi = v; }
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  { cpu.stos(4, true); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = 0x1684u32; cpu.eax = v; }
  { let v: u32 = 0x5u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v = (cpu.edi as u16); cpu.set_segment(0, v as u16); }
  { cpu.interrupt(0x2f); }
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x617baecu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x617baeeu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x617baf0u32.wrapping_add(cpu.bases[3]), v); }
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
  { let v: u32 = cpu.ebx; cpu.w32(0x617baf2u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x617baf6u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x617bafau32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x8u32; cpu.eax = v; }
  { let v: u32 = 0x60b3412u32; cpu.edx = v; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = (cpu.ebx as u16); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x2u32; cpu.w32(0x617b9d8u32.wrapping_add(cpu.bases[3]), v); }
  { let v = 0xd68du32; cpu.push(v); }
  { let v = 0x3cu32; cpu.push(v); }
  cpu.flat_ds(0x60b3771); cpu.push(0x60b3776); f_060ac5a0(cpu); if cpu.returned_to != 0x60b3776 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { cpu.and8(cpu.r8(0x5u32.wrapping_add(cpu.ebp)), 0x2u8); }
  {  }
  if cpu.zf { pc = 0x60b3784; continue; }
  pc = 0x60b3783; continue;
 }
 0x60b3783 => {
  {  }
  pc = 0x60b3784; continue;
 }
 0x60b3784 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_flags(v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.flat_ds(0x60b3789); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b4090 => {
  cpu.flat_ds(0x60b4090); cpu.push(0x60b4095); f_060b4c30(cpu); if cpu.returned_to != 0x60b4095 { return; }
  { let v = 0xc8u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.flat_ds(0x60b409c); cpu.push(0x60b40a1); f_060ad110(cpu); if cpu.returned_to != 0x60b40a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  cpu.flat_ds(0x60b40a8); cpu.push(0x60b40ad); f_060ad110(cpu); if cpu.returned_to != 0x60b40ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8000u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  cpu.flat_ds(0x60b40b7); cpu.push(0x60b40bc); f_060ad110(cpu); if cpu.returned_to != 0x60b40bc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  cpu.flat_ds(0x60b40c3); cpu.push(0x60b40c8); f_060ad110(cpu); if cpu.returned_to != 0x60b40c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x10u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  cpu.flat_ds(0x60b40cf); cpu.push(0x60b40d4); f_060ad110(cpu); if cpu.returned_to != 0x60b40d4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v = 0x5u32; cpu.push(v); }
  cpu.flat_ds(0x60b40db); cpu.push(0x60b40e0); f_060ad110(cpu); if cpu.returned_to != 0x60b40e0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x28fu32; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  cpu.flat_ds(0x60b40ea); cpu.push(0x60b40ef); f_060ad110(cpu); if cpu.returned_to != 0x60b40ef { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  cpu.flat_ds(0x60b40f6); cpu.push(0x60b40fb); f_060ad110(cpu); if cpu.returned_to != 0x60b40fb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.flat_ds(0x60b4102); cpu.push(0x60b4107); f_060ad110(cpu); if cpu.returned_to != 0x60b4107 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x9u32; cpu.push(v); }
  cpu.flat_ds(0x60b410e); cpu.push(0x60b4113); f_060ad110(cpu); if cpu.returned_to != 0x60b4113 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  cpu.flat_ds(0x60b411a); cpu.push(0x60b411f); f_060ad110(cpu); if cpu.returned_to != 0x60b411f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x78u32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  cpu.flat_ds(0x60b4126); cpu.push(0x60b412b); f_060ad110(cpu); if cpu.returned_to != 0x60b412b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x8u32; cpu.push(v); }
  { let v = 0xcu32; cpu.push(v); }
  cpu.flat_ds(0x60b4132); cpu.push(0x60b4137); f_060ad110(cpu); if cpu.returned_to != 0x60b4137 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xdu32; cpu.push(v); }
  cpu.flat_ds(0x60b413e); cpu.push(0x60b4143); f_060ad110(cpu); if cpu.returned_to != 0x60b4143 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0xeu32; cpu.push(v); }
  cpu.flat_ds(0x60b414a); cpu.push(0x60b414f); f_060ad110(cpu); if cpu.returned_to != 0x60b414f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xfu32; cpu.push(v); }
  cpu.flat_ds(0x60b4156); cpu.push(0x60b415b); f_060ad110(cpu); if cpu.returned_to != 0x60b415b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x2u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  cpu.flat_ds(0x60b4162); cpu.push(0x60b4167); f_060ad110(cpu); if cpu.returned_to != 0x60b4167 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x11u32; cpu.push(v); }
  cpu.flat_ds(0x60b416e); cpu.push(0x60b4173); f_060ad110(cpu); if cpu.returned_to != 0x60b4173 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x12u32; cpu.push(v); }
  cpu.flat_ds(0x60b417a); cpu.push(0x60b417f); f_060ad110(cpu); if cpu.returned_to != 0x60b417f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x13u32; cpu.push(v); }
  cpu.flat_ds(0x60b4186); cpu.push(0x60b418b); f_060ad110(cpu); if cpu.returned_to != 0x60b418b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b4190; continue;
 }
 0x60b4190 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d1d5cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4190; continue; }
  pc = 0x60b41a0; continue;
 }
 0x60b41a0 => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x62d1da0u32.wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60b3674; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b41b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b41b0); }
 let mut pc: u32 = 0x60b41b0;
 loop { match pc {
 0x60b41b0 => {
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
  if cpu.zf { pc = 0x60b424e; continue; }
  pc = 0x60b41d4; continue;
 }
 0x60b41d4 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b950u32); cpu.push(0x60b41db); cpu.call(t); if cpu.returned_to != 0x60b41db { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b424e; continue; }
  pc = 0x60b41e1; continue;
 }
 0x60b41e1 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b41fb; continue; }
  pc = 0x60b41f6; continue;
 }
 0x60b41f6 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x60b41fb; continue;
 }
 0x60b41fb => {
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
  cpu.push(0x60b4228); f_060acc60(cpu); if cpu.returned_to != 0x60b4228 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b4237; continue; }
  pc = 0x60b4230; continue;
 }
 0x60b4230 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b4237 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x62d1f0cu32; cpu.edi = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x62d1f0cu32; cpu.eax = v; }
  { cpu.movs(4, true); }
  pc = 0x60b424e; continue;
 }
 0x60b424e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4260(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4260); }
 let mut pc: u32 = 0x60b4260;
 loop { match pc {
 0x60b4260 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b4278; continue; }
  pc = 0x60b426e; continue;
 }
 0x60b426e => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.push(0x60b4275); cpu.call(t); if cpu.returned_to != 0x60b4275 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b4278; continue;
 }
 0x60b4278 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b4289; continue; }
  pc = 0x60b4280; continue;
 }
 0x60b4280 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b4286); f_060ac030(cpu); if cpu.returned_to != 0x60b4286 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b4289; continue;
 }
 0x60b4289 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b42b2; continue; }
  pc = 0x60b428f; continue;
 }
 0x60b428f => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b42a0; continue; }
  pc = 0x60b4297; continue;
 }
 0x60b4297 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b429d); f_060acdb0(cpu); if cpu.returned_to != 0x60b429d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b42a0; continue;
 }
 0x60b42a0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b42af); f_060acc60(cpu); if cpu.returned_to != 0x60b42af { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60b42b2; continue;
 }
 0x60b42b2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x60b42c0); f_060acf30(cpu); if cpu.returned_to != 0x60b42c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b42d3); f_060b3200(cpu); if cpu.returned_to != 0x60b42d3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b42dc); f_060b32d0(cpu); if cpu.returned_to != 0x60b42dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b42e1; continue;
 }
 0x60b42e1 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x62d1d60u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x60b42f1; continue; }
  pc = 0x60b42e9; continue;
 }
 0x60b42e9 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x62d1d60u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b42f1; continue;
 }
 0x60b42f1 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x40u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b42e1; continue; }
  pc = 0x60b42f9; continue;
 }
 0x60b42f9 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4300); }
 let mut pc: u32 = 0x60b4300;
 loop { match pc {
 0x60b4300 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = 0x2cu32; cpu.push(v); }
  cpu.push(0x60b430f); f_060b3300(cpu); if cpu.returned_to != 0x60b430f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b4335; continue; }
  pc = 0x60b4318; continue;
 }
 0x60b4318 => {
  { let v: u32 = 0x616ac54u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b4324; continue;
 }
 0x60b4324 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4324; continue; }
  pc = 0x60b432e; continue;
 }
 0x60b432e => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b4335 => {
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
  cpu.push(0x60b434f); f_060b3240(cpu); if cpu.returned_to != 0x60b434f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b437c; continue; }
  pc = 0x60b4356; continue;
 }
 0x60b4356 => {
  { let v: u32 = 0x616ac80u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b4362; continue;
 }
 0x60b4362 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4362; continue; }
  pc = 0x60b436c; continue;
 }
 0x60b436c => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b4372); f_060b32d0(cpu); if cpu.returned_to != 0x60b4372 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b437c => {
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
  { let v: u32 = 0x616ac9cu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b43a9; continue;
 }
 0x60b43a9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b43b7; continue; }
  pc = 0x60b43b1; continue;
 }
 0x60b43b1 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b43b7; continue; }
  pc = 0x60b43b5; continue;
 }
 0x60b43b5 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b43b7; continue;
 }
 0x60b43b7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b43c4; continue; }
  pc = 0x60b43bc; continue;
 }
 0x60b43bc => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b43c4; continue; }
  pc = 0x60b43c1; continue;
 }
 0x60b43c1 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b43c4; continue;
 }
 0x60b43c4 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b43d3; continue; }
  pc = 0x60b43c8; continue;
 }
 0x60b43c8 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b43d3; continue; }
  pc = 0x60b43cc; continue;
 }
 0x60b43cc => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b43d3; continue; }
  pc = 0x60b43cf; continue;
 }
 0x60b43cf => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b43a9; continue;
 }
 0x60b43d3 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b43da; continue; }
  pc = 0x60b43d8; continue;
 }
 0x60b43d8 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b43da; continue;
 }
 0x60b43da => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b43ea; continue; }
  pc = 0x60b43de; continue;
 }
 0x60b43de => {
  { let v: u32 = 0x0u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b446e; continue;
 }
 0x60b43ea => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x616aca4u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  pc = 0x60b43f7; continue;
 }
 0x60b43f7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b4405; continue; }
  pc = 0x60b43ff; continue;
 }
 0x60b43ff => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4405; continue; }
  pc = 0x60b4403; continue;
 }
 0x60b4403 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b4405; continue;
 }
 0x60b4405 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b4412; continue; }
  pc = 0x60b440a; continue;
 }
 0x60b440a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4412; continue; }
  pc = 0x60b440f; continue;
 }
 0x60b440f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b4412; continue;
 }
 0x60b4412 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b4421; continue; }
  pc = 0x60b4416; continue;
 }
 0x60b4416 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b4421; continue; }
  pc = 0x60b441a; continue;
 }
 0x60b441a => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b4421; continue; }
  pc = 0x60b441d; continue;
 }
 0x60b441d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b43f7; continue;
 }
 0x60b4421 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b4428; continue; }
  pc = 0x60b4426; continue;
 }
 0x60b4426 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b4428; continue;
 }
 0x60b4428 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60b4467; continue; }
  pc = 0x60b442c; continue;
 }
 0x60b442c => {
  { let v: u32 = 0x616acacu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b4438; continue;
 }
 0x60b4438 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4438; continue; }
  pc = 0x60b4442; continue;
 }
 0x60b4442 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b4454); f_060b3200(cpu); if cpu.returned_to != 0x60b4454 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b445d); f_060b32d0(cpu); if cpu.returned_to != 0x60b445d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b4467 => {
  { let v: u32 = 0x1u32; cpu.w32(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b446e; continue;
 }
 0x60b446e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b4472; continue;
 }
 0x60b4472 => {
  { cpu.sub32(cpu.r32(0x62d1d60u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x60b448a; continue; }
  pc = 0x60b447b; continue;
 }
 0x60b447b => {
  { let v: u32 = cpu.ebp; cpu.w32(0x62d1d60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x32u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b4493; continue;
 }
 0x60b448a => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x10u32); }
  if cpu.cf { cpu.poll(); pc = 0x60b4472; continue; }
  pc = 0x60b4493; continue;
 }
 0x60b4493 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x60b44d2; continue; }
  pc = 0x60b4498; continue;
 }
 0x60b4498 => {
  { let v: u32 = 0x616acc4u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b44a4; continue;
 }
 0x60b44a4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b44a4; continue; }
  pc = 0x60b44ae; continue;
 }
 0x60b44ae => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b44bf); f_060b3200(cpu); if cpu.returned_to != 0x60b44bf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b44c8); f_060b32d0(cpu); if cpu.returned_to != 0x60b44c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b44d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x30u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.ebp), v); }
  { let v = 0x66u32; cpu.push(v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60b44f0); f_060acff0(cpu); if cpu.returned_to != 0x60b44f0 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x36u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u16 = cpu.r16(0x34u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x60b4510); f_060acf30(cpu); if cpu.returned_to != 0x60b4510 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x300u32; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x60b4530); f_060acc60(cpu); if cpu.returned_to != 0x60b4530 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x2eu32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b454b; continue; }
  pc = 0x60b453f; continue;
 }
 0x60b453f => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b45d7; continue;
 }
 0x60b454b => {
  { let v = 0x60b4bd0u32; cpu.push(v); }
  cpu.push(0x60b4555); f_060ac770(cpu); if cpu.returned_to != 0x60b4555 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b45ab; continue; }
  pc = 0x60b4560; continue;
 }
 0x60b4560 => {
  { let v: u32 = 0x616acdcu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b456c; continue;
 }
 0x60b456c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b456c; continue; }
  pc = 0x60b4576; continue;
 }
 0x60b4576 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x36u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x66u32; cpu.push(v); }
  cpu.push(0x60b4584); f_060acf30(cpu); if cpu.returned_to != 0x60b4584 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b4598); f_060b3200(cpu); if cpu.returned_to != 0x60b4598 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b45a1); f_060b32d0(cpu); if cpu.returned_to != 0x60b45a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b45ab => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b45b2); f_060ac660(cpu); if cpu.returned_to != 0x60b45b2 { return; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b45c8); f_060ac4e0(cpu); if cpu.returned_to != 0x60b45c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b45d4); f_060ac290(cpu); if cpu.returned_to != 0x60b45d4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b45d7; continue;
 }
 0x60b45d7 => {
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
pub fn f_060b45e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b45e0); }
 let mut pc: u32 = 0x60b45e0;
 loop { match pc {
 0x60b45e0 => {
  { let v: u32 = 0x62d1da0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b45f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b45f0); }
 let mut pc: u32 = 0x60b45f0;
 loop { match pc {
 0x60b45f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d1ea0u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1ea0u32.wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4610); }
 let mut pc: u32 = 0x60b4610;
 loop { match pc {
 0x60b4610 => {
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
  cpu.push(0x60b4672); f_060b3ddb(cpu); if cpu.returned_to != 0x60b4672 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b46ea; continue; }
  pc = 0x60b4681; continue;
 }
 0x60b4681 => {
  { cpu.sub8(cpu.r8(0x617c528u32), 0x0u8); }
  if !cpu.zf { pc = 0x60b4691; continue; }
  pc = 0x60b468a; continue;
 }
 0x60b468a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b4a9d; continue;
 }
 0x60b4691 => {
  { let v: u32 = 0x617c528u32; cpu.esi = v; }
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b469f; continue;
 }
 0x60b469f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b469f; continue; }
  pc = 0x60b46a9; continue;
 }
 0x60b46a9 => {
  { let v: u32 = 0x224u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b46bf; continue;
 }
 0x60b46bf => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b46bf; continue; }
  pc = 0x60b46c5; continue;
 }
 0x60b46c5 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x228u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b46d4); f_060b3ddb(cpu); if cpu.returned_to != 0x60b46d4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x374u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b46ea; continue; }
  pc = 0x60b46e3; continue;
 }
 0x60b46e3 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b4a9d; continue;
 }
 0x60b46ea => {
  { let v: u32 = 0x10cu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b46ff; continue;
 }
 0x60b46ff => {
  { let v: u32 = 0x50u32; cpu.ecx = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x60b471e); f_060b4ab0(cpu); if cpu.returned_to != 0x60b471e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b4a2a; continue; }
  pc = 0x60b4726; continue;
 }
 0x60b4726 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x60b4758; continue; }
  pc = 0x60b4740; continue;
 }
 0x60b4740 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x60b4758; continue; }
  pc = 0x60b474a; continue;
 }
 0x60b474a => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ (cpu.ebx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x325u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b4740; continue; }
  pc = 0x60b4758; continue;
 }
 0x60b4758 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60b4767; continue;
 }
 0x60b475c => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x60b477e; continue; }
  pc = 0x60b4766; continue;
 }
 0x60b4766 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60b4767; continue;
 }
 0x60b4767 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x60b475c; continue; }
  pc = 0x60b477e; continue;
 }
 0x60b477e => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  pc = 0x60b4794; continue;
 }
 0x60b4789 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if cpu.zf { pc = 0x60b47ab; continue; }
  pc = 0x60b4793; continue;
 }
 0x60b4793 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60b4794; continue;
 }
 0x60b4794 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x60b4789; continue; }
  pc = 0x60b47ab; continue;
 }
 0x60b47ab => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.esi = v; }
  pc = 0x60b47c2; continue;
 }
 0x60b47b7 => {
  { cpu.sub8(cpu.r8(0x324u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1))), 0x20u8); }
  if !cpu.zf { pc = 0x60b47d9; continue; }
  pc = 0x60b47c1; continue;
 }
 0x60b47c1 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60b47c2; continue;
 }
 0x60b47c2 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x60b47b7; continue; }
  pc = 0x60b47d9; continue;
 }
 0x60b47d9 => {
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0x324u32.wrapping_add(cpu.esp); cpu.ebp = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.cf { cpu.poll(); pc = 0x60b46ff; continue; }
  pc = 0x60b47fd; continue;
 }
 0x60b47fd => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x3bu8); }
  if cpu.zf { cpu.poll(); pc = 0x60b46ff; continue; }
  pc = 0x60b4809; continue;
 }
 0x60b4809 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x616ac08u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b4815; continue;
 }
 0x60b4815 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b4823; continue; }
  pc = 0x60b481d; continue;
 }
 0x60b481d => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4823; continue; }
  pc = 0x60b4821; continue;
 }
 0x60b4821 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b4823; continue;
 }
 0x60b4823 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b4830; continue; }
  pc = 0x60b4828; continue;
 }
 0x60b4828 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4830; continue; }
  pc = 0x60b482d; continue;
 }
 0x60b482d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b4830; continue;
 }
 0x60b4830 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b483f; continue; }
  pc = 0x60b4834; continue;
 }
 0x60b4834 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b483f; continue; }
  pc = 0x60b4838; continue;
 }
 0x60b4838 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b483f; continue; }
  pc = 0x60b483b; continue;
 }
 0x60b483b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b4815; continue;
 }
 0x60b483f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b4846; continue; }
  pc = 0x60b4844; continue;
 }
 0x60b4844 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b4846; continue;
 }
 0x60b4846 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b4864; continue; }
  pc = 0x60b484a; continue;
 }
 0x60b484a => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b4855; continue;
 }
 0x60b4855 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4855; continue; }
  pc = 0x60b485f; continue;
 }
 0x60b485f => {
  cpu.poll(); pc = 0x60b46ff; continue;
 }
 0x60b4864 => {
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { let v: u32 = 0x616ac10u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b4870; continue;
 }
 0x60b4870 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b487e; continue; }
  pc = 0x60b4878; continue;
 }
 0x60b4878 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b487e; continue; }
  pc = 0x60b487c; continue;
 }
 0x60b487c => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b487e; continue;
 }
 0x60b487e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b488b; continue; }
  pc = 0x60b4883; continue;
 }
 0x60b4883 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b488b; continue; }
  pc = 0x60b4888; continue;
 }
 0x60b4888 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b488b; continue;
 }
 0x60b488b => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b489a; continue; }
  pc = 0x60b488f; continue;
 }
 0x60b488f => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b489a; continue; }
  pc = 0x60b4893; continue;
 }
 0x60b4893 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b489a; continue; }
  pc = 0x60b4896; continue;
 }
 0x60b4896 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b4870; continue;
 }
 0x60b489a => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b48a1; continue; }
  pc = 0x60b489f; continue;
 }
 0x60b489f => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b48a1; continue;
 }
 0x60b48a1 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b48ba; continue; }
  pc = 0x60b48a5; continue;
 }
 0x60b48a5 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b48ab; continue;
 }
 0x60b48ab => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b48ab; continue; }
  pc = 0x60b48b5; continue;
 }
 0x60b48b5 => {
  cpu.poll(); pc = 0x60b46ff; continue;
 }
 0x60b48ba => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x616ac18u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b48c6; continue;
 }
 0x60b48c6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b48d4; continue; }
  pc = 0x60b48ce; continue;
 }
 0x60b48ce => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b48d4; continue; }
  pc = 0x60b48d2; continue;
 }
 0x60b48d2 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b48d4; continue;
 }
 0x60b48d4 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b48e1; continue; }
  pc = 0x60b48d9; continue;
 }
 0x60b48d9 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b48e1; continue; }
  pc = 0x60b48de; continue;
 }
 0x60b48de => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b48e1; continue;
 }
 0x60b48e1 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b48f0; continue; }
  pc = 0x60b48e5; continue;
 }
 0x60b48e5 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b48f0; continue; }
  pc = 0x60b48e9; continue;
 }
 0x60b48e9 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b48f0; continue; }
  pc = 0x60b48ec; continue;
 }
 0x60b48ec => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b48c6; continue;
 }
 0x60b48f0 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b48f7; continue; }
  pc = 0x60b48f5; continue;
 }
 0x60b48f5 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b48f7; continue;
 }
 0x60b48f7 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b4915; continue; }
  pc = 0x60b48fb; continue;
 }
 0x60b48fb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x10u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b4905); f_060b4b30(cpu); if cpu.returned_to != 0x60b4905 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x100u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b46ff; continue;
 }
 0x60b4915 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616ac20u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b4921; continue;
 }
 0x60b4921 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b492f; continue; }
  pc = 0x60b4929; continue;
 }
 0x60b4929 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b492f; continue; }
  pc = 0x60b492d; continue;
 }
 0x60b492d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b492f; continue;
 }
 0x60b492f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b493c; continue; }
  pc = 0x60b4934; continue;
 }
 0x60b4934 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b493c; continue; }
  pc = 0x60b4939; continue;
 }
 0x60b4939 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b493c; continue;
 }
 0x60b493c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b494b; continue; }
  pc = 0x60b4940; continue;
 }
 0x60b4940 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b494b; continue; }
  pc = 0x60b4944; continue;
 }
 0x60b4944 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b494b; continue; }
  pc = 0x60b4947; continue;
 }
 0x60b4947 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b4921; continue;
 }
 0x60b494b => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b4952; continue; }
  pc = 0x60b4950; continue;
 }
 0x60b4950 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b4952; continue;
 }
 0x60b4952 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b4970; continue; }
  pc = 0x60b4956; continue;
 }
 0x60b4956 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b4960); f_060b4b30(cpu); if cpu.returned_to != 0x60b4960 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x102u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b46ff; continue;
 }
 0x60b4970 => {
  { let v: u32 = 0xau32; cpu.ecx = v; }
  { let v: u32 = 0x616ac24u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b497c; continue;
 }
 0x60b497c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b498a; continue; }
  pc = 0x60b4984; continue;
 }
 0x60b4984 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b498a; continue; }
  pc = 0x60b4988; continue;
 }
 0x60b4988 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b498a; continue;
 }
 0x60b498a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b4997; continue; }
  pc = 0x60b498f; continue;
 }
 0x60b498f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4997; continue; }
  pc = 0x60b4994; continue;
 }
 0x60b4994 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b4997; continue;
 }
 0x60b4997 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b49a6; continue; }
  pc = 0x60b499b; continue;
 }
 0x60b499b => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b49a6; continue; }
  pc = 0x60b499f; continue;
 }
 0x60b499f => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b49a6; continue; }
  pc = 0x60b49a2; continue;
 }
 0x60b49a2 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b497c; continue;
 }
 0x60b49a6 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b49ad; continue; }
  pc = 0x60b49ab; continue;
 }
 0x60b49ab => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b49ad; continue;
 }
 0x60b49ad => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b49cb; continue; }
  pc = 0x60b49b1; continue;
 }
 0x60b49b1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b49bb); f_060b4b30(cpu); if cpu.returned_to != 0x60b49bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x104u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b46ff; continue;
 }
 0x60b49cb => {
  { let v: u32 = 0xbu32; cpu.ecx = v; }
  { let v: u32 = 0x616ac30u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b49d7; continue;
 }
 0x60b49d7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b49e5; continue; }
  pc = 0x60b49df; continue;
 }
 0x60b49df => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b49e5; continue; }
  pc = 0x60b49e3; continue;
 }
 0x60b49e3 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b49e5; continue;
 }
 0x60b49e5 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b49f2; continue; }
  pc = 0x60b49ea; continue;
 }
 0x60b49ea => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b49f2; continue; }
  pc = 0x60b49ef; continue;
 }
 0x60b49ef => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b49f2; continue;
 }
 0x60b49f2 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b4a01; continue; }
  pc = 0x60b49f6; continue;
 }
 0x60b49f6 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b4a01; continue; }
  pc = 0x60b49fa; continue;
 }
 0x60b49fa => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b4a01; continue; }
  pc = 0x60b49fd; continue;
 }
 0x60b49fd => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b49d7; continue;
 }
 0x60b4a01 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b4a08; continue; }
  pc = 0x60b4a06; continue;
 }
 0x60b4a06 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b4a08; continue;
 }
 0x60b4a08 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60b46ff; continue; }
  pc = 0x60b4a10; continue;
 }
 0x60b4a10 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b4a1a); f_060b4b30(cpu); if cpu.returned_to != 0x60b4a1a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x106u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60b46ff; continue;
 }
 0x60b4a2a => {
  { let v: u32 = cpu.r32(0x374u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b4a37); f_060b3f05(cpu); if cpu.returned_to != 0x60b4a37 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b4a63; continue; }
  pc = 0x60b4a51; continue;
 }
 0x60b4a51 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60b4a88; continue; }
  pc = 0x60b4a63; continue;
 }
 0x60b4a63 => {
  { let v: u32 = 0x616ac3cu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b4a6f; continue;
 }
 0x60b4a6f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b4a6f; continue; }
  pc = 0x60b4a79; continue;
 }
 0x60b4a79 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x378u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b4a88 => {
  { let v: u32 = 0x46u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x60b4a9d; continue;
 }
 0x60b4a9d => {
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
pub fn f_060b4ab0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4ab0); }
 let mut pc: u32 = 0x60b4ab0;
 loop { match pc {
 0x60b4ab0 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x60b4b17; continue; }
  pc = 0x60b4ac5; continue;
 }
 0x60b4ac5 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x60b4aef; continue; }
  pc = 0x60b4acb; continue;
 }
 0x60b4acb => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b4ae5); f_060b3e51(cpu); if cpu.returned_to != 0x60b4ae5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b4b17; continue; }
  pc = 0x60b4aef; continue;
 }
 0x60b4aef => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xdu8); }
  if cpu.zf { pc = 0x60b4b10; continue; }
  pc = 0x60b4b00; continue;
 }
 0x60b4b00 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0xau8); }
  if !cpu.zf { pc = 0x60b4b0c; continue; }
  pc = 0x60b4b06; continue;
 }
 0x60b4b06 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b4b17; continue;
 }
 0x60b4b0c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0xffffffffu32.wrapping_add(cpu.edi), v); }
  pc = 0x60b4b10; continue;
 }
 0x60b4b10 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.cf { cpu.poll(); pc = 0x60b4ac5; continue; }
  pc = 0x60b4b17; continue;
 }
 0x60b4b17 => {
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
pub fn f_060b4b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4b30); }
 let mut pc: u32 = 0x60b4b30;
 loop { match pc {
 0x60b4b30 => {
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
  pc = 0x60b4b98; continue;
 }
 0x60b4b51 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b4b61; continue; }
  pc = 0x60b4b59; continue;
 }
 0x60b4b59 => {
  { cpu.sub8((cpu.edx as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4b61; continue; }
  pc = 0x60b4b5e; continue;
 }
 0x60b4b5e => {
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x20u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60b4b61; continue;
 }
 0x60b4b61 => {
  { cpu.sub8((cpu.edx as u8), 0x2du8); }
  if !cpu.zf { pc = 0x60b4b6b; continue; }
  pc = 0x60b4b66; continue;
 }
 0x60b4b66 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b4b96; continue;
 }
 0x60b4b6b => {
  { cpu.sub8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x60b4b96; continue; }
  pc = 0x60b4b70; continue;
 }
 0x60b4b70 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b4b92; continue; }
  pc = 0x60b4b76; continue;
 }
 0x60b4b76 => {
  { cpu.sub8((cpu.edx as u8), cpu.r8(0x616abf4u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x60b4b8d; continue; }
  pc = 0x60b4b7e; continue;
 }
 0x60b4b7e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b4b92; continue;
 }
 0x60b4b8d => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b4b76; continue; }
  pc = 0x60b4b92; continue;
 }
 0x60b4b92 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x60b4bac; continue; }
  pc = 0x60b4b96; continue;
 }
 0x60b4b96 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x60b4b98; continue;
 }
 0x60b4b98 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if cpu.cf { cpu.poll(); pc = 0x60b4b51; continue; }
  pc = 0x60b4bac; continue;
 }
 0x60b4bac => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b4bbd; continue; }
  pc = 0x60b4bb4; continue;
 }
 0x60b4bb4 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b4bbd; continue;
 }
 0x60b4bbd => {
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
pub fn f_060b4bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4bd0); }
 let mut pc: u32 = 0x60b4bd0;
 loop { match pc {
 0x60b4bd0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c628u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b4c1c; continue; }
  pc = 0x60b4bdb; continue;
 }
 0x60b4bdb => {
  { let v: u32 = 0x1u32; cpu.w32(0x617c628u32, v); }
  cpu.push(0x60b4bea); f_060acda0(cpu); if cpu.returned_to != 0x60b4bea { return; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x302u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f08u32, v); }
  cpu.push(0x60b4c02); f_060acc60(cpu); if cpu.returned_to != 0x60b4c02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d1f08u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.push(0x60b4c13); f_060acd90(cpu); if cpu.returned_to != 0x60b4c13 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x617c628u32, v); }
  pc = 0x60b4c1c; continue;
 }
 0x60b4c1c => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4c30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4c30); }
 let mut pc: u32 = 0x60b4c30;
 loop { match pc {
 0x60b4c30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c62cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60b4cb6; continue; }
  pc = 0x60b4c3c; continue;
 }
 0x60b4c3c => {
  { let v = 0x60b3f40u32; cpu.push(v); }
  { let v = 0x60b4c30u32; cpu.push(v); }
  cpu.push(0x60b4c4b); f_060b3160(cpu); if cpu.returned_to != 0x60b4c4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0x62d1d60u32; cpu.push(v); }
  cpu.push(0x60b4c5a); f_060b30a0(cpu); if cpu.returned_to != 0x60b4c5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x50u32; cpu.push(v); }
  { let v = 0x62d1ea0u32; cpu.push(v); }
  cpu.push(0x60b4c69); f_060b30a0(cpu); if cpu.returned_to != 0x60b4c69 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x100u32; cpu.push(v); }
  { let v = 0x62d1da0u32; cpu.push(v); }
  cpu.push(0x60b4c7b); f_060b30a0(cpu); if cpu.returned_to != 0x60b4c7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x18u32; cpu.push(v); }
  { let v = 0x62d1ef0u32; cpu.push(v); }
  cpu.push(0x60b4c8a); f_060b30a0(cpu); if cpu.returned_to != 0x60b4c8a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c628u32; cpu.push(v); }
  cpu.push(0x60b4c99); f_060b30a0(cpu); if cpu.returned_to != 0x60b4c99 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f08u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60b4cad); f_060b30a0(cpu); if cpu.returned_to != 0x60b4cad { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c62cu32, v); }
  pc = 0x60b4cb6; continue;
 }
 0x60b4cb6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4cc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4cc0); }
 let mut pc: u32 = 0x60b4cc0;
 loop { match pc {
 0x60b4cc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c6b4u32), 0x0u32); }
  if cpu.zf { pc = 0x60b4d74; continue; }
  pc = 0x60b4cd0; continue;
 }
 0x60b4cd0 => {
  { let v = 0x60b4cc0u32; cpu.push(v); }
  { let v = 0x60b6a40u32; cpu.push(v); }
  cpu.push(0x60b4cdf); f_060b30c0(cpu); if cpu.returned_to != 0x60b4cdf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x617c630u32; cpu.push(v); }
  cpu.push(0x60b4cf1); f_060b3080(cpu); if cpu.returned_to != 0x60b4cf1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c6b0u32; cpu.push(v); }
  cpu.push(0x60b4d00); f_060b3080(cpu); if cpu.returned_to != 0x60b4d00 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f38u32; cpu.push(v); }
  cpu.push(0x60b4d0f); f_060b3080(cpu); if cpu.returned_to != 0x60b4d0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x62d1f24u32; cpu.push(v); }
  cpu.push(0x60b4d1e); f_060b3080(cpu); if cpu.returned_to != 0x60b4d1e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f40u32; cpu.push(v); }
  cpu.push(0x60b4d2d); f_060b3080(cpu); if cpu.returned_to != 0x60b4d2d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f44u32; cpu.push(v); }
  cpu.push(0x60b4d3c); f_060b3080(cpu); if cpu.returned_to != 0x60b4d3c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f34u32; cpu.push(v); }
  cpu.push(0x60b4d4b); f_060b3080(cpu); if cpu.returned_to != 0x60b4d4b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f3cu32; cpu.push(v); }
  cpu.push(0x60b4d5a); f_060b3080(cpu); if cpu.returned_to != 0x60b4d5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f30u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60b4d6b); f_060b3080(cpu); if cpu.returned_to != 0x60b4d6b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b4u32, v); }
  pc = 0x60b4d74; continue;
 }
 0x60b4d74 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4d80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4d80); }
 let mut pc: u32 = 0x60b4d80;
 loop { match pc {
 0x60b4d80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b4d96; continue; }
  pc = 0x60b4d91; continue;
 }
 0x60b4d91 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b4d96; continue;
 }
 0x60b4d96 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b4d9f; continue; }
  pc = 0x60b4d9a; continue;
 }
 0x60b4d9a => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b4d9f; continue;
 }
 0x60b4d9f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60b4daf; continue; }
  pc = 0x60b4da6; continue;
 }
 0x60b4da6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b4daf; continue;
 }
 0x60b4daf => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b4dbc; continue; }
  pc = 0x60b4db7; continue;
 }
 0x60b4db7 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b4dbc; continue;
 }
 0x60b4dbc => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4f20); }
 let mut pc: u32 = 0x60b4f20;
 loop { match pc {
 0x60b4f20 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.esp; cpu.push(v); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b4f60; continue; }
  pc = 0x60b4f45; continue;
 }
 0x60b4f45 => {
  cpu.poll(); pc = cpu.r32(0x60b4f20u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x60b4f4d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x60b4f60; continue;
 }
 0x60b4f54 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  pc = 0x60b4f60; continue;
 }
 0x60b4f5b => {
  { let v: u32 = 0x4u32; cpu.edx = v; }
  pc = 0x60b4f60; continue;
 }
 0x60b4f60 => {
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
  { let v: u32 = cpu.r32(0x62d1eb8u32); cpu.edx = v; }
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
  if cpu.zf { pc = 0x60b4fb7; continue; }
  pc = 0x60b4fa2; continue;
 }
 0x60b4fa2 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi.wrapping_mul(2)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x60b4fb7; continue; }
  pc = 0x60b4fad; continue;
 }
 0x60b4fad => {
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if cpu.zf { pc = 0x60b4fb7; continue; }
  pc = 0x60b4fb4; continue;
 }
 0x60b4fb4 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  pc = 0x60b4fb7; continue;
 }
 0x60b4fb7 => {
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
pub fn f_060b4fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4fd0); }
 let mut pc: u32 = 0x60b4fd0;
 loop { match pc {
 0x60b4fd0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b4fe0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b4fe0); }
 let mut pc: u32 = 0x60b4fe0;
 loop { match pc {
 0x60b4fe0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b501b; continue; }
  pc = 0x60b4ff7; continue;
 }
 0x60b4ff7 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b4ff9; continue;
 }
 0x60b4ff9 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x60b500d; continue; }
  pc = 0x60b5004; continue;
 }
 0x60b5004 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b500a); f_060b65b0(cpu); if cpu.returned_to != 0x60b500a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b500d; continue;
 }
 0x60b500d => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8a8u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b4ff9; continue; }
  pc = 0x60b501b; continue;
 }
 0x60b501b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5020); }
 let mut pc: u32 = 0x60b5020;
 loop { match pc {
 0x60b5020 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b502b; continue; }
  pc = 0x60b5028; continue;
 }
 0x60b5028 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b502b => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x868u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5040); }
 let mut pc: u32 = 0x60b5040;
 loop { match pc {
 0x60b5040 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b505b; continue; }
  pc = 0x60b5048; continue;
 }
 0x60b5048 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x868u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b505b; continue;
 }
 0x60b505b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5060); }
 let mut pc: u32 = 0x60b5060;
 loop { match pc {
 0x60b5060 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b507b; continue; }
  pc = 0x60b5069; continue;
 }
 0x60b5069 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60b507b; continue;
 }
 0x60b507b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b50c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b50c0); }
 let mut pc: u32 = 0x60b50c0;
 loop { match pc {
 0x60b50c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b5106; continue; }
  pc = 0x60b50ca; continue;
 }
 0x60b50ca => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60b5106; continue; }
  pc = 0x60b50d2; continue;
 }
 0x60b50d2 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x60b5106; continue; }
  pc = 0x60b50d7; continue;
 }
 0x60b50d7 => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b50f6; continue; }
  pc = 0x60b50ef; continue;
 }
 0x60b50ef => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x60b50f6); cpu.call(t); if cpu.returned_to != 0x60b50f6 { return; }
  pc = 0x60b50f6; continue;
 }
 0x60b50f6 => {
  { cpu.sub32(cpu.r32(0x864u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b5106; continue; }
  pc = 0x60b50ff; continue;
 }
 0x60b50ff => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x60b5106); cpu.call(t); if cpu.returned_to != 0x60b5106 { return; }
  pc = 0x60b5106; continue;
 }
 0x60b5106 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5110(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5110); }
 let mut pc: u32 = 0x60b5110;
 loop { match pc {
 0x60b5110 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b5164; continue; }
  pc = 0x60b511d; continue;
 }
 0x60b511d => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x8u32); }
  if !cpu.zf { pc = 0x60b5164; continue; }
  pc = 0x60b5123; continue;
 }
 0x60b5123 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60b5164; continue; }
  pc = 0x60b5134; continue;
 }
 0x60b5134 => {
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
  cpu.push(0x60b515a); f_060acc60(cpu); if cpu.returned_to != 0x60b515a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b5164; continue;
 }
 0x60b5164 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5190); }
 let mut pc: u32 = 0x60b5190;
 loop { match pc {
 0x60b5190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b520a; continue; }
  pc = 0x60b519e; continue;
 }
 0x60b519e => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x60b520a; continue; }
  pc = 0x60b51a4; continue;
 }
 0x60b51a4 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b520a; continue; }
  pc = 0x60b51b2; continue;
 }
 0x60b51b2 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b520a; continue; }
  pc = 0x60b51b8; continue;
 }
 0x60b51b8 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60b520a; continue; }
  pc = 0x60b51da; continue;
 }
 0x60b51da => {
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
  cpu.push(0x60b5200); f_060acc60(cpu); if cpu.returned_to != 0x60b5200 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b520a; continue;
 }
 0x60b520a => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5220); }
 let mut pc: u32 = 0x60b5220;
 loop { match pc {
 0x60b5220 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5236; continue; }
  pc = 0x60b5228; continue;
 }
 0x60b5228 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x40u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b5236; continue;
 }
 0x60b5236 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5240); }
 let mut pc: u32 = 0x60b5240;
 loop { match pc {
 0x60b5240 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b524f; continue; }
  pc = 0x60b5248; continue;
 }
 0x60b5248 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b524f; continue;
 }
 0x60b524f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5250(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5250); }
 let mut pc: u32 = 0x60b5250;
 loop { match pc {
 0x60b5250 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b525e; continue; }
  pc = 0x60b5258; continue;
 }
 0x60b5258 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b525e => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b52b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b52b0); }
 let mut pc: u32 = 0x60b52b0;
 loop { match pc {
 0x60b52b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b52cd; continue; }
  pc = 0x60b52bc; continue;
 }
 0x60b52bc => {
  { cpu.sub32(cpu.edx, cpu.r32(0x54u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b52cd; continue; }
  pc = 0x60b52c1; continue;
 }
 0x60b52c1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b52ca); f_060b65b0(cpu); if cpu.returned_to != 0x60b52ca { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b52cd; continue;
 }
 0x60b52cd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b52d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b52d0); }
 let mut pc: u32 = 0x60b52d0;
 loop { match pc {
 0x60b52d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b52d9; continue; }
  pc = 0x60b52d8; continue;
 }
 0x60b52d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b52d9 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b52e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b52e0); }
 let mut pc: u32 = 0x60b52e0;
 loop { match pc {
 0x60b52e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b52fd; continue; }
  pc = 0x60b52ec; continue;
 }
 0x60b52ec => {
  { cpu.sub32(cpu.edx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b52fd; continue; }
  pc = 0x60b52f1; continue;
 }
 0x60b52f1 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b52fa); f_060b65b0(cpu); if cpu.returned_to != 0x60b52fa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b52fd; continue;
 }
 0x60b52fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5300); }
 let mut pc: u32 = 0x60b5300;
 loop { match pc {
 0x60b5300 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5309; continue; }
  pc = 0x60b5308; continue;
 }
 0x60b5308 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5309 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5310); }
 let mut pc: u32 = 0x60b5310;
 loop { match pc {
 0x60b5310 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b531f; continue; }
  pc = 0x60b5318; continue;
 }
 0x60b5318 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b531f; continue;
 }
 0x60b531f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5320); }
 let mut pc: u32 = 0x60b5320;
 loop { match pc {
 0x60b5320 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5329; continue; }
  pc = 0x60b5328; continue;
 }
 0x60b5328 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5329 => {
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5330); }
 let mut pc: u32 = 0x60b5330;
 loop { match pc {
 0x60b5330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b535e; continue; }
  pc = 0x60b533d; continue;
 }
 0x60b533d => {
  { cpu.sub32(cpu.edx, cpu.r32(0x44u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x60b534b; continue; }
  pc = 0x60b5342; continue;
 }
 0x60b5342 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x48u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b535e; continue; }
  pc = 0x60b534b; continue;
 }
 0x60b534b => {
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b535b); f_060b65b0(cpu); if cpu.returned_to != 0x60b535b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b535e; continue;
 }
 0x60b535e => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5360(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5360); }
 let mut pc: u32 = 0x60b5360;
 loop { match pc {
 0x60b5360 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5384; continue; }
  pc = 0x60b5368; continue;
 }
 0x60b5368 => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b5384; continue;
 }
 0x60b5384 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5390); }
 let mut pc: u32 = 0x60b5390;
 loop { match pc {
 0x60b5390 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5399; continue; }
  pc = 0x60b5398; continue;
 }
 0x60b5398 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5399 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b53a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b53a0); }
 let mut pc: u32 = 0x60b53a0;
 loop { match pc {
 0x60b53a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5484; continue; }
  pc = 0x60b53ad; continue;
 }
 0x60b53ad => {
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
  { let v: u32 = cpu.r32(0x62d1eb4u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x48u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x50u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x2b11u32; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b5443; continue; }
  pc = 0x60b543e; continue;
 }
 0x60b543e => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if !cpu.zf { pc = 0x60b544c; continue; }
  pc = 0x60b5443; continue;
 }
 0x60b5443 => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b5453; continue;
 }
 0x60b544c => {
  { let v: u32 = 0x40u32; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b5453; continue;
 }
 0x60b5453 => {
  { let v: u32 = 0x0u32; cpu.w32(0x85cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x860u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x864u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0x858u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b5481); f_060b65b0(cpu); if cpu.returned_to != 0x60b5481 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b5484; continue;
 }
 0x60b5484 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5490); }
 let mut pc: u32 = 0x60b5490;
 loop { match pc {
 0x60b5490 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b549f; continue; }
  pc = 0x60b5498; continue;
 }
 0x60b5498 => {
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b549f; continue;
 }
 0x60b549f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b54a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b54a0); }
 let mut pc: u32 = 0x60b54a0;
 loop { match pc {
 0x60b54a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x60b54ac); f_060ad100(cpu); if cpu.returned_to != 0x60b54ac { return; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b54cc; continue; }
  pc = 0x60b54b5; continue;
 }
 0x60b54b5 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x60b54b8; continue;
 }
 0x60b54b8 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x60b54cc; continue; }
  pc = 0x60b54be; continue;
 }
 0x60b54be => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b54b8; continue; }
  pc = 0x60b54cc; continue;
 }
 0x60b54cc => {
  { cpu.sub32(cpu.eax, cpu.r32(0x64u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x60b54f2; continue; }
  pc = 0x60b54d1; continue;
 }
 0x60b54d1 => {
  { let v: u32 = 0x616ae58u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b54dd; continue;
 }
 0x60b54dd => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b54dd; continue; }
  pc = 0x60b54e7; continue;
 }
 0x60b54e7 => {
  cpu.push(0x60b54ec); f_060ad0f0(cpu); if cpu.returned_to != 0x60b54ec { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b54f2 => {
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5503); f_060ab6b0(cpu); if cpu.returned_to != 0x60b5503 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.push(0x60b550b); f_060ad0f0(cpu); if cpu.returned_to != 0x60b550b { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5520(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5520); }
 let mut pc: u32 = 0x60b5520;
 loop { match pc {
 0x60b5520 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b5549; continue; }
  pc = 0x60b5531; continue;
 }
 0x60b5531 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x60b5534; continue;
 }
 0x60b5534 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x4u32); }
  if !cpu.zf { pc = 0x60b553b; continue; }
  pc = 0x60b553a; continue;
 }
 0x60b553a => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  pc = 0x60b553b; continue;
 }
 0x60b553b => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8a8u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b5534; continue; }
  pc = 0x60b5549; continue;
 }
 0x60b5549 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5550(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5550); }
 let mut pc: u32 = 0x60b5550;
 loop { match pc {
 0x60b5550 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b555d); f_060abcf0(cpu); if cpu.returned_to != 0x60b555d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5570(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5570); }
 let mut pc: u32 = 0x60b5570;
 loop { match pc {
 0x60b5570 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x616ae30u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5587); f_060ac870(cpu); if cpu.returned_to != 0x60b5587 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b55ab; continue; }
  pc = 0x60b558e; continue;
 }
 0x60b558e => {
  { let v: u32 = 0x616ae38u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b559a; continue;
 }
 0x60b559a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b559a; continue; }
  pc = 0x60b55a4; continue;
 }
 0x60b55a4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60b55e0; continue;
 }
 0x60b55ab => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b55c0); f_060abae0(cpu); if cpu.returned_to != 0x60b55c0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b55de; continue; }
  pc = 0x60b55d0; continue;
 }
 0x60b55d0 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b55de => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b55e0; continue;
 }
 0x60b55e0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b55f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b55f0); }
 let mut pc: u32 = 0x60b55f0;
 loop { match pc {
 0x60b55f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5601); f_060abda0(cpu); if cpu.returned_to != 0x60b5601 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5629; continue; }
  pc = 0x60b560a; continue;
 }
 0x60b560a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5615); f_060b5710(cpu); if cpu.returned_to != 0x60b5615 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5627; continue; }
  pc = 0x60b561e; continue;
 }
 0x60b561e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5624); f_060abcf0(cpu); if cpu.returned_to != 0x60b5624 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b5627; continue;
 }
 0x60b5627 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b5629; continue;
 }
 0x60b5629 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5630); }
 let mut pc: u32 = 0x60b5630;
 loop { match pc {
 0x60b5630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5648); f_060b2f10(cpu); if cpu.returned_to != 0x60b5648 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b56b6; continue; }
  pc = 0x60b5651; continue;
 }
 0x60b5651 => {
  { cpu.sub8(cpu.r8(0x617c528u32), 0x0u8); }
  if !cpu.zf { pc = 0x60b5677; continue; }
  pc = 0x60b565a; continue;
 }
 0x60b565a => {
  { let v: u32 = 0x616ae18u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b5666; continue;
 }
 0x60b5666 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5666; continue; }
  pc = 0x60b5670; continue;
 }
 0x60b5670 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b5702; continue;
 }
 0x60b5677 => {
  { let v: u32 = 0x617c528u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b5680; continue;
 }
 0x60b5680 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5680; continue; }
  pc = 0x60b568a; continue;
 }
 0x60b568a => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b569b; continue;
 }
 0x60b569b => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b569b; continue; }
  pc = 0x60b56a1; continue;
 }
 0x60b56a1 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b56ad); f_060b2f10(cpu); if cpu.returned_to != 0x60b56ad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x60b565a; continue; }
  pc = 0x60b56b6; continue;
 }
 0x60b56b6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b56c2); f_060abda0(cpu); if cpu.returned_to != 0x60b56c2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x60b56cd); f_060b32d0(cpu); if cpu.returned_to != 0x60b56cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60b56e0; continue; }
  pc = 0x60b56d4; continue;
 }
 0x60b56d4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b56e0 => {
  { let v: u32 = cpu.r32(0x114u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b56ee); f_060b5710(cpu); if cpu.returned_to != 0x60b56ee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5700; continue; }
  pc = 0x60b56f7; continue;
 }
 0x60b56f7 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b56fd); f_060abcf0(cpu); if cpu.returned_to != 0x60b56fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b5700; continue;
 }
 0x60b5700 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b5702; continue;
 }
 0x60b5702 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x100u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5710); }
 let mut pc: u32 = 0x60b5710;
 loop { match pc {
 0x60b5710 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x60b571c); f_060b6a40(cpu); if cpu.returned_to != 0x60b571c { return; }
  { let v = 0x90u32; cpu.push(v); }
  cpu.push(0x60b5726); f_060b3300(cpu); if cpu.returned_to != 0x60b5726 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b574c; continue; }
  pc = 0x60b572f; continue;
 }
 0x60b572f => {
  { let v: u32 = 0x616ad20u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b573b; continue;
 }
 0x60b573b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b573b; continue; }
  pc = 0x60b5745; continue;
 }
 0x60b5745 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b5d42; continue;
 }
 0x60b574c => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616ad48u32; cpu.esi = v; }
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
  if cpu.zf { pc = 0x60b57a3; continue; }
  pc = 0x60b5778; continue;
 }
 0x60b5778 => {
  { let v: u32 = 0x616ad50u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b5786; continue;
 }
 0x60b5786 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5786; continue; }
  pc = 0x60b5790; continue;
 }
 0x60b5790 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5796); f_060b32d0(cpu); if cpu.returned_to != 0x60b5796 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b57a3 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b57b4); f_060acc60(cpu); if cpu.returned_to != 0x60b57b4 { return; }
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
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60b5873; continue; }
  pc = 0x60b583d; continue;
 }
 0x60b583d => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5856); f_060b5df0(cpu); if cpu.returned_to != 0x60b5856 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5873; continue; }
  pc = 0x60b585d; continue;
 }
 0x60b585d => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x60b5873; continue;
 }
 0x60b5873 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b58bc; continue; }
  pc = 0x60b587a; continue;
 }
 0x60b587a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5883); f_060abeb0(cpu); if cpu.returned_to != 0x60b5883 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b58bc; continue; }
  pc = 0x60b588c; continue;
 }
 0x60b588c => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x60b58a1); f_060b5df0(cpu); if cpu.returned_to != 0x60b58a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b58bc; continue; }
  pc = 0x60b58a8; continue;
 }
 0x60b58a8 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x60b58bc; continue;
 }
 0x60b58bc => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60b5957; continue; }
  pc = 0x60b58c8; continue;
 }
 0x60b58c8 => {
  { cpu.sub32(cpu.r32(0x62d1ee4u32), 0x1u32); }
  if !cpu.zf { pc = 0x60b5957; continue; }
  pc = 0x60b58d5; continue;
 }
 0x60b58d5 => {
  { let v: u32 = cpu.edi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b5943; continue;
 }
 0x60b58df => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60b590b; continue; }
  pc = 0x60b58fd; continue;
 }
 0x60b58fd => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x60b590b; continue;
 }
 0x60b590b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5912); f_060b5df0(cpu); if cpu.returned_to != 0x60b5912 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b592f; continue; }
  pc = 0x60b5919; continue;
 }
 0x60b5919 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x60b5957; continue;
 }
 0x60b592f => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x18u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b5943; continue;
 }
 0x60b5943 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b58df; continue; }
  pc = 0x60b5957; continue;
 }
 0x60b5957 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b5987; continue; }
  pc = 0x60b595e; continue;
 }
 0x60b595e => {
  { let v: u32 = 0x616ad68u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b596a; continue;
 }
 0x60b596a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b596a; continue; }
  pc = 0x60b5974; continue;
 }
 0x60b5974 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b597a); f_060b32d0(cpu); if cpu.returned_to != 0x60b597a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5987 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { cpu.sub32(cpu.r32(0x62d1ee8u32), 0x0u32); }
  if cpu.zf { pc = 0x60b59a5; continue; }
  pc = 0x60b599e; continue;
 }
 0x60b599e => {
  { let v: u32 = 0x2000u32; cpu.ebx = v; }
  pc = 0x60b59ab; continue;
 }
 0x60b59a5 => {
  { let v: u32 = cpu.r32(0x62d1ea8u32); cpu.ebx = v; }
  pc = 0x60b59ab; continue;
 }
 0x60b59ab => {
  { let v: u32 = 0xfu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x60b59c4; continue;
 }
 0x60b59c4 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x2cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x34u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b59d9); f_060b3240(cpu); if cpu.returned_to != 0x60b59d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5a09; continue; }
  pc = 0x60b59e0; continue;
 }
 0x60b59e0 => {
  { let v: u32 = 0x616ad8cu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b59ec; continue;
 }
 0x60b59ec => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b59ec; continue; }
  pc = 0x60b59f6; continue;
 }
 0x60b59f6 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b59fc); f_060b32d0(cpu); if cpu.returned_to != 0x60b59fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5a09 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5a21; continue; }
  pc = 0x60b5a10; continue;
 }
 0x60b5a10 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5a1e); f_060b3200(cpu); if cpu.returned_to != 0x60b5a1e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60b5a21; continue;
 }
 0x60b5a21 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x60b59c4; continue; }
  pc = 0x60b5a55; continue;
 }
 0x60b5a55 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5a5b); f_060b5f20(cpu); if cpu.returned_to != 0x60b5a5b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5a89; continue; }
  pc = 0x60b5a62; continue;
 }
 0x60b5a62 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5a73); f_060b3200(cpu); if cpu.returned_to != 0x60b5a73 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5a7c); f_060b32d0(cpu); if cpu.returned_to != 0x60b5a7c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5a89 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b5a9b); f_060acc60(cpu); if cpu.returned_to != 0x60b5a9b { return; }
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
  cpu.push(0x60b5ad8); f_060b3300(cpu); if cpu.returned_to != 0x60b5ad8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x54u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5b53; continue; }
  pc = 0x60b5ae2; continue;
 }
 0x60b5ae2 => {
  { let v: u32 = 0x616adacu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b5aee; continue;
 }
 0x60b5aee => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5aee; continue; }
  pc = 0x60b5af8; continue;
 }
 0x60b5af8 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b5b0d; continue; }
  pc = 0x60b5b04; continue;
 }
 0x60b5b04 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5b0a); f_060acdb0(cpu); if cpu.returned_to != 0x60b5b0a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b5b0d; continue;
 }
 0x60b5b0d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5b1f); f_060acc60(cpu); if cpu.returned_to != 0x60b5b1f { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5b3d); f_060b3200(cpu); if cpu.returned_to != 0x60b5b3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5b46); f_060b32d0(cpu); if cpu.returned_to != 0x60b5b46 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5b53 => {
  { let v: u32 = cpu.r32(0x62d1eb0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x64u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x8a8u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5b67); f_060b3300(cpu); if cpu.returned_to != 0x60b5b67 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b5bf2; continue; }
  pc = 0x60b5b75; continue;
 }
 0x60b5b75 => {
  { let v: u32 = 0x616add0u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b5b81; continue;
 }
 0x60b5b81 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5b81; continue; }
  pc = 0x60b5b8b; continue;
 }
 0x60b5b8b => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b5ba0; continue; }
  pc = 0x60b5b97; continue;
 }
 0x60b5b97 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5b9d); f_060acdb0(cpu); if cpu.returned_to != 0x60b5b9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b5ba0; continue;
 }
 0x60b5ba0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b5bb2); f_060acc60(cpu); if cpu.returned_to != 0x60b5bb2 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5bd0); f_060b3200(cpu); if cpu.returned_to != 0x60b5bd0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5bdc); f_060b32d0(cpu); if cpu.returned_to != 0x60b5bdc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5be5); f_060b32d0(cpu); if cpu.returned_to != 0x60b5be5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5bf2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b5c47; continue; }
  pc = 0x60b5bff; continue;
 }
 0x60b5bff => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x616adf8u32; cpu.esi = v; }
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b5bff; continue; }
  pc = 0x60b5c47; continue;
 }
 0x60b5c47 => {
  { let v = 0x60b63a0u32; cpu.push(v); }
  cpu.push(0x60b5c51); f_060ac770(cpu); if cpu.returned_to != 0x60b5c51 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b5ce9; continue; }
  pc = 0x60b5c60; continue;
 }
 0x60b5c60 => {
  { let v: u32 = 0x616ae00u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b5c6c; continue;
 }
 0x60b5c6c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5c6c; continue; }
  pc = 0x60b5c76; continue;
 }
 0x60b5c76 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b5c8b; continue; }
  pc = 0x60b5c82; continue;
 }
 0x60b5c82 => {
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5c88); f_060acdb0(cpu); if cpu.returned_to != 0x60b5c88 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b5c8b; continue;
 }
 0x60b5c8b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b5c9d); f_060acc60(cpu); if cpu.returned_to != 0x60b5c9d { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5cbb); f_060b3200(cpu); if cpu.returned_to != 0x60b5cbb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5cc7); f_060b32d0(cpu); if cpu.returned_to != 0x60b5cc7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5cd3); f_060b32d0(cpu); if cpu.returned_to != 0x60b5cd3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5cdc); f_060b32d0(cpu); if cpu.returned_to != 0x60b5cdc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b5ce9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b5cf0); f_060ac660(cpu); if cpu.returned_to != 0x60b5cf0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d1ea0u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b5d02); f_060ac4e0(cpu); if cpu.returned_to != 0x60b5d02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5d0e); f_060ac290(cpu); if cpu.returned_to != 0x60b5d0e { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x60b5d50u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b5d27); f_060c3d86(cpu); if cpu.returned_to != 0x60b5d27 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5d32); f_060c3cf3(cpu); if cpu.returned_to != 0x60b5d32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5d3d); f_060c3cf3(cpu); if cpu.returned_to != 0x60b5d3d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x60b5d42; continue;
 }
 0x60b5d42 => {
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
pub fn f_060b5d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5d50); }
 let mut pc: u32 = 0x60b5d50;
 loop { match pc {
 0x60b5d50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b5da6; continue; }
  pc = 0x60b5d5f; continue;
 }
 0x60b5d5f => {
  if cpu.zf { pc = 0x60b5d7d; continue; }
  pc = 0x60b5d61; continue;
 }
 0x60b5d61 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5d73); f_060acc60(cpu); if cpu.returned_to != 0x60b5d73 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b5d7d; continue;
 }
 0x60b5d7d => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5d83); f_060c3d86(cpu); if cpu.returned_to != 0x60b5d83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5d8e); f_060c3cf3(cpu); if cpu.returned_to != 0x60b5d8e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5d99); f_060c3cf3(cpu); if cpu.returned_to != 0x60b5d99 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xau32; cpu.push(v); }
  cpu.push(0x60b5da3); f_060acbb0(cpu); if cpu.returned_to != 0x60b5da3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b5da6; continue;
 }
 0x60b5da6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b5daf); f_060ac030(cpu); if cpu.returned_to != 0x60b5daf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b5dc3); f_060b3200(cpu); if cpu.returned_to != 0x60b5dc3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b5dcf); f_060b32d0(cpu); if cpu.returned_to != 0x60b5dcf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5ddb); f_060b32d0(cpu); if cpu.returned_to != 0x60b5ddb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b5de4); f_060b32d0(cpu); if cpu.returned_to != 0x60b5de4 { return; }
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
pub fn f_060b5df0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5df0); }
 let mut pc: u32 = 0x60b5df0;
 loop { match pc {
 0x60b5df0 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x60b5e92; continue; }
  pc = 0x60b5e19; continue;
 }
 0x60b5e19 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b5e3e; continue; }
  pc = 0x60b5e37; continue;
 }
 0x60b5e37 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b5e3e; continue;
 }
 0x60b5e3e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b5e51; continue; }
  pc = 0x60b5e48; continue;
 }
 0x60b5e48 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b5e51; continue;
 }
 0x60b5e51 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b5e65; continue; }
  pc = 0x60b5e5c; continue;
 }
 0x60b5e5c => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b5e65; continue;
 }
 0x60b5e65 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b5e79; continue; }
  pc = 0x60b5e70; continue;
 }
 0x60b5e70 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b5e79; continue;
 }
 0x60b5e79 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b5e7b; continue;
 }
 0x60b5e7b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x0u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b5e8a; continue; }
  pc = 0x60b5e82; continue;
 }
 0x60b5e82 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x60b5e8a; continue;
 }
 0x60b5e8a => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b5e7b; continue; }
  pc = 0x60b5e92; continue;
 }
 0x60b5e92 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x16u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x62d1ec4u32); cpu.ebx = v; }
  { cpu.movs(4, true); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60b5ec9; continue; }
  pc = 0x60b5eae; continue;
 }
 0x60b5eae => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x60b5ee3; continue;
 }
 0x60b5ec9 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x26u32.wrapping_add(cpu.esp), v); }
  { let v = 0x304u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  pc = 0x60b5ee3; continue;
 }
 0x60b5ee3 => {
  cpu.push(0x60b5ee8); f_060acc60(cpu); if cpu.returned_to != 0x60b5ee8 { return; }
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
pub fn f_060b5f00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b5f00); } r_060b5f00(cpu, 0x60b5f00); }
pub fn f_060b5f10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b5f10); } r_060b5f00(cpu, 0x60b5f10); }
fn r_060b5f00(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b5f00 => {
  { let v: u32 = cpu.rcr32(cpu.r32(0xbu32.wrapping_add(cpu.edi)), (cpu.ecx & 0xff)); cpu.w32(0xbu32.wrapping_add(cpu.edi), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.cf = !cpu.cf; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { cpu.pushad(); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xbu32.wrapping_add(cpu.eax)), cpu.esp); }
  { cpu.push(cpu.segments[0] as u32); }
  pc = 0x60b5f10; continue;
 }
 0x60b5f10 => {
  { let v = cpu.pop(); cpu.set_flags(v); }
  { cpu.popad(); }
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xad060b61u32; cpu.ebp = v; }
  { cpu.popad(); }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.call(0x60b5f1c); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b5f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b5f20); }
 let mut pc: u32 = 0x60b5f20;
 loop { match pc {
 0x60b5f20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b5fb3; continue; }
  pc = 0x60b5f3a; continue;
 }
 0x60b5f3a => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b5f72; continue; }
  pc = 0x60b5f43; continue;
 }
 0x60b5f43 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b5f45; continue;
 }
 0x60b5f45 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8a4u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x4u32); }
  if !cpu.zf { pc = 0x60b5f65; continue; }
  pc = 0x60b5f5e; continue;
 }
 0x60b5f5e => {
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60b5f65; continue;
 }
 0x60b5f65 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b5f45; continue; }
  pc = 0x60b5f72; continue;
 }
 0x60b5f72 => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b5f94; continue; }
  pc = 0x60b5f78; continue;
 }
 0x60b5f78 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b5f8a); f_060acc60(cpu); if cpu.returned_to != 0x60b5f8a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b5f94; continue;
 }
 0x60b5f94 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5f9a); f_060c3d86(cpu); if cpu.returned_to != 0x60b5f9a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5fa5); f_060c3cf3(cpu); if cpu.returned_to != 0x60b5fa5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b5fb0); f_060c3cf3(cpu); if cpu.returned_to != 0x60b5fb0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60b5fb3; continue;
 }
 0x60b5fb3 => {
  { let v: u32 = cpu.r32(0x62d1ebcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1ec0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6059; continue; }
  pc = 0x60b5fcb; continue;
 }
 0x60b5fcb => {
  cpu.poll(); pc = cpu.r32(0x60b5f00u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x60b5fd3 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6059; continue;
 }
 0x60b5ff5 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6059; continue;
 }
 0x60b6017 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6059; continue;
 }
 0x60b6039 => {
  { let v: u32 = 0x3u32; cpu.ecx = v; }
  { let v: u32 = 0x2u32; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6059; continue;
 }
 0x60b6059 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b605e; continue;
 }
 0x60b605e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edi.wrapping_mul(1))), 0x0u8); }
  if cpu.zf { pc = 0x60b606c; continue; }
  pc = 0x60b6067; continue;
 }
 0x60b6067 => {
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.esi), v); }
  pc = 0x60b6074; continue;
 }
 0x60b606c => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b605e; continue; }
  pc = 0x60b6074; continue;
 }
 0x60b6074 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x62d1ea4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x60b609d; continue; }
  pc = 0x60b6094; continue;
 }
 0x60b6094 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b60bf; continue; }
  pc = 0x60b6096; continue;
 }
 0x60b6096 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x60b60dd; continue; }
  pc = 0x60b609b; continue;
 }
 0x60b609b => {
  pc = 0x60b60fb; continue;
 }
 0x60b609d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b60fb; continue; }
  pc = 0x60b60a1; continue;
 }
 0x60b60a1 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x60b618a; continue;
 }
 0x60b60bf => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x60b618a; continue;
 }
 0x60b60dd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(8)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  pc = 0x60b618a; continue;
 }
 0x60b60fb => {
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
  { let v: u32 = cpu.r32(0x62d1ea4u32); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b6160; continue;
 }
 0x60b6160 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b616b; continue; }
  pc = 0x60b6169; continue;
 }
 0x60b6169 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60b616b; continue;
 }
 0x60b616b => {
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b617e; continue; }
  pc = 0x60b616f; continue;
 }
 0x60b616f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.edx.wrapping_mul(1)))); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b617c; continue; }
  pc = 0x60b617a; continue;
 }
 0x60b617a => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60b617c; continue;
 }
 0x60b617c => {
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x60b617e; continue;
 }
 0x60b617e => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.edi, 0x3u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b6160; continue; }
  pc = 0x60b6187; continue;
 }
 0x60b6187 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  pc = 0x60b618a; continue;
 }
 0x60b618a => {
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b61d4; continue; }
  pc = 0x60b6195; continue;
 }
 0x60b6195 => {
  cpu.poll(); pc = cpu.r32(0x60b5f10u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x60b619d => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b61d4; continue;
 }
 0x60b61ad => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b61d4; continue;
 }
 0x60b61bd => {
  { let v: u32 = 0x1u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b61cd; continue;
 }
 0x60b61c6 => {
  { let v: u32 = 0x2u32; cpu.w32(0x40u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b61cd; continue;
 }
 0x60b61cd => {
  { let v: u32 = 0x2u32; cpu.w32(0x44u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b61d4; continue;
 }
 0x60b61d4 => {
  { let v: u32 = cpu.r32(0x62d1eacu32); cpu.eax = v; }
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
  { let v: u32 = cpu.r32(0x62d1ee8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r16(0x18u32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b6259; continue; }
  pc = 0x60b6225; continue;
 }
 0x60b6225 => {
  { cpu.sub32(cpu.ebp, 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b6232; continue; }
  pc = 0x60b622d; continue;
 }
 0x60b622d => {
  { let v: u32 = 0x800u32; cpu.ebp = v; }
  pc = 0x60b6232; continue;
 }
 0x60b6232 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esp)), 0x800u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b6259; continue; }
  pc = 0x60b623c; continue;
 }
 0x60b623c => {
  { let v: u32 = 0x616acf4u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b6248; continue;
 }
 0x60b6248 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b6248; continue; }
  pc = 0x60b6252; continue;
 }
 0x60b6252 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b6397; continue;
 }
 0x60b6259 => {
  { let v: u32 = 0x7fffffffu32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.edi = v; }
  pc = 0x60b6285; continue;
 }
 0x60b6265 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b6270; continue; }
  pc = 0x60b626e; continue;
 }
 0x60b626e => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60b6270; continue;
 }
 0x60b6270 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b6283; continue; }
  pc = 0x60b6274; continue;
 }
 0x60b6274 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b627f; continue; }
  pc = 0x60b627d; continue;
 }
 0x60b627d => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60b627f; continue;
 }
 0x60b627f => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x60b6283; continue;
 }
 0x60b6283 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.edi); cpu.edi = v; }
  pc = 0x60b6285; continue;
 }
 0x60b6285 => {
  { let v: u32 = cpu.r32(0x62d1ea8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b6265; continue; }
  pc = 0x60b6297; continue;
 }
 0x60b6297 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x60b62a5; continue; }
  pc = 0x60b62a2; continue;
 }
 0x60b62a2 => {
  { let v: u32 = cpu.edi; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b62a5; continue;
 }
 0x60b62a5 => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x14u32.wrapping_add(cpu.esi))); }
  if (cpu.sf == cpu.of) { pc = 0x60b62ad; continue; }
  pc = 0x60b62aa; continue;
 }
 0x60b62aa => {
  { let v: u32 = cpu.ebp; cpu.w32(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b62ad; continue;
 }
 0x60b62ad => {
  { let v: u32 = cpu.r32(0x62d1ee8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b62ca; continue; }
  pc = 0x60b62ba; continue;
 }
 0x60b62ba => {
  { let v: u32 = cpu.shr32(cpu.eax, 12u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xff000u32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 12u32); cpu.eax = v; }
  pc = 0x60b62ca; continue;
 }
 0x60b62ca => {
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
  if cpu.zf { pc = 0x60b6392; continue; }
  pc = 0x60b6316; continue;
 }
 0x60b6316 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b633c; continue; }
  pc = 0x60b631f; continue;
 }
 0x60b631f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b6321; continue;
 }
 0x60b6321 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8a4u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8a8u32); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b6321; continue; }
  pc = 0x60b633c; continue;
 }
 0x60b633c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b6342); f_060c3d86(cpu); if cpu.returned_to != 0x60b6342 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b634d); f_060c3cf3(cpu); if cpu.returned_to != 0x60b634d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b6358); f_060c3cf3(cpu); if cpu.returned_to != 0x60b6358 { return; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60b6392; continue; }
  pc = 0x60b6362; continue;
 }
 0x60b6362 => {
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
  cpu.push(0x60b6388); f_060acc60(cpu); if cpu.returned_to != 0x60b6388 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x58u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b6392; continue;
 }
 0x60b6392 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60b6397; continue;
 }
 0x60b6397 => {
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
pub fn f_060b63a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b63a0); }
 let mut pc: u32 = 0x60b63a0;
 loop { match pc {
 0x60b63a0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x617c6b0u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b659d; continue; }
  pc = 0x60b63b6; continue;
 }
 0x60b63b6 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1ec8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b0u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f38u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b63de; continue; }
  pc = 0x60b63d4; continue;
 }
 0x60b63d4 => {
  cpu.push(0x60b63d9); f_060acda0(cpu); if cpu.returned_to != 0x60b63d9 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f40u32, v); }
  pc = 0x60b63de; continue;
 }
 0x60b63de => {
  { let v: u32 = cpu.r32(0x62d1f38u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x60b641b; continue; }
  pc = 0x60b63f0; continue;
 }
 0x60b63f0 => {
  { cpu.sub32(cpu.r32(0x62d1ec8u32), 0x0u32); }
  if cpu.zf { pc = 0x60b6408; continue; }
  pc = 0x60b63f9; continue;
 }
 0x60b63f9 => {
  { let v: u32 = cpu.r32(0x62d1f40u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b6405); f_060acd90(cpu); if cpu.returned_to != 0x60b6405 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b6408; continue;
 }
 0x60b6408 => {
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b0u32, v); }
  pc = 0x60b659d; continue;
 }
 0x60b641b => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1f3cu32, v); }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b6431; continue; }
  pc = 0x60b642c; continue;
 }
 0x60b642c => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x3cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x60b646a; continue; }
  pc = 0x60b6431; continue;
 }
 0x60b6431 => {
  { let v: u32 = cpu.r32(0x62d1ec8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f34u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b6450; continue; }
  pc = 0x60b6441; continue;
 }
 0x60b6441 => {
  { let v: u32 = cpu.r32(0x62d1f40u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b644d); f_060acd90(cpu); if cpu.returned_to != 0x60b644d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b6450; continue;
 }
 0x60b6450 => {
  { let v: u32 = cpu.xor32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x617c6b0u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f34u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 0x60b646a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b6473); f_060c3d86(cpu); if cpu.returned_to != 0x60b6473 { return; }
  { let v: u32 = cpu.r32(0x62d1f38u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1f44u32, v); }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f34u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f30u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b64d1; continue; }
  pc = 0x60b6498; continue;
 }
 0x60b6498 => {
  { let v: u32 = cpu.r32(0x62d1f30u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x60b64ae; continue; }
  pc = 0x60b64a3; continue;
 }
 0x60b64a3 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x62d1f44u32)); cpu.w32(0x62d1f44u32, v); }
  cpu.push(0x60b64ae); f_060b68a0(cpu); if cpu.returned_to != 0x60b64ae { return; }
  pc = 0x60b64ae; continue;
 }
 0x60b64ae => {
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x62d1f30u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8a8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f34u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x62d1f30u32, v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x60b6498; continue; }
  pc = 0x60b64d1; continue;
 }
 0x60b64d1 => {
  { let v: u32 = cpu.r32(0x62d1f38u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f44u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x68u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d1f3cu32); cpu.ebx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b64ef); f_060c3cf3(cpu); if cpu.returned_to != 0x60b64ef { return; }
  { let v: u32 = cpu.r32(0x62d1f38u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x20u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x60b6531; continue; }
  pc = 0x60b64ff; continue;
 }
 0x60b64ff => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x62d1f24u32; cpu.push(v); }
  { let v: u16 = cpu.r16(0x62d1f3cu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = 0x405u32; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ebx as u8) ^ 0x1u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x62d1f28u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x62d1f2au32, v); }
  cpu.push(0x60b652e); f_060acc60(cpu); if cpu.returned_to != 0x60b652e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60b6531; continue;
 }
 0x60b6531 => {
  { let v: u32 = cpu.r32(0x62d1f38u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b654d; continue; }
  pc = 0x60b6544; continue;
 }
 0x60b6544 => {
  { let v: u32 = 0x0u32; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b6577; continue;
 }
 0x60b654d => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5cu32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x60b6577; continue; }
  pc = 0x60b655b; continue;
 }
 0x60b655b => {
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b6577; continue; }
  pc = 0x60b6561; continue;
 }
 0x60b6561 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b6571); f_060acc60(cpu); if cpu.returned_to != 0x60b6571 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b6577; continue;
 }
 0x60b6577 => {
  { cpu.sub32(cpu.r32(0x62d1ec8u32), 0x0u32); }
  if cpu.zf { pc = 0x60b658f; continue; }
  pc = 0x60b6580; continue;
 }
 0x60b6580 => {
  { let v: u32 = cpu.r32(0x62d1f40u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b658c); f_060acd90(cpu); if cpu.returned_to != 0x60b658c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b658f; continue;
 }
 0x60b658f => {
  { let v: u32 = cpu.xor32(cpu.edi, cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x62d1f34u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x617c6b0u32, v); }
  pc = 0x60b659d; continue;
 }
 0x60b659d => {
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f34u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b65b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b65b0); }
 let mut pc: u32 = 0x60b65b0;
 loop { match pc {
 0x60b65b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b65cc; continue; }
  pc = 0x60b65c3; continue;
 }
 0x60b65c3 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b65d7; continue;
 }
 0x60b65cc => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60b65d7; continue; }
  pc = 0x60b65d0; continue;
 }
 0x60b65d0 => {
  { let v: u32 = 0x0u32; cpu.w32(0x50u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b65d7; continue;
 }
 0x60b65d7 => {
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b65e8; continue; }
  pc = 0x60b65df; continue;
 }
 0x60b65df => {
  { let v: u32 = 0x7fu32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b65f3; continue;
 }
 0x60b65e8 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60b65f3; continue; }
  pc = 0x60b65ec; continue;
 }
 0x60b65ec => {
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b65f3; continue;
 }
 0x60b65f3 => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b6622; continue; }
  pc = 0x60b661c; continue;
 }
 0x60b661c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6622; continue;
 }
 0x60b6622 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.esp)), 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b6631; continue; }
  pc = 0x60b6629; continue;
 }
 0x60b6629 => {
  { let v: u32 = 0x7fu32; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6631; continue;
 }
 0x60b6631 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0x1u32); }
  if cpu.zf { pc = 0x60b663e; continue; }
  pc = 0x60b6639; continue;
 }
 0x60b6639 => {
  { cpu.sub32(cpu.ebp, 0x3u32); }
  if !cpu.zf { pc = 0x60b6687; continue; }
  pc = 0x60b663e; continue;
 }
 0x60b663e => {
  { let v: u32 = 0x7fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x617c630u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x617c630u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x458u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b6897; continue;
 }
 0x60b6687 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b6696; continue; }
  pc = 0x60b668f; continue;
 }
 0x60b668f => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b6696; continue;
 }
 0x60b6696 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x60b66c4; continue; }
  pc = 0x60b66a1; continue;
 }
 0x60b66a1 => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if cpu.zf { pc = 0x60b66c4; continue; }
  pc = 0x60b66a6; continue;
 }
 0x60b66a6 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b66b3; continue; }
  pc = 0x60b66aa; continue;
 }
 0x60b66aa => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if !cpu.zf { pc = 0x60b67fc; continue; }
  pc = 0x60b66b3; continue;
 }
 0x60b66b3 => {
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.ecx)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x2u32); }
  if cpu.zf { pc = 0x60b66c4; continue; }
  pc = 0x60b66bb; continue;
 }
 0x60b66bb => {
  { cpu.sub32(cpu.esi, 0x3u32); }
  if !cpu.zf { pc = 0x60b67fc; continue; }
  pc = 0x60b66c4; continue;
 }
 0x60b66c4 => {
  { let v: u32 = 0x7fu32; cpu.ebx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edi); cpu.ebx = v; }
  { let v: u32 = 0x458u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x617c630u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r8(0x617c630u32.wrapping_add(cpu.edi)) as u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and8(cpu.r8(0x48u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x60b679f; continue; }
  pc = 0x60b66fc; continue;
 }
 0x60b66fc => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebp, 0x0u32); cpu.ecx = v; }
  pc = 0x60b670d; continue;
 }
 0x60b670d => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b670d; continue; }
  pc = 0x60b6743; continue;
 }
 0x60b6743 => {
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
  pc = 0x60b6767; continue;
 }
 0x60b6767 => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b6897; continue; }
  pc = 0x60b679d; continue;
 }
 0x60b679d => {
  cpu.poll(); pc = 0x60b6767; continue;
 }
 0x60b679f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 17u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = 0xffff8000u32; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 15u32); cpu.ecx = v; }
  pc = 0x60b67c0; continue;
 }
 0x60b67c0 => {
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
  if (cpu.sf == cpu.of) { pc = 0x60b6897; continue; }
  pc = 0x60b67fa; continue;
 }
 0x60b67fa => {
  cpu.poll(); pc = 0x60b67c0; continue;
 }
 0x60b67fc => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x58u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x48u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60b6867; continue; }
  pc = 0x60b680e; continue;
 }
 0x60b680e => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x0u32); cpu.edx = v; }
  pc = 0x60b6819; continue;
 }
 0x60b6819 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b6819; continue; }
  pc = 0x60b6834; continue;
 }
 0x60b6834 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 8u32); cpu.esi = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x60b684e; continue;
 }
 0x60b684e => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x100u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 7u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x60b6897; continue; }
  pc = 0x60b6865; continue;
 }
 0x60b6865 => {
  cpu.poll(); pc = 0x60b684e; continue;
 }
 0x60b6867 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 17u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0xffff8000u32; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 15u32); cpu.edx = v; }
  pc = 0x60b687c; continue;
 }
 0x60b687c => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x100u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 7u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x8000u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b687c; continue; }
  pc = 0x60b6897; continue;
 }
 0x60b6897 => {
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
pub fn f_060b68a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b68a0); }
 let mut pc: u32 = 0x60b68a0;
 loop { match pc {
 0x60b68a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x60b69b6; continue; }
  pc = 0x60b68b2; continue;
 }
 0x60b68b2 => {
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x60b68b4; continue;
 }
 0x60b68b4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if (cpu.sf == cpu.of) { pc = 0x60b69b6; continue; }
  pc = 0x60b68c0; continue;
 }
 0x60b68c0 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60b68ee; continue; }
  pc = 0x60b68d8; continue;
 }
 0x60b68d8 => {
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b68ee; continue; }
  pc = 0x60b68e0; continue;
 }
 0x60b68e0 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x14u32.wrapping_add(cpu.eax))); }
  if !cpu.cf { pc = 0x60b68ee; continue; }
  pc = 0x60b68e5; continue;
 }
 0x60b68e5 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b68ee; continue;
 }
 0x60b68ee => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b6994; continue; }
  pc = 0x60b68f6; continue;
 }
 0x60b68f6 => {
  { cpu.sub32(cpu.r32(0x860u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b6906; continue; }
  pc = 0x60b68ff; continue;
 }
 0x60b68ff => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x860u32.wrapping_add(cpu.esi)); cpu.push(0x60b6906); cpu.call(t); if cpu.returned_to != 0x60b6906 { return; }
  pc = 0x60b6906; continue;
 }
 0x60b6906 => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x60b692a; continue; }
  pc = 0x60b690c; continue;
 }
 0x60b690c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x60b68b4; continue; }
  pc = 0x60b6922; continue;
 }
 0x60b6922 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x60b68b4; continue;
 }
 0x60b692a => {
  { cpu.sub32(cpu.r32(0x858u32.wrapping_add(cpu.esi)), 0x2u32); }
  if !cpu.zf { pc = 0x60b696e; continue; }
  pc = 0x60b6933; continue;
 }
 0x60b6933 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) ^ 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60b696e; continue; }
  pc = 0x60b6948; continue;
 }
 0x60b6948 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60b6957; continue; }
  pc = 0x60b694e; continue;
 }
 0x60b694e => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b69b6; continue;
 }
 0x60b6957 => {
  { cpu.sub32(cpu.r32(0x1cu32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x60b6966; continue; }
  pc = 0x60b695d; continue;
 }
 0x60b695d => {
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b69b6; continue;
 }
 0x60b6966 => {
  { let v: u32 = cpu.edx; cpu.w32(0x2cu32.wrapping_add(cpu.esi), v); }
  cpu.poll(); pc = 0x60b68b4; continue;
 }
 0x60b696e => {
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x34u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b69b6; continue; }
  pc = 0x60b6986; continue;
 }
 0x60b6986 => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x864u32.wrapping_add(cpu.esi)); cpu.push(0x60b698d); cpu.call(t); if cpu.returned_to != 0x60b698d { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6994 => {
  { cpu.sub32(cpu.r32(0x85cu32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b69a4; continue; }
  pc = 0x60b699d; continue;
 }
 0x60b699d => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x85cu32.wrapping_add(cpu.esi)); cpu.push(0x60b69a4); cpu.call(t); if cpu.returned_to != 0x60b69a4 { return; }
  pc = 0x60b69a4; continue;
 }
 0x60b69a4 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b69ac); f_060c3b90(cpu); if cpu.returned_to != 0x60b69ac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  cpu.poll(); pc = 0x60b68b4; continue;
 }
 0x60b69b6 => {
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
pub fn f_060b69c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b69c0); }
 let mut pc: u32 = 0x60b69c0;
 loop { match pc {
 0x60b69c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x58u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b69e7; continue; }
  pc = 0x60b69cb; continue;
 }
 0x60b69cb => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x402u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b69dd); f_060acc60(cpu); if cpu.returned_to != 0x60b69dd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b69e7; continue;
 }
 0x60b69e7 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6a40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6a40); }
 let mut pc: u32 = 0x60b6a40;
 loop { match pc {
 0x60b6a40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c6b4u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b6afc; continue; }
  pc = 0x60b6a50; continue;
 }
 0x60b6a50 => {
  { let v = 0x60b4cc0u32; cpu.push(v); }
  { let v = 0x60b6a40u32; cpu.push(v); }
  cpu.push(0x60b6a5f); f_060b3160(cpu); if cpu.returned_to != 0x60b6a5f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x617c630u32; cpu.push(v); }
  cpu.push(0x60b6a71); f_060b30a0(cpu); if cpu.returned_to != 0x60b6a71 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c6b0u32; cpu.push(v); }
  cpu.push(0x60b6a80); f_060b30a0(cpu); if cpu.returned_to != 0x60b6a80 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f38u32; cpu.push(v); }
  cpu.push(0x60b6a8f); f_060b30a0(cpu); if cpu.returned_to != 0x60b6a8f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0xcu32; cpu.push(v); }
  { let v = 0x62d1f24u32; cpu.push(v); }
  cpu.push(0x60b6a9e); f_060b30a0(cpu); if cpu.returned_to != 0x60b6a9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f40u32; cpu.push(v); }
  cpu.push(0x60b6aad); f_060b30a0(cpu); if cpu.returned_to != 0x60b6aad { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f44u32; cpu.push(v); }
  cpu.push(0x60b6abc); f_060b30a0(cpu); if cpu.returned_to != 0x60b6abc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f34u32; cpu.push(v); }
  cpu.push(0x60b6acb); f_060b30a0(cpu); if cpu.returned_to != 0x60b6acb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f3cu32; cpu.push(v); }
  cpu.push(0x60b6ada); f_060b30a0(cpu); if cpu.returned_to != 0x60b6ada { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f30u32; cpu.push(v); }
  cpu.push(0x60b6ae9); f_060b30a0(cpu); if cpu.returned_to != 0x60b6ae9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60b6af6); f_060c3db2(cpu); if cpu.returned_to != 0x60b6af6 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b4u32, v); }
  pc = 0x60b6afc; continue;
 }
 0x60b6afc => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6b00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6b00); }
 let mut pc: u32 = 0x60b6b00;
 loop { match pc {
 0x60b6b00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c6b8u32), 0x0u32); }
  if cpu.zf { pc = 0x60b6b26; continue; }
  pc = 0x60b6b0c; continue;
 }
 0x60b6b0c => {
  { let v = 0x60b6b00u32; cpu.push(v); }
  { let v = 0x60b72e0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60b6b1d); f_060b30c0(cpu); if cpu.returned_to != 0x60b6b1d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b8u32, v); }
  pc = 0x60b6b26; continue;
 }
 0x60b6b26 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6b30); }
 let mut pc: u32 = 0x60b6b30;
 loop { match pc {
 0x60b6b30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b6b4b; continue; }
  pc = 0x60b6b39; continue;
 }
 0x60b6b39 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x888u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x888u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60b6b4b; continue;
 }
 0x60b6b4b => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6b50); }
 let mut pc: u32 = 0x60b6b50;
 loop { match pc {
 0x60b6b50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b6b65; continue; }
  pc = 0x60b6b5f; continue;
 }
 0x60b6b5f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6b65 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x616ae80u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b6b71; continue;
 }
 0x60b6b71 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6b7f; continue; }
  pc = 0x60b6b79; continue;
 }
 0x60b6b79 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6b7f; continue; }
  pc = 0x60b6b7d; continue;
 }
 0x60b6b7d => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6b7f; continue;
 }
 0x60b6b7f => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6b8c; continue; }
  pc = 0x60b6b84; continue;
 }
 0x60b6b84 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6b8c; continue; }
  pc = 0x60b6b89; continue;
 }
 0x60b6b89 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b6b8c; continue;
 }
 0x60b6b8c => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b6b9b; continue; }
  pc = 0x60b6b90; continue;
 }
 0x60b6b90 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b6b9b; continue; }
  pc = 0x60b6b94; continue;
 }
 0x60b6b94 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b6b9b; continue; }
  pc = 0x60b6b97; continue;
 }
 0x60b6b97 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b6b71; continue;
 }
 0x60b6b9b => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b6ba2; continue; }
  pc = 0x60b6ba0; continue;
 }
 0x60b6ba0 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6ba2; continue;
 }
 0x60b6ba2 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b6baa; continue; }
  pc = 0x60b6ba6; continue;
 }
 0x60b6ba6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b6c0d; continue;
 }
 0x60b6baa => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616ae8cu32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  pc = 0x60b6bb7; continue;
 }
 0x60b6bb7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6bc5; continue; }
  pc = 0x60b6bbf; continue;
 }
 0x60b6bbf => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6bc5; continue; }
  pc = 0x60b6bc3; continue;
 }
 0x60b6bc3 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6bc5; continue;
 }
 0x60b6bc5 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6bd2; continue; }
  pc = 0x60b6bca; continue;
 }
 0x60b6bca => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6bd2; continue; }
  pc = 0x60b6bcf; continue;
 }
 0x60b6bcf => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b6bd2; continue;
 }
 0x60b6bd2 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b6be1; continue; }
  pc = 0x60b6bd6; continue;
 }
 0x60b6bd6 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b6be1; continue; }
  pc = 0x60b6bda; continue;
 }
 0x60b6bda => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b6be1; continue; }
  pc = 0x60b6bdd; continue;
 }
 0x60b6bdd => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b6bb7; continue;
 }
 0x60b6be1 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b6be8; continue; }
  pc = 0x60b6be6; continue;
 }
 0x60b6be6 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6be8; continue;
 }
 0x60b6be8 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60b6c08; continue; }
  pc = 0x60b6bec; continue;
 }
 0x60b6bec => {
  { let v: u32 = 0x616aedcu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b6bf8; continue;
 }
 0x60b6bf8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b6bf8; continue; }
  pc = 0x60b6c02; continue;
 }
 0x60b6c02 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6c08 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60b6c0d; continue;
 }
 0x60b6c0d => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x60b6c18; continue; }
  pc = 0x60b6c11; continue;
 }
 0x60b6c11 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60b6c55; continue; }
  pc = 0x60b6c16; continue;
 }
 0x60b6c16 => {
  pc = 0x60b6c69; continue;
 }
 0x60b6c18 => {
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
  cpu.push(0x60b6c53); f_060b7018(cpu); if cpu.returned_to != 0x60b6c53 { return; }
  pc = 0x60b6c66; continue;
 }
 0x60b6c55 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b6c66); f_060b6e90(cpu); if cpu.returned_to != 0x60b6c66 { return; }
  pc = 0x60b6c66; continue;
 }
 0x60b6c66 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60b6c69; continue;
 }
 0x60b6c69 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.ebx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b6c8e; continue; }
  pc = 0x60b6c72; continue;
 }
 0x60b6c72 => {
  { let v: u32 = 0x616af04u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b6c7e; continue;
 }
 0x60b6c7e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b6c7e; continue; }
  pc = 0x60b6c88; continue;
 }
 0x60b6c88 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6c8e => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6ca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6ca0); }
 let mut pc: u32 = 0x60b6ca0;
 loop { match pc {
 0x60b6ca0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.sub32(cpu.r32(0x617c6b8u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b6cce; continue; }
  pc = 0x60b6cb1; continue;
 }
 0x60b6cb1 => {
  { let v = 0x60b6b00u32; cpu.push(v); }
  { let v = 0x60b72e0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60b6cc5); f_060b3160(cpu); if cpu.returned_to != 0x60b6cc5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b8u32, v); }
  pc = 0x60b6cce; continue;
 }
 0x60b6cce => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x60b6cd9; continue; }
  pc = 0x60b6cd2; continue;
 }
 0x60b6cd2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6cd9 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x616ae80u32; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  pc = 0x60b6ce5; continue;
 }
 0x60b6ce5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6cf3; continue; }
  pc = 0x60b6ced; continue;
 }
 0x60b6ced => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6cf3; continue; }
  pc = 0x60b6cf1; continue;
 }
 0x60b6cf1 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6cf3; continue;
 }
 0x60b6cf3 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6d00; continue; }
  pc = 0x60b6cf8; continue;
 }
 0x60b6cf8 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6d00; continue; }
  pc = 0x60b6cfd; continue;
 }
 0x60b6cfd => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b6d00; continue;
 }
 0x60b6d00 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b6d0f; continue; }
  pc = 0x60b6d04; continue;
 }
 0x60b6d04 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b6d0f; continue; }
  pc = 0x60b6d08; continue;
 }
 0x60b6d08 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b6d0f; continue; }
  pc = 0x60b6d0b; continue;
 }
 0x60b6d0b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b6ce5; continue;
 }
 0x60b6d0f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b6d16; continue; }
  pc = 0x60b6d14; continue;
 }
 0x60b6d14 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6d16; continue;
 }
 0x60b6d16 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b6d1e; continue; }
  pc = 0x60b6d1a; continue;
 }
 0x60b6d1a => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b6d82; continue;
 }
 0x60b6d1e => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616ae8cu32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edi = v; }
  pc = 0x60b6d2b; continue;
 }
 0x60b6d2b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6d39; continue; }
  pc = 0x60b6d33; continue;
 }
 0x60b6d33 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6d39; continue; }
  pc = 0x60b6d37; continue;
 }
 0x60b6d37 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6d39; continue;
 }
 0x60b6d39 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6d46; continue; }
  pc = 0x60b6d3e; continue;
 }
 0x60b6d3e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6d46; continue; }
  pc = 0x60b6d43; continue;
 }
 0x60b6d43 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b6d46; continue;
 }
 0x60b6d46 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b6d55; continue; }
  pc = 0x60b6d4a; continue;
 }
 0x60b6d4a => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b6d55; continue; }
  pc = 0x60b6d4e; continue;
 }
 0x60b6d4e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b6d55; continue; }
  pc = 0x60b6d51; continue;
 }
 0x60b6d51 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b6d2b; continue;
 }
 0x60b6d55 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b6d5c; continue; }
  pc = 0x60b6d5a; continue;
 }
 0x60b6d5a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6d5c; continue;
 }
 0x60b6d5c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60b6d7d; continue; }
  pc = 0x60b6d60; continue;
 }
 0x60b6d60 => {
  { let v: u32 = 0x616ae94u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b6d6c; continue;
 }
 0x60b6d6c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b6d6c; continue; }
  pc = 0x60b6d76; continue;
 }
 0x60b6d76 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6d7d => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x60b6d82; continue;
 }
 0x60b6d82 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b6d8c); f_060ab930(cpu); if cpu.returned_to != 0x60b6d8c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b6e4a; continue; }
  pc = 0x60b6d9b; continue;
 }
 0x60b6d9b => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b6da1); f_060ab6b0(cpu); if cpu.returned_to != 0x60b6da1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x888u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x60b6db9; continue; }
  pc = 0x60b6db2; continue;
 }
 0x60b6db2 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x60b6e04; continue; }
  pc = 0x60b6db7; continue;
 }
 0x60b6db7 => {
  pc = 0x60b6e22; continue;
 }
 0x60b6db9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x898u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = 0x60b6fd0u32; cpu.push(v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b6df7); f_060a9ae0(cpu); if cpu.returned_to != 0x60b6df7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b6e02); f_060b7018(cpu); if cpu.returned_to != 0x60b6e02 { return; }
  pc = 0x60b6e1f; continue;
 }
 0x60b6e04 => {
  { let v = 0x60b6e50u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x8a0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b6e15); f_060a9ae0(cpu); if cpu.returned_to != 0x60b6e15 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b6e1f); f_060b6e90(cpu); if cpu.returned_to != 0x60b6e1f { return; }
  pc = 0x60b6e1f; continue;
 }
 0x60b6e1f => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60b6e22; continue;
 }
 0x60b6e22 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b6e48; continue; }
  pc = 0x60b6e2b; continue;
 }
 0x60b6e2b => {
  { let v: u32 = 0x616aebcu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b6e37; continue;
 }
 0x60b6e37 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b6e37; continue; }
  pc = 0x60b6e41; continue;
 }
 0x60b6e41 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b6e48 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b6e4a; continue;
 }
 0x60b6e4a => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6e50); }
 let mut pc: u32 = 0x60b6e50;
 loop { match pc {
 0x60b6e50 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b6e65; continue; }
  pc = 0x60b6e5e; continue;
 }
 0x60b6e5e => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x60b6e65); cpu.call(t); if cpu.returned_to != 0x60b6e65 { return; }
  pc = 0x60b6e65; continue;
 }
 0x60b6e65 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b6e77; continue; }
  pc = 0x60b6e6e; continue;
 }
 0x60b6e6e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b6e74); f_060ab760(cpu); if cpu.returned_to != 0x60b6e74 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b6e77; continue;
 }
 0x60b6e77 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6e90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6e90); }
 let mut pc: u32 = 0x60b6e90;
 loop { match pc {
 0x60b6e90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x616ae70u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x60b6ea4; continue;
 }
 0x60b6ea4 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b6ead; continue;
 }
 0x60b6ead => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6ebb; continue; }
  pc = 0x60b6eb5; continue;
 }
 0x60b6eb5 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6ebb; continue; }
  pc = 0x60b6eb9; continue;
 }
 0x60b6eb9 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6ebb; continue;
 }
 0x60b6ebb => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6ec8; continue; }
  pc = 0x60b6ec0; continue;
 }
 0x60b6ec0 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6ec8; continue; }
  pc = 0x60b6ec5; continue;
 }
 0x60b6ec5 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b6ec8; continue;
 }
 0x60b6ec8 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b6ed7; continue; }
  pc = 0x60b6ecc; continue;
 }
 0x60b6ecc => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b6ed7; continue; }
  pc = 0x60b6ed0; continue;
 }
 0x60b6ed0 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b6ed7; continue; }
  pc = 0x60b6ed3; continue;
 }
 0x60b6ed3 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b6ead; continue;
 }
 0x60b6ed7 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b6ede; continue; }
  pc = 0x60b6edc; continue;
 }
 0x60b6edc => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6ede; continue;
 }
 0x60b6ede => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60b6ef4; continue; }
  pc = 0x60b6ee2; continue;
 }
 0x60b6ee2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x60b6ea4; continue;
 }
 0x60b6ef4 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x60b6f08; continue; }
  pc = 0x60b6efb; continue;
 }
 0x60b6efb => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x60b6f08; continue; }
  pc = 0x60b6f02; continue;
 }
 0x60b6f02 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x60b6f42; continue;
 }
 0x60b6f08 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x60b6f1c; continue; }
  pc = 0x60b6f0f; continue;
 }
 0x60b6f0f => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x8u16); }
  if !cpu.zf { pc = 0x60b6f1c; continue; }
  pc = 0x60b6f16; continue;
 }
 0x60b6f16 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x60b6f42; continue;
 }
 0x60b6f1c => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x1u16); }
  if !cpu.zf { pc = 0x60b6f30; continue; }
  pc = 0x60b6f23; continue;
 }
 0x60b6f23 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x60b6f30; continue; }
  pc = 0x60b6f2a; continue;
 }
 0x60b6f2a => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x60b6f42; continue;
 }
 0x60b6f30 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.ebx)), 0x2u16); }
  if !cpu.zf { pc = 0x60b6f4b; continue; }
  pc = 0x60b6f37; continue;
 }
 0x60b6f37 => {
  { cpu.sub16(cpu.r16(0x16u32.wrapping_add(cpu.ebx)), 0x10u16); }
  if !cpu.zf { pc = 0x60b6f4b; continue; }
  pc = 0x60b6f3e; continue;
 }
 0x60b6f3e => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x60b6f42; continue;
 }
 0x60b6f42 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b6f48); f_060ab3f0(cpu); if cpu.returned_to != 0x60b6f48 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60b6f4b; continue;
 }
 0x60b6f4b => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b6f55); f_060ab070(cpu); if cpu.returned_to != 0x60b6f55 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xcu32); cpu.ebx = v; }
  pc = 0x60b6f5f; continue;
 }
 0x60b6f5f => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616ae78u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b6f6b; continue;
 }
 0x60b6f6b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6f79; continue; }
  pc = 0x60b6f73; continue;
 }
 0x60b6f73 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6f79; continue; }
  pc = 0x60b6f77; continue;
 }
 0x60b6f77 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6f79; continue;
 }
 0x60b6f79 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b6f86; continue; }
  pc = 0x60b6f7e; continue;
 }
 0x60b6f7e => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b6f86; continue; }
  pc = 0x60b6f83; continue;
 }
 0x60b6f83 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b6f86; continue;
 }
 0x60b6f86 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b6f95; continue; }
  pc = 0x60b6f8a; continue;
 }
 0x60b6f8a => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b6f95; continue; }
  pc = 0x60b6f8e; continue;
 }
 0x60b6f8e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60b6f95; continue; }
  pc = 0x60b6f91; continue;
 }
 0x60b6f91 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b6f6b; continue;
 }
 0x60b6f95 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b6f9c; continue; }
  pc = 0x60b6f9a; continue;
 }
 0x60b6f9a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b6f9c; continue;
 }
 0x60b6f9c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60b6fb2; continue; }
  pc = 0x60b6fa0; continue;
 }
 0x60b6fa0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60b6f5f; continue;
 }
 0x60b6fb2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b6fc0); f_060ab4c0(cpu); if cpu.returned_to != 0x60b6fc0 { return; }
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
pub fn f_060b6fd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b6fd0); }
 let mut pc: u32 = 0x60b6fd0;
 loop { match pc {
 0x60b6fd0 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b6fdc); f_060b7018(cpu); if cpu.returned_to != 0x60b6fdc { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b6ff0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b6ff0); } r_060b6ff0(cpu, 0x60b6ff0); }
pub fn f_060b7018(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60b7018); } r_060b6ff0(cpu, 0x60b7018); }
fn r_060b6ff0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60b6ff0 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  if cpu.of { pc = 0x60b6ffe; continue; }
  pc = 0x60b6ff3; continue;
 }
 0x60b6ff3 => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v = cpu.ecx; cpu.push(v); }
  if cpu.of { pc = 0x60b7002; continue; }
  pc = 0x60b6ff7; continue;
 }
 0x60b6ff7 => {
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.cf { pc = 0x60b7006; continue; }
  pc = 0x60b6ffb; continue;
 }
 0x60b6ffb => {
  { cpu.push(cpu.segments[0] as u32); }
  if cpu.cf { pc = 0x60b700a; continue; }
  pc = 0x60b6fff; continue;
 }
 0x60b6ffe => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = 0x67060b70u32; cpu.edx = v; }
  if cpu.cf { pc = 0x60b7012; continue; }
  pc = 0x60b7007; continue;
 }
 0x60b6fff => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = 0x67060b70u32; cpu.edx = v; }
  if cpu.cf { pc = 0x60b7012; continue; }
  pc = 0x60b7007; continue;
 }
 0x60b7002 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  if cpu.cf { pc = 0x60b7012; continue; }
  pc = 0x60b7007; continue;
 }
 0x60b7006 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.unimplemented(0x60b7008, "Int1");
  if cpu.of { pc = 0x60b7016; continue; }
  pc = 0x60b700b; continue;
 }
 0x60b7007 => {
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60b7008, "Int1");
  if cpu.of { pc = 0x60b7016; continue; }
  pc = 0x60b700b; continue;
 }
 0x60b700a => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ecx)) | ((cpu.edx >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ecx), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.xor8(((cpu.edx >> 8) as u8), cpu.r8(0xbu32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60b7014, "Int3");
  if !cpu.of { pc = 0x60b7022; continue; }
  pc = 0x60b7017; continue;
 }
 0x60b700b => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0xbu32.wrapping_add(cpu.ecx)) | ((cpu.edx >> 8) as u8); cpu.w8(0xbu32.wrapping_add(cpu.ecx), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.xor8(((cpu.edx >> 8) as u8), cpu.r8(0xbu32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.push(cpu.segments[0] as u32); }
  cpu.unimplemented(0x60b7014, "Int3");
  if !cpu.of { pc = 0x60b7022; continue; }
  pc = 0x60b7017; continue;
 }
 0x60b7012 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.unimplemented(0x60b7014, "Int3");
  if !cpu.of { pc = 0x60b7022; continue; }
  pc = 0x60b7017; continue;
 }
 0x60b7016 => {
  { let v: u32 = cpu.eax | cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  pc = 0x60b7018; continue;
 }
 0x60b7017 => {
  { cpu.push(cpu.segments[0] as u32); }
  pc = 0x60b7018; continue;
 }
 0x60b7018 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b7026; continue;
 }
 0x60b7022 => {
  { let v: u8 = (cpu.eax as u8) & 0x18u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b7026; continue;
 }
 0x60b7026 => {
  { let v: u32 = cpu.r32(0x88cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x9u8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b7267; continue; }
  pc = 0x60b7036; continue;
 }
 0x60b7036 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x60b6ff0u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x60b7043 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b7049); f_060b7290(cpu); if cpu.returned_to != 0x60b7049 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b7051 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b7267; continue; }
  pc = 0x60b705e; continue;
 }
 0x60b705e => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b7064); f_060b72d0(cpu); if cpu.returned_to != 0x60b7064 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b7075); f_060ab4c0(cpu); if cpu.returned_to != 0x60b7075 { return; }
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
  cpu.push(0x60b7094); f_060ab070(cpu); if cpu.returned_to != 0x60b7094 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b70a1); f_060ab3f0(cpu); if cpu.returned_to != 0x60b70a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b7262; continue; }
  pc = 0x60b70ac; continue;
 }
 0x60b70ac => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b70b2); f_060ab340(cpu); if cpu.returned_to != 0x60b70b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b7262; continue;
 }
 0x60b70ba => {
  { let v: u32 = cpu.r32(0x898u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b7267; continue; }
  pc = 0x60b70c9; continue;
 }
 0x60b70c9 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { pc = 0x60b70e2; continue; }
  pc = 0x60b70d3; continue;
 }
 0x60b70d3 => {
  { let v: u32 = 0x1u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7267; continue;
 }
 0x60b70e2 => {
  { let v: u32 = 0x0u32; cpu.w32(0x89cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7267; continue;
 }
 0x60b70f1 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x890u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7267; continue;
 }
 0x60b7108 => {
  { let v: u32 = cpu.r32(0x894u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffu32); }
  if cpu.zf { pc = 0x60b7127; continue; }
  pc = 0x60b7116; continue;
 }
 0x60b7116 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x894u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b7267; continue; }
  pc = 0x60b7127; continue;
 }
 0x60b7127 => {
  { let v: u32 = cpu.r32(0x890u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x60b7267; continue;
 }
 0x60b7132 => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b7267; continue; }
  pc = 0x60b713f; continue;
 }
 0x60b713f => {
  { cpu.sub8(cpu.r8(0x7u32.wrapping_add(cpu.esi)), 0x0u8); }
  if cpu.zf { pc = 0x60b7165; continue; }
  pc = 0x60b7145; continue;
 }
 0x60b7145 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b714f); f_060ab3f0(cpu); if cpu.returned_to != 0x60b714f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7a12000u32; cpu.eax = v; }
  pc = 0x60b7183; continue;
 }
 0x60b7165 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b716f); f_060ab3f0(cpu); if cpu.returned_to != 0x60b716f { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x10000u32; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0xf424000u32; cpu.eax = v; }
  pc = 0x60b7183; continue;
 }
 0x60b7183 => {
  { let v = cpu.edi; cpu.div32(v); }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b718f); f_060ab070(cpu); if cpu.returned_to != 0x60b718f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b71a2); f_060b72d0(cpu); if cpu.returned_to != 0x60b71a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b71b3); f_060ab4c0(cpu); if cpu.returned_to != 0x60b71b3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b7262; continue; }
  pc = 0x60b71be; continue;
 }
 0x60b71be => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b71c4); f_060ab340(cpu); if cpu.returned_to != 0x60b71c4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b7262; continue;
 }
 0x60b71cc => {
  { cpu.sub32(cpu.r32(0x89cu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60b7267; continue; }
  pc = 0x60b71d9; continue;
 }
 0x60b71d9 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b71df); f_060b72d0(cpu); if cpu.returned_to != 0x60b71df { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0xcu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b71f0); f_060ab4c0(cpu); if cpu.returned_to != 0x60b71f0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b71fd); f_060ab070(cpu); if cpu.returned_to != 0x60b71fd { return; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x1u8); }
  if !cpu.zf { pc = 0x60b7215; continue; }
  pc = 0x60b7208; continue;
 }
 0x60b7208 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x60b7215; continue; }
  pc = 0x60b720f; continue;
 }
 0x60b720f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x60b724c; continue;
 }
 0x60b7215 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x60b7228; continue; }
  pc = 0x60b721b; continue;
 }
 0x60b721b => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x0u16); }
  if !cpu.zf { pc = 0x60b7228; continue; }
  pc = 0x60b7222; continue;
 }
 0x60b7222 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  pc = 0x60b724c; continue;
 }
 0x60b7228 => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x1u8); }
  if !cpu.zf { pc = 0x60b723b; continue; }
  pc = 0x60b722e; continue;
 }
 0x60b722e => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x60b723b; continue; }
  pc = 0x60b7235; continue;
 }
 0x60b7235 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  pc = 0x60b724c; continue;
 }
 0x60b723b => {
  { cpu.sub8(cpu.r8(0x9u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x60b7255; continue; }
  pc = 0x60b7241; continue;
 }
 0x60b7241 => {
  { cpu.sub16(cpu.r16(0xau32.wrapping_add(cpu.esi)), 0x4u16); }
  if !cpu.zf { pc = 0x60b7255; continue; }
  pc = 0x60b7248; continue;
 }
 0x60b7248 => {
  { let v = 0x1u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  pc = 0x60b724c; continue;
 }
 0x60b724c => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b7252); f_060ab3f0(cpu); if cpu.returned_to != 0x60b7252 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60b7255; continue;
 }
 0x60b7255 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b7262; continue; }
  pc = 0x60b7259; continue;
 }
 0x60b7259 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b725f); f_060ab340(cpu); if cpu.returned_to != 0x60b725f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b7262; continue;
 }
 0x60b7262 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x60b7267; continue;
 }
 0x60b7267 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b726d); f_060b72d0(cpu); if cpu.returned_to != 0x60b726d { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88cu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x60b7026; continue; }
  pc = 0x60b7283; continue;
 }
 0x60b7283 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7290); }
 let mut pc: u32 = 0x60b7290;
 loop { match pc {
 0x60b7290 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x888u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b72a6; continue; }
  pc = 0x60b729f; continue;
 }
 0x60b729f => {
  { let v = cpu.esi; cpu.push(v); }
  let t = cpu.r32(0x888u32.wrapping_add(cpu.esi)); cpu.push(0x60b72a6); cpu.call(t); if cpu.returned_to != 0x60b72a6 { return; }
  pc = 0x60b72a6; continue;
 }
 0x60b72a6 => {
  { cpu.sub32(cpu.r32(0x8a0u32.wrapping_add(cpu.esi)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b72b8; continue; }
  pc = 0x60b72af; continue;
 }
 0x60b72af => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b72b5); f_060ab760(cpu); if cpu.returned_to != 0x60b72b5 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b72b8; continue;
 }
 0x60b72b8 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8a0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b72d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b72d0); }
 let mut pc: u32 = 0x60b72d0;
 loop { match pc {
 0x60b72d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b72e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b72e0); }
 let mut pc: u32 = 0x60b72e0;
 loop { match pc {
 0x60b72e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c6b8u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b7309; continue; }
  pc = 0x60b72ec; continue;
 }
 0x60b72ec => {
  { let v = 0x60b6b00u32; cpu.push(v); }
  { let v = 0x60b72e0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60b7300); f_060b3160(cpu); if cpu.returned_to != 0x60b7300 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c6b8u32, v); }
  pc = 0x60b7309; continue;
 }
 0x60b7309 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7310); }
 let mut pc: u32 = 0x60b7310;
 loop { match pc {
 0x60b7310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c740u32), 0x0u32); }
  if cpu.zf { pc = 0x60b740f; continue; }
  pc = 0x60b7320; continue;
 }
 0x60b7320 => {
  { let v = 0x60b7310u32; cpu.push(v); }
  { let v = 0x60baaf0u32; cpu.push(v); }
  cpu.push(0x60b732f); f_060b30c0(cpu); if cpu.returned_to != 0x60b732f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x617c6bcu32; cpu.push(v); }
  cpu.push(0x60b7341); f_060b3080(cpu); if cpu.returned_to != 0x60b7341 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c73cu32; cpu.push(v); }
  cpu.push(0x60b7350); f_060b3080(cpu); if cpu.returned_to != 0x60b7350 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f5cu32; cpu.push(v); }
  cpu.push(0x60b735f); f_060b3080(cpu); if cpu.returned_to != 0x60b735f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f60u32; cpu.push(v); }
  cpu.push(0x60b736e); f_060b3080(cpu); if cpu.returned_to != 0x60b736e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f68u32; cpu.push(v); }
  cpu.push(0x60b737d); f_060b3080(cpu); if cpu.returned_to != 0x60b737d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f78u32; cpu.push(v); }
  cpu.push(0x60b738c); f_060b3080(cpu); if cpu.returned_to != 0x60b738c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f6cu32; cpu.push(v); }
  cpu.push(0x60b739b); f_060b3080(cpu); if cpu.returned_to != 0x60b739b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f70u32; cpu.push(v); }
  cpu.push(0x60b73aa); f_060b3080(cpu); if cpu.returned_to != 0x60b73aa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f58u32; cpu.push(v); }
  cpu.push(0x60b73b9); f_060b3080(cpu); if cpu.returned_to != 0x60b73b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f74u32; cpu.push(v); }
  cpu.push(0x60b73c8); f_060b3080(cpu); if cpu.returned_to != 0x60b73c8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f54u32; cpu.push(v); }
  cpu.push(0x60b73d7); f_060b3080(cpu); if cpu.returned_to != 0x60b73d7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f50u32; cpu.push(v); }
  cpu.push(0x60b73e6); f_060b3080(cpu); if cpu.returned_to != 0x60b73e6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f48u32; cpu.push(v); }
  cpu.push(0x60b73f5); f_060b3080(cpu); if cpu.returned_to != 0x60b73f5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f4cu32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60b7406); f_060b3080(cpu); if cpu.returned_to != 0x60b7406 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c740u32, v); }
  pc = 0x60b740f; continue;
 }
 0x60b740f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7420); }
 let mut pc: u32 = 0x60b7420;
 loop { match pc {
 0x60b7420 => {
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
  cpu.push(0x60b7444); f_060ba7c0(cpu); if cpu.returned_to != 0x60b7444 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  cpu.push(0x60b7458); f_060baa40(cpu); if cpu.returned_to != 0x60b7458 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60b746a; continue; }
  pc = 0x60b7463; continue;
 }
 0x60b7463 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x60b746c; continue;
 }
 0x60b746a => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x60b746c; continue;
 }
 0x60b746c => {
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
  cpu.push(0x60b749e); f_060baa40(cpu); if cpu.returned_to != 0x60b749e { return; }
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
pub fn f_060b75d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b75d0); }
 let mut pc: u32 = 0x60b75d0;
 loop { match pc {
 0x60b75d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b7601; continue; }
  pc = 0x60b75dd; continue;
 }
 0x60b75dd => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x94u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.edx)), 0x1u32); }
  if !cpu.zf { pc = 0x60b7601; continue; }
  pc = 0x60b75f9; continue;
 }
 0x60b75f9 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x60u32.wrapping_add(cpu.edx))); }
  if cpu.zf { pc = 0x60b7601; continue; }
  pc = 0x60b75fe; continue;
 }
 0x60b75fe => {
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b7601; continue;
 }
 0x60b7601 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7610); }
 let mut pc: u32 = 0x60b7610;
 loop { match pc {
 0x60b7610 => {
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
  if !cpu.zf { pc = 0x60b771a; continue; }
  pc = 0x60b7635; continue;
 }
 0x60b7635 => {
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
  cpu.push(0x60b765e); f_060ba8c0(cpu); if cpu.returned_to != 0x60b765e { return; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b76e8; continue; }
  pc = 0x60b7675; continue;
 }
 0x60b7675 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b767e; continue;
 }
 0x60b767e => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x60b76c9; continue; }
  pc = 0x60b7684; continue;
 }
 0x60b7684 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x60b7689; continue;
 }
 0x60b7689 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b76c2; continue; }
  pc = 0x60b7694; continue;
 }
 0x60b7694 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x60b76c2; continue; }
  pc = 0x60b769f; continue;
 }
 0x60b769f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b76b5); f_060b9e10(cpu); if cpu.returned_to != 0x60b76b5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b76c2; continue;
 }
 0x60b76c2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x60b7689; continue; }
  pc = 0x60b76c9; continue;
 }
 0x60b76c9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60b767e; continue; }
  pc = 0x60b76e8; continue;
 }
 0x60b76e8 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b7713; continue; }
  pc = 0x60b76ff; continue;
 }
 0x60b76ff => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if cpu.zf { pc = 0x60b7713; continue; }
  pc = 0x60b7709; continue;
 }
 0x60b7709 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b7710); f_060b9a70(cpu); if cpu.returned_to != 0x60b7710 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60b7713; continue;
 }
 0x60b7713 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b771a; continue;
 }
 0x60b771a => {
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
pub fn f_060b7730(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7730); }
 let mut pc: u32 = 0x60b7730;
 loop { match pc {
 0x60b7730 => {
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
  pc = 0x60b7759; continue;
 }
 0x60b7759 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x60b7778; continue; }
  pc = 0x60b775e; continue;
 }
 0x60b775e => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if cpu.zf { pc = 0x60b7778; continue; }
  pc = 0x60b7765; continue;
 }
 0x60b7765 => {
  { cpu.sub32(cpu.ecx, 0x2u32); }
  if cpu.zf { pc = 0x60b7778; continue; }
  pc = 0x60b776a; continue;
 }
 0x60b776a => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b7778; continue; }
  pc = 0x60b7774; continue;
 }
 0x60b7774 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b7778; continue;
 }
 0x60b7778 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b7759; continue; }
  pc = 0x60b7781; continue;
 }
 0x60b7781 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b77b8; continue; }
  pc = 0x60b7786; continue;
 }
 0x60b7786 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x60b7797; continue;
 }
 0x60b7797 => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if cpu.zf { pc = 0x60b77af; continue; }
  pc = 0x60b779c; continue;
 }
 0x60b779c => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x24u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b77af; continue; }
  pc = 0x60b77a1; continue;
 }
 0x60b77a1 => {
  { let v: u32 = cpu.r32(0x164u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.esi, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b77af; continue; }
  pc = 0x60b77ab; continue;
 }
 0x60b77ab => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  pc = 0x60b77af; continue;
 }
 0x60b77af => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b7797; continue; }
  pc = 0x60b77b8; continue;
 }
 0x60b77b8 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b77cf; continue; }
  pc = 0x60b77bd; continue;
 }
 0x60b77bd => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b78a0; continue;
 }
 0x60b77cf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b77e2); f_060ba8c0(cpu); if cpu.returned_to != 0x60b77e2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b7866; continue; }
  pc = 0x60b77f7; continue;
 }
 0x60b77f7 => {
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b7800; continue;
 }
 0x60b7800 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x1u32); }
  if cpu.zf { pc = 0x60b7847; continue; }
  pc = 0x60b7806; continue;
 }
 0x60b7806 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x60b780b; continue;
 }
 0x60b780b => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b7840; continue; }
  pc = 0x60b7816; continue;
 }
 0x60b7816 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x98u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)))); }
  if !cpu.zf { pc = 0x60b7840; continue; }
  pc = 0x60b781f; continue;
 }
 0x60b781f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b7833); f_060b9e10(cpu); if cpu.returned_to != 0x60b7833 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7840; continue;
 }
 0x60b7840 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x60b780b; continue; }
  pc = 0x60b7847; continue;
 }
 0x60b7847 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x71cu32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b7800; continue; }
  pc = 0x60b7866; continue;
 }
 0x60b7866 => {
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
  pc = 0x60b78a0; continue;
 }
 0x60b78a0 => {
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
pub fn f_060b78b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b78b0); }
 let mut pc: u32 = 0x60b78b0;
 loop { match pc {
 0x60b78b0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b78bf; continue; }
  pc = 0x60b78b8; continue;
 }
 0x60b78b8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b78bf; continue;
 }
 0x60b78bf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b78c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b78c0); }
 let mut pc: u32 = 0x60b78c0;
 loop { match pc {
 0x60b78c0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b78cb; continue; }
  pc = 0x60b78c8; continue;
 }
 0x60b78c8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b78cb => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6dcu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b78e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b78e0); }
 let mut pc: u32 = 0x60b78e0;
 loop { match pc {
 0x60b78e0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b78fb; continue; }
  pc = 0x60b78e8; continue;
 }
 0x60b78e8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x6dcu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b78fb; continue;
 }
 0x60b78fb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7900); }
 let mut pc: u32 = 0x60b7900;
 loop { match pc {
 0x60b7900 => {
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
pub fn f_060b7920(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7920); }
 let mut pc: u32 = 0x60b7920;
 loop { match pc {
 0x60b7920 => {
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
pub fn f_060b7960(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7960); }
 let mut pc: u32 = 0x60b7960;
 loop { match pc {
 0x60b7960 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b7975; continue; }
  pc = 0x60b7969; continue;
 }
 0x60b7969 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60b7975; continue;
 }
 0x60b7975 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b79c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b79c0); }
 let mut pc: u32 = 0x60b79c0;
 loop { match pc {
 0x60b79c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60b7a47; continue; }
  pc = 0x60b79d4; continue;
 }
 0x60b79d4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b7a47; continue; }
  pc = 0x60b79df; continue;
 }
 0x60b79df => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = 0xau32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.edx; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b7a03; continue; }
  pc = 0x60b79f0; continue;
 }
 0x60b79f0 => {
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  pc = 0x60b79f2; continue;
 }
 0x60b79f2 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if cpu.zf { pc = 0x60b7a03; continue; }
  pc = 0x60b79fb; continue;
 }
 0x60b79fb => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x6u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b79f2; continue; }
  pc = 0x60b7a03; continue;
 }
 0x60b7a03 => {
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x60b7a47; continue; }
  pc = 0x60b7a07; continue;
 }
 0x60b7a07 => {
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
  { let v: u32 = cpu.r32(0x62d1edcu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60b7a47; continue; }
  pc = 0x60b7a31; continue;
 }
 0x60b7a31 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = 0x10u32.wrapping_add(cpu.edi); cpu.edx = v; }
  pc = 0x60b7a36; continue;
 }
 0x60b7a36 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b7a36; continue; }
  pc = 0x60b7a47; continue;
 }
 0x60b7a47 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7a50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7a50); }
 let mut pc: u32 = 0x60b7a50;
 loop { match pc {
 0x60b7a50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b7abd; continue; }
  pc = 0x60b7a5c; continue;
 }
 0x60b7a5c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d1ed8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b7a9d; continue; }
  pc = 0x60b7a7b; continue;
 }
 0x60b7a7b => {
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x60b7a92; continue; }
  pc = 0x60b7a87; continue;
 }
 0x60b7a87 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x68u32.wrapping_add(cpu.edx))); }
  if (cpu.sf != cpu.of) { pc = 0x60b7a92; continue; }
  pc = 0x60b7a8f; continue;
 }
 0x60b7a8f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60b7a92; continue;
 }
 0x60b7a92 => {
  { let v: u32 = cpu.r32(0x62d1ed8u32); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b7a7b; continue; }
  pc = 0x60b7a9d; continue;
 }
 0x60b7a9d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x60b7aa3; continue; }
  pc = 0x60b7aa1; continue;
 }
 0x60b7aa1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b7aa3; continue;
 }
 0x60b7aa3 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b7aad; continue; }
  pc = 0x60b7aab; continue;
 }
 0x60b7aab => {
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b7aad; continue;
 }
 0x60b7aad => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b7ab7; continue; }
  pc = 0x60b7ab5; continue;
 }
 0x60b7ab5 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60b7ab7; continue;
 }
 0x60b7ab7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b7abd; continue;
 }
 0x60b7abd => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7ad0); }
 let mut pc: u32 = 0x60b7ad0;
 loop { match pc {
 0x60b7ad0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60b7add; continue; }
  pc = 0x60b7ad9; continue;
 }
 0x60b7ad9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b7add => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x40u32); cpu.ecx = v; }
  pc = 0x60b7ae9; continue;
 }
 0x60b7ae9 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x60b7af2; continue; }
  pc = 0x60b7af1; continue;
 }
 0x60b7af1 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x60b7af2; continue;
 }
 0x60b7af2 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b7ae9; continue; }
  pc = 0x60b7af9; continue;
 }
 0x60b7af9 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7dc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7dc0); }
 let mut pc: u32 = 0x60b7dc0;
 loop { match pc {
 0x60b7dc0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b7dc9; continue; }
  pc = 0x60b7dc8; continue;
 }
 0x60b7dc8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b7dc9 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7dd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7dd0); }
 let mut pc: u32 = 0x60b7dd0;
 loop { match pc {
 0x60b7dd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b7e49; continue; }
  pc = 0x60b7ddf; continue;
 }
 0x60b7ddf => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x60b7e00; continue; }
  pc = 0x60b7df6; continue;
 }
 0x60b7df6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b7e00 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b7e09; continue; }
  pc = 0x60b7e04; continue;
 }
 0x60b7e04 => {
  { let v: u32 = cpu.esi; cpu.w32(0x3cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7e36; continue;
 }
 0x60b7e09 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b7e23; continue; }
  pc = 0x60b7e21; continue;
 }
 0x60b7e21 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60b7e23; continue;
 }
 0x60b7e23 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x44u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x48u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7e36; continue;
 }
 0x60b7e36 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b7e3c); f_060b9950(cpu); if cpu.returned_to != 0x60b7e3c { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b7e49; continue;
 }
 0x60b7e49 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7e50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7e50); }
 let mut pc: u32 = 0x60b7e50;
 loop { match pc {
 0x60b7e50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b7eb1; continue; }
  pc = 0x60b7e5e; continue;
 }
 0x60b7e5e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x50u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x60b7eab; continue; }
  pc = 0x60b7e75; continue;
 }
 0x60b7e75 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b7e7e; continue; }
  pc = 0x60b7e79; continue;
 }
 0x60b7e79 => {
  { let v: u32 = cpu.esi; cpu.w32(0x4cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7eab; continue;
 }
 0x60b7e7e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 5u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 3u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x60b7e98; continue; }
  pc = 0x60b7e96; continue;
 }
 0x60b7e96 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x60b7e98; continue;
 }
 0x60b7e98 => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x54u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b7eab; continue;
 }
 0x60b7eab => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b7eb1; continue;
 }
 0x60b7eb1 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7ec0); }
 let mut pc: u32 = 0x60b7ec0;
 loop { match pc {
 0x60b7ec0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b7ecf; continue; }
  pc = 0x60b7ec8; continue;
 }
 0x60b7ec8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b7ecf; continue;
 }
 0x60b7ecf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7ed0); }
 let mut pc: u32 = 0x60b7ed0;
 loop { match pc {
 0x60b7ed0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b7f02; continue; }
  pc = 0x60b7eda; continue;
 }
 0x60b7eda => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x60b7f02; continue; }
  pc = 0x60b7ee2; continue;
 }
 0x60b7ee2 => {
  { cpu.sub32(cpu.edx, 0x2u32); }
  if cpu.zf { pc = 0x60b7f02; continue; }
  pc = 0x60b7ee7; continue;
 }
 0x60b7ee7 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b7eed); f_060a9000(cpu); if cpu.returned_to != 0x60b7eed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b7f02; continue; }
  pc = 0x60b7efe; continue;
 }
 0x60b7efe => {
  { let v = cpu.ebx; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x60b7f02); cpu.call(t); if cpu.returned_to != 0x60b7f02 { return; }
  pc = 0x60b7f02; continue;
 }
 0x60b7f02 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7f10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7f10); }
 let mut pc: u32 = 0x60b7f10;
 loop { match pc {
 0x60b7f10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60b7f6b; continue; }
  pc = 0x60b7f1c; continue;
 }
 0x60b7f1c => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edi)), 0x8u32); }
  if !cpu.zf { pc = 0x60b7f6b; continue; }
  pc = 0x60b7f22; continue;
 }
 0x60b7f22 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60b7f29; continue;
 }
 0x60b7f29 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.esi)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b7f49; continue; }
  pc = 0x60b7f32; continue;
 }
 0x60b7f32 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b7f38); f_060a70a0(cpu); if cpu.returned_to != 0x60b7f38 { return; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b7f43; continue; }
  pc = 0x60b7f41; continue;
 }
 0x60b7f41 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x60b7f43; continue;
 }
 0x60b7f43 => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.esi), v); }
  pc = 0x60b7f49; continue;
 }
 0x60b7f49 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b7f29; continue; }
  pc = 0x60b7f52; continue;
 }
 0x60b7f52 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x60b7f54; continue;
 }
 0x60b7f54 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b7f5b); f_060b9a70(cpu); if cpu.returned_to != 0x60b7f5b { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b7f54; continue; }
  pc = 0x60b7f64; continue;
 }
 0x60b7f64 => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x60b7f6b; continue;
 }
 0x60b7f6b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b7f70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b7f70); }
 let mut pc: u32 = 0x60b7f70;
 loop { match pc {
 0x60b7f70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60b800c; continue; }
  pc = 0x60b7f80; continue;
 }
 0x60b7f80 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebp)), 0x4u32); }
  if !cpu.zf { pc = 0x60b800c; continue; }
  pc = 0x60b7f8a; continue;
 }
 0x60b7f8a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = 0x8u32; cpu.w32(0x8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ebp.wrapping_add(0x40u32); cpu.ebp = v; }
  cpu.push(0x60b7f9c); f_060b9d50(cpu); if cpu.returned_to != 0x60b7f9c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xffffffc4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x60b7fa2; continue;
 }
 0x60b7fa2 => {
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b7fc5; continue; }
  pc = 0x60b7fb3; continue;
 }
 0x60b7fb3 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b7fc2); f_060ba8c0(cpu); if cpu.returned_to != 0x60b7fc2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60b7fc5; continue;
 }
 0x60b7fc5 => {
  { cpu.sub32(cpu.r32(0x1d8u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b7fd6; continue; }
  pc = 0x60b7fce; continue;
 }
 0x60b7fce => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x60b7fd6; continue;
 }
 0x60b7fd6 => {
  { cpu.sub32(cpu.r32(0x258u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b7ff1; continue; }
  pc = 0x60b7fdf; continue;
 }
 0x60b7fdf => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b7fee); f_060ba8c0(cpu); if cpu.returned_to != 0x60b7fee { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60b7ff1; continue;
 }
 0x60b7ff1 => {
  { cpu.sub32(cpu.r32(0x198u32.wrapping_add(cpu.ebx)), 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b8005; continue; }
  pc = 0x60b7ffa; continue;
 }
 0x60b7ffa => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b8002); f_060a6fe0(cpu); if cpu.returned_to != 0x60b8002 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  pc = 0x60b8005; continue;
 }
 0x60b8005 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x60b7fa2; continue; }
  pc = 0x60b800c; continue;
 }
 0x60b800c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b8020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8020); }
 let mut pc: u32 = 0x60b8020;
 loop { match pc {
 0x60b8020 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b8051; continue; }
  pc = 0x60b8029; continue;
 }
 0x60b8029 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x1u32); }
  if cpu.zf { pc = 0x60b8051; continue; }
  pc = 0x60b802f; continue;
 }
 0x60b802f => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b8035); f_060a9000(cpu); if cpu.returned_to != 0x60b8035 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b803e); f_060b99b0(cpu); if cpu.returned_to != 0x60b803e { return; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b8051; continue;
 }
 0x60b8051 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b8060(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8060); }
 let mut pc: u32 = 0x60b8060;
 loop { match pc {
 0x60b8060 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b8076; continue; }
  pc = 0x60b806f; continue;
 }
 0x60b806f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b8477; continue;
 }
 0x60b8076 => {
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60b808c); f_060ba630(cpu); if cpu.returned_to != 0x60b808c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b80b3; continue; }
  pc = 0x60b8093; continue;
 }
 0x60b8093 => {
  { let v: u32 = 0x616b104u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b809f; continue;
 }
 0x60b809f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b809f; continue; }
  pc = 0x60b80a9; continue;
 }
 0x60b80a9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b80b3 => {
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
  if !cpu.cf { pc = 0x60b81b2; continue; }
  pc = 0x60b8110; continue;
 }
 0x60b8110 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616b11cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b8126; continue; }
  pc = 0x60b8124; continue;
 }
 0x60b8124 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8126; continue;
 }
 0x60b8126 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b8131; continue; }
  pc = 0x60b812a; continue;
 }
 0x60b812a => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b8131; continue;
 }
 0x60b8131 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616b124u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b8147; continue; }
  pc = 0x60b8145; continue;
 }
 0x60b8145 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8147; continue;
 }
 0x60b8147 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b8152; continue; }
  pc = 0x60b814b; continue;
 }
 0x60b814b => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b8152; continue;
 }
 0x60b8152 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616b12cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b8168; continue; }
  pc = 0x60b8166; continue;
 }
 0x60b8166 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8168; continue;
 }
 0x60b8168 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b8173; continue; }
  pc = 0x60b816c; continue;
 }
 0x60b816c => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b8173; continue;
 }
 0x60b8173 => {
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
  if cpu.cf { cpu.poll(); pc = 0x60b8110; continue; }
  pc = 0x60b81b2; continue;
 }
 0x60b81b2 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.eax)), 0x0u32); }
  if !cpu.zf { pc = 0x60b81dc; continue; }
  pc = 0x60b81bc; continue;
 }
 0x60b81bc => {
  { let v: u32 = 0x616b104u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b81c8; continue;
 }
 0x60b81c8 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b81c8; continue; }
  pc = 0x60b81d2; continue;
 }
 0x60b81d2 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b81dc => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b820e); f_060b99b0(cpu); if cpu.returned_to != 0x60b820e { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x62d1ed4u32); cpu.eax = v; }
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
  if cpu.zf { pc = 0x60b8472; continue; }
  pc = 0x60b8265; continue;
 }
 0x60b8265 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b826e); f_060ba780(cpu); if cpu.returned_to != 0x60b826e { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60b8282; continue; }
  pc = 0x60b827b; continue;
 }
 0x60b827b => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x60b8296; continue;
 }
 0x60b8282 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b8290); f_060ba780(cpu); if cpu.returned_to != 0x60b8290 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x60b8296; continue;
 }
 0x60b8296 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x62d1f7cu32; cpu.edi = v; }
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
  { let v: u32 = 0x62d1f7cu32; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60b836a; continue; }
  pc = 0x60b82ca; continue;
 }
 0x60b82ca => {
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b82ce; continue;
 }
 0x60b82ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if !cpu.cf { pc = 0x60b836a; continue; }
  pc = 0x60b82dc; continue;
 }
 0x60b82dc => {
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
  let t = cpu.r32(0x1a8u32.wrapping_add(cpu.eax)); cpu.push(0x60b8305); cpu.call(t); if cpu.returned_to != 0x60b8305 { return; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b8313; continue; }
  pc = 0x60b830c; continue;
 }
 0x60b830c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  cpu.poll(); pc = 0x60b82ce; continue;
 }
 0x60b8313 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x62d1f7cu32); cpu.eax = v; }
  pc = 0x60b832f; continue;
 }
 0x60b8323 => {
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b832f; continue;
 }
 0x60b832f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if cpu.cf { cpu.poll(); pc = 0x60b8323; continue; }
  pc = 0x60b8339; continue;
 }
 0x60b8339 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x5u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x8u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.cf { pc = 0x60b835d; continue; }
  pc = 0x60b8348; continue;
 }
 0x60b8348 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.dec8((cpu.ebx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x4u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60b82ce; continue;
 }
 0x60b835d => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8).wrapping_sub(0x2u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x5u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x60b82ce; continue;
 }
 0x60b836a => {
  { cpu.sub16(cpu.r16(0x8u32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x60b8472; continue; }
  pc = 0x60b8375; continue;
 }
 0x60b8375 => {
  cpu.push(0x60b837a); f_060acba0(cpu); if cpu.returned_to != 0x60b837a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b83a1; continue; }
  pc = 0x60b837e; continue;
 }
 0x60b837e => {
  { let v: u32 = 0x616b134u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b838a; continue;
 }
 0x60b838a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b838a; continue; }
  pc = 0x60b8394; continue;
 }
 0x60b8394 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b83a1 => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b83f7; continue; }
  pc = 0x60b83bf; continue;
 }
 0x60b83bf => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b83e0); f_060acc60(cpu); if cpu.returned_to != 0x60b83e0 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b83f7; continue;
 }
 0x60b83f7 => {
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
  cpu.push(0x60b8438); f_060acc60(cpu); if cpu.returned_to != 0x60b8438 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x18u32.wrapping_add(cpu.eax))); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.sub16(cpu.r16(0xcu32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x60b8472; continue; }
  pc = 0x60b844d; continue;
 }
 0x60b844d => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 8u32); cpu.eax = v; }
  cpu.push(0x60b8465); f_060b8480(cpu); if cpu.returned_to != 0x60b8465 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8472 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60b8477; continue;
 }
 0x60b8477 => {
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
pub fn f_060b8480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8480); }
 let mut pc: u32 = 0x60b8480;
 loop { match pc {
 0x60b8480 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v = 0x62d1da0u32; cpu.push(v); }
  { let v = 0x616b0d0u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x617b958u32); cpu.push(0x60b84a4); cpu.call(t); if cpu.returned_to != 0x60b84a4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b84b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b84b0); }
 let mut pc: u32 = 0x60b84b0;
 loop { match pc {
 0x60b84b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b84c9; continue; }
  pc = 0x60b84b9; continue;
 }
 0x60b84b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b84bf); f_060a9000(cpu); if cpu.returned_to != 0x60b84bf { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b84c9; continue;
 }
 0x60b84c9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b84d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b84d0); }
 let mut pc: u32 = 0x60b84d0;
 loop { match pc {
 0x60b84d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x60b84dc); f_060ad100(cpu); if cpu.returned_to != 0x60b84dc { return; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b84fc; continue; }
  pc = 0x60b84e5; continue;
 }
 0x60b84e5 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x60b84e8; continue;
 }
 0x60b84e8 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x1u32); }
  if cpu.zf { pc = 0x60b84fc; continue; }
  pc = 0x60b84ee; continue;
 }
 0x60b84ee => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x71cu32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b84e8; continue; }
  pc = 0x60b84fc; continue;
 }
 0x60b84fc => {
  { cpu.sub32(cpu.eax, cpu.r32(0x20u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x60b8522; continue; }
  pc = 0x60b8501; continue;
 }
 0x60b8501 => {
  { let v: u32 = 0x616b0b4u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b850d; continue;
 }
 0x60b850d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b850d; continue; }
  pc = 0x60b8517; continue;
 }
 0x60b8517 => {
  cpu.push(0x60b851c); f_060ad0f0(cpu); if cpu.returned_to != 0x60b851c { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8522 => {
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
  cpu.push(0x60b8545); f_060b99b0(cpu); if cpu.returned_to != 0x60b8545 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x2cu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b8554); f_060ad0f0(cpu); if cpu.returned_to != 0x60b8554 { return; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b8560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8560); }
 let mut pc: u32 = 0x60b8560;
 loop { match pc {
 0x60b8560 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = 0x617c6bcu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b856e; continue;
 }
 0x60b856e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b856e; continue; }
  pc = 0x60b8578; continue;
 }
 0x60b8578 => {
  { let v: u32 = 0x617c6bcu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b85d1; continue; }
  pc = 0x60b8590; continue;
 }
 0x60b8590 => {
  { let v: u8 = cpu.r8(0x617c6bcu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x5cu8); }
  if !cpu.zf { pc = 0x60b85bd; continue; }
  pc = 0x60b859b; continue;
 }
 0x60b859b => {
  { let v: u32 = 0x617c6bcu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if !cpu.zf { pc = 0x60b85d1; continue; }
  pc = 0x60b85b1; continue;
 }
 0x60b85b1 => {
  { let v: u8 = cpu.xor8((cpu.ecx as u8), (cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x617c6bcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b85bd => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x2eu8); }
  if !cpu.zf { pc = 0x60b85ce; continue; }
  pc = 0x60b85c2; continue;
 }
 0x60b85c2 => {
  { let v: u8 = cpu.xor8(((cpu.ebx >> 8) as u8), ((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8); cpu.w8(0x617c6bcu32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b85ce => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60b8590; continue; }
  pc = 0x60b85d1; continue;
 }
 0x60b85d1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b85e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b85e0); }
 let mut pc: u32 = 0x60b85e0;
 loop { match pc {
 0x60b85e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.edx)), 0x112u32); }
  if cpu.cf { pc = 0x60b8684; continue; }
  pc = 0x60b85fa; continue;
 }
 0x60b85fa => {
  { let v: u32 = cpu.edx.wrapping_add(0xbau32); cpu.edx = v; }
  { let v: u32 = 0x616b078u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b8607; continue;
 }
 0x60b8607 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b8615; continue; }
  pc = 0x60b860f; continue;
 }
 0x60b860f => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b8615; continue; }
  pc = 0x60b8613; continue;
 }
 0x60b8613 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8615; continue;
 }
 0x60b8615 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b8622; continue; }
  pc = 0x60b861a; continue;
 }
 0x60b861a => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b8622; continue; }
  pc = 0x60b861f; continue;
 }
 0x60b861f => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b8622; continue;
 }
 0x60b8622 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b862e; continue; }
  pc = 0x60b8626; continue;
 }
 0x60b8626 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b862e; continue; }
  pc = 0x60b862a; continue;
 }
 0x60b862a => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b8607; continue;
 }
 0x60b862e => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b8635; continue; }
  pc = 0x60b8633; continue;
 }
 0x60b8633 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8635; continue;
 }
 0x60b8635 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b8642; continue; }
  pc = 0x60b8639; continue;
 }
 0x60b8639 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8642 => {
  { let v: u32 = 0x616b08cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b8649; continue;
 }
 0x60b8649 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b8657; continue; }
  pc = 0x60b8651; continue;
 }
 0x60b8651 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b8657; continue; }
  pc = 0x60b8655; continue;
 }
 0x60b8655 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8657; continue;
 }
 0x60b8657 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b8664; continue; }
  pc = 0x60b865c; continue;
 }
 0x60b865c => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b8664; continue; }
  pc = 0x60b8661; continue;
 }
 0x60b8661 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b8664; continue;
 }
 0x60b8664 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b8670; continue; }
  pc = 0x60b8668; continue;
 }
 0x60b8668 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b8670; continue; }
  pc = 0x60b866c; continue;
 }
 0x60b866c => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b8649; continue;
 }
 0x60b8670 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b8677; continue; }
  pc = 0x60b8675; continue;
 }
 0x60b8675 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b8677; continue;
 }
 0x60b8677 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b8684; continue; }
  pc = 0x60b867b; continue;
 }
 0x60b867b => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8684 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 4u32); cpu.ecx = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  if cpu.zf { pc = 0x60b873a; continue; }
  pc = 0x60b86a1; continue;
 }
 0x60b86a1 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x60b873a; continue; }
  pc = 0x60b86aa; continue;
 }
 0x60b86aa => {
  { let v: u32 = 0x616b0a8u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b86b1; continue;
 }
 0x60b86b1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b86bf; continue; }
  pc = 0x60b86b9; continue;
 }
 0x60b86b9 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b86bf; continue; }
  pc = 0x60b86bd; continue;
 }
 0x60b86bd => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b86bf; continue;
 }
 0x60b86bf => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b86cc; continue; }
  pc = 0x60b86c4; continue;
 }
 0x60b86c4 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b86cc; continue; }
  pc = 0x60b86c9; continue;
 }
 0x60b86c9 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b86cc; continue;
 }
 0x60b86cc => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b86d8; continue; }
  pc = 0x60b86d0; continue;
 }
 0x60b86d0 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b86d8; continue; }
  pc = 0x60b86d4; continue;
 }
 0x60b86d4 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b86b1; continue;
 }
 0x60b86d8 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b86df; continue; }
  pc = 0x60b86dd; continue;
 }
 0x60b86dd => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b86df; continue;
 }
 0x60b86df => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b86f8; continue; }
  pc = 0x60b86e3; continue;
 }
 0x60b86e3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.eax)), 0x5000u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b873a; continue; }
  pc = 0x60b86ef; continue;
 }
 0x60b86ef => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b86f8 => {
  { let v: u32 = 0x616b0acu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x60b86ff; continue;
 }
 0x60b86ff => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b870d; continue; }
  pc = 0x60b8707; continue;
 }
 0x60b8707 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b870d; continue; }
  pc = 0x60b870b; continue;
 }
 0x60b870b => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b870d; continue;
 }
 0x60b870d => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60b871a; continue; }
  pc = 0x60b8712; continue;
 }
 0x60b8712 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60b871a; continue; }
  pc = 0x60b8717; continue;
 }
 0x60b8717 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60b871a; continue;
 }
 0x60b871a => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60b8726; continue; }
  pc = 0x60b871e; continue;
 }
 0x60b871e => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60b8726; continue; }
  pc = 0x60b8722; continue;
 }
 0x60b8722 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60b86ff; continue;
 }
 0x60b8726 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60b872d; continue; }
  pc = 0x60b872b; continue;
 }
 0x60b872b => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60b872d; continue;
 }
 0x60b872d => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60b873a; continue; }
  pc = 0x60b8731; continue;
 }
 0x60b8731 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b873a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b8740(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8740); }
 let mut pc: u32 = 0x60b8740;
 loop { match pc {
 0x60b8740 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60b874d); f_060abcf0(cpu); if cpu.returned_to != 0x60b874d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b8760(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8760); }
 let mut pc: u32 = 0x60b8760;
 loop { match pc {
 0x60b8760 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x118u32); cpu.esp = v; }
  { let v = 0x616b050u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8777); f_060ac870(cpu); if cpu.returned_to != 0x60b8777 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b879b; continue; }
  pc = 0x60b877e; continue;
 }
 0x60b877e => {
  { let v: u32 = 0x616b058u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b878a; continue;
 }
 0x60b878a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b878a; continue; }
  pc = 0x60b8794; continue;
 }
 0x60b8794 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60b87d0; continue;
 }
 0x60b879b => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x84u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b87b0); f_060a94e0(cpu); if cpu.returned_to != 0x60b87b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x124u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b87ce; continue; }
  pc = 0x60b87c0; continue;
 }
 0x60b87c0 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b87ce => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b87d0; continue;
 }
 0x60b87d0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x118u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b87e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b87e0); }
 let mut pc: u32 = 0x60b87e0;
 loop { match pc {
 0x60b87e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b87f1); f_060abda0(cpu); if cpu.returned_to != 0x60b87f1 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b8819; continue; }
  pc = 0x60b87fa; continue;
 }
 0x60b87fa => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8805); f_060b8970(cpu); if cpu.returned_to != 0x60b8805 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b8817; continue; }
  pc = 0x60b880e; continue;
 }
 0x60b880e => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b8814); f_060abcf0(cpu); if cpu.returned_to != 0x60b8814 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60b8817; continue;
 }
 0x60b8817 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60b8819; continue;
 }
 0x60b8819 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b8820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8820); }
 let mut pc: u32 = 0x60b8820;
 loop { match pc {
 0x60b8820 => {
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
  cpu.push(0x60b8844); f_060b2f10(cpu); if cpu.returned_to != 0x60b8844 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b8902; continue; }
  pc = 0x60b8851; continue;
 }
 0x60b8851 => {
  { cpu.sub8(cpu.r8(0x617c528u32), 0x0u8); }
  if cpu.zf { pc = 0x60b88ea; continue; }
  pc = 0x60b885e; continue;
 }
 0x60b885e => {
  { let v: u32 = 0x617c528u32; cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8867; continue;
 }
 0x60b8867 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8867; continue; }
  pc = 0x60b8871; continue;
 }
 0x60b8871 => {
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b8882; continue;
 }
 0x60b8882 => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8882; continue; }
  pc = 0x60b8888; continue;
 }
 0x60b8888 => {
  { let v: u32 = 0x617c6bcu32; cpu.esi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8896; continue;
 }
 0x60b8896 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8896; continue; }
  pc = 0x60b88a0; continue;
 }
 0x60b88a0 => {
  { let v: u32 = 0x617c528u32; cpu.esi = v; }
  { let v: u32 = 0x617c6bcu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b88ac; continue;
 }
 0x60b88ac => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b88ac; continue; }
  pc = 0x60b88b6; continue;
 }
 0x60b88b6 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x617c6bcu32; cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b88cf; continue;
 }
 0x60b88cf => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b88cf; continue; }
  pc = 0x60b88d5; continue;
 }
 0x60b88d5 => {
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b88e1); f_060b2f10(cpu); if cpu.returned_to != 0x60b88e1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b8902; continue; }
  pc = 0x60b88ea; continue;
 }
 0x60b88ea => {
  { let v: u32 = 0x616b038u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b88f6; continue;
 }
 0x60b88f6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b88f6; continue; }
  pc = 0x60b8900; continue;
 }
 0x60b8900 => {
  pc = 0x60b8940; continue;
 }
 0x60b8902 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b890e); f_060abda0(cpu); if cpu.returned_to != 0x60b890e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x60b8919); f_060b32d0(cpu); if cpu.returned_to != 0x60b8919 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b8940; continue; }
  pc = 0x60b8920; continue;
 }
 0x60b8920 => {
  { let v: u32 = cpu.r32(0x198u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b892e); f_060b8970(cpu); if cpu.returned_to != 0x60b892e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b8940; continue; }
  pc = 0x60b8937; continue;
 }
 0x60b8937 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b893d); f_060abcf0(cpu); if cpu.returned_to != 0x60b893d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b8940; continue;
 }
 0x60b8940 => {
  { cpu.sub8(cpu.r8(0x100u32.wrapping_add(cpu.esp)), 0x0u8); }
  if cpu.zf { pc = 0x60b8962; continue; }
  pc = 0x60b894a; continue;
 }
 0x60b894a => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x617c6bcu32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8958; continue;
 }
 0x60b8958 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8958; continue; }
  pc = 0x60b8962; continue;
 }
 0x60b8962 => {
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
pub fn f_060b8970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b8970); }
 let mut pc: u32 = 0x60b8970;
 loop { match pc {
 0x60b8970 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x40u32); cpu.esp = v; }
  cpu.push(0x60b897c); f_060baaf0(cpu); if cpu.returned_to != 0x60b897c { return; }
  { let v = 0x1d8u32; cpu.push(v); }
  cpu.push(0x60b8986); f_060b3300(cpu); if cpu.returned_to != 0x60b8986 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b89ac; continue; }
  pc = 0x60b898f; continue;
 }
 0x60b898f => {
  { let v: u32 = 0x616af5cu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b899b; continue;
 }
 0x60b899b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b899b; continue; }
  pc = 0x60b89a5; continue;
 }
 0x60b89a5 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b903a; continue;
 }
 0x60b89ac => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616af84u32; cpu.esi = v; }
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
  if cpu.zf { pc = 0x60b8a04; continue; }
  pc = 0x60b89d9; continue;
 }
 0x60b89d9 => {
  { let v: u32 = 0x616af8cu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b89e7; continue;
 }
 0x60b89e7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b89e7; continue; }
  pc = 0x60b89f1; continue;
 }
 0x60b89f1 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b89f7); f_060b32d0(cpu); if cpu.returned_to != 0x60b89f7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8a04 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x301u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8a16); f_060acc60(cpu); if cpu.returned_to != 0x60b8a16 { return; }
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
  if cpu.zf { pc = 0x60b8aae; continue; }
  pc = 0x60b8a5f; continue;
 }
 0x60b8a5f => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x60b8aae; continue; }
  pc = 0x60b8a64; continue;
 }
 0x60b8a64 => {
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x617b950u32); cpu.push(0x60b8a6b); cpu.call(t); if cpu.returned_to != 0x60b8a6b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b8aae; continue; }
  pc = 0x60b8a71; continue;
 }
 0x60b8a71 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if cpu.zf { pc = 0x60b8aae; continue; }
  pc = 0x60b8a76; continue;
 }
 0x60b8a76 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b8a90; continue; }
  pc = 0x60b8a8b; continue;
 }
 0x60b8a8b => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x60b8a90; continue;
 }
 0x60b8a90 => {
  { let v: u32 = cpu.r32(0x62d1f50u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u8 = (cpu.ebx as u8) & 0x3u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.movs(4, true); }
  { let v: u8 = (cpu.ebx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.movs(1, true); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x60b8aae; continue;
 }
 0x60b8aae => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if cpu.zf { pc = 0x60b8b06; continue; }
  pc = 0x60b8ac7; continue;
 }
 0x60b8ac7 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x60b8b06; continue; }
  pc = 0x60b8acc; continue;
 }
 0x60b8acc => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x617c6bcu32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8adc; continue;
 }
 0x60b8adc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8adc; continue; }
  pc = 0x60b8ae6; continue;
 }
 0x60b8ae6 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x80u32); cpu.edi = v; }
  { cpu.df = false; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x60b8afe; continue;
 }
 0x60b8afe => {
  { cpu.lods(1, false); }
  { cpu.stos(1, false); }
  { cpu.and8((cpu.eax as u8), 0xffu8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8afe; continue; }
  pc = 0x60b8b04; continue;
 }
 0x60b8b04 => {
  pc = 0x60b8b10; continue;
 }
 0x60b8b06 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x80u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b8b10; continue;
 }
 0x60b8b10 => {
  { let v: u32 = 0x18u32; cpu.ecx = v; }
  { let v: u8 = 0xffu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
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
  if cpu.zf { pc = 0x60b8b77; continue; }
  pc = 0x60b8b43; continue;
 }
 0x60b8b43 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8b5a); f_060b90b0(cpu); if cpu.returned_to != 0x60b8b5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b8b77; continue; }
  pc = 0x60b8b61; continue;
 }
 0x60b8b61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { cpu.movs(4, true); }
  pc = 0x60b8b77; continue;
 }
 0x60b8b77 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b8bc0; continue; }
  pc = 0x60b8b7e; continue;
 }
 0x60b8b7e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b8b87); f_060abeb0(cpu); if cpu.returned_to != 0x60b8b87 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b8bc0; continue; }
  pc = 0x60b8b90; continue;
 }
 0x60b8b90 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.movs(4, true); }
  cpu.push(0x60b8ba5); f_060b90b0(cpu); if cpu.returned_to != 0x60b8ba5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b8bc0; continue; }
  pc = 0x60b8bac; continue;
 }
 0x60b8bac => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x60b8bc0; continue;
 }
 0x60b8bc0 => {
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60b8c5b; continue; }
  pc = 0x60b8bcc; continue;
 }
 0x60b8bcc => {
  { cpu.sub32(cpu.r32(0x62d1ee4u32), 0x1u32); }
  if !cpu.zf { pc = 0x60b8c5b; continue; }
  pc = 0x60b8bd9; continue;
 }
 0x60b8bd9 => {
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b8c47; continue;
 }
 0x60b8be3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60b8c0f; continue; }
  pc = 0x60b8c01; continue;
 }
 0x60b8c01 => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { cpu.movs(4, true); }
  pc = 0x60b8c0f; continue;
 }
 0x60b8c0f => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8c16); f_060b90b0(cpu); if cpu.returned_to != 0x60b8c16 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60b8c33; continue; }
  pc = 0x60b8c1d; continue;
 }
 0x60b8c1d => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.esp; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x30u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  pc = 0x60b8c5b; continue;
 }
 0x60b8c33 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x34u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b8c47; continue;
 }
 0x60b8c47 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b8be3; continue; }
  pc = 0x60b8c5b; continue;
 }
 0x60b8c5b => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60b8c8b; continue; }
  pc = 0x60b8c62; continue;
 }
 0x60b8c62 => {
  { let v: u32 = 0x616afa4u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8c6e; continue;
 }
 0x60b8c6e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8c6e; continue; }
  pc = 0x60b8c78; continue;
 }
 0x60b8c78 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8c7e); f_060b32d0(cpu); if cpu.returned_to != 0x60b8c7e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8c8b => {
  { let v: u32 = 0x6u32; cpu.ecx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = 0x62d1ef0u32; cpu.edi = v; }
  { let v = 0x0u32; cpu.push(v); }
  { cpu.movs(4, true); }
  { let v = 0x305u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b8cad); f_060acc60(cpu); if cpu.returned_to != 0x60b8cad { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x501u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8ccf); f_060acc60(cpu); if cpu.returned_to != 0x60b8ccf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x18u32.wrapping_add(cpu.esp)), 0x0u16); }
  if !cpu.zf { pc = 0x60b8d22; continue; }
  pc = 0x60b8cda; continue;
 }
 0x60b8cda => {
  { let v: u32 = 0x616afc4u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8ce6; continue;
 }
 0x60b8ce6 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8ce6; continue; }
  pc = 0x60b8cf0; continue;
 }
 0x60b8cf0 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b8d02); f_060acc60(cpu); if cpu.returned_to != 0x60b8d02 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b8d15); f_060b32d0(cpu); if cpu.returned_to != 0x60b8d15 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8d22 => {
  { let v: u32 = cpu.r32(0x62d1ed0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 6u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8d41); f_060b3300(cpu); if cpu.returned_to != 0x60b8d41 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b8d93; continue; }
  pc = 0x60b8d4b; continue;
 }
 0x60b8d4b => {
  { let v: u32 = 0x616aff0u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8d57; continue;
 }
 0x60b8d57 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8d57; continue; }
  pc = 0x60b8d61; continue;
 }
 0x60b8d61 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b8d73); f_060acc60(cpu); if cpu.returned_to != 0x60b8d73 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b8d86); f_060b32d0(cpu); if cpu.returned_to != 0x60b8d86 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8d93 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b8de7; continue; }
  pc = 0x60b8da0; continue;
 }
 0x60b8da0 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60b8da2; continue;
 }
 0x60b8da2 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x616b018u32; cpu.esi = v; }
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b8da2; continue; }
  pc = 0x60b8de7; continue;
 }
 0x60b8de7 => {
  { let v: u32 = 0xf4240u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d1eccu32); cpu.ebx = v; }
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
  pc = 0x60b8e3c; continue;
 }
 0x60b8e3c => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x60u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xa0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xe0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x120u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x160u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8e3c; continue; }
  pc = 0x60b8e61; continue;
 }
 0x60b8e61 => {
  { let v = 0x60b9190u32; cpu.push(v); }
  cpu.push(0x60b8e6b); f_060ac770(cpu); if cpu.returned_to != 0x60b8e6b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60b8eca; continue; }
  pc = 0x60b8e76; continue;
 }
 0x60b8e76 => {
  { let v: u32 = 0x616b020u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b8e82; continue;
 }
 0x60b8e82 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b8e82; continue; }
  pc = 0x60b8e8c; continue;
 }
 0x60b8e8c => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x306u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b8e9e); f_060acc60(cpu); if cpu.returned_to != 0x60b8e9e { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8eb4); f_060b32d0(cpu); if cpu.returned_to != 0x60b8eb4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8ebd); f_060b32d0(cpu); if cpu.returned_to != 0x60b8ebd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x40u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60b8eca => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b8ed1); f_060ac660(cpu); if cpu.returned_to != 0x60b8ed1 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = 0x60b9050u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x60b8ee8; continue;
 }
 0x60b8ee8 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8ef7); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8ef7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f07); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f07 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f18); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f18 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x70u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f25); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f25 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f32); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d1ed4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x7u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f44); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f44 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v = 0xau32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f52); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x7fu32; cpu.push(v); }
  { let v = 0xbu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f60); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f60 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x40u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f6d); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f6d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = 0x28u32; cpu.push(v); }
  { let v = 0x5bu32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f7b); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x5du32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f88); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f88 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x64u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8f95); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8f95 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x65u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8fa2); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8fa2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = 0x26u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8faf); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8faf { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d1ee0u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b8fc2); f_060ba8c0(cpu); if cpu.returned_to != 0x60b8fc2 { return; }
  { let v: u32 = cpu.r32(0x1acu32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.sub32(cpu.edi, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x60b8ffe; continue; }
  pc = 0x60b8fcf; continue;
 }
 0x60b8fcf => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x28u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b8fef); f_060acc60(cpu); if cpu.returned_to != 0x60b8fef { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60b8ffe; continue;
 }
 0x60b8ffe => {
  { cpu.and16((cpu.esi as u16), 0x3u16); }
  if !cpu.zf { pc = 0x60b900f; continue; }
  pc = 0x60b9005; continue;
 }
 0x60b9005 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60b900c); f_060acbb0(cpu); if cpu.returned_to != 0x60b900c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b900f; continue;
 }
 0x60b900f => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b8ee8; continue; }
  pc = 0x60b9019; continue;
 }
 0x60b9019 => {
  { let v: u32 = cpu.r32(0x62d1eccu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b9029); f_060ac4e0(cpu); if cpu.returned_to != 0x60b9029 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b9035); f_060ac290(cpu); if cpu.returned_to != 0x60b9035 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x60b903a; continue;
 }
 0x60b903a => {
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
pub fn f_060b9050(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9050); }
 let mut pc: u32 = 0x60b9050;
 loop { match pc {
 0x60b9050 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b907f; continue; }
  pc = 0x60b9061; continue;
 }
 0x60b9061 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b9063; continue;
 }
 0x60b9063 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x60b906f); f_060a8ea0(cpu); if cpu.returned_to != 0x60b906f { return; }
  { let v: u32 = cpu.esi.wrapping_add(0x71cu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b9063; continue; }
  pc = 0x60b907f; continue;
 }
 0x60b907f => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9088); f_060ac030(cpu); if cpu.returned_to != 0x60b9088 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60b9094); f_060b32d0(cpu); if cpu.returned_to != 0x60b9094 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b909d); f_060b32d0(cpu); if cpu.returned_to != 0x60b909d { return; }
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
pub fn f_060b90b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b90b0); }
 let mut pc: u32 = 0x60b90b0;
 loop { match pc {
 0x60b90b0 => {
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
  if (cpu.cf || cpu.zf) { pc = 0x60b9157; continue; }
  pc = 0x60b90d9; continue;
 }
 0x60b90d9 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b9100; continue; }
  pc = 0x60b90f9; continue;
 }
 0x60b90f9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b9100; continue;
 }
 0x60b9100 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b9114; continue; }
  pc = 0x60b910b; continue;
 }
 0x60b910b => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b9114; continue;
 }
 0x60b9114 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b9129; continue; }
  pc = 0x60b9120; continue;
 }
 0x60b9120 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b9129; continue;
 }
 0x60b9129 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b913e; continue; }
  pc = 0x60b9135; continue;
 }
 0x60b9135 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.esp), v); }
  pc = 0x60b913e; continue;
 }
 0x60b913e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b9140; continue;
 }
 0x60b9140 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1))), 0x1u32); }
  if (cpu.sf == cpu.of) { pc = 0x60b914f; continue; }
  pc = 0x60b9147; continue;
 }
 0x60b9147 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x60b914f; continue;
 }
 0x60b914f => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b9140; continue; }
  pc = 0x60b9157; continue;
 }
 0x60b9157 => {
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
  cpu.push(0x60b917d); f_060acc60(cpu); if cpu.returned_to != 0x60b917d { return; }
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
pub fn f_060b9190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9190); }
 let mut pc: u32 = 0x60b9190;
 loop { match pc {
 0x60b9190 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x60b993a; continue; }
  pc = 0x60b91a9; continue;
 }
 0x60b91a9 => {
  { cpu.sub32(cpu.r32(0x617c73cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60b993a; continue; }
  pc = 0x60b91b6; continue;
 }
 0x60b91b6 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617c73cu32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1f68u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f5cu32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60b98e5; continue; }
  pc = 0x60b91da; continue;
 }
 0x60b91da => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.eax)), 0x4u32); }
  if !cpu.zf { pc = 0x60b98c1; continue; }
  pc = 0x60b91e9; continue;
 }
 0x60b91e9 => {
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x38u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d1f78u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b9203; continue;
 }
 0x60b9203 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0x64u32); }
  if (cpu.sf != cpu.of) { pc = 0x60b97f9; continue; }
  pc = 0x60b9214; continue;
 }
 0x60b9214 => {
  { let v: u32 = 0xffffff9cu32.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5cu32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b92bb; continue; }
  pc = 0x60b9228; continue;
 }
 0x60b9228 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f60u32, v); }
  { let v: u32 = 0xffffffffu32; cpu.edi = v; }
  pc = 0x60b9235; continue;
 }
 0x60b9235 => {
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x55cu32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b92a5; continue; }
  pc = 0x60b924d; continue;
 }
 0x60b924d => {
  { let v: u32 = cpu.r32(0x65cu32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x65cu32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x60b92a5; continue; }
  pc = 0x60b925f; continue;
 }
 0x60b925f => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b927e); f_060b9e10(cpu); if cpu.returned_to != 0x60b927e { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x558u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if cpu.zf { pc = 0x60b92bb; continue; }
  pc = 0x60b92a5; continue;
 }
 0x60b92a5 => {
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1f60u32, v); }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b9235; continue; }
  pc = 0x60b92bb; continue;
 }
 0x60b92bb => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x60b976c; continue; }
  pc = 0x60b92cf; continue;
 }
 0x60b92cf => {
  { cpu.sub32(cpu.r32(0x62d1f78u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b974a; continue; }
  pc = 0x60b92dc; continue;
 }
 0x60b92dc => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f74u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.cf { pc = 0x60b974a; continue; }
  pc = 0x60b92fa; continue;
 }
 0x60b92fa => {
  { cpu.sub32(cpu.eax, 0xf7u32); }
  if cpu.cf { pc = 0x60b9313; continue; }
  pc = 0x60b9301; continue;
 }
 0x60b9301 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b94ff; continue; }
  pc = 0x60b9307; continue;
 }
 0x60b9307 => {
  { cpu.sub32(cpu.eax, 0xffu32); }
  if cpu.zf { pc = 0x60b9323; continue; }
  pc = 0x60b930e; continue;
 }
 0x60b930e => {
  pc = 0x60b959e; continue;
 }
 0x60b9313 => {
  { cpu.sub32(cpu.eax, 0xf0u32); }
  if cpu.zf { pc = 0x60b94ff; continue; }
  pc = 0x60b931e; continue;
 }
 0x60b931e => {
  pc = 0x60b959e; continue;
 }
 0x60b9323 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x18u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0xffffffffu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f54u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60b933b); f_060ba7c0(cpu); if cpu.returned_to != 0x60b933b { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f50u32, v); }
  { let v: u32 = cpu.r32(0x62d1f54u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x51u32); }
  if cpu.cf { pc = 0x60b9361; continue; }
  pc = 0x60b934d; continue;
 }
 0x60b934d => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9404; continue; }
  pc = 0x60b9353; continue;
 }
 0x60b9353 => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if cpu.zf { pc = 0x60b944b; continue; }
  pc = 0x60b935c; continue;
 }
 0x60b935c => {
  pc = 0x60b94ec; continue;
 }
 0x60b9361 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x60b94ec; continue; }
  pc = 0x60b936a; continue;
 }
 0x60b936a => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d1f78u32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60b938a; continue; }
  pc = 0x60b9381; continue;
 }
 0x60b9381 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x60b93c8; continue; }
  pc = 0x60b938a; continue;
 }
 0x60b938a => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60b94ec; continue; }
  pc = 0x60b93b8; continue;
 }
 0x60b93b8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  pc = 0x60b94e9; continue;
 }
 0x60b93c8 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x30u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b93d1); f_060a9000(cpu); if cpu.returned_to != 0x60b93d1 { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b94ec; continue; }
  pc = 0x60b93eb; continue;
 }
 0x60b93eb => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x2cu32.wrapping_add(cpu.ebx)); cpu.push(0x60b93f1); cpu.call(t); if cpu.returned_to != 0x60b93f1 { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f50u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60b92cf; continue;
 }
 0x60b9404 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
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
  { let v: u32 = cpu.ecx; cpu.w32(0x62d1f70u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x74u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f50u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60b92cf; continue;
 }
 0x60b944b => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f70u32, v); }
  { let v: u32 = 0xf42400u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d1eccu32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x62d1f70u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f6cu32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x60b94a6; continue; }
  pc = 0x60b9492; continue;
 }
 0x60b9492 => {
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f70u32, v); }
  { let v: u8 = cpu.r8(0x62d1f70u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x60b94ae; continue;
 }
 0x60b94a6 => {
  { let v: u8 = cpu.r8(0x62d1f70u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  pc = 0x60b94ae; continue;
 }
 0x60b94ae => {
  { let v: u32 = cpu.eax; cpu.w32(0x6cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
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
  if cpu.zf { pc = 0x60b94ec; continue; }
  pc = 0x60b94dd; continue;
 }
 0x60b94dd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  pc = 0x60b94e9; continue;
 }
 0x60b94e9 => {
  let t = cpu.r32(0x28u32.wrapping_add(cpu.ebx)); cpu.push(0x60b94ec); cpu.call(t); if cpu.returned_to != 0x60b94ec { return; }
  pc = 0x60b94ec; continue;
 }
 0x60b94ec => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f50u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)).wrapping_add(cpu.ebx); cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60b92cf; continue;
 }
 0x60b94ff => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v = 0x62d1f48u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f48u32, v); }
  cpu.push(0x60b9517); f_060ba7c0(cpu); if cpu.returned_to != 0x60b9517 { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x62d1f48u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f50u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f50u32, v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  cpu.push(0x60b9545); f_060baa40(cpu); if cpu.returned_to != 0x60b9545 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.ebx, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60b9557; continue; }
  pc = 0x60b9550; continue;
 }
 0x60b9550 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x60b9559; continue;
 }
 0x60b9557 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x60b9559; continue;
 }
 0x60b9559 => {
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
  cpu.push(0x60b9583); f_060baa40(cpu); if cpu.returned_to != 0x60b9583 { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f50u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  cpu.poll(); pc = 0x60b92cf; continue;
 }
 0x60b959e => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1f4cu32, v); }
  { let v: u32 = cpu.r32(0x62d1f74u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x62d1f58u32, v); }
  { let v: u32 = cpu.r32(0x62d1f74u32) & 0xf0u32; cpu.w32(0x62d1f74u32, v); }
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
  cpu.push(0x60b95e5); f_060b9e10(cpu); if cpu.returned_to != 0x60b95e5 { return; }
  { let v: u32 = cpu.r32(0x62d1f74u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if cpu.zf { pc = 0x60b9677; continue; }
  pc = 0x60b95fa; continue;
 }
 0x60b95fa => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d1f4cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x60b92cf; continue; }
  pc = 0x60b9610; continue;
 }
 0x60b9610 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x60b9640; continue; }
  pc = 0x60b9623; continue;
 }
 0x60b9623 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9659; continue; }
  pc = 0x60b9625; continue;
 }
 0x60b9625 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x60b9637; continue; }
  pc = 0x60b962c; continue;
 }
 0x60b962c => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9660; continue; }
  pc = 0x60b962e; continue;
 }
 0x60b962e => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x60b9659; continue; }
  pc = 0x60b9635; continue;
 }
 0x60b9635 => {
  pc = 0x60b9667; continue;
 }
 0x60b9637 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x60b9660; continue; }
  pc = 0x60b963e; continue;
 }
 0x60b963e => {
  pc = 0x60b9667; continue;
 }
 0x60b9640 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x60b9652; continue; }
  pc = 0x60b9647; continue;
 }
 0x60b9647 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9659; continue; }
  pc = 0x60b9649; continue;
 }
 0x60b9649 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x60b9659; continue; }
  pc = 0x60b9650; continue;
 }
 0x60b9650 => {
  pc = 0x60b9667; continue;
 }
 0x60b9652 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x60b9667; continue; }
  pc = 0x60b9659; continue;
 }
 0x60b9659 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x60b9669; continue;
 }
 0x60b9660 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x60b9669; continue;
 }
 0x60b9667 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b9669; continue;
 }
 0x60b9669 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(cpu.eax); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x60b92cf; continue;
 }
 0x60b9677 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d1f60u32, v); }
  pc = 0x60b9687; continue;
 }
 0x60b9687 => {
  { cpu.sub32(cpu.r32(0x55cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(1))), 0xffffffffu32); }
  if cpu.zf { pc = 0x60b96a8; continue; }
  pc = 0x60b9691; continue;
 }
 0x60b9691 => {
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x62d1f60u32, v); }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b9687; continue; }
  pc = 0x60b96a8; continue;
 }
 0x60b96a8 => {
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if !cpu.zf { pc = 0x60b96f1; continue; }
  pc = 0x60b96b3; continue;
 }
 0x60b96b3 => {
  { let v: u32 = 0x616af3cu32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60b96bf; continue;
 }
 0x60b96bf => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60b96bf; continue; }
  pc = 0x60b96c9; continue;
 }
 0x60b96c9 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  cpu.push(0x60b96d7); f_060a9000(cpu); if cpu.returned_to != 0x60b96d7 { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x617c73cu32, v); }
  { let v: u32 = 0x2u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b993a; continue;
 }
 0x60b96f1 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.r32(0x558u32.wrapping_add(cpu.eax))); cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d1f58u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x55cu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5dcu32.wrapping_add(cpu.eax).wrapping_add(cpu.ebx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x18u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x3u32); cpu.esi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60b972f); f_060ba7c0(cpu); if cpu.returned_to != 0x60b972f { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.ecx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x65cu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  cpu.poll(); pc = 0x60b92cf; continue;
 }
 0x60b974a => {
  { cpu.sub32(cpu.r32(0x62d1f78u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b976c; continue; }
  pc = 0x60b9753; continue;
 }
 0x60b9753 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x34u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b976c; continue;
 }
 0x60b976c => {
  { cpu.sub32(cpu.r32(0x62d1f78u32), 0x0u32); }
  if !cpu.zf { cpu.poll(); pc = 0x60b9203; continue; }
  pc = 0x60b9779; continue;
 }
 0x60b9779 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x74u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b9203; continue; }
  pc = 0x60b9794; continue;
 }
 0x60b9794 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x70u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.edi, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x60b97b7; continue; }
  pc = 0x60b97a9; continue;
 }
 0x60b97a9 => {
  { let v: u32 = cpu.r32(0x64u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x64u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b97b7; continue;
 }
 0x60b97b7 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x28u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { cpu.poll(); pc = 0x60b9203; continue; }
  pc = 0x60b97c6; continue;
 }
 0x60b97c6 => {
  { let v = 0x62d1f64u32; cpu.push(v); }
  { let v = 0x62d1f60u32; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b97d6); f_060a7b60(cpu); if cpu.returned_to != 0x60b97d6 { return; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d1f60u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d1f64u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.eax)); cpu.push(0x60b97f4); cpu.call(t); if cpu.returned_to != 0x60b97f4 { return; }
  cpu.poll(); pc = 0x60b9203; continue;
 }
 0x60b97f9 => {
  { cpu.sub32(cpu.r32(0x62d1f78u32), 0x0u32); }
  if !cpu.zf { pc = 0x60b98c1; continue; }
  pc = 0x60b9806; continue;
 }
 0x60b9806 => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b986b; continue; }
  pc = 0x60b980e; continue;
 }
 0x60b980e => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b981c; continue;
 }
 0x60b981c => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x44u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x48u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x60b9857; continue; }
  pc = 0x60b982b; continue;
 }
 0x60b982b => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edi); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x44u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b9844; continue; }
  pc = 0x60b983c; continue;
 }
 0x60b983c => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b984a; continue;
 }
 0x60b9844 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x3cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b984a; continue;
 }
 0x60b984a => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x40u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x60b981c; continue; }
  pc = 0x60b9857; continue;
 }
 0x60b9857 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { cpu.and8(cpu.r8(0x38u32.wrapping_add(cpu.eax)), 0x7u8); }
  if !cpu.zf { pc = 0x60b986b; continue; }
  pc = 0x60b9862; continue;
 }
 0x60b9862 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b9868); f_060b9950(cpu); if cpu.returned_to != 0x60b9868 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b986b; continue;
 }
 0x60b986b => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60b98c1; continue; }
  pc = 0x60b9878; continue;
 }
 0x60b9878 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b9886; continue;
 }
 0x60b9886 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x60b98c1; continue; }
  pc = 0x60b9895; continue;
 }
 0x60b9895 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x54u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b98ae; continue; }
  pc = 0x60b98a6; continue;
 }
 0x60b98a6 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b98b4; continue;
 }
 0x60b98ae => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4cu32.wrapping_add(cpu.eax), v); }
  pc = 0x60b98b4; continue;
 }
 0x60b98b4 => {
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4cu32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x50u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { cpu.poll(); pc = 0x60b9886; continue; }
  pc = 0x60b98c1; continue;
 }
 0x60b98c1 => {
  { let v: u32 = cpu.r32(0x62d1f68u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x62d1f5cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x71cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x62d1f68u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x62d1f5cu32, v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x60b91da; continue; }
  pc = 0x60b98e5; continue;
 }
 0x60b98e5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.eax)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b9932; continue; }
  pc = 0x60b98f2; continue;
 }
 0x60b98f2 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b9917); f_060acc60(cpu); if cpu.returned_to != 0x60b9917 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b9932; continue;
 }
 0x60b9932 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x617c73cu32, v); }
  pc = 0x60b993a; continue;
 }
 0x60b993a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b9950(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9950); }
 let mut pc: u32 = 0x60b9950;
 loop { match pc {
 0x60b9950 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x60b995b; continue;
 }
 0x60b995b => {
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9979; continue; }
  pc = 0x60b9966; continue;
 }
 0x60b9966 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60b9976); f_060b9e10(cpu); if cpu.returned_to != 0x60b9976 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9979; continue;
 }
 0x60b9979 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.esi, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b995b; continue; }
  pc = 0x60b9982; continue;
 }
 0x60b9982 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b9990(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9990); }
 let mut pc: u32 = 0x60b9990;
 loop { match pc {
 0x60b9990 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60b999b); f_060b99b0(cpu); if cpu.returned_to != 0x60b999b { return; }
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
pub fn f_060b99b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b99b0); }
 let mut pc: u32 = 0x60b99b0;
 loop { match pc {
 0x60b99b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60b99bb; continue;
 }
 0x60b99bb => {
  { let v: u32 = cpu.eax; cpu.w32(0x98u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60b99bb; continue; }
  pc = 0x60b99ca; continue;
 }
 0x60b99ca => {
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
  pc = 0x60b99f5; continue;
 }
 0x60b99f5 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x84u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b99f5; continue; }
  pc = 0x60b9a06; continue;
 }
 0x60b9a06 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x60b9a0e; continue;
 }
 0x60b9a0e => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x558u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { cpu.poll(); pc = 0x60b9a0e; continue; }
  pc = 0x60b9a1f; continue;
 }
 0x60b9a1f => {
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
pub fn f_060b9a70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9a70); }
 let mut pc: u32 = 0x60b9a70;
 loop { match pc {
 0x60b9a70 => {
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
  if cpu.zf { pc = 0x60b9aa3; continue; }
  pc = 0x60b9a90; continue;
 }
 0x60b9a90 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x72u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9aa0); f_060b9e10(cpu); if cpu.returned_to != 0x60b9aa0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9aa3; continue;
 }
 0x60b9aa3 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xd8u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9aca; continue; }
  pc = 0x60b9ab7; continue;
 }
 0x60b9ab7 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xc0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9ac7); f_060b9e10(cpu); if cpu.returned_to != 0x60b9ac7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9aca; continue;
 }
 0x60b9aca => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x158u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9af6; continue; }
  pc = 0x60b9ade; continue;
 }
 0x60b9ade => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x118u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xe0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9af3); f_060b9e10(cpu); if cpu.returned_to != 0x60b9af3 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9af6; continue;
 }
 0x60b9af6 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x218u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9b1d; continue; }
  pc = 0x60b9b0a; continue;
 }
 0x60b9b0a => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9b1a); f_060b9e10(cpu); if cpu.returned_to != 0x60b9b1a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9b1d; continue;
 }
 0x60b9b1d => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9b44; continue; }
  pc = 0x60b9b31; continue;
 }
 0x60b9b31 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x6fu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9b41); f_060b9e10(cpu); if cpu.returned_to != 0x60b9b41 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9b44; continue;
 }
 0x60b9b44 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x258u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9b6b; continue; }
  pc = 0x60b9b58; continue;
 }
 0x60b9b58 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x70u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9b68); f_060b9e10(cpu); if cpu.returned_to != 0x60b9b68 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9b6b; continue;
 }
 0x60b9b6b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x358u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9b92; continue; }
  pc = 0x60b9b7f; continue;
 }
 0x60b9b7f => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9b8f); f_060b9e10(cpu); if cpu.returned_to != 0x60b9b8f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9b92; continue;
 }
 0x60b9b92 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x398u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9bb9; continue; }
  pc = 0x60b9ba6; continue;
 }
 0x60b9ba6 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x7u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9bb6); f_060b9e10(cpu); if cpu.returned_to != 0x60b9bb6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9bb9; continue;
 }
 0x60b9bb9 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x3d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9be0; continue; }
  pc = 0x60b9bcd; continue;
 }
 0x60b9bcd => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xau32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9bdd); f_060b9e10(cpu); if cpu.returned_to != 0x60b9bdd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9be0; continue;
 }
 0x60b9be0 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x418u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9c07; continue; }
  pc = 0x60b9bf4; continue;
 }
 0x60b9bf4 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0xbu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9c04); f_060b9e10(cpu); if cpu.returned_to != 0x60b9c04 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9c07; continue;
 }
 0x60b9c07 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x458u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9c2e; continue; }
  pc = 0x60b9c1b; continue;
 }
 0x60b9c1b => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x40u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9c2b); f_060b9e10(cpu); if cpu.returned_to != 0x60b9c2b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9c2e; continue;
 }
 0x60b9c2e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x498u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9c55; continue; }
  pc = 0x60b9c42; continue;
 }
 0x60b9c42 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5bu32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9c52); f_060b9e10(cpu); if cpu.returned_to != 0x60b9c52 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9c55; continue;
 }
 0x60b9c55 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4d8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9c7c; continue; }
  pc = 0x60b9c69; continue;
 }
 0x60b9c69 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = 0x5du32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9c79); f_060b9e10(cpu); if cpu.returned_to != 0x60b9c79 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9c7c; continue;
 }
 0x60b9c7c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x518u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9ca2; continue; }
  pc = 0x60b9c90; continue;
 }
 0x60b9c90 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = 0x6u32; cpu.push(v); }
  { let v: u8 = (cpu.ebx as u8) | 0xb0u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9c9f); f_060b9e10(cpu); if cpu.returned_to != 0x60b9c9f { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  pc = 0x60b9ca2; continue;
 }
 0x60b9ca2 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060b9cb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9cb0); }
 let mut pc: u32 = 0x60b9cb0;
 loop { match pc {
 0x60b9cb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.edi = v; }
  pc = 0x60b9cc7; continue;
 }
 0x60b9cc7 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if !cpu.zf { pc = 0x60b9cf2; continue; }
  pc = 0x60b9cd1; continue;
 }
 0x60b9cd1 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9ce5); f_060b9e10(cpu); if cpu.returned_to != 0x60b9ce5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b9cf2; continue;
 }
 0x60b9cf2 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x60b9cc7; continue; }
  pc = 0x60b9cf9; continue;
 }
 0x60b9cf9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b9d3d; continue; }
  pc = 0x60b9d05; continue;
 }
 0x60b9d05 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b9d26); f_060acc60(cpu); if cpu.returned_to != 0x60b9d26 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b9d3d; continue;
 }
 0x60b9d3d => {
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
pub fn f_060b9d50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9d50); }
 let mut pc: u32 = 0x60b9d50;
 loop { match pc {
 0x60b9d50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.edi); cpu.ebp = v; }
  pc = 0x60b9d65; continue;
 }
 0x60b9d65 => {
  { let v: u32 = cpu.r32(0x55cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9d92; continue; }
  pc = 0x60b9d70; continue;
 }
 0x60b9d70 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5dcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0x80u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x60b9d85); f_060b9e10(cpu); if cpu.returned_to != 0x60b9d85 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x55cu32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b9d92; continue;
 }
 0x60b9d92 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if !cpu.zf { cpu.poll(); pc = 0x60b9d65; continue; }
  pc = 0x60b9d99; continue;
 }
 0x60b9d99 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x558u32.wrapping_add(cpu.edi), v); }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60b9de7; continue; }
  pc = 0x60b9daf; continue;
 }
 0x60b9daf => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60b9dd0); f_060acc60(cpu); if cpu.returned_to != 0x60b9dd0 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60b9de7; continue;
 }
 0x60b9de7 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60b9dfe; continue; }
  pc = 0x60b9deb; continue;
 }
 0x60b9deb => {
  cpu.push(0x60b9df0); f_060acba0(cpu); if cpu.returned_to != 0x60b9df0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60b9dfe; continue; }
  pc = 0x60b9df4; continue;
 }
 0x60b9df4 => {
  { let v = 0x3u32; cpu.push(v); }
  cpu.push(0x60b9dfb); f_060acbb0(cpu); if cpu.returned_to != 0x60b9dfb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60b9dfe; continue;
 }
 0x60b9dfe => {
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
pub fn f_060b9e10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60b9e10); }
 let mut pc: u32 = 0x60b9e10;
 loop { match pc {
 0x60b9e10 => {
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
  if cpu.zf { pc = 0x60b9e64; continue; }
  pc = 0x60b9e54; continue;
 }
 0x60b9e54 => {
  { cpu.sub32(cpu.edx, 0xc0u32); }
  if cpu.zf { pc = 0x60b9e64; continue; }
  pc = 0x60b9e5c; continue;
 }
 0x60b9e5c => {
  { cpu.sub32(cpu.edx, 0xe0u32); }
  if !cpu.zf { pc = 0x60b9e80; continue; }
  pc = 0x60b9e64; continue;
 }
 0x60b9e64 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edx; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0xd8u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60b9e7d); f_060ba280(cpu); if cpu.returned_to != 0x60b9e7d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60b9e80; continue;
 }
 0x60b9e80 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0xb0u32); }
  if !cpu.zf { pc = 0x60b9fb7; continue; }
  pc = 0x60b9e8e; continue;
 }
 0x60b9e8e => {
  { cpu.sub32(cpu.r32(0x38u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60b9eb5; continue; }
  pc = 0x60b9e95; continue;
 }
 0x60b9e95 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x2d8u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if cpu.zf { pc = 0x60b9eb5; continue; }
  pc = 0x60b9ea9; continue;
 }
 0x60b9ea9 => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.w32(0x2d8u32.wrapping_add(cpu.eax), v); }
  pc = 0x60b9eb5; continue;
 }
 0x60b9eb5 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, 0x6fu32); }
  if cpu.cf { pc = 0x60b9f0c; continue; }
  pc = 0x60b9ed4; continue;
 }
 0x60b9ed4 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba140; continue; }
  pc = 0x60b9eda; continue;
 }
 0x60b9eda => {
  { cpu.sub32(cpu.ebp, 0x75u32); }
  if cpu.cf { pc = 0x60b9efc; continue; }
  pc = 0x60b9edf; continue;
 }
 0x60b9edf => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba0b9; continue; }
  pc = 0x60b9ee5; continue;
 }
 0x60b9ee5 => {
  { cpu.sub32(cpu.ebp, 0x76u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60ba028; continue; }
  pc = 0x60b9eee; continue;
 }
 0x60b9eee => {
  { cpu.sub32(cpu.ebp, 0x77u32); }
  if cpu.zf { pc = 0x60ba068; continue; }
  pc = 0x60b9ef7; continue;
 }
 0x60b9ef7 => {
  pc = 0x60b9fb7; continue;
 }
 0x60b9efc => {
  { cpu.sub32(cpu.ebp, 0x73u32); }
  if cpu.cf { pc = 0x60b9fb7; continue; }
  pc = 0x60b9f05; continue;
 }
 0x60b9f05 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9f3a; continue; }
  pc = 0x60b9f07; continue;
 }
 0x60b9f07 => {
  pc = 0x60ba084; continue;
 }
 0x60b9f0c => {
  { cpu.sub32(cpu.ebp, 0x6cu32); }
  if cpu.cf { pc = 0x60b9f21; continue; }
  pc = 0x60b9f11; continue;
 }
 0x60b9f11 => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9f53; continue; }
  pc = 0x60b9f13; continue;
 }
 0x60b9f13 => {
  { cpu.sub32(cpu.ebp, 0x6du32); }
  if (cpu.cf || cpu.zf) { pc = 0x60ba12e; continue; }
  pc = 0x60b9f1c; continue;
 }
 0x60b9f1c => {
  pc = 0x60ba16d; continue;
 }
 0x60b9f21 => {
  { cpu.sub32(cpu.ebp, 0x6u32); }
  if cpu.cf { pc = 0x60b9fb7; continue; }
  pc = 0x60b9f2a; continue;
 }
 0x60b9f2a => {
  if (cpu.cf || cpu.zf) { pc = 0x60b9f75; continue; }
  pc = 0x60b9f2c; continue;
 }
 0x60b9f2c => {
  { cpu.sub32(cpu.ebp, 0x7u32); }
  if cpu.zf { pc = 0x60b9ffb; continue; }
  pc = 0x60b9f35; continue;
 }
 0x60b9f35 => {
  pc = 0x60b9fb7; continue;
 }
 0x60b9f3a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b9fb7; continue;
 }
 0x60b9f53 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60b9fb7; continue; }
  pc = 0x60b9f5d; continue;
 }
 0x60b9f5d => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.push(0x60b9f67); cpu.call(t); if cpu.returned_to != 0x60b9f67 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2d8u32.wrapping_add(cpu.edx), v); }
  pc = 0x60b9fb7; continue;
 }
 0x60b9f75 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x64u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9f88); f_060b9e10(cpu); if cpu.returned_to != 0x60b9f88 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x65u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9f9e); f_060b9e10(cpu); if cpu.returned_to != 0x60b9f9e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = 0x26u32; cpu.push(v); }
  { let v: u8 = (cpu.eax as u8) | 0xb0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60b9fb4); f_060b9e10(cpu); if cpu.returned_to != 0x60b9fb4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x14u32); cpu.esp = v; }
  pc = 0x60b9fb7; continue;
 }
 0x60b9fb7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x60b9fd3; continue; }
  pc = 0x60b9fca; continue;
 }
 0x60b9fca => {
  { cpu.sub32(cpu.esi, cpu.r32(0x64u32.wrapping_add(cpu.eax))); }
  if !cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60b9fd3; continue;
 }
 0x60b9fd3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x90u32); }
  if !cpu.zf { pc = 0x60ba1f2; continue; }
  pc = 0x60b9ff0; continue;
 }
 0x60b9ff0 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x60ba200; continue;
 }
 0x60b9ffb => {
  { let v: u32 = cpu.r32(0x3cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x1b4u32.wrapping_add(cpu.ebx))); cpu.edx = v; }
  { let v: u32 = 0x3f01u32; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.edi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.sub32(cpu.eax, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60ba020; continue; }
  pc = 0x60ba01b; continue;
 }
 0x60ba01b => {
  { let v: u32 = 0x7fu32; cpu.edi = v; }
  pc = 0x60ba020; continue;
 }
 0x60ba020 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60b9fb7; continue; }
  pc = 0x60ba024; continue;
 }
 0x60ba024 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  cpu.poll(); pc = 0x60b9fb7; continue;
 }
 0x60ba028 => {
  { let v: u32 = 0x0u32; cpu.w32(0x60u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x6cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x70u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x64u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x70u32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba053; continue;
 }
 0x60ba053 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  let t = cpu.r32(0x28u32.wrapping_add(cpu.esi)); cpu.push(0x60ba060); cpu.call(t); if cpu.returned_to != 0x60ba060 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba068 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba072; continue;
 }
 0x60ba072 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x24u32.wrapping_add(cpu.esi)); cpu.push(0x60ba07c); cpu.call(t); if cpu.returned_to != 0x60ba07c { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba084 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ba088; continue;
 }
 0x60ba088 => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if cpu.zf { pc = 0x60ba09a; continue; }
  pc = 0x60ba091; continue;
 }
 0x60ba091 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60ba088; continue; }
  pc = 0x60ba09a; continue;
 }
 0x60ba09a => {
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba0a3; continue;
 }
 0x60ba0a3 => {
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
 0x60ba0b9 => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60ba26f; continue; }
  pc = 0x60ba0c2; continue;
 }
 0x60ba0c2 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x60ba0ca; continue;
 }
 0x60ba0ca => {
  { cpu.sub32(cpu.r32(0x88u32.wrapping_add(cpu.edx)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x60ba0db; continue; }
  pc = 0x60ba0d3; continue;
 }
 0x60ba0d3 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60ba0ca; continue; }
  pc = 0x60ba0db; continue;
 }
 0x60ba0db => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba0e4; continue;
 }
 0x60ba0e4 => {
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x88u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60ba101; continue; }
  pc = 0x60ba0f3; continue;
 }
 0x60ba0f3 => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba101 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60ba11c; continue; }
  pc = 0x60ba10e; continue;
 }
 0x60ba10e => {
  { let v: u32 = cpu.r32(0x78u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba11c => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x88u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba12e => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ba135); f_060a7aa0(cpu); if cpu.returned_to != 0x60ba135 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba140 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba14a; continue;
 }
 0x60ba14a => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x60ba15e; continue; }
  pc = 0x60ba14f; continue;
 }
 0x60ba14f => {
  { let v: u32 = 0x0u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba15e => {
  { let v: u32 = 0x2u32; cpu.w32(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba16d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x60ba1b5; continue; }
  pc = 0x60ba177; continue;
 }
 0x60ba177 => {
  { cpu.sub32(cpu.r32(0x24u32.wrapping_add(cpu.eax)), 0x1u32); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba181; continue;
 }
 0x60ba181 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60ba18b); f_060a70a0(cpu); if cpu.returned_to != 0x60ba18b { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba198; continue;
 }
 0x60ba198 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ba1a0); f_060a6f10(cpu); if cpu.returned_to != 0x60ba1a0 { return; }
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
 0x60ba1b5 => {
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba1c1; continue;
 }
 0x60ba1c1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ba1cc); f_060b9cb0(cpu); if cpu.returned_to != 0x60ba1cc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60ba1dc); f_060a6fe0(cpu); if cpu.returned_to != 0x60ba1dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60ba1e7); f_060a6f10(cpu); if cpu.returned_to != 0x60ba1e7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba1f2 => {
  { cpu.sub32(cpu.edx, 0x80u32); }
  if !cpu.zf { pc = 0x60ba200; continue; }
  pc = 0x60ba1fa; continue;
 }
 0x60ba1fa => {
  { let v: u32 = cpu.dec32(cpu.r32(0x164u32.wrapping_add(cpu.eax))); cpu.w32(0x164u32.wrapping_add(cpu.eax), v); }
  pc = 0x60ba200; continue;
 }
 0x60ba200 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0xe4u32.wrapping_add(cpu.eax), v); }
  { cpu.sub32(cpu.ecx, 0x90u32); }
  if !cpu.zf { pc = 0x60ba22d; continue; }
  pc = 0x60ba21f; continue;
 }
 0x60ba21f => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x218u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))), 0x40u32); }
  if (cpu.sf == cpu.of) { pc = 0x60ba26f; continue; }
  pc = 0x60ba22d; continue;
 }
 0x60ba22d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x1a4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x60ba255; continue; }
  pc = 0x60ba23a; continue;
 }
 0x60ba23a => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.edi; cpu.push(v); }
  let t = cpu.r32(0x1a4u32.wrapping_add(cpu.eax)); cpu.push(0x60ba251); cpu.call(t); if cpu.returned_to != 0x60ba251 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60ba26f; continue; }
  pc = 0x60ba255; continue;
 }
 0x60ba255 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax | cpu.edi; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60ba26c); f_060ba8c0(cpu); if cpu.returned_to != 0x60ba26c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60ba26f; continue;
 }
 0x60ba26f => {
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
pub fn f_060ba280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba280); }
 let mut pc: u32 = 0x60ba280;
 loop { match pc {
 0x60ba280 => {
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
  if cpu.cf { pc = 0x60ba2bd; continue; }
  pc = 0x60ba2af; continue;
 }
 0x60ba2af => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba2c9; continue; }
  pc = 0x60ba2b1; continue;
 }
 0x60ba2b1 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x60ba2d4; continue; }
  pc = 0x60ba2b9; continue;
 }
 0x60ba2b9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba2bd => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x60ba2ea; continue; }
  pc = 0x60ba2c5; continue;
 }
 0x60ba2c5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba2c9 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba2d4 => {
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
 0x60ba2ea => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x60ba33b; continue; }
  pc = 0x60ba2ef; continue;
 }
 0x60ba2ef => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba447; continue; }
  pc = 0x60ba2f5; continue;
 }
 0x60ba2f5 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x60ba325; continue; }
  pc = 0x60ba2fa; continue;
 }
 0x60ba2fa => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba3b6; continue; }
  pc = 0x60ba300; continue;
 }
 0x60ba300 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x60ba318; continue; }
  pc = 0x60ba305; continue;
 }
 0x60ba305 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba3d2; continue; }
  pc = 0x60ba30b; continue;
 }
 0x60ba30b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x60ba3e0; continue; }
  pc = 0x60ba314; continue;
 }
 0x60ba314 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba318 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x60ba3c4; continue; }
  pc = 0x60ba321; continue;
 }
 0x60ba321 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba325 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x60ba32e; continue; }
  pc = 0x60ba32a; continue;
 }
 0x60ba32a => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba38c; continue; }
  pc = 0x60ba32c; continue;
 }
 0x60ba32c => {
  pc = 0x60ba39a; continue;
 }
 0x60ba32e => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x60ba3a8; continue; }
  pc = 0x60ba337; continue;
 }
 0x60ba337 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba33b => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x60ba36b; continue; }
  pc = 0x60ba340; continue;
 }
 0x60ba340 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba40c; continue; }
  pc = 0x60ba346; continue;
 }
 0x60ba346 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x60ba35e; continue; }
  pc = 0x60ba34b; continue;
 }
 0x60ba34b => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba42a; continue; }
  pc = 0x60ba351; continue;
 }
 0x60ba351 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x60ba439; continue; }
  pc = 0x60ba35a; continue;
 }
 0x60ba35a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba35e => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x60ba41b; continue; }
  pc = 0x60ba367; continue;
 }
 0x60ba367 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba36b => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x60ba383; continue; }
  pc = 0x60ba370; continue;
 }
 0x60ba370 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba455; continue; }
  pc = 0x60ba376; continue;
 }
 0x60ba376 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x60ba3fd; continue; }
  pc = 0x60ba37f; continue;
 }
 0x60ba37f => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba383 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60ba3ee; continue; }
  pc = 0x60ba388; continue;
 }
 0x60ba388 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba38c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba39a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3a8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x140u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3b6 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3c4 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3d2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3e0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3ee => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba3fd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba40c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba41b => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba42a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba439 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba447 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba455 => {
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
pub fn f_060ba470(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba470); }
 let mut pc: u32 = 0x60ba470;
 loop { match pc {
 0x60ba470 => {
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
  if cpu.cf { pc = 0x60ba4a9; continue; }
  pc = 0x60ba49a; continue;
 }
 0x60ba49a => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba4b6; continue; }
  pc = 0x60ba49c; continue;
 }
 0x60ba49c => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x60ba4bb; continue; }
  pc = 0x60ba4a4; continue;
 }
 0x60ba4a4 => {
  pc = 0x60ba61a; continue;
 }
 0x60ba4a9 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.zf { pc = 0x60ba4cc; continue; }
  pc = 0x60ba4b1; continue;
 }
 0x60ba4b1 => {
  pc = 0x60ba61a; continue;
 }
 0x60ba4b6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba4bb => {
  { let v: u32 = cpu.r32(0x80u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x40u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 7u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba4cc => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.cf { pc = 0x60ba530; continue; }
  pc = 0x60ba4d1; continue;
 }
 0x60ba4d1 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba607; continue; }
  pc = 0x60ba4d7; continue;
 }
 0x60ba4d7 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x60ba50f; continue; }
  pc = 0x60ba4dc; continue;
 }
 0x60ba4dc => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba5a8; continue; }
  pc = 0x60ba4e2; continue;
 }
 0x60ba4e2 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x60ba4fe; continue; }
  pc = 0x60ba4e7; continue;
 }
 0x60ba4e7 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba5ba; continue; }
  pc = 0x60ba4ed; continue;
 }
 0x60ba4ed => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x60ba5c3; continue; }
  pc = 0x60ba4f6; continue;
 }
 0x60ba4f6 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba4fe => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x60ba5b1; continue; }
  pc = 0x60ba507; continue;
 }
 0x60ba507 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba50f => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x60ba51f; continue; }
  pc = 0x60ba514; continue;
 }
 0x60ba514 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba58d; continue; }
  pc = 0x60ba51a; continue;
 }
 0x60ba51a => {
  pc = 0x60ba596; continue;
 }
 0x60ba51f => {
  { cpu.sub32(cpu.eax, 0x6bu32); }
  if cpu.zf { pc = 0x60ba59f; continue; }
  pc = 0x60ba528; continue;
 }
 0x60ba528 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba530 => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.cf { pc = 0x60ba568; continue; }
  pc = 0x60ba535; continue;
 }
 0x60ba535 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba5e0; continue; }
  pc = 0x60ba53b; continue;
 }
 0x60ba53b => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.cf { pc = 0x60ba557; continue; }
  pc = 0x60ba540; continue;
 }
 0x60ba540 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba5f4; continue; }
  pc = 0x60ba546; continue;
 }
 0x60ba546 => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.zf { pc = 0x60ba5fe; continue; }
  pc = 0x60ba54f; continue;
 }
 0x60ba54f => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba557 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.zf { pc = 0x60ba5ea; continue; }
  pc = 0x60ba560; continue;
 }
 0x60ba560 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba568 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.cf { pc = 0x60ba580; continue; }
  pc = 0x60ba56d; continue;
 }
 0x60ba56d => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba610; continue; }
  pc = 0x60ba573; continue;
 }
 0x60ba573 => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.zf { pc = 0x60ba5d6; continue; }
  pc = 0x60ba578; continue;
 }
 0x60ba578 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba580 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60ba5cc; continue; }
  pc = 0x60ba585; continue;
 }
 0x60ba585 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba58d => {
  { let v: u32 = cpu.r32(0xc0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba596 => {
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba59f => {
  { let v: u32 = cpu.r32(0x140u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5a8 => {
  { let v: u32 = cpu.r32(0x180u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5b1 => {
  { let v: u32 = cpu.r32(0x1c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5ba => {
  { let v: u32 = cpu.r32(0x200u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5c3 => {
  { let v: u32 = cpu.r32(0x240u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5cc => {
  { let v: u32 = cpu.r32(0x280u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5d6 => {
  { let v: u32 = cpu.r32(0x2c0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5e0 => {
  { let v: u32 = cpu.r32(0x300u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5ea => {
  { let v: u32 = cpu.r32(0x340u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5f4 => {
  { let v: u32 = cpu.r32(0x380u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba5fe => {
  { let v: u32 = cpu.r32(0x3c0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba607 => {
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba610 => {
  { let v: u32 = cpu.r32(0x440u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba61a => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ba630(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba630); }
 let mut pc: u32 = 0x60ba630;
 loop { match pc {
 0x60ba630 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  pc = 0x60ba641; continue;
 }
 0x60ba641 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = 0x616af24u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60ba659; continue; }
  pc = 0x60ba657; continue;
 }
 0x60ba657 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60ba659; continue;
 }
 0x60ba659 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60ba67b; continue; }
  pc = 0x60ba65d; continue;
 }
 0x60ba65d => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616af2cu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60ba673; continue; }
  pc = 0x60ba671; continue;
 }
 0x60ba671 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60ba673; continue;
 }
 0x60ba673 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60ba76e; continue; }
  pc = 0x60ba67b; continue;
 }
 0x60ba67b => {
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
  { let v: u32 = 0x616af34u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60ba6c7; continue; }
  pc = 0x60ba6c5; continue;
 }
 0x60ba6c5 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60ba6c7; continue;
 }
 0x60ba6c7 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x60ba641; continue; }
  pc = 0x60ba6cf; continue;
 }
 0x60ba6cf => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616af24u32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60ba6e5; continue; }
  pc = 0x60ba6e3; continue;
 }
 0x60ba6e3 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60ba6e5; continue;
 }
 0x60ba6e5 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60ba6f8; continue; }
  pc = 0x60ba6e9; continue;
 }
 0x60ba6e9 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60ba76e; continue; }
  pc = 0x60ba6f1; continue;
 }
 0x60ba6f1 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60ba770; continue;
 }
 0x60ba6f8 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x60ba76e; continue; }
  pc = 0x60ba705; continue;
 }
 0x60ba705 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616af34u32; cpu.esi = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.edx); cpu.edi = v; }
  { cpu.df = false; }
  { cpu.cmps(1, 1); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60ba71c; continue; }
  pc = 0x60ba71a; continue;
 }
 0x60ba71a => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60ba71c; continue;
 }
 0x60ba71c => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60ba730; continue; }
  pc = 0x60ba720; continue;
 }
 0x60ba720 => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x60ba730; continue; }
  pc = 0x60ba726; continue;
 }
 0x60ba726 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60ba730 => {
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
  if cpu.cf { cpu.poll(); pc = 0x60ba705; continue; }
  pc = 0x60ba76e; continue;
 }
 0x60ba76e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ba770; continue;
 }
 0x60ba770 => {
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
pub fn f_060ba780(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba780); }
 let mut pc: u32 = 0x60ba780;
 loop { match pc {
 0x60ba780 => {
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
pub fn f_060ba7c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba7c0); }
 let mut pc: u32 = 0x60ba7c0;
 loop { match pc {
 0x60ba7c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x4u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  pc = 0x60ba7ce; continue;
 }
 0x60ba7ce => {
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
  if cpu.zf { pc = 0x60ba7e9; continue; }
  pc = 0x60ba7e6; continue;
 }
 0x60ba7e6 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x60ba7ce; continue; }
  pc = 0x60ba7e9; continue;
 }
 0x60ba7e9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060ba7f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba7f0); }
 let mut pc: u32 = 0x60ba7f0;
 loop { match pc {
 0x60ba7f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x34u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60ba840; continue; }
  pc = 0x60ba808; continue;
 }
 0x60ba808 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60ba829); f_060acc60(cpu); if cpu.returned_to != 0x60ba829 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60ba840; continue;
 }
 0x60ba840 => {
  { cpu.sub32(cpu.edi, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60ba84f; continue; }
  pc = 0x60ba848; continue;
 }
 0x60ba848 => {
  { let v: u32 = 0x200u32; cpu.ecx = v; }
  pc = 0x60ba851; continue;
 }
 0x60ba84f => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x60ba851; continue;
 }
 0x60ba851 => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60ba8b5; continue; }
  pc = 0x60ba87d; continue;
 }
 0x60ba87d => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60ba89e); f_060acc60(cpu); if cpu.returned_to != 0x60ba89e { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60ba8b5; continue;
 }
 0x60ba8b5 => {
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
pub fn f_060ba8c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60ba8c0); }
 let mut pc: u32 = 0x60ba8c0;
 loop { match pc {
 0x60ba8c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x30u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x38u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x62d1eecu32); cpu.esi = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60ba900; continue; }
  pc = 0x60ba8e2; continue;
 }
 0x60ba8e2 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x80u32); }
  if cpu.zf { pc = 0x60ba8fb; continue; }
  pc = 0x60ba8f0; continue;
 }
 0x60ba8f0 => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if !cpu.zf { pc = 0x60ba900; continue; }
  pc = 0x60ba8f7; continue;
 }
 0x60ba8f7 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60ba900; continue; }
  pc = 0x60ba8fb; continue;
 }
 0x60ba8fb => {
  { let v: u32 = 0x2u32; cpu.edi = v; }
  pc = 0x60ba900; continue;
 }
 0x60ba900 => {
  { let v: u8 = cpu.r8(0x34u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi & 0xf0u32; cpu.esi = v; }
  pc = 0x60ba918; continue;
 }
 0x60ba918 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x60baa32; continue; }
  pc = 0x60ba922; continue;
 }
 0x60ba922 => {
  { cpu.sub32(cpu.esi, 0xb0u32); }
  if cpu.cf { pc = 0x60ba94a; continue; }
  pc = 0x60ba92a; continue;
 }
 0x60ba92a => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba966; continue; }
  pc = 0x60ba92c; continue;
 }
 0x60ba92c => {
  { cpu.sub32(cpu.esi, 0xd0u32); }
  if cpu.cf { pc = 0x60ba940; continue; }
  pc = 0x60ba934; continue;
 }
 0x60ba934 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba96d; continue; }
  pc = 0x60ba936; continue;
 }
 0x60ba936 => {
  { cpu.sub32(cpu.esi, 0xe0u32); }
  if cpu.zf { pc = 0x60ba966; continue; }
  pc = 0x60ba93e; continue;
 }
 0x60ba93e => {
  pc = 0x60ba974; continue;
 }
 0x60ba940 => {
  { cpu.sub32(cpu.esi, 0xc0u32); }
  if cpu.zf { pc = 0x60ba96d; continue; }
  pc = 0x60ba948; continue;
 }
 0x60ba948 => {
  pc = 0x60ba974; continue;
 }
 0x60ba94a => {
  { cpu.sub32(cpu.esi, 0x90u32); }
  if cpu.cf { pc = 0x60ba95e; continue; }
  pc = 0x60ba952; continue;
 }
 0x60ba952 => {
  if (cpu.cf || cpu.zf) { pc = 0x60ba966; continue; }
  pc = 0x60ba954; continue;
 }
 0x60ba954 => {
  { cpu.sub32(cpu.esi, 0xa0u32); }
  if cpu.zf { pc = 0x60ba966; continue; }
  pc = 0x60ba95c; continue;
 }
 0x60ba95c => {
  pc = 0x60ba974; continue;
 }
 0x60ba95e => {
  { cpu.sub32(cpu.esi, 0x80u32); }
  if !cpu.zf { pc = 0x60ba974; continue; }
  pc = 0x60ba966; continue;
 }
 0x60ba966 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  pc = 0x60ba976; continue;
 }
 0x60ba96d => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  pc = 0x60ba976; continue;
 }
 0x60ba974 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60ba976; continue;
 }
 0x60ba976 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x200u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60ba9c8; continue; }
  pc = 0x60ba987; continue;
 }
 0x60ba987 => {
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60ba9c8; continue; }
  pc = 0x60ba990; continue;
 }
 0x60ba990 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60ba9b1); f_060acc60(cpu); if cpu.returned_to != 0x60ba9b1 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60ba9c8; continue;
 }
 0x60ba9c8 => {
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
  if !cpu.zf { pc = 0x60baa27; continue; }
  pc = 0x60baa09; continue;
 }
 0x60baa09 => {
  { let v: u32 = cpu.r32(0x1b0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x100u32.wrapping_add(cpu.eax), v); }
  pc = 0x60baa27; continue;
 }
 0x60baa27 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx))); cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x60ba918; continue;
 }
 0x60baa32 => {
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
pub fn f_060baa40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60baa40); }
 let mut pc: u32 = 0x60baa40;
 loop { match pc {
 0x60baa40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.r32(0x1acu32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60baa89; continue; }
  pc = 0x60baa51; continue;
 }
 0x60baa51 => {
  { let v: u16 = cpu.r16(0x1acu32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x502u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60baa72); f_060acc60(cpu); if cpu.returned_to != 0x60baa72 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x1acu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x1b0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60baa89; continue;
 }
 0x60baa89 => {
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060baa90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60baa90); }
 let mut pc: u32 = 0x60baa90;
 loop { match pc {
 0x60baa90 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf0u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if cpu.cf { pc = 0x60baabf; continue; }
  pc = 0x60baaa0; continue;
 }
 0x60baaa0 => {
  if (cpu.cf || cpu.zf) { pc = 0x60baad9; continue; }
  pc = 0x60baaa2; continue;
 }
 0x60baaa2 => {
  { cpu.sub32(cpu.eax, 0xd0u32); }
  if cpu.cf { pc = 0x60baab5; continue; }
  pc = 0x60baaa9; continue;
 }
 0x60baaa9 => {
  if (cpu.cf || cpu.zf) { pc = 0x60baadf; continue; }
  pc = 0x60baaab; continue;
 }
 0x60baaab => {
  { cpu.sub32(cpu.eax, 0xe0u32); }
  if cpu.zf { pc = 0x60baad9; continue; }
  pc = 0x60baab2; continue;
 }
 0x60baab2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60baab5 => {
  { cpu.sub32(cpu.eax, 0xc0u32); }
  if cpu.zf { pc = 0x60baadf; continue; }
  pc = 0x60baabc; continue;
 }
 0x60baabc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60baabf => {
  { cpu.sub32(cpu.eax, 0x90u32); }
  if cpu.cf { pc = 0x60baad2; continue; }
  pc = 0x60baac6; continue;
 }
 0x60baac6 => {
  if (cpu.cf || cpu.zf) { pc = 0x60baad9; continue; }
  pc = 0x60baac8; continue;
 }
 0x60baac8 => {
  { cpu.sub32(cpu.eax, 0xa0u32); }
  if cpu.zf { pc = 0x60baad9; continue; }
  pc = 0x60baacf; continue;
 }
 0x60baacf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60baad2 => {
  { cpu.sub32(cpu.eax, 0x80u32); }
  if !cpu.zf { pc = 0x60baae5; continue; }
  pc = 0x60baad9; continue;
 }
 0x60baad9 => {
  { let v: u32 = 0x3u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60baadf => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60baae5 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060baaf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60baaf0); }
 let mut pc: u32 = 0x60baaf0;
 loop { match pc {
 0x60baaf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c740u32), 0x0u32); }
  if !cpu.zf { pc = 0x60bac01; continue; }
  pc = 0x60bab00; continue;
 }
 0x60bab00 => {
  { let v = 0x60b7310u32; cpu.push(v); }
  { let v = 0x60baaf0u32; cpu.push(v); }
  cpu.push(0x60bab0f); f_060b3160(cpu); if cpu.returned_to != 0x60bab0f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x80u32; cpu.push(v); }
  { let v = 0x617c6bcu32; cpu.push(v); }
  cpu.push(0x60bab21); f_060b30a0(cpu); if cpu.returned_to != 0x60bab21 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c73cu32; cpu.push(v); }
  cpu.push(0x60bab30); f_060b30a0(cpu); if cpu.returned_to != 0x60bab30 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f5cu32; cpu.push(v); }
  cpu.push(0x60bab3f); f_060b30a0(cpu); if cpu.returned_to != 0x60bab3f { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f60u32; cpu.push(v); }
  cpu.push(0x60bab4e); f_060b30a0(cpu); if cpu.returned_to != 0x60bab4e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f64u32; cpu.push(v); }
  cpu.push(0x60bab5d); f_060b30a0(cpu); if cpu.returned_to != 0x60bab5d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f68u32; cpu.push(v); }
  cpu.push(0x60bab6c); f_060b30a0(cpu); if cpu.returned_to != 0x60bab6c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f78u32; cpu.push(v); }
  cpu.push(0x60bab7b); f_060b30a0(cpu); if cpu.returned_to != 0x60bab7b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f6cu32; cpu.push(v); }
  cpu.push(0x60bab8a); f_060b30a0(cpu); if cpu.returned_to != 0x60bab8a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f70u32; cpu.push(v); }
  cpu.push(0x60bab99); f_060b30a0(cpu); if cpu.returned_to != 0x60bab99 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f58u32; cpu.push(v); }
  cpu.push(0x60baba8); f_060b30a0(cpu); if cpu.returned_to != 0x60baba8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f74u32; cpu.push(v); }
  cpu.push(0x60babb7); f_060b30a0(cpu); if cpu.returned_to != 0x60babb7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f54u32; cpu.push(v); }
  cpu.push(0x60babc6); f_060b30a0(cpu); if cpu.returned_to != 0x60babc6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f50u32; cpu.push(v); }
  cpu.push(0x60babd5); f_060b30a0(cpu); if cpu.returned_to != 0x60babd5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f48u32; cpu.push(v); }
  cpu.push(0x60babe4); f_060b30a0(cpu); if cpu.returned_to != 0x60babe4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d1f4cu32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60babf8); f_060b30a0(cpu); if cpu.returned_to != 0x60babf8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c740u32, v); }
  pc = 0x60bac01; continue;
 }
 0x60bac01 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bac10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bac10); } r_060bac10(cpu, 0x60bac10); }
pub fn f_060bae10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bae10); } r_060bac10(cpu, 0x60bae10); }
fn r_060bac10(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60bac10 => {
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
  if cpu.zf { pc = 0x60bac9a; continue; }
  pc = 0x60bac9a; continue;
 }
 0x60bac9a => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if !cpu.pf() { pc = 0x60bac9e; continue; }
  pc = 0x60bac9e; continue;
 }
 0x60bac9e => {
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
  cpu.unimplemented(0x60bad0c, "Outsb");
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
  if (cpu.sf == cpu.of) { pc = 0x60bad58; continue; }
  pc = 0x60bad52; continue;
 }
 0x60bad52 => {
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 && !cpu.zf } { pc = 0x60bad5c; continue; }
  pc = 0x60bad56; continue;
 }
 0x60bad56 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60bad58; continue;
 }
 0x60bad58 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x60bad5c; continue;
 }
 0x60bad5c => {
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
  cpu.unimplemented(0x60bad8c, "Outsd");
  cpu.unimplemented(0x60bad8d, "Sldt");
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
  pc = 0x60bae18; continue;
 }
 0x60bae10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c750u32), 0x0u32); }
  if cpu.zf { pc = 0x60bae36; continue; }
  pc = 0x60bae1c; continue;
 }
 0x60bae18 => {
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.add8(cpu.r8(0x68u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1))), ((cpu.edx >> 8) as u8)); cpu.w8(0x68u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)), v); }
  { let v: u8 = cpu.adc8(cpu.r8(0x8068060bu32.wrapping_add(cpu.esi)), ((cpu.ecx >> 8) as u8)); cpu.w8(0x8068060bu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0xdb31060bu32; cpu.eax = v; }
  cpu.push(0x60bae2d); f_060b30c0(cpu); if cpu.returned_to != 0x60bae2d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c750u32, v); }
  pc = 0x60bae36; continue;
 }
 0x60bae1c => {
  { let v = 0x60bae10u32; cpu.push(v); }
  { let v = 0x60bb880u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60bae2d); f_060b30c0(cpu); if cpu.returned_to != 0x60bae2d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c750u32, v); }
  pc = 0x60bae36; continue;
 }
 0x60bae36 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bae40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bae40); }
 let mut pc: u32 = 0x60bae40;
 loop { match pc {
 0x60bae40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60bae9e; continue; }
  pc = 0x60bae4c; continue;
 }
 0x60bae4c => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bae77; continue; }
  pc = 0x60bae58; continue;
 }
 0x60bae58 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x60bae5a; continue;
 }
 0x60bae5a => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.push(0x60bae6a); f_060ab760(cpu); if cpu.returned_to != 0x60bae6a { return; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60bae5a; continue; }
  pc = 0x60bae77; continue;
 }
 0x60bae77 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bae83); f_060a7550(cpu); if cpu.returned_to != 0x60bae83 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bae92); f_060a7440(cpu); if cpu.returned_to != 0x60bae92 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60bae9b); f_060b32d0(cpu); if cpu.returned_to != 0x60bae9b { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60bae9e; continue;
 }
 0x60bae9e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb0b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb0b0); }
 let mut pc: u32 = 0x60bb0b0;
 loop { match pc {
 0x60bb0b0 => {
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
  if cpu.zf { pc = 0x60bb0e6; continue; }
  pc = 0x60bb0d6; continue;
 }
 0x60bb0d6 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.zf { pc = 0x60bb0e6; continue; }
  pc = 0x60bb0de; continue;
 }
 0x60bb0de => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if !cpu.zf { pc = 0x60bb0fd; continue; }
  pc = 0x60bb0e6; continue;
 }
 0x60bb0e6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x14u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bb0fa); f_060bb6b0(cpu); if cpu.returned_to != 0x60bb0fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  pc = 0x60bb0fd; continue;
 }
 0x60bb0fd => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if !cpu.zf { pc = 0x60bb111; continue; }
  pc = 0x60bb105; continue;
 }
 0x60bb105 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x60bb111; continue; }
  pc = 0x60bb10c; continue;
 }
 0x60bb10c => {
  { let v: u32 = 0x80u32; cpu.ebx = v; }
  pc = 0x60bb111; continue;
 }
 0x60bb111 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi.wrapping_mul(4)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, 0xb0u32); }
  if cpu.cf { pc = 0x60bb147; continue; }
  pc = 0x60bb122; continue;
 }
 0x60bb122 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb16a; continue; }
  pc = 0x60bb124; continue;
 }
 0x60bb124 => {
  { cpu.sub32(cpu.ebx, 0xc0u32); }
  if cpu.cf { pc = 0x60bb4c8; continue; }
  pc = 0x60bb130; continue;
 }
 0x60bb130 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb288; continue; }
  pc = 0x60bb136; continue;
 }
 0x60bb136 => {
  { cpu.sub32(cpu.ebx, 0xe0u32); }
  if cpu.zf { pc = 0x60bb2a8; continue; }
  pc = 0x60bb142; continue;
 }
 0x60bb142 => {
  pc = 0x60bb4c8; continue;
 }
 0x60bb147 => {
  { cpu.sub32(cpu.ebx, 0x80u32); }
  if cpu.cf { pc = 0x60bb4c8; continue; }
  pc = 0x60bb153; continue;
 }
 0x60bb153 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb465; continue; }
  pc = 0x60bb159; continue;
 }
 0x60bb159 => {
  { cpu.sub32(cpu.ebx, 0x90u32); }
  if cpu.zf { pc = 0x60bb2f3; continue; }
  pc = 0x60bb165; continue;
 }
 0x60bb165 => {
  pc = 0x60bb4c8; continue;
 }
 0x60bb16a => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x7u32); }
  if cpu.cf { pc = 0x60bb194; continue; }
  pc = 0x60bb173; continue;
 }
 0x60bb173 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb1a1; continue; }
  pc = 0x60bb175; continue;
 }
 0x60bb175 => {
  { cpu.sub32(cpu.edx, 0xau32); }
  if cpu.cf { pc = 0x60bb4c8; continue; }
  pc = 0x60bb17e; continue;
 }
 0x60bb17e => {
  { cpu.sub32(cpu.edx, 0xbu32); }
  if (cpu.cf || cpu.zf) { pc = 0x60bb1a1; continue; }
  pc = 0x60bb183; continue;
 }
 0x60bb183 => {
  { cpu.sub32(cpu.edx, 0x7bu32); }
  if cpu.zf { pc = 0x60bb237; continue; }
  pc = 0x60bb18c; continue;
 }
 0x60bb18c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x60bb194 => {
  { cpu.sub32(cpu.edx, 0x6u32); }
  if cpu.zf { pc = 0x60bb1ec; continue; }
  pc = 0x60bb199; continue;
 }
 0x60bb199 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x60bb1a1 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb1b1; continue;
 }
 0x60bb1b1 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x60bb1b3; continue;
 }
 0x60bb1b3 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x60bb1d8; continue; }
  pc = 0x60bb1bb; continue;
 }
 0x60bb1bb => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bb1c7); f_060aac60(cpu); if cpu.returned_to != 0x60bb1c7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x60bb1d8; continue; }
  pc = 0x60bb1cf; continue;
 }
 0x60bb1cf => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bb1d8); f_060bb610(cpu); if cpu.returned_to != 0x60bb1d8 { return; }
  pc = 0x60bb1d8; continue;
 }
 0x60bb1d8 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb1ea; continue;
 }
 0x60bb1ea => {
  cpu.poll(); pc = 0x60bb1b3; continue;
 }
 0x60bb1ec => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb1fc; continue;
 }
 0x60bb1fc => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x60bb1fe; continue;
 }
 0x60bb1fe => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x60bb223; continue; }
  pc = 0x60bb206; continue;
 }
 0x60bb206 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bb212); f_060aac60(cpu); if cpu.returned_to != 0x60bb212 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x60bb223; continue; }
  pc = 0x60bb21a; continue;
 }
 0x60bb21a => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bb223); f_060bb500(cpu); if cpu.returned_to != 0x60bb223 { return; }
  pc = 0x60bb223; continue;
 }
 0x60bb223 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb235; continue;
 }
 0x60bb235 => {
  cpu.poll(); pc = 0x60bb1fe; continue;
 }
 0x60bb237 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb247; continue;
 }
 0x60bb247 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x60bb249; continue;
 }
 0x60bb249 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x60bb274; continue; }
  pc = 0x60bb251; continue;
 }
 0x60bb251 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bb25d); f_060aac60(cpu); if cpu.returned_to != 0x60bb25d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x60bb274; continue; }
  pc = 0x60bb265; continue;
 }
 0x60bb265 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bb271); f_060ab130(cpu); if cpu.returned_to != 0x60bb271 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60bb274; continue;
 }
 0x60bb274 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb286; continue;
 }
 0x60bb286 => {
  cpu.poll(); pc = 0x60bb249; continue;
 }
 0x60bb288 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x1d4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x60bb29a); f_060bb680(cpu); if cpu.returned_to != 0x60bb29a { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x60bb2a8 => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb2b8; continue;
 }
 0x60bb2b8 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x60bb2ba; continue;
 }
 0x60bb2ba => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebp))); }
  if !cpu.zf { pc = 0x60bb2df; continue; }
  pc = 0x60bb2c2; continue;
 }
 0x60bb2c2 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bb2ce); f_060aac60(cpu); if cpu.returned_to != 0x60bb2ce { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x60bb2df; continue; }
  pc = 0x60bb2d6; continue;
 }
 0x60bb2d6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bb2df); f_060bb500(cpu); if cpu.returned_to != 0x60bb2df { return; }
  pc = 0x60bb2df; continue;
 }
 0x60bb2df => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb2f1; continue;
 }
 0x60bb2f1 => {
  cpu.poll(); pc = 0x60bb2ba; continue;
 }
 0x60bb2f3 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if !cpu.zf { pc = 0x60bb30f; continue; }
  pc = 0x60bb2f8; continue;
 }
 0x60bb2f8 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  cpu.push(0x60bb309); f_060bb680(cpu); if cpu.returned_to != 0x60bb309 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x494u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60bb30f; continue;
 }
 0x60bb30f => {
  { cpu.sub32(cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))), 0x0u32); }
  if cpu.zf { pc = 0x60bb4c8; continue; }
  pc = 0x60bb31d; continue;
 }
 0x60bb31d => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb34d; continue; }
  pc = 0x60bb329; continue;
 }
 0x60bb329 => {
  { let v: u32 = cpu.esi; cpu.ebp = v; }
  pc = 0x60bb32b; continue;
 }
 0x60bb32b => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bb337); f_060aac60(cpu); if cpu.returned_to != 0x60bb337 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x60bb34d; continue; }
  pc = 0x60bb33f; continue;
 }
 0x60bb33f => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60bb32b; continue; }
  pc = 0x60bb34d; continue;
 }
 0x60bb34d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x514u32.wrapping_add(cpu.esi))); }
  if !cpu.zf { pc = 0x60bb390; continue; }
  pc = 0x60bb355; continue;
 }
 0x60bb355 => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb380; continue; }
  pc = 0x60bb362; continue;
 }
 0x60bb362 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x60bb364; continue;
 }
 0x60bb364 => {
  { let v: u32 = cpu.r32(0x658u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.ecx); }
  if cpu.cf { pc = 0x60bb372; continue; }
  pc = 0x60bb36e; continue;
 }
 0x60bb36e => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  pc = 0x60bb372; continue;
 }
 0x60bb372 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60bb364; continue; }
  pc = 0x60bb380; continue;
 }
 0x60bb380 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60bb38d); f_060ab130(cpu); if cpu.returned_to != 0x60bb38d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60bb390; continue;
 }
 0x60bb390 => {
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
  cpu.push(0x60bb3f2); f_060ab6b0(cpu); if cpu.returned_to != 0x60bb3f2 { return; }
  { let v: u32 = cpu.r32(0x494u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60bb411); f_060ab3f0(cpu); if cpu.returned_to != 0x60bb411 { return; }
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
  cpu.push(0x60bb435); f_060ab4c0(cpu); if cpu.returned_to != 0x60bb435 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bb441); f_060bb500(cpu); if cpu.returned_to != 0x60bb441 { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bb44a); f_060bb610(cpu); if cpu.returned_to != 0x60bb44a { return; }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60bb457); f_060ab340(cpu); if cpu.returned_to != 0x60bb457 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x60bb465 => {
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x60bb4c8; continue; }
  pc = 0x60bb46a; continue;
 }
 0x60bb46a => {
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb4c8; continue; }
  pc = 0x60bb476; continue;
 }
 0x60bb476 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x60bb478; continue;
 }
 0x60bb478 => {
  { cpu.sub32(cpu.edi, cpu.r32(0x518u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x60bb4ba; continue; }
  pc = 0x60bb480; continue;
 }
 0x60bb480 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x558u32.wrapping_add(cpu.ebx))); }
  if !cpu.zf { pc = 0x60bb4ba; continue; }
  pc = 0x60bb48c; continue;
 }
 0x60bb48c => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bb498); f_060aac60(cpu); if cpu.returned_to != 0x60bb498 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if !cpu.zf { pc = 0x60bb4ba; continue; }
  pc = 0x60bb4a0; continue;
 }
 0x60bb4a0 => {
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bb4ac); f_060ab130(cpu); if cpu.returned_to != 0x60bb4ac { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 0x60bb4ba => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x514u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60bb478; continue; }
  pc = 0x60bb4c8; continue;
 }
 0x60bb4c8 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(24);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb4d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb4d0); }
 let mut pc: u32 = 0x60bb4d0;
 loop { match pc {
 0x60bb4d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1b8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.push(0x60bb4ea); f_060bb680(cpu); if cpu.returned_to != 0x60bb4ea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb500(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb500); }
 let mut pc: u32 = 0x60bb500;
 loop { match pc {
 0x60bb500 => {
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
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb556; continue; }
  pc = 0x60bb54a; continue;
 }
 0x60bb54a => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x1fffu32); cpu.eax = v; }
  pc = 0x60bb56c; continue;
 }
 0x60bb556 => {
  if (cpu.sf == cpu.of) { pc = 0x60bb568; continue; }
  pc = 0x60bb558; continue;
 }
 0x60bb558 => {
  { let v: u32 = 0x2000u32; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x60bb56c; continue;
 }
 0x60bb568 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60bb56c; continue;
 }
 0x60bb56c => {
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x454u32.wrapping_add(cpu.esi).wrapping_add(cpu.edi.wrapping_mul(4)))); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x60bb57e; continue; }
  pc = 0x60bb57c; continue;
 }
 0x60bb57c => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60bb57e; continue;
 }
 0x60bb57e => {
  { cpu.sub32(cpu.edx, 0x7fu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x60bb588; continue; }
  pc = 0x60bb583; continue;
 }
 0x60bb583 => {
  { let v: u32 = 0x7fu32; cpu.edx = v; }
  pc = 0x60bb588; continue;
 }
 0x60bb588 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60bac10u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x60bac10u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x60bac10u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 13u32); cpu.edx = v; }
  { let v: u32 = cpu.sbb32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 13u32); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.eax.wrapping_mul(1)); cpu.ebp = v; }
  { cpu.sub32(cpu.edi, 0x9u32); }
  if cpu.zf { pc = 0x60bb5d9; continue; }
  pc = 0x60bb5c2; continue;
 }
 0x60bb5c2 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x60bb5e0; continue;
 }
 0x60bb5d9 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  pc = 0x60bb5e0; continue;
 }
 0x60bb5e0 => {
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bb5f9); f_060ab070(cpu); if cpu.returned_to != 0x60bb5f9 { return; }
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
pub fn f_060bb610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb610); }
 let mut pc: u32 = 0x60bb610;
 loop { match pc {
 0x60bb610 => {
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
  cpu.push(0x60bb65d); f_060aafb0(cpu); if cpu.returned_to != 0x60bb65d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x314u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4d4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bb673); f_060aaef0(cpu); if cpu.returned_to != 0x60bb673 { return; }
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
pub fn f_060bb680(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb680); }
 let mut pc: u32 = 0x60bb680;
 loop { match pc {
 0x60bb680 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x60bb686; continue;
 }
 0x60bb686 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, 0xffffffffu32); }
  if cpu.zf { pc = 0x60bb6a2; continue; }
  pc = 0x60bb68d; continue;
 }
 0x60bb68d => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if !cpu.zf { pc = 0x60bb696; continue; }
  pc = 0x60bb691; continue;
 }
 0x60bb691 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.eax))); }
  if cpu.zf { pc = 0x60bb6a4; continue; }
  pc = 0x60bb696; continue;
 }
 0x60bb696 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x20u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x200u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x60bb686; continue; }
  pc = 0x60bb6a2; continue;
 }
 0x60bb6a2 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60bb6a4; continue;
 }
 0x60bb6a4 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb6b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb6b0); }
 let mut pc: u32 = 0x60bb6b0;
 loop { match pc {
 0x60bb6b0 => {
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
  if cpu.cf { pc = 0x60bb6ed; continue; }
  pc = 0x60bb6df; continue;
 }
 0x60bb6df => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb6f9; continue; }
  pc = 0x60bb6e1; continue;
 }
 0x60bb6e1 => {
  { cpu.sub32(cpu.edi, 0xe0u32); }
  if cpu.zf { pc = 0x60bb704; continue; }
  pc = 0x60bb6e9; continue;
 }
 0x60bb6e9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb6ed => {
  { cpu.sub32(cpu.edi, 0xb0u32); }
  if cpu.zf { pc = 0x60bb71a; continue; }
  pc = 0x60bb6f5; continue;
 }
 0x60bb6f5 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb6f9 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb704 => {
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
 0x60bb71a => {
  { cpu.sub32(cpu.eax, 0x5bu32); }
  if cpu.cf { pc = 0x60bb76b; continue; }
  pc = 0x60bb71f; continue;
 }
 0x60bb71f => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb851; continue; }
  pc = 0x60bb725; continue;
 }
 0x60bb725 => {
  { cpu.sub32(cpu.eax, 0x70u32); }
  if cpu.cf { pc = 0x60bb755; continue; }
  pc = 0x60bb72a; continue;
 }
 0x60bb72a => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb7cf; continue; }
  pc = 0x60bb730; continue;
 }
 0x60bb730 => {
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.cf { pc = 0x60bb748; continue; }
  pc = 0x60bb735; continue;
 }
 0x60bb735 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb7eb; continue; }
  pc = 0x60bb73b; continue;
 }
 0x60bb73b => {
  { cpu.sub32(cpu.eax, 0x77u32); }
  if cpu.zf { pc = 0x60bb7f9; continue; }
  pc = 0x60bb744; continue;
 }
 0x60bb744 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb748 => {
  { cpu.sub32(cpu.eax, 0x72u32); }
  if cpu.zf { pc = 0x60bb7dd; continue; }
  pc = 0x60bb751; continue;
 }
 0x60bb751 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb755 => {
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if cpu.cf { pc = 0x60bb75e; continue; }
  pc = 0x60bb75a; continue;
 }
 0x60bb75a => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb7b3; continue; }
  pc = 0x60bb75c; continue;
 }
 0x60bb75c => {
  pc = 0x60bb7c1; continue;
 }
 0x60bb75e => {
  { cpu.sub32(cpu.eax, 0x5du32); }
  if cpu.zf { pc = 0x60bb85f; continue; }
  pc = 0x60bb767; continue;
 }
 0x60bb767 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb76b => {
  { cpu.sub32(cpu.eax, 0x7u32); }
  if cpu.cf { pc = 0x60bb79b; continue; }
  pc = 0x60bb770; continue;
 }
 0x60bb770 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb816; continue; }
  pc = 0x60bb776; continue;
 }
 0x60bb776 => {
  { cpu.sub32(cpu.eax, 0xbu32); }
  if cpu.cf { pc = 0x60bb78e; continue; }
  pc = 0x60bb77b; continue;
 }
 0x60bb77b => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb834; continue; }
  pc = 0x60bb781; continue;
 }
 0x60bb781 => {
  { cpu.sub32(cpu.eax, 0x40u32); }
  if cpu.zf { pc = 0x60bb843; continue; }
  pc = 0x60bb78a; continue;
 }
 0x60bb78a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb78e => {
  { cpu.sub32(cpu.eax, 0xau32); }
  if cpu.zf { pc = 0x60bb825; continue; }
  pc = 0x60bb797; continue;
 }
 0x60bb797 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb79b => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x60bb878; continue; }
  pc = 0x60bb7a4; continue;
 }
 0x60bb7a4 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bb807; continue; }
  pc = 0x60bb7a6; continue;
 }
 0x60bb7a6 => {
  { cpu.sub32(cpu.eax, 0x6u32); }
  if cpu.zf { pc = 0x60bb86d; continue; }
  pc = 0x60bb7af; continue;
 }
 0x60bb7af => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb7b3 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb7c1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb7cf => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x180u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb7dd => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x1c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb7eb => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x200u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb7f9 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x240u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb807 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x280u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb816 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x2c0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb825 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x300u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb834 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x340u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb843 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x380u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb851 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x3c0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb85f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x400u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bb86d => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x440u32.wrapping_add(cpu.esi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  pc = 0x60bb878; continue;
 }
 0x60bb878 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb880(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb880); }
 let mut pc: u32 = 0x60bb880;
 loop { match pc {
 0x60bb880 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c750u32), 0x0u32); }
  if !cpu.zf { pc = 0x60bb8a9; continue; }
  pc = 0x60bb88c; continue;
 }
 0x60bb88c => {
  { let v = 0x60bae10u32; cpu.push(v); }
  { let v = 0x60bb880u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60bb8a0); f_060b3160(cpu); if cpu.returned_to != 0x60bb8a0 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c750u32, v); }
  pc = 0x60bb8a9; continue;
 }
 0x60bb8a9 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb8b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb8b0); }
 let mut pc: u32 = 0x60bb8b0;
 loop { match pc {
 0x60bb8b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bb8bb); f_060a6650(cpu); if cpu.returned_to != 0x60bb8bb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { pc = 0x60bb8db; continue; }
  pc = 0x60bb8c3; continue;
 }
 0x60bb8c3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60bb8d8); f_060a6040(cpu); if cpu.returned_to != 0x60bb8d8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60bb8db; continue;
 }
 0x60bb8db => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bb8e1); f_060a6650(cpu); if cpu.returned_to != 0x60bb8e1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb8f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb8f0); }
 let mut pc: u32 = 0x60bb8f0;
 loop { match pc {
 0x60bb8f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bb8fc); f_060a6650(cpu); if cpu.returned_to != 0x60bb8fc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60bb930; continue; }
  pc = 0x60bb904; continue;
 }
 0x60bb904 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bb90a); f_060a6260(cpu); if cpu.returned_to != 0x60bb90a { return; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60bb930); f_060bc4c0(cpu); if cpu.returned_to != 0x60bb930 { return; }
  pc = 0x60bb930; continue;
 }
 0x60bb930 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bb936); f_060a6650(cpu); if cpu.returned_to != 0x60bb936 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb940(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb940); }
 let mut pc: u32 = 0x60bb940;
 loop { match pc {
 0x60bb940 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bb94c); f_060a6650(cpu); if cpu.returned_to != 0x60bb94c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bb98c; continue; }
  pc = 0x60bb953; continue;
 }
 0x60bb953 => {
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60bb973); f_060bc4c0(cpu); if cpu.returned_to != 0x60bb973 { return; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60bb98c); f_060bc4c0(cpu); if cpu.returned_to != 0x60bb98c { return; }
  pc = 0x60bb98c; continue;
 }
 0x60bb98c => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bb992); f_060a6650(cpu); if cpu.returned_to != 0x60bb992 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bb9a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bb9a0); }
 let mut pc: u32 = 0x60bb9a0;
 loop { match pc {
 0x60bb9a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bb9b2); f_060a6650(cpu); if cpu.returned_to != 0x60bb9b2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bba1f; continue; }
  pc = 0x60bb9b9; continue;
 }
 0x60bb9b9 => {
  { cpu.sub32(cpu.eax, 0x3u32); }
  if cpu.zf { pc = 0x60bb9d7; continue; }
  pc = 0x60bb9be; continue;
 }
 0x60bb9be => {
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60bb9d7); f_060bc4c0(cpu); if cpu.returned_to != 0x60bb9d7 { return; }
  pc = 0x60bb9d7; continue;
 }
 0x60bb9d7 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.ebp); cpu.eax = v; }
  if cpu.zf { pc = 0x60bba1f; continue; }
  pc = 0x60bb9df; continue;
 }
 0x60bb9df => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x84u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ebp); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60bba03); f_060bc350(cpu); if cpu.returned_to != 0x60bba03 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60bba0d); f_060bc350(cpu); if cpu.returned_to != 0x60bba0d { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60bba1f); f_060bc4c0(cpu); if cpu.returned_to != 0x60bba1f { return; }
  pc = 0x60bba1f; continue;
 }
 0x60bba1f => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bba25); f_060a6650(cpu); if cpu.returned_to != 0x60bba25 { return; }
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
pub fn f_060bba30(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bba30); } r_060bba30(cpu, 0x60bba30); }
pub fn f_060bba60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x60bba60); } r_060bba30(cpu, 0x60bba60); }
fn r_060bba30(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x60bba30 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bba3a); f_060a6650(cpu); if cpu.returned_to != 0x60bba3a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60bba52; continue; }
  pc = 0x60bba42; continue;
 }
 0x60bba42 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x60bbef0; continue;
 }
 0x60bba52 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bba60 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bba6a); f_060a6650(cpu); if cpu.returned_to != 0x60bba6a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x60bba7f; continue; }
  pc = 0x60bba72; continue;
 }
 0x60bba72 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x60bbef0; continue;
 }
 0x60bba7f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bbef0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u16 = 0xbu16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x60bbf49); f_060bc4c0(cpu); if cpu.returned_to != 0x60bbf49 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bbfe2; continue; }
  pc = 0x60bbf51; continue;
 }
 0x60bbf51 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x60bbfb6; continue; }
  pc = 0x60bbf55; continue;
 }
 0x60bbf55 => {
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
  pc = 0x60bbfe2; continue;
 }
 0x60bbfb6 => {
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
  pc = 0x60bbfe2; continue;
 }
 0x60bbfe2 => {
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
pub fn f_060bbb40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbb40); }
 let mut pc: u32 = 0x60bbb40;
 loop { match pc {
 0x60bbb40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bbb5a); f_060a6650(cpu); if cpu.returned_to != 0x60bbb5a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bbb6b; continue; }
  pc = 0x60bbb61; continue;
 }
 0x60bbb61 => {
  { cpu.sub32(cpu.esi, 0x1u32); }
  if cpu.cf { pc = 0x60bbb6b; continue; }
  pc = 0x60bbb66; continue;
 }
 0x60bbb66 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x14u32.wrapping_add(cpu.ebx))); }
  if (cpu.cf || cpu.zf) { pc = 0x60bbb85; continue; }
  pc = 0x60bbb6b; continue;
 }
 0x60bbb6b => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60bbb75; continue; }
  pc = 0x60bbb6f; continue;
 }
 0x60bbb6f => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x60bbb75; continue;
 }
 0x60bbb75 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60bbb9b; continue; }
  pc = 0x60bbb79; continue;
 }
 0x60bbb79 => {
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bbb85 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60bbb8f; continue; }
  pc = 0x60bbb89; continue;
 }
 0x60bbb89 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x60bbb8f; continue;
 }
 0x60bbb8f => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x60bbb9b; continue; }
  pc = 0x60bbb93; continue;
 }
 0x60bbb93 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx).wrapping_add(cpu.esi.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60bbb9b; continue;
 }
 0x60bbb9b => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bbba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbba0); }
 let mut pc: u32 = 0x60bbba0;
 loop { match pc {
 0x60bbba0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bbbaa); f_060a6650(cpu); if cpu.returned_to != 0x60bbbaa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bbbb2; continue; }
  pc = 0x60bbbb1; continue;
 }
 0x60bbbb1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bbbb2 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bbbc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbbc0); }
 let mut pc: u32 = 0x60bbbc0;
 loop { match pc {
 0x60bbbc0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60bbc19; continue; }
  pc = 0x60bbbc9; continue;
 }
 0x60bbbc9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x60bbbd0); f_060bbd30(cpu); if cpu.returned_to != 0x60bbbd0 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bbc0b; continue; }
  pc = 0x60bbbd6; continue;
 }
 0x60bbbd6 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60bbbf1; continue; }
  pc = 0x60bbbdc; continue;
 }
 0x60bbbdc => {
  { let v: u32 = 0x14u32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x60bbbe6); f_060bc1f0(cpu); if cpu.returned_to != 0x60bbbe6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bbc0b; continue; }
  pc = 0x60bbbea; continue;
 }
 0x60bbbea => {
  { let v: u32 = 0x0u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bbbf1; continue;
 }
 0x60bbbf1 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x60bbbfa; continue; }
  pc = 0x60bbbf6; continue;
 }
 0x60bbbf6 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bbbfa => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, 0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bbc0b => {
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bbc19; continue;
 }
 0x60bbc19 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bbc20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbc20); }
 let mut pc: u32 = 0x60bbc20;
 loop { match pc {
 0x60bbc20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60bbc3c; continue; }
  pc = 0x60bbc29; continue;
 }
 0x60bbc29 => {
  { let v: u32 = 0x5u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x60bbc35); f_060bbff0(cpu); if cpu.returned_to != 0x60bbc35 { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bbc3c; continue;
 }
 0x60bbc3c => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bbc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbc60); }
 let mut pc: u32 = 0x60bbc60;
 loop { match pc {
 0x60bbc60 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60bbcaa; continue; }
  pc = 0x60bbc6a; continue;
 }
 0x60bbc6a => {
  { let v: u32 = cpu.r32(0x617c754u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60bbc9d; continue; }
  pc = 0x60bbc74; continue;
 }
 0x60bbc74 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617c754u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60bbc9d; continue; }
  pc = 0x60bbc81; continue;
 }
 0x60bbc81 => {
  { let v: u32 = cpu.r32(0x62d2184u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bbc9a); f_060b3200(cpu); if cpu.returned_to != 0x60bbc9a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x60bbc9d; continue;
 }
 0x60bbc9d => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bbca7); f_060b32d0(cpu); if cpu.returned_to != 0x60bbca7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60bbcaa; continue;
 }
 0x60bbcaa => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bbcb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbcb0); }
 let mut pc: u32 = 0x60bbcb0;
 loop { match pc {
 0x60bbcb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = 0x1a8u32; cpu.push(v); }
  cpu.push(0x60bbcc2); f_060b3300(cpu); if cpu.returned_to != 0x60bbcc2 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bbd1a; continue; }
  pc = 0x60bbccb; continue;
 }
 0x60bbccb => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60bbcd2); f_060bc0a0(cpu); if cpu.returned_to != 0x60bbcd2 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.eax, 0xffu32); }
  if !cpu.zf { pc = 0x60bbce8; continue; }
  pc = 0x60bbcdb; continue;
 }
 0x60bbcdb => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bbce1); f_060b32d0(cpu); if cpu.returned_to != 0x60bbce1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x60bbd1a; continue;
 }
 0x60bbce8 => {
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
  pc = 0x60bbd1a; continue;
 }
 0x60bbd1a => {
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
pub fn f_060bbd30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbd30); }
 let mut pc: u32 = 0x60bbd30;
 loop { match pc {
 0x60bbd30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u16 = 0x5u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = 0x6u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x60bbd81); f_060bc4c0(cpu); if cpu.returned_to != 0x60bbd81 { return; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x80u8); }
  if !cpu.zf { pc = 0x60bbd8f; continue; }
  pc = 0x60bbd89; continue;
 }
 0x60bbd89 => {
  { cpu.and8(cpu.r8(0x1u32.wrapping_add(cpu.edi)), 0x1u8); }
  if cpu.zf { pc = 0x60bbd93; continue; }
  pc = 0x60bbd8f; continue;
 }
 0x60bbd8f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60bbd9f; continue;
 }
 0x60bbd93 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60bbd9f; continue;
 }
 0x60bbd9f => {
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
pub fn f_060bbdb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbdb0); }
 let mut pc: u32 = 0x60bbdb0;
 loop { match pc {
 0x60bbdb0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x85u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  cpu.push(0x60bbdca); f_060bc4c0(cpu); if cpu.returned_to != 0x60bbdca { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bbff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bbff0); }
 let mut pc: u32 = 0x60bbff0;
 loop { match pc {
 0x60bbff0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.edx = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u16 = 0x1u16; cpu.w16(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = 0xcu8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bc045); f_060bc4c0(cpu); if cpu.returned_to != 0x60bc045 { return; }
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
pub fn f_060bc0a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc0a0); }
 let mut pc: u32 = 0x60bc0a0;
 loop { match pc {
 0x60bc0a0 => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60bc119); cpu.call(t); if cpu.returned_to != 0x60bc119 { return; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { pc = 0x60bc129; continue; }
  pc = 0x60bc11f; continue;
 }
 0x60bc11f => {
  { let v: u32 = 0xffu32; cpu.eax = v; }
  pc = 0x60bc1dc; continue;
 }
 0x60bc129 => {
  { cpu.sub32(cpu.r32(0x617c754u32), 0x0u32); }
  if !cpu.zf { pc = 0x60bc167; continue; }
  pc = 0x60bc132; continue;
 }
 0x60bc132 => {
  { let v = 0x62d2184u32; cpu.push(v); }
  { let v = 0x62d217cu32; cpu.push(v); }
  { let v = 0x62d2180u32; cpu.push(v); }
  { let v = 0x8u32; cpu.push(v); }
  cpu.push(0x60bc148); f_060b3240(cpu); if cpu.returned_to != 0x60bc148 { return; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { pc = 0x60bc167; continue; }
  pc = 0x60bc155; continue;
 }
 0x60bc155 => {
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
 0x60bc167 => {
  { let v: u32 = 0x150du32; cpu.ecx = v; }
  { let v: u32 = 0x300u32; cpu.ebp = v; }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.eax = v; }
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60bc1c2); cpu.call(t); if cpu.returned_to != 0x60bc1c2 { return; }
  { let v: u32 = cpu.r32(0x617c754u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c754u32, v); }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x60bc1dc; continue;
 }
 0x60bc1dc => {
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
pub fn f_060bc1f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc1f0); }
 let mut pc: u32 = 0x60bc1f0;
 loop { match pc {
 0x60bc1f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x62d2180u32); cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u32 = 0x1au32.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = 0x21u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xau8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x60bc251); f_060bc4c0(cpu); if cpu.returned_to != 0x60bc251 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bc313; continue; }
  pc = 0x60bc259; continue;
 }
 0x60bc259 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x7u16; cpu.w16(0x5u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = 0xbu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60bc2e0; continue;
 }
 0x60bc281 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0xcu32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x62d217cu32); cpu.ebx = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.edi), v); }
  cpu.push(0x60bc2b3); f_060bc4c0(cpu); if cpu.returned_to != 0x60bc2b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bc313; continue; }
  pc = 0x60bc2b7; continue;
 }
 0x60bc2b7 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60bc2c9); f_060bc420(cpu); if cpu.returned_to != 0x60bc2c9 { return; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0xcu32.wrapping_add(cpu.esp))); cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x60bc2e0; continue;
 }
 0x60bc2e0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x2u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x60bc281; continue; }
  pc = 0x60bc2ed; continue;
 }
 0x60bc2ed => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  cpu.push(0x60bc302); f_060bc420(cpu); if cpu.returned_to != 0x60bc302 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x60bc315; continue;
 }
 0x60bc313 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60bc315; continue;
 }
 0x60bc315 => {
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
pub fn f_060bc320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc320); }
 let mut pc: u32 = 0x60bc320;
 loop { match pc {
 0x60bc320 => {
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
pub fn f_060bc350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc350); }
 let mut pc: u32 = 0x60bc350;
 loop { match pc {
 0x60bc350 => {
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
pub fn f_060bc3c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc3c0); }
 let mut pc: u32 = 0x60bc3c0;
 loop { match pc {
 0x60bc3c0 => {
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
pub fn f_060bc420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc420); }
 let mut pc: u32 = 0x60bc420;
 loop { match pc {
 0x60bc420 => {
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
pub fn f_060bc490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc490); }
 let mut pc: u32 = 0x60bc490;
 loop { match pc {
 0x60bc490 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = 0xdu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x3u8; cpu.w8(0x2u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x7u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x0u32; cpu.w32(0x9u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bc4b1); f_060bc4c0(cpu); if cpu.returned_to != 0x60bc4b1 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc4c0); }
 let mut pc: u32 = 0x60bc4c0;
 loop { match pc {
 0x60bc4c0 => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60bc562); cpu.call(t); if cpu.returned_to != 0x60bc562 { return; }
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
pub fn f_060bc580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc580); }
 let mut pc: u32 = 0x60bc580;
 loop { match pc {
 0x60bc580 => {
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
  let t = cpu.r32(0x617b954u32); cpu.push(0x60bc5ba); cpu.call(t); if cpu.returned_to != 0x60bc5ba { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x38u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc5c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc5c0); }
 let mut pc: u32 = 0x60bc5c0;
 loop { match pc {
 0x60bc5c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c76cu32), 0x0u32); }
  if cpu.zf { pc = 0x60bc5e6; continue; }
  pc = 0x60bc5cc; continue;
 }
 0x60bc5cc => {
  { let v = 0x60bc5c0u32; cpu.push(v); }
  { let v = 0x60bcd20u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x60bc5dd); f_060b30c0(cpu); if cpu.returned_to != 0x60bc5dd { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c76cu32, v); }
  pc = 0x60bc5e6; continue;
 }
 0x60bc5e6 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc5f0); }
 let mut pc: u32 = 0x60bc5f0;
 loop { match pc {
 0x60bc5f0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bc5fe; continue; }
  pc = 0x60bc5f8; continue;
 }
 0x60bc5f8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc5fe => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc610(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc610); }
 let mut pc: u32 = 0x60bc610;
 loop { match pc {
 0x60bc610 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc61c); f_060a5bc0(cpu); if cpu.returned_to != 0x60bc61c { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bc691; continue; }
  pc = 0x60bc62b; continue;
 }
 0x60bc62b => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc636); f_060a5a00(cpu); if cpu.returned_to != 0x60bc636 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bc64b; continue; }
  pc = 0x60bc63d; continue;
 }
 0x60bc63d => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x60bc643); f_060a5b10(cpu); if cpu.returned_to != 0x60bc643 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc64b => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60bc68f; continue; }
  pc = 0x60bc652; continue;
 }
 0x60bc652 => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x60bc68f; continue; }
  pc = 0x60bc659; continue;
 }
 0x60bc659 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x60bc68f; continue; }
  pc = 0x60bc661; continue;
 }
 0x60bc661 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bc668; continue; }
  pc = 0x60bc663; continue;
 }
 0x60bc663 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  pc = 0x60bc68d; continue;
 }
 0x60bc668 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc671); f_060a8b40(cpu); if cpu.returned_to != 0x60bc671 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  if cpu.zf { cpu.poll(); pc = 0x60bc668; continue; }
  pc = 0x60bc679; continue;
 }
 0x60bc679 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc67e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc687); f_060aac60(cpu); if cpu.returned_to != 0x60bc687 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x4u32); }
  pc = 0x60bc68d; continue;
 }
 0x60bc68d => {
  if cpu.zf { cpu.poll(); pc = 0x60bc67e; continue; }
  pc = 0x60bc68f; continue;
 }
 0x60bc68f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  pc = 0x60bc691; continue;
 }
 0x60bc691 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc6a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc6a0); }
 let mut pc: u32 = 0x60bc6a0;
 loop { match pc {
 0x60bc6a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60bc6e3; continue; }
  pc = 0x60bc6a9; continue;
 }
 0x60bc6a9 => {
  cpu.push(0x60bc6ae); f_060ad100(cpu); if cpu.returned_to != 0x60bc6ae { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60bc6de; continue; }
  pc = 0x60bc6b5; continue;
 }
 0x60bc6b5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x60bc6d7; continue; }
  pc = 0x60bc6bd; continue;
 }
 0x60bc6bd => {
  if (cpu.cf || cpu.zf) { pc = 0x60bc6c6; continue; }
  pc = 0x60bc6bf; continue;
 }
 0x60bc6bf => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x60bc6ce; continue; }
  pc = 0x60bc6c4; continue;
 }
 0x60bc6c4 => {
  pc = 0x60bc6d7; continue;
 }
 0x60bc6c6 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc6cc); f_060a8ea0(cpu); if cpu.returned_to != 0x60bc6cc { return; }
  pc = 0x60bc6d4; continue;
 }
 0x60bc6ce => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc6d4); f_060ab130(cpu); if cpu.returned_to != 0x60bc6d4 { return; }
  pc = 0x60bc6d4; continue;
 }
 0x60bc6d4 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60bc6d7; continue;
 }
 0x60bc6d7 => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bc6de; continue;
 }
 0x60bc6de => {
  cpu.push(0x60bc6e3); f_060ad0f0(cpu); if cpu.returned_to != 0x60bc6e3 { return; }
  pc = 0x60bc6e3; continue;
 }
 0x60bc6e3 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc6f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc6f0); }
 let mut pc: u32 = 0x60bc6f0;
 loop { match pc {
 0x60bc6f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60bc852; continue; }
  pc = 0x60bc700; continue;
 }
 0x60bc700 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if cpu.zf { pc = 0x60bc70f; continue; }
  pc = 0x60bc706; continue;
 }
 0x60bc706 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bc70c); f_060a5950(cpu); if cpu.returned_to != 0x60bc70c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60bc70f; continue;
 }
 0x60bc70f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x60bc852; continue; }
  pc = 0x60bc71b; continue;
 }
 0x60bc71b => {
  if (cpu.cf || cpu.zf) { pc = 0x60bc72b; continue; }
  pc = 0x60bc71d; continue;
 }
 0x60bc71d => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x60bc7b9; continue; }
  pc = 0x60bc726; continue;
 }
 0x60bc726 => {
  pc = 0x60bc852; continue;
 }
 0x60bc72b => {
  { let v: u32 = cpu.r32(0x62d218cu32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc737); f_060a9330(cpu); if cpu.returned_to != 0x60bc737 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bc854; continue; }
  pc = 0x60bc745; continue;
 }
 0x60bc745 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc750); f_060a9160(cpu); if cpu.returned_to != 0x60bc750 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x60bc76b; continue; }
  pc = 0x60bc758; continue;
 }
 0x60bc758 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60bc761); f_060a9280(cpu); if cpu.returned_to != 0x60bc761 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc76b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60bc777); f_060a7370(cpu); if cpu.returned_to != 0x60bc777 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x60bcce0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc788); f_060a7770(cpu); if cpu.returned_to != 0x60bc788 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bc799); f_060a8c40(cpu); if cpu.returned_to != 0x60bc799 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60bc7ac); f_060a90b0(cpu); if cpu.returned_to != 0x60bc7ac { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc7b9 => {
  { let v: u32 = cpu.r32(0x62d2188u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bc7c5); f_060ab930(cpu); if cpu.returned_to != 0x60bc7c5 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bc854; continue; }
  pc = 0x60bc7d3; continue;
 }
 0x60bc7d3 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc7d9); f_060ab6b0(cpu); if cpu.returned_to != 0x60bc7d9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v = 0xffffffffu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc7ea); f_060ab590(cpu); if cpu.returned_to != 0x60bc7ea { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bc804; continue; }
  pc = 0x60bc7f1; continue;
 }
 0x60bc7f1 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bc7fa); f_060ab760(cpu); if cpu.returned_to != 0x60bc7fa { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc804 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc810); f_060a9900(cpu); if cpu.returned_to != 0x60bc810 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v = 0x60bcca0u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bc821); f_060a9ae0(cpu); if cpu.returned_to != 0x60bc821 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60bc832); f_060aae30(cpu); if cpu.returned_to != 0x60bc832 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x3u32; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x60bc845); f_060ab340(cpu); if cpu.returned_to != 0x60bc845 { return; }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc852 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60bc854; continue;
 }
 0x60bc854 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc860(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc860); }
 let mut pc: u32 = 0x60bc860;
 loop { match pc {
 0x60bc860 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x60bc8fd; continue; }
  pc = 0x60bc86e; continue;
 }
 0x60bc86e => {
  cpu.push(0x60bc873); f_060ad100(cpu); if cpu.returned_to != 0x60bc873 { return; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x60bc8a3; continue; }
  pc = 0x60bc87a; continue;
 }
 0x60bc87a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.cf { pc = 0x60bc89c; continue; }
  pc = 0x60bc882; continue;
 }
 0x60bc882 => {
  if (cpu.cf || cpu.zf) { pc = 0x60bc88b; continue; }
  pc = 0x60bc884; continue;
 }
 0x60bc884 => {
  { cpu.sub32(cpu.eax, 0x2u32); }
  if cpu.zf { pc = 0x60bc893; continue; }
  pc = 0x60bc889; continue;
 }
 0x60bc889 => {
  pc = 0x60bc89c; continue;
 }
 0x60bc88b => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc891); f_060a8ea0(cpu); if cpu.returned_to != 0x60bc891 { return; }
  pc = 0x60bc899; continue;
 }
 0x60bc893 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bc899); f_060ab130(cpu); if cpu.returned_to != 0x60bc899 { return; }
  pc = 0x60bc899; continue;
 }
 0x60bc899 => {
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x60bc89c; continue;
 }
 0x60bc89c => {
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bc8a3; continue;
 }
 0x60bc8a3 => {
  cpu.push(0x60bc8a8); f_060ad0f0(cpu); if cpu.returned_to != 0x60bc8a8 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60bc8bd; continue; }
  pc = 0x60bc8ae; continue;
 }
 0x60bc8ae => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x60bc8b4); f_060b32d0(cpu); if cpu.returned_to != 0x60bc8b4 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bc8bd; continue;
 }
 0x60bc8bd => {
  { let v: u32 = cpu.r32(0x617c768u32); cpu.eax = v; }
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if !cpu.zf { pc = 0x60bc8d0; continue; }
  pc = 0x60bc8c6; continue;
 }
 0x60bc8c6 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617c768u32, v); }
  pc = 0x60bc8f4; continue;
 }
 0x60bc8d0 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bc8f4; continue; }
  pc = 0x60bc8d4; continue;
 }
 0x60bc8d4 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if !cpu.zf { pc = 0x60bc8ed; continue; }
  pc = 0x60bc8db; continue;
 }
 0x60bc8db => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bc8e7); f_060b32d0(cpu); if cpu.returned_to != 0x60bc8e7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc8ed => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x60bc8d4; continue; }
  pc = 0x60bc8f4; continue;
 }
 0x60bc8f4 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bc8fa); f_060b32d0(cpu); if cpu.returned_to != 0x60bc8fa { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x60bc8fd; continue;
 }
 0x60bc8fd => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bc900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bc900); }
 let mut pc: u32 = 0x60bc900;
 loop { match pc {
 0x60bc900 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v = 0x14u32; cpu.push(v); }
  cpu.push(0x60bc90e); f_060b3300(cpu); if cpu.returned_to != 0x60bc90e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bc937; continue; }
  pc = 0x60bc91a; continue;
 }
 0x60bc91a => {
  { let v: u32 = 0x616b1a0u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60bc926; continue;
 }
 0x60bc926 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60bc926; continue; }
  pc = 0x60bc930; continue;
 }
 0x60bc930 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60bcb80; continue;
 }
 0x60bc937 => {
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
  cpu.push(0x60bc965); f_060b2f10(cpu); if cpu.returned_to != 0x60bc965 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bc998; continue; }
  pc = 0x60bc96f; continue;
 }
 0x60bc96f => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x60bc975); f_060a5b10(cpu); if cpu.returned_to != 0x60bc975 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x616b1b0u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60bc984; continue;
 }
 0x60bc984 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60bc984; continue; }
  pc = 0x60bc98e; continue;
 }
 0x60bc98e => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bc998 => {
  { let v: u32 = 0x8u32; cpu.ecx = v; }
  { let v: u32 = 0x616b1c0u32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x60bc9a4; continue;
 }
 0x60bc9a4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bc9b2; continue; }
  pc = 0x60bc9ac; continue;
 }
 0x60bc9ac => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bc9b2; continue; }
  pc = 0x60bc9b0; continue;
 }
 0x60bc9b0 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bc9b2; continue;
 }
 0x60bc9b2 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bc9bf; continue; }
  pc = 0x60bc9b7; continue;
 }
 0x60bc9b7 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bc9bf; continue; }
  pc = 0x60bc9bc; continue;
 }
 0x60bc9bc => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60bc9bf; continue;
 }
 0x60bc9bf => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60bc9ce; continue; }
  pc = 0x60bc9c3; continue;
 }
 0x60bc9c3 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60bc9ce; continue; }
  pc = 0x60bc9c7; continue;
 }
 0x60bc9c7 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60bc9ce; continue; }
  pc = 0x60bc9ca; continue;
 }
 0x60bc9ca => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60bc9a4; continue;
 }
 0x60bc9ce => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60bc9d5; continue; }
  pc = 0x60bc9d3; continue;
 }
 0x60bc9d3 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bc9d5; continue;
 }
 0x60bc9d5 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60bc9e5; continue; }
  pc = 0x60bc9d9; continue;
 }
 0x60bc9d9 => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60bcaf0; continue;
 }
 0x60bc9e5 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x616b1ccu32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x60bc9f5; continue;
 }
 0x60bc9f5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bca03; continue; }
  pc = 0x60bc9fd; continue;
 }
 0x60bc9fd => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bca03; continue; }
  pc = 0x60bca01; continue;
 }
 0x60bca01 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bca03; continue;
 }
 0x60bca03 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bca10; continue; }
  pc = 0x60bca08; continue;
 }
 0x60bca08 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bca10; continue; }
  pc = 0x60bca0d; continue;
 }
 0x60bca0d => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60bca10; continue;
 }
 0x60bca10 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60bca1f; continue; }
  pc = 0x60bca14; continue;
 }
 0x60bca14 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60bca1f; continue; }
  pc = 0x60bca18; continue;
 }
 0x60bca18 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60bca1f; continue; }
  pc = 0x60bca1b; continue;
 }
 0x60bca1b => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60bc9f5; continue;
 }
 0x60bca1f => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60bca26; continue; }
  pc = 0x60bca24; continue;
 }
 0x60bca24 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bca26; continue;
 }
 0x60bca26 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60bca36; continue; }
  pc = 0x60bca2a; continue;
 }
 0x60bca2a => {
  { let v: u32 = 0x2u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60bcaf0; continue;
 }
 0x60bca36 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = 0x616b1d4u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x60bca43; continue;
 }
 0x60bca43 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bca51; continue; }
  pc = 0x60bca4b; continue;
 }
 0x60bca4b => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bca51; continue; }
  pc = 0x60bca4f; continue;
 }
 0x60bca4f => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bca51; continue;
 }
 0x60bca51 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bca5e; continue; }
  pc = 0x60bca56; continue;
 }
 0x60bca56 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bca5e; continue; }
  pc = 0x60bca5b; continue;
 }
 0x60bca5b => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60bca5e; continue;
 }
 0x60bca5e => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60bca6d; continue; }
  pc = 0x60bca62; continue;
 }
 0x60bca62 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60bca6d; continue; }
  pc = 0x60bca66; continue;
 }
 0x60bca66 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60bca6d; continue; }
  pc = 0x60bca69; continue;
 }
 0x60bca69 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60bca43; continue;
 }
 0x60bca6d => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60bca74; continue; }
  pc = 0x60bca72; continue;
 }
 0x60bca72 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bca74; continue;
 }
 0x60bca74 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x60bcabd; continue; }
  pc = 0x60bca78; continue;
 }
 0x60bca78 => {
  { let v: u32 = 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = 0x616b1dcu32; cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  pc = 0x60bca88; continue;
 }
 0x60bca88 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.sub8((cpu.eax as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bca96; continue; }
  pc = 0x60bca90; continue;
 }
 0x60bca90 => {
  { cpu.sub8((cpu.eax as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bca96; continue; }
  pc = 0x60bca94; continue;
 }
 0x60bca94 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bca96; continue;
 }
 0x60bca96 => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x61u8); }
  if cpu.cf { pc = 0x60bcaa3; continue; }
  pc = 0x60bca9b; continue;
 }
 0x60bca9b => {
  { cpu.sub8(((cpu.eax >> 8) as u8), 0x7au8); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bcaa3; continue; }
  pc = 0x60bcaa0; continue;
 }
 0x60bcaa0 => {
  { let v: u8 = ((cpu.eax >> 8) as u8).wrapping_sub(0x20u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x60bcaa3; continue;
 }
 0x60bcaa3 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x60bcab2; continue; }
  pc = 0x60bcaa7; continue;
 }
 0x60bcaa7 => {
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x60bcab2; continue; }
  pc = 0x60bcaab; continue;
 }
 0x60bcaab => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x60bcab2; continue; }
  pc = 0x60bcaae; continue;
 }
 0x60bcaae => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.poll(); pc = 0x60bca88; continue;
 }
 0x60bcab2 => {
  { let v: u8 = (!cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  if (cpu.cf || cpu.zf) { pc = 0x60bcab9; continue; }
  pc = 0x60bcab7; continue;
 }
 0x60bcab7 => {
  { let v: u8 = cpu.neg8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bcab9; continue;
 }
 0x60bcab9 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x60bcae9; continue; }
  pc = 0x60bcabd; continue;
 }
 0x60bcabd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x60bcac6); f_060a5b10(cpu); if cpu.returned_to != 0x60bcac6 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x616b1e4u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60bcad5; continue;
 }
 0x60bcad5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60bcad5; continue; }
  pc = 0x60bcadf; continue;
 }
 0x60bcadf => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bcae9 => {
  { let v: u32 = 0x1u32; cpu.w32(0x4u32.wrapping_add(cpu.ebp), v); }
  pc = 0x60bcaf0; continue;
 }
 0x60bcaf0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x1u32); }
  if !cpu.zf { pc = 0x60bcb2b; continue; }
  pc = 0x60bcaf9; continue;
 }
 0x60bcaf9 => {
  { cpu.sub32(cpu.r32(0x62d218cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60bcb2b; continue; }
  pc = 0x60bcb02; continue;
 }
 0x60bcb02 => {
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x60bcb08); f_060a5b10(cpu); if cpu.returned_to != 0x60bcb08 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x616b1f8u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60bcb17; continue;
 }
 0x60bcb17 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60bcb17; continue; }
  pc = 0x60bcb21; continue;
 }
 0x60bcb21 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bcb2b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x2u32); }
  if !cpu.zf { pc = 0x60bcb69; continue; }
  pc = 0x60bcb34; continue;
 }
 0x60bcb34 => {
  { cpu.sub32(cpu.r32(0x62d2188u32), 0x0u32); }
  if !cpu.zf { pc = 0x60bcb69; continue; }
  pc = 0x60bcb3d; continue;
 }
 0x60bcb3d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x60bcb46); f_060a5b10(cpu); if cpu.returned_to != 0x60bcb46 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x616b214u32; cpu.esi = v; }
  { let v: u32 = 0x62d1da0u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x60bcb55; continue;
 }
 0x60bcb55 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60bcb55; continue; }
  pc = 0x60bcb5f; continue;
 }
 0x60bcb5f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x60bcb69 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = 0x2u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x617c768u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x60bcb80; continue;
 }
 0x60bcb80 => {
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
pub fn f_060bcca0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcca0); }
 let mut pc: u32 = 0x60bcca0;
 loop { match pc {
 0x60bcca0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bccac); f_060a97f0(cpu); if cpu.returned_to != 0x60bccac { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60bccbe); f_060a97f0(cpu); if cpu.returned_to != 0x60bccbe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60bccce); f_060ab760(cpu); if cpu.returned_to != 0x60bccce { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bcce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcce0); }
 let mut pc: u32 = 0x60bcce0;
 loop { match pc {
 0x60bcce0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x60bccec); f_060a7260(cpu); if cpu.returned_to != 0x60bccec { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60bccfe); f_060a7260(cpu); if cpu.returned_to != 0x60bccfe { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x60bcd0e); f_060a9280(cpu); if cpu.returned_to != 0x60bcd0e { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bcd20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcd20); }
 let mut pc: u32 = 0x60bcd20;
 loop { match pc {
 0x60bcd20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x617c76cu32), 0x0u32); }
  if !cpu.zf { pc = 0x60bcd76; continue; }
  pc = 0x60bcd2c; continue;
 }
 0x60bcd2c => {
  { let v = 0x60bc5c0u32; cpu.push(v); }
  { let v = 0x60bcd20u32; cpu.push(v); }
  cpu.push(0x60bcd3b); f_060b3160(cpu); if cpu.returned_to != 0x60bcd3b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d218cu32; cpu.push(v); }
  cpu.push(0x60bcd4a); f_060b30a0(cpu); if cpu.returned_to != 0x60bcd4a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x62d2188u32; cpu.push(v); }
  cpu.push(0x60bcd59); f_060b30a0(cpu); if cpu.returned_to != 0x60bcd59 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x617c768u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x60bcd6d); f_060b30a0(cpu); if cpu.returned_to != 0x60bcd6d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x617c76cu32, v); }
  pc = 0x60bcd76; continue;
 }
 0x60bcd76 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bcd7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcd7a); }
 let mut pc: u32 = 0x60bcd7a;
 loop { match pc {
 0x60bcd7a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617b834u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x60bcd91); f_060bce3b(cpu); if cpu.returned_to != 0x60bcd91 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x617b834u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  cpu.push(0x60bcdb3); f_060bd034(cpu); if cpu.returned_to != 0x60bcdb3 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bce14; continue; }
  pc = 0x60bcdb9; continue;
 }
 0x60bcdb9 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x617b834u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617aff8u32, v); }
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
  { let v: u32 = cpu.r32(0x617aff8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x617b838u32); cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x60bcdfb); f_060bce3b(cpu); if cpu.returned_to != 0x60bcdfb { return; }
  { let v: u32 = cpu.r32(0x617aff8u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.esi); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617aff4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x60bce1e; continue;
 }
 0x60bce14 => {
  { let v: u32 = cpu.eax; cpu.w32(0x617aff8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x617aff4u32, v); }
  pc = 0x60bce1e; continue;
 }
 0x60bce1e => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d2238u32, v); }
  { let v: u32 = cpu.r32(0x617aff8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x62d223cu32, v); }
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
pub fn f_060bce3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bce3b); }
 let mut pc: u32 = 0x60bce3b;
 loop { match pc {
 0x60bce3b => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x62d2240u32); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x60bce52; continue;
 }
 0x60bce52 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x20u32); }
  if cpu.zf { pc = 0x60bce60; continue; }
  pc = 0x60bce5b; continue;
 }
 0x60bce5b => {
  { cpu.sub32(cpu.ebx, 0x9u32); }
  if !cpu.zf { pc = 0x60bce63; continue; }
  pc = 0x60bce60; continue;
 }
 0x60bce60 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60bce52; continue;
 }
 0x60bce63 => {
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x60bcf35; continue; }
  pc = 0x60bce6b; continue;
 }
 0x60bce6b => {
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x22u32); }
  if !cpu.zf { pc = 0x60bce75; continue; }
  pc = 0x60bce72; continue;
 }
 0x60bce72 => {
  { let v: u8 = 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60bce75; continue;
 }
 0x60bce75 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x60bce7a; continue;
 }
 0x60bce7a => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x60bcea3; continue; }
  pc = 0x60bce83; continue;
 }
 0x60bce83 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x60bcf02; continue; }
  pc = 0x60bce8c; continue;
 }
 0x60bce8c => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60bcea3; continue; }
  pc = 0x60bce90; continue;
 }
 0x60bce90 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { pc = 0x60bce99; continue; }
  pc = 0x60bce94; continue;
 }
 0x60bce94 => {
  { let v: u8 = 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60bce96; continue;
 }
 0x60bce96 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x60bce7a; continue;
 }
 0x60bce99 => {
  { cpu.sub8((cpu.edx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x60bcea3; continue; }
  pc = 0x60bce9e; continue;
 }
 0x60bce9e => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) ^ (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.poll(); pc = 0x60bce7a; continue;
 }
 0x60bcea3 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x20u32); }
  if cpu.zf { pc = 0x60bceb1; continue; }
  pc = 0x60bceac; continue;
 }
 0x60bceac => {
  { cpu.sub32(cpu.ecx, 0x9u32); }
  if !cpu.zf { pc = 0x60bceb5; continue; }
  pc = 0x60bceb1; continue;
 }
 0x60bceb1 => {
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x60bcf02; continue; }
  pc = 0x60bceb5; continue;
 }
 0x60bceb5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.edx >> 8) as u8), ((cpu.edx >> 8) as u8)); }
  if cpu.zf { pc = 0x60bcf02; continue; }
  pc = 0x60bcebb; continue;
 }
 0x60bcebb => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  if !cpu.zf { pc = 0x60bcef3; continue; }
  pc = 0x60bcec4; continue;
 }
 0x60bcec4 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x60bced4; continue; }
  pc = 0x60bcec8; continue;
 }
 0x60bcec8 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  pc = 0x60bced0; continue;
 }
 0x60bced0 => {
  if !cpu.zf { pc = 0x60bcef3; continue; }
  pc = 0x60bced2; continue;
 }
 0x60bced2 => {
  pc = 0x60bcef2; continue;
 }
 0x60bced4 => {
  { cpu.sub8((cpu.edx as u8), 0x1u8); }
  if !cpu.zf { pc = 0x60bcee8; continue; }
  pc = 0x60bced9; continue;
 }
 0x60bced9 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if cpu.zf { pc = 0x60bcef2; continue; }
  pc = 0x60bcee3; continue;
 }
 0x60bcee3 => {
  { cpu.sub32(cpu.ecx, 0x5cu32); }
  cpu.poll(); pc = 0x60bced0; continue;
 }
 0x60bcee8 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ecx, 0x22u32); }
  if !cpu.zf { pc = 0x60bcef3; continue; }
  pc = 0x60bcef2; continue;
 }
 0x60bcef2 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x60bcef3; continue;
 }
 0x60bcef3 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { cpu.poll(); pc = 0x60bce96; continue; }
  pc = 0x60bcef7; continue;
 }
 0x60bcef7 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x60bce7a; continue;
 }
 0x60bcf02 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x60bcf28; continue; }
  pc = 0x60bcf06; continue;
 }
 0x60bcf06 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if !cpu.zf { pc = 0x60bcf1f; continue; }
  pc = 0x60bcf1b; continue;
 }
 0x60bcf1b => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x60bcf35; continue;
 }
 0x60bcf1f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x60bce52; continue;
 }
 0x60bcf28 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x60bce60; continue; }
  pc = 0x60bcf35; continue;
 }
 0x60bcf35 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x62d2240u32, v); }
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
pub fn f_060bcf4a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcf4a); }
 let mut pc: u32 = 0x60bcf4a;
 loop { match pc {
 0x60bcf4a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x617b848u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3u32); cpu.edx = v; }
  { let v: u8 = (cpu.edx as u8) & 0xfcu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x60bcf5f); f_060c15b1(cpu); if cpu.returned_to != 0x60bcf5f { return; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.cf { pc = 0x60bcf6b; continue; }
  pc = 0x60bcf63; continue;
 }
 0x60bcf63 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_sub(cpu.eax); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  pc = 0x60bcf6d; continue;
 }
 0x60bcf6b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60bcf6d; continue;
 }
 0x60bcf6d => {
  { let v: u32 = cpu.r32(0x617b848u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x617b84cu32, v); }
  cpu.push(0x60bcf7f); f_060c3de1(cpu); if cpu.returned_to != 0x60bcf7f { return; }
  { let v: u32 = cpu.r32(0x62d223cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x62d2238u32); cpu.eax = v; }
  cpu.push(0x60bcf8f); f_0607de98(cpu); if cpu.returned_to != 0x60bcf8f { return; }
  cpu.push(0x60bcf94); f_060c3e4b(cpu); if cpu.returned_to != 0x60bcf94 { return; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bcf9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcf9a); }
 let mut pc: u32 = 0x60bcf9a;
 loop { match pc {
 0x60bcf9a => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x60bcfa1; continue;
 }
 0x60bcfa1 => {
  { let v: u32 = 0x617cdc6u32; cpu.esi = v; }
  { let v: u32 = 0x617ce02u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bcfb1; continue;
 }
 0x60bcfb1 => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x60bcfc9; continue; }
  pc = 0x60bcfb5; continue;
 }
 0x60bcfb5 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x60bcfc4; continue; }
  pc = 0x60bcfba; continue;
 }
 0x60bcfba => {
  { cpu.sub8(cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bcfc4; continue; }
  pc = 0x60bcfbf; continue;
 }
 0x60bcfbf => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bcfc4; continue;
 }
 0x60bcfc4 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x60bcfb1; continue;
 }
 0x60bcfc9 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x60bcfdf; continue; }
  pc = 0x60bcfcd; continue;
 }
 0x60bcfcd => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x60bcfda; continue; }
  pc = 0x60bcfd4; continue;
 }
 0x60bcfd4 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x60bcfd7); cpu.push(0x60bcfd9); cpu.call(t); if cpu.returned_to != 0x60bcfd9 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x60bcfda; continue;
 }
 0x60bcfda => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60bcfa1; continue;
 }
 0x60bcfdf => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x60bcfe4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bcfe5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bcfe5); }
 let mut pc: u32 = 0x60bcfe5;
 loop { match pc {
 0x60bcfe5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x60bcfed; continue;
 }
 0x60bcfed => {
  { let v: u32 = 0x617ce02u32; cpu.esi = v; }
  { let v: u32 = 0x617ce14u32; cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bcffd; continue;
 }
 0x60bcffd => {
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.cf { pc = 0x60bd015; continue; }
  pc = 0x60bd001; continue;
 }
 0x60bd001 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x2u8); }
  if cpu.zf { pc = 0x60bd010; continue; }
  pc = 0x60bd006; continue;
 }
 0x60bd006 => {
  { cpu.sub8((cpu.eax as u8), cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]))); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bd010; continue; }
  pc = 0x60bd00b; continue;
 }
 0x60bd00b => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x60bd010; continue;
 }
 0x60bd010 => {
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  cpu.poll(); pc = 0x60bcffd; continue;
 }
 0x60bd015 => {
  { cpu.sub32(cpu.ebx, cpu.edi); }
  if cpu.zf { pc = 0x60bd02f; continue; }
  pc = 0x60bd019; continue;
 }
 0x60bd019 => {
  { cpu.sub8((cpu.eax as u8), ((cpu.edx >> 8) as u8)); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bd02a; continue; }
  pc = 0x60bd01d; continue;
 }
 0x60bd01d => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x60bd02a; continue; }
  pc = 0x60bd024; continue;
 }
 0x60bd024 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v = cpu.edx; cpu.push(v); }
  let t = cpu.eax; cpu.flat_ds(0x60bd027); cpu.push(0x60bd029); cpu.call(t); if cpu.returned_to != 0x60bd029 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x60bd02a; continue;
 }
 0x60bd02a => {
  { let v: u8 = 0x2u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  cpu.poll(); pc = 0x60bcfed; continue;
 }
 0x60bd02f => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.flat_ds(0x60bd033); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_060bd034(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd034); }
 let mut pc: u32 = 0x60bd034;
 loop { match pc {
 0x60bd034 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[4] as u32); }
  { cpu.push(cpu.segments[5] as u32); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bd04d; continue; }
  pc = 0x60bd048; continue;
 }
 0x60bd048 => {
  { cpu.sub32(cpu.eax, 0xffffffd4u32); }
  if (cpu.cf || cpu.zf) { pc = 0x60bd054; continue; }
  pc = 0x60bd04d; continue;
 }
 0x60bd04d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x60bd108; continue;
 }
 0x60bd054 => {
  { let v: u32 = 0x617c770u32; cpu.ebp = v; }
  { let v: u32 = 0x3u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.edi as u16) & 0xfffcu16; cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x60bd067; continue;
 }
 0x60bd067 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { cpu.sub32(cpu.edi, 0xcu32); }
  if !cpu.cf { pc = 0x60bd073; continue; }
  pc = 0x60bd06e; continue;
 }
 0x60bd06e => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  pc = 0x60bd073; continue;
 }
 0x60bd073 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x617c778u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x60bd093; continue; }
  pc = 0x60bd07b; continue;
 }
 0x60bd07b => {
  { let v: u32 = cpu.r32(0x617c774u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x60bd0cb; continue; }
  pc = 0x60bd085; continue;
 }
 0x60bd085 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x617c778u32, v); }
  { let v: u32 = cpu.r32(0x617c770u32); cpu.ecx = v; }
  pc = 0x60bd0cb; continue;
 }
 0x60bd093 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x60bd085; continue;
 }
 0x60bd097 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x617c774u32, v); }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bd0b0); f_060c3e7a(cpu); if cpu.returned_to != 0x60bd0b0 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bd0fe; continue; }
  pc = 0x60bd0b6; continue;
 }
 0x60bd0b6 => {
  { let v: u32 = cpu.r32(0x617c778u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.cf || cpu.zf) { pc = 0x60bd0c8; continue; }
  pc = 0x60bd0c3; continue;
 }
 0x60bd0c3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x617c778u32, v); }
  pc = 0x60bd0c8; continue;
 }
 0x60bd0c8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x60bd0cb; continue;
 }
 0x60bd0cb => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x60bd097; continue; }
  pc = 0x60bd0cf; continue;
 }
 0x60bd0cf => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esp)), 0x0u8); }
  if !cpu.zf { pc = 0x60bd0e0; continue; }
  pc = 0x60bd0d5; continue;
 }
 0x60bd0d5 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bd0dc); f_060c441e(cpu); if cpu.returned_to != 0x60bd0dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bd0f5; continue; }
  pc = 0x60bd0e0; continue;
 }
 0x60bd0e0 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bd0e7); f_060c45e7(cpu); if cpu.returned_to != 0x60bd0e7 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bd0fe; continue; }
  pc = 0x60bd0eb; continue;
 }
 0x60bd0eb => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60bd067; continue;
 }
 0x60bd0f5 => {
  { let v: u8 = 0x1u8; cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x60bd067; continue;
 }
 0x60bd0fe => {
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x62d2244u32, v); }
  pc = 0x60bd108; continue;
 }
 0x60bd108 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v = cpu.pop(); cpu.set_segment(5, v as u16); }
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
pub fn f_060bd117(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd117); }
 let mut pc: u32 = 0x60bd117;
 loop { match pc {
 0x60bd117 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bd193; continue; }
  pc = 0x60bd126; continue;
 }
 0x60bd126 => {
  { let v: u32 = cpu.r32(0x62d2190u32); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x60bd139; continue; }
  pc = 0x60bd130; continue;
 }
 0x60bd130 => {
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bd139; continue; }
  pc = 0x60bd134; continue;
 }
 0x60bd134 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.ecx))); }
  if cpu.cf { pc = 0x60bd153; continue; }
  pc = 0x60bd139; continue;
 }
 0x60bd139 => {
  { let v: u32 = cpu.r32(0x617c770u32); cpu.ecx = v; }
  pc = 0x60bd14c; continue;
 }
 0x60bd141 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (!cpu.cf && !cpu.zf) { pc = 0x60bd149; continue; }
  pc = 0x60bd145; continue;
 }
 0x60bd145 => {
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if cpu.cf { pc = 0x60bd153; continue; }
  pc = 0x60bd149; continue;
 }
 0x60bd149 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x60bd14c; continue;
 }
 0x60bd14c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x60bd141; continue; }
  pc = 0x60bd153; continue;
 }
 0x60bd153 => {
  { let v: u32 = 0x617c770u32; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x60bd169); f_060c3f22(cpu); if cpu.returned_to != 0x60bd169 { return; }
  { let v: u32 = cpu.r32(0x617c774u32); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x62d2190u32, v); }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if !cpu.cf { pc = 0x60bd18b; continue; }
  pc = 0x60bd179; continue;
 }
 0x60bd179 => {
  { let v: u32 = cpu.r32(0x617c778u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x60bd18b; continue; }
  pc = 0x60bd185; continue;
 }
 0x60bd185 => {
  { let v: u32 = cpu.edx; cpu.w32(0x617c778u32, v); }
  pc = 0x60bd18b; continue;
 }
 0x60bd18b => {
  { let v: u8 = cpu.xor8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x62d2244u32, v); }
  pc = 0x60bd193; continue;
 }
 0x60bd193 => {
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
pub fn f_060bd199(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x60bd199); }
 let mut pc: u32 = 0x60bd199;
 loop { match pc {
 0x60bd199 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x617ca9cu32); cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x60bd1f1; continue; }
  pc = 0x60bd1ab; continue;
 }
 0x60bd1ab => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x60bd1f1; continue; }
  pc = 0x60bd1af; continue;
 }
 0x60bd1af => {
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
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  pc = 0x60bd1eb; continue;
 }
 0x60bd1c5 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x60bd1d0); f_060a5476(cpu); if cpu.returned_to != 0x60bd1d0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x60bd1e8; continue; }
  pc = 0x60bd1d4; continue;
 }
 0x60bd1d4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3du32); }
  if !cpu.zf { pc = 0x60bd1e8; continue; }
  pc = 0x60bd1e1; continue;
 }
 0x60bd1e1 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x60bd1f3; continue;
 }
 0x60bd1e8 => {
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  pc = 0x60bd1eb; continue;
 }
 0x60bd1eb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x60bd1c5; continue; }
  pc = 0x60bd1f1; continue;
 }
 0x60bd1f1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x60bd1f3; continue;
 }
 0x60bd1f3 => {
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
