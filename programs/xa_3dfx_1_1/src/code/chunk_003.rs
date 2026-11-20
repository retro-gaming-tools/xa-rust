#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_020a5ef0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a5ef0); }
 let mut pc: u32 = 0x20a5ef0;
 loop { match pc {
 0x20a5ef0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a5f37; continue; }
  pc = 0x20a5f10; continue;
 }
 0x20a5f10 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a5f1e; continue; }
  pc = 0x20a5f15; continue;
 }
 0x20a5f15 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a5f37; continue; }
  pc = 0x20a5f1e; continue;
 }
 0x20a5f1e => {
  cpu.push(0x20a5f23); f_020b4041(cpu); if cpu.returned_to != 0x20a5f23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a5f37; continue; }
  pc = 0x20a5f27; continue;
 }
 0x20a5f27 => {
  cpu.push(0x20a5f2c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a5f2c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a5f37; continue; }
  pc = 0x20a5f30; continue;
 }
 0x20a5f30 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a5f39; continue;
 }
 0x20a5f37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a5f39; continue;
 }
 0x20a5f39 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a5f6b; continue; }
  pc = 0x20a5f3d; continue;
 }
 0x20a5f3d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a9fcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a5f63); cpu.call(t); if cpu.returned_to != 0x20a5f63 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a5f6b); f_020adba0(cpu); if cpu.returned_to != 0x20a5f6b { return; }
  pc = 0x20a5f6b; continue;
 }
 0x20a5f6b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a5f78); f_020bcca0(cpu); if cpu.returned_to != 0x20a5f78 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a5fa0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a5fa0); }
 let mut pc: u32 = 0x20a5fa0;
 loop { match pc {
 0x20a5fa0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a5fe8; continue; }
  pc = 0x20a5fc1; continue;
 }
 0x20a5fc1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a5fcf; continue; }
  pc = 0x20a5fc6; continue;
 }
 0x20a5fc6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a5fe8; continue; }
  pc = 0x20a5fcf; continue;
 }
 0x20a5fcf => {
  cpu.push(0x20a5fd4); f_020b4041(cpu); if cpu.returned_to != 0x20a5fd4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a5fe8; continue; }
  pc = 0x20a5fd8; continue;
 }
 0x20a5fd8 => {
  cpu.push(0x20a5fdd); f_020ad7d0(cpu); if cpu.returned_to != 0x20a5fdd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a5fe8; continue; }
  pc = 0x20a5fe1; continue;
 }
 0x20a5fe1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a5fea; continue;
 }
 0x20a5fe8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a5fea; continue;
 }
 0x20a5fea => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a601c; continue; }
  pc = 0x20a5fee; continue;
 }
 0x20a5fee => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a9e4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6014); cpu.call(t); if cpu.returned_to != 0x20a6014 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a601c); f_020adba0(cpu); if cpu.returned_to != 0x20a601c { return; }
  pc = 0x20a601c; continue;
 }
 0x20a601c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6029); f_020bcd40(cpu); if cpu.returned_to != 0x20a6029 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a6077; continue; }
  pc = 0x20a6038; continue;
 }
 0x20a6038 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a604a; continue; }
  pc = 0x20a6041; continue;
 }
 0x20a6041 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6077; continue; }
  pc = 0x20a604a; continue;
 }
 0x20a604a => {
  cpu.push(0x20a604f); f_020b4041(cpu); if cpu.returned_to != 0x20a604f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6077; continue; }
  pc = 0x20a6053; continue;
 }
 0x20a6053 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a606d); f_020b42d3(cpu); if cpu.returned_to != 0x20a606d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6077); f_020adb00(cpu); if cpu.returned_to != 0x20a6077 { return; }
  pc = 0x20a6077; continue;
 }
 0x20a6077 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6220(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6220); }
 let mut pc: u32 = 0x20a6220;
 loop { match pc {
 0x20a6220 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6268; continue; }
  pc = 0x20a6241; continue;
 }
 0x20a6241 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a624f; continue; }
  pc = 0x20a6246; continue;
 }
 0x20a6246 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6268; continue; }
  pc = 0x20a624f; continue;
 }
 0x20a624f => {
  cpu.push(0x20a6254); f_020b4041(cpu); if cpu.returned_to != 0x20a6254 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6268; continue; }
  pc = 0x20a6258; continue;
 }
 0x20a6258 => {
  cpu.push(0x20a625d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a625d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6268; continue; }
  pc = 0x20a6261; continue;
 }
 0x20a6261 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a626a; continue;
 }
 0x20a6268 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a626a; continue;
 }
 0x20a626a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a629c; continue; }
  pc = 0x20a626e; continue;
 }
 0x20a626e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a994u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6294); cpu.call(t); if cpu.returned_to != 0x20a6294 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a629c); f_020adba0(cpu); if cpu.returned_to != 0x20a629c { return; }
  pc = 0x20a629c; continue;
 }
 0x20a629c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a62a9); f_020bbd30(cpu); if cpu.returned_to != 0x20a62a9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a62f7; continue; }
  pc = 0x20a62b8; continue;
 }
 0x20a62b8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a62ca; continue; }
  pc = 0x20a62c1; continue;
 }
 0x20a62c1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a62f7; continue; }
  pc = 0x20a62ca; continue;
 }
 0x20a62ca => {
  cpu.push(0x20a62cf); f_020b4041(cpu); if cpu.returned_to != 0x20a62cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a62f7; continue; }
  pc = 0x20a62d3; continue;
 }
 0x20a62d3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a62ed); f_020b42d3(cpu); if cpu.returned_to != 0x20a62ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a62f7); f_020adb50(cpu); if cpu.returned_to != 0x20a62f7 { return; }
  pc = 0x20a62f7; continue;
 }
 0x20a62f7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6320); }
 let mut pc: u32 = 0x20a6320;
 loop { match pc {
 0x20a6320 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6368; continue; }
  pc = 0x20a6341; continue;
 }
 0x20a6341 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a634f; continue; }
  pc = 0x20a6346; continue;
 }
 0x20a6346 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6368; continue; }
  pc = 0x20a634f; continue;
 }
 0x20a634f => {
  cpu.push(0x20a6354); f_020b4041(cpu); if cpu.returned_to != 0x20a6354 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6368; continue; }
  pc = 0x20a6358; continue;
 }
 0x20a6358 => {
  cpu.push(0x20a635d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a635d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6368; continue; }
  pc = 0x20a6361; continue;
 }
 0x20a6361 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a636a; continue;
 }
 0x20a6368 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a636a; continue;
 }
 0x20a636a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a639c; continue; }
  pc = 0x20a636e; continue;
 }
 0x20a636e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a978u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6394); cpu.call(t); if cpu.returned_to != 0x20a6394 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a639c); f_020adba0(cpu); if cpu.returned_to != 0x20a639c { return; }
  pc = 0x20a639c; continue;
 }
 0x20a639c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a63a9); f_020bbd80(cpu); if cpu.returned_to != 0x20a63a9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a63f7; continue; }
  pc = 0x20a63b8; continue;
 }
 0x20a63b8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a63ca; continue; }
  pc = 0x20a63c1; continue;
 }
 0x20a63c1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a63f7; continue; }
  pc = 0x20a63ca; continue;
 }
 0x20a63ca => {
  cpu.push(0x20a63cf); f_020b4041(cpu); if cpu.returned_to != 0x20a63cf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a63f7; continue; }
  pc = 0x20a63d3; continue;
 }
 0x20a63d3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a63ed); f_020b42d3(cpu); if cpu.returned_to != 0x20a63ed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a63f7); f_020adb50(cpu); if cpu.returned_to != 0x20a63f7 { return; }
  pc = 0x20a63f7; continue;
 }
 0x20a63f7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6420(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6420); }
 let mut pc: u32 = 0x20a6420;
 loop { match pc {
 0x20a6420 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6467; continue; }
  pc = 0x20a6440; continue;
 }
 0x20a6440 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a644e; continue; }
  pc = 0x20a6445; continue;
 }
 0x20a6445 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6467; continue; }
  pc = 0x20a644e; continue;
 }
 0x20a644e => {
  cpu.push(0x20a6453); f_020b4041(cpu); if cpu.returned_to != 0x20a6453 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6467; continue; }
  pc = 0x20a6457; continue;
 }
 0x20a6457 => {
  cpu.push(0x20a645c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a645c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6467; continue; }
  pc = 0x20a6460; continue;
 }
 0x20a6460 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6469; continue;
 }
 0x20a6467 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6469; continue;
 }
 0x20a6469 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a64b7; continue; }
  pc = 0x20a646d; continue;
 }
 0x20a646d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a954u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a64af); cpu.call(t); if cpu.returned_to != 0x20a64af { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a64b7); f_020adba0(cpu); if cpu.returned_to != 0x20a64b7 { return; }
  pc = 0x20a64b7; continue;
 }
 0x20a64b7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a64d4); f_020bbde0(cpu); if cpu.returned_to != 0x20a64d4 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20a6522; continue; }
  pc = 0x20a64e3; continue;
 }
 0x20a64e3 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a64f5; continue; }
  pc = 0x20a64ec; continue;
 }
 0x20a64ec => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6522; continue; }
  pc = 0x20a64f5; continue;
 }
 0x20a64f5 => {
  cpu.push(0x20a64fa); f_020b4041(cpu); if cpu.returned_to != 0x20a64fa { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6522; continue; }
  pc = 0x20a64fe; continue;
 }
 0x20a64fe => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6518); f_020b42d3(cpu); if cpu.returned_to != 0x20a6518 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6522); f_020adb50(cpu); if cpu.returned_to != 0x20a6522 { return; }
  pc = 0x20a6522; continue;
 }
 0x20a6522 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a6540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6540); }
 let mut pc: u32 = 0x20a6540;
 loop { match pc {
 0x20a6540 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6588; continue; }
  pc = 0x20a6561; continue;
 }
 0x20a6561 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a656f; continue; }
  pc = 0x20a6566; continue;
 }
 0x20a6566 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6588; continue; }
  pc = 0x20a656f; continue;
 }
 0x20a656f => {
  cpu.push(0x20a6574); f_020b4041(cpu); if cpu.returned_to != 0x20a6574 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6588; continue; }
  pc = 0x20a6578; continue;
 }
 0x20a6578 => {
  cpu.push(0x20a657d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a657d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6588; continue; }
  pc = 0x20a6581; continue;
 }
 0x20a6581 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a658a; continue;
 }
 0x20a6588 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a658a; continue;
 }
 0x20a658a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a65bc; continue; }
  pc = 0x20a658e; continue;
 }
 0x20a658e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a938u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a65b4); cpu.call(t); if cpu.returned_to != 0x20a65b4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a65bc); f_020adba0(cpu); if cpu.returned_to != 0x20a65bc { return; }
  pc = 0x20a65bc; continue;
 }
 0x20a65bc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a65c9); f_020bbe70(cpu); if cpu.returned_to != 0x20a65c9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a6617; continue; }
  pc = 0x20a65d8; continue;
 }
 0x20a65d8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a65ea; continue; }
  pc = 0x20a65e1; continue;
 }
 0x20a65e1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6617; continue; }
  pc = 0x20a65ea; continue;
 }
 0x20a65ea => {
  cpu.push(0x20a65ef); f_020b4041(cpu); if cpu.returned_to != 0x20a65ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6617; continue; }
  pc = 0x20a65f3; continue;
 }
 0x20a65f3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a660d); f_020b42d3(cpu); if cpu.returned_to != 0x20a660d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6617); f_020adb50(cpu); if cpu.returned_to != 0x20a6617 { return; }
  pc = 0x20a6617; continue;
 }
 0x20a6617 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6640); }
 let mut pc: u32 = 0x20a6640;
 loop { match pc {
 0x20a6640 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6688; continue; }
  pc = 0x20a6661; continue;
 }
 0x20a6661 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a666f; continue; }
  pc = 0x20a6666; continue;
 }
 0x20a6666 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6688; continue; }
  pc = 0x20a666f; continue;
 }
 0x20a666f => {
  cpu.push(0x20a6674); f_020b4041(cpu); if cpu.returned_to != 0x20a6674 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6688; continue; }
  pc = 0x20a6678; continue;
 }
 0x20a6678 => {
  cpu.push(0x20a667d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a667d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6688; continue; }
  pc = 0x20a6681; continue;
 }
 0x20a6681 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a668a; continue;
 }
 0x20a6688 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a668a; continue;
 }
 0x20a668a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a66bc; continue; }
  pc = 0x20a668e; continue;
 }
 0x20a668e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a918u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a66b4); cpu.call(t); if cpu.returned_to != 0x20a66b4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a66bc); f_020adba0(cpu); if cpu.returned_to != 0x20a66bc { return; }
  pc = 0x20a66bc; continue;
 }
 0x20a66bc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a66c9); f_020bbea0(cpu); if cpu.returned_to != 0x20a66c9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a6717; continue; }
  pc = 0x20a66d8; continue;
 }
 0x20a66d8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a66ea; continue; }
  pc = 0x20a66e1; continue;
 }
 0x20a66e1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6717; continue; }
  pc = 0x20a66ea; continue;
 }
 0x20a66ea => {
  cpu.push(0x20a66ef); f_020b4041(cpu); if cpu.returned_to != 0x20a66ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6717; continue; }
  pc = 0x20a66f3; continue;
 }
 0x20a66f3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a670d); f_020b42d3(cpu); if cpu.returned_to != 0x20a670d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6717); f_020adb50(cpu); if cpu.returned_to != 0x20a6717 { return; }
  pc = 0x20a6717; continue;
 }
 0x20a6717 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6840); }
 let mut pc: u32 = 0x20a6840;
 loop { match pc {
 0x20a6840 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6887; continue; }
  pc = 0x20a6860; continue;
 }
 0x20a6860 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a686e; continue; }
  pc = 0x20a6865; continue;
 }
 0x20a6865 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6887; continue; }
  pc = 0x20a686e; continue;
 }
 0x20a686e => {
  cpu.push(0x20a6873); f_020b4041(cpu); if cpu.returned_to != 0x20a6873 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6887; continue; }
  pc = 0x20a6877; continue;
 }
 0x20a6877 => {
  cpu.push(0x20a687c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a687c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6887; continue; }
  pc = 0x20a6880; continue;
 }
 0x20a6880 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6889; continue;
 }
 0x20a6887 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6889; continue;
 }
 0x20a6889 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a68e5; continue; }
  pc = 0x20a688d; continue;
 }
 0x20a688d => {
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
  { let v = 0x216a8ccu32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a68dd); cpu.call(t); if cpu.returned_to != 0x20a68dd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a68e5); f_020adba0(cpu); if cpu.returned_to != 0x20a68e5 { return; }
  pc = 0x20a68e5; continue;
 }
 0x20a68e5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20a690a); f_020bbf80(cpu); if cpu.returned_to != 0x20a690a { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a6930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6930); }
 let mut pc: u32 = 0x20a6930;
 loop { match pc {
 0x20a6930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6978; continue; }
  pc = 0x20a6951; continue;
 }
 0x20a6951 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a695f; continue; }
  pc = 0x20a6956; continue;
 }
 0x20a6956 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6978; continue; }
  pc = 0x20a695f; continue;
 }
 0x20a695f => {
  cpu.push(0x20a6964); f_020b4041(cpu); if cpu.returned_to != 0x20a6964 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6978; continue; }
  pc = 0x20a6968; continue;
 }
 0x20a6968 => {
  cpu.push(0x20a696d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a696d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6978; continue; }
  pc = 0x20a6971; continue;
 }
 0x20a6971 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a697a; continue;
 }
 0x20a6978 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a697a; continue;
 }
 0x20a697a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a69ac; continue; }
  pc = 0x20a697e; continue;
 }
 0x20a697e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a8b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a69a4); cpu.call(t); if cpu.returned_to != 0x20a69a4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a69ac); f_020adba0(cpu); if cpu.returned_to != 0x20a69ac { return; }
  pc = 0x20a69ac; continue;
 }
 0x20a69ac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a69b9); f_020bbfe0(cpu); if cpu.returned_to != 0x20a69b9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a6a07; continue; }
  pc = 0x20a69c8; continue;
 }
 0x20a69c8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a69da; continue; }
  pc = 0x20a69d1; continue;
 }
 0x20a69d1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6a07; continue; }
  pc = 0x20a69da; continue;
 }
 0x20a69da => {
  cpu.push(0x20a69df); f_020b4041(cpu); if cpu.returned_to != 0x20a69df { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6a07; continue; }
  pc = 0x20a69e3; continue;
 }
 0x20a69e3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a69fd); f_020b42d3(cpu); if cpu.returned_to != 0x20a69fd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6a07); f_020adb50(cpu); if cpu.returned_to != 0x20a6a07 { return; }
  pc = 0x20a6a07; continue;
 }
 0x20a6a07 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6a30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6a30); }
 let mut pc: u32 = 0x20a6a30;
 loop { match pc {
 0x20a6a30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6a78; continue; }
  pc = 0x20a6a51; continue;
 }
 0x20a6a51 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a6a5f; continue; }
  pc = 0x20a6a56; continue;
 }
 0x20a6a56 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6a78; continue; }
  pc = 0x20a6a5f; continue;
 }
 0x20a6a5f => {
  cpu.push(0x20a6a64); f_020b4041(cpu); if cpu.returned_to != 0x20a6a64 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6a78; continue; }
  pc = 0x20a6a68; continue;
 }
 0x20a6a68 => {
  cpu.push(0x20a6a6d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a6a6d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6a78; continue; }
  pc = 0x20a6a71; continue;
 }
 0x20a6a71 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6a7a; continue;
 }
 0x20a6a78 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6a7a; continue;
 }
 0x20a6a7a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6aac; continue; }
  pc = 0x20a6a7e; continue;
 }
 0x20a6a7e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a894u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6aa4); cpu.call(t); if cpu.returned_to != 0x20a6aa4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a6aac); f_020adba0(cpu); if cpu.returned_to != 0x20a6aac { return; }
  pc = 0x20a6aac; continue;
 }
 0x20a6aac => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6ab9); f_020bc000(cpu); if cpu.returned_to != 0x20a6ab9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a6b07; continue; }
  pc = 0x20a6ac8; continue;
 }
 0x20a6ac8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a6ada; continue; }
  pc = 0x20a6ad1; continue;
 }
 0x20a6ad1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6b07; continue; }
  pc = 0x20a6ada; continue;
 }
 0x20a6ada => {
  cpu.push(0x20a6adf); f_020b4041(cpu); if cpu.returned_to != 0x20a6adf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6b07; continue; }
  pc = 0x20a6ae3; continue;
 }
 0x20a6ae3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a6afd); f_020b42d3(cpu); if cpu.returned_to != 0x20a6afd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6b07); f_020adb50(cpu); if cpu.returned_to != 0x20a6b07 { return; }
  pc = 0x20a6b07; continue;
 }
 0x20a6b07 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6b30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6b30); }
 let mut pc: u32 = 0x20a6b30;
 loop { match pc {
 0x20a6b30 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6b77; continue; }
  pc = 0x20a6b50; continue;
 }
 0x20a6b50 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a6b5e; continue; }
  pc = 0x20a6b55; continue;
 }
 0x20a6b55 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6b77; continue; }
  pc = 0x20a6b5e; continue;
 }
 0x20a6b5e => {
  cpu.push(0x20a6b63); f_020b4041(cpu); if cpu.returned_to != 0x20a6b63 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6b77; continue; }
  pc = 0x20a6b67; continue;
 }
 0x20a6b67 => {
  cpu.push(0x20a6b6c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a6b6c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6b77; continue; }
  pc = 0x20a6b70; continue;
 }
 0x20a6b70 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6b79; continue;
 }
 0x20a6b77 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6b79; continue;
 }
 0x20a6b79 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6bab; continue; }
  pc = 0x20a6b7d; continue;
 }
 0x20a6b7d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a874u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6ba3); cpu.call(t); if cpu.returned_to != 0x20a6ba3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a6bab); f_020adba0(cpu); if cpu.returned_to != 0x20a6bab { return; }
  pc = 0x20a6bab; continue;
 }
 0x20a6bab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6bb8); f_020bc060(cpu); if cpu.returned_to != 0x20a6bb8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a6c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6c90); }
 let mut pc: u32 = 0x20a6c90;
 loop { match pc {
 0x20a6c90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6cd7; continue; }
  pc = 0x20a6cb0; continue;
 }
 0x20a6cb0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a6cbe; continue; }
  pc = 0x20a6cb5; continue;
 }
 0x20a6cb5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6cd7; continue; }
  pc = 0x20a6cbe; continue;
 }
 0x20a6cbe => {
  cpu.push(0x20a6cc3); f_020b4041(cpu); if cpu.returned_to != 0x20a6cc3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6cd7; continue; }
  pc = 0x20a6cc7; continue;
 }
 0x20a6cc7 => {
  cpu.push(0x20a6ccc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a6ccc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6cd7; continue; }
  pc = 0x20a6cd0; continue;
 }
 0x20a6cd0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6cd9; continue;
 }
 0x20a6cd7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6cd9; continue;
 }
 0x20a6cd9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6d0b; continue; }
  pc = 0x20a6cdd; continue;
 }
 0x20a6cdd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a83cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6d03); cpu.call(t); if cpu.returned_to != 0x20a6d03 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a6d0b); f_020adba0(cpu); if cpu.returned_to != 0x20a6d0b { return; }
  pc = 0x20a6d0b; continue;
 }
 0x20a6d0b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6d18); f_020bc0a0(cpu); if cpu.returned_to != 0x20a6d18 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a6d40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6d40); }
 let mut pc: u32 = 0x20a6d40;
 loop { match pc {
 0x20a6d40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6d88; continue; }
  pc = 0x20a6d61; continue;
 }
 0x20a6d61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a6d6f; continue; }
  pc = 0x20a6d66; continue;
 }
 0x20a6d66 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6d88; continue; }
  pc = 0x20a6d6f; continue;
 }
 0x20a6d6f => {
  cpu.push(0x20a6d74); f_020b4041(cpu); if cpu.returned_to != 0x20a6d74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6d88; continue; }
  pc = 0x20a6d78; continue;
 }
 0x20a6d78 => {
  cpu.push(0x20a6d7d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a6d7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6d88; continue; }
  pc = 0x20a6d81; continue;
 }
 0x20a6d81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6d8a; continue;
 }
 0x20a6d88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6d8a; continue;
 }
 0x20a6d8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6dbc; continue; }
  pc = 0x20a6d8e; continue;
 }
 0x20a6d8e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a824u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6db4); cpu.call(t); if cpu.returned_to != 0x20a6db4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a6dbc); f_020adba0(cpu); if cpu.returned_to != 0x20a6dbc { return; }
  pc = 0x20a6dbc; continue;
 }
 0x20a6dbc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6dc9); f_020bc0f0(cpu); if cpu.returned_to != 0x20a6dc9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a6e17; continue; }
  pc = 0x20a6dd8; continue;
 }
 0x20a6dd8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a6dea; continue; }
  pc = 0x20a6de1; continue;
 }
 0x20a6de1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6e17; continue; }
  pc = 0x20a6dea; continue;
 }
 0x20a6dea => {
  cpu.push(0x20a6def); f_020b4041(cpu); if cpu.returned_to != 0x20a6def { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6e17; continue; }
  pc = 0x20a6df3; continue;
 }
 0x20a6df3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a6e0d); f_020b42d3(cpu); if cpu.returned_to != 0x20a6e0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a6e17); f_020adb00(cpu); if cpu.returned_to != 0x20a6e17 { return; }
  pc = 0x20a6e17; continue;
 }
 0x20a6e17 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a6e40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a6e40); }
 let mut pc: u32 = 0x20a6e40;
 loop { match pc {
 0x20a6e40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a6e87; continue; }
  pc = 0x20a6e60; continue;
 }
 0x20a6e60 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a6e6e; continue; }
  pc = 0x20a6e65; continue;
 }
 0x20a6e65 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a6e87; continue; }
  pc = 0x20a6e6e; continue;
 }
 0x20a6e6e => {
  cpu.push(0x20a6e73); f_020b4041(cpu); if cpu.returned_to != 0x20a6e73 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a6e87; continue; }
  pc = 0x20a6e77; continue;
 }
 0x20a6e77 => {
  cpu.push(0x20a6e7c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a6e7c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6e87; continue; }
  pc = 0x20a6e80; continue;
 }
 0x20a6e80 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a6e89; continue;
 }
 0x20a6e87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a6e89; continue;
 }
 0x20a6e89 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a6ebb; continue; }
  pc = 0x20a6e8d; continue;
 }
 0x20a6e8d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a7fcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a6eb3); cpu.call(t); if cpu.returned_to != 0x20a6eb3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a6ebb); f_020adba0(cpu); if cpu.returned_to != 0x20a6ebb { return; }
  pc = 0x20a6ebb; continue;
 }
 0x20a6ebb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a6ec8); f_020bb280(cpu); if cpu.returned_to != 0x20a6ec8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a72f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a72f0); }
 let mut pc: u32 = 0x20a72f0;
 loop { match pc {
 0x20a72f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7337; continue; }
  pc = 0x20a7310; continue;
 }
 0x20a7310 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a731e; continue; }
  pc = 0x20a7315; continue;
 }
 0x20a7315 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7337; continue; }
  pc = 0x20a731e; continue;
 }
 0x20a731e => {
  cpu.push(0x20a7323); f_020b4041(cpu); if cpu.returned_to != 0x20a7323 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7337; continue; }
  pc = 0x20a7327; continue;
 }
 0x20a7327 => {
  cpu.push(0x20a732c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a732c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7337; continue; }
  pc = 0x20a7330; continue;
 }
 0x20a7330 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a7339; continue;
 }
 0x20a7337 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a7339; continue;
 }
 0x20a7339 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7387; continue; }
  pc = 0x20a733d; continue;
 }
 0x20a733d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a718u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a737f); cpu.call(t); if cpu.returned_to != 0x20a737f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7387); f_020adba0(cpu); if cpu.returned_to != 0x20a7387 { return; }
  pc = 0x20a7387; continue;
 }
 0x20a7387 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a73a4); f_020b7a10(cpu); if cpu.returned_to != 0x20a73a4 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a73c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a73c0); }
 let mut pc: u32 = 0x20a73c0;
 loop { match pc {
 0x20a73c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7407; continue; }
  pc = 0x20a73e0; continue;
 }
 0x20a73e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a73ee; continue; }
  pc = 0x20a73e5; continue;
 }
 0x20a73e5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7407; continue; }
  pc = 0x20a73ee; continue;
 }
 0x20a73ee => {
  cpu.push(0x20a73f3); f_020b4041(cpu); if cpu.returned_to != 0x20a73f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7407; continue; }
  pc = 0x20a73f7; continue;
 }
 0x20a73f7 => {
  cpu.push(0x20a73fc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a73fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7407; continue; }
  pc = 0x20a7400; continue;
 }
 0x20a7400 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a7409; continue;
 }
 0x20a7407 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a7409; continue;
 }
 0x20a7409 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7449; continue; }
  pc = 0x20a740d; continue;
 }
 0x20a740d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a6f8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a7441); cpu.call(t); if cpu.returned_to != 0x20a7441 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7449); f_020adba0(cpu); if cpu.returned_to != 0x20a7449 { return; }
  pc = 0x20a7449; continue;
 }
 0x20a7449 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a745e); f_020b7a50(cpu); if cpu.returned_to != 0x20a745e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a7480(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7480); }
 let mut pc: u32 = 0x20a7480;
 loop { match pc {
 0x20a7480 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a74c8; continue; }
  pc = 0x20a74a1; continue;
 }
 0x20a74a1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a74af; continue; }
  pc = 0x20a74a6; continue;
 }
 0x20a74a6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a74c8; continue; }
  pc = 0x20a74af; continue;
 }
 0x20a74af => {
  cpu.push(0x20a74b4); f_020b4041(cpu); if cpu.returned_to != 0x20a74b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a74c8; continue; }
  pc = 0x20a74b8; continue;
 }
 0x20a74b8 => {
  cpu.push(0x20a74bd); f_020ad7d0(cpu); if cpu.returned_to != 0x20a74bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a74c8; continue; }
  pc = 0x20a74c1; continue;
 }
 0x20a74c1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a74ca; continue;
 }
 0x20a74c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a74ca; continue;
 }
 0x20a74ca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a74fc; continue; }
  pc = 0x20a74ce; continue;
 }
 0x20a74ce => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a6dcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a74f4); cpu.call(t); if cpu.returned_to != 0x20a74f4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a74fc); f_020adba0(cpu); if cpu.returned_to != 0x20a74fc { return; }
  pc = 0x20a74fc; continue;
 }
 0x20a74fc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a7509); f_020b7b70(cpu); if cpu.returned_to != 0x20a7509 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a7557; continue; }
  pc = 0x20a7518; continue;
 }
 0x20a7518 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a752a; continue; }
  pc = 0x20a7521; continue;
 }
 0x20a7521 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7557; continue; }
  pc = 0x20a752a; continue;
 }
 0x20a752a => {
  cpu.push(0x20a752f); f_020b4041(cpu); if cpu.returned_to != 0x20a752f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7557; continue; }
  pc = 0x20a7533; continue;
 }
 0x20a7533 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a754d); f_020b42d3(cpu); if cpu.returned_to != 0x20a754d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a7557); f_020adb50(cpu); if cpu.returned_to != 0x20a7557 { return; }
  pc = 0x20a7557; continue;
 }
 0x20a7557 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a7580(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7580); }
 let mut pc: u32 = 0x20a7580;
 loop { match pc {
 0x20a7580 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a75c7; continue; }
  pc = 0x20a75a0; continue;
 }
 0x20a75a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a75ae; continue; }
  pc = 0x20a75a5; continue;
 }
 0x20a75a5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a75c7; continue; }
  pc = 0x20a75ae; continue;
 }
 0x20a75ae => {
  cpu.push(0x20a75b3); f_020b4041(cpu); if cpu.returned_to != 0x20a75b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a75c7; continue; }
  pc = 0x20a75b7; continue;
 }
 0x20a75b7 => {
  cpu.push(0x20a75bc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a75bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a75c7; continue; }
  pc = 0x20a75c0; continue;
 }
 0x20a75c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a75c9; continue;
 }
 0x20a75c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a75c9; continue;
 }
 0x20a75c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7609; continue; }
  pc = 0x20a75cd; continue;
 }
 0x20a75cd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a6b8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a7601); cpu.call(t); if cpu.returned_to != 0x20a7601 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7609); f_020adba0(cpu); if cpu.returned_to != 0x20a7609 { return; }
  pc = 0x20a7609; continue;
 }
 0x20a7609 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a761e); f_020b7cf0(cpu); if cpu.returned_to != 0x20a761e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a7640(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7640); }
 let mut pc: u32 = 0x20a7640;
 loop { match pc {
 0x20a7640 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7688; continue; }
  pc = 0x20a7661; continue;
 }
 0x20a7661 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a766f; continue; }
  pc = 0x20a7666; continue;
 }
 0x20a7666 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7688; continue; }
  pc = 0x20a766f; continue;
 }
 0x20a766f => {
  cpu.push(0x20a7674); f_020b4041(cpu); if cpu.returned_to != 0x20a7674 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7688; continue; }
  pc = 0x20a7678; continue;
 }
 0x20a7678 => {
  cpu.push(0x20a767d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a767d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7688; continue; }
  pc = 0x20a7681; continue;
 }
 0x20a7681 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a768a; continue;
 }
 0x20a7688 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a768a; continue;
 }
 0x20a768a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a76ca; continue; }
  pc = 0x20a768e; continue;
 }
 0x20a768e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a694u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a76c2); cpu.call(t); if cpu.returned_to != 0x20a76c2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a76ca); f_020adba0(cpu); if cpu.returned_to != 0x20a76ca { return; }
  pc = 0x20a76ca; continue;
 }
 0x20a76ca => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a76df); f_020b7d00(cpu); if cpu.returned_to != 0x20a76df { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a772c; continue; }
  pc = 0x20a76ed; continue;
 }
 0x20a76ed => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a76ff; continue; }
  pc = 0x20a76f6; continue;
 }
 0x20a76f6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a772c; continue; }
  pc = 0x20a76ff; continue;
 }
 0x20a76ff => {
  cpu.push(0x20a7704); f_020b4041(cpu); if cpu.returned_to != 0x20a7704 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a772c; continue; }
  pc = 0x20a7708; continue;
 }
 0x20a7708 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a7722); f_020b42d3(cpu); if cpu.returned_to != 0x20a7722 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a772c); f_020adb50(cpu); if cpu.returned_to != 0x20a772c { return; }
  pc = 0x20a772c; continue;
 }
 0x20a772c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a7750(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7750); }
 let mut pc: u32 = 0x20a7750;
 loop { match pc {
 0x20a7750 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7797; continue; }
  pc = 0x20a7770; continue;
 }
 0x20a7770 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a777e; continue; }
  pc = 0x20a7775; continue;
 }
 0x20a7775 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7797; continue; }
  pc = 0x20a777e; continue;
 }
 0x20a777e => {
  cpu.push(0x20a7783); f_020b4041(cpu); if cpu.returned_to != 0x20a7783 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7797; continue; }
  pc = 0x20a7787; continue;
 }
 0x20a7787 => {
  cpu.push(0x20a778c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a778c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7797; continue; }
  pc = 0x20a7790; continue;
 }
 0x20a7790 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a7799; continue;
 }
 0x20a7797 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a7799; continue;
 }
 0x20a7799 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a77e7; continue; }
  pc = 0x20a779d; continue;
 }
 0x20a779d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a668u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a77df); cpu.call(t); if cpu.returned_to != 0x20a77df { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a77e7); f_020adba0(cpu); if cpu.returned_to != 0x20a77e7 { return; }
  pc = 0x20a77e7; continue;
 }
 0x20a77e7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7804); f_020b7d20(cpu); if cpu.returned_to != 0x20a7804 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a7820(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7820); }
 let mut pc: u32 = 0x20a7820;
 loop { match pc {
 0x20a7820 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7868; continue; }
  pc = 0x20a7841; continue;
 }
 0x20a7841 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a784f; continue; }
  pc = 0x20a7846; continue;
 }
 0x20a7846 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7868; continue; }
  pc = 0x20a784f; continue;
 }
 0x20a784f => {
  cpu.push(0x20a7854); f_020b4041(cpu); if cpu.returned_to != 0x20a7854 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7868; continue; }
  pc = 0x20a7858; continue;
 }
 0x20a7858 => {
  cpu.push(0x20a785d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a785d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7868; continue; }
  pc = 0x20a7861; continue;
 }
 0x20a7861 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a786a; continue;
 }
 0x20a7868 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a786a; continue;
 }
 0x20a786a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a78aa; continue; }
  pc = 0x20a786e; continue;
 }
 0x20a786e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a63cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a78a2); cpu.call(t); if cpu.returned_to != 0x20a78a2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a78aa); f_020adba0(cpu); if cpu.returned_to != 0x20a78aa { return; }
  pc = 0x20a78aa; continue;
 }
 0x20a78aa => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a78bf); f_020b7d40(cpu); if cpu.returned_to != 0x20a78bf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a790c; continue; }
  pc = 0x20a78cd; continue;
 }
 0x20a78cd => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a78df; continue; }
  pc = 0x20a78d6; continue;
 }
 0x20a78d6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a790c; continue; }
  pc = 0x20a78df; continue;
 }
 0x20a78df => {
  cpu.push(0x20a78e4); f_020b4041(cpu); if cpu.returned_to != 0x20a78e4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a790c; continue; }
  pc = 0x20a78e8; continue;
 }
 0x20a78e8 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a7902); f_020b42d3(cpu); if cpu.returned_to != 0x20a7902 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a790c); f_020adb00(cpu); if cpu.returned_to != 0x20a790c { return; }
  pc = 0x20a790c; continue;
 }
 0x20a790c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a7930(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7930); }
 let mut pc: u32 = 0x20a7930;
 loop { match pc {
 0x20a7930 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7978; continue; }
  pc = 0x20a7951; continue;
 }
 0x20a7951 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a795f; continue; }
  pc = 0x20a7956; continue;
 }
 0x20a7956 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7978; continue; }
  pc = 0x20a795f; continue;
 }
 0x20a795f => {
  cpu.push(0x20a7964); f_020b4041(cpu); if cpu.returned_to != 0x20a7964 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7978; continue; }
  pc = 0x20a7968; continue;
 }
 0x20a7968 => {
  cpu.push(0x20a796d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a796d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7978; continue; }
  pc = 0x20a7971; continue;
 }
 0x20a7971 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a797a; continue;
 }
 0x20a7978 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a797a; continue;
 }
 0x20a797a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a79ba; continue; }
  pc = 0x20a797e; continue;
 }
 0x20a797e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a610u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a79b2); cpu.call(t); if cpu.returned_to != 0x20a79b2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a79ba); f_020adba0(cpu); if cpu.returned_to != 0x20a79ba { return; }
  pc = 0x20a79ba; continue;
 }
 0x20a79ba => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a79cf); f_020b7d60(cpu); if cpu.returned_to != 0x20a79cf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7a1c; continue; }
  pc = 0x20a79dd; continue;
 }
 0x20a79dd => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a79ef; continue; }
  pc = 0x20a79e6; continue;
 }
 0x20a79e6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7a1c; continue; }
  pc = 0x20a79ef; continue;
 }
 0x20a79ef => {
  cpu.push(0x20a79f4); f_020b4041(cpu); if cpu.returned_to != 0x20a79f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7a1c; continue; }
  pc = 0x20a79f8; continue;
 }
 0x20a79f8 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a7a12); f_020b42d3(cpu); if cpu.returned_to != 0x20a7a12 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a7a1c); f_020adb00(cpu); if cpu.returned_to != 0x20a7a1c { return; }
  pc = 0x20a7a1c; continue;
 }
 0x20a7a1c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a7b50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7b50); }
 let mut pc: u32 = 0x20a7b50;
 loop { match pc {
 0x20a7b50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7b98; continue; }
  pc = 0x20a7b71; continue;
 }
 0x20a7b71 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a7b7f; continue; }
  pc = 0x20a7b76; continue;
 }
 0x20a7b76 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7b98; continue; }
  pc = 0x20a7b7f; continue;
 }
 0x20a7b7f => {
  cpu.push(0x20a7b84); f_020b4041(cpu); if cpu.returned_to != 0x20a7b84 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7b98; continue; }
  pc = 0x20a7b88; continue;
 }
 0x20a7b88 => {
  cpu.push(0x20a7b8d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a7b8d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7b98; continue; }
  pc = 0x20a7b91; continue;
 }
 0x20a7b91 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a7b9a; continue;
 }
 0x20a7b98 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a7b9a; continue;
 }
 0x20a7b9a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7bda; continue; }
  pc = 0x20a7b9e; continue;
 }
 0x20a7b9e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a5bcu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a7bd2); cpu.call(t); if cpu.returned_to != 0x20a7bd2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7bda); f_020adba0(cpu); if cpu.returned_to != 0x20a7bda { return; }
  pc = 0x20a7bda; continue;
 }
 0x20a7bda => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a7bef); f_020b7da0(cpu); if cpu.returned_to != 0x20a7bef { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7c3c; continue; }
  pc = 0x20a7bfd; continue;
 }
 0x20a7bfd => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a7c0f; continue; }
  pc = 0x20a7c06; continue;
 }
 0x20a7c06 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7c3c; continue; }
  pc = 0x20a7c0f; continue;
 }
 0x20a7c0f => {
  cpu.push(0x20a7c14); f_020b4041(cpu); if cpu.returned_to != 0x20a7c14 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7c3c; continue; }
  pc = 0x20a7c18; continue;
 }
 0x20a7c18 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a7c32); f_020b42d3(cpu); if cpu.returned_to != 0x20a7c32 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a7c3c); f_020adb00(cpu); if cpu.returned_to != 0x20a7c3c { return; }
  pc = 0x20a7c3c; continue;
 }
 0x20a7c3c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a7e80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7e80); }
 let mut pc: u32 = 0x20a7e80;
 loop { match pc {
 0x20a7e80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7ec7; continue; }
  pc = 0x20a7ea0; continue;
 }
 0x20a7ea0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a7eae; continue; }
  pc = 0x20a7ea5; continue;
 }
 0x20a7ea5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7ec7; continue; }
  pc = 0x20a7eae; continue;
 }
 0x20a7eae => {
  cpu.push(0x20a7eb3); f_020b4041(cpu); if cpu.returned_to != 0x20a7eb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7ec7; continue; }
  pc = 0x20a7eb7; continue;
 }
 0x20a7eb7 => {
  cpu.push(0x20a7ebc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a7ebc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7ec7; continue; }
  pc = 0x20a7ec0; continue;
 }
 0x20a7ec0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a7ec9; continue;
 }
 0x20a7ec7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a7ec9; continue;
 }
 0x20a7ec9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7f09; continue; }
  pc = 0x20a7ecd; continue;
 }
 0x20a7ecd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a548u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a7f01); cpu.call(t); if cpu.returned_to != 0x20a7f01 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7f09); f_020adba0(cpu); if cpu.returned_to != 0x20a7f09 { return; }
  pc = 0x20a7f09; continue;
 }
 0x20a7f09 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a7f1e); f_020b7e00(cpu); if cpu.returned_to != 0x20a7f1e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a7f40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a7f40); }
 let mut pc: u32 = 0x20a7f40;
 loop { match pc {
 0x20a7f40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x114u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x12cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a7f96; continue; }
  pc = 0x20a7f6f; continue;
 }
 0x20a7f6f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a7f7d; continue; }
  pc = 0x20a7f74; continue;
 }
 0x20a7f74 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a7f96; continue; }
  pc = 0x20a7f7d; continue;
 }
 0x20a7f7d => {
  cpu.push(0x20a7f82); f_020b4041(cpu); if cpu.returned_to != 0x20a7f82 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a7f96; continue; }
  pc = 0x20a7f86; continue;
 }
 0x20a7f86 => {
  cpu.push(0x20a7f8b); f_020ad7d0(cpu); if cpu.returned_to != 0x20a7f8b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7f96; continue; }
  pc = 0x20a7f8f; continue;
 }
 0x20a7f8f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a7f98; continue;
 }
 0x20a7f96 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a7f98; continue;
 }
 0x20a7f98 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a7fd8; continue; }
  pc = 0x20a7f9c; continue;
 }
 0x20a7f9c => {
  { let v: u32 = cpu.r32(0x128u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x100u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a50cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x10cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x114u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x118u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a7fd0); cpu.call(t); if cpu.returned_to != 0x20a7fd0 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7fd8); f_020adba0(cpu); if cpu.returned_to != 0x20a7fd8 { return; }
  pc = 0x20a7fd8; continue;
 }
 0x20a7fd8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x130u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a7fe7); f_020b7e90(cpu); if cpu.returned_to != 0x20a7fe7 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20a806e; continue; }
  pc = 0x20a7ff8; continue;
 }
 0x20a7ff8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a800a; continue; }
  pc = 0x20a8001; continue;
 }
 0x20a8001 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a806e; continue; }
  pc = 0x20a800a; continue;
 }
 0x20a800a => {
  cpu.push(0x20a800f); f_020b4041(cpu); if cpu.returned_to != 0x20a800f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a806e; continue; }
  pc = 0x20a8013; continue;
 }
 0x20a8013 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a802d); f_020b42d3(cpu); if cpu.returned_to != 0x20a802d { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x110u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a534u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x118u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x11cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a8061); cpu.call(t); if cpu.returned_to != 0x20a8061 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a806e); f_020adba0(cpu); if cpu.returned_to != 0x20a806e { return; }
  pc = 0x20a806e; continue;
 }
 0x20a806e => {
  { let v: u32 = cpu.dec32(cpu.r32(0x22d13e4u32)); cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a8080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a8080); }
 let mut pc: u32 = 0x20a8080;
 loop { match pc {
 0x20a8080 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a80c8; continue; }
  pc = 0x20a80a1; continue;
 }
 0x20a80a1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a80af; continue; }
  pc = 0x20a80a6; continue;
 }
 0x20a80a6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a80c8; continue; }
  pc = 0x20a80af; continue;
 }
 0x20a80af => {
  cpu.push(0x20a80b4); f_020b4041(cpu); if cpu.returned_to != 0x20a80b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a80c8; continue; }
  pc = 0x20a80b8; continue;
 }
 0x20a80b8 => {
  cpu.push(0x20a80bd); f_020ad7d0(cpu); if cpu.returned_to != 0x20a80bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a80c8; continue; }
  pc = 0x20a80c1; continue;
 }
 0x20a80c1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a80ca; continue;
 }
 0x20a80c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a80ca; continue;
 }
 0x20a80ca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a810a; continue; }
  pc = 0x20a80ce; continue;
 }
 0x20a80ce => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a4ecu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a8102); cpu.call(t); if cpu.returned_to != 0x20a8102 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a810a); f_020adba0(cpu); if cpu.returned_to != 0x20a810a { return; }
  pc = 0x20a810a; continue;
 }
 0x20a810a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a811f); f_020b7f10(cpu); if cpu.returned_to != 0x20a811f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a816c; continue; }
  pc = 0x20a812d; continue;
 }
 0x20a812d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a813f; continue; }
  pc = 0x20a8136; continue;
 }
 0x20a8136 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a816c; continue; }
  pc = 0x20a813f; continue;
 }
 0x20a813f => {
  cpu.push(0x20a8144); f_020b4041(cpu); if cpu.returned_to != 0x20a8144 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a816c; continue; }
  pc = 0x20a8148; continue;
 }
 0x20a8148 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a8162); f_020b42d3(cpu); if cpu.returned_to != 0x20a8162 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a816c); f_020adb50(cpu); if cpu.returned_to != 0x20a816c { return; }
  pc = 0x20a816c; continue;
 }
 0x20a816c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020a8790(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a8790); }
 let mut pc: u32 = 0x20a8790;
 loop { match pc {
 0x20a8790 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a87d7; continue; }
  pc = 0x20a87b0; continue;
 }
 0x20a87b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a87be; continue; }
  pc = 0x20a87b5; continue;
 }
 0x20a87b5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a87d7; continue; }
  pc = 0x20a87be; continue;
 }
 0x20a87be => {
  cpu.push(0x20a87c3); f_020b4041(cpu); if cpu.returned_to != 0x20a87c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a87d7; continue; }
  pc = 0x20a87c7; continue;
 }
 0x20a87c7 => {
  cpu.push(0x20a87cc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a87cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a87d7; continue; }
  pc = 0x20a87d0; continue;
 }
 0x20a87d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a87d9; continue;
 }
 0x20a87d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a87d9; continue;
 }
 0x20a87d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a880b; continue; }
  pc = 0x20a87dd; continue;
 }
 0x20a87dd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a3f4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a8803); cpu.call(t); if cpu.returned_to != 0x20a8803 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a880b); f_020adba0(cpu); if cpu.returned_to != 0x20a880b { return; }
  pc = 0x20a880b; continue;
 }
 0x20a880b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a8818); f_020b89a0(cpu); if cpu.returned_to != 0x20a8818 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a8840(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a8840); }
 let mut pc: u32 = 0x20a8840;
 loop { match pc {
 0x20a8840 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a8888; continue; }
  pc = 0x20a8861; continue;
 }
 0x20a8861 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a886f; continue; }
  pc = 0x20a8866; continue;
 }
 0x20a8866 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a8888; continue; }
  pc = 0x20a886f; continue;
 }
 0x20a886f => {
  cpu.push(0x20a8874); f_020b4041(cpu); if cpu.returned_to != 0x20a8874 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a8888; continue; }
  pc = 0x20a8878; continue;
 }
 0x20a8878 => {
  cpu.push(0x20a887d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a887d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a8888; continue; }
  pc = 0x20a8881; continue;
 }
 0x20a8881 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a888a; continue;
 }
 0x20a8888 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a888a; continue;
 }
 0x20a888a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a88bc; continue; }
  pc = 0x20a888e; continue;
 }
 0x20a888e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a3d8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a88b4); cpu.call(t); if cpu.returned_to != 0x20a88b4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a88bc); f_020adba0(cpu); if cpu.returned_to != 0x20a88bc { return; }
  pc = 0x20a88bc; continue;
 }
 0x20a88bc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a88c9); f_020b8a20(cpu); if cpu.returned_to != 0x20a88c9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a8917; continue; }
  pc = 0x20a88d8; continue;
 }
 0x20a88d8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a88ea; continue; }
  pc = 0x20a88e1; continue;
 }
 0x20a88e1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a8917; continue; }
  pc = 0x20a88ea; continue;
 }
 0x20a88ea => {
  cpu.push(0x20a88ef); f_020b4041(cpu); if cpu.returned_to != 0x20a88ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a8917; continue; }
  pc = 0x20a88f3; continue;
 }
 0x20a88f3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a890d); f_020b42d3(cpu); if cpu.returned_to != 0x20a890d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a8917); f_020adb50(cpu); if cpu.returned_to != 0x20a8917 { return; }
  pc = 0x20a8917; continue;
 }
 0x20a8917 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a8f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a8f20); }
 let mut pc: u32 = 0x20a8f20;
 loop { match pc {
 0x20a8f20 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a8f68; continue; }
  pc = 0x20a8f41; continue;
 }
 0x20a8f41 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a8f4f; continue; }
  pc = 0x20a8f46; continue;
 }
 0x20a8f46 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a8f68; continue; }
  pc = 0x20a8f4f; continue;
 }
 0x20a8f4f => {
  cpu.push(0x20a8f54); f_020b4041(cpu); if cpu.returned_to != 0x20a8f54 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a8f68; continue; }
  pc = 0x20a8f58; continue;
 }
 0x20a8f58 => {
  cpu.push(0x20a8f5d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a8f5d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a8f68; continue; }
  pc = 0x20a8f61; continue;
 }
 0x20a8f61 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a8f6a; continue;
 }
 0x20a8f68 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a8f6a; continue;
 }
 0x20a8f6a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a8f9c; continue; }
  pc = 0x20a8f6e; continue;
 }
 0x20a8f6e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a2ecu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a8f94); cpu.call(t); if cpu.returned_to != 0x20a8f94 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a8f9c); f_020adba0(cpu); if cpu.returned_to != 0x20a8f9c { return; }
  pc = 0x20a8f9c; continue;
 }
 0x20a8f9c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a8fa9); f_020b8200(cpu); if cpu.returned_to != 0x20a8fa9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a8ff7; continue; }
  pc = 0x20a8fb8; continue;
 }
 0x20a8fb8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a8fca; continue; }
  pc = 0x20a8fc1; continue;
 }
 0x20a8fc1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a8ff7; continue; }
  pc = 0x20a8fca; continue;
 }
 0x20a8fca => {
  cpu.push(0x20a8fcf); f_020b4041(cpu); if cpu.returned_to != 0x20a8fcf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a8ff7; continue; }
  pc = 0x20a8fd3; continue;
 }
 0x20a8fd3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a8fed); f_020b42d3(cpu); if cpu.returned_to != 0x20a8fed { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a8ff7); f_020adb50(cpu); if cpu.returned_to != 0x20a8ff7 { return; }
  pc = 0x20a8ff7; continue;
 }
 0x20a8ff7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020a9020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9020); }
 let mut pc: u32 = 0x20a9020;
 loop { match pc {
 0x20a9020 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9067; continue; }
  pc = 0x20a9040; continue;
 }
 0x20a9040 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a904e; continue; }
  pc = 0x20a9045; continue;
 }
 0x20a9045 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9067; continue; }
  pc = 0x20a904e; continue;
 }
 0x20a904e => {
  cpu.push(0x20a9053); f_020b4041(cpu); if cpu.returned_to != 0x20a9053 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9067; continue; }
  pc = 0x20a9057; continue;
 }
 0x20a9057 => {
  cpu.push(0x20a905c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a905c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9067; continue; }
  pc = 0x20a9060; continue;
 }
 0x20a9060 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9069; continue;
 }
 0x20a9067 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9069; continue;
 }
 0x20a9069 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a90a9; continue; }
  pc = 0x20a906d; continue;
 }
 0x20a906d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a2c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a90a1); cpu.call(t); if cpu.returned_to != 0x20a90a1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a90a9); f_020adba0(cpu); if cpu.returned_to != 0x20a90a9 { return; }
  pc = 0x20a90a9; continue;
 }
 0x20a90a9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a90be); f_020b8300(cpu); if cpu.returned_to != 0x20a90be { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a90e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a90e0); }
 let mut pc: u32 = 0x20a90e0;
 loop { match pc {
 0x20a90e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9127; continue; }
  pc = 0x20a9100; continue;
 }
 0x20a9100 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a910e; continue; }
  pc = 0x20a9105; continue;
 }
 0x20a9105 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9127; continue; }
  pc = 0x20a910e; continue;
 }
 0x20a910e => {
  cpu.push(0x20a9113); f_020b4041(cpu); if cpu.returned_to != 0x20a9113 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9127; continue; }
  pc = 0x20a9117; continue;
 }
 0x20a9117 => {
  cpu.push(0x20a911c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a911c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9127; continue; }
  pc = 0x20a9120; continue;
 }
 0x20a9120 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9129; continue;
 }
 0x20a9127 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9129; continue;
 }
 0x20a9129 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9177; continue; }
  pc = 0x20a912d; continue;
 }
 0x20a912d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a29cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a916f); cpu.call(t); if cpu.returned_to != 0x20a916f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9177); f_020adba0(cpu); if cpu.returned_to != 0x20a9177 { return; }
  pc = 0x20a9177; continue;
 }
 0x20a9177 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9194); f_020b8210(cpu); if cpu.returned_to != 0x20a9194 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a91b0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a91b0); }
 let mut pc: u32 = 0x20a91b0;
 loop { match pc {
 0x20a91b0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a91f7; continue; }
  pc = 0x20a91d0; continue;
 }
 0x20a91d0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a91de; continue; }
  pc = 0x20a91d5; continue;
 }
 0x20a91d5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a91f7; continue; }
  pc = 0x20a91de; continue;
 }
 0x20a91de => {
  cpu.push(0x20a91e3); f_020b4041(cpu); if cpu.returned_to != 0x20a91e3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a91f7; continue; }
  pc = 0x20a91e7; continue;
 }
 0x20a91e7 => {
  cpu.push(0x20a91ec); f_020ad7d0(cpu); if cpu.returned_to != 0x20a91ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a91f7; continue; }
  pc = 0x20a91f0; continue;
 }
 0x20a91f0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a91f9; continue;
 }
 0x20a91f7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a91f9; continue;
 }
 0x20a91f9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9247; continue; }
  pc = 0x20a91fd; continue;
 }
 0x20a91fd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a274u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a923f); cpu.call(t); if cpu.returned_to != 0x20a923f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9247); f_020adba0(cpu); if cpu.returned_to != 0x20a9247 { return; }
  pc = 0x20a9247; continue;
 }
 0x20a9247 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9264); f_020b8290(cpu); if cpu.returned_to != 0x20a9264 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a9280(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9280); }
 let mut pc: u32 = 0x20a9280;
 loop { match pc {
 0x20a9280 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a92c7; continue; }
  pc = 0x20a92a0; continue;
 }
 0x20a92a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a92ae; continue; }
  pc = 0x20a92a5; continue;
 }
 0x20a92a5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a92c7; continue; }
  pc = 0x20a92ae; continue;
 }
 0x20a92ae => {
  cpu.push(0x20a92b3); f_020b4041(cpu); if cpu.returned_to != 0x20a92b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a92c7; continue; }
  pc = 0x20a92b7; continue;
 }
 0x20a92b7 => {
  cpu.push(0x20a92bc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a92bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a92c7; continue; }
  pc = 0x20a92c0; continue;
 }
 0x20a92c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a92c9; continue;
 }
 0x20a92c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a92c9; continue;
 }
 0x20a92c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a92fb; continue; }
  pc = 0x20a92cd; continue;
 }
 0x20a92cd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a258u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a92f3); cpu.call(t); if cpu.returned_to != 0x20a92f3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a92fb); f_020adba0(cpu); if cpu.returned_to != 0x20a92fb { return; }
  pc = 0x20a92fb; continue;
 }
 0x20a92fb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9308); f_020b8310(cpu); if cpu.returned_to != 0x20a9308 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a9330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9330); }
 let mut pc: u32 = 0x20a9330;
 loop { match pc {
 0x20a9330 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9377; continue; }
  pc = 0x20a9350; continue;
 }
 0x20a9350 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a935e; continue; }
  pc = 0x20a9355; continue;
 }
 0x20a9355 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9377; continue; }
  pc = 0x20a935e; continue;
 }
 0x20a935e => {
  cpu.push(0x20a9363); f_020b4041(cpu); if cpu.returned_to != 0x20a9363 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9377; continue; }
  pc = 0x20a9367; continue;
 }
 0x20a9367 => {
  cpu.push(0x20a936c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a936c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9377; continue; }
  pc = 0x20a9370; continue;
 }
 0x20a9370 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9379; continue;
 }
 0x20a9377 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9379; continue;
 }
 0x20a9379 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a93ab; continue; }
  pc = 0x20a937d; continue;
 }
 0x20a937d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a23cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a93a3); cpu.call(t); if cpu.returned_to != 0x20a93a3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a93ab); f_020adba0(cpu); if cpu.returned_to != 0x20a93ab { return; }
  pc = 0x20a93ab; continue;
 }
 0x20a93ab => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a93b8); f_020b8350(cpu); if cpu.returned_to != 0x20a93b8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a93e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a93e0); }
 let mut pc: u32 = 0x20a93e0;
 loop { match pc {
 0x20a93e0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9427; continue; }
  pc = 0x20a9400; continue;
 }
 0x20a9400 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a940e; continue; }
  pc = 0x20a9405; continue;
 }
 0x20a9405 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9427; continue; }
  pc = 0x20a940e; continue;
 }
 0x20a940e => {
  cpu.push(0x20a9413); f_020b4041(cpu); if cpu.returned_to != 0x20a9413 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9427; continue; }
  pc = 0x20a9417; continue;
 }
 0x20a9417 => {
  cpu.push(0x20a941c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a941c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9427; continue; }
  pc = 0x20a9420; continue;
 }
 0x20a9420 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9429; continue;
 }
 0x20a9427 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9429; continue;
 }
 0x20a9429 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a945b; continue; }
  pc = 0x20a942d; continue;
 }
 0x20a942d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a220u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9453); cpu.call(t); if cpu.returned_to != 0x20a9453 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a945b); f_020adba0(cpu); if cpu.returned_to != 0x20a945b { return; }
  pc = 0x20a945b; continue;
 }
 0x20a945b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9468); f_020b83b0(cpu); if cpu.returned_to != 0x20a9468 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a9490(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9490); }
 let mut pc: u32 = 0x20a9490;
 loop { match pc {
 0x20a9490 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a94d7; continue; }
  pc = 0x20a94b0; continue;
 }
 0x20a94b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a94be; continue; }
  pc = 0x20a94b5; continue;
 }
 0x20a94b5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a94d7; continue; }
  pc = 0x20a94be; continue;
 }
 0x20a94be => {
  cpu.push(0x20a94c3); f_020b4041(cpu); if cpu.returned_to != 0x20a94c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a94d7; continue; }
  pc = 0x20a94c7; continue;
 }
 0x20a94c7 => {
  cpu.push(0x20a94cc); f_020ad7d0(cpu); if cpu.returned_to != 0x20a94cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a94d7; continue; }
  pc = 0x20a94d0; continue;
 }
 0x20a94d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a94d9; continue;
 }
 0x20a94d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a94d9; continue;
 }
 0x20a94d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a950b; continue; }
  pc = 0x20a94dd; continue;
 }
 0x20a94dd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a204u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9503); cpu.call(t); if cpu.returned_to != 0x20a9503 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a950b); f_020adba0(cpu); if cpu.returned_to != 0x20a950b { return; }
  pc = 0x20a950b; continue;
 }
 0x20a950b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9518); f_020b8460(cpu); if cpu.returned_to != 0x20a9518 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a9540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9540); }
 let mut pc: u32 = 0x20a9540;
 loop { match pc {
 0x20a9540 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9587; continue; }
  pc = 0x20a9560; continue;
 }
 0x20a9560 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a956e; continue; }
  pc = 0x20a9565; continue;
 }
 0x20a9565 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9587; continue; }
  pc = 0x20a956e; continue;
 }
 0x20a956e => {
  cpu.push(0x20a9573); f_020b4041(cpu); if cpu.returned_to != 0x20a9573 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9587; continue; }
  pc = 0x20a9577; continue;
 }
 0x20a9577 => {
  cpu.push(0x20a957c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a957c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9587; continue; }
  pc = 0x20a9580; continue;
 }
 0x20a9580 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9589; continue;
 }
 0x20a9587 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9589; continue;
 }
 0x20a9589 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a95d7; continue; }
  pc = 0x20a958d; continue;
 }
 0x20a958d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a1e0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a95cf); cpu.call(t); if cpu.returned_to != 0x20a95cf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a95d7); f_020adba0(cpu); if cpu.returned_to != 0x20a95d7 { return; }
  pc = 0x20a95d7; continue;
 }
 0x20a95d7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a95f4); f_020b84a0(cpu); if cpu.returned_to != 0x20a95f4 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20a9642; continue; }
  pc = 0x20a9603; continue;
 }
 0x20a9603 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a9615; continue; }
  pc = 0x20a960c; continue;
 }
 0x20a960c => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9642; continue; }
  pc = 0x20a9615; continue;
 }
 0x20a9615 => {
  cpu.push(0x20a961a); f_020b4041(cpu); if cpu.returned_to != 0x20a961a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9642; continue; }
  pc = 0x20a961e; continue;
 }
 0x20a961e => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9638); f_020b42d3(cpu); if cpu.returned_to != 0x20a9638 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a9642); f_020adb50(cpu); if cpu.returned_to != 0x20a9642 { return; }
  pc = 0x20a9642; continue;
 }
 0x20a9642 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a9660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9660); }
 let mut pc: u32 = 0x20a9660;
 loop { match pc {
 0x20a9660 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a96a7; continue; }
  pc = 0x20a9680; continue;
 }
 0x20a9680 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a968e; continue; }
  pc = 0x20a9685; continue;
 }
 0x20a9685 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a96a7; continue; }
  pc = 0x20a968e; continue;
 }
 0x20a968e => {
  cpu.push(0x20a9693); f_020b4041(cpu); if cpu.returned_to != 0x20a9693 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a96a7; continue; }
  pc = 0x20a9697; continue;
 }
 0x20a9697 => {
  cpu.push(0x20a969c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a969c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a96a7; continue; }
  pc = 0x20a96a0; continue;
 }
 0x20a96a0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a96a9; continue;
 }
 0x20a96a7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a96a9; continue;
 }
 0x20a96a9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a96db; continue; }
  pc = 0x20a96ad; continue;
 }
 0x20a96ad => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a1bcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a96d3); cpu.call(t); if cpu.returned_to != 0x20a96d3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a96db); f_020adba0(cpu); if cpu.returned_to != 0x20a96db { return; }
  pc = 0x20a96db; continue;
 }
 0x20a96db => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a96e8); f_020b88f0(cpu); if cpu.returned_to != 0x20a96e8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020a9710(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9710); }
 let mut pc: u32 = 0x20a9710;
 loop { match pc {
 0x20a9710 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9758; continue; }
  pc = 0x20a9731; continue;
 }
 0x20a9731 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a973f; continue; }
  pc = 0x20a9736; continue;
 }
 0x20a9736 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9758; continue; }
  pc = 0x20a973f; continue;
 }
 0x20a973f => {
  cpu.push(0x20a9744); f_020b4041(cpu); if cpu.returned_to != 0x20a9744 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9758; continue; }
  pc = 0x20a9748; continue;
 }
 0x20a9748 => {
  cpu.push(0x20a974d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a974d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9758; continue; }
  pc = 0x20a9751; continue;
 }
 0x20a9751 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a975a; continue;
 }
 0x20a9758 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a975a; continue;
 }
 0x20a975a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a978c; continue; }
  pc = 0x20a975e; continue;
 }
 0x20a975e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a194u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9784); cpu.call(t); if cpu.returned_to != 0x20a9784 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a978c); f_020adba0(cpu); if cpu.returned_to != 0x20a978c { return; }
  pc = 0x20a978c; continue;
 }
 0x20a978c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9799); f_020b8910(cpu); if cpu.returned_to != 0x20a9799 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a97e7; continue; }
  pc = 0x20a97a8; continue;
 }
 0x20a97a8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a97ba; continue; }
  pc = 0x20a97b1; continue;
 }
 0x20a97b1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a97e7; continue; }
  pc = 0x20a97ba; continue;
 }
 0x20a97ba => {
  cpu.push(0x20a97bf); f_020b4041(cpu); if cpu.returned_to != 0x20a97bf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a97e7; continue; }
  pc = 0x20a97c3; continue;
 }
 0x20a97c3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a97dd); f_020b42d3(cpu); if cpu.returned_to != 0x20a97dd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a97e7); f_020adb00(cpu); if cpu.returned_to != 0x20a97e7 { return; }
  pc = 0x20a97e7; continue;
 }
 0x20a97e7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020a9810(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9810); }
 let mut pc: u32 = 0x20a9810;
 loop { match pc {
 0x20a9810 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9857; continue; }
  pc = 0x20a9830; continue;
 }
 0x20a9830 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a983e; continue; }
  pc = 0x20a9835; continue;
 }
 0x20a9835 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9857; continue; }
  pc = 0x20a983e; continue;
 }
 0x20a983e => {
  cpu.push(0x20a9843); f_020b4041(cpu); if cpu.returned_to != 0x20a9843 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9857; continue; }
  pc = 0x20a9847; continue;
 }
 0x20a9847 => {
  cpu.push(0x20a984c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a984c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9857; continue; }
  pc = 0x20a9850; continue;
 }
 0x20a9850 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9859; continue;
 }
 0x20a9857 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9859; continue;
 }
 0x20a9859 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a988b; continue; }
  pc = 0x20a985d; continue;
 }
 0x20a985d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a170u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9883); cpu.call(t); if cpu.returned_to != 0x20a9883 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a988b); f_020adba0(cpu); if cpu.returned_to != 0x20a988b { return; }
  pc = 0x20a988b; continue;
 }
 0x20a988b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9898); f_020b8b80(cpu); if cpu.returned_to != 0x20a9898 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a98c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a98c0); }
 let mut pc: u32 = 0x20a98c0;
 loop { match pc {
 0x20a98c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9908; continue; }
  pc = 0x20a98e1; continue;
 }
 0x20a98e1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a98ef; continue; }
  pc = 0x20a98e6; continue;
 }
 0x20a98e6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9908; continue; }
  pc = 0x20a98ef; continue;
 }
 0x20a98ef => {
  cpu.push(0x20a98f4); f_020b4041(cpu); if cpu.returned_to != 0x20a98f4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9908; continue; }
  pc = 0x20a98f8; continue;
 }
 0x20a98f8 => {
  cpu.push(0x20a98fd); f_020ad7d0(cpu); if cpu.returned_to != 0x20a98fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9908; continue; }
  pc = 0x20a9901; continue;
 }
 0x20a9901 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a990a; continue;
 }
 0x20a9908 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a990a; continue;
 }
 0x20a990a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a994a; continue; }
  pc = 0x20a990e; continue;
 }
 0x20a990e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a148u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9942); cpu.call(t); if cpu.returned_to != 0x20a9942 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a994a); f_020adba0(cpu); if cpu.returned_to != 0x20a994a { return; }
  pc = 0x20a994a; continue;
 }
 0x20a994a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a995f); f_020b8c60(cpu); if cpu.returned_to != 0x20a995f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a99ac; continue; }
  pc = 0x20a996d; continue;
 }
 0x20a996d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a997f; continue; }
  pc = 0x20a9976; continue;
 }
 0x20a9976 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a99ac; continue; }
  pc = 0x20a997f; continue;
 }
 0x20a997f => {
  cpu.push(0x20a9984); f_020b4041(cpu); if cpu.returned_to != 0x20a9984 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a99ac; continue; }
  pc = 0x20a9988; continue;
 }
 0x20a9988 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a99a2); f_020b42d3(cpu); if cpu.returned_to != 0x20a99a2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a99ac); f_020adb00(cpu); if cpu.returned_to != 0x20a99ac { return; }
  pc = 0x20a99ac; continue;
 }
 0x20a99ac => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020a99d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a99d0); }
 let mut pc: u32 = 0x20a99d0;
 loop { match pc {
 0x20a99d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9a18; continue; }
  pc = 0x20a99f1; continue;
 }
 0x20a99f1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a99ff; continue; }
  pc = 0x20a99f6; continue;
 }
 0x20a99f6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9a18; continue; }
  pc = 0x20a99ff; continue;
 }
 0x20a99ff => {
  cpu.push(0x20a9a04); f_020b4041(cpu); if cpu.returned_to != 0x20a9a04 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9a18; continue; }
  pc = 0x20a9a08; continue;
 }
 0x20a9a08 => {
  cpu.push(0x20a9a0d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a9a0d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9a18; continue; }
  pc = 0x20a9a11; continue;
 }
 0x20a9a11 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9a1a; continue;
 }
 0x20a9a18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9a1a; continue;
 }
 0x20a9a1a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9a4c; continue; }
  pc = 0x20a9a1e; continue;
 }
 0x20a9a1e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a12cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9a44); cpu.call(t); if cpu.returned_to != 0x20a9a44 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9a4c); f_020adba0(cpu); if cpu.returned_to != 0x20a9a4c { return; }
  pc = 0x20a9a4c; continue;
 }
 0x20a9a4c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9a59); f_020b8ba0(cpu); if cpu.returned_to != 0x20a9a59 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a9aa7; continue; }
  pc = 0x20a9a68; continue;
 }
 0x20a9a68 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a9a7a; continue; }
  pc = 0x20a9a71; continue;
 }
 0x20a9a71 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9aa7; continue; }
  pc = 0x20a9a7a; continue;
 }
 0x20a9a7a => {
  cpu.push(0x20a9a7f); f_020b4041(cpu); if cpu.returned_to != 0x20a9a7f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9aa7; continue; }
  pc = 0x20a9a83; continue;
 }
 0x20a9a83 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a9a9d); f_020b42d3(cpu); if cpu.returned_to != 0x20a9a9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a9aa7); f_020adb50(cpu); if cpu.returned_to != 0x20a9aa7 { return; }
  pc = 0x20a9aa7; continue;
 }
 0x20a9aa7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a9ad0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9ad0); }
 let mut pc: u32 = 0x20a9ad0;
 loop { match pc {
 0x20a9ad0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9b18; continue; }
  pc = 0x20a9af1; continue;
 }
 0x20a9af1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a9aff; continue; }
  pc = 0x20a9af6; continue;
 }
 0x20a9af6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9b18; continue; }
  pc = 0x20a9aff; continue;
 }
 0x20a9aff => {
  cpu.push(0x20a9b04); f_020b4041(cpu); if cpu.returned_to != 0x20a9b04 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9b18; continue; }
  pc = 0x20a9b08; continue;
 }
 0x20a9b08 => {
  cpu.push(0x20a9b0d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a9b0d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9b18; continue; }
  pc = 0x20a9b11; continue;
 }
 0x20a9b11 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9b1a; continue;
 }
 0x20a9b18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9b1a; continue;
 }
 0x20a9b1a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9b4c; continue; }
  pc = 0x20a9b1e; continue;
 }
 0x20a9b1e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a10cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9b44); cpu.call(t); if cpu.returned_to != 0x20a9b44 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9b4c); f_020adba0(cpu); if cpu.returned_to != 0x20a9b4c { return; }
  pc = 0x20a9b4c; continue;
 }
 0x20a9b4c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20a9b59); f_020b5960(cpu); if cpu.returned_to != 0x20a9b59 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20a9ba7; continue; }
  pc = 0x20a9b68; continue;
 }
 0x20a9b68 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a9b7a; continue; }
  pc = 0x20a9b71; continue;
 }
 0x20a9b71 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9ba7; continue; }
  pc = 0x20a9b7a; continue;
 }
 0x20a9b7a => {
  cpu.push(0x20a9b7f); f_020b4041(cpu); if cpu.returned_to != 0x20a9b7f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9ba7; continue; }
  pc = 0x20a9b83; continue;
 }
 0x20a9b83 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20a9b9d); f_020b42d3(cpu); if cpu.returned_to != 0x20a9b9d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a9ba7); f_020adb50(cpu); if cpu.returned_to != 0x20a9ba7 { return; }
  pc = 0x20a9ba7; continue;
 }
 0x20a9ba7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a9bd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9bd0); }
 let mut pc: u32 = 0x20a9bd0;
 loop { match pc {
 0x20a9bd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9c18; continue; }
  pc = 0x20a9bf1; continue;
 }
 0x20a9bf1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a9bff; continue; }
  pc = 0x20a9bf6; continue;
 }
 0x20a9bf6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9c18; continue; }
  pc = 0x20a9bff; continue;
 }
 0x20a9bff => {
  cpu.push(0x20a9c04); f_020b4041(cpu); if cpu.returned_to != 0x20a9c04 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9c18; continue; }
  pc = 0x20a9c08; continue;
 }
 0x20a9c08 => {
  cpu.push(0x20a9c0d); f_020ad7d0(cpu); if cpu.returned_to != 0x20a9c0d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9c18; continue; }
  pc = 0x20a9c11; continue;
 }
 0x20a9c11 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9c1a; continue;
 }
 0x20a9c18 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9c1a; continue;
 }
 0x20a9c1a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9c5a; continue; }
  pc = 0x20a9c1e; continue;
 }
 0x20a9c1e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a0ecu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9c52); cpu.call(t); if cpu.returned_to != 0x20a9c52 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9c5a); f_020adba0(cpu); if cpu.returned_to != 0x20a9c5a { return; }
  pc = 0x20a9c5a; continue;
 }
 0x20a9c5a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a9c6f); f_020b5460(cpu); if cpu.returned_to != 0x20a9c6f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9cbc; continue; }
  pc = 0x20a9c7d; continue;
 }
 0x20a9c7d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a9c8f; continue; }
  pc = 0x20a9c86; continue;
 }
 0x20a9c86 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9cbc; continue; }
  pc = 0x20a9c8f; continue;
 }
 0x20a9c8f => {
  cpu.push(0x20a9c94); f_020b4041(cpu); if cpu.returned_to != 0x20a9c94 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9cbc; continue; }
  pc = 0x20a9c98; continue;
 }
 0x20a9c98 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a9cb2); f_020b42d3(cpu); if cpu.returned_to != 0x20a9cb2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a9cbc); f_020adb50(cpu); if cpu.returned_to != 0x20a9cbc { return; }
  pc = 0x20a9cbc; continue;
 }
 0x20a9cbc => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020a9ce0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9ce0); }
 let mut pc: u32 = 0x20a9ce0;
 loop { match pc {
 0x20a9ce0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9d27; continue; }
  pc = 0x20a9d00; continue;
 }
 0x20a9d00 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a9d0e; continue; }
  pc = 0x20a9d05; continue;
 }
 0x20a9d05 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9d27; continue; }
  pc = 0x20a9d0e; continue;
 }
 0x20a9d0e => {
  cpu.push(0x20a9d13); f_020b4041(cpu); if cpu.returned_to != 0x20a9d13 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9d27; continue; }
  pc = 0x20a9d17; continue;
 }
 0x20a9d17 => {
  cpu.push(0x20a9d1c); f_020ad7d0(cpu); if cpu.returned_to != 0x20a9d1c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9d27; continue; }
  pc = 0x20a9d20; continue;
 }
 0x20a9d20 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9d29; continue;
 }
 0x20a9d27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9d29; continue;
 }
 0x20a9d29 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9d77; continue; }
  pc = 0x20a9d2d; continue;
 }
 0x20a9d2d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a0c4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9d6f); cpu.call(t); if cpu.returned_to != 0x20a9d6f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9d77); f_020adba0(cpu); if cpu.returned_to != 0x20a9d77 { return; }
  pc = 0x20a9d77; continue;
 }
 0x20a9d77 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9d94); f_020b5480(cpu); if cpu.returned_to != 0x20a9d94 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020a9ec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20a9ec0); }
 let mut pc: u32 = 0x20a9ec0;
 loop { match pc {
 0x20a9ec0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20a9f08; continue; }
  pc = 0x20a9ee1; continue;
 }
 0x20a9ee1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20a9eef; continue; }
  pc = 0x20a9ee6; continue;
 }
 0x20a9ee6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9f08; continue; }
  pc = 0x20a9eef; continue;
 }
 0x20a9eef => {
  cpu.push(0x20a9ef4); f_020b4041(cpu); if cpu.returned_to != 0x20a9ef4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9f08; continue; }
  pc = 0x20a9ef8; continue;
 }
 0x20a9ef8 => {
  cpu.push(0x20a9efd); f_020ad7d0(cpu); if cpu.returned_to != 0x20a9efd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9f08; continue; }
  pc = 0x20a9f01; continue;
 }
 0x20a9f01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20a9f0a; continue;
 }
 0x20a9f08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20a9f0a; continue;
 }
 0x20a9f0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9f4a; continue; }
  pc = 0x20a9f0e; continue;
 }
 0x20a9f0e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216a074u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20a9f42); cpu.call(t); if cpu.returned_to != 0x20a9f42 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20a9f4a); f_020adba0(cpu); if cpu.returned_to != 0x20a9f4a { return; }
  pc = 0x20a9f4a; continue;
 }
 0x20a9f4a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20a9f5f); f_020b54a0(cpu); if cpu.returned_to != 0x20a9f5f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20a9fac; continue; }
  pc = 0x20a9f6d; continue;
 }
 0x20a9f6d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20a9f7f; continue; }
  pc = 0x20a9f76; continue;
 }
 0x20a9f76 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20a9fac; continue; }
  pc = 0x20a9f7f; continue;
 }
 0x20a9f7f => {
  cpu.push(0x20a9f84); f_020b4041(cpu); if cpu.returned_to != 0x20a9f84 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20a9fac; continue; }
  pc = 0x20a9f88; continue;
 }
 0x20a9f88 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20a9fa2); f_020b42d3(cpu); if cpu.returned_to != 0x20a9fa2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20a9fac); f_020adb00(cpu); if cpu.returned_to != 0x20a9fac { return; }
  pc = 0x20a9fac; continue;
 }
 0x20a9fac => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020aab80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aab80); }
 let mut pc: u32 = 0x20aab80;
 loop { match pc {
 0x20aab80 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20aabc7; continue; }
  pc = 0x20aaba0; continue;
 }
 0x20aaba0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20aabae; continue; }
  pc = 0x20aaba5; continue;
 }
 0x20aaba5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20aabc7; continue; }
  pc = 0x20aabae; continue;
 }
 0x20aabae => {
  cpu.push(0x20aabb3); f_020b4041(cpu); if cpu.returned_to != 0x20aabb3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20aabc7; continue; }
  pc = 0x20aabb7; continue;
 }
 0x20aabb7 => {
  cpu.push(0x20aabbc); f_020ad7d0(cpu); if cpu.returned_to != 0x20aabbc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aabc7; continue; }
  pc = 0x20aabc0; continue;
 }
 0x20aabc0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20aabc9; continue;
 }
 0x20aabc7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20aabc9; continue;
 }
 0x20aabc9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aac09; continue; }
  pc = 0x20aabcd; continue;
 }
 0x20aabcd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169e64u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20aac01); cpu.call(t); if cpu.returned_to != 0x20aac01 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20aac09); f_020adba0(cpu); if cpu.returned_to != 0x20aac09 { return; }
  pc = 0x20aac09; continue;
 }
 0x20aac09 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20aac1e); f_020b5420(cpu); if cpu.returned_to != 0x20aac1e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aac40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aac40); }
 let mut pc: u32 = 0x20aac40;
 loop { match pc {
 0x20aac40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20aac88; continue; }
  pc = 0x20aac61; continue;
 }
 0x20aac61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20aac6f; continue; }
  pc = 0x20aac66; continue;
 }
 0x20aac66 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20aac88; continue; }
  pc = 0x20aac6f; continue;
 }
 0x20aac6f => {
  cpu.push(0x20aac74); f_020b4041(cpu); if cpu.returned_to != 0x20aac74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20aac88; continue; }
  pc = 0x20aac78; continue;
 }
 0x20aac78 => {
  cpu.push(0x20aac7d); f_020ad7d0(cpu); if cpu.returned_to != 0x20aac7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aac88; continue; }
  pc = 0x20aac81; continue;
 }
 0x20aac81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20aac8a; continue;
 }
 0x20aac88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20aac8a; continue;
 }
 0x20aac8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aacbc; continue; }
  pc = 0x20aac8e; continue;
 }
 0x20aac8e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169e44u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20aacb4); cpu.call(t); if cpu.returned_to != 0x20aacb4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20aacbc); f_020adba0(cpu); if cpu.returned_to != 0x20aacbc { return; }
  pc = 0x20aacbc; continue;
 }
 0x20aacbc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20aacc9); f_020b5690(cpu); if cpu.returned_to != 0x20aacc9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20aad17; continue; }
  pc = 0x20aacd8; continue;
 }
 0x20aacd8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20aacea; continue; }
  pc = 0x20aace1; continue;
 }
 0x20aace1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20aad17; continue; }
  pc = 0x20aacea; continue;
 }
 0x20aacea => {
  cpu.push(0x20aacef); f_020b4041(cpu); if cpu.returned_to != 0x20aacef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20aad17; continue; }
  pc = 0x20aacf3; continue;
 }
 0x20aacf3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20aad0d); f_020b42d3(cpu); if cpu.returned_to != 0x20aad0d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20aad17); f_020adb50(cpu); if cpu.returned_to != 0x20aad17 { return; }
  pc = 0x20aad17; continue;
 }
 0x20aad17 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020ab040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab040); }
 let mut pc: u32 = 0x20ab040;
 loop { match pc {
 0x20ab040 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab088; continue; }
  pc = 0x20ab061; continue;
 }
 0x20ab061 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab06f; continue; }
  pc = 0x20ab066; continue;
 }
 0x20ab066 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab088; continue; }
  pc = 0x20ab06f; continue;
 }
 0x20ab06f => {
  cpu.push(0x20ab074); f_020b4041(cpu); if cpu.returned_to != 0x20ab074 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab088; continue; }
  pc = 0x20ab078; continue;
 }
 0x20ab078 => {
  cpu.push(0x20ab07d); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab07d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab088; continue; }
  pc = 0x20ab081; continue;
 }
 0x20ab081 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab08a; continue;
 }
 0x20ab088 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab08a; continue;
 }
 0x20ab08a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab0bc; continue; }
  pc = 0x20ab08e; continue;
 }
 0x20ab08e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169dd0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab0b4); cpu.call(t); if cpu.returned_to != 0x20ab0b4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab0bc); f_020adba0(cpu); if cpu.returned_to != 0x20ab0bc { return; }
  pc = 0x20ab0bc; continue;
 }
 0x20ab0bc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ab0c9); f_020b57d0(cpu); if cpu.returned_to != 0x20ab0c9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20ab117; continue; }
  pc = 0x20ab0d8; continue;
 }
 0x20ab0d8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ab0ea; continue; }
  pc = 0x20ab0e1; continue;
 }
 0x20ab0e1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab117; continue; }
  pc = 0x20ab0ea; continue;
 }
 0x20ab0ea => {
  cpu.push(0x20ab0ef); f_020b4041(cpu); if cpu.returned_to != 0x20ab0ef { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab117; continue; }
  pc = 0x20ab0f3; continue;
 }
 0x20ab0f3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ab10d); f_020b42d3(cpu); if cpu.returned_to != 0x20ab10d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ab117); f_020adb50(cpu); if cpu.returned_to != 0x20ab117 { return; }
  pc = 0x20ab117; continue;
 }
 0x20ab117 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ab140(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab140); }
 let mut pc: u32 = 0x20ab140;
 loop { match pc {
 0x20ab140 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab187; continue; }
  pc = 0x20ab160; continue;
 }
 0x20ab160 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab16e; continue; }
  pc = 0x20ab165; continue;
 }
 0x20ab165 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab187; continue; }
  pc = 0x20ab16e; continue;
 }
 0x20ab16e => {
  cpu.push(0x20ab173); f_020b4041(cpu); if cpu.returned_to != 0x20ab173 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab187; continue; }
  pc = 0x20ab177; continue;
 }
 0x20ab177 => {
  cpu.push(0x20ab17c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab17c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab187; continue; }
  pc = 0x20ab180; continue;
 }
 0x20ab180 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab189; continue;
 }
 0x20ab187 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab189; continue;
 }
 0x20ab189 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab1d7; continue; }
  pc = 0x20ab18d; continue;
 }
 0x20ab18d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169da4u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab1cf); cpu.call(t); if cpu.returned_to != 0x20ab1cf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab1d7); f_020adba0(cpu); if cpu.returned_to != 0x20ab1d7 { return; }
  pc = 0x20ab1d7; continue;
 }
 0x20ab1d7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab1f4); f_020b5660(cpu); if cpu.returned_to != 0x20ab1f4 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab210(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab210); }
 let mut pc: u32 = 0x20ab210;
 loop { match pc {
 0x20ab210 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab257; continue; }
  pc = 0x20ab230; continue;
 }
 0x20ab230 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab23e; continue; }
  pc = 0x20ab235; continue;
 }
 0x20ab235 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab257; continue; }
  pc = 0x20ab23e; continue;
 }
 0x20ab23e => {
  cpu.push(0x20ab243); f_020b4041(cpu); if cpu.returned_to != 0x20ab243 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab257; continue; }
  pc = 0x20ab247; continue;
 }
 0x20ab247 => {
  cpu.push(0x20ab24c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab24c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab257; continue; }
  pc = 0x20ab250; continue;
 }
 0x20ab250 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab259; continue;
 }
 0x20ab257 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab259; continue;
 }
 0x20ab259 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab299; continue; }
  pc = 0x20ab25d; continue;
 }
 0x20ab25d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169d7cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab291); cpu.call(t); if cpu.returned_to != 0x20ab291 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab299); f_020adba0(cpu); if cpu.returned_to != 0x20ab299 { return; }
  pc = 0x20ab299; continue;
 }
 0x20ab299 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ab2ae); f_020b5680(cpu); if cpu.returned_to != 0x20ab2ae { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab2d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab2d0); }
 let mut pc: u32 = 0x20ab2d0;
 loop { match pc {
 0x20ab2d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab317; continue; }
  pc = 0x20ab2f0; continue;
 }
 0x20ab2f0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab2fe; continue; }
  pc = 0x20ab2f5; continue;
 }
 0x20ab2f5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab317; continue; }
  pc = 0x20ab2fe; continue;
 }
 0x20ab2fe => {
  cpu.push(0x20ab303); f_020b4041(cpu); if cpu.returned_to != 0x20ab303 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab317; continue; }
  pc = 0x20ab307; continue;
 }
 0x20ab307 => {
  cpu.push(0x20ab30c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab30c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab317; continue; }
  pc = 0x20ab310; continue;
 }
 0x20ab310 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab319; continue;
 }
 0x20ab317 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab319; continue;
 }
 0x20ab319 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab359; continue; }
  pc = 0x20ab31d; continue;
 }
 0x20ab31d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169d5cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab351); cpu.call(t); if cpu.returned_to != 0x20ab351 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab359); f_020adba0(cpu); if cpu.returned_to != 0x20ab359 { return; }
  pc = 0x20ab359; continue;
 }
 0x20ab359 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ab36e); f_020b56f0(cpu); if cpu.returned_to != 0x20ab36e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab390); }
 let mut pc: u32 = 0x20ab390;
 loop { match pc {
 0x20ab390 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab3d7; continue; }
  pc = 0x20ab3b0; continue;
 }
 0x20ab3b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab3be; continue; }
  pc = 0x20ab3b5; continue;
 }
 0x20ab3b5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab3d7; continue; }
  pc = 0x20ab3be; continue;
 }
 0x20ab3be => {
  cpu.push(0x20ab3c3); f_020b4041(cpu); if cpu.returned_to != 0x20ab3c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab3d7; continue; }
  pc = 0x20ab3c7; continue;
 }
 0x20ab3c7 => {
  cpu.push(0x20ab3cc); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab3cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab3d7; continue; }
  pc = 0x20ab3d0; continue;
 }
 0x20ab3d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab3d9; continue;
 }
 0x20ab3d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab3d9; continue;
 }
 0x20ab3d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab419; continue; }
  pc = 0x20ab3dd; continue;
 }
 0x20ab3dd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169d38u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab411); cpu.call(t); if cpu.returned_to != 0x20ab411 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab419); f_020adba0(cpu); if cpu.returned_to != 0x20ab419 { return; }
  pc = 0x20ab419; continue;
 }
 0x20ab419 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ab42e); f_020b5720(cpu); if cpu.returned_to != 0x20ab42e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab450); }
 let mut pc: u32 = 0x20ab450;
 loop { match pc {
 0x20ab450 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab497; continue; }
  pc = 0x20ab470; continue;
 }
 0x20ab470 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab47e; continue; }
  pc = 0x20ab475; continue;
 }
 0x20ab475 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab497; continue; }
  pc = 0x20ab47e; continue;
 }
 0x20ab47e => {
  cpu.push(0x20ab483); f_020b4041(cpu); if cpu.returned_to != 0x20ab483 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab497; continue; }
  pc = 0x20ab487; continue;
 }
 0x20ab487 => {
  cpu.push(0x20ab48c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab48c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab497; continue; }
  pc = 0x20ab490; continue;
 }
 0x20ab490 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab499; continue;
 }
 0x20ab497 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab499; continue;
 }
 0x20ab499 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab4d9; continue; }
  pc = 0x20ab49d; continue;
 }
 0x20ab49d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169d10u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab4d1); cpu.call(t); if cpu.returned_to != 0x20ab4d1 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab4d9); f_020adba0(cpu); if cpu.returned_to != 0x20ab4d9 { return; }
  pc = 0x20ab4d9; continue;
 }
 0x20ab4d9 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ab4ee); f_020b5750(cpu); if cpu.returned_to != 0x20ab4ee { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab510); }
 let mut pc: u32 = 0x20ab510;
 loop { match pc {
 0x20ab510 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab557; continue; }
  pc = 0x20ab530; continue;
 }
 0x20ab530 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab53e; continue; }
  pc = 0x20ab535; continue;
 }
 0x20ab535 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab557; continue; }
  pc = 0x20ab53e; continue;
 }
 0x20ab53e => {
  cpu.push(0x20ab543); f_020b4041(cpu); if cpu.returned_to != 0x20ab543 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab557; continue; }
  pc = 0x20ab547; continue;
 }
 0x20ab547 => {
  cpu.push(0x20ab54c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab54c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab557; continue; }
  pc = 0x20ab550; continue;
 }
 0x20ab550 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab559; continue;
 }
 0x20ab557 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab559; continue;
 }
 0x20ab559 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab58b; continue; }
  pc = 0x20ab55d; continue;
 }
 0x20ab55d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169cf8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab583); cpu.call(t); if cpu.returned_to != 0x20ab583 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab58b); f_020adba0(cpu); if cpu.returned_to != 0x20ab58b { return; }
  pc = 0x20ab58b; continue;
 }
 0x20ab58b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ab598); f_020b5500(cpu); if cpu.returned_to != 0x20ab598 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ab5c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab5c0); }
 let mut pc: u32 = 0x20ab5c0;
 loop { match pc {
 0x20ab5c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab607; continue; }
  pc = 0x20ab5e0; continue;
 }
 0x20ab5e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab5ee; continue; }
  pc = 0x20ab5e5; continue;
 }
 0x20ab5e5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab607; continue; }
  pc = 0x20ab5ee; continue;
 }
 0x20ab5ee => {
  cpu.push(0x20ab5f3); f_020b4041(cpu); if cpu.returned_to != 0x20ab5f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab607; continue; }
  pc = 0x20ab5f7; continue;
 }
 0x20ab5f7 => {
  cpu.push(0x20ab5fc); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab5fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab607; continue; }
  pc = 0x20ab600; continue;
 }
 0x20ab600 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab609; continue;
 }
 0x20ab607 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab609; continue;
 }
 0x20ab609 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab63b; continue; }
  pc = 0x20ab60d; continue;
 }
 0x20ab60d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169cdcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab633); cpu.call(t); if cpu.returned_to != 0x20ab633 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab63b); f_020adba0(cpu); if cpu.returned_to != 0x20ab63b { return; }
  pc = 0x20ab63b; continue;
 }
 0x20ab63b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ab648); f_020b5550(cpu); if cpu.returned_to != 0x20ab648 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab720); }
 let mut pc: u32 = 0x20ab720;
 loop { match pc {
 0x20ab720 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab767; continue; }
  pc = 0x20ab740; continue;
 }
 0x20ab740 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab74e; continue; }
  pc = 0x20ab745; continue;
 }
 0x20ab745 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab767; continue; }
  pc = 0x20ab74e; continue;
 }
 0x20ab74e => {
  cpu.push(0x20ab753); f_020b4041(cpu); if cpu.returned_to != 0x20ab753 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab767; continue; }
  pc = 0x20ab757; continue;
 }
 0x20ab757 => {
  cpu.push(0x20ab75c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab75c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab767; continue; }
  pc = 0x20ab760; continue;
 }
 0x20ab760 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab769; continue;
 }
 0x20ab767 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab769; continue;
 }
 0x20ab769 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab79b; continue; }
  pc = 0x20ab76d; continue;
 }
 0x20ab76d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169ca8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab793); cpu.call(t); if cpu.returned_to != 0x20ab793 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab79b); f_020adba0(cpu); if cpu.returned_to != 0x20ab79b { return; }
  pc = 0x20ab79b; continue;
 }
 0x20ab79b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ab7a8); f_020b55d0(cpu); if cpu.returned_to != 0x20ab7a8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ab7d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab7d0); }
 let mut pc: u32 = 0x20ab7d0;
 loop { match pc {
 0x20ab7d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab817; continue; }
  pc = 0x20ab7f0; continue;
 }
 0x20ab7f0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab7fe; continue; }
  pc = 0x20ab7f5; continue;
 }
 0x20ab7f5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab817; continue; }
  pc = 0x20ab7fe; continue;
 }
 0x20ab7fe => {
  cpu.push(0x20ab803); f_020b4041(cpu); if cpu.returned_to != 0x20ab803 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab817; continue; }
  pc = 0x20ab807; continue;
 }
 0x20ab807 => {
  cpu.push(0x20ab80c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab80c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab817; continue; }
  pc = 0x20ab810; continue;
 }
 0x20ab810 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab819; continue;
 }
 0x20ab817 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab819; continue;
 }
 0x20ab819 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab867; continue; }
  pc = 0x20ab81d; continue;
 }
 0x20ab81d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169c84u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab85f); cpu.call(t); if cpu.returned_to != 0x20ab85f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab867); f_020adba0(cpu); if cpu.returned_to != 0x20ab867 { return; }
  pc = 0x20ab867; continue;
 }
 0x20ab867 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab884); f_020b5770(cpu); if cpu.returned_to != 0x20ab884 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab8a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab8a0); }
 let mut pc: u32 = 0x20ab8a0;
 loop { match pc {
 0x20ab8a0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab8e7; continue; }
  pc = 0x20ab8c0; continue;
 }
 0x20ab8c0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab8ce; continue; }
  pc = 0x20ab8c5; continue;
 }
 0x20ab8c5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab8e7; continue; }
  pc = 0x20ab8ce; continue;
 }
 0x20ab8ce => {
  cpu.push(0x20ab8d3); f_020b4041(cpu); if cpu.returned_to != 0x20ab8d3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab8e7; continue; }
  pc = 0x20ab8d7; continue;
 }
 0x20ab8d7 => {
  cpu.push(0x20ab8dc); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab8dc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab8e7; continue; }
  pc = 0x20ab8e0; continue;
 }
 0x20ab8e0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab8e9; continue;
 }
 0x20ab8e7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab8e9; continue;
 }
 0x20ab8e9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab937; continue; }
  pc = 0x20ab8ed; continue;
 }
 0x20ab8ed => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169c5cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab92f); cpu.call(t); if cpu.returned_to != 0x20ab92f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab937); f_020adba0(cpu); if cpu.returned_to != 0x20ab937 { return; }
  pc = 0x20ab937; continue;
 }
 0x20ab937 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ab954); f_020b57a0(cpu); if cpu.returned_to != 0x20ab954 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020ab970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ab970); }
 let mut pc: u32 = 0x20ab970;
 loop { match pc {
 0x20ab970 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ab9b7; continue; }
  pc = 0x20ab990; continue;
 }
 0x20ab990 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ab99e; continue; }
  pc = 0x20ab995; continue;
 }
 0x20ab995 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ab9b7; continue; }
  pc = 0x20ab99e; continue;
 }
 0x20ab99e => {
  cpu.push(0x20ab9a3); f_020b4041(cpu); if cpu.returned_to != 0x20ab9a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ab9b7; continue; }
  pc = 0x20ab9a7; continue;
 }
 0x20ab9a7 => {
  cpu.push(0x20ab9ac); f_020ad7d0(cpu); if cpu.returned_to != 0x20ab9ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ab9b7; continue; }
  pc = 0x20ab9b0; continue;
 }
 0x20ab9b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ab9b9; continue;
 }
 0x20ab9b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ab9b9; continue;
 }
 0x20ab9b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aba07; continue; }
  pc = 0x20ab9bd; continue;
 }
 0x20ab9bd => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169c38u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ab9ff); cpu.call(t); if cpu.returned_to != 0x20ab9ff { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20aba07); f_020adba0(cpu); if cpu.returned_to != 0x20aba07 { return; }
  pc = 0x20aba07; continue;
 }
 0x20aba07 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20aba24); f_020b6f90(cpu); if cpu.returned_to != 0x20aba24 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20aba72; continue; }
  pc = 0x20aba33; continue;
 }
 0x20aba33 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20aba45; continue; }
  pc = 0x20aba3c; continue;
 }
 0x20aba3c => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20aba72; continue; }
  pc = 0x20aba45; continue;
 }
 0x20aba45 => {
  cpu.push(0x20aba4a); f_020b4041(cpu); if cpu.returned_to != 0x20aba4a { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20aba72; continue; }
  pc = 0x20aba4e; continue;
 }
 0x20aba4e => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20aba68); f_020b42d3(cpu); if cpu.returned_to != 0x20aba68 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20aba72); f_020adb50(cpu); if cpu.returned_to != 0x20aba72 { return; }
  pc = 0x20aba72; continue;
 }
 0x20aba72 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020aba90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aba90); }
 let mut pc: u32 = 0x20aba90;
 loop { match pc {
 0x20aba90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20abad7; continue; }
  pc = 0x20abab0; continue;
 }
 0x20abab0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ababe; continue; }
  pc = 0x20abab5; continue;
 }
 0x20abab5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abad7; continue; }
  pc = 0x20ababe; continue;
 }
 0x20ababe => {
  cpu.push(0x20abac3); f_020b4041(cpu); if cpu.returned_to != 0x20abac3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abad7; continue; }
  pc = 0x20abac7; continue;
 }
 0x20abac7 => {
  cpu.push(0x20abacc); f_020ad7d0(cpu); if cpu.returned_to != 0x20abacc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abad7; continue; }
  pc = 0x20abad0; continue;
 }
 0x20abad0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20abad9; continue;
 }
 0x20abad7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20abad9; continue;
 }
 0x20abad9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abb0b; continue; }
  pc = 0x20abadd; continue;
 }
 0x20abadd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169c20u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20abb03); cpu.call(t); if cpu.returned_to != 0x20abb03 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abb0b); f_020adba0(cpu); if cpu.returned_to != 0x20abb0b { return; }
  pc = 0x20abb0b; continue;
 }
 0x20abb0b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20abb18); f_020b57e0(cpu); if cpu.returned_to != 0x20abb18 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020abb40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20abb40); }
 let mut pc: u32 = 0x20abb40;
 loop { match pc {
 0x20abb40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20abb87; continue; }
  pc = 0x20abb60; continue;
 }
 0x20abb60 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20abb6e; continue; }
  pc = 0x20abb65; continue;
 }
 0x20abb65 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abb87; continue; }
  pc = 0x20abb6e; continue;
 }
 0x20abb6e => {
  cpu.push(0x20abb73); f_020b4041(cpu); if cpu.returned_to != 0x20abb73 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abb87; continue; }
  pc = 0x20abb77; continue;
 }
 0x20abb77 => {
  cpu.push(0x20abb7c); f_020ad7d0(cpu); if cpu.returned_to != 0x20abb7c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abb87; continue; }
  pc = 0x20abb80; continue;
 }
 0x20abb80 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20abb89; continue;
 }
 0x20abb87 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20abb89; continue;
 }
 0x20abb89 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abbbb; continue; }
  pc = 0x20abb8d; continue;
 }
 0x20abb8d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169bfcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20abbb3); cpu.call(t); if cpu.returned_to != 0x20abbb3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abbbb); f_020adba0(cpu); if cpu.returned_to != 0x20abbbb { return; }
  pc = 0x20abbbb; continue;
 }
 0x20abbbb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20abbc8); f_020b58d0(cpu); if cpu.returned_to != 0x20abbc8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020abbf0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20abbf0); }
 let mut pc: u32 = 0x20abbf0;
 loop { match pc {
 0x20abbf0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20abc37; continue; }
  pc = 0x20abc10; continue;
 }
 0x20abc10 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20abc1e; continue; }
  pc = 0x20abc15; continue;
 }
 0x20abc15 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abc37; continue; }
  pc = 0x20abc1e; continue;
 }
 0x20abc1e => {
  cpu.push(0x20abc23); f_020b4041(cpu); if cpu.returned_to != 0x20abc23 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abc37; continue; }
  pc = 0x20abc27; continue;
 }
 0x20abc27 => {
  cpu.push(0x20abc2c); f_020ad7d0(cpu); if cpu.returned_to != 0x20abc2c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abc37; continue; }
  pc = 0x20abc30; continue;
 }
 0x20abc30 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20abc39; continue;
 }
 0x20abc37 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20abc39; continue;
 }
 0x20abc39 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abc87; continue; }
  pc = 0x20abc3d; continue;
 }
 0x20abc3d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169bd0u32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20abc7f); cpu.call(t); if cpu.returned_to != 0x20abc7f { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abc87); f_020adba0(cpu); if cpu.returned_to != 0x20abc87 { return; }
  pc = 0x20abc87; continue;
 }
 0x20abc87 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abca4); f_020b70e0(cpu); if cpu.returned_to != 0x20abca4 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20abcf2; continue; }
  pc = 0x20abcb3; continue;
 }
 0x20abcb3 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20abcc5; continue; }
  pc = 0x20abcbc; continue;
 }
 0x20abcbc => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abcf2; continue; }
  pc = 0x20abcc5; continue;
 }
 0x20abcc5 => {
  cpu.push(0x20abcca); f_020b4041(cpu); if cpu.returned_to != 0x20abcca { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abcf2; continue; }
  pc = 0x20abcce; continue;
 }
 0x20abcce => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20abce8); f_020b42d3(cpu); if cpu.returned_to != 0x20abce8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20abcf2); f_020adb00(cpu); if cpu.returned_to != 0x20abcf2 { return; }
  pc = 0x20abcf2; continue;
 }
 0x20abcf2 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn o_020abd10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20abd10); }
 let mut pc: u32 = 0x20abd10;
 loop { match pc {
 0x20abd10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20abd58; continue; }
  pc = 0x20abd31; continue;
 }
 0x20abd31 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20abd3f; continue; }
  pc = 0x20abd36; continue;
 }
 0x20abd36 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abd58; continue; }
  pc = 0x20abd3f; continue;
 }
 0x20abd3f => {
  cpu.push(0x20abd44); f_020b4041(cpu); if cpu.returned_to != 0x20abd44 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abd58; continue; }
  pc = 0x20abd48; continue;
 }
 0x20abd48 => {
  cpu.push(0x20abd4d); f_020ad7d0(cpu); if cpu.returned_to != 0x20abd4d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abd58; continue; }
  pc = 0x20abd51; continue;
 }
 0x20abd51 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20abd5a; continue;
 }
 0x20abd58 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20abd5a; continue;
 }
 0x20abd5a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abd8c; continue; }
  pc = 0x20abd5e; continue;
 }
 0x20abd5e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169bacu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20abd84); cpu.call(t); if cpu.returned_to != 0x20abd84 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abd8c); f_020adba0(cpu); if cpu.returned_to != 0x20abd8c { return; }
  pc = 0x20abd8c; continue;
 }
 0x20abd8c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20abd99); f_020b58e0(cpu); if cpu.returned_to != 0x20abd99 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20abde7; continue; }
  pc = 0x20abda8; continue;
 }
 0x20abda8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20abdba; continue; }
  pc = 0x20abdb1; continue;
 }
 0x20abdb1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abde7; continue; }
  pc = 0x20abdba; continue;
 }
 0x20abdba => {
  cpu.push(0x20abdbf); f_020b4041(cpu); if cpu.returned_to != 0x20abdbf { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abde7; continue; }
  pc = 0x20abdc3; continue;
 }
 0x20abdc3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20abddd); f_020b42d3(cpu); if cpu.returned_to != 0x20abddd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20abde7); f_020adb00(cpu); if cpu.returned_to != 0x20abde7 { return; }
  pc = 0x20abde7; continue;
 }
 0x20abde7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020abe10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20abe10); }
 let mut pc: u32 = 0x20abe10;
 loop { match pc {
 0x20abe10 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20abe57; continue; }
  pc = 0x20abe30; continue;
 }
 0x20abe30 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20abe3e; continue; }
  pc = 0x20abe35; continue;
 }
 0x20abe35 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abe57; continue; }
  pc = 0x20abe3e; continue;
 }
 0x20abe3e => {
  cpu.push(0x20abe43); f_020b4041(cpu); if cpu.returned_to != 0x20abe43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abe57; continue; }
  pc = 0x20abe47; continue;
 }
 0x20abe47 => {
  cpu.push(0x20abe4c); f_020ad7d0(cpu); if cpu.returned_to != 0x20abe4c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abe57; continue; }
  pc = 0x20abe50; continue;
 }
 0x20abe50 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20abe59; continue;
 }
 0x20abe57 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20abe59; continue;
 }
 0x20abe59 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abe8b; continue; }
  pc = 0x20abe5d; continue;
 }
 0x20abe5d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169b88u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20abe83); cpu.call(t); if cpu.returned_to != 0x20abe83 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abe8b); f_020adba0(cpu); if cpu.returned_to != 0x20abe8b { return; }
  pc = 0x20abe8b; continue;
 }
 0x20abe8b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20abe98); f_020b5990(cpu); if cpu.returned_to != 0x20abe98 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020abec0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20abec0); }
 let mut pc: u32 = 0x20abec0;
 loop { match pc {
 0x20abec0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20abf08; continue; }
  pc = 0x20abee1; continue;
 }
 0x20abee1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20abeef; continue; }
  pc = 0x20abee6; continue;
 }
 0x20abee6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abf08; continue; }
  pc = 0x20abeef; continue;
 }
 0x20abeef => {
  cpu.push(0x20abef4); f_020b4041(cpu); if cpu.returned_to != 0x20abef4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abf08; continue; }
  pc = 0x20abef8; continue;
 }
 0x20abef8 => {
  cpu.push(0x20abefd); f_020ad7d0(cpu); if cpu.returned_to != 0x20abefd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abf08; continue; }
  pc = 0x20abf01; continue;
 }
 0x20abf01 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20abf0a; continue;
 }
 0x20abf08 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20abf0a; continue;
 }
 0x20abf0a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abf4a; continue; }
  pc = 0x20abf0e; continue;
 }
 0x20abf0e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169b60u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20abf42); cpu.call(t); if cpu.returned_to != 0x20abf42 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20abf4a); f_020adba0(cpu); if cpu.returned_to != 0x20abf4a { return; }
  pc = 0x20abf4a; continue;
 }
 0x20abf4a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20abf5f); f_020b5a70(cpu); if cpu.returned_to != 0x20abf5f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20abfac; continue; }
  pc = 0x20abf6d; continue;
 }
 0x20abf6d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20abf7f; continue; }
  pc = 0x20abf76; continue;
 }
 0x20abf76 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20abfac; continue; }
  pc = 0x20abf7f; continue;
 }
 0x20abf7f => {
  cpu.push(0x20abf84); f_020b4041(cpu); if cpu.returned_to != 0x20abf84 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20abfac; continue; }
  pc = 0x20abf88; continue;
 }
 0x20abf88 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20abfa2); f_020b42d3(cpu); if cpu.returned_to != 0x20abfa2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20abfac); f_020adb00(cpu); if cpu.returned_to != 0x20abfac { return; }
  pc = 0x20abfac; continue;
 }
 0x20abfac => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn o_020abfd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20abfd0); }
 let mut pc: u32 = 0x20abfd0;
 loop { match pc {
 0x20abfd0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac018; continue; }
  pc = 0x20abff1; continue;
 }
 0x20abff1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20abfff; continue; }
  pc = 0x20abff6; continue;
 }
 0x20abff6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac018; continue; }
  pc = 0x20abfff; continue;
 }
 0x20abfff => {
  cpu.push(0x20ac004); f_020b4041(cpu); if cpu.returned_to != 0x20ac004 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac018; continue; }
  pc = 0x20ac008; continue;
 }
 0x20ac008 => {
  cpu.push(0x20ac00d); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac00d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac018; continue; }
  pc = 0x20ac011; continue;
 }
 0x20ac011 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac01a; continue;
 }
 0x20ac018 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac01a; continue;
 }
 0x20ac01a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac04c; continue; }
  pc = 0x20ac01e; continue;
 }
 0x20ac01e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169b44u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac044); cpu.call(t); if cpu.returned_to != 0x20ac044 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac04c); f_020adba0(cpu); if cpu.returned_to != 0x20ac04c { return; }
  pc = 0x20ac04c; continue;
 }
 0x20ac04c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac059); f_020b59b0(cpu); if cpu.returned_to != 0x20ac059 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20ac0a7; continue; }
  pc = 0x20ac068; continue;
 }
 0x20ac068 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ac07a; continue; }
  pc = 0x20ac071; continue;
 }
 0x20ac071 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac0a7; continue; }
  pc = 0x20ac07a; continue;
 }
 0x20ac07a => {
  cpu.push(0x20ac07f); f_020b4041(cpu); if cpu.returned_to != 0x20ac07f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac0a7; continue; }
  pc = 0x20ac083; continue;
 }
 0x20ac083 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ac09d); f_020b42d3(cpu); if cpu.returned_to != 0x20ac09d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ac0a7); f_020adb50(cpu); if cpu.returned_to != 0x20ac0a7 { return; }
  pc = 0x20ac0a7; continue;
 }
 0x20ac0a7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ac0d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac0d0); }
 let mut pc: u32 = 0x20ac0d0;
 loop { match pc {
 0x20ac0d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac117; continue; }
  pc = 0x20ac0f0; continue;
 }
 0x20ac0f0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac0fe; continue; }
  pc = 0x20ac0f5; continue;
 }
 0x20ac0f5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac117; continue; }
  pc = 0x20ac0fe; continue;
 }
 0x20ac0fe => {
  cpu.push(0x20ac103); f_020b4041(cpu); if cpu.returned_to != 0x20ac103 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac117; continue; }
  pc = 0x20ac107; continue;
 }
 0x20ac107 => {
  cpu.push(0x20ac10c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac10c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac117; continue; }
  pc = 0x20ac110; continue;
 }
 0x20ac110 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac119; continue;
 }
 0x20ac117 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac119; continue;
 }
 0x20ac119 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac14b; continue; }
  pc = 0x20ac11d; continue;
 }
 0x20ac11d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169b24u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac143); cpu.call(t); if cpu.returned_to != 0x20ac143 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac14b); f_020adba0(cpu); if cpu.returned_to != 0x20ac14b { return; }
  pc = 0x20ac14b; continue;
 }
 0x20ac14b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac158); f_020b46a0(cpu); if cpu.returned_to != 0x20ac158 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac180); }
 let mut pc: u32 = 0x20ac180;
 loop { match pc {
 0x20ac180 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac1c8; continue; }
  pc = 0x20ac1a1; continue;
 }
 0x20ac1a1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac1af; continue; }
  pc = 0x20ac1a6; continue;
 }
 0x20ac1a6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac1c8; continue; }
  pc = 0x20ac1af; continue;
 }
 0x20ac1af => {
  cpu.push(0x20ac1b4); f_020b4041(cpu); if cpu.returned_to != 0x20ac1b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac1c8; continue; }
  pc = 0x20ac1b8; continue;
 }
 0x20ac1b8 => {
  cpu.push(0x20ac1bd); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac1bd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac1c8; continue; }
  pc = 0x20ac1c1; continue;
 }
 0x20ac1c1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac1ca; continue;
 }
 0x20ac1c8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac1ca; continue;
 }
 0x20ac1ca => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac20a; continue; }
  pc = 0x20ac1ce; continue;
 }
 0x20ac1ce => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169b04u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac202); cpu.call(t); if cpu.returned_to != 0x20ac202 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac20a); f_020adba0(cpu); if cpu.returned_to != 0x20ac20a { return; }
  pc = 0x20ac20a; continue;
 }
 0x20ac20a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ac21f); f_020b4740(cpu); if cpu.returned_to != 0x20ac21f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac26c; continue; }
  pc = 0x20ac22d; continue;
 }
 0x20ac22d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ac23f; continue; }
  pc = 0x20ac236; continue;
 }
 0x20ac236 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac26c; continue; }
  pc = 0x20ac23f; continue;
 }
 0x20ac23f => {
  cpu.push(0x20ac244); f_020b4041(cpu); if cpu.returned_to != 0x20ac244 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac26c; continue; }
  pc = 0x20ac248; continue;
 }
 0x20ac248 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ac262); f_020b42d3(cpu); if cpu.returned_to != 0x20ac262 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ac26c); f_020adb00(cpu); if cpu.returned_to != 0x20ac26c { return; }
  pc = 0x20ac26c; continue;
 }
 0x20ac26c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ac290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac290); }
 let mut pc: u32 = 0x20ac290;
 loop { match pc {
 0x20ac290 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac2d8; continue; }
  pc = 0x20ac2b1; continue;
 }
 0x20ac2b1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac2bf; continue; }
  pc = 0x20ac2b6; continue;
 }
 0x20ac2b6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac2d8; continue; }
  pc = 0x20ac2bf; continue;
 }
 0x20ac2bf => {
  cpu.push(0x20ac2c4); f_020b4041(cpu); if cpu.returned_to != 0x20ac2c4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac2d8; continue; }
  pc = 0x20ac2c8; continue;
 }
 0x20ac2c8 => {
  cpu.push(0x20ac2cd); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac2cd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac2d8; continue; }
  pc = 0x20ac2d1; continue;
 }
 0x20ac2d1 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac2da; continue;
 }
 0x20ac2d8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac2da; continue;
 }
 0x20ac2da => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac30c; continue; }
  pc = 0x20ac2de; continue;
 }
 0x20ac2de => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169ae4u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac304); cpu.call(t); if cpu.returned_to != 0x20ac304 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac30c); f_020adba0(cpu); if cpu.returned_to != 0x20ac30c { return; }
  pc = 0x20ac30c; continue;
 }
 0x20ac30c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac319); f_020b45f0(cpu); if cpu.returned_to != 0x20ac319 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20ac367; continue; }
  pc = 0x20ac328; continue;
 }
 0x20ac328 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ac33a; continue; }
  pc = 0x20ac331; continue;
 }
 0x20ac331 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac367; continue; }
  pc = 0x20ac33a; continue;
 }
 0x20ac33a => {
  cpu.push(0x20ac33f); f_020b4041(cpu); if cpu.returned_to != 0x20ac33f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac367; continue; }
  pc = 0x20ac343; continue;
 }
 0x20ac343 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ac35d); f_020b42d3(cpu); if cpu.returned_to != 0x20ac35d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ac367); f_020adb00(cpu); if cpu.returned_to != 0x20ac367 { return; }
  pc = 0x20ac367; continue;
 }
 0x20ac367 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ac390(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac390); }
 let mut pc: u32 = 0x20ac390;
 loop { match pc {
 0x20ac390 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac3d0; continue; }
  pc = 0x20ac3a9; continue;
 }
 0x20ac3a9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac3b7; continue; }
  pc = 0x20ac3ae; continue;
 }
 0x20ac3ae => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac3d0; continue; }
  pc = 0x20ac3b7; continue;
 }
 0x20ac3b7 => {
  cpu.push(0x20ac3bc); f_020b4041(cpu); if cpu.returned_to != 0x20ac3bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac3d0; continue; }
  pc = 0x20ac3c0; continue;
 }
 0x20ac3c0 => {
  cpu.push(0x20ac3c5); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac3c5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac3d0; continue; }
  pc = 0x20ac3c9; continue;
 }
 0x20ac3c9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac3d2; continue;
 }
 0x20ac3d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac3d2; continue;
 }
 0x20ac3d2 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac3fc; continue; }
  pc = 0x20ac3d6; continue;
 }
 0x20ac3d6 => {
  { let v: u32 = 0x2169ac8u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2169ac8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac3f9); f_020b42d3(cpu); if cpu.returned_to != 0x20ac3f9 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x20ac3fc; continue;
 }
 0x20ac3fc => {
  cpu.push(0x20ac401); f_020b40cb(cpu); if cpu.returned_to != 0x20ac401 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x22d13e4u32)); cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac410(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac410); }
 let mut pc: u32 = 0x20ac410;
 loop { match pc {
 0x20ac410 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac457; continue; }
  pc = 0x20ac430; continue;
 }
 0x20ac430 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac43e; continue; }
  pc = 0x20ac435; continue;
 }
 0x20ac435 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac457; continue; }
  pc = 0x20ac43e; continue;
 }
 0x20ac43e => {
  cpu.push(0x20ac443); f_020b4041(cpu); if cpu.returned_to != 0x20ac443 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac457; continue; }
  pc = 0x20ac447; continue;
 }
 0x20ac447 => {
  cpu.push(0x20ac44c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac44c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac457; continue; }
  pc = 0x20ac450; continue;
 }
 0x20ac450 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac459; continue;
 }
 0x20ac457 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac459; continue;
 }
 0x20ac459 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac48b; continue; }
  pc = 0x20ac45d; continue;
 }
 0x20ac45d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169aa8u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac483); cpu.call(t); if cpu.returned_to != 0x20ac483 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac48b); f_020adba0(cpu); if cpu.returned_to != 0x20ac48b { return; }
  pc = 0x20ac48b; continue;
 }
 0x20ac48b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac498); f_020b40aa(cpu); if cpu.returned_to != 0x20ac498 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac4c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac4c0); }
 let mut pc: u32 = 0x20ac4c0;
 loop { match pc {
 0x20ac4c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac500; continue; }
  pc = 0x20ac4d9; continue;
 }
 0x20ac4d9 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac4e7; continue; }
  pc = 0x20ac4de; continue;
 }
 0x20ac4de => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac500; continue; }
  pc = 0x20ac4e7; continue;
 }
 0x20ac4e7 => {
  cpu.push(0x20ac4ec); f_020b4041(cpu); if cpu.returned_to != 0x20ac4ec { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac500; continue; }
  pc = 0x20ac4f0; continue;
 }
 0x20ac4f0 => {
  cpu.push(0x20ac4f5); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac4f5 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac500; continue; }
  pc = 0x20ac4f9; continue;
 }
 0x20ac4f9 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac502; continue;
 }
 0x20ac500 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac502; continue;
 }
 0x20ac502 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac52c; continue; }
  pc = 0x20ac506; continue;
 }
 0x20ac506 => {
  { let v: u32 = 0x2169a90u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2169a90u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac529); f_020b42d3(cpu); if cpu.returned_to != 0x20ac529 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x20ac52c; continue;
 }
 0x20ac52c => {
  cpu.push(0x20ac531); f_020b415f(cpu); if cpu.returned_to != 0x20ac531 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x22d13e4u32)); cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac540(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac540); }
 let mut pc: u32 = 0x20ac540;
 loop { match pc {
 0x20ac540 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac587; continue; }
  pc = 0x20ac560; continue;
 }
 0x20ac560 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac56e; continue; }
  pc = 0x20ac565; continue;
 }
 0x20ac565 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac587; continue; }
  pc = 0x20ac56e; continue;
 }
 0x20ac56e => {
  cpu.push(0x20ac573); f_020b4041(cpu); if cpu.returned_to != 0x20ac573 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac587; continue; }
  pc = 0x20ac577; continue;
 }
 0x20ac577 => {
  cpu.push(0x20ac57c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac57c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac587; continue; }
  pc = 0x20ac580; continue;
 }
 0x20ac580 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac589; continue;
 }
 0x20ac587 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac589; continue;
 }
 0x20ac589 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac5bb; continue; }
  pc = 0x20ac58d; continue;
 }
 0x20ac58d => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169a78u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac5b3); cpu.call(t); if cpu.returned_to != 0x20ac5b3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac5bb); f_020adba0(cpu); if cpu.returned_to != 0x20ac5bb { return; }
  pc = 0x20ac5bb; continue;
 }
 0x20ac5bb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac5c8); f_020b4135(cpu); if cpu.returned_to != 0x20ac5c8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac5f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac5f0); }
 let mut pc: u32 = 0x20ac5f0;
 loop { match pc {
 0x20ac5f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac630; continue; }
  pc = 0x20ac609; continue;
 }
 0x20ac609 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac617; continue; }
  pc = 0x20ac60e; continue;
 }
 0x20ac60e => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac630; continue; }
  pc = 0x20ac617; continue;
 }
 0x20ac617 => {
  cpu.push(0x20ac61c); f_020b4041(cpu); if cpu.returned_to != 0x20ac61c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac630; continue; }
  pc = 0x20ac620; continue;
 }
 0x20ac620 => {
  cpu.push(0x20ac625); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac625 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac630; continue; }
  pc = 0x20ac629; continue;
 }
 0x20ac629 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac632; continue;
 }
 0x20ac630 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac632; continue;
 }
 0x20ac632 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac65c; continue; }
  pc = 0x20ac636; continue;
 }
 0x20ac636 => {
  { let v: u32 = 0x2169a5cu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2169a5cu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac659); f_020b42d3(cpu); if cpu.returned_to != 0x20ac659 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  pc = 0x20ac65c; continue;
 }
 0x20ac65c => {
  cpu.push(0x20ac661); f_020b4115(cpu); if cpu.returned_to != 0x20ac661 { return; }
  { let v: u32 = cpu.dec32(cpu.r32(0x22d13e4u32)); cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac670(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac670); }
 let mut pc: u32 = 0x20ac670;
 loop { match pc {
 0x20ac670 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac6b7; continue; }
  pc = 0x20ac690; continue;
 }
 0x20ac690 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac69e; continue; }
  pc = 0x20ac695; continue;
 }
 0x20ac695 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac6b7; continue; }
  pc = 0x20ac69e; continue;
 }
 0x20ac69e => {
  cpu.push(0x20ac6a3); f_020b4041(cpu); if cpu.returned_to != 0x20ac6a3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac6b7; continue; }
  pc = 0x20ac6a7; continue;
 }
 0x20ac6a7 => {
  cpu.push(0x20ac6ac); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac6ac { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac6b7; continue; }
  pc = 0x20ac6b0; continue;
 }
 0x20ac6b0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac6b9; continue;
 }
 0x20ac6b7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac6b9; continue;
 }
 0x20ac6b9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac6eb; continue; }
  pc = 0x20ac6bd; continue;
 }
 0x20ac6bd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169a44u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac6e3); cpu.call(t); if cpu.returned_to != 0x20ac6e3 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac6eb); f_020adba0(cpu); if cpu.returned_to != 0x20ac6eb { return; }
  pc = 0x20ac6eb; continue;
 }
 0x20ac6eb => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac6f8); f_020b40eb(cpu); if cpu.returned_to != 0x20ac6f8 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac720(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac720); }
 let mut pc: u32 = 0x20ac720;
 loop { match pc {
 0x20ac720 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac762; continue; }
  pc = 0x20ac73b; continue;
 }
 0x20ac73b => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac749; continue; }
  pc = 0x20ac740; continue;
 }
 0x20ac740 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac762; continue; }
  pc = 0x20ac749; continue;
 }
 0x20ac749 => {
  cpu.push(0x20ac74e); f_020b4041(cpu); if cpu.returned_to != 0x20ac74e { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac762; continue; }
  pc = 0x20ac752; continue;
 }
 0x20ac752 => {
  cpu.push(0x20ac757); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac757 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac762; continue; }
  pc = 0x20ac75b; continue;
 }
 0x20ac75b => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac764; continue;
 }
 0x20ac762 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac764; continue;
 }
 0x20ac764 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac78e; continue; }
  pc = 0x20ac768; continue;
 }
 0x20ac768 => {
  { let v: u32 = 0x2169a28u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2169a28u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ac78b); f_020b42d3(cpu); if cpu.returned_to != 0x20ac78b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20ac78e; continue;
 }
 0x20ac78e => {
  cpu.push(0x20ac793); f_020b4215(cpu); if cpu.returned_to != 0x20ac793 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20ac7de; continue; }
  pc = 0x20ac79f; continue;
 }
 0x20ac79f => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ac7b1; continue; }
  pc = 0x20ac7a8; continue;
 }
 0x20ac7a8 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac7de; continue; }
  pc = 0x20ac7b1; continue;
 }
 0x20ac7b1 => {
  cpu.push(0x20ac7b6); f_020b4041(cpu); if cpu.returned_to != 0x20ac7b6 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac7de; continue; }
  pc = 0x20ac7ba; continue;
 }
 0x20ac7ba => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ac7d4); f_020b42d3(cpu); if cpu.returned_to != 0x20ac7d4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ac7de); f_020adb50(cpu); if cpu.returned_to != 0x20ac7de { return; }
  pc = 0x20ac7de; continue;
 }
 0x20ac7de => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac8c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac8c0); }
 let mut pc: u32 = 0x20ac8c0;
 loop { match pc {
 0x20ac8c0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac907; continue; }
  pc = 0x20ac8e0; continue;
 }
 0x20ac8e0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac8ee; continue; }
  pc = 0x20ac8e5; continue;
 }
 0x20ac8e5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac907; continue; }
  pc = 0x20ac8ee; continue;
 }
 0x20ac8ee => {
  cpu.push(0x20ac8f3); f_020b4041(cpu); if cpu.returned_to != 0x20ac8f3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac907; continue; }
  pc = 0x20ac8f7; continue;
 }
 0x20ac8f7 => {
  cpu.push(0x20ac8fc); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac8fc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac907; continue; }
  pc = 0x20ac900; continue;
 }
 0x20ac900 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac909; continue;
 }
 0x20ac907 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac909; continue;
 }
 0x20ac909 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac949; continue; }
  pc = 0x20ac90d; continue;
 }
 0x20ac90d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21699e4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ac941); cpu.call(t); if cpu.returned_to != 0x20ac941 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ac949); f_020adba0(cpu); if cpu.returned_to != 0x20ac949 { return; }
  pc = 0x20ac949; continue;
 }
 0x20ac949 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ac95e); f_020b41af(cpu); if cpu.returned_to != 0x20ac95e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ac980(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ac980); }
 let mut pc: u32 = 0x20ac980;
 loop { match pc {
 0x20ac980 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ac9c7; continue; }
  pc = 0x20ac9a0; continue;
 }
 0x20ac9a0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ac9ae; continue; }
  pc = 0x20ac9a5; continue;
 }
 0x20ac9a5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ac9c7; continue; }
  pc = 0x20ac9ae; continue;
 }
 0x20ac9ae => {
  cpu.push(0x20ac9b3); f_020b4041(cpu); if cpu.returned_to != 0x20ac9b3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ac9c7; continue; }
  pc = 0x20ac9b7; continue;
 }
 0x20ac9b7 => {
  cpu.push(0x20ac9bc); f_020ad7d0(cpu); if cpu.returned_to != 0x20ac9bc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ac9c7; continue; }
  pc = 0x20ac9c0; continue;
 }
 0x20ac9c0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ac9c9; continue;
 }
 0x20ac9c7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ac9c9; continue;
 }
 0x20ac9c9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aca09; continue; }
  pc = 0x20ac9cd; continue;
 }
 0x20ac9cd => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21699c4u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20aca01); cpu.call(t); if cpu.returned_to != 0x20aca01 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20aca09); f_020adba0(cpu); if cpu.returned_to != 0x20aca09 { return; }
  pc = 0x20aca09; continue;
 }
 0x20aca09 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20aca1e); f_020b417f(cpu); if cpu.returned_to != 0x20aca1e { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aca40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aca40); }
 let mut pc: u32 = 0x20aca40;
 loop { match pc {
 0x20aca40 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20aca88; continue; }
  pc = 0x20aca61; continue;
 }
 0x20aca61 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20aca6f; continue; }
  pc = 0x20aca66; continue;
 }
 0x20aca66 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20aca88; continue; }
  pc = 0x20aca6f; continue;
 }
 0x20aca6f => {
  cpu.push(0x20aca74); f_020b4041(cpu); if cpu.returned_to != 0x20aca74 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20aca88; continue; }
  pc = 0x20aca78; continue;
 }
 0x20aca78 => {
  cpu.push(0x20aca7d); f_020ad7d0(cpu); if cpu.returned_to != 0x20aca7d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20aca88; continue; }
  pc = 0x20aca81; continue;
 }
 0x20aca81 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20aca8a; continue;
 }
 0x20aca88 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20aca8a; continue;
 }
 0x20aca8a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acaca; continue; }
  pc = 0x20aca8e; continue;
 }
 0x20aca8e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21699a8u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20acac2); cpu.call(t); if cpu.returned_to != 0x20acac2 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acaca); f_020adba0(cpu); if cpu.returned_to != 0x20acaca { return; }
  pc = 0x20acaca; continue;
 }
 0x20acaca => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20acadf); f_020b4088(cpu); if cpu.returned_to != 0x20acadf { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acb2c; continue; }
  pc = 0x20acaed; continue;
 }
 0x20acaed => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20acaff; continue; }
  pc = 0x20acaf6; continue;
 }
 0x20acaf6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acb2c; continue; }
  pc = 0x20acaff; continue;
 }
 0x20acaff => {
  cpu.push(0x20acb04); f_020b4041(cpu); if cpu.returned_to != 0x20acb04 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acb2c; continue; }
  pc = 0x20acb08; continue;
 }
 0x20acb08 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20acb22); f_020b42d3(cpu); if cpu.returned_to != 0x20acb22 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20acb2c); f_020adb50(cpu); if cpu.returned_to != 0x20acb2c { return; }
  pc = 0x20acb2c; continue;
 }
 0x20acb2c => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020acb50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20acb50); }
 let mut pc: u32 = 0x20acb50;
 loop { match pc {
 0x20acb50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20acb98; continue; }
  pc = 0x20acb71; continue;
 }
 0x20acb71 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20acb7f; continue; }
  pc = 0x20acb76; continue;
 }
 0x20acb76 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acb98; continue; }
  pc = 0x20acb7f; continue;
 }
 0x20acb7f => {
  cpu.push(0x20acb84); f_020b4041(cpu); if cpu.returned_to != 0x20acb84 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acb98; continue; }
  pc = 0x20acb88; continue;
 }
 0x20acb88 => {
  cpu.push(0x20acb8d); f_020ad7d0(cpu); if cpu.returned_to != 0x20acb8d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acb98; continue; }
  pc = 0x20acb91; continue;
 }
 0x20acb91 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20acb9a; continue;
 }
 0x20acb98 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20acb9a; continue;
 }
 0x20acb9a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acbcc; continue; }
  pc = 0x20acb9e; continue;
 }
 0x20acb9e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216998cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20acbc4); cpu.call(t); if cpu.returned_to != 0x20acbc4 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acbcc); f_020adba0(cpu); if cpu.returned_to != 0x20acbcc { return; }
  pc = 0x20acbcc; continue;
 }
 0x20acbcc => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20acbd9); f_020b4047(cpu); if cpu.returned_to != 0x20acbd9 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20acc27; continue; }
  pc = 0x20acbe8; continue;
 }
 0x20acbe8 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20acbfa; continue; }
  pc = 0x20acbf1; continue;
 }
 0x20acbf1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acc27; continue; }
  pc = 0x20acbfa; continue;
 }
 0x20acbfa => {
  cpu.push(0x20acbff); f_020b4041(cpu); if cpu.returned_to != 0x20acbff { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acc27; continue; }
  pc = 0x20acc03; continue;
 }
 0x20acc03 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20acc1d); f_020b42d3(cpu); if cpu.returned_to != 0x20acc1d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20acc27); f_020adb50(cpu); if cpu.returned_to != 0x20acc27 { return; }
  pc = 0x20acc27; continue;
 }
 0x20acc27 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020acc50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20acc50); }
 let mut pc: u32 = 0x20acc50;
 loop { match pc {
 0x20acc50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x3a0u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x3b4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x3b8u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20acca6; continue; }
  pc = 0x20acc7f; continue;
 }
 0x20acc7f => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20acc8d; continue; }
  pc = 0x20acc84; continue;
 }
 0x20acc84 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acca6; continue; }
  pc = 0x20acc8d; continue;
 }
 0x20acc8d => {
  cpu.push(0x20acc92); f_020b4041(cpu); if cpu.returned_to != 0x20acc92 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acca6; continue; }
  pc = 0x20acc96; continue;
 }
 0x20acc96 => {
  cpu.push(0x20acc9b); f_020ad7d0(cpu); if cpu.returned_to != 0x20acc9b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acca6; continue; }
  pc = 0x20acc9f; continue;
 }
 0x20acc9f => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20acca8; continue;
 }
 0x20acca6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20acca8; continue;
 }
 0x20acca8 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acce4; continue; }
  pc = 0x20accac; continue;
 }
 0x20accac => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169910u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x38cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20accd7); cpu.call(t); if cpu.returned_to != 0x20accd7 { return; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acce4); f_020adba0(cpu); if cpu.returned_to != 0x20acce4 { return; }
  pc = 0x20acce4; continue;
 }
 0x20acce4 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20acceb); f_020b4a50(cpu); if cpu.returned_to != 0x20acceb { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acf1d; continue; }
  pc = 0x20accf8; continue;
 }
 0x20accf8 => {
  { cpu.sub32(cpu.r32(0x22d13e8u32), 0x0u32); }
  if cpu.zf { pc = 0x20acf65; continue; }
  pc = 0x20acd05; continue;
 }
 0x20acd05 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20acd1b; continue; }
  pc = 0x20acd0e; continue;
 }
 0x20acd0e => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acf65; continue; }
  pc = 0x20acd1b; continue;
 }
 0x20acd1b => {
  cpu.push(0x20acd20); f_020b4041(cpu); if cpu.returned_to != 0x20acd20 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acf65; continue; }
  pc = 0x20acd28; continue;
 }
 0x20acd28 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20acd42); f_020b42d3(cpu); if cpu.returned_to != 0x20acd42 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x388u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216992cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x394u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20acd6f); cpu.call(t); if cpu.returned_to != 0x20acd6f { return; }
  { let v: u32 = 0x280u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acd7c); f_020adba0(cpu); if cpu.returned_to != 0x20acd7c { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20acd96); f_020b42d3(cpu); if cpu.returned_to != 0x20acd96 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216993cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a8u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x3acu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20acdb8); cpu.call(t); if cpu.returned_to != 0x20acdb8 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acdc0); f_020adba0(cpu); if cpu.returned_to != 0x20acdc0 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20acdda); f_020b42d3(cpu); if cpu.returned_to != 0x20acdda { return; }
  { let v: u32 = cpu.r32(0xfeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x38cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216994cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x398u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ace0a); cpu.call(t); if cpu.returned_to != 0x20ace0a { return; }
  { let v: u32 = 0x180u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ace17); f_020adba0(cpu); if cpu.returned_to != 0x20ace17 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ace30); f_020b42d3(cpu); if cpu.returned_to != 0x20ace30 { return; }
  { let v: u32 = cpu.r32(0x100u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x390u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216995cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x39cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ace60); cpu.call(t); if cpu.returned_to != 0x20ace60 { return; }
  { let v: u32 = 0x200u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ace6d); f_020adba0(cpu); if cpu.returned_to != 0x20ace6d { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20ace87); f_020b42d3(cpu); if cpu.returned_to != 0x20ace87 { return; }
  { let v: u32 = cpu.r32(0x102u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x394u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216996cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a0u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20aceb7); cpu.call(t); if cpu.returned_to != 0x20aceb7 { return; }
  { let v: u32 = 0x80u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acec4); f_020adba0(cpu); if cpu.returned_to != 0x20acec4 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebx = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20acede); f_020b42d3(cpu); if cpu.returned_to != 0x20acede { return; }
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x398u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216997cu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x3a4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20acf0e); cpu.call(t); if cpu.returned_to != 0x20acf0e { return; }
  { let v: u32 = 0x300u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20acf1b); f_020adba0(cpu); if cpu.returned_to != 0x20acf1b { return; }
  pc = 0x20acf65; continue;
 }
 0x20acf1d => {
  { cpu.sub32(cpu.r32(0x22d13e8u32), 0x0u32); }
  if cpu.zf { pc = 0x20acf65; continue; }
  pc = 0x20acf26; continue;
 }
 0x20acf26 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20acf38; continue; }
  pc = 0x20acf2f; continue;
 }
 0x20acf2f => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acf65; continue; }
  pc = 0x20acf38; continue;
 }
 0x20acf38 => {
  cpu.push(0x20acf3d); f_020b4041(cpu); if cpu.returned_to != 0x20acf3d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acf65; continue; }
  pc = 0x20acf41; continue;
 }
 0x20acf41 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20acf5b); f_020b42d3(cpu); if cpu.returned_to != 0x20acf5b { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20acf65); f_020adb50(cpu); if cpu.returned_to != 0x20acf65 { return; }
  pc = 0x20acf65; continue;
 }
 0x20acf65 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020acf80(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20acf80); }
 let mut pc: u32 = 0x20acf80;
 loop { match pc {
 0x20acf80 => {
  f_020b4041(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020acf90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20acf90); }
 let mut pc: u32 = 0x20acf90;
 loop { match pc {
 0x20acf90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20acfd7; continue; }
  pc = 0x20acfb0; continue;
 }
 0x20acfb0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20acfbe; continue; }
  pc = 0x20acfb5; continue;
 }
 0x20acfb5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20acfd7; continue; }
  pc = 0x20acfbe; continue;
 }
 0x20acfbe => {
  cpu.push(0x20acfc3); f_020b4041(cpu); if cpu.returned_to != 0x20acfc3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20acfd7; continue; }
  pc = 0x20acfc7; continue;
 }
 0x20acfc7 => {
  cpu.push(0x20acfcc); f_020ad7d0(cpu); if cpu.returned_to != 0x20acfcc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20acfd7; continue; }
  pc = 0x20acfd0; continue;
 }
 0x20acfd0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20acfd9; continue;
 }
 0x20acfd7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20acfd9; continue;
 }
 0x20acfd9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad00b; continue; }
  pc = 0x20acfdd; continue;
 }
 0x20acfdd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169900u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad003); cpu.call(t); if cpu.returned_to != 0x20ad003 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad00b); f_020adba0(cpu); if cpu.returned_to != 0x20ad00b { return; }
  pc = 0x20ad00b; continue;
 }
 0x20ad00b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ad018); f_020b4410(cpu); if cpu.returned_to != 0x20ad018 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad040); }
 let mut pc: u32 = 0x20ad040;
 loop { match pc {
 0x20ad040 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x90u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad087; continue; }
  pc = 0x20ad060; continue;
 }
 0x20ad060 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad06e; continue; }
  pc = 0x20ad065; continue;
 }
 0x20ad065 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad087; continue; }
  pc = 0x20ad06e; continue;
 }
 0x20ad06e => {
  cpu.push(0x20ad073); f_020b4041(cpu); if cpu.returned_to != 0x20ad073 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad087; continue; }
  pc = 0x20ad077; continue;
 }
 0x20ad077 => {
  cpu.push(0x20ad07c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad07c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad087; continue; }
  pc = 0x20ad080; continue;
 }
 0x20ad080 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x20ad089; continue;
 }
 0x20ad087 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20ad089; continue;
 }
 0x20ad089 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad0e5; continue; }
  pc = 0x20ad08d; continue;
 }
 0x20ad08d => {
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
  { let v = 0x21698d8u32; cpu.push(v); }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad0dd); cpu.call(t); if cpu.returned_to != 0x20ad0dd { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad0e5); f_020adba0(cpu); if cpu.returned_to != 0x20ad0e5 { return; }
  pc = 0x20ad0e5; continue;
 }
 0x20ad0e5 => {
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xacu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20ad10a); f_020b3f67(cpu); if cpu.returned_to != 0x20ad10a { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x10u32); cpu.esp = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20ad158; continue; }
  pc = 0x20ad119; continue;
 }
 0x20ad119 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ad12b; continue; }
  pc = 0x20ad122; continue;
 }
 0x20ad122 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad158; continue; }
  pc = 0x20ad12b; continue;
 }
 0x20ad12b => {
  cpu.push(0x20ad130); f_020b4041(cpu); if cpu.returned_to != 0x20ad130 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad158; continue; }
  pc = 0x20ad134; continue;
 }
 0x20ad134 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ad14e); f_020b42d3(cpu); if cpu.returned_to != 0x20ad14e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ad158); f_020adb50(cpu); if cpu.returned_to != 0x20ad158 { return; }
  pc = 0x20ad158; continue;
 }
 0x20ad158 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x22d13e4u32)); cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ad170(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad170); }
 let mut pc: u32 = 0x20ad170;
 loop { match pc {
 0x20ad170 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20ad17a); f_020b3f00(cpu); if cpu.returned_to != 0x20ad17a { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad180); }
 let mut pc: u32 = 0x20ad180;
 loop { match pc {
 0x20ad180 => {
  pc = 0x20b3efc; continue;
 }
 0x20b3efc => {
  { let v = cpu.flags(); cpu.push(v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad190(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad190); }
 let mut pc: u32 = 0x20ad190;
 loop { match pc {
 0x20ad190 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad1d7; continue; }
  pc = 0x20ad1b0; continue;
 }
 0x20ad1b0 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad1be; continue; }
  pc = 0x20ad1b5; continue;
 }
 0x20ad1b5 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad1d7; continue; }
  pc = 0x20ad1be; continue;
 }
 0x20ad1be => {
  cpu.push(0x20ad1c3); f_020b4041(cpu); if cpu.returned_to != 0x20ad1c3 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad1d7; continue; }
  pc = 0x20ad1c7; continue;
 }
 0x20ad1c7 => {
  cpu.push(0x20ad1cc); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad1cc { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad1d7; continue; }
  pc = 0x20ad1d0; continue;
 }
 0x20ad1d0 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad1d9; continue;
 }
 0x20ad1d7 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad1d9; continue;
 }
 0x20ad1d9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad20b; continue; }
  pc = 0x20ad1dd; continue;
 }
 0x20ad1dd => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21698bcu32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad203); cpu.call(t); if cpu.returned_to != 0x20ad203 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad20b); f_020adba0(cpu); if cpu.returned_to != 0x20ad20b { return; }
  pc = 0x20ad20b; continue;
 }
 0x20ad20b => {
  { let v: u32 = cpu.r32(0x94u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ad218); f_020b3eb7(cpu); if cpu.returned_to != 0x20ad218 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad240(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad240); }
 let mut pc: u32 = 0x20ad240;
 loop { match pc {
 0x20ad240 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad287; continue; }
  pc = 0x20ad260; continue;
 }
 0x20ad260 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad26e; continue; }
  pc = 0x20ad265; continue;
 }
 0x20ad265 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad287; continue; }
  pc = 0x20ad26e; continue;
 }
 0x20ad26e => {
  cpu.push(0x20ad273); f_020b4041(cpu); if cpu.returned_to != 0x20ad273 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad287; continue; }
  pc = 0x20ad277; continue;
 }
 0x20ad277 => {
  cpu.push(0x20ad27c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad27c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad287; continue; }
  pc = 0x20ad280; continue;
 }
 0x20ad280 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad289; continue;
 }
 0x20ad287 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad289; continue;
 }
 0x20ad289 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad2d7; continue; }
  pc = 0x20ad28d; continue;
 }
 0x20ad28d => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x88u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216989cu32; cpu.push(v); }
  { let v = 0x3u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad2cf); cpu.call(t); if cpu.returned_to != 0x20ad2cf { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad2d7); f_020adba0(cpu); if cpu.returned_to != 0x20ad2d7 { return; }
  pc = 0x20ad2d7; continue;
 }
 0x20ad2d7 => {
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad2f4); f_020b3ca1(cpu); if cpu.returned_to != 0x20ad2f4 { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad310(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad310); }
 let mut pc: u32 = 0x20ad310;
 loop { match pc {
 0x20ad310 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad357; continue; }
  pc = 0x20ad330; continue;
 }
 0x20ad330 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad33e; continue; }
  pc = 0x20ad335; continue;
 }
 0x20ad335 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad357; continue; }
  pc = 0x20ad33e; continue;
 }
 0x20ad33e => {
  cpu.push(0x20ad343); f_020b4041(cpu); if cpu.returned_to != 0x20ad343 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad357; continue; }
  pc = 0x20ad347; continue;
 }
 0x20ad347 => {
  cpu.push(0x20ad34c); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad34c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad357; continue; }
  pc = 0x20ad350; continue;
 }
 0x20ad350 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad359; continue;
 }
 0x20ad357 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad359; continue;
 }
 0x20ad359 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad399; continue; }
  pc = 0x20ad35d; continue;
 }
 0x20ad35d => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x216987cu32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad391); cpu.call(t); if cpu.returned_to != 0x20ad391 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad399); f_020adba0(cpu); if cpu.returned_to != 0x20ad399 { return; }
  pc = 0x20ad399; continue;
 }
 0x20ad399 => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ad3ae); f_020b3c29(cpu); if cpu.returned_to != 0x20ad3ae { return; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad3d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad3d0); }
 let mut pc: u32 = 0x20ad3d0;
 loop { match pc {
 0x20ad3d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad418; continue; }
  pc = 0x20ad3f1; continue;
 }
 0x20ad3f1 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad3ff; continue; }
  pc = 0x20ad3f6; continue;
 }
 0x20ad3f6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad418; continue; }
  pc = 0x20ad3ff; continue;
 }
 0x20ad3ff => {
  cpu.push(0x20ad404); f_020b4041(cpu); if cpu.returned_to != 0x20ad404 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad418; continue; }
  pc = 0x20ad408; continue;
 }
 0x20ad408 => {
  cpu.push(0x20ad40d); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad40d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad418; continue; }
  pc = 0x20ad411; continue;
 }
 0x20ad411 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad41a; continue;
 }
 0x20ad418 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad41a; continue;
 }
 0x20ad41a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad44c; continue; }
  pc = 0x20ad41e; continue;
 }
 0x20ad41e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169860u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad444); cpu.call(t); if cpu.returned_to != 0x20ad444 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad44c); f_020adba0(cpu); if cpu.returned_to != 0x20ad44c { return; }
  pc = 0x20ad44c; continue;
 }
 0x20ad44c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ad459); f_020b3c01(cpu); if cpu.returned_to != 0x20ad459 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20ad4a7; continue; }
  pc = 0x20ad468; continue;
 }
 0x20ad468 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ad47a; continue; }
  pc = 0x20ad471; continue;
 }
 0x20ad471 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad4a7; continue; }
  pc = 0x20ad47a; continue;
 }
 0x20ad47a => {
  cpu.push(0x20ad47f); f_020b4041(cpu); if cpu.returned_to != 0x20ad47f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad4a7; continue; }
  pc = 0x20ad483; continue;
 }
 0x20ad483 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ad49d); f_020b42d3(cpu); if cpu.returned_to != 0x20ad49d { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ad4a7); f_020adb00(cpu); if cpu.returned_to != 0x20ad4a7 { return; }
  pc = 0x20ad4a7; continue;
 }
 0x20ad4a7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ad4d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad4d0); }
 let mut pc: u32 = 0x20ad4d0;
 loop { match pc {
 0x20ad4d0 => {
  pc = 0x20b3797; continue;
 }
 0x20b3797 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x217bbeau32)); cpu.w32(0x217bbeau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad4e0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ad4e0); } r_020ad4e0(cpu, 0x20ad4e0); }
pub fn f_020b3790(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3790); } r_020ad4e0(cpu, 0x20b3790); }
fn r_020ad4e0(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20ad4e0 => {
  pc = 0x20b3790; continue;
 }
 0x20b3790 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x217bbeau32)); cpu.w32(0x217bbeau32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad4f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad4f0); }
 let mut pc: u32 = 0x20ad4f0;
 loop { match pc {
 0x20ad4f0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x88u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad538; continue; }
  pc = 0x20ad511; continue;
 }
 0x20ad511 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad51f; continue; }
  pc = 0x20ad516; continue;
 }
 0x20ad516 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad538; continue; }
  pc = 0x20ad51f; continue;
 }
 0x20ad51f => {
  cpu.push(0x20ad524); f_020b4041(cpu); if cpu.returned_to != 0x20ad524 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad538; continue; }
  pc = 0x20ad528; continue;
 }
 0x20ad528 => {
  cpu.push(0x20ad52d); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad52d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad538; continue; }
  pc = 0x20ad531; continue;
 }
 0x20ad531 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad53a; continue;
 }
 0x20ad538 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad53a; continue;
 }
 0x20ad53a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad57a; continue; }
  pc = 0x20ad53e; continue;
 }
 0x20ad53e => {
  { let v: u32 = cpu.r32(0x9cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169844u32; cpu.push(v); }
  { let v = 0x2u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad572); cpu.call(t); if cpu.returned_to != 0x20ad572 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad57a); f_020adba0(cpu); if cpu.returned_to != 0x20ad57a { return; }
  pc = 0x20ad57a; continue;
 }
 0x20ad57a => {
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0xa0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ad58f); f_020b4a30(cpu); if cpu.returned_to != 0x20ad58f { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.eax = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad5dc; continue; }
  pc = 0x20ad59d; continue;
 }
 0x20ad59d => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ad5af; continue; }
  pc = 0x20ad5a6; continue;
 }
 0x20ad5a6 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad5dc; continue; }
  pc = 0x20ad5af; continue;
 }
 0x20ad5af => {
  cpu.push(0x20ad5b4); f_020b4041(cpu); if cpu.returned_to != 0x20ad5b4 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad5dc; continue; }
  pc = 0x20ad5b8; continue;
 }
 0x20ad5b8 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ad5d2); f_020b42d3(cpu); if cpu.returned_to != 0x20ad5d2 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ad5dc); f_020adb50(cpu); if cpu.returned_to != 0x20ad5dc { return; }
  pc = 0x20ad5dc; continue;
 }
 0x20ad5dc => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ad600(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad600); }
 let mut pc: u32 = 0x20ad600;
 loop { match pc {
 0x20ad600 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x84u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20ad648; continue; }
  pc = 0x20ad621; continue;
 }
 0x20ad621 => {
  { cpu.sub32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x20ad62f; continue; }
  pc = 0x20ad626; continue;
 }
 0x20ad626 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad648; continue; }
  pc = 0x20ad62f; continue;
 }
 0x20ad62f => {
  cpu.push(0x20ad634); f_020b4041(cpu); if cpu.returned_to != 0x20ad634 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad648; continue; }
  pc = 0x20ad638; continue;
 }
 0x20ad638 => {
  cpu.push(0x20ad63d); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad63d { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad648; continue; }
  pc = 0x20ad641; continue;
 }
 0x20ad641 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad64a; continue;
 }
 0x20ad648 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad64a; continue;
 }
 0x20ad64a => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad67c; continue; }
  pc = 0x20ad64e; continue;
 }
 0x20ad64e => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x80u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x2169824u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad674); cpu.call(t); if cpu.returned_to != 0x20ad674 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad67c); f_020adba0(cpu); if cpu.returned_to != 0x20ad67c { return; }
  pc = 0x20ad67c; continue;
 }
 0x20ad67c => {
  { let v: u32 = cpu.r32(0x98u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x20ad689); f_020b4430(cpu); if cpu.returned_to != 0x20ad689 { return; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20ad6d7; continue; }
  pc = 0x20ad698; continue;
 }
 0x20ad698 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ad6aa; continue; }
  pc = 0x20ad6a1; continue;
 }
 0x20ad6a1 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad6d7; continue; }
  pc = 0x20ad6aa; continue;
 }
 0x20ad6aa => {
  cpu.push(0x20ad6af); f_020b4041(cpu); if cpu.returned_to != 0x20ad6af { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad6d7; continue; }
  pc = 0x20ad6b3; continue;
 }
 0x20ad6b3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xeu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x217a4d8u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ad6cd); f_020b42d3(cpu); if cpu.returned_to != 0x20ad6cd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ad6d7); f_020adb00(cpu); if cpu.returned_to != 0x20ad6d7 { return; }
  pc = 0x20ad6d7; continue;
 }
 0x20ad6d7 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x22d13e4u32, v); }
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
pub fn f_020ad700(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad700); }
 let mut pc: u32 = 0x20ad700;
 loop { match pc {
 0x20ad700 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217a4d0u32), 0x0u32); }
  if cpu.zf { pc = 0x20ad7c8; continue; }
  pc = 0x20ad711; continue;
 }
 0x20ad711 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x22d13e8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d13e4u32, v); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20ad74f; continue; }
  pc = 0x20ad728; continue;
 }
 0x20ad728 => {
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { pc = 0x20ad736; continue; }
  pc = 0x20ad72d; continue;
 }
 0x20ad72d => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad74f; continue; }
  pc = 0x20ad736; continue;
 }
 0x20ad736 => {
  cpu.push(0x20ad73b); f_020b4041(cpu); if cpu.returned_to != 0x20ad73b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad74f; continue; }
  pc = 0x20ad73f; continue;
 }
 0x20ad73f => {
  cpu.push(0x20ad744); f_020ad7d0(cpu); if cpu.returned_to != 0x20ad744 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad74f; continue; }
  pc = 0x20ad748; continue;
 }
 0x20ad748 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ad751; continue;
 }
 0x20ad74f => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ad751; continue;
 }
 0x20ad751 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20ad77b; continue; }
  pc = 0x20ad755; continue;
 }
 0x20ad755 => {
  { let v: u32 = 0x2169810u32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x2169810u32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ad778); f_020b42d3(cpu); if cpu.returned_to != 0x20ad778 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20ad77b; continue;
 }
 0x20ad77b => {
  cpu.push(0x20ad780); f_020b44a0(cpu); if cpu.returned_to != 0x20ad780 { return; }
  { cpu.sub32(cpu.r32(0x22d13e8u32), 0x0u32); }
  if cpu.zf { pc = 0x20ad7b3; continue; }
  pc = 0x20ad789; continue;
 }
 0x20ad789 => {
  { cpu.sub32(cpu.r32(0x22d13e4u32), 0x1u32); }
  if cpu.zf { pc = 0x20ad79b; continue; }
  pc = 0x20ad792; continue;
 }
 0x20ad792 => {
  { cpu.sub32(cpu.r32(0x22d13ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20ad7b3; continue; }
  pc = 0x20ad79b; continue;
 }
 0x20ad79b => {
  cpu.push(0x20ad7a0); f_020b4041(cpu); if cpu.returned_to != 0x20ad7a0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ad7b3; continue; }
  pc = 0x20ad7a4; continue;
 }
 0x20ad7a4 => {
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  cpu.push(0x20ad7b0); f_020b4345(cpu); if cpu.returned_to != 0x20ad7b0 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x20ad7b3; continue;
 }
 0x20ad7b3 => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217a4d0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
  pc = 0x20ad7c8; continue;
 }
 0x20ad7c8 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ad7d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad7d0); }
 let mut pc: u32 = 0x20ad7d0;
 loop { match pc {
 0x20ad7d0 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x94u32); cpu.esp = v; }
  cpu.push(0x20ad7de); f_020b379e(cpu); if cpu.returned_to != 0x20ad7de { return; }
  { let v: u32 = cpu.r32(0x22d13f0u32); cpu.edx = v; }
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
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x20ad896; continue; }
  pc = 0x20ad83e; continue;
 }
 0x20ad83e => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x90u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edi as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.esi as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x84u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21697d0u32; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v: u32 = 0x8cu32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8cu32.wrapping_add(cpu.esp), v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x98u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x9cu32.wrapping_add(cpu.esp), v); }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad88c); cpu.call(t); if cpu.returned_to != 0x20ad88c { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20ad894); f_020adba0(cpu); if cpu.returned_to != 0x20ad894 { return; }
  pc = 0x20ad8bc; continue;
 }
 0x20ad896 => {
  { let v: u32 = 0x21697ecu32; cpu.edi = v; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x21697ecu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20ad8b9); f_020b42d3(cpu); if cpu.returned_to != 0x20ad8b9 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20ad8bc; continue;
 }
 0x20ad8bc => {
  { let v: u32 = cpu.r32(0x22d13e4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x1u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20ad8df; continue; }
  pc = 0x20ad8c7; continue;
 }
 0x20ad8c7 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x21697fcu32; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x20ad8dc); f_020b42d3(cpu); if cpu.returned_to != 0x20ad8dc { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  pc = 0x20ad8df; continue;
 }
 0x20ad8df => {
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
pub fn f_020ad8f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ad8f0); }
 let mut pc: u32 = 0x20ad8f0;
 loop { match pc {
 0x20ad8f0 => {
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
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20ad940); cpu.call(t); if cpu.returned_to != 0x20ad940 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20ad95e); f_020b42d3(cpu); if cpu.returned_to != 0x20ad95e { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x94u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(12);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020adb00(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20adb00); }
 let mut pc: u32 = 0x20adb00;
 loop { match pc {
 0x20adb00 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x21697c0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20adb24); cpu.call(t); if cpu.returned_to != 0x20adb24 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20adb42); f_020b42d3(cpu); if cpu.returned_to != 0x20adb42 { return; }
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
pub fn f_020adb50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20adb50); }
 let mut pc: u32 = 0x20adb50;
 loop { match pc {
 0x20adb50 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x44u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x40u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = 0x21697b0u32; cpu.push(v); }
  { let v = 0x1u32; cpu.push(v); }
  { let v: u32 = 0x4cu32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.esp); cpu.edi = v; }
  let t = cpu.r32(0x217ba38u32); cpu.push(0x20adb74); cpu.call(t); if cpu.returned_to != 0x20adb74 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { cpu.scas(1, 2); }
  { let v: u32 = !cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20adb92); f_020b42d3(cpu); if cpu.returned_to != 0x20adb92 { return; }
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
pub fn f_020adba0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20adba0); }
 let mut pc: u32 = 0x20adba0;
 loop { match pc {
 0x20adba0 => {
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
  { let v: u32 = cpu.r32(0x22d13f4u32); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20adbc1); f_020b42d3(cpu); if cpu.returned_to != 0x20adbc1 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020adbd0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20adbd0); }
 let mut pc: u32 = 0x20adbd0;
 loop { match pc {
 0x20adbd0 => {
  { cpu.sub32(cpu.r32(0x217a4d4u32), 0x0u32); }
  if !cpu.zf { pc = 0x20adc50; continue; }
  pc = 0x20adbd9; continue;
 }
 0x20adbd9 => {
  { let v = 0x20a5ae0u32; cpu.push(v); }
  { let v = 0x20adbd0u32; cpu.push(v); }
  cpu.push(0x20adbe8); f_020b35a0(cpu); if cpu.returned_to != 0x20adbe8 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d13e4u32; cpu.push(v); }
  cpu.push(0x20adbf7); f_020b34e0(cpu); if cpu.returned_to != 0x20adbf7 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d13e8u32; cpu.push(v); }
  cpu.push(0x20adc06); f_020b34e0(cpu); if cpu.returned_to != 0x20adc06 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d13ecu32; cpu.push(v); }
  cpu.push(0x20adc15); f_020b34e0(cpu); if cpu.returned_to != 0x20adc15 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d13f4u32; cpu.push(v); }
  cpu.push(0x20adc24); f_020b34e0(cpu); if cpu.returned_to != 0x20adc24 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x217a4d0u32; cpu.push(v); }
  cpu.push(0x20adc33); f_020b34e0(cpu); if cpu.returned_to != 0x20adc33 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = 0x4u32; cpu.push(v); }
  { let v = 0x22d13f0u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20adc47); f_020b34e0(cpu); if cpu.returned_to != 0x20adc47 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217a4d4u32, v); }
  pc = 0x20adc50; continue;
 }
 0x20adc50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020adc60(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20adc60); }
 let mut pc: u32 = 0x20adc60;
 loop { match pc {
 0x20adc60 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if cpu.zf { pc = 0x20adca9; continue; }
  pc = 0x20adc69; continue;
 }
 0x20adc69 => {
  if !cpu.cf { pc = 0x20adc93; continue; }
  pc = 0x20adc6b; continue;
 }
 0x20adc6b => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.eax); }
  if (cpu.cf || cpu.zf) { pc = 0x20adc93; continue; }
  pc = 0x20adc72; continue;
 }
 0x20adc72 => {
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
 0x20adc93 => {
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
  pc = 0x20adca9; continue;
 }
 0x20adca9 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020adcad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20adcad); } r_020adcad(cpu, 0x20adcad); }
pub fn f_020add3d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20add3d); } r_020adcad(cpu, 0x20add3d); }
fn r_020adcad(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20adcad => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)) ^ ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.xor8((cpu.eax as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.unimplemented(0x20adcb3, "Aaa");
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), ((cpu.ebx >> 8) as u8)); }
  { cpu.popad(); }
  cpu.unimplemented(0x20adcb8, "Bound");
  { let v = 0x6a69u16; cpu.esp = cpu.esp.wrapping_sub(2); cpu.w16(cpu.esp, v); }
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6eu32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(2))), 0x6fu32); cpu.ebp = v; }
  if cpu.of { pc = 0x20add39; continue; }
  pc = 0x20adcc8; continue;
 }
 0x20adcc8 => {
  if cpu.cf { pc = 0x20add3d; continue; }
  pc = 0x20adcca; continue;
 }
 0x20adcca => {
  if cpu.zf { cpu.call(0x20add41); return; }
  pc = 0x20adccc; continue;
 }
 0x20adccc => {
  if (cpu.cf || cpu.zf) { pc = 0x20add45; continue; }
  pc = 0x20adcce; continue;
 }
 0x20adcce => {
  if cpu.sf { pc = 0x20add49; continue; }
  pc = 0x20adcd0; continue;
 }
 0x20adcd0 => {
  if cpu.pf() { f_020adcd2(cpu); return; }
  f_020adcd2(cpu); return;
 }
 0x20add39 => {
  { let v: u8 = 0x17u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8).wrapping_add((cpu.ebx as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  pc = 0x20add3d; continue;
 }
 0x20add3d => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20add43); f_020add37(cpu); if cpu.returned_to != 0x20add43 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  pc = 0x20add45; continue;
 }
 0x20add45 => {
  if cpu.zf { pc = 0x20add5f; continue; }
  pc = 0x20add47; continue;
 }
 0x20add47 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), 0x41c64e6du32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x3039u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x20add5f; continue;
 }
 0x20add49 => {
  cpu.unimplemented(0x20add49, "Insd");
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u8 = 0xc2u8; cpu.w8(0xffffff81u32.wrapping_add(cpu.ecx), v); }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.eax)), cpu.esi); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  pc = 0x20add5f; continue;
 }
 0x20add5f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020adcd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20adcd2); }
 let mut pc: u32 = 0x20adcd2;
 loop { match pc {
 0x20adcd2 => {
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
  pc = 0x20adce8; continue;
 }
 0x20adce8 => {
  { let v: u32 = 0x24u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x20adcadu32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x20adce8; continue; }
  pc = 0x20add08; continue;
 }
 0x20add08 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20add08; continue; }
  pc = 0x20add12; continue;
 }
 0x20add12 => {
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
pub fn f_020add1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20add1c); }
 let mut pc: u32 = 0x20add1c;
 loop { match pc {
 0x20add1c => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if !cpu.zf { pc = 0x20add2e; continue; }
  pc = 0x20add24; continue;
 }
 0x20add24 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20add2e; continue; }
  pc = 0x20add28; continue;
 }
 0x20add28 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20add2e; continue;
 }
 0x20add2e => {
  cpu.push(0x20add33); f_020adcd2(cpu); if cpu.returned_to != 0x20add33 { return; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020add37(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20add37); }
 let mut pc: u32 = 0x20add37;
 loop { match pc {
 0x20add37 => {
  { let v: u32 = 0x217b0d0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020add61(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20add61); }
 let mut pc: u32 = 0x20add61;
 loop { match pc {
 0x20add61 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x20add69); f_020add37(cpu); if cpu.returned_to != 0x20add69 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20add6f; continue; }
  pc = 0x20add6d; continue;
 }
 0x20add6d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20add6f; continue;
 }
 0x20add6f => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020add74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20add74); } r_020add74(cpu, 0x20add74); }
pub fn f_020adde8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20adde8); } r_020add74(cpu, 0x20adde8); }
pub fn f_020ae005(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ae005); } r_020add74(cpu, 0x20ae005); }
pub fn f_020c4643(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20c4643); } r_020add74(cpu, 0x20c4643); }
fn r_020add74(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20add74 => {
  pc = 0x20addec; continue;
 }
 0x20adde8 => {
  cpu.unimplemented(0x20adde8, "Arpl");
  { let v: u8 = (cpu.ebx as u8).wrapping_add(((cpu.ebx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b924u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b910u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x217b91cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x217b947u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x217b948u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x20ade6e; continue; }
  pc = 0x20ade30; continue;
 }
 0x20addec => {
  {  }
  { let v: u32 = cpu.esp & 0xfffffffcu32; cpu.esp = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b924u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b910u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x24u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x217b91cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x50484152u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x217b947u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x217b948u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = 0x81u32; cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x4458u16); }
  if !cpu.zf { pc = 0x20ade6e; continue; }
  pc = 0x20ade30; continue;
 }
 0x20ade30 => {
  { let v: u8 = (cpu.ebx as u8).wrapping_sub(0x30u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r16(0x217b91cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.bases[0])); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0xfffu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfffff000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b910u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.shr32(cpu.ebx, 12u32); cpu.ebx = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20adea6; continue;
 }
 0x20ade6e => {
  { cpu.sub16((cpu.eax as u16), 0x4243u16); }
  if !cpu.zf { pc = 0x20adea8; continue; }
  pc = 0x20ade74; continue;
 }
 0x20ade74 => {
  { let v: u32 = cpu.edx; cpu.w32(0x217b0dcu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u8 = 0x4au8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x217b91cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.esi, 4u32); cpu.esi = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x9u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20adea6; continue;
 }
 0x20adea6 => {
  pc = 0x20adf01; continue;
 }
 0x20adea8 => {
  { let v: u16 = 0x78u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0xff00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20adeea; continue; }
  pc = 0x20adeb6; continue;
 }
 0x20adeb6 => {
  { let v: u16 = (cpu.segments[5] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), 0x0u16); }
  if cpu.zf { pc = 0x20adec5; continue; }
  pc = 0x20adebf; continue;
 }
 0x20adebf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x217b0e0u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x20adec5; continue;
 }
 0x20adec5 => {
  { let v: u16 = 0x6u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = cpu.or16((cpu.edx as u16), (cpu.ecx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x20aded9; continue; }
  pc = 0x20aded7; continue;
 }
 0x20aded7 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20aded9; continue;
 }
 0x20aded9 => {
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x217b91cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = cpu.r16(0x2cu32.wrapping_add(cpu.bases[0])); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  pc = 0x20adf01; continue;
 }
 0x20adeea => {
  { let v: u16 = (cpu.segments[3] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x24u16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2cu16; cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x17u16; cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20adf01; continue;
 }
 0x20adf01 => {
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x217b93eu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x217b93fu32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.ebx; cpu.set_segment(0, v as u16); }
  { let v: u16 = (cpu.segments[3] as u16); cpu.w16(0x20ae049u32.wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x217b941u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x217b945u32.wrapping_add(cpu.bases[3]), v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.r16(0x217b91cu32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = 0x22d2888u32; cpu.edx = v; }
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
  if cpu.zf { pc = 0x20adf56; continue; }
  pc = 0x20adf53; continue;
 }
 0x20adf53 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.movs(1, true); }
  pc = 0x20adf56; continue;
 }
 0x20adf56 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.stos(1, false); }
  { cpu.stos(1, false); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.r16(0x217b945u32.wrapping_add(cpu.bases[0])); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x20adf68; continue;
 }
 0x20adf68 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x20202020u32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x37386f6eu32); }
  if !cpu.zf { pc = 0x20adf7d; continue; }
  pc = 0x20adf76; continue;
 }
 0x20adf76 => {
  { cpu.sub8(cpu.r8(0x4u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x3du8); }
  if !cpu.zf { pc = 0x20adf7d; continue; }
  pc = 0x20adf7c; continue;
 }
 0x20adf7c => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20adf7d; continue;
 }
 0x20adf7d => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.lods(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x20adf7d; continue; }
  pc = 0x20adf83; continue;
 }
 0x20adf83 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20adf68; continue; }
  pc = 0x20adf88; continue;
 }
 0x20adf88 => {
  { cpu.lods(1, false); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  pc = 0x20adf8b; continue;
 }
 0x20adf8b => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3])), 0x0u8); }
  { cpu.movs(1, false); }
  if !cpu.zf { cpu.poll(); pc = 0x20adf8b; continue; }
  pc = 0x20adf91; continue;
 }
 0x20adf91 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x217b93cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.edi; cpu.w32(0x217b920u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b90cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0x22d2888u32; cpu.ecx = v; }
  { let v: u32 = 0x217d040u32; cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edi); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x20adfcc; continue; }
  pc = 0x20adfbf; continue;
 }
 0x20adfbf => {
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20adfcc; continue; }
  pc = 0x20adfc7; continue;
 }
 0x20adfc7 => {
  { let v: u32 = 0x1000u32; cpu.ecx = v; }
  pc = 0x20adfcc; continue;
 }
 0x20adfcc => {
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, true); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x3u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.stos(1, true); }
  { let v: u32 = 0x22d2888u32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xfu32); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) & 0xf0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x217b914u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x217b918u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xffu32; cpu.eax = v; }
  cpu.flat_ds(0x20adff7); cpu.push(0x20adffc); f_020bd3da(cpu); if cpu.returned_to != 0x20adffc { return; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  cpu.flat_ds(0x20adffe); cpu.push(0x20ae003); f_020bd38a(cpu); if cpu.returned_to != 0x20ae003 { return; }
  pc = 0x20ae003; continue;
 }
 0x20ae003 => {
  pc = 0x20ae028; continue;
 }
 0x20ae005 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x20adde8u32; cpu.edx = v; }
  { let v: u16 = 0x3d01u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { cpu.df = false; }
  pc = 0x20ae019; continue;
 }
 0x20ae019 => {
  { cpu.lods(1, false); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20ae019; continue; }
  pc = 0x20ae01e; continue;
 }
 0x20ae01e => {
  { let v: u32 = cpu.esi; cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x20ae028; continue;
 }
 0x20ae028 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  cpu.flat_ds(0x20ae033); cpu.push(0x20ae038); f_020bd425(cpu); if cpu.returned_to != 0x20ae038 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u8 = 0x4cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x20ae03d); f_020ae040(cpu); return;
 }
 0x20c4643 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x217cb74u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20c4646); cpu.push(0x20c464c); cpu.call(t); if cpu.returned_to != 0x20c464c { return; }
  let t = cpu.r32(0x217cb78u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20c464c); cpu.push(0x20c4652); cpu.call(t); if cpu.returned_to != 0x20c4652 { return; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.poll(); pc = 0x20ae003; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae040(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae040); }
 let mut pc: u32 = 0x20ae040;
 loop { match pc {
 0x20ae040 => {
  { let v = cpu.r16(0x20ae049u32); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20ae048); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae049(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ae049); } r_020ae049(cpu, 0x20ae049); }
pub fn f_020ae04b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ae04b); } r_020ae049(cpu, 0x20ae04b); }
pub fn f_020ae050(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ae050); } r_020ae049(cpu, 0x20ae050); }
fn r_020ae049(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20ae049 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20ae04b; continue;
 }
 0x20ae04b => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x51u32.wrapping_add(cpu.ebx)).wrapping_add((cpu.edx as u8)); cpu.w8(0x51u32.wrapping_add(cpu.ebx), v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217b0e4u32), 0x0u32); }
  if cpu.zf { pc = 0x20ae076; continue; }
  pc = 0x20ae05c; continue;
 }
 0x20ae050 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217b0e4u32), 0x0u32); }
  if cpu.zf { pc = 0x20ae076; continue; }
  pc = 0x20ae05c; continue;
 }
 0x20ae05c => {
  { let v = 0x20ae050u32; cpu.push(v); }
  { let v = 0x20ae350u32; cpu.push(v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20ae06d); f_020b3500(cpu); if cpu.returned_to != 0x20ae06d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b0e4u32, v); }
  pc = 0x20ae076; continue;
 }
 0x20ae076 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae080); }
 let mut pc: u32 = 0x20ae080;
 loop { match pc {
 0x20ae080 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x217a4d0u32), 0x0u32); }
  if !cpu.zf { pc = 0x20ae20f; continue; }
  pc = 0x20ae094; continue;
 }
 0x20ae094 => {
  { let v: u32 = 0x20ae340u32; cpu.ebx = v; }
  { let v: u32 = 0x20ae330u32; cpu.ecx = v; }
  { let v: u32 = 0x20ae320u32; cpu.esi = v; }
  { let v: u32 = 0x20ae300u32; cpu.edi = v; }
  { let v: u32 = 0x20ae238u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x217b0e4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217ba28u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x217ba2cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x217ba30u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x217ba34u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x217ba38u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ae0f6; continue; }
  pc = 0x20ae0d4; continue;
 }
 0x20ae0d4 => {
  { let v = 0x20ae050u32; cpu.push(v); }
  { let v = 0x20ae350u32; cpu.push(v); }
  cpu.push(0x20ae0e3); f_020b35a0(cpu); if cpu.returned_to != 0x20ae0e3 { return; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217b0e4u32, v); }
  cpu.push(0x20ae0f6); f_020adbd0(cpu); if cpu.returned_to != 0x20ae0f6 { return; }
  pc = 0x20ae0f6; continue;
 }
 0x20ae0f6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x216aa8cu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d13e8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d13ecu32, v); }
  cpu.push(0x20ae10e); f_020bd5d9(cpu); if cpu.returned_to != 0x20ae10e { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x20ae12a; continue; }
  pc = 0x20ae116; continue;
 }
 0x20ae116 => {
  cpu.push(0x20ae11b); f_020b44d0(cpu); if cpu.returned_to != 0x20ae11b { return; }
  { let v: u32 = 0x1u32; cpu.w32(0x217a4d0u32, v); }
  pc = 0x20ae20f; continue;
 }
 0x20ae12a => {
  { let v: u32 = 0x216aa98u32; cpu.eax = v; }
  cpu.push(0x20ae134); f_020bd5d9(cpu); if cpu.returned_to != 0x20ae134 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20ae144; continue; }
  pc = 0x20ae13a; continue;
 }
 0x20ae13a => {
  { let v: u32 = 0x1u32; cpu.w32(0x22d13ecu32, v); }
  pc = 0x20ae144; continue;
 }
 0x20ae144 => {
  { let v = cpu.edx; cpu.push(v); }
  cpu.push(0x20ae14a); f_020b4252(cpu); if cpu.returned_to != 0x20ae14a { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13f4u32, v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20ae16f; continue; }
  pc = 0x20ae157; continue;
 }
 0x20ae157 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20ae161); f_020b44d0(cpu); if cpu.returned_to != 0x20ae161 { return; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217a4d0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ae16f => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x20ae176); f_020a58ac(cpu); if cpu.returned_to != 0x20ae176 { return; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x20ae17d); f_020a592c(cpu); if cpu.returned_to != 0x20ae17d { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x22d1420u32; cpu.edi = v; }
  cpu.push(0x20ae18b); f_020bd778(cpu); if cpu.returned_to != 0x20ae18b { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20ae18f; continue;
 }
 0x20ae18f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20ae18f; continue; }
  pc = 0x20ae199; continue;
 }
 0x20ae199 => {
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = 0x216aaa8u32; cpu.push(v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x22d1438u32, v); }
  cpu.push(0x20ae1ab); f_020adba0(cpu); if cpu.returned_to != 0x20ae1ab { return; }
  { let v = 0x216aafcu32; cpu.push(v); }
  cpu.push(0x20ae1b5); f_020adba0(cpu); if cpu.returned_to != 0x20ae1b5 { return; }
  { let v = 0x216ab38u32; cpu.push(v); }
  cpu.push(0x20ae1bf); f_020adba0(cpu); if cpu.returned_to != 0x20ae1bf { return; }
  { let v = 0x22d1420u32; cpu.push(v); }
  cpu.push(0x20ae1c9); f_020adba0(cpu); if cpu.returned_to != 0x20ae1c9 { return; }
  { let v = 0x216ab48u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  cpu.push(0x20ae1d8); f_020adba0(cpu); if cpu.returned_to != 0x20ae1d8 { return; }
  cpu.push(0x20ae1dd); f_020b44d0(cpu); if cpu.returned_to != 0x20ae1dd { return; }
  cpu.push(0x20ae1e2); f_020b379e(cpu); if cpu.returned_to != 0x20ae1e2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13f0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e8u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x22d13e4u32, v); }
  cpu.push(0x20ae1f8); f_020ad7d0(cpu); if cpu.returned_to != 0x20ae1f8 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v = 0x216aba0u32; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d13e4u32, v); }
  cpu.push(0x20ae209); f_020adba0(cpu); if cpu.returned_to != 0x20ae209 { return; }
  { let v: u32 = cpu.edi; cpu.w32(0x217a4d0u32, v); }
  pc = 0x20ae20f; continue;
 }
 0x20ae20f => {
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
pub fn f_020ae220(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ae220); } r_020ae220(cpu, 0x20ae220); }
pub fn f_020ae238(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20ae238); } r_020ae220(cpu, 0x20ae238); }
fn r_020ae220(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20ae220 => {
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x20ae22d; continue; }
  pc = 0x20ae223; continue;
 }
 0x20ae223 => {
  { let v: u8 = ((cpu.ecx >> 8) as u8).wrapping_add(cpu.r8(0xau32.wrapping_add(cpu.edx))); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ebx >> 8) as u8).wrapping_add(cpu.r8(0xffffffe2u32.wrapping_add(cpu.edi))); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) | cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let a = cpu.esi; let b = cpu.eax; { let v: u32 = b; cpu.esi = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x20ae22d; continue;
 }
 0x20ae22d => {
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x20ae239; continue; }
  pc = 0x20ae22f; continue;
 }
 0x20ae22f => {
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0xd0020ae2u32.wrapping_add(cpu.ecx))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  if { cpu.ecx = cpu.ecx.wrapping_sub(1); cpu.ecx != 0 } { pc = 0x20ae241; continue; }
  pc = 0x20ae237; continue;
 }
 0x20ae237 => {
  { let v: u8 = (cpu.edx as u8).wrapping_add(cpu.r8(0x57u32.wrapping_add(cpu.esi))); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20ae2ed; continue; }
  pc = 0x20ae254; continue;
 }
 0x20ae238 => {
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x20ae239; continue;
 }
 0x20ae239 => {
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20ae2ed; continue; }
  pc = 0x20ae254; continue;
 }
 0x20ae241 => {
  { let v: u8 = (cpu.eax as u8) & 0x14u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, 0x5u32); }
  if (!cpu.cf && !cpu.zf) { pc = 0x20ae2ed; continue; }
  pc = 0x20ae254; continue;
 }
 0x20ae254 => {
  cpu.poll(); pc = cpu.r32(0x20ae220u32.wrapping_add(cpu.ecx.wrapping_mul(4))); continue;
 }
 0x20ae25c => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20ae263); f_0209c702(cpu); if cpu.returned_to != 0x20ae263 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x20ae26c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20ae276); f_0209c702(cpu); if cpu.returned_to != 0x20ae276 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x20ae27f => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20ae28d); f_0209c702(cpu); if cpu.returned_to != 0x20ae28d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x20ae296 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20ae2a8); f_0209c702(cpu); if cpu.returned_to != 0x20ae2a8 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x14u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x20ae2b1 => {
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
  cpu.push(0x20ae2c7); f_0209c702(cpu); if cpu.returned_to != 0x20ae2c7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 0x20ae2d0 => {
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
  cpu.push(0x20ae2ea); f_0209c702(cpu); if cpu.returned_to != 0x20ae2ea { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  pc = 0x20ae2ed; continue;
 }
 0x20ae2ed => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(20);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae300(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae300); }
 let mut pc: u32 = 0x20ae300;
 loop { match pc {
 0x20ae300 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20ae311); f_020a594c(cpu); if cpu.returned_to != 0x20ae311 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae320(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae320); }
 let mut pc: u32 = 0x20ae320;
 loop { match pc {
 0x20ae320 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20ae329); f_020bd5d9(cpu); if cpu.returned_to != 0x20ae329 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae330(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae330); }
 let mut pc: u32 = 0x20ae330;
 loop { match pc {
 0x20ae330 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20ae339); f_020bd557(cpu); if cpu.returned_to != 0x20ae339 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae340); }
 let mut pc: u32 = 0x20ae340;
 loop { match pc {
 0x20ae340 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20ae349); f_020bd474(cpu); if cpu.returned_to != 0x20ae349 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae350(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae350); }
 let mut pc: u32 = 0x20ae350;
 loop { match pc {
 0x20ae350 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x217b0e4u32), 0x0u32); }
  if !cpu.zf { pc = 0x20ae37e; continue; }
  pc = 0x20ae35c; continue;
 }
 0x20ae35c => {
  { let v = 0x20ae050u32; cpu.push(v); }
  { let v = 0x20ae350u32; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  cpu.push(0x20ae370); f_020b35a0(cpu); if cpu.returned_to != 0x20ae370 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b0e4u32, v); }
  cpu.push(0x20ae37e); f_020adbd0(cpu); if cpu.returned_to != 0x20ae37e { return; }
  pc = 0x20ae37e; continue;
 }
 0x20ae37e => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae382(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae382); }
 let mut pc: u32 = 0x20ae382;
 loop { match pc {
 0x20ae382 => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x217b0e8u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae38a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae38a); }
 let mut pc: u32 = 0x20ae38a;
 loop { match pc {
 0x20ae38a => {
  { let a = cpu.r32(0x4u32.wrapping_add(cpu.esp)); let b = cpu.eax; { let v: u32 = b; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); } { let v: u32 = a; cpu.eax = v; } }
  cpu.push(0x20ae393); f_020ae39d(cpu); if cpu.returned_to != 0x20ae393 { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(8);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae39d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae39d); }
 let mut pc: u32 = 0x20ae39d;
 loop { match pc {
 0x20ae39d => {
  { cpu.sub32(cpu.eax, cpu.esp); }
  if !cpu.cf { pc = 0x20ae3ae; continue; }
  pc = 0x20ae3a1; continue;
 }
 0x20ae3a1 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x217b920u32)); }
  if (cpu.cf || cpu.zf) { pc = 0x20ae3ae; continue; }
  pc = 0x20ae3ad; continue;
 }
 0x20ae3ad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ae3ae => {
  { let v: u16 = (cpu.segments[2] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), cpu.r16(0x217b0e8u32)); }
  if cpu.zf { f_020ae3bb(cpu); return; }
  pc = 0x20ae3ba; continue;
 }
 0x20ae3ba => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae3bb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae3bb); }
 let mut pc: u32 = 0x20ae3bb;
 loop { match pc {
 0x20ae3bb => {
  { let v: u32 = 0x217b0eau32; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  cpu.push(0x20ae3ca); f_020bd78a(cpu); if cpu.returned_to != 0x20ae3ca { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  f_020ae3d0(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae3d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae3d0); }
 let mut pc: u32 = 0x20ae3d0;
 loop { match pc {
 0x20ae3d0 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x22d16ccu32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x22d16ccu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20ae421); f_020a594c(cpu); if cpu.returned_to != 0x20ae421 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.r32(0x22d16ccu32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae434(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae434); }
 let mut pc: u32 = 0x20ae434;
 loop { match pc {
 0x20ae434 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20ae465); f_020a594c(cpu); if cpu.returned_to != 0x20ae465 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae474(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae474); }
 let mut pc: u32 = 0x20ae474;
 loop { match pc {
 0x20ae474 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20ae4bc); f_020a594c(cpu); if cpu.returned_to != 0x20ae4bc { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae4cc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae4cc); }
 let mut pc: u32 = 0x20ae4cc;
 loop { match pc {
 0x20ae4cc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.edx = v; }
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
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20ae514); f_020a594c(cpu); if cpu.returned_to != 0x20ae514 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae524(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae524); }
 let mut pc: u32 = 0x20ae524;
 loop { match pc {
 0x20ae524 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20ae555); f_020a594c(cpu); if cpu.returned_to != 0x20ae555 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae560(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae560); }
 let mut pc: u32 = 0x20ae560;
 loop { match pc {
 0x20ae560 => {
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
  pc = 0x20ae633; continue;
 }
 0x20ae57d => {
  { let v: u32 = cpu.r32(0x22d16b8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d163cu32); cpu.ebp = v; }
  { let v: u8 = 0x1du8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x2u16; cpu.w16(0x22u32.wrapping_add(cpu.eax), v); }
  { let v: u16 = 0x1eu16; cpu.w16(0x28u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d16b0u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u16 = cpu.r16(0x22d16b0u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.esi); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebp, 4u32); cpu.ebp = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) ^ ((cpu.ecx >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp & 0xffffu32; cpu.ebp = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22d16c8u32); cpu.ebp = v; }
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
  { let v: u16 = cpu.r16(0x217b0feu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x12u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x1eu32); cpu.esi = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  cpu.push(0x20ae627); f_020ae474(cpu); if cpu.returned_to != 0x20ae627 { return; }
  { let v: u32 = cpu.ecx.wrapping_add(0x1eu32); cpu.ecx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22d16eau32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20ae633; continue;
 }
 0x20ae633 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x217b0fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20ae57d; continue; }
  pc = 0x20ae643; continue;
 }
 0x20ae643 => {
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
pub fn f_020ae650(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae650); }
 let mut pc: u32 = 0x20ae650;
 loop { match pc {
 0x20ae650 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x22d16c8u32); cpu.edi = v; }
  { cpu.sub16(cpu.r16(0x217b100u32), 0x0u16); }
  if !cpu.zf { pc = 0x20ae8ea; continue; }
  pc = 0x20ae672; continue;
 }
 0x20ae672 => {
  { let v: u32 = 0x21400b5u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x2140000u32); cpu.ebx = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20ae689); f_0209b06c(cpu); if cpu.returned_to != 0x20ae689 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16d4u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ae6ab; continue; }
  pc = 0x20ae692; continue;
 }
 0x20ae692 => {
  { let v = 0x216abb0u32; cpu.push(v); }
  { let v = 0x216abe8u32; cpu.push(v); }
  cpu.push(0x20ae6a1); f_020a3d38(cpu); if cpu.returned_to != 0x20ae6a1 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20ae8ef; continue;
 }
 0x20ae6ab => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x2140000u32; cpu.eax = v; }
  cpu.push(0x20ae6b7); f_0209c228(cpu); if cpu.returned_to != 0x20ae6b7 { return; }
  { let v: u32 = 0x2140008u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x2140000u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = 0x60u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16d8u32, v); }
  pc = 0x20ae6d0; continue;
 }
 0x20ae6d0 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax.wrapping_mul(4)).wrapping_add(cpu.bases[5])); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x22d16e4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16e0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ae6fe; continue; }
  pc = 0x20ae6f9; continue;
 }
 0x20ae6f9 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x20ae70d; continue; }
  pc = 0x20ae6fe; continue;
 }
 0x20ae6fe => {
  { let v = cpu.edx; cpu.set_segment(5, v as u16); }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[5])), 0xcfu8); }
  if cpu.zf { pc = 0x20ae70d; continue; }
  pc = 0x20ae706; continue;
 }
 0x20ae706 => {
  { let v: u8 = cpu.inc8((cpu.ecx as u8)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.ecx as u8), 0x66u8); }
  if (cpu.cf || cpu.zf) { cpu.poll(); pc = 0x20ae6d0; continue; }
  pc = 0x20ae70d; continue;
 }
 0x20ae70d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x22d16ebu32, v); }
  { cpu.sub8((cpu.ecx as u8), 0x67u8); }
  if !cpu.zf { pc = 0x20ae73b; continue; }
  pc = 0x20ae718; continue;
 }
 0x20ae718 => {
  { let v = 0x216abf4u32; cpu.push(v); }
  { let v = 0x216abe8u32; cpu.push(v); }
  cpu.push(0x20ae727); f_020a3d38(cpu); if cpu.returned_to != 0x20ae727 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ae73b => {
  { let v: u32 = 0x200u32; cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20ae755); f_020a594c(cpu); if cpu.returned_to != 0x20ae755 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d16e6u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x22d16e8u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x22d16d8u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x22d16d8u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax & 0xfu32; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20ae7af); f_020a594c(cpu); if cpu.returned_to != 0x20ae7af { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x217b0fcu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x217b100u32, v); }
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
  { let v: u16 = cpu.r16(0x217b0feu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x102u32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0xcu32); cpu.eax = v; }
  cpu.push(0x20ae80e); f_0209b06c(cpu); if cpu.returned_to != 0x20ae80e { return; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16d0u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20ae840; continue; }
  pc = 0x20ae81d; continue;
 }
 0x20ae81d => {
  { let v = 0x216ac34u32; cpu.push(v); }
  { let v = 0x216abe8u32; cpu.push(v); }
  cpu.push(0x20ae82c); f_020a3d38(cpu); if cpu.returned_to != 0x20ae82c { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20ae840 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x217b0fcu32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d163cu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x2u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16b8u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16bcu32, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x30u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16b0u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16b4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(0x1eu32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20ae8a8; continue;
 }
 0x20ae891 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x217b0feu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x12u32); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  pc = 0x20ae8a8; continue;
 }
 0x20ae8a8 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u16 = cpu.r16(0x217b0fcu32); cpu.esi = (cpu.esi & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20ae891; continue; }
  pc = 0x20ae8b5; continue;
 }
 0x20ae8b5 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x217b0feu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c4u32, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x22d16ebu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16ccu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x22d16bcu32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x22d16c8u32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x22d16a3u32, v); }
  { let v: u8 = 0x0u8; cpu.w8(0x8u32.wrapping_add(cpu.eax), v); }
  pc = 0x20ae8ea; continue;
 }
 0x20ae8ea => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20ae8ef; continue;
 }
 0x20ae8ef => {
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
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
pub fn f_020ae900(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae900); }
 let mut pc: u32 = 0x20ae900;
 loop { match pc {
 0x20ae900 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0x217b100u32), 0x0u16); }
  if cpu.zf { pc = 0x20ae96a; continue; }
  pc = 0x20ae90f; continue;
 }
 0x20ae90f => {
  { let v: u32 = cpu.r32(0x22d16d0u32); cpu.eax = v; }
  cpu.push(0x20ae919); f_0209b6f0(cpu); if cpu.returned_to != 0x20ae919 { return; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  cpu.push(0x20ae923); f_0209b6f0(cpu); if cpu.returned_to != 0x20ae923 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = 0x201u32; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x22d16e6u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x22d16e8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x31u32; cpu.eax = v; }
  cpu.push(0x20ae95c); f_020a594c(cpu); if cpu.returned_to != 0x20ae95c { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x217b100u32, v); }
  pc = 0x20ae96a; continue;
 }
 0x20ae96a => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae970(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae970); }
 let mut pc: u32 = 0x20ae970;
 loop { match pc {
 0x20ae970 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20ae99e); f_020a594c(cpu); if cpu.returned_to != 0x20ae99e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020ae9ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20ae9ac); }
 let mut pc: u32 = 0x20ae9ac;
 loop { match pc {
 0x20ae9ac => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x22d163cu32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x22d163cu32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x3u32; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20aea0d); f_020a594c(cpu); if cpu.returned_to != 0x20aea0d { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.r32(0x22d163cu32); cpu.eax = v; }
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
pub fn f_020aea28(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aea28); }
 let mut pc: u32 = 0x20aea28;
 loop { match pc {
 0x20aea28 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { cpu.sub32(cpu.r32(0x22d163cu32), 0x0u32); }
  if cpu.zf { pc = 0x20aea7c; continue; }
  pc = 0x20aea36; continue;
 }
 0x20aea36 => {
  { let v: u32 = cpu.r32(0x22d16d4u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x22d163cu32); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x14u32.wrapping_add(cpu.esp), v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x22d16ebu32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20aea6f); f_020a594c(cpu); if cpu.returned_to != 0x20aea6f { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x22d16c0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x22d163cu32, v); }
  pc = 0x20aea7c; continue;
 }
 0x20aea7c => {
  { let v: u32 = cpu.add32(cpu.esp, 0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aea84(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aea84); }
 let mut pc: u32 = 0x20aea84;
 loop { match pc {
 0x20aea84 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x22d16bcu32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d16c4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  cpu.push(0x20aeaa8); f_0209c228(cpu); if cpu.returned_to != 0x20aeaa8 { return; }
  { let v: u32 = cpu.r32(0x22d16c4u32); cpu.esi = v; }
  cpu.push(0x20aeab3); f_0209ea80(cpu); if cpu.returned_to != 0x20aeab3 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x2u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x22d16a3u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x6u32.wrapping_add(cpu.esi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = 0x1cu32.wrapping_add(cpu.esp); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x22d16bcu32); cpu.ebp = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { let v: u16 = 0x0u16; cpu.w16(0x4u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x22d163cu32); cpu.edx = v; }
  { let v: u16 = 0x0u16; cpu.w16(0x6u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x22d16b4u32); cpu.esi = v; }
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
  pc = 0x20aeb0a; continue;
 }
 0x20aeb0a => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0xffu8; cpu.w8(0x1bu32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { cpu.sub32(cpu.eax, 0x6u32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20aeb0a; continue; }
  pc = 0x20aeb15; continue;
 }
 0x20aeb15 => {
  { let v: u32 = cpu.r32(0x22d16b4u32); cpu.eax = v; }
  { let v: u8 = 0x4u8; cpu.w8(0x5u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x26u32.wrapping_add(cpu.ebp), v); }
  { let v: u16 = cpu.r16(0x22d16c4u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) ^ ((cpu.eax >> 8) as u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.eax as u8) & 0xfu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.r16(0x22d16b4u32); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2au32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x22d16c4u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d16bcu32, v); }
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
  cpu.push(0x20aeb7f); f_020ae4cc(cpu); if cpu.returned_to != 0x20aeb7f { return; }
  { let v: u32 = cpu.r32(0x22d16bcu32); cpu.ebp = v; }
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
pub fn f_020aeb90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aeb90); }
 let mut pc: u32 = 0x20aeb90;
 loop { match pc {
 0x20aeb90 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x22d16c8u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x7fffffffu32; cpu.esi = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20aebe1; continue;
 }
 0x20aebbb => {
  { let v: u32 = cpu.r32(0x22d16b8u32); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x8u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebx.wrapping_mul(1))), 0x0u8); }
  if !cpu.zf { pc = 0x20aebda; continue; }
  pc = 0x20aebc8; continue;
 }
 0x20aebc8 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.ecx)); cpu.edi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if (cpu.cf || cpu.zf) { pc = 0x20aebda; continue; }
  pc = 0x20aebd4; continue;
 }
 0x20aebd4 => {
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20aebda; continue;
 }
 0x20aebda => {
  { let v: u32 = cpu.ebx.wrapping_add(0x30u32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20aebe1; continue;
 }
 0x20aebe1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x217b0fcu32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20aebbb; continue; }
  pc = 0x20aebee; continue;
 }
 0x20aebee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20aecfc; continue; }
  pc = 0x20aebfb; continue;
 }
 0x20aebfb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = 0x12u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20aec0f; continue; }
  pc = 0x20aec0d; continue;
 }
 0x20aec0d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20aec0f; continue;
 }
 0x20aec0f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20aec2d; continue; }
  pc = 0x20aec16; continue;
 }
 0x20aec16 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d16b0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x20aec2d; continue;
 }
 0x20aec2d => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d16b8u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22d163cu32); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xau32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0x1eu32); cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16).wrapping_add(cpu.r16(0x22d16b0u32)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.edx >> 8) as u8) ^ ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = (cpu.edx as u8) & 0xfu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x24u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x22d16b0u32); cpu.ebx = v; }
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
  { let v: u16 = cpu.r16(0x217b0feu32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x12u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d16c8u32, v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2eu32.wrapping_add(cpu.eax), v); }
  cpu.push(0x20aece0); f_020ae474(cpu); if cpu.returned_to != 0x20aece0 { return; }
  { let v: u32 = cpu.r32(0x22d16c8u32); cpu.ebp = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x22d16eau32, v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d16c8u32, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20aecfc => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x22d16c8u32, v); }
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
pub fn f_020aed10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aed10); }
 let mut pc: u32 = 0x20aed10;
 loop { match pc {
 0x20aed10 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aed40; continue; }
  pc = 0x20aed14; continue;
 }
 0x20aed14 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  pc = 0x20aed16; continue;
 }
 0x20aed16 => {
  { cpu.and8((cpu.eax as u8), 0x3u8); }
  if cpu.zf { pc = 0x20aed23; continue; }
  pc = 0x20aed1a; continue;
 }
 0x20aed1a => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20aed16; continue; }
  pc = 0x20aed23; continue;
 }
 0x20aed23 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  cpu.push(0x20aed2c); f_020aed47(cpu); if cpu.returned_to != 0x20aed2c { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aed40; continue; }
  pc = 0x20aed32; continue;
 }
 0x20aed32 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aed40; continue; }
  pc = 0x20aed37; continue;
 }
 0x20aed37 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x1u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aed40; continue; }
  pc = 0x20aed3d; continue;
 }
 0x20aed3d => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x2u32.wrapping_add(cpu.eax), v); }
  pc = 0x20aed40; continue;
 }
 0x20aed40 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aed47(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aed47); }
 let mut pc: u32 = 0x20aed47;
 loop { match pc {
 0x20aed47 => {
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aedb2; continue; }
  pc = 0x20aed4b; continue;
 }
 0x20aed4b => {
  { cpu.and8((cpu.eax as u8), 0x1fu8); }
  if cpu.zf { pc = 0x20aed57; continue; }
  pc = 0x20aed4f; continue;
 }
 0x20aed4f => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20aed4b; continue; }
  pc = 0x20aed57; continue;
 }
 0x20aed57 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.shr32(cpu.ecx, 2u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aed97; continue; }
  pc = 0x20aed5d; continue;
 }
 0x20aed5d => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aed89; continue; }
  pc = 0x20aed60; continue;
 }
 0x20aed60 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  if cpu.zf { pc = 0x20aed86; continue; }
  pc = 0x20aed6e; continue;
 }
 0x20aed6e => {
  { cpu.sub8(cpu.r8(0x20u32.wrapping_add(cpu.eax)), (cpu.edx as u8)); }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x18u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x1cu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x20u32.wrapping_add(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20aed60; continue; }
  pc = 0x20aed83; continue;
 }
 0x20aed83 => {
  { let v: u32 = 0xfffffff0u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x20aed86; continue;
 }
 0x20aed86 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x20aed89; continue;
 }
 0x20aed89 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x10u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x20aed97; continue;
 }
 0x20aed97 => {
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.and32(cpu.ecx, 0x3u32); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aedb2; continue; }
  pc = 0x20aed9d; continue;
 }
 0x20aed9d => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aedb2; continue; }
  pc = 0x20aeda5; continue;
 }
 0x20aeda5 => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x20aedb2; continue; }
  pc = 0x20aedad; continue;
 }
 0x20aedad => {
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x4u32.wrapping_add(cpu.eax); cpu.eax = v; }
  pc = 0x20aedb2; continue;
 }
 0x20aedb2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aedc0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aedc0); }
 let mut pc: u32 = 0x20aedc0;
 loop { match pc {
 0x20aedc0 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u8 = (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.edx, 8u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  cpu.push(0x20aedd5); f_020aed10(cpu); if cpu.returned_to != 0x20aedd5 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aedd8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aedd8); }
 let mut pc: u32 = 0x20aedd8;
 loop { match pc {
 0x20aedd8 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { pc = 0x20aedee; continue; }
  pc = 0x20aede1; continue;
 }
 0x20aede1 => {
  { cpu.interrupt(0x1a); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x2u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20aee03; continue;
 }
 0x20aedee => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  if !cpu.zf { pc = 0x20aee03; continue; }
  pc = 0x20aedf6; continue;
 }
 0x20aedf6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x1a); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20aee03; continue;
 }
 0x20aee03 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aee07(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aee07); }
 let mut pc: u32 = 0x20aee07;
 loop { match pc {
 0x20aee07 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x216ac64u32; cpu.eax = v; }
  cpu.push(0x20aee1c); f_020bd5d9(cpu); if cpu.returned_to != 0x20aee1c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20aee29; continue; }
  pc = 0x20aee22; continue;
 }
 0x20aee22 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20aee60; continue;
 }
 0x20aee29 => {
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20aee34; continue; }
  pc = 0x20aee2f; continue;
 }
 0x20aee2f => {
  { let v: u32 = 0x216ac74u32; cpu.ebx = v; }
  pc = 0x20aee34; continue;
 }
 0x20aee34 => {
  { let v = 0x0u32; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  cpu.push(0x20aee43); f_020bd7a7(cpu); if cpu.returned_to != 0x20aee43 { return; }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x20aee4f; continue; }
  pc = 0x20aee48; continue;
 }
 0x20aee48 => {
  { let v: u32 = 0x216ac80u32; cpu.eax = v; }
  pc = 0x20aee54; continue;
 }
 0x20aee4f => {
  { let v: u32 = 0x216ac84u32; cpu.eax = v; }
  pc = 0x20aee54; continue;
 }
 0x20aee54 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = 0x0u32; cpu.push(v); }
  cpu.push(0x20aee5d); f_020bdb32(cpu); if cpu.returned_to != 0x20aee5d { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x18u32); cpu.esp = v; }
  pc = 0x20aee60; continue;
 }
 0x20aee60 => {
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
pub fn f_020aee67(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aee67); }
 let mut pc: u32 = 0x20aee67;
 loop { match pc {
 0x20aee67 => {
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
pub fn f_020aee8c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aee8c); }
 let mut pc: u32 = 0x20aee8c;
 loop { match pc {
 0x20aee8c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x217b93eu32); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20aeea9; continue; }
  pc = 0x20aee9b; continue;
 }
 0x20aee9b => {
  { cpu.sub32(cpu.edx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20aeea9; continue; }
  pc = 0x20aeea0; continue;
 }
 0x20aeea0 => {
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x2502u32; cpu.eax = v; }
  pc = 0x20aeeb3; continue;
 }
 0x20aeea9 => {
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) ^ (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(((cpu.eax >> 8) as u8), 0x35u8); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20aeeb3; continue;
 }
 0x20aeeb3 => {
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
pub fn f_020aeebe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aeebe); }
 let mut pc: u32 = 0x20aeebe;
 loop { match pc {
 0x20aeebe => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x2u32); }
  if (cpu.sf != cpu.of) { pc = 0x20aeee1; continue; }
  pc = 0x20aeece; continue;
 }
 0x20aeece => {
  { cpu.sub32(cpu.ebx, 0x8u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20aeee1; continue; }
  pc = 0x20aeed3; continue;
 }
 0x20aeed3 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = (cpu.eax as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x20aeee0); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20aeee1 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.flat_ds(0x20aeeea); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aeeeb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aeeeb); }
 let mut pc: u32 = 0x20aeeeb;
 loop { match pc {
 0x20aeeeb => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aeeed(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aeeed); }
 let mut pc: u32 = 0x20aeeed;
 loop { match pc {
 0x20aeeed => {
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
pub fn f_020aeef7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aeef7); }
 let mut pc: u32 = 0x20aeef7;
 loop { match pc {
 0x20aeef7 => {
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
pub fn f_020aeeff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aeeff); }
 let mut pc: u32 = 0x20aeeff;
 loop { match pc {
 0x20aeeff => {
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aef01(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aef01); }
 let mut pc: u32 = 0x20aef01;
 loop { match pc {
 0x20aef01 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x20aef13); f_020bdb48(cpu); if cpu.returned_to != 0x20aef13 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aef1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aef1a); }
 let mut pc: u32 = 0x20aef1a;
 loop { match pc {
 0x20aef1a => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x4u32.wrapping_add(cpu.edx))); }
  if !cpu.cf { pc = 0x20aef35; continue; }
  pc = 0x20aef29; continue;
 }
 0x20aef29 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.eax))); cpu.w32(0x10u32.wrapping_add(cpu.eax), v); }
  pc = 0x20aef35; continue;
 }
 0x20aef35 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aef39(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aef39); }
 let mut pc: u32 = 0x20aef39;
 loop { match pc {
 0x20aef39 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x20aef1au32; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20aef59); f_020af268(cpu); if cpu.returned_to != 0x20aef59 { return; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aef62(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aef62); }
 let mut pc: u32 = 0x20aef62;
 loop { match pc {
 0x20aef62 => {
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
  cpu.push(0x20aef82); f_020aef39(cpu); if cpu.returned_to != 0x20aef82 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020aef89(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20aef89); }
 let mut pc: u32 = 0x20aef89;
 loop { match pc {
 0x20aef89 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  cpu.push(0x20aefa4); f_020b1bf3(cpu); if cpu.returned_to != 0x20aefa4 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af03a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af03a); }
 let mut pc: u32 = 0x20af03a;
 loop { match pc {
 0x20af03a => {
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
  pc = 0x20af048; continue;
 }
 0x20af048 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20af06c; continue; }
  pc = 0x20af04e; continue;
 }
 0x20af04e => {
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
  cpu.poll(); pc = 0x20af048; continue;
 }
 0x20af06c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x43300000u32; cpu.edi = v; }
  cpu.push(0x20af078); f_020af085(cpu); if cpu.returned_to != 0x20af078 { return; }
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
pub fn f_020af085(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af085); }
 let mut pc: u32 = 0x20af085;
 loop { match pc {
 0x20af085 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi | cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.esi, cpu.edx); cpu.esi = v; }
  if cpu.zf { pc = 0x20af0d5; continue; }
  pc = 0x20af08d; continue;
 }
 0x20af08d => {
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if !cpu.zf { pc = 0x20af0a9; continue; }
  pc = 0x20af095; continue;
 }
 0x20af095 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edx); cpu.edx = v; }
  { cpu.and32(cpu.edx, 0xfff00000u32); }
  if cpu.zf { cpu.poll(); pc = 0x20af095; continue; }
  pc = 0x20af0a7; continue;
 }
 0x20af0a7 => {
  pc = 0x20af0cd; continue;
 }
 0x20af0a9 => {
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if cpu.zf { pc = 0x20af0cd; continue; }
  pc = 0x20af0b1; continue;
 }
 0x20af0b1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x100000u32); cpu.edi = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.rcr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.rcr32(cpu.esi, 1u32); cpu.esi = v; }
  { cpu.and32(cpu.edx, 0xffe00000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20af0b1; continue; }
  pc = 0x20af0c5; continue;
 }
 0x20af0c5 => {
  { let v: u32 = cpu.adc32(cpu.esi, cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  pc = 0x20af0cd; continue;
 }
 0x20af0cd => {
  { let v: u32 = cpu.edx & 0xfffffu32; cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.edx, cpu.edi); cpu.edx = v; }
  pc = 0x20af0d5; continue;
 }
 0x20af0d5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af0d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af0d6); }
 let mut pc: u32 = 0x20af0d6;
 loop { match pc {
 0x20af0d6 => {
  { let v: u32 = 0xdu32; cpu.eax = v; }
  f_020af0db(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af0db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af0db); }
 let mut pc: u32 = 0x20af0db;
 loop { match pc {
 0x20af0db => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x20af0e3); f_020b17b7(cpu); if cpu.returned_to != 0x20af0e3 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af0e7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af0e7); }
 let mut pc: u32 = 0x20af0e7;
 loop { match pc {
 0x20af0e7 => {
  { let v: u32 = 0xeu32; cpu.eax = v; }
  f_020af0db(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af0ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af0ee); }
 let mut pc: u32 = 0x20af0ee;
 loop { match pc {
 0x20af0ee => {
  { let v: u32 = 0x9u32; cpu.eax = v; }
  cpu.push(0x20af0f8); f_020af0db(cpu); if cpu.returned_to != 0x20af0f8 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af0fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af0fe); }
 let mut pc: u32 = 0x20af0fe;
 loop { match pc {
 0x20af0fe => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x20af106); f_020b17bd(cpu); if cpu.returned_to != 0x20af106 { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af10a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af10a); }
 let mut pc: u32 = 0x20af10a;
 loop { match pc {
 0x20af10a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if cpu.zf { pc = 0x20af130; continue; }
  pc = 0x20af11b; continue;
 }
 0x20af11b => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x20af12d); f_020af14b(cpu); if cpu.returned_to != 0x20af12d { return; }
  { let v = cpu.st(0); cpu.wf64(0x8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20af130; continue;
 }
 0x20af130 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20af13a); f_020bdf60(cpu); if cpu.returned_to != 0x20af13a { return; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20af143); f_020bdf26(cpu); if cpu.returned_to != 0x20af143 { return; }
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
pub fn f_020af14b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af14b); }
 let mut pc: u32 = 0x20af14b;
 loop { match pc {
 0x20af14b => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x216b6acu32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x20af176; continue; }
  pc = 0x20af166; continue;
 }
 0x20af166 => {
  { let v: u32 = cpu.r32(0x216b6bcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x216b6c0u32); cpu.eax = v; }
  pc = 0x20af193; continue;
 }
 0x20af176 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf64(0x216b6b4u32); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.zf { pc = 0x20af197; continue; }
  pc = 0x20af185; continue;
 }
 0x20af185 => {
  { let v: u32 = cpu.r32(0x216b6c4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x216b6c8u32); cpu.eax = v; }
  pc = 0x20af193; continue;
 }
 0x20af193 => {
  { let v: u32 = cpu.eax; cpu.w32(0x24u32.wrapping_add(cpu.esp), v); }
  pc = 0x20af197; continue;
 }
 0x20af197 => {
  { let v: u32 = 0x216b74cu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x20af1da; continue; }
  pc = 0x20af1a0; continue;
 }
 0x20af1a0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20af1f8; continue; }
  pc = 0x20af1ac; continue;
 }
 0x20af1ac => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x217ba10u32), 0x1u8); }
  if cpu.zf { pc = 0x20af1cc; continue; }
  pc = 0x20af1b9; continue;
 }
 0x20af1b9 => {
  { let v = cpu.r32(0x216b752u32); cpu.push(v); }
  { let v = cpu.r32(0x216b74eu32); cpu.push(v); }
  cpu.push(0x20af1ca); f_020b314c(cpu); if cpu.returned_to != 0x20af1ca { return; }
  pc = 0x20af1d2; continue;
 }
 0x20af1cc => {
  { let v = cpu.st(0) / cpu.rf64(0x216b74eu32); cpu.set_st(0, v); }
  pc = 0x20af1d2; continue;
 }
 0x20af1d2 => {
  { let v: u32 = cpu.edx.wrapping_sub(0xd8u32); cpu.edx = v; }
  pc = 0x20af1f4; continue;
 }
 0x20af1da => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { cpu.sub32(cpu.ecx, 0x134u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20af1f8; continue; }
  pc = 0x20af1e4; continue;
 }
 0x20af1e4 => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x216b74eu32); cpu.set_st(0, v); }
  { let v: u32 = 0xffffff28u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  pc = 0x20af1f4; continue;
 }
 0x20af1f4 => {
  { let v = cpu.st(0); cpu.wf64(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x20af1f8; continue;
 }
 0x20af1f8 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x3ff00000u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20af206; continue;
 }
 0x20af206 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x20af218; continue; }
  pc = 0x20af20d; continue;
 }
 0x20af20d => {
  { let v = cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x2u32.wrapping_add(cpu.eax)); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  pc = 0x20af218; continue;
 }
 0x20af218 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20af229; continue; }
  pc = 0x20af21c; continue;
 }
 0x20af21c => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.eax)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x1u32); }
  if cpu.zf { cpu.poll(); pc = 0x20af206; continue; }
  pc = 0x20af224; continue;
 }
 0x20af224 => {
  { let v: u32 = cpu.eax.wrapping_add(0xau32); cpu.eax = v; }
  cpu.poll(); pc = 0x20af206; continue;
 }
 0x20af229 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x20af24e; continue; }
  pc = 0x20af22d; continue;
 }
 0x20af22d => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { cpu.and8(cpu.r8(0x217ba10u32), 0x1u8); }
  if cpu.zf { pc = 0x20af249; continue; }
  pc = 0x20af23a; continue;
 }
 0x20af23a => {
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.push(v); }
  cpu.push(0x20af247); f_020b314c(cpu); if cpu.returned_to != 0x20af247 { return; }
  pc = 0x20af255; continue;
 }
 0x20af249 => {
  { let v = cpu.st(0) / cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x20af255; continue;
 }
 0x20af24e => {
  { let v = cpu.rf64(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf64(0x0u32.wrapping_add(cpu.esp)); cpu.set_st(0, v); }
  pc = 0x20af255; continue;
 }
 0x20af255 => {
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
pub fn f_020af268(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af268); }
 let mut pc: u32 = 0x20af268;
 loop { match pc {
 0x20af268 => {
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
  pc = 0x20af33a; continue;
 }
 0x20af2a7 => {
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x20af2b5); f_020af607(cpu); if cpu.returned_to != 0x20af2b5 { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x115u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20af5f5; continue; }
  pc = 0x20af2c9; continue;
 }
 0x20af2c9 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x20af404; continue; }
  pc = 0x20af2db; continue;
 }
 0x20af2db => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20af371; continue; }
  pc = 0x20af303; continue;
 }
 0x20af303 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20af321; continue; }
  pc = 0x20af30b; continue;
 }
 0x20af30b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x20af315; continue;
 }
 0x20af315 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20af33a; continue;
 }
 0x20af321 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20af366; continue; }
  pc = 0x20af328; continue;
 }
 0x20af328 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x20af331; continue;
 }
 0x20af331 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af33a; continue;
 }
 0x20af33a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x20af5f5; continue; }
  pc = 0x20af344; continue;
 }
 0x20af344 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x25u32); }
  if cpu.zf { cpu.poll(); pc = 0x20af2a7; continue; }
  pc = 0x20af354; continue;
 }
 0x20af354 => {
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af364); cpu.call(t); if cpu.returned_to != 0x20af364 { return; }
  cpu.poll(); pc = 0x20af33a; continue;
 }
 0x20af366 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  cpu.poll(); pc = 0x20af331; continue;
 }
 0x20af371 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20af3cc; continue; }
  pc = 0x20af383; continue;
 }
 0x20af383 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20af3a2; continue; }
  pc = 0x20af38b; continue;
 }
 0x20af38b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.eax); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  cpu.poll(); pc = 0x20af33a; continue;
 }
 0x20af3a2 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20af3c1; continue; }
  pc = 0x20af3a9; continue;
 }
 0x20af3a9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  pc = 0x20af3b2; continue;
 }
 0x20af3b2 => {
  { let v: u32 = cpu.r32(0x110u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx), v); }
  cpu.poll(); pc = 0x20af33a; continue;
 }
 0x20af3c1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.poll(); pc = 0x20af3b2; continue;
 }
 0x20af3cc => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20af3e1; continue; }
  pc = 0x20af3d2; continue;
 }
 0x20af3d2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  cpu.poll(); pc = 0x20af315; continue;
 }
 0x20af3e1 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20af3f6; continue; }
  pc = 0x20af3e8; continue;
 }
 0x20af3e8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  cpu.poll(); pc = 0x20af331; continue;
 }
 0x20af3f6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebp); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  cpu.poll(); pc = 0x20af331; continue;
 }
 0x20af404 => {
  { let v: u32 = 0x120u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.ebx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x20af41b); f_020af945(cpu); if cpu.returned_to != 0x20af41b { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x20af467; continue; }
  pc = 0x20af42f; continue;
 }
 0x20af42f => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x20af467; continue; }
  pc = 0x20af43b; continue;
 }
 0x20af43b => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x20af467; continue; }
  pc = 0x20af44e; continue;
 }
 0x20af44e => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af465); cpu.call(t); if cpu.returned_to != 0x20af465 { return; }
  cpu.poll(); pc = 0x20af43b; continue;
 }
 0x20af467 => {
  { let v: u32 = 0x117u32.wrapping_add(cpu.esp); cpu.ecx = v; }
  pc = 0x20af46e; continue;
 }
 0x20af46e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if cpu.zf { pc = 0x20af489; continue; }
  pc = 0x20af474; continue;
 }
 0x20af474 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af487); cpu.call(t); if cpu.returned_to != 0x20af487 { return; }
  cpu.poll(); pc = 0x20af46e; continue;
 }
 0x20af489 => {
  { let v: u32 = cpu.r32(0x10cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10cu32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20af4b3; continue; }
  pc = 0x20af49e; continue;
 }
 0x20af49e => {
  { let v: u32 = 0x30u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af4b1); cpu.call(t); if cpu.returned_to != 0x20af4b1 { return; }
  cpu.poll(); pc = 0x20af489; continue;
 }
 0x20af4b3 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x20af4fb; continue; }
  pc = 0x20af4c3; continue;
 }
 0x20af4c3 => {
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x20u32); }
  if cpu.zf { pc = 0x20af4fb; continue; }
  pc = 0x20af4cf; continue;
 }
 0x20af4cf => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x20af4fb; continue; }
  pc = 0x20af4e2; continue;
 }
 0x20af4e2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x116u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af4f9); cpu.call(t); if cpu.returned_to != 0x20af4f9 { return; }
  cpu.poll(); pc = 0x20af4cf; continue;
 }
 0x20af4fb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x115u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x73u32); }
  if cpu.zf { pc = 0x20af50e; continue; }
  pc = 0x20af509; continue;
 }
 0x20af509 => {
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x20af57a; continue; }
  pc = 0x20af50e; continue;
 }
 0x20af50e => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20af54f; continue; }
  pc = 0x20af51e; continue;
 }
 0x20af51e => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20af5a5; continue; }
  pc = 0x20af537; continue;
 }
 0x20af537 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af54a); cpu.call(t); if cpu.returned_to != 0x20af54a { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x20af51e; continue;
 }
 0x20af54f => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20af5a5; continue; }
  pc = 0x20af564; continue;
 }
 0x20af564 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af577); cpu.call(t); if cpu.returned_to != 0x20af577 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20af54f; continue;
 }
 0x20af57a => {
  { let v: u32 = cpu.r32(0x108u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x108u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20af5a5; continue; }
  pc = 0x20af58f; continue;
 }
 0x20af58f => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af5a2); cpu.call(t); if cpu.returned_to != 0x20af5a2 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20af57a; continue;
 }
 0x20af5a5 => {
  { let v: u8 = cpu.r8(0x114u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x8u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x20af33a; continue; }
  pc = 0x20af5b9; continue;
 }
 0x20af5b9 => {
  { cpu.sub32(cpu.r32(0x104u32.wrapping_add(cpu.esp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x20af33a; continue; }
  pc = 0x20af5c7; continue;
 }
 0x20af5c7 => {
  { let v: u32 = cpu.r32(0x104u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x104u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x20af33a; continue; }
  pc = 0x20af5e0; continue;
 }
 0x20af5e0 => {
  { let v: u32 = 0x20u32; cpu.edx = v; }
  { let v: u32 = 0x100u32.wrapping_add(cpu.esp); cpu.eax = v; }
  let t = cpu.r32(0x11cu32.wrapping_add(cpu.esp)); cpu.push(0x20af5f3); cpu.call(t); if cpu.returned_to != 0x20af5f3 { return; }
  cpu.poll(); pc = 0x20af5c7; continue;
 }
 0x20af5f5 => {
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
pub fn f_020af607(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af607); }
 let mut pc: u32 = 0x20af607;
 loop { match pc {
 0x20af607 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20af61b); f_020af745(cpu); if cpu.returned_to != 0x20af61b { return; }
  { let v: u32 = 0x0u32; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x20af655; continue; }
  pc = 0x20af62f; continue;
 }
 0x20af62f => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20af655; continue; }
  pc = 0x20af634; continue;
 }
 0x20af634 => {
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
  if (cpu.sf != cpu.of) { pc = 0x20af680; continue; }
  pc = 0x20af64e; continue;
 }
 0x20af64e => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x20af634; continue; }
  pc = 0x20af653; continue;
 }
 0x20af653 => {
  pc = 0x20af680; continue;
 }
 0x20af655 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x20af680; continue; }
  pc = 0x20af65e; continue;
 }
 0x20af65e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20af67f; continue; }
  pc = 0x20af66f; continue;
 }
 0x20af66f => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  { let v: u8 = (cpu.edx as u8) | 0x8u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20af67f; continue;
 }
 0x20af67f => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20af680; continue;
 }
 0x20af680 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x20af6d9; continue; }
  pc = 0x20af690; continue;
 }
 0x20af690 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x20af6bc; continue; }
  pc = 0x20af6a1; continue;
 }
 0x20af6a1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20af6b9; continue; }
  pc = 0x20af6b2; continue;
 }
 0x20af6b2 => {
  { let v: u32 = 0xffffffffu32; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20af6b9; continue;
 }
 0x20af6b9 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20af6d9; continue;
 }
 0x20af6bc => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x20af6d9; continue; }
  pc = 0x20af6c5; continue;
 }
 0x20af6c5 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20af6d9; continue; }
  pc = 0x20af6ca; continue;
 }
 0x20af6ca => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0xau32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x30u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.ebx), v); }
  cpu.poll(); pc = 0x20af6bc; continue;
 }
 0x20af6d9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8((cpu.edx as u8), 0x4eu8); }
  if cpu.cf { pc = 0x20af6fe; continue; }
  pc = 0x20af6e3; continue;
 }
 0x20af6e3 => {
  if (cpu.cf || cpu.zf) { pc = 0x20af73b; continue; }
  pc = 0x20af6e5; continue;
 }
 0x20af6e5 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x20af6f5; continue; }
  pc = 0x20af6ea; continue;
 }
 0x20af6ea => {
  if (cpu.cf || cpu.zf) { pc = 0x20af70e; continue; }
  pc = 0x20af6ec; continue;
 }
 0x20af6ec => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x20af70e; continue; }
  pc = 0x20af6f1; continue;
 }
 0x20af6f1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20af6f5 => {
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.zf { pc = 0x20af71c; continue; }
  pc = 0x20af6fa; continue;
 }
 0x20af6fa => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20af6fe => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x20af741; continue; }
  pc = 0x20af703; continue;
 }
 0x20af703 => {
  if (cpu.cf || cpu.zf) { pc = 0x20af731; continue; }
  pc = 0x20af705; continue;
 }
 0x20af705 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x20af722; continue; }
  pc = 0x20af70a; continue;
 }
 0x20af70a => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20af70e => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x20u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20af715; continue;
 }
 0x20af715 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20af71c => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x10u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20af73f; continue;
 }
 0x20af722 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8((cpu.edx as u8), 0x40u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20af731 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.or8(((cpu.edx >> 8) as u8), 0x80u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.poll(); pc = 0x20af715; continue;
 }
 0x20af73b => {
  { let v: u8 = cpu.or8(cpu.r8(0x14u32.wrapping_add(cpu.ebx)), 0x40u8); cpu.w8(0x14u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20af73f; continue;
 }
 0x20af73f => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20af741; continue;
 }
 0x20af741 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af745(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af745); }
 let mut pc: u32 = 0x20af745;
 loop { match pc {
 0x20af745 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af74d; continue;
 }
 0x20af74d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x20af75c; continue; }
  pc = 0x20af756; continue;
 }
 0x20af756 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x8u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af79c; continue;
 }
 0x20af75c => {
  { cpu.sub32(cpu.eax, 0x23u32); }
  if !cpu.zf { pc = 0x20af767; continue; }
  pc = 0x20af761; continue;
 }
 0x20af761 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x1u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af79c; continue;
 }
 0x20af767 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x20af77c; continue; }
  pc = 0x20af76c; continue;
 }
 0x20af76c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af79c; continue;
 }
 0x20af77c => {
  { cpu.sub32(cpu.eax, 0x20u32); }
  if !cpu.zf { pc = 0x20af793; continue; }
  pc = 0x20af781; continue;
 }
 0x20af781 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x20af79c; continue; }
  pc = 0x20af78d; continue;
 }
 0x20af78d => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.edx)) | 0x2u8; cpu.w8(0x14u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af79c; continue;
 }
 0x20af793 => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x20af79f; continue; }
  pc = 0x20af798; continue;
 }
 0x20af798 => {
  { let v: u8 = 0x30u8; cpu.w8(0x16u32.wrapping_add(cpu.edx), v); }
  pc = 0x20af79c; continue;
 }
 0x20af79c => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20af74d; continue;
 }
 0x20af79f => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af7a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20af7a4); } r_020af7a4(cpu, 0x20af7a4); }
pub fn f_020af7c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20af7c8); } r_020af7a4(cpu, 0x20af7c8); }
fn r_020af7a4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20af7a4 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20af7b2; continue;
 }
 0x20af7b2 => {
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.ebx as u8), (cpu.ebx as u8)); }
  if cpu.zf { pc = 0x20af7c3; continue; }
  pc = 0x20af7bc; continue;
 }
 0x20af7bc => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x20af7c3; continue; }
  pc = 0x20af7c0; continue;
 }
 0x20af7c0 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20af7b2; continue;
 }
 0x20af7c3 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20af7c8 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20af7d6; continue;
 }
 0x20af7d6 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { cpu.and16((cpu.edi as u16), (cpu.edi as u16)); }
  if cpu.zf { cpu.poll(); pc = 0x20af7c3; continue; }
  pc = 0x20af7e4; continue;
 }
 0x20af7e4 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { cpu.poll(); pc = 0x20af7c3; continue; }
  pc = 0x20af7e8; continue;
 }
 0x20af7e8 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.poll(); pc = 0x20af7d6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af7eb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af7eb); }
 let mut pc: u32 = 0x20af7eb;
 loop { match pc {
 0x20af7eb => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  cpu.push(0x20af803); f_020add1c(cpu); if cpu.returned_to != 0x20af803 { return; }
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
  pc = 0x20af828; continue;
 }
 0x20af821 => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x20af828; continue;
 }
 0x20af828 => {
  { cpu.sub32(cpu.edx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x20af821; continue; }
  pc = 0x20af82c; continue;
 }
 0x20af82c => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edx = v; }
  pc = 0x20af82f; continue;
 }
 0x20af82f => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x20af83a; continue; }
  pc = 0x20af833; continue;
 }
 0x20af833 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x20af82f; continue;
 }
 0x20af83a => {
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
pub fn f_020af849(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af849); }
 let mut pc: u32 = 0x20af849;
 loop { match pc {
 0x20af849 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20af865; continue; }
  pc = 0x20af85a; continue;
 }
 0x20af85a => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20af865; continue;
 }
 0x20af865 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if !cpu.zf { pc = 0x20af872; continue; }
  pc = 0x20af86b; continue;
 }
 0x20af86b => {
  { let v: u32 = 0x4u32; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x20af872; continue;
 }
 0x20af872 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20af885); f_020add1c(cpu); if cpu.returned_to != 0x20af885 { return; }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  pc = 0x20af887; continue;
 }
 0x20af887 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x20af894; continue; }
  pc = 0x20af890; continue;
 }
 0x20af890 => {
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  cpu.poll(); pc = 0x20af887; continue;
 }
 0x20af894 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x20af8c6; continue; }
  pc = 0x20af89a; continue;
 }
 0x20af89a => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  pc = 0x20af8be; continue;
 }
 0x20af8a3 => {
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
  pc = 0x20af8be; continue;
 }
 0x20af8be => {
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20af8a3; continue; }
  pc = 0x20af8c3; continue;
 }
 0x20af8c3 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20af8c6; continue;
 }
 0x20af8c6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = (cpu.eax as u8) ^ (cpu.eax as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0x80u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20af937; continue; }
  pc = 0x20af8d9; continue;
 }
 0x20af8d9 => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if !cpu.zf { pc = 0x20af916; continue; }
  pc = 0x20af8dd; continue;
 }
 0x20af8dd => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u8 = 0x31u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20af8e3; continue;
 }
 0x20af8e3 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ecx); cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if !cpu.zf { pc = 0x20af8f3; continue; }
  pc = 0x20af8ef; continue;
 }
 0x20af8ef => {
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  cpu.poll(); pc = 0x20af8e3; continue;
 }
 0x20af8f3 => {
  { cpu.sub32(cpu.edx, 0x2eu32); }
  if !cpu.zf { pc = 0x20af90d; continue; }
  pc = 0x20af8f8; continue;
 }
 0x20af8f8 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.eax), v); }
  pc = 0x20af901; continue;
 }
 0x20af901 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x20af90d; continue; }
  pc = 0x20af90a; continue;
 }
 0x20af90a => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x20af901; continue;
 }
 0x20af90d => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20af937; continue;
 }
 0x20af916 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x20af921; continue; }
  pc = 0x20af920; continue;
 }
 0x20af920 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x20af921; continue;
 }
 0x20af921 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x39u32); }
  if cpu.zf { pc = 0x20af932; continue; }
  pc = 0x20af92a; continue;
 }
 0x20af92a => {
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20af937; continue;
 }
 0x20af932 => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx), v); }
  cpu.poll(); pc = 0x20af8d9; continue;
 }
 0x20af937 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af93e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af93e); }
 let mut pc: u32 = 0x20af93e;
 loop { match pc {
 0x20af93e => {
  let t = cpu.r32(0x217c85cu32); cpu.push(0x20af944); cpu.call(t); if cpu.returned_to != 0x20af944 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020af945(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20af945); }
 let mut pc: u32 = 0x20af945;
 loop { match pc {
 0x20af945 => {
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
  if cpu.cf { pc = 0x20af98e; continue; }
  pc = 0x20af972; continue;
 }
 0x20af972 => {
  if (cpu.cf || cpu.zf) { pc = 0x20af9df; continue; }
  pc = 0x20af978; continue;
 }
 0x20af978 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x20af989; continue; }
  pc = 0x20af97d; continue;
 }
 0x20af97d => {
  if (cpu.cf || cpu.zf) { pc = 0x20af99c; continue; }
  pc = 0x20af97f; continue;
 }
 0x20af97f => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  pc = 0x20af982; continue;
 }
 0x20af982 => {
  if cpu.zf { pc = 0x20af99c; continue; }
  pc = 0x20af984; continue;
 }
 0x20af984 => {
  pc = 0x20af9e9; continue;
 }
 0x20af989 => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  cpu.poll(); pc = 0x20af982; continue;
 }
 0x20af98e => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x20af9e9; continue; }
  pc = 0x20af993; continue;
 }
 0x20af993 => {
  if (cpu.cf || cpu.zf) { pc = 0x20af99c; continue; }
  pc = 0x20af995; continue;
 }
 0x20af995 => {
  { cpu.sub8((cpu.edx as u8), 0x64u8); }
  if cpu.zf { pc = 0x20af9df; continue; }
  pc = 0x20af99a; continue;
 }
 0x20af99a => {
  pc = 0x20af9e9; continue;
 }
 0x20af99c => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20af9b6; continue; }
  pc = 0x20af9aa; continue;
 }
 0x20af9aa => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  pc = 0x20af9db; continue;
 }
 0x20af9b6 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20af9d1; continue; }
  pc = 0x20af9c4; continue;
 }
 0x20af9c4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as u32; cpu.ebp = v; }
  pc = 0x20af9db; continue;
 }
 0x20af9d1 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x20af9db; continue;
 }
 0x20af9db => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)) & 0xf9u8; cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x20af9df; continue;
 }
 0x20af9df => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0xffffffffu32); }
  if cpu.zf { pc = 0x20af9e9; continue; }
  pc = 0x20af9e5; continue;
 }
 0x20af9e5 => {
  { let v: u8 = 0x20u8; cpu.w8(0x16u32.wrapping_add(cpu.esi), v); }
  pc = 0x20af9e9; continue;
 }
 0x20af9e9 => {
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { cpu.sub8((cpu.edx as u8), 0x65u8); }
  if cpu.cf { pc = 0x20afa56; continue; }
  pc = 0x20af9f6; continue;
 }
 0x20af9f6 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afad6; continue; }
  pc = 0x20af9fc; continue;
 }
 0x20af9fc => {
  { cpu.sub8((cpu.edx as u8), 0x6fu8); }
  if cpu.cf { pc = 0x20afa3d; continue; }
  pc = 0x20afa01; continue;
 }
 0x20afa01 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afaec; continue; }
  pc = 0x20afa07; continue;
 }
 0x20afa07 => {
  { cpu.sub8((cpu.edx as u8), 0x73u8); }
  if cpu.cf { pc = 0x20afa2f; continue; }
  pc = 0x20afa0c; continue;
 }
 0x20afa0c => {
  if (cpu.cf || cpu.zf) { pc = 0x20afb81; continue; }
  pc = 0x20afa12; continue;
 }
 0x20afa12 => {
  { cpu.sub8((cpu.edx as u8), 0x75u8); }
  if cpu.cf { pc = 0x20afd53; continue; }
  pc = 0x20afa1b; continue;
 }
 0x20afa1b => {
  if (cpu.cf || cpu.zf) { pc = 0x20afc6d; continue; }
  pc = 0x20afa21; continue;
 }
 0x20afa21 => {
  { cpu.sub8((cpu.edx as u8), 0x78u8); }
  if cpu.zf { pc = 0x20afc4a; continue; }
  pc = 0x20afa2a; continue;
 }
 0x20afa2a => {
  pc = 0x20afd53; continue;
 }
 0x20afa2f => {
  { cpu.sub8((cpu.edx as u8), 0x70u8); }
  if cpu.zf { pc = 0x20afc90; continue; }
  pc = 0x20afa38; continue;
 }
 0x20afa38 => {
  pc = 0x20afd53; continue;
 }
 0x20afa3d => {
  { cpu.sub8((cpu.edx as u8), 0x67u8); }
  if cpu.cf { pc = 0x20afaa4; continue; }
  pc = 0x20afa42; continue;
 }
 0x20afa42 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afad6; continue; }
  pc = 0x20afa48; continue;
 }
 0x20afa48 => {
  { cpu.sub8((cpu.edx as u8), 0x69u8); }
  if cpu.zf { pc = 0x20afb30; continue; }
  pc = 0x20afa51; continue;
 }
 0x20afa51 => {
  pc = 0x20afd53; continue;
 }
 0x20afa56 => {
  { cpu.sub8((cpu.edx as u8), 0x50u8); }
  if cpu.cf { pc = 0x20afa8e; continue; }
  pc = 0x20afa5b; continue;
 }
 0x20afa5b => {
  if (cpu.cf || cpu.zf) { pc = 0x20afc90; continue; }
  pc = 0x20afa61; continue;
 }
 0x20afa61 => {
  { cpu.sub8((cpu.edx as u8), 0x58u8); }
  if cpu.cf { pc = 0x20afa80; continue; }
  pc = 0x20afa66; continue;
 }
 0x20afa66 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afc4a; continue; }
  pc = 0x20afa6c; continue;
 }
 0x20afa6c => {
  { cpu.sub8((cpu.edx as u8), 0x63u8); }
  if cpu.cf { pc = 0x20afd53; continue; }
  pc = 0x20afa75; continue;
 }
 0x20afa75 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afd27; continue; }
  pc = 0x20afa7b; continue;
 }
 0x20afa7b => {
  pc = 0x20afb30; continue;
 }
 0x20afa80 => {
  { cpu.sub8((cpu.edx as u8), 0x53u8); }
  if cpu.zf { pc = 0x20afb81; continue; }
  pc = 0x20afa89; continue;
 }
 0x20afa89 => {
  pc = 0x20afd53; continue;
 }
 0x20afa8e => {
  { cpu.sub8((cpu.edx as u8), 0x46u8); }
  if cpu.cf { pc = 0x20afa9f; continue; }
  pc = 0x20afa93; continue;
 }
 0x20afa93 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afaa4; continue; }
  pc = 0x20afa95; continue;
 }
 0x20afa95 => {
  { cpu.sub8((cpu.edx as u8), 0x47u8); }
  pc = 0x20afa98; continue;
 }
 0x20afa98 => {
  if cpu.zf { pc = 0x20afad6; continue; }
  pc = 0x20afa9a; continue;
 }
 0x20afa9a => {
  pc = 0x20afd53; continue;
 }
 0x20afa9f => {
  { cpu.sub8((cpu.edx as u8), 0x45u8); }
  cpu.poll(); pc = 0x20afa98; continue;
 }
 0x20afaa4 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afad6; continue; }
  pc = 0x20afab2; continue;
 }
 0x20afab2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  cpu.push(0x20afac5); f_020af849(cpu); if cpu.returned_to != 0x20afac5 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  pc = 0x20afb26; continue;
 }
 0x20afad6 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20afae1); f_020af93e(cpu); if cpu.returned_to != 0x20afae1 { return; }
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x20afb26; continue;
 }
 0x20afaec => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20afb01; continue; }
  pc = 0x20afafd; continue;
 }
 0x20afafd => {
  { let v: u8 = 0x30u8; cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x20afb01; continue;
 }
 0x20afb01 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20afb0a); f_020be16d(cpu); if cpu.returned_to != 0x20afb0a { return; }
  pc = 0x20afb0a; continue;
 }
 0x20afb0a => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x20afb1d; continue; }
  pc = 0x20afb10; continue;
 }
 0x20afb10 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if !cpu.zf { pc = 0x20afb1d; continue; }
  pc = 0x20afb19; continue;
 }
 0x20afb19 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20afb1d; continue;
 }
 0x20afb1d => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20afb26; continue;
 }
 0x20afb26 => {
  cpu.push(0x20afb2b); f_020af7a4(cpu); if cpu.returned_to != 0x20afb2b { return; }
  pc = 0x20afd7c; continue;
 }
 0x20afb30 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afb51; continue; }
  pc = 0x20afb3e; continue;
 }
 0x20afb3e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  pc = 0x20afb4a; continue;
 }
 0x20afb4a => {
  cpu.push(0x20afb4f); f_020be16d(cpu); if cpu.returned_to != 0x20afb4f { return; }
  cpu.poll(); pc = 0x20afb0a; continue;
 }
 0x20afb51 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x10u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afb6e; continue; }
  pc = 0x20afb5f; continue;
 }
 0x20afb5f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)) as i16 as i32 as u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20afb4a; continue;
 }
 0x20afb6e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  cpu.push(0x20afb7f); f_020add1c(cpu); if cpu.returned_to != 0x20afb7f { return; }
  cpu.poll(); pc = 0x20afb0a; continue;
 }
 0x20afb81 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afbb2; continue; }
  pc = 0x20afb8f; continue;
 }
 0x20afb8f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20afba6; continue; }
  pc = 0x20afba1; continue;
 }
 0x20afba1 => {
  { cpu.and16((cpu.edx as u16), (cpu.edx as u16)); }
  if cpu.zf { pc = 0x20afbaa; continue; }
  pc = 0x20afba6; continue;
 }
 0x20afba6 => {
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  pc = 0x20afbe8; continue;
 }
 0x20afbaa => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x20afbe6; continue;
 }
 0x20afbb2 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x40u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afbd2; continue; }
  pc = 0x20afbc0; continue;
 }
 0x20afbc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { cpu.poll(); pc = 0x20afbaa; continue; }
  pc = 0x20afbce; continue;
 }
 0x20afbce => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  pc = 0x20afbe6; continue;
 }
 0x20afbd2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x20afbce; continue; }
  pc = 0x20afbe0; continue;
 }
 0x20afbe0 => {
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  pc = 0x20afbe6; continue;
 }
 0x20afbe6 => {
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  pc = 0x20afbe8; continue;
 }
 0x20afbe8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8) & 0xf9u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  { cpu.sub32(cpu.eax, 0x53u32); }
  if !cpu.zf { pc = 0x20afc22; continue; }
  pc = 0x20afc00; continue;
 }
 0x20afc00 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20afc19; continue; }
  pc = 0x20afc0b; continue;
 }
 0x20afc0b => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  pc = 0x20afd7c; continue;
 }
 0x20afc19 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  pc = 0x20afd7c; continue;
 }
 0x20afc22 => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20afc3e; continue; }
  pc = 0x20afc2d; continue;
 }
 0x20afc2d => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.push(0x20afc39); f_020af7c8(cpu); if cpu.returned_to != 0x20afc39 { return; }
  pc = 0x20afd7c; continue;
 }
 0x20afc3e => {
  { let v: u32 = (cpu.segments[0] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  cpu.poll(); pc = 0x20afb26; continue;
 }
 0x20afc4a => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20afc68; continue; }
  pc = 0x20afc56; continue;
 }
 0x20afc56 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20afc68; continue; }
  pc = 0x20afc5a; continue;
 }
 0x20afc5a => {
  { let v: u8 = 0x30u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x19u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afc68; continue;
 }
 0x20afc68 => {
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  pc = 0x20afc6d; continue;
 }
 0x20afc6d => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20afc76); f_020be123(cpu); if cpu.returned_to != 0x20afc76 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20afb0a; continue; }
  pc = 0x20afc84; continue;
 }
 0x20afc84 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20afc8b); f_020afe2e(cpu); if cpu.returned_to != 0x20afc8b { return; }
  cpu.poll(); pc = 0x20afb0a; continue;
 }
 0x20afc90 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esi)), 0x0u32); }
  if !cpu.zf { pc = 0x20afcb4; continue; }
  pc = 0x20afc96; continue;
 }
 0x20afc96 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afcad; continue; }
  pc = 0x20afca4; continue;
 }
 0x20afca4 => {
  { let v: u32 = 0xdu32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afcb4; continue;
 }
 0x20afcad => {
  { let v: u32 = 0x8u32; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afcb4; continue;
 }
 0x20afcb4 => {
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
  if cpu.zf { pc = 0x20afcff; continue; }
  pc = 0x20afcd6; continue;
 }
 0x20afcd6 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  cpu.push(0x20afcef); f_020af7eb(cpu); if cpu.returned_to != 0x20afcef { return; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = 0x5u32.wrapping_add(cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x20afd08; continue;
 }
 0x20afcff => {
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  pc = 0x20afd08; continue;
 }
 0x20afd08 => {
  cpu.push(0x20afd0d); f_020af7eb(cpu); if cpu.returned_to != 0x20afd0d { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x50u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20afb1d; continue; }
  pc = 0x20afd1b; continue;
 }
 0x20afd1b => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20afd22); f_020afe2e(cpu); if cpu.returned_to != 0x20afd22 { return; }
  cpu.poll(); pc = 0x20afb1d; continue;
 }
 0x20afd27 => {
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
  pc = 0x20afd7c; continue;
 }
 0x20afd53 => {
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
  pc = 0x20afd7c; continue;
 }
 0x20afd7c => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20afdd1; continue; }
  pc = 0x20afd82; continue;
 }
 0x20afd82 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0])); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2du32); }
  if !cpu.zf { pc = 0x20afd94; continue; }
  pc = 0x20afd8c; continue;
 }
 0x20afd8c => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x2du8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x20afdba; continue;
 }
 0x20afd94 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afda8; continue; }
  pc = 0x20afda2; continue;
 }
 0x20afda2 => {
  { let v: u8 = 0x20u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afdba; continue;
 }
 0x20afda8 => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x4u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20afdbe; continue; }
  pc = 0x20afdb6; continue;
 }
 0x20afdb6 => {
  { let v: u8 = 0x2bu8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afdba; continue;
 }
 0x20afdba => {
  { let v: u8 = 0x0u8; cpu.w8(0x18u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afdbe; continue;
 }
 0x20afdbe => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20afdca; continue; }
  pc = 0x20afdc5; continue;
 }
 0x20afdc5 => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afdd1; continue;
 }
 0x20afdca => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  pc = 0x20afdd1; continue;
 }
 0x20afdd1 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x16u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x2au32); }
  if !cpu.zf { pc = 0x20afde8; continue; }
  pc = 0x20afddb; continue;
 }
 0x20afddb => {
  { let v: u8 = cpu.r8(0x14u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = 0x0u8; cpu.w8(0x17u32.wrapping_add(cpu.esi), v); }
  { let v: u8 = (cpu.edx as u8) & 0xf9u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x14u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afde8; continue;
 }
 0x20afde8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20afdf4; continue; }
  pc = 0x20afdf0; continue;
 }
 0x20afdf0 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20afe01; continue; }
  pc = 0x20afdf4; continue;
 }
 0x20afdf4 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x15u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x63u32); }
  if cpu.zf { pc = 0x20afe01; continue; }
  pc = 0x20afdfe; continue;
 }
 0x20afdfe => {
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  pc = 0x20afe01; continue;
 }
 0x20afe01 => {
  { let v: u32 = 0xffffffffu32; cpu.ebx = v; }
  { let v: u32 = 0x17u32.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  cpu.push(0x20afe10); f_020af7a4(cpu); if cpu.returned_to != 0x20afe10 { return; }
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
pub fn f_020afe2e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20afe2e); }
 let mut pc: u32 = 0x20afe2e;
 loop { match pc {
 0x20afe2e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x20afe31; continue;
 }
 0x20afe31 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.edx)), 0x0u8); }
  if cpu.zf { pc = 0x20afe44; continue; }
  pc = 0x20afe36; continue;
 }
 0x20afe36 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.push(0x20afe3f); f_020a3a84(cpu); if cpu.returned_to != 0x20afe3f { return; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.poll(); pc = 0x20afe31; continue;
 }
 0x20afe44 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020afe50(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20afe50); }
 let mut pc: u32 = 0x20afe50;
 loop { match pc {
 0x20afe50 => {
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
pub fn f_020afe69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20afe69); }
 let mut pc: u32 = 0x20afe69;
 loop { match pc {
 0x20afe69 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.push(0x20afe6e); cpu.call(t); if cpu.returned_to != 0x20afe6e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020afe70(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20afe70); }
 let mut pc: u32 = 0x20afe70;
 loop { match pc {
 0x20afe70 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  let t = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.push(0x20afe76); cpu.call(t); if cpu.returned_to != 0x20afe76 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020afe78(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20afe78); }
 let mut pc: u32 = 0x20afe78;
 loop { match pc {
 0x20afe78 => {
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
  pc = 0x20afe97; continue;
 }
 0x20afe97 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.ebx, 0xffu32); cpu.ebx = v; }
  if cpu.zf { pc = 0x20b009f; continue; }
  pc = 0x20afeae; continue;
 }
 0x20afeae => {
  { let v: u8 = (cpu.ebx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20afed4; continue; }
  pc = 0x20afec6; continue;
 }
 0x20afec6 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20afecd); f_020b01b4(cpu); if cpu.returned_to != 0x20afecd { return; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  pc = 0x20b005f; continue;
 }
 0x20afed4 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x20afefe; continue; }
  pc = 0x20afed9; continue;
 }
 0x20afed9 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20afee0); f_020afe69(cpu); if cpu.returned_to != 0x20afee0 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x20b005e; continue; }
  pc = 0x20afee8; continue;
 }
 0x20afee8 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x20b009f; continue; }
  pc = 0x20afef2; continue;
 }
 0x20afef2 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x20afef9); f_020afe70(cpu); if cpu.returned_to != 0x20afef9 { return; }
  pc = 0x20b009f; continue;
 }
 0x20afefe => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20aff07); f_020b00bf(cpu); if cpu.returned_to != 0x20aff07 { return; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20aff16; continue; }
  pc = 0x20aff12; continue;
 }
 0x20aff12 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20aff16; continue;
 }
 0x20aff16 => {
  { cpu.sub32(cpu.ebx, 0x65u32); }
  if cpu.cf { pc = 0x20aff70; continue; }
  pc = 0x20aff1b; continue;
 }
 0x20aff1b => {
  { cpu.sub32(cpu.ebx, 0x67u32); }
  if (cpu.cf || cpu.zf) { pc = 0x20afffc; continue; }
  pc = 0x20aff24; continue;
 }
 0x20aff24 => {
  { cpu.sub32(cpu.ebx, 0x70u32); }
  if cpu.cf { pc = 0x20aff56; continue; }
  pc = 0x20aff29; continue;
 }
 0x20aff29 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afff0; continue; }
  pc = 0x20aff2f; continue;
 }
 0x20aff2f => {
  { cpu.sub32(cpu.ebx, 0x75u32); }
  if cpu.cf { pc = 0x20aff48; continue; }
  pc = 0x20aff34; continue;
 }
 0x20aff34 => {
  if (cpu.cf || cpu.zf) { pc = 0x20affe3; continue; }
  pc = 0x20aff3a; continue;
 }
 0x20aff3a => {
  { cpu.sub32(cpu.ebx, 0x78u32); }
  if cpu.zf { pc = 0x20afff0; continue; }
  pc = 0x20aff43; continue;
 }
 0x20aff43 => {
  pc = 0x20b005f; continue;
 }
 0x20aff48 => {
  { cpu.sub32(cpu.ebx, 0x73u32); }
  if cpu.zf { pc = 0x20b0007; continue; }
  pc = 0x20aff51; continue;
 }
 0x20aff51 => {
  pc = 0x20b005f; continue;
 }
 0x20aff56 => {
  { cpu.sub32(cpu.ebx, 0x6eu32); }
  if cpu.cf { pc = 0x20aff66; continue; }
  pc = 0x20aff5b; continue;
 }
 0x20aff5b => {
  if (cpu.cf || cpu.zf) { pc = 0x20b0042; continue; }
  pc = 0x20aff61; continue;
 }
 0x20aff61 => {
  pc = 0x20affd7; continue;
 }
 0x20aff66 => {
  { cpu.sub32(cpu.ebx, 0x69u32); }
  if cpu.zf { pc = 0x20affca; continue; }
  pc = 0x20aff6b; continue;
 }
 0x20aff6b => {
  pc = 0x20b005f; continue;
 }
 0x20aff70 => {
  { cpu.sub32(cpu.ebx, 0x58u32); }
  if cpu.cf { pc = 0x20aff96; continue; }
  pc = 0x20aff75; continue;
 }
 0x20aff75 => {
  if (cpu.cf || cpu.zf) { pc = 0x20afff0; continue; }
  pc = 0x20aff7b; continue;
 }
 0x20aff7b => {
  { cpu.sub32(cpu.ebx, 0x63u32); }
  if cpu.cf { pc = 0x20aff88; continue; }
  pc = 0x20aff80; continue;
 }
 0x20aff80 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b001f; continue; }
  pc = 0x20aff86; continue;
 }
 0x20aff86 => {
  pc = 0x20affb5; continue;
 }
 0x20aff88 => {
  { cpu.sub32(cpu.ebx, 0x5bu32); }
  if cpu.zf { pc = 0x20b0012; continue; }
  pc = 0x20aff91; continue;
 }
 0x20aff91 => {
  pc = 0x20b005f; continue;
 }
 0x20aff96 => {
  { cpu.sub32(cpu.ebx, 0x45u32); }
  if cpu.cf { pc = 0x20affa7; continue; }
  pc = 0x20aff9b; continue;
 }
 0x20aff9b => {
  if (cpu.cf || cpu.zf) { pc = 0x20afffc; continue; }
  pc = 0x20aff9d; continue;
 }
 0x20aff9d => {
  { cpu.sub32(cpu.ebx, 0x47u32); }
  if cpu.zf { pc = 0x20afffc; continue; }
  pc = 0x20affa2; continue;
 }
 0x20affa2 => {
  pc = 0x20b005f; continue;
 }
 0x20affa7 => {
  { cpu.sub32(cpu.ebx, 0x25u32); }
  if cpu.zf { pc = 0x20b004f; continue; }
  pc = 0x20affb0; continue;
 }
 0x20affb0 => {
  pc = 0x20b005f; continue;
 }
 0x20affb5 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0xau32; cpu.ebx = v; }
  pc = 0x20affbf; continue;
 }
 0x20affbf => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20affc3; continue;
 }
 0x20affc3 => {
  cpu.push(0x20affc8); f_020b085d(cpu); if cpu.returned_to != 0x20affc8 { return; }
  pc = 0x20b0028; continue;
 }
 0x20affca => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.poll(); pc = 0x20affc3; continue;
 }
 0x20affd7 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x8u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x20affbf; continue;
 }
 0x20affe3 => {
  { let v: u32 = 0xau32; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  cpu.poll(); pc = 0x20affc3; continue;
 }
 0x20afff0 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x10u32; cpu.ebx = v; }
  cpu.poll(); pc = 0x20affbf; continue;
 }
 0x20afffc => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b0005); f_020b0546(cpu); if cpu.returned_to != 0x20b0005 { return; }
  pc = 0x20b0028; continue;
 }
 0x20b0007 => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b0010); f_020b0288(cpu); if cpu.returned_to != 0x20b0010 { return; }
  pc = 0x20b0028; continue;
 }
 0x20b0012 => {
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b001d); f_020b044c(cpu); if cpu.returned_to != 0x20b001d { return; }
  pc = 0x20b0028; continue;
 }
 0x20b001f => {
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b0028); f_020b01f3(cpu); if cpu.returned_to != 0x20b0028 { return; }
  pc = 0x20b0028; continue;
 }
 0x20b0028 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b009f; continue; }
  pc = 0x20b0032; continue;
 }
 0x20b0032 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebx); cpu.ebp = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b005f; continue; }
  pc = 0x20b003c; continue;
 }
 0x20b003c => {
  { let v: u32 = cpu.inc32(cpu.r32(0x4u32.wrapping_add(cpu.esp))); cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b005f; continue;
 }
 0x20b0042 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b004d); f_020b03ab(cpu); if cpu.returned_to != 0x20b004d { return; }
  pc = 0x20b005f; continue;
 }
 0x20b004f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b0056); f_020afe69(cpu); if cpu.returned_to != 0x20b0056 { return; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x20afee8; continue; }
  pc = 0x20b005e; continue;
 }
 0x20b005e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20b005f; continue;
 }
 0x20b005f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { cpu.poll(); pc = 0x20afe97; continue; }
  pc = 0x20b0069; continue;
 }
 0x20b0069 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x25u32); }
  if !cpu.zf { pc = 0x20b009f; continue; }
  pc = 0x20b0078; continue;
 }
 0x20b0078 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  cpu.push(0x20b0085); f_020b00bf(cpu); if cpu.returned_to != 0x20b0085 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x6eu32); }
  if !cpu.zf { pc = 0x20b009f; continue; }
  pc = 0x20b0094; continue;
 }
 0x20b0094 => {
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b009f); f_020b03ab(cpu); if cpu.returned_to != 0x20b009f { return; }
  pc = 0x20b009f; continue;
 }
 0x20b009f => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b00b3; continue; }
  pc = 0x20b00a6; continue;
 }
 0x20b00a6 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if cpu.zf { pc = 0x20b00b3; continue; }
  pc = 0x20b00ac; continue;
 }
 0x20b00ac => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b00b7; continue;
 }
 0x20b00b3 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x20b00b7; continue;
 }
 0x20b00b7 => {
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
pub fn f_020b00bf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b00bf); }
 let mut pc: u32 = 0x20b00bf;
 loop { match pc {
 0x20b00bf => {
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
  if !cpu.zf { pc = 0x20b00f2; continue; }
  pc = 0x20b00e8; continue;
 }
 0x20b00e8 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b00f2; continue;
 }
 0x20b00f2 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20b0144; continue; }
  pc = 0x20b0115; continue;
 }
 0x20b0115 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x20b0117; continue;
 }
 0x20b0117 => {
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
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x20u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x20b0117; continue; }
  pc = 0x20b0141; continue;
 }
 0x20b0141 => {
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b0144; continue;
 }
 0x20b0144 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0x4eu32); }
  if !cpu.zf { pc = 0x20b0158; continue; }
  pc = 0x20b0150; continue;
 }
 0x20b0150 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)) | 0x8u8; cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b0168; continue;
 }
 0x20b0158 => {
  { cpu.sub32(cpu.edx, 0x46u32); }
  if !cpu.zf { pc = 0x20b0168; continue; }
  pc = 0x20b015d; continue;
 }
 0x20b015d => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) | 0x4u8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b0168; continue;
 }
 0x20b0168 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.sub8((cpu.edx as u8), 0x68u8); }
  if cpu.cf { pc = 0x20b0182; continue; }
  pc = 0x20b0172; continue;
 }
 0x20b0172 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b0189; continue; }
  pc = 0x20b0174; continue;
 }
 0x20b0174 => {
  { cpu.sub8((cpu.edx as u8), 0x6cu8); }
  if cpu.cf { pc = 0x20b01ad; continue; }
  pc = 0x20b0179; continue;
 }
 0x20b0179 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b0196; continue; }
  pc = 0x20b017b; continue;
 }
 0x20b017b => {
  { cpu.sub8((cpu.edx as u8), 0x77u8); }
  if cpu.zf { pc = 0x20b0196; continue; }
  pc = 0x20b0180; continue;
 }
 0x20b0180 => {
  pc = 0x20b01ad; continue;
 }
 0x20b0182 => {
  { cpu.sub8((cpu.edx as u8), 0x4cu8); }
  if cpu.zf { pc = 0x20b01a2; continue; }
  pc = 0x20b0187; continue;
 }
 0x20b0187 => {
  pc = 0x20b01ad; continue;
 }
 0x20b0189 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x10u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b01ad; continue;
 }
 0x20b0196 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b01ad; continue;
 }
 0x20b01a2 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) | 0x40u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x10u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b01ad; continue;
 }
 0x20b01ad => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b01b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b01b4); }
 let mut pc: u32 = 0x20b01b4;
 loop { match pc {
 0x20b01b4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b01bb; continue;
 }
 0x20b01bb => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20b01c2); f_020afe69(cpu); if cpu.returned_to != 0x20b01c2 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20b01e0; continue; }
  pc = 0x20b01dd; continue;
 }
 0x20b01dd => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20b01bb; continue;
 }
 0x20b01e0 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if !cpu.zf { pc = 0x20b01ed; continue; }
  pc = 0x20b01e6; continue;
 }
 0x20b01e6 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20b01ed); f_020afe70(cpu); if cpu.returned_to != 0x20b01ed { return; }
  pc = 0x20b01ed; continue;
 }
 0x20b01ed => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b01f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b01f3); } r_020b01f3(cpu, 0x20b01f3); }
pub fn f_020b0288(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b0288); } r_020b01f3(cpu, 0x20b0288); }
pub fn f_020b0546(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b0546); } r_020b01f3(cpu, 0x20b0546); }
fn r_020b01f3(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b01f3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x1u8); }
  if cpu.zf { pc = 0x20b023a; continue; }
  pc = 0x20b0203; continue;
 }
 0x20b0203 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b0217; continue; }
  pc = 0x20b020b; continue;
 }
 0x20b020b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x20b023a; continue;
 }
 0x20b0217 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x20b022c; continue; }
  pc = 0x20b021c; continue;
 }
 0x20b021c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x20b023a; continue;
 }
 0x20b022c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x20b023a; continue;
 }
 0x20b023a => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.sub32(cpu.edx, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b0249; continue; }
  pc = 0x20b0244; continue;
 }
 0x20b0244 => {
  { let v: u32 = 0x1u32; cpu.edx = v; }
  pc = 0x20b0249; continue;
 }
 0x20b0249 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b027d; continue; }
  pc = 0x20b024d; continue;
 }
 0x20b024d => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20b0254); f_020afe69(cpu); if cpu.returned_to != 0x20b0254 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x20b027d; continue; }
  pc = 0x20b025e; continue;
 }
 0x20b025e => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x20b0249; continue; }
  pc = 0x20b0264; continue;
 }
 0x20b0264 => {
  { cpu.and8((cpu.eax as u8), 0x20u8); }
  if cpu.zf { pc = 0x20b0274; continue; }
  pc = 0x20b0268; continue;
 }
 0x20b0268 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx.wrapping_add(0x2u32); cpu.ecx = v; }
  cpu.poll(); pc = 0x20b0249; continue;
 }
 0x20b0274 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x20b0249; continue;
 }
 0x20b027d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  pc = 0x20b0282; continue;
 }
 0x20b0282 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b0288 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.eax)), 0x20u8); }
  if cpu.zf { pc = 0x20b029f; continue; }
  pc = 0x20b0298; continue;
 }
 0x20b0298 => {
  { let v: u8 = 0x2u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b02a8; continue;
 }
 0x20b029f => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x10u8); }
  { let v: u8 = 0x1u8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b02a8; continue;
 }
 0x20b02a8 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b02e1; continue; }
  pc = 0x20b02af; continue;
 }
 0x20b02af => {
  { cpu.and8((cpu.eax as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b02bf; continue; }
  pc = 0x20b02b3; continue;
 }
 0x20b02b3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ecx = v; cpu.set_segment(0, selector); }
  pc = 0x20b02e1; continue;
 }
 0x20b02bf => {
  { cpu.and8((cpu.eax as u8), 0x8u8); }
  if cpu.zf { pc = 0x20b02d3; continue; }
  pc = 0x20b02c3; continue;
 }
 0x20b02c3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  pc = 0x20b02e1; continue;
 }
 0x20b02d3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ecx)); cpu.ecx = v; }
  pc = 0x20b02e1; continue;
 }
 0x20b02e1 => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b02e3; continue;
 }
 0x20b02e3 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20b02ea); f_020afe69(cpu); if cpu.returned_to != 0x20b02ea { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b0309; continue; }
  pc = 0x20b0306; continue;
 }
 0x20b0306 => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20b02e3; continue;
 }
 0x20b0309 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x2u8); }
  if cpu.zf { pc = 0x20b0316; continue; }
  pc = 0x20b030f; continue;
 }
 0x20b030f => {
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b0380; continue;
 }
 0x20b0316 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.ebx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b0376; continue; }
  pc = 0x20b0323; continue;
 }
 0x20b0323 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b034e; continue; }
  pc = 0x20b032c; continue;
 }
 0x20b032c => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20b033f; continue; }
  pc = 0x20b0337; continue;
 }
 0x20b0337 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0346; continue;
 }
 0x20b033f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0346; continue;
 }
 0x20b0346 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  pc = 0x20b034e; continue;
 }
 0x20b034e => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20b0355); f_020b0a74(cpu); if cpu.returned_to != 0x20b0355 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0380; continue; }
  pc = 0x20b035d; continue;
 }
 0x20b035d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x20b0323; continue; }
  pc = 0x20b0376; continue;
 }
 0x20b0376 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20b0380); f_020afe70(cpu); if cpu.returned_to != 0x20b0380 { return; }
  pc = 0x20b0380; continue;
 }
 0x20b0380 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ebx)), 0x1u8); }
  if cpu.zf { pc = 0x20b03a1; continue; }
  pc = 0x20b0386; continue;
 }
 0x20b0386 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b03a1; continue; }
  pc = 0x20b038a; continue;
 }
 0x20b038a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20b039b; continue; }
  pc = 0x20b0395; continue;
 }
 0x20b0395 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b03a1; continue;
 }
 0x20b039b => {
  { let v: u16 = 0x0u16; cpu.w16(0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b03a1; continue;
 }
 0x20b03a1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x8u32); cpu.esp = v; }
  cpu.poll(); pc = 0x20b0282; continue;
 }
 0x20b0546 => {
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
  pc = 0x20b055d; continue;
 }
 0x20b055d => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b0564); f_020afe69(cpu); if cpu.returned_to != 0x20b0564 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.edx as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = (cpu.edx as u8) & 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.edx, 0xffu32); cpu.edx = v; }
  if cpu.zf { pc = 0x20b0584; continue; }
  pc = 0x20b057e; continue;
 }
 0x20b057e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x60u32.wrapping_add(cpu.esp))); cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b055d; continue;
 }
 0x20b0584 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b058e; continue;
 }
 0x20b058e => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b078e; continue; }
  pc = 0x20b059f; continue;
 }
 0x20b059f => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x20b05a9; continue; }
  pc = 0x20b05a4; continue;
 }
 0x20b05a4 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x20b05c7; continue; }
  pc = 0x20b05a9; continue;
 }
 0x20b05a9 => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b05b6); f_020b0a74(cpu); if cpu.returned_to != 0x20b05b6 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x60u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b05c7; continue;
 }
 0x20b05c7 => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x20b05e8; continue; }
  pc = 0x20b05df; continue;
 }
 0x20b05df => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x20b078e; continue; }
  pc = 0x20b05e8; continue;
 }
 0x20b05e8 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b0650; continue; }
  pc = 0x20b0608; continue;
 }
 0x20b0608 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  pc = 0x20b060d; continue;
 }
 0x20b060d => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.and8(((cpu.edx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x20b0627; continue; }
  pc = 0x20b0618; continue;
 }
 0x20b0618 => {
  { let v: u32 = cpu.imul2_32(cpu.r32(0x6au32.wrapping_add(cpu.esp)), 0xau32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x30u32); cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x6au32.wrapping_add(cpu.esp), v); }
  pc = 0x20b0627; continue;
 }
 0x20b0627 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  cpu.push(0x20b062f); f_020b0a74(cpu); if cpu.returned_to != 0x20b062f { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b063a; continue;
 }
 0x20b063a => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20b060d; continue; }
  pc = 0x20b0650; continue;
 }
 0x20b0650 => {
  { cpu.sub32(cpu.ecx, 0x2eu32); }
  if !cpu.zf { pc = 0x20b0700; continue; }
  pc = 0x20b0659; continue;
 }
 0x20b0659 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b0662); f_020b0a74(cpu); if cpu.returned_to != 0x20b0662 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b066e; continue;
 }
 0x20b066e => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if !cpu.zf { pc = 0x20b068c; continue; }
  pc = 0x20b0672; continue;
 }
 0x20b0672 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b078e; continue; }
  pc = 0x20b068c; continue;
 }
 0x20b068c => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x20b068d; continue;
 }
 0x20b068d => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b06b7; continue; }
  pc = 0x20b06a5; continue;
 }
 0x20b06a5 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b06af); f_020b0a74(cpu); if cpu.returned_to != 0x20b06af { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b068d; continue; }
  pc = 0x20b06b7; continue;
 }
 0x20b06b7 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x20b06f7; continue; }
  pc = 0x20b06bd; continue;
 }
 0x20b06bd => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b06c5; continue;
 }
 0x20b06c5 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x20b06ee; continue; }
  pc = 0x20b06d0; continue;
 }
 0x20b06d0 => {
  { let v: u8 = (cpu.eax as u8).wrapping_sub(0x30u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0xau32; cpu.w32(0x64u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5au32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v = cpu.r32(0x64u32.wrapping_add(cpu.esp)); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x58u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b06c5; continue;
 }
 0x20b06ee => {
  { let v: u32 = cpu.r32(0x58u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b06f7; continue;
 }
 0x20b06f7 => {
  { cpu.sub32(cpu.ecx, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b0700; continue;
 }
 0x20b0700 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if !cpu.zf { pc = 0x20b078e; continue; }
  pc = 0x20b070a; continue;
 }
 0x20b070a => {
  { cpu.sub32(cpu.ecx, 0x65u32); }
  if cpu.zf { pc = 0x20b0718; continue; }
  pc = 0x20b070f; continue;
 }
 0x20b070f => {
  { cpu.sub32(cpu.ecx, 0x45u32); }
  if !cpu.zf { pc = 0x20b078e; continue; }
  pc = 0x20b0718; continue;
 }
 0x20b0718 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b0722); f_020b0a74(cpu); if cpu.returned_to != 0x20b0722 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b072e; continue;
 }
 0x20b072e => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x20b0738; continue; }
  pc = 0x20b0733; continue;
 }
 0x20b0733 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x20b074a; continue; }
  pc = 0x20b0738; continue;
 }
 0x20b0738 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b0742); f_020b0a74(cpu); if cpu.returned_to != 0x20b0742 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b074a; continue;
 }
 0x20b074a => {
  { let v: u8 = (cpu.ecx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { pc = 0x20b0766; continue; }
  pc = 0x20b0762; continue;
 }
 0x20b0762 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b078e; continue;
 }
 0x20b0766 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.push(0x20b0770); f_020b0a74(cpu); if cpu.returned_to != 0x20b0770 { return; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b0797; continue; }
  pc = 0x20b0778; continue;
 }
 0x20b0778 => {
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20b0766; continue; }
  pc = 0x20b078e; continue;
 }
 0x20b078e => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20b0797); f_020afe70(cpu); if cpu.returned_to != 0x20b0797 { return; }
  pc = 0x20b0797; continue;
 }
 0x20b0797 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b0852; continue; }
  pc = 0x20b079f; continue;
 }
 0x20b079f => {
  { let v: u32 = cpu.r32(0x60u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b0852; continue; }
  pc = 0x20b07b1; continue;
 }
 0x20b07b1 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x10u8); }
  if cpu.zf { pc = 0x20b07ca; continue; }
  pc = 0x20b07ba; continue;
 }
 0x20b07ba => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2du32); }
  if !cpu.zf { pc = 0x20b07d6; continue; }
  pc = 0x20b07c4; continue;
 }
 0x20b07c4 => {
  { let v: u32 = cpu.neg32(cpu.r32(0x68u32.wrapping_add(cpu.esp))); cpu.w32(0x68u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b07d6; continue;
 }
 0x20b07ca => {
  { let v: u32 = 0x50u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esp; cpu.eax = v; }
  let t = cpu.r32(0x217c860u32); cpu.push(0x20b07d6); cpu.call(t); if cpu.returned_to != 0x20b07d6 { return; }
  pc = 0x20b07d6; continue;
 }
 0x20b07d6 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b07ee; continue; }
  pc = 0x20b07de; continue;
 }
 0x20b07de => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x8u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.ecx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.ebx = v; cpu.set_segment(0, selector); }
  pc = 0x20b0819; continue;
 }
 0x20b07ee => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x20b0807; continue; }
  pc = 0x20b07f3; continue;
 }
 0x20b07f3 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  pc = 0x20b0819; continue;
 }
 0x20b0807 => {
  { let v: u32 = cpu.r32(0x5cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.ebx = v; }
  pc = 0x20b0819; continue;
 }
 0x20b0819 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x10u8); }
  if cpu.zf { pc = 0x20b0827; continue; }
  pc = 0x20b0821; continue;
 }
 0x20b0821 => {
  { let v: u32 = cpu.r32(0x68u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x20b084f; continue;
 }
 0x20b0827 => {
  { cpu.and8((cpu.ecx as u8), 0x20u8); }
  if !cpu.zf { pc = 0x20b0831; continue; }
  pc = 0x20b082c; continue;
 }
 0x20b082c => {
  { cpu.and8((cpu.ecx as u8), 0x40u8); }
  if cpu.zf { pc = 0x20b0842; continue; }
  pc = 0x20b0831; continue;
 }
 0x20b0831 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0852; continue;
 }
 0x20b0842 => {
  { let v: u32 = cpu.r32(0x50u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x54u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  cpu.push(0x20b084f); f_020be188(cpu); if cpu.returned_to != 0x20b084f { return; }
  pc = 0x20b084f; continue;
 }
 0x20b084f => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0852; continue;
 }
 0x20b0852 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x6cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.poll(); pc = 0x20b0282; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b03ab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b03ab); }
 let mut pc: u32 = 0x20b03ab;
 loop { match pc {
 0x20b03ab => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b0404; continue; }
  pc = 0x20b03b7; continue;
 }
 0x20b03b7 => {
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b03c8; continue; }
  pc = 0x20b03bc; continue;
 }
 0x20b03bc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.edx = v; cpu.set_segment(0, selector); }
  pc = 0x20b03ed; continue;
 }
 0x20b03c8 => {
  { cpu.and8((cpu.ecx as u8), 0x8u8); }
  if cpu.zf { pc = 0x20b03dd; continue; }
  pc = 0x20b03cd; continue;
 }
 0x20b03cd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x20b03ed; continue;
 }
 0x20b03dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(0x4u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.ecx = v; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.edx = v; }
  pc = 0x20b03ed; continue;
 }
 0x20b03ed => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x10u8); }
  if cpu.zf { pc = 0x20b03fe; continue; }
  pc = 0x20b03f5; continue;
 }
 0x20b03f5 => {
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b03fe => {
  { cpu.and8(((cpu.ecx >> 8) as u8), 0x20u8); }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0404; continue;
 }
 0x20b0404 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b0409(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0409); }
 let mut pc: u32 = 0x20b0409;
 loop { match pc {
 0x20b0409 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x20b041e); f_020aedc0(cpu); if cpu.returned_to != 0x20b041e { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b0446; continue; }
  pc = 0x20b0427; continue;
 }
 0x20b0427 => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0x7u32; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 3u32); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x216b6ccu32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1))) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b0446; continue; }
  pc = 0x20b0440; continue;
 }
 0x20b0440 => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x5du32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b0427; continue; }
  pc = 0x20b0446; continue;
 }
 0x20b0446 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b044c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b044c); }
 let mut pc: u32 = 0x20b044c;
 loop { match pc {
 0x20b044c => {
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
  if cpu.zf { pc = 0x20b0473; continue; }
  pc = 0x20b0470; continue;
 }
 0x20b0470 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b0473; continue;
 }
 0x20b0473 => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  cpu.push(0x20b047c); f_020b0409(cpu); if cpu.returned_to != 0x20b047c { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b04ba; continue; }
  pc = 0x20b0486; continue;
 }
 0x20b0486 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b0497; continue; }
  pc = 0x20b048b; continue;
 }
 0x20b048b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.esi); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.esi = v; cpu.set_segment(0, selector); }
  pc = 0x20b04ba; continue;
 }
 0x20b0497 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x20b04ac; continue; }
  pc = 0x20b049c; continue;
 }
 0x20b049c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.esi = v; }
  pc = 0x20b04ba; continue;
 }
 0x20b04ac => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.eax; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edx)); cpu.esi = v; }
  pc = 0x20b04ba; continue;
 }
 0x20b04ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ecx)); cpu.edx = v; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b04bf; continue;
 }
 0x20b04bf => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x20b052d; continue; }
  pc = 0x20b04c3; continue;
 }
 0x20b04c3 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20b04ca); f_020afe69(cpu); if cpu.returned_to != 0x20b04ca { return; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.esp), v); }
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if !cpu.zf { pc = 0x20b052d; continue; }
  pc = 0x20b04d6; continue;
 }
 0x20b04d6 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 3u32); cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7u32; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x216b6ccu32.wrapping_add(cpu.ebx)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebp); }
  { let v: u8 = (cpu.zf) as u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.ebx & 0xffu32; cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ebp); }
  if cpu.zf { pc = 0x20b0519; continue; }
  pc = 0x20b0510; continue;
 }
 0x20b0510 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  cpu.push(0x20b0517); f_020afe70(cpu); if cpu.returned_to != 0x20b0517 { return; }
  pc = 0x20b052d; continue;
 }
 0x20b0519 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.ecx)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x20b04bf; continue; }
  pc = 0x20b0523; continue;
 }
 0x20b0523 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20b04bf; continue;
 }
 0x20b052d => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.ecx)), 0x1u8); }
  if cpu.zf { pc = 0x20b053b; continue; }
  pc = 0x20b0533; continue;
 }
 0x20b0533 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b053b; continue; }
  pc = 0x20b0537; continue;
 }
 0x20b0537 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b053b; continue;
 }
 0x20b053b => {
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
pub fn f_020b085d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b085d); }
 let mut pc: u32 = 0x20b085d;
 loop { match pc {
 0x20b085d => {
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
  pc = 0x20b0877; continue;
 }
 0x20b0877 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b087e); f_020afe69(cpu); if cpu.returned_to != 0x20b087e { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = cpu.inc8((cpu.eax as u8)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.ebp = v; }
  { let v: u8 = cpu.r8(0x216b5a0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x2u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.and32(cpu.eax, 0xffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b0899; continue; }
  pc = 0x20b0894; continue;
 }
 0x20b0894 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b0877; continue;
 }
 0x20b0899 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x2u8); }
  if !cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b08a3; continue;
 }
 0x20b08a3 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0xcu32.wrapping_add(cpu.esi), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b09cc; continue; }
  pc = 0x20b08b4; continue;
 }
 0x20b08b4 => {
  { let v: u32 = 0x2bu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x20b08e8; continue; }
  pc = 0x20b08c5; continue;
 }
 0x20b08c5 => {
  { cpu.sub32(cpu.ebx, 0x2bu32); }
  if cpu.zf { pc = 0x20b08cf; continue; }
  pc = 0x20b08ca; continue;
 }
 0x20b08ca => {
  { cpu.sub32(cpu.ebx, 0x2du32); }
  if !cpu.zf { pc = 0x20b08e8; continue; }
  pc = 0x20b08cf; continue;
 }
 0x20b08cf => {
  { let v: u32 = cpu.inc32(cpu.r32(0x0u32.wrapping_add(cpu.esp))); cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20b08dd); f_020b0a74(cpu); if cpu.returned_to != 0x20b08dd { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b08e8; continue;
 }
 0x20b08e8 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x20b0944; continue; }
  pc = 0x20b08ec; continue;
 }
 0x20b08ec => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x20b093d; continue; }
  pc = 0x20b08f1; continue;
 }
 0x20b08f1 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x20b08fd); f_020b0a74(cpu); if cpu.returned_to != 0x20b08fd { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b0908; continue;
 }
 0x20b0908 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x20b0912; continue; }
  pc = 0x20b090d; continue;
 }
 0x20b090d => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x20b0936; continue; }
  pc = 0x20b0912; continue;
 }
 0x20b0912 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  cpu.push(0x20b091c); f_020b0a74(cpu); if cpu.returned_to != 0x20b091c { return; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x2u32); cpu.ebp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b092f; continue;
 }
 0x20b092f => {
  { let v: u32 = 0x10u32; cpu.edx = v; }
  pc = 0x20b0988; continue;
 }
 0x20b0936 => {
  { let v: u32 = 0x8u32; cpu.edx = v; }
  pc = 0x20b0988; continue;
 }
 0x20b093d => {
  { let v: u32 = 0xau32; cpu.edx = v; }
  pc = 0x20b0988; continue;
 }
 0x20b0944 => {
  { cpu.sub32(cpu.edx, 0x10u32); }
  if !cpu.zf { pc = 0x20b0988; continue; }
  pc = 0x20b0949; continue;
 }
 0x20b0949 => {
  { cpu.sub32(cpu.ebx, 0x30u32); }
  if !cpu.zf { pc = 0x20b0988; continue; }
  pc = 0x20b094e; continue;
 }
 0x20b094e => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  cpu.push(0x20b095a); f_020b0a74(cpu); if cpu.returned_to != 0x20b095a { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b0965; continue;
 }
 0x20b0965 => {
  { cpu.sub32(cpu.eax, 0x78u32); }
  if cpu.zf { pc = 0x20b096f; continue; }
  pc = 0x20b096a; continue;
 }
 0x20b096a => {
  { cpu.sub32(cpu.eax, 0x58u32); }
  if !cpu.zf { pc = 0x20b0988; continue; }
  pc = 0x20b096f; continue;
 }
 0x20b096f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  cpu.push(0x20b0979); f_020b0a74(cpu); if cpu.returned_to != 0x20b0979 { return; }
  { let v: u32 = cpu.ebx.wrapping_add(0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b0981; continue;
 }
 0x20b0981 => {
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b0988; continue;
 }
 0x20b0988 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20b098f); f_020b0a4d(cpu); if cpu.returned_to != 0x20b098f { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20b09a2; continue; }
  pc = 0x20b0993; continue;
 }
 0x20b0993 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x20b09a0); f_020b0a74(cpu); if cpu.returned_to != 0x20b09a0 { return; }
  cpu.poll(); pc = 0x20b0981; continue;
 }
 0x20b09a2 => {
  { cpu.sub32(cpu.ebx, 0x3au32); }
  if !cpu.zf { pc = 0x20b09cc; continue; }
  pc = 0x20b09a7; continue;
 }
 0x20b09a7 => {
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.esi)), 0x80u8); }
  if cpu.zf { pc = 0x20b09cc; continue; }
  pc = 0x20b09ad; continue;
 }
 0x20b09ad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x20b09b5); f_020b0a74(cpu); if cpu.returned_to != 0x20b09b5 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b09d5; continue; }
  pc = 0x20b09bc; continue;
 }
 0x20b09bc => {
  cpu.push(0x20b09c1); f_020b0a4d(cpu); if cpu.returned_to != 0x20b09c1 { return; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20b09cc; continue; }
  pc = 0x20b09c5; continue;
 }
 0x20b09c5 => {
  { let v: u32 = cpu.imul2_32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  cpu.poll(); pc = 0x20b09ad; continue;
 }
 0x20b09cc => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  cpu.push(0x20b09d5); f_020afe70(cpu); if cpu.returned_to != 0x20b09d5 { return; }
  pc = 0x20b09d5; continue;
 }
 0x20b09d5 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esp)), 0x2du32); }
  if !cpu.zf { pc = 0x20b09de; continue; }
  pc = 0x20b09dc; continue;
 }
 0x20b09dc => {
  { let v: u32 = cpu.neg32(cpu.edi); cpu.edi = v; }
  pc = 0x20b09de; continue;
 }
 0x20b09de => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b0a43; continue; }
  pc = 0x20b09e2; continue;
 }
 0x20b09e2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edx); cpu.ecx = v; }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b0a43; continue; }
  pc = 0x20b09ef; continue;
 }
 0x20b09ef => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b0a04; continue; }
  pc = 0x20b09f4; continue;
 }
 0x20b09f4 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let at = 0xfffffff8u32.wrapping_add(cpu.edx); let selector = cpu.r16(at.wrapping_add(4)); let v: u32 = cpu.r32(at); cpu.eax = v; cpu.set_segment(0, selector); }
  pc = 0x20b0a2f; continue;
 }
 0x20b0a04 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x8u8); }
  if cpu.zf { pc = 0x20b0a1d; continue; }
  pc = 0x20b0a09; continue;
 }
 0x20b0a09 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(0x4u32); cpu.ebp = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x20b0a2f; continue;
 }
 0x20b0a1d => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = (cpu.segments[3] as u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v = cpu.edx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  pc = 0x20b0a2f; continue;
 }
 0x20b0a2f => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x10u8); }
  if cpu.zf { pc = 0x20b0a3d; continue; }
  pc = 0x20b0a37; continue;
 }
 0x20b0a37 => {
  { let v: u16 = (cpu.edi as u16); cpu.w16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0a43; continue;
 }
 0x20b0a3d => {
  { cpu.and8((cpu.ebx as u8), 0x20u8); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[0]), v); }
  pc = 0x20b0a43; continue;
 }
 0x20b0a43 => {
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
pub fn f_020b0a4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0a4d); }
 let mut pc: u32 = 0x20b0a4d;
 loop { match pc {
 0x20b0a4d => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b0a5b; continue; }
  pc = 0x20b0a52; continue;
 }
 0x20b0a52 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b0a5b; continue; }
  pc = 0x20b0a57; continue;
 }
 0x20b0a57 => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x30u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b0a5b => {
  cpu.push(0x20b0a60); f_020a593c(cpu); if cpu.returned_to != 0x20b0a60 { return; }
  { cpu.sub32(cpu.eax, 0x61u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b0a6e; continue; }
  pc = 0x20b0a65; continue;
 }
 0x20b0a65 => {
  { cpu.sub32(cpu.eax, 0x66u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b0a6e; continue; }
  pc = 0x20b0a6a; continue;
 }
 0x20b0a6a => {
  { let v: u32 = cpu.sub32(cpu.eax, 0x57u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b0a6e => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b0a74(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0a74); }
 let mut pc: u32 = 0x20b0a74;
 loop { match pc {
 0x20b0a74 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.edx), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b0a92; continue; }
  pc = 0x20b0a85; continue;
 }
 0x20b0a85 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20b0a8c); f_020afe69(cpu); if cpu.returned_to != 0x20b0a8c { return; }
  { cpu.and8(cpu.r8(0x10u32.wrapping_add(cpu.edx)), 0x2u8); }
  if cpu.zf { pc = 0x20b0a97; continue; }
  pc = 0x20b0a92; continue;
 }
 0x20b0a92 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b0a97; continue;
 }
 0x20b0a97 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b0a9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0a9a); }
 let mut pc: u32 = 0x20b0a9a;
 loop { match pc {
 0x20b0a9a => {
  { let v: u32 = cpu.r32(0x217b898u32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b0b54(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0b54); }
 let mut pc: u32 = 0x20b0b54;
 loop { match pc {
 0x20b0b54 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b0b61); f_020be1d4(cpu); if cpu.returned_to != 0x20b0b61 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.push(v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.push(v); }
  cpu.push(0x20b0b76); f_020be60e(cpu); if cpu.returned_to != 0x20b0b76 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b0bee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0bee); }
 let mut pc: u32 = 0x20b0bee;
 loop { match pc {
 0x20b0bee => {
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
  cpu.push(0x20b0c20); f_020b0b54(cpu); if cpu.returned_to != 0x20b0c20 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  if cpu.zf { pc = 0x20b0c30; continue; }
  pc = 0x20b0c2a; continue;
 }
 0x20b0c2a => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b0c30 => {
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
pub fn f_020b0c7a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0c7a); }
 let mut pc: u32 = 0x20b0c7a;
 loop { match pc {
 0x20b0c7a => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(28); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0xffffffe4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v = 0x31u32; cpu.push(v); }
  { let v: u16 = 0x6u16; cpu.w16(0xffffffe4u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b0c96); f_020b0b54(cpu); if cpu.returned_to != 0x20b0c96 { return; }
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
pub fn f_020b0caa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0caa); }
 let mut pc: u32 = 0x20b0caa;
 loop { match pc {
 0x20b0c60 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b0c69); f_020b0b54(cpu); if cpu.returned_to != 0x20b0c69 { return; }
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { cpu.sub16(cpu.r16(0xfffffffcu32.wrapping_add(cpu.ebp)), 0x0u16); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = (cpu.eax as u8) as u32; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b0caa => {
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
  cpu.poll(); pc = 0x20b0c60; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b0eab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b0eab); }
 let mut pc: u32 = 0x20b0eab;
 loop { match pc {
 0x20b0eab => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(12); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = 0xfffffff4u32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b0ebd); f_020be1d4(cpu); if cpu.returned_to != 0x20b0ebd { return; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  cpu.push(0x20b0ec9); f_020b0c7a(cpu); if cpu.returned_to != 0x20b0ec9 { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x20b0edd; continue; }
  pc = 0x20b0ed3; continue;
 }
 0x20b0ed3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b0edd; continue; }
  pc = 0x20b0ed7; continue;
 }
 0x20b0ed7 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b0edd => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b0ee7); f_020b0bee(cpu); if cpu.returned_to != 0x20b0ee7 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b0ef7; continue; }
  pc = 0x20b0eed; continue;
 }
 0x20b0eed => {
  { cpu.sub32(cpu.esi, 0x100000u32); }
  if !cpu.cf { pc = 0x20b0f0d; continue; }
  pc = 0x20b0ef5; continue;
 }
 0x20b0ef5 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  pc = 0x20b0ef7; continue;
 }
 0x20b0ef7 => {
  { let v: u32 = cpu.r16(0xfffffffau32.wrapping_add(cpu.ebp)) as u32; cpu.eax = v; }
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  cpu.push(0x20b0f05); f_020b0caa(cpu); if cpu.returned_to != 0x20b0f05 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b0f0d; continue; }
  pc = 0x20b0f09; continue;
 }
 0x20b0f09 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  pc = 0x20b0f0d; continue;
 }
 0x20b0f0d => {
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b11ee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b11ee); }
 let mut pc: u32 = 0x20b11ee;
 loop { match pc {
 0x20b11ee => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x20be77du32; cpu.edx = v; }
  { let v: u32 = 0x20be8c6u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217c85cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c860u32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b123a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b123a); }
 let mut pc: u32 = 0x20b123a;
 loop { match pc {
 0x20b123a => {
  { let v: u8 = 0x3u8; cpu.w8(0x217420cu32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x1u8; cpu.w8(0x217b908u32.wrapping_add(cpu.bases[3]), v); }
  cpu.unimplemented(0x20b1248, "Smsw");
  { let v: u16 = cpu.r16(0x217b906u32.wrapping_add(cpu.bases[3])) & 0x6u16; cpu.w16(0x217b906u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x20b125b, "Sidt");
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x38u32); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { cpu.sub8(cpu.r8(0x217b940u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x20b128c; continue; }
  pc = 0x20b1272; continue;
 }
 0x20b1272 => {
  cpu.flat_ds(0x20b1272); cpu.push(0x20b1277); f_020b1305(cpu); if cpu.returned_to != 0x20b1277 { return; }
  { let v = cpu.r16(0x217b0e2u32.wrapping_add(cpu.bases[3])); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[0]), v); }
  cpu.flat_ds(0x20b1285); cpu.push(0x20b128a); f_020b136f(cpu); if cpu.returned_to != 0x20b128a { return; }
  pc = 0x20b1304; continue;
 }
 0x20b128c => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x20b12b0; continue; }
  pc = 0x20b1295; continue;
 }
 0x20b1295 => {
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x20becbcu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u8 = 0x4u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0xf3u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  pc = 0x20b1304; continue;
 }
 0x20b12b0 => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x20b12d8; continue; }
  pc = 0x20b12b9; continue;
 }
 0x20b12b9 => {
  cpu.flat_ds(0x20b12b9); cpu.push(0x20b12be); f_020b1305(cpu); if cpu.returned_to != 0x20b12be { return; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = 0xe02u32; cpu.eax = v; }
  { let v: u32 = 0x4u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217b0dcu32.wrapping_add(cpu.bases[3])); cpu.ecx = v; }
  let t = cpu.r32(0x30u32.wrapping_add(cpu.ecx).wrapping_add(cpu.bases[3])); cpu.flat_ds(0x20b12d3); cpu.push(0x20b12d6); cpu.call(t); if cpu.returned_to != 0x20b12d6 { return; }
  pc = 0x20b1304; continue;
 }
 0x20b12d8 => {
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x20b12f1; continue; }
  pc = 0x20b12e1; continue;
 }
 0x20b12e1 => {
  { let v: u16 = cpu.r16(0x217b0e0u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x20b12ea); cpu.push(0x20b12ef); f_020c1afa(cpu); if cpu.returned_to != 0x20b12ef { return; }
  pc = 0x20b1304; continue;
 }
 0x20b12f1 => {
  { let v: u16 = (cpu.edx as u16).wrapping_sub((cpu.edx as u16)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  cpu.flat_ds(0x20b12f6); cpu.push(0x20b12fb); f_020c1afa(cpu); if cpu.returned_to != 0x20b12fb { return; }
  { cpu.sub8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b1304; continue; }
  pc = 0x20b12ff; continue;
 }
 0x20b12ff => {
  cpu.flat_ds(0x20b12ff); cpu.push(0x20b1304); f_020b1322(cpu); if cpu.returned_to != 0x20b1304 { return; }
  pc = 0x20b1304; continue;
 }
 0x20b1304 => {
  cpu.flat_ds(0x20b1304); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1305(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1305); }
 let mut pc: u32 = 0x20b1305;
 loop { match pc {
 0x20b1305 => {
  { let v: u32 = 0x20becbcu32; cpu.ecx = v; }
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
pub fn f_020b1322(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1322); }
 let mut pc: u32 = 0x20b1322;
 loop { match pc {
 0x20b1322 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b900u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x217b904u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.push(cpu.segments[1] as u32); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  if (cpu.sf != cpu.of) { pc = 0x20b1359; continue; }
  pc = 0x20b1349; continue;
 }
 0x20b1349 => {
  { let v: u32 = 0x20bec4cu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x20b1352); cpu.push(0x20b1357); f_020b136f(cpu); if cpu.returned_to != 0x20b1357 { return; }
  pc = 0x20b136e; continue;
 }
 0x20b1359 => {
  { let v: u32 = 0x20becbcu32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx | 0x4u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffffdu32; cpu.ecx = v; }
  {  }
  pc = 0x20b136e; continue;
 }
 0x20b136e => {
  cpu.flat_ds(0x20b136e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b136f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b136f); }
 let mut pc: u32 = 0x20b136f;
 loop { match pc {
 0x20b136f => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x217b8c0u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.r32(0x217b8c0u32) | 0x4u32; cpu.w32(0x217b8c0u32, v); }
  { let v: u32 = cpu.and32(cpu.r32(0x217b8c0u32), 0xfffffffdu32); cpu.w32(0x217b8c0u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1409(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1409); }
 let mut pc: u32 = 0x20b1409;
 loop { match pc {
 0x20b1409 => {
  { let v: u8 = 0x7u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2504u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x217b900u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.r16(0x217b904u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { cpu.sub8(cpu.r8(0x217b93eu32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if (cpu.sf != cpu.of) { pc = 0x20b1430; continue; }
  pc = 0x20b1429; continue;
 }
 0x20b1429 => {
  cpu.flat_ds(0x20b1429); cpu.push(0x20b142e); f_020b1444(cpu); if cpu.returned_to != 0x20b142e { return; }
  pc = 0x20b1443; continue;
 }
 0x20b1430 => {
  { let v: u32 = 0x11u32; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0xfffffff9u32; cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x217b906u32.wrapping_add(cpu.bases[3])); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  {  }
  pc = 0x20b1443; continue;
 }
 0x20b1443 => {
  cpu.flat_ds(0x20b1443); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1444(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1444); }
 let mut pc: u32 = 0x20b1444;
 loop { match pc {
 0x20b1444 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u32 = 0x217b8c0u32; cpu.edx = v; }
  { let v: u16 = 0x2535u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x217b906u32); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217b8c0u32, v); }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.interrupt(0x21); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1465(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1465); }
 let mut pc: u32 = 0x20b1465;
 loop { match pc {
 0x20b1465 => {
  {  }
  cpu.unimplemented(0x20b1466, "Fnsave");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b146a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b146a); }
 let mut pc: u32 = 0x20b146a;
 loop { match pc {
 0x20b146a => {
  cpu.unimplemented(0x20b146a, "Frstor");
  {  }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b146e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b146e); }
 let mut pc: u32 = 0x20b146e;
 loop { match pc {
 0x20b146e => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.sub8(cpu.r8(0x217420du32), 0x0u8); }
  if cpu.zf { pc = 0x20b148f; continue; }
  pc = 0x20b1479; continue;
 }
 0x20b1479 => {
  { let v: u32 = 0x20b1465u32; cpu.edx = v; }
  { let v: u32 = 0x20b146au32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217c8ecu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x217c8f0u32, v); }
  pc = 0x20b148f; continue;
 }
 0x20b148f => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x217c8e8u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20b149c); f_020c1c28(cpu); if cpu.returned_to != 0x20b149c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b149f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b149f); }
 let mut pc: u32 = 0x20b149f;
 loop { match pc {
 0x20b149f => {
  { cpu.sub8(cpu.r8(0x217420du32), 0x0u8); }
  if !cpu.zf { f_020b146e(cpu); return; }
  pc = 0x20b14a8; continue;
 }
 0x20b14a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b14a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b14a9); }
 let mut pc: u32 = 0x20b14a9;
 loop { match pc {
 0x20b14a9 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x217420cu32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b14f1; continue; }
  pc = 0x20b14b4; continue;
 }
 0x20b14b4 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x217420du32, v); }
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
  if !cpu.zf { pc = 0x20b14dc; continue; }
  pc = 0x20b14d3; continue;
 }
 0x20b14d3 => {
  cpu.push(0x20b14d8); f_020b146e(cpu); if cpu.returned_to != 0x20b14d8 { return; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b14dc; continue;
 }
 0x20b14dc => {
  { cpu.sub8(cpu.r8(0x217b93cu32), 0x0u8); }
  if !cpu.zf { pc = 0x20b14f1; continue; }
  pc = 0x20b14e5; continue;
 }
 0x20b14e5 => {
  { let v: u8 = (cpu.edx as u8); cpu.w8(0x217420cu32, v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x217420du32, v); }
  pc = 0x20b14f1; continue;
 }
 0x20b14f1 => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b14f3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b14f3); }
 let mut pc: u32 = 0x20b14f3;
 loop { match pc {
 0x20b14f3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b14f4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b14f4); }
 let mut pc: u32 = 0x20b14f4;
 loop { match pc {
 0x20b14f4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = 0x20u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  cpu.push(0x20b1507); f_020aedc0(cpu); if cpu.returned_to != 0x20b1507 { return; }
  pc = 0x20b1528; continue;
 }
 0x20b1509 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8) & 0x7u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x216b6d4u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.edx >> 8) as u8) | (cpu.edx as u8); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  pc = 0x20b1528; continue;
 }
 0x20b1528 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1509; continue; }
  pc = 0x20b152e; continue;
 }
 0x20b152e => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1532(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1532); }
 let mut pc: u32 = 0x20b1532;
 loop { match pc {
 0x20b1532 => {
  if !cpu.cf { pc = 0x20b1542; continue; }
  pc = 0x20b1534; continue;
 }
 0x20b1534 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xffffu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  cpu.push(0x20b153f); f_020c1c5a(cpu); if cpu.returned_to != 0x20b153f { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x20b1544; continue;
 }
 0x20b1542 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20b1544; continue;
 }
 0x20b1544 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1545(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1545); }
 let mut pc: u32 = 0x20b1545;
 loop { match pc {
 0x20b1545 => {
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
  cpu.push(0x20b1563); f_020b1569(cpu); if cpu.returned_to != 0x20b1563 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1569(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1569); }
 let mut pc: u32 = 0x20b1569;
 loop { match pc {
 0x20b1569 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  pc = 0x20b1575; continue;
 }
 0x20b1575 => {
  { let v: u32 = cpu.r8(0x0u32.wrapping_add(cpu.esi)) as u32; cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0x20u32); }
  if !cpu.zf { pc = 0x20b1580; continue; }
  pc = 0x20b157d; continue;
 }
 0x20b157d => {
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  cpu.poll(); pc = 0x20b1575; continue;
 }
 0x20b1580 => {
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
  if (cpu.sf != cpu.of) { pc = 0x20b15b3; continue; }
  pc = 0x20b15ad; continue;
 }
 0x20b15ad => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x20b15b3; continue;
 }
 0x20b15b3 => {
  { let v: u8 = cpu.r8(0x28u32.wrapping_add(cpu.esp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x3u8); }
  if cpu.zf { pc = 0x20b1636; continue; }
  pc = 0x20b15c0; continue;
 }
 0x20b15c0 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b1636; continue; }
  pc = 0x20b15c9; continue;
 }
 0x20b15c9 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20b15d0); f_020c1ccb(cpu); if cpu.returned_to != 0x20b15d0 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b1636; continue; }
  pc = 0x20b15d4; continue;
 }
 0x20b15d4 => {
  { cpu.and8(cpu.r8(0x29u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { pc = 0x20b15fe; continue; }
  pc = 0x20b15db; continue;
 }
 0x20b15db => {
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x20b15fe; continue; }
  pc = 0x20b15e0; continue;
 }
 0x20b15e0 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = 0x7u32; cpu.eax = v; }
  cpu.push(0x20b15f4); f_020af0db(cpu); if cpu.returned_to != 0x20b15f4 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b1749; continue;
 }
 0x20b15fe => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x40u8); }
  if cpu.zf { pc = 0x20b1636; continue; }
  pc = 0x20b1605; continue;
 }
 0x20b1605 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u8 = 0x40u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b1636; continue; }
  pc = 0x20b161c; continue;
 }
 0x20b161c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  pc = 0x20b1626; continue;
 }
 0x20b1626 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20b1631); f_020c1c5a(cpu); if cpu.returned_to != 0x20b1631 { return; }
  pc = 0x20b1749; continue;
 }
 0x20b1636 => {
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if !cpu.zf { pc = 0x20b16d8; continue; }
  pc = 0x20b163f; continue;
 }
 0x20b163f => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x20u8); }
  if cpu.zf { cpu.poll(); pc = 0x20b1626; continue; }
  pc = 0x20b1646; continue;
 }
 0x20b1646 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2u32); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1626; continue; }
  pc = 0x20b1651; continue;
 }
 0x20b1651 => {
  { let v: u32 = 0x30u32.wrapping_add(cpu.esp); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b1664; continue; }
  pc = 0x20b165f; continue;
 }
 0x20b165f => {
  { let v: u32 = 0x180u32; cpu.eax = v; }
  pc = 0x20b1664; continue;
 }
 0x20b1664 => {
  { let v: u32 = cpu.r32(0x217c908u32); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.eax & cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if !cpu.zf { pc = 0x20b1679; continue; }
  pc = 0x20b1674; continue;
 }
 0x20b1674 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x20b1679; continue;
 }
 0x20b1679 => {
  { let v: u32 = cpu.and32(cpu.ecx, 0xffu32); cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u8 = 0x3cu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1626; continue; }
  pc = 0x20b1690; continue;
 }
 0x20b1690 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b16d8; continue; }
  pc = 0x20b169e; continue;
 }
 0x20b169e => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { let v: u8 = 0x3eu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1626; continue; }
  pc = 0x20b16b3; continue;
 }
 0x20b16b3 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1626; continue; }
  pc = 0x20b16d2; continue;
 }
 0x20b16d2 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.edi = (cpu.edi & 0xffff0000) | v as u32; }
  pc = 0x20b16d8; continue;
 }
 0x20b16d8 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20b16df); f_020c1cf2(cpu); if cpu.returned_to != 0x20b16df { return; }
  { let v: u8 = (cpu.eax as u8) & 0x3cu8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20b16ea); f_020c1ccb(cpu); if cpu.returned_to != 0x20b16ea { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b16f1; continue; }
  pc = 0x20b16ee; continue;
 }
 0x20b16ee => {
  { let v: u8 = ((cpu.edx >> 8) as u8) | 0x20u8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x20b16f1; continue;
 }
 0x20b16f1 => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)) & 0x7fu8; cpu.w8(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x2u32); }
  if !cpu.zf { pc = 0x20b1700; continue; }
  pc = 0x20b16fd; continue;
 }
 0x20b16fd => {
  { let v: u8 = (cpu.edx as u8) | 0x3u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20b1700; continue;
 }
 0x20b1700 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b170a; continue; }
  pc = 0x20b1707; continue;
 }
 0x20b1707 => {
  { let v: u8 = (cpu.edx as u8) | 0x1u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20b170a; continue;
 }
 0x20b170a => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x1u32); }
  if !cpu.zf { pc = 0x20b1714; continue; }
  pc = 0x20b1711; continue;
 }
 0x20b1711 => {
  { let v: u8 = (cpu.edx as u8) | 0x2u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20b1714; continue;
 }
 0x20b1714 => {
  { cpu.and8(cpu.r8(0x28u32.wrapping_add(cpu.esp)), 0x10u8); }
  if cpu.zf { pc = 0x20b171e; continue; }
  pc = 0x20b171b; continue;
 }
 0x20b171b => {
  { let v: u8 = (cpu.edx as u8) | 0x80u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x20b171e; continue;
 }
 0x20b171e => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x29u32.wrapping_add(cpu.esp)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if cpu.zf { pc = 0x20b1732; continue; }
  pc = 0x20b172b; continue;
 }
 0x20b172b => {
  { cpu.and8((cpu.ebx as u8), 0x2u8); }
  if cpu.zf { pc = 0x20b1740; continue; }
  pc = 0x20b1730; continue;
 }
 0x20b1730 => {
  pc = 0x20b173e; continue;
 }
 0x20b1732 => {
  { cpu.sub32(cpu.r32(0x217b30du32), 0x200u32); }
  if !cpu.zf { pc = 0x20b1740; continue; }
  pc = 0x20b173e; continue;
 }
 0x20b173e => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x20b1740; continue;
 }
 0x20b1740 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  cpu.push(0x20b1747); f_020c1d47(cpu); if cpu.returned_to != 0x20b1747 { return; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  pc = 0x20b1749; continue;
 }
 0x20b1749 => {
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
pub fn f_020b1752(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1752); }
 let mut pc: u32 = 0x20b1752;
 loop { match pc {
 0x20b1752 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  cpu.push(0x20b1760); f_020c1cf2(cpu); if cpu.returned_to != 0x20b1760 { return; }
  { let v: u8 = (cpu.eax as u8) | 0x40u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b176d); f_020c1d47(cpu); if cpu.returned_to != 0x20b176d { return; }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x20u8); }
  if cpu.zf { pc = 0x20b17ad; continue; }
  pc = 0x20b1772; continue;
 }
 0x20b1772 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = 0x44u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20b1792; continue; }
  pc = 0x20b1785; continue;
 }
 0x20b1785 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20b1790); f_020c1c5a(cpu); if cpu.returned_to != 0x20b1790 { return; }
  pc = 0x20b17af; continue;
 }
 0x20b1792 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1785; continue; }
  pc = 0x20b17ad; continue;
 }
 0x20b17ad => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b17af; continue;
 }
 0x20b17af => {
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
pub fn f_020b17b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b17b7); }
 let mut pc: u32 = 0x20b17b7;
 loop { match pc {
 0x20b17b7 => {
  { let v: u32 = 0x22d16fcu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b17bd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b17bd); }
 let mut pc: u32 = 0x20b17bd;
 loop { match pc {
 0x20b17bd => {
  { let v: u32 = 0x22d1700u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b17c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b17c3); }
 let mut pc: u32 = 0x20b17c3;
 loop { match pc {
 0x20b17c3 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x20b17d3; continue; }
  pc = 0x20b17c7; continue;
 }
 0x20b17c7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x217c90cu32)); }
  if (cpu.cf || cpu.zf) { f_020c1d5c(cpu); return; }
  pc = 0x20b17d3; continue;
 }
 0x20b17d3 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  cpu.push(0x20b17dd); f_020af0db(cpu); if cpu.returned_to != 0x20b17dd { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b17e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b17e3); }
 let mut pc: u32 = 0x20b17e3;
 loop { match pc {
 0x20b17e3 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x10u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  cpu.push(0x20b17f2); f_020c1cf2(cpu); if cpu.returned_to != 0x20b17f2 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b180e; continue; }
  pc = 0x20b17fa; continue;
 }
 0x20b17fa => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x20b17ff; continue;
 }
 0x20b17ff => {
  cpu.push(0x20b1804); f_020af0db(cpu); if cpu.returned_to != 0x20b1804 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b18d6; continue;
 }
 0x20b180e => {
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if !cpu.zf { pc = 0x20b1819; continue; }
  pc = 0x20b1812; continue;
 }
 0x20b1812 => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20b17ff; continue;
 }
 0x20b1819 => {
  { cpu.and8((cpu.eax as u8), 0x40u8); }
  if cpu.zf { pc = 0x20b1845; continue; }
  pc = 0x20b181d; continue;
 }
 0x20b181d => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u8 = 0x3fu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b18d4; continue; }
  pc = 0x20b1836; continue;
 }
 0x20b1836 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edi as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x20b183b; continue;
 }
 0x20b183b => {
  cpu.push(0x20b1840); f_020c1c5a(cpu); if cpu.returned_to != 0x20b1840 { return; }
  pc = 0x20b18d6; continue;
 }
 0x20b1845 => {
  { let v: u32 = cpu.ebx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.xor32(cpu.esi, cpu.esi); cpu.esi = v; }
  pc = 0x20b184b; continue;
 }
 0x20b184b => {
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
  if (cpu.sf == cpu.of) { pc = 0x20b186f; continue; }
  pc = 0x20b1868; continue;
 }
 0x20b1868 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.poll(); pc = 0x20b183b; continue;
 }
 0x20b186f => {
  if cpu.zf { pc = 0x20b18d4; continue; }
  pc = 0x20b1871; continue;
 }
 0x20b1871 => {
  { let v: u32 = cpu.edi ^ cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.ecx = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b18b1; continue;
 }
 0x20b1880 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.ebx, 0x1au32); }
  if !cpu.zf { pc = 0x20b18a1; continue; }
  pc = 0x20b1889; continue;
 }
 0x20b1889 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  cpu.push(0x20b189f); f_020b1ab3(cpu); if cpu.returned_to != 0x20b189f { return; }
  pc = 0x20b18d4; continue;
 }
 0x20b18a1 => {
  { cpu.sub32(cpu.ebx, 0xdu32); }
  if cpu.zf { pc = 0x20b18af; continue; }
  pc = 0x20b18a6; continue;
 }
 0x20b18a6 => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)), v); }
  pc = 0x20b18af; continue;
 }
 0x20b18af => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x20b18b1; continue;
 }
 0x20b18b1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x10u32.wrapping_add(cpu.esp))); }
  if cpu.cf { cpu.poll(); pc = 0x20b1880; continue; }
  pc = 0x20b18b7; continue;
 }
 0x20b18b7 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x9u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x20u8); }
  if !cpu.zf { pc = 0x20b18d4; continue; }
  pc = 0x20b18cc; continue;
 }
 0x20b18cc => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { cpu.poll(); pc = 0x20b184b; continue; }
  pc = 0x20b18d4; continue;
 }
 0x20b18d4 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b18d6; continue;
 }
 0x20b18d6 => {
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
pub fn f_020b18de(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b18de); }
 let mut pc: u32 = 0x20b18de;
 loop { match pc {
 0x20b18de => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0x18u32); cpu.esp = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x20b18ef); f_020c1cf2(cpu); if cpu.returned_to != 0x20b18ef { return; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b190a; continue; }
  pc = 0x20b18f6; continue;
 }
 0x20b18f6 => {
  { let v: u32 = 0x4u32; cpu.eax = v; }
  pc = 0x20b18fb; continue;
 }
 0x20b18fb => {
  cpu.push(0x20b1900); f_020af0db(cpu); if cpu.returned_to != 0x20b1900 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b1aac; continue;
 }
 0x20b190a => {
  { cpu.and8((cpu.eax as u8), 0x2u8); }
  if !cpu.zf { pc = 0x20b1915; continue; }
  pc = 0x20b190e; continue;
 }
 0x20b190e => {
  { let v: u32 = 0x6u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20b18fb; continue;
 }
 0x20b1915 => {
  { cpu.and8((cpu.eax as u8), 0x80u8); }
  if cpu.zf { pc = 0x20b194c; continue; }
  pc = 0x20b1919; continue;
 }
 0x20b1919 => {
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
  if (cpu.sf == cpu.of) { pc = 0x20b194c; continue; }
  pc = 0x20b193c; continue;
 }
 0x20b193c => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0xfffffff4u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20b1947); f_020c1c5a(cpu); if cpu.returned_to != 0x20b1947 { return; }
  pc = 0x20b1aac; continue;
 }
 0x20b194c => {
  { cpu.and8(cpu.r8(0xffffffecu32.wrapping_add(cpu.ebp)), 0x40u8); }
  if cpu.zf { pc = 0x20b1987; continue; }
  pc = 0x20b1952; continue;
 }
 0x20b1952 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b193c; continue; }
  pc = 0x20b196c; continue;
 }
 0x20b196c => {
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if cpu.zf { pc = 0x20b1aa9; continue; }
  pc = 0x20b1975; continue;
 }
 0x20b1975 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x20b197f); f_020af0db(cpu); if cpu.returned_to != 0x20b197f { return; }
  { let v: u32 = cpu.esi; cpu.w32(0xffffffdcu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b1aa9; continue;
 }
 0x20b1987 => {
  cpu.push(0x20b198c); f_020c1d91(cpu); if cpu.returned_to != 0x20b198c { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0xb0u32); }
  if !cpu.cf { pc = 0x20b199a; continue; }
  pc = 0x20b1995; continue;
 }
 0x20b1995 => {
  cpu.push(0x20b199a); f_020ae3bb(cpu); if cpu.returned_to != 0x20b199a { return; }
  pc = 0x20b199a; continue;
 }
 0x20b199a => {
  { let v: u32 = 0x200u32; cpu.edi = v; }
  { cpu.sub32(cpu.edx, 0x230u32); }
  if !cpu.cf { pc = 0x20b19ac; continue; }
  pc = 0x20b19a7; continue;
 }
 0x20b19a7 => {
  { let v: u32 = 0x80u32; cpu.edi = v; }
  pc = 0x20b19ac; continue;
 }
 0x20b19ac => {
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
  pc = 0x20b19c8; continue;
 }
 0x20b19c8 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp))); }
  if !cpu.cf { pc = 0x20b1a73; continue; }
  pc = 0x20b19d4; continue;
 }
 0x20b19d4 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if !cpu.zf { pc = 0x20b1a28; continue; }
  pc = 0x20b19e0; continue;
 }
 0x20b19e0 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u8 = 0xdu8; cpu.w8(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1)), v); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.edi); }
  if !cpu.zf { pc = 0x20b1a28; continue; }
  pc = 0x20b19ec; continue;
 }
 0x20b19ec => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b193c; continue; }
  pc = 0x20b1a08; continue;
 }
 0x20b1a08 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if cpu.zf { pc = 0x20b1a20; continue; }
  pc = 0x20b1a0c; continue;
 }
 0x20b1a0c => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x20b1a16); f_020af0db(cpu); if cpu.returned_to != 0x20b1a16 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.esi); cpu.eax = v; }
  pc = 0x20b1aac; continue;
 }
 0x20b1a20 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b1a28; continue;
 }
 0x20b1a28 => {
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
  if !cpu.zf { cpu.poll(); pc = 0x20b19c8; continue; }
  pc = 0x20b1a46; continue;
 }
 0x20b1a46 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b193c; continue; }
  pc = 0x20b1a62; continue;
 }
 0x20b1a62 => {
  { cpu.sub32(cpu.eax, cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1a0c; continue; }
  pc = 0x20b1a66; continue;
 }
 0x20b1a66 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  cpu.poll(); pc = 0x20b19c8; continue;
 }
 0x20b1a73 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b1aa9; continue; }
  pc = 0x20b1a77; continue;
 }
 0x20b1a77 => {
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
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b193c; continue; }
  pc = 0x20b1a94; continue;
 }
 0x20b1a94 => {
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x20b1aa9; continue; }
  pc = 0x20b1a98; continue;
 }
 0x20b1a98 => {
  { let v: u32 = 0xcu32; cpu.eax = v; }
  cpu.push(0x20b1aa2); f_020af0db(cpu); if cpu.returned_to != 0x20b1aa2 { return; }
  { let v: u32 = cpu.r32(0xffffffe8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.edi); cpu.eax = v; }
  pc = 0x20b1aac; continue;
 }
 0x20b1aa9 => {
  { let v: u32 = cpu.r32(0xffffffdcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  pc = 0x20b1aac; continue;
 }
 0x20b1aac => {
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
pub fn f_020b1ab3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1ab3); }
 let mut pc: u32 = 0x20b1ab3;
 loop { match pc {
 0x20b1ab3 => {
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
  if (cpu.sf == cpu.of) { pc = 0x20b1aed; continue; }
  pc = 0x20b1adc; continue;
 }
 0x20b1adc => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20b1ae6); f_020c1c5a(cpu); if cpu.returned_to != 0x20b1ae6 { return; }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b1af0; continue;
 }
 0x20b1aed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  pc = 0x20b1af0; continue;
 }
 0x20b1af0 => {
  { let v: u32 = cpu.add32(cpu.esp, 0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1af6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1af6); }
 let mut pc: u32 = 0x20b1af6;
 loop { match pc {
 0x20b1af6 => {
  pc = 0x20c1d9a; continue;
 }
 0x20c1d9a => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x41u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20c1db7; continue; }
  pc = 0x20c1dab; continue;
 }
 0x20c1dab => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20c1db5); f_020c1c5a(cpu); if cpu.returned_to != 0x20c1db5 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20c1db7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1afb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1afb); }
 let mut pc: u32 = 0x20b1afb;
 loop { match pc {
 0x20b1afb => {
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
  if (cpu.sf == cpu.of) { pc = 0x20b1b2a; continue; }
  pc = 0x20b1b1c; continue;
 }
 0x20b1b1c => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  pc = 0x20b1b21; continue;
 }
 0x20b1b21 => {
  cpu.push(0x20b1b26); f_020af0db(cpu); if cpu.returned_to != 0x20b1b26 { return; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b1b77; continue;
 }
 0x20b1b2a => {
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
  if !cpu.zf { pc = 0x20b1b6e; continue; }
  pc = 0x20b1b58; continue;
 }
 0x20b1b58 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20b1b5f); f_020bd474(cpu); if cpu.returned_to != 0x20b1b5f { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b1b6c; continue; }
  pc = 0x20b1b65; continue;
 }
 0x20b1b65 => {
  { let v: u32 = 0x5u32; cpu.eax = v; }
  cpu.poll(); pc = 0x20b1b21; continue;
 }
 0x20b1b6c => {
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  pc = 0x20b1b6e; continue;
 }
 0x20b1b6e => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20b1b77); f_0209ebc4(cpu); if cpu.returned_to != 0x20b1b77 { return; }
  pc = 0x20b1b77; continue;
 }
 0x20b1b77 => {
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
pub fn f_020b1b83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1b83); }
 let mut pc: u32 = 0x20b1b83;
 loop { match pc {
 0x20b1b83 => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u8 = 0x3bu8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.rcl32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.ror32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b1ba0; continue; }
  pc = 0x20b1b94; continue;
 }
 0x20b1b94 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  cpu.push(0x20b1b9e); f_020c1c5a(cpu); if cpu.returned_to != 0x20b1b9e { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b1ba0 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1ba4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1ba4); }
 let mut pc: u32 = 0x20b1ba4;
 loop { match pc {
 0x20b1ba4 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  cpu.push(0x20b1bad); f_020a3db4(cpu); if cpu.returned_to != 0x20b1bad { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b1bce; continue; }
  pc = 0x20b1bb4; continue;
 }
 0x20b1bb4 => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x20b1bbb); f_020b1bd1(cpu); if cpu.returned_to != 0x20b1bbb { return; }
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if cpu.zf { pc = 0x20b1bce; continue; }
  pc = 0x20b1bc0; continue;
 }
 0x20b1bc0 => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { pc = 0x20b1bcc; continue; }
  pc = 0x20b1bc4; continue;
 }
 0x20b1bc4 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b1bcc => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  pc = 0x20b1bce; continue;
 }
 0x20b1bce => {
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1bd1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1bd1); }
 let mut pc: u32 = 0x20b1bd1;
 loop { match pc {
 0x20b1bd1 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.push(0x20b1bdf); f_020b1ab3(cpu); if cpu.returned_to != 0x20b1bdf { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1be2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1be2); }
 let mut pc: u32 = 0x20b1be2;
 loop { match pc {
 0x20b1be2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  cpu.push(0x20b1bee); f_020c1dbb(cpu); if cpu.returned_to != 0x20b1bee { return; }
  { let v: u32 = cpu.inc32(cpu.r32(0x10u32.wrapping_add(cpu.ebx))); cpu.w32(0x10u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1bf3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1bf3); }
 let mut pc: u32 = 0x20b1bf3;
 loop { match pc {
 0x20b1bf3 => {
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
  if !cpu.zf { pc = 0x20b1c16; continue; }
  pc = 0x20b1c0f; continue;
 }
 0x20b1c0f => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b1c16); f_020c1e5f(cpu); if cpu.returned_to != 0x20b1c16 { return; }
  pc = 0x20b1c16; continue;
 }
 0x20b1c16 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.ecx as u8), 0x4u8); }
  if cpu.zf { pc = 0x20b1c31; continue; }
  pc = 0x20b1c20; continue;
 }
 0x20b1c20 => {
  { let v: u8 = (cpu.ecx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfau8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) | 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  pc = 0x20b1c31; continue;
 }
 0x20b1c31 => {
  { let v: u32 = 0x20b1be2u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  cpu.push(0x20b1c3d); f_020af268(cpu); if cpu.returned_to != 0x20b1c3d { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.edi, cpu.edi); }
  if cpu.zf { pc = 0x20b1c58; continue; }
  pc = 0x20b1c43; continue;
 }
 0x20b1c43 => {
  { let v: u8 = cpu.r8(0xdu32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8) & 0xfau8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ebx as u8) | 0x4u8; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u8 = (cpu.ebx as u8); cpu.w8(0xdu32.wrapping_add(cpu.esi), v); }
  cpu.push(0x20b1c58); f_020c1ed6(cpu); if cpu.returned_to != 0x20b1c58 { return; }
  pc = 0x20b1c58; continue;
 }
 0x20b1c58 => {
  { cpu.and8(cpu.r8(0xcu32.wrapping_add(cpu.esi)), 0x20u8); }
  if cpu.zf { pc = 0x20b1c63; continue; }
  pc = 0x20b1c5e; continue;
 }
 0x20b1c5e => {
  { let v: u32 = 0xffffffffu32; cpu.edx = v; }
  pc = 0x20b1c63; continue;
 }
 0x20b1c63 => {
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
pub fn f_020b1c74(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b1c74); } r_020b1c74(cpu, 0x20b1c74); }
pub fn f_020b1c7c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b1c7c); } r_020b1c74(cpu, 0x20b1c7c); }
fn r_020b1c74(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b1c74 => {
  cpu.unimplemented(0x20b1c74, "Insb");
  { let v: u32 = cpu.imul2_32(cpu.r32(0x32u32.wrapping_add(cpu.ebp)), 0xf8830078u32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.esp = v; }
  if cpu.zf { pc = 0x20b1c86; continue; }
  pc = 0x20b1c81; continue;
 }
 0x20b1c7c => {
  { cpu.sub32(cpu.eax, 0x5cu32); }
  if cpu.zf { pc = 0x20b1c86; continue; }
  pc = 0x20b1c81; continue;
 }
 0x20b1c81 => {
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x20b1c8f; continue; }
  pc = 0x20b1c86; continue;
 }
 0x20b1c86 => {
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.edx)), 0x0u32); }
  if !cpu.zf { pc = 0x20b1c8d; continue; }
  pc = 0x20b1c8b; continue;
 }
 0x20b1c8b => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edx), v); }
  pc = 0x20b1c8d; continue;
 }
 0x20b1c8d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  pc = 0x20b1c8f; continue;
 }
 0x20b1c8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b1c90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1c90); }
 let mut pc: u32 = 0x20b1c90;
 loop { match pc {
 0x20b1c90 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b1cb4; continue; }
  pc = 0x20b1ca5; continue;
 }
 0x20b1ca5 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if cpu.zf { pc = 0x20b1cb4; continue; }
  pc = 0x20b1cab; continue;
 }
 0x20b1cab => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u8 = 0x3au8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20b1cb4; continue;
 }
 0x20b1cb4 => {
  { let v: u8 = 0x0u8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b1d72; continue; }
  pc = 0x20b1cc0; continue;
 }
 0x20b1cc0 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x20b1d72; continue; }
  pc = 0x20b1cc9; continue;
 }
 0x20b1cc9 => {
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x20b1cd4); f_020c203c(cpu); if cpu.returned_to != 0x20b1cd4 { return; }
  cpu.push(0x20b1cd9); f_020b1c7c(cpu); if cpu.returned_to != 0x20b1cd9 { return; }
  cpu.push(0x20b1cde); f_020c2094(cpu); if cpu.returned_to != 0x20b1cde { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20b1ce3; continue;
 }
 0x20b1ce3 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b1cfb; continue; }
  pc = 0x20b1ceb; continue;
 }
 0x20b1ceb => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1ce3; continue; }
  pc = 0x20b1cfb; continue;
 }
 0x20b1cfb => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x217c9ecu32), 0x0u32); }
  if cpu.zf { pc = 0x20b1d19; continue; }
  pc = 0x20b1d05; continue;
 }
 0x20b1d05 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x20b1d1b; continue;
 }
 0x20b1d19 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b1d1b; continue;
 }
 0x20b1d1b => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b1d38; continue; }
  pc = 0x20b1d27; continue;
 }
 0x20b1d27 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x20b1d38; continue;
 }
 0x20b1d38 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1cc9; continue; }
  pc = 0x20b1d40; continue;
 }
 0x20b1d40 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b1d4f; continue; }
  pc = 0x20b1d47; continue;
 }
 0x20b1d47 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b1d4f; continue;
 }
 0x20b1d4f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.ebp; cpu.edx = v; }
  cpu.push(0x20b1d59); f_020c2308(cpu); if cpu.returned_to != 0x20b1d59 { return; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x20b1d6b; continue; }
  pc = 0x20b1d68; continue;
 }
 0x20b1d68 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20b1d72; continue;
 }
 0x20b1d6b => {
  { let v: u8 = cpu.r8(0x4u32.wrapping_add(cpu.esp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b1d72; continue;
 }
 0x20b1d72 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.esp)), 0x0u32); }
  if !cpu.zf { pc = 0x20b1d81; continue; }
  pc = 0x20b1d79; continue;
 }
 0x20b1d79 => {
  { let v: u32 = 0x5cu32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b1d81; continue;
 }
 0x20b1d81 => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x20b1e29; continue; }
  pc = 0x20b1d8c; continue;
 }
 0x20b1d8c => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x20b1d97); f_020c203c(cpu); if cpu.returned_to != 0x20b1d97 { return; }
  cpu.push(0x20b1d9c); f_020b1c7c(cpu); if cpu.returned_to != 0x20b1d9c { return; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if cpu.zf { pc = 0x20b1daf; continue; }
  pc = 0x20b1da4; continue;
 }
 0x20b1da4 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if !cpu.zf { pc = 0x20b1daf; continue; }
  pc = 0x20b1dad; continue;
 }
 0x20b1dad => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x20b1daf; continue;
 }
 0x20b1daf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x20b1dba); f_020c203c(cpu); if cpu.returned_to != 0x20b1dba { return; }
  cpu.push(0x20b1dbf); f_020b1c7c(cpu); if cpu.returned_to != 0x20b1dbf { return; }
  cpu.push(0x20b1dc4); f_020c2094(cpu); if cpu.returned_to != 0x20b1dc4 { return; }
  { let v: u32 = cpu.ebp; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x20b1dc9; continue;
 }
 0x20b1dc9 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if cpu.zf { pc = 0x20b1de1; continue; }
  pc = 0x20b1dd1; continue;
 }
 0x20b1dd1 => {
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x1u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub8((cpu.eax as u8), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1dc9; continue; }
  pc = 0x20b1de1; continue;
 }
 0x20b1de1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b1dfd; continue; }
  pc = 0x20b1deb; continue;
 }
 0x20b1deb => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x20b1dfd; continue;
 }
 0x20b1dfd => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x217c9ecu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b1e1d; continue; }
  pc = 0x20b1e0a; continue;
 }
 0x20b1e0a => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ecx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217c9f1u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8) & 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  pc = 0x20b1e1f; continue;
 }
 0x20b1e1d => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  pc = 0x20b1e1f; continue;
 }
 0x20b1e1f => {
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.eax); cpu.ecx = v; }
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ecx)), 0x0u8); }
  if !cpu.zf { cpu.poll(); pc = 0x20b1daf; continue; }
  pc = 0x20b1e27; continue;
 }
 0x20b1e27 => {
  pc = 0x20b1e38; continue;
 }
 0x20b1e29 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if !cpu.zf { pc = 0x20b1e38; continue; }
  pc = 0x20b1e36; continue;
 }
 0x20b1e36 => {
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  pc = 0x20b1e38; continue;
 }
 0x20b1e38 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x20b1e6e; continue; }
  pc = 0x20b1e40; continue;
 }
 0x20b1e40 => {
  { cpu.sub8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), 0x0u8); }
  if cpu.zf { pc = 0x20b1e6e; continue; }
  pc = 0x20b1e45; continue;
 }
 0x20b1e45 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if cpu.zf { pc = 0x20b1e57; continue; }
  pc = 0x20b1e52; continue;
 }
 0x20b1e52 => {
  { let v: u8 = 0x2eu8; cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20b1e57; continue;
 }
 0x20b1e57 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x20b1e6e; continue; }
  pc = 0x20b1e61; continue;
 }
 0x20b1e61 => {
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = 0x1u32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x20b1e57; continue;
 }
 0x20b1e6e => {
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
pub fn f_020b1e7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1e7b); }
 let mut pc: u32 = 0x20b1e7b;
 loop { match pc {
 0x20b1e7b => {
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
  if cpu.zf { pc = 0x20b1f2d; continue; }
  pc = 0x20b1f0b; continue;
 }
 0x20b1f0b => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.ebx)), 0x17u32); }
  if cpu.zf { pc = 0x20b1f2d; continue; }
  pc = 0x20b1f11; continue;
 }
 0x20b1f11 => {
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
  pc = 0x20b1f2d; continue;
 }
 0x20b1f2d => {
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
pub fn f_020b1f3e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b1f3e); }
 let mut pc: u32 = 0x20b1f3e;
 loop { match pc {
 0x20b1f3e => {
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
  { let v: u32 = 0x216b6dcu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { cpu.sub32(cpu.esi, 0xf4fb7ea1u32); }
  if (cpu.sf == cpu.of) { pc = 0x20b1f72; continue; }
  pc = 0x20b1f68; continue;
 }
 0x20b1f68 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  pc = 0x20b208b; continue;
 }
 0x20b1f72 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xcu32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x20b1f83; continue;
 }
 0x20b1f83 => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if (cpu.sf == cpu.of) { pc = 0x20b1f8d; continue; }
  pc = 0x20b1f87; continue;
 }
 0x20b1f87 => {
  { let v: u32 = cpu.dec32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0xcu32); cpu.edi = v; }
  cpu.poll(); pc = 0x20b1f83; continue;
 }
 0x20b1f8d => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1f68; continue; }
  pc = 0x20b1f91; continue;
 }
 0x20b1f91 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.esi); cpu.eax = v; }
  cpu.push(0x20b1f9c); f_020b25a2(cpu); if cpu.returned_to != 0x20b1f9c { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b1fa5; continue; }
  pc = 0x20b1fa0; continue;
 }
 0x20b1fa0 => {
  { let v: u32 = 0x216b6f6u32; cpu.ebx = v; }
  pc = 0x20b1fa5; continue;
 }
 0x20b1fa5 => {
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
  if cpu.zf { pc = 0x20b1fd2; continue; }
  pc = 0x20b1fd1; continue;
 }
 0x20b1fd1 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b1fd2; continue;
 }
 0x20b1fd2 => {
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
  pc = 0x20b1ff3; continue;
 }
 0x20b1ff3 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20b2000; continue; }
  pc = 0x20b1ff7; continue;
 }
 0x20b1ff7 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x20b1ff3; continue;
 }
 0x20b2000 => {
  { let v: u32 = cpu.ebp; cpu.ecx = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  cpu.push(0x20b200d); f_020b2462(cpu); if cpu.returned_to != 0x20b200d { return; }
  cpu.push(0x20b2012); f_020b20fe(cpu); if cpu.returned_to != 0x20b2012 { return; }
  { let v: u32 = cpu.r32(0x217b998u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf == cpu.of) { pc = 0x20b2028; continue; }
  pc = 0x20b2021; continue;
 }
 0x20b2021 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  cpu.push(0x20b2028); f_020b26f6(cpu); if cpu.returned_to != 0x20b2028 { return; }
  pc = 0x20b2028; continue;
 }
 0x20b2028 => {
  { cpu.sub32(cpu.r32(0x20u32.wrapping_add(cpu.ebp)), 0x0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b2034; continue; }
  pc = 0x20b202e; continue;
 }
 0x20b202e => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x217b99cu32)); cpu.esi = v; }
  pc = 0x20b2034; continue;
 }
 0x20b2034 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20b2041; continue; }
  pc = 0x20b2038; continue;
 }
 0x20b2038 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x15180u32); cpu.esi = v; }
  cpu.poll(); pc = 0x20b2034; continue;
 }
 0x20b2041 => {
  { cpu.sub32(cpu.edi, 0x63deu32); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1f68; continue; }
  pc = 0x20b204d; continue;
 }
 0x20b204d => {
  if !cpu.zf { pc = 0x20b206b; continue; }
  pc = 0x20b204f; continue;
 }
 0x20b204f => {
  { let v: u32 = cpu.r32(0x217b998u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x15180u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x20b1f68; continue; }
  pc = 0x20b2062; continue;
 }
 0x20b2062 => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x20b2089; continue; }
  pc = 0x20b2066; continue;
 }
 0x20b2066 => {
  cpu.poll(); pc = 0x20b1f68; continue;
 }
 0x20b206b => {
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
  pc = 0x20b2089; continue;
 }
 0x20b2089 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  pc = 0x20b208b; continue;
 }
 0x20b208b => {
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
pub fn f_020b2092(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2092); }
 let mut pc: u32 = 0x20b2092;
 loop { match pc {
 0x20b2092 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217b9eau32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x217b9eau32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0x1u32; cpu.eax = v; }
  { let v: u8 = (cpu.ecx as u8) & 0xfeu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u8 = (cpu.ecx as u8); cpu.w8(0x217b9eau32, v); }
  { let v: u32 = !cpu.ebx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x217b9eau32); cpu.ecx = v; }
  { let v: u32 = cpu.ebx & 0x1u32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x217b9eau32, v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  { let v: u8 = (!cpu.zf) as u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x217b9eau32); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & 0x1u32; cpu.edx = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8) & 0xfdu8; cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.w8(0x217b9eau32, v); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.r32(0x217b9eau32), cpu.edx); cpu.w32(0x217b9eau32, v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b20fe(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b20fe); }
 let mut pc: u32 = 0x20b20fe;
 loop { match pc {
 0x20b20e6 => {
  { let v: u8 = cpu.r8(0x217b9eau32); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), 0x1u8); }
  if cpu.zf { pc = 0x20b20f6; continue; }
  pc = 0x20b20f1; continue;
 }
 0x20b20f1 => {
  { cpu.and8(((cpu.eax >> 8) as u8), 0x2u8); }
  if !cpu.zf { pc = 0x20b20fd; continue; }
  pc = 0x20b20f6; continue;
 }
 0x20b20f6 => {
  { let v: u8 = cpu.or8(cpu.r8(0x217b9eau32), 0x2u8); cpu.w8(0x217b9eau32, v); }
  pc = 0x20b20fd; continue;
 }
 0x20b20fd => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b20fe => {
  { let v: u32 = 0x216ac8cu32; cpu.eax = v; }
  cpu.push(0x20b2108); f_020bd5d9(cpu); if cpu.returned_to != 0x20b2108 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { cpu.poll(); pc = 0x20b20e6; continue; }
  pc = 0x20b210c; continue;
 }
 0x20b210c => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x4u32); cpu.esp = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x217b998u32; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x217b9a0u32, v); }
  { let v: u32 = 0x217b9a4u32; cpu.edx = v; }
  cpu.push(0x20b212c); f_020b2167(cpu); if cpu.returned_to != 0x20b212c { return; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and8(((cpu.eax >> 8) as u8), ((cpu.eax >> 8) as u8)); }
  if !cpu.zf { pc = 0x20b2394; continue; }
  pc = 0x20b2138; continue;
 }
 0x20b2138 => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x217b9c3u32, v); }
  pc = 0x20b2458; continue;
 }
 0x20b2394 => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  { let v: u32 = 0x217b9c3u32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x217b998u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b9a0u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(0xe10u32); cpu.eax = v; }
  { let v: u32 = cpu.esp; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20b23ba); f_020b2167(cpu); if cpu.returned_to != 0x20b23ba { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x217b998u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ecx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x217b99cu32, v); }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.sub32(cpu.ebx, 0x2cu32); }
  if !cpu.zf { pc = 0x20b23e3; continue; }
  pc = 0x20b23d6; continue;
 }
 0x20b23d6 => {
  { let v: u32 = 0x217b950u32; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x20b23e1); f_020b2290(cpu); if cpu.returned_to != 0x20b23e1 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  pc = 0x20b23e3; continue;
 }
 0x20b23e3 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if !cpu.zf { pc = 0x20b2458; continue; }
  pc = 0x20b23ec; continue;
 }
 0x20b23ec => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0x217b974u32; cpu.edx = v; }
  cpu.push(0x20b23f9); f_020b2290(cpu); if cpu.returned_to != 0x20b23f9 { return; }
  { let v: u32 = cpu.r32(0x217b99cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0xe10u32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x217b97cu32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x217b99cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x3cu32; cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x217b978u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x217b99cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x217b974u32); cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x217b97cu32, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x217b978u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x217b974u32, v); }
  pc = 0x20b2458; continue;
 }
 0x20b2458 => {
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
pub fn f_020b2143(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2143); }
 let mut pc: u32 = 0x20b2143;
 loop { match pc {
 0x20b2143 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x20b2149; continue;
 }
 0x20b2149 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.edx, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b2162; continue; }
  pc = 0x20b2152; continue;
 }
 0x20b2152 => {
  { cpu.sub32(cpu.edx, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b2162; continue; }
  pc = 0x20b2157; continue;
 }
 0x20b2157 => {
  { let v: u32 = cpu.imul2_32(cpu.ebx, 0xau32); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0xffffffd0u32.wrapping_add(cpu.edx); cpu.ebx = v; }
  cpu.poll(); pc = 0x20b2149; continue;
 }
 0x20b2162 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b2167(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2167); }
 let mut pc: u32 = 0x20b2167;
 loop { match pc {
 0x20b2167 => {
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
  if !cpu.zf { pc = 0x20b217f; continue; }
  pc = 0x20b217e; continue;
 }
 0x20b217e => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  pc = 0x20b217f; continue;
 }
 0x20b217f => {
  { let v: u32 = cpu.ebp; cpu.esi = v; }
  pc = 0x20b2181; continue;
 }
 0x20b2181 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20b21a8; continue; }
  pc = 0x20b2188; continue;
 }
 0x20b2188 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = (cpu.edx as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2cu32); }
  if cpu.zf { pc = 0x20b21a8; continue; }
  pc = 0x20b2191; continue;
 }
 0x20b2191 => {
  { cpu.sub32(cpu.eax, 0x2du32); }
  if cpu.zf { pc = 0x20b21a8; continue; }
  pc = 0x20b2196; continue;
 }
 0x20b2196 => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if cpu.zf { pc = 0x20b21a8; continue; }
  pc = 0x20b219b; continue;
 }
 0x20b219b => {
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b21a5; continue; }
  pc = 0x20b21a0; continue;
 }
 0x20b21a0 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b21a8; continue; }
  pc = 0x20b21a5; continue;
 }
 0x20b21a5 => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  cpu.poll(); pc = 0x20b2181; continue;
 }
 0x20b21a8 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { cpu.sub32(cpu.eax, 0x1eu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b21bd; continue; }
  pc = 0x20b21b5; continue;
 }
 0x20b21b5 => {
  { let v: u32 = 0x1eu32; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  pc = 0x20b21bd; continue;
 }
 0x20b21bd => {
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
  if !cpu.zf { pc = 0x20b21fa; continue; }
  pc = 0x20b21f3; continue;
 }
 0x20b21f3 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  pc = 0x20b21ff; continue;
 }
 0x20b21fa => {
  { cpu.sub32(cpu.eax, 0x2bu32); }
  if !cpu.zf { pc = 0x20b2201; continue; }
  pc = 0x20b21ff; continue;
 }
 0x20b21ff => {
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x20b2201; continue;
 }
 0x20b2201 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax & 0xffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x30u32); }
  if (cpu.sf != cpu.of) { pc = 0x20b2286; continue; }
  pc = 0x20b2212; continue;
 }
 0x20b2212 => {
  { cpu.sub32(cpu.eax, 0x39u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b2286; continue; }
  pc = 0x20b2217; continue;
 }
 0x20b2217 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  cpu.push(0x20b222f); f_020b2143(cpu); if cpu.returned_to != 0x20b222f { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x20b2259; continue; }
  pc = 0x20b223a; continue;
 }
 0x20b223a => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x20b2244); f_020b2143(cpu); if cpu.returned_to != 0x20b2244 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x20b2259; continue; }
  pc = 0x20b224f; continue;
 }
 0x20b224f => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x20b2257); f_020b2143(cpu); if cpu.returned_to != 0x20b2257 { return; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x20b2259; continue;
 }
 0x20b2259 => {
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
  if cpu.zf { pc = 0x20b2286; continue; }
  pc = 0x20b2282; continue;
 }
 0x20b2282 => {
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x20b2286; continue;
 }
 0x20b2286 => {
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
pub fn f_020b2290(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2290); }
 let mut pc: u32 = 0x20b2290;
 loop { match pc {
 0x20b2290 => {
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
  if !cpu.zf { pc = 0x20b22af; continue; }
  pc = 0x20b22a8; continue;
 }
 0x20b22a8 => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  pc = 0x20b22af; continue;
 }
 0x20b22af => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x4du32); }
  if !cpu.zf { pc = 0x20b22bb; continue; }
  pc = 0x20b22b8; continue;
 }
 0x20b22b8 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  pc = 0x20b22bb; continue;
 }
 0x20b22bb => {
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x20u32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x20b22c9); f_020b2143(cpu); if cpu.returned_to != 0x20b22c9 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b22da; continue; }
  pc = 0x20b22d1; continue;
 }
 0x20b22d1 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b2327; continue;
 }
 0x20b22da => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x20b2320; continue; }
  pc = 0x20b22eb; continue;
 }
 0x20b22eb => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x20b22f7); f_020b2143(cpu); if cpu.returned_to != 0x20b22f7 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2eu32); }
  if !cpu.zf { pc = 0x20b2320; continue; }
  pc = 0x20b230b; continue;
 }
 0x20b230b => {
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = 0xcu32.wrapping_add(cpu.esp); cpu.edx = v; }
  cpu.push(0x20b2317); f_020b2143(cpu); if cpu.returned_to != 0x20b2317 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b2320; continue;
 }
 0x20b2320 => {
  { let v: u32 = 0x0u32; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  pc = 0x20b2327; continue;
 }
 0x20b2327 => {
  { let v: u32 = 0x2u32; cpu.edx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esi; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub32(cpu.eax, 0x2fu32); }
  if !cpu.zf { pc = 0x20b2378; continue; }
  pc = 0x20b2342; continue;
 }
 0x20b2342 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.ebx); cpu.eax = v; }
  cpu.push(0x20b234e); f_020b2143(cpu); if cpu.returned_to != 0x20b234e { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x20b2378; continue; }
  pc = 0x20b2359; continue;
 }
 0x20b2359 => {
  { let v: u32 = 0x4u32.wrapping_add(cpu.esp); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x20b2363); f_020b2143(cpu); if cpu.returned_to != 0x20b2363 { return; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.edx, 0x3au32); }
  if !cpu.zf { pc = 0x20b2378; continue; }
  pc = 0x20b236e; continue;
 }
 0x20b236e => {
  { let v: u32 = cpu.esp; cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  cpu.push(0x20b2376); f_020b2143(cpu); if cpu.returned_to != 0x20b2376 { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  pc = 0x20b2378; continue;
 }
 0x20b2378 => {
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
pub fn f_020b2462(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2462); }
 let mut pc: u32 = 0x20b2462;
 loop { match pc {
 0x20b2462 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { cpu.sub32(cpu.edx, 0xa8c0u32); }
  if !cpu.cf { pc = 0x20b248f; continue; }
  pc = 0x20b2473; continue;
 }
 0x20b2473 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b248f; continue; }
  pc = 0x20b2477; continue;
 }
 0x20b2477 => {
  { let v: u32 = 0x15180u32.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  pc = 0x20b249e; continue;
 }
 0x20b248f => {
  { let v: u32 = 0x15180u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v = cpu.ebx; cpu.div32(v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  pc = 0x20b249e; continue;
 }
 0x20b249e => {
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
  if (cpu.cf || cpu.zf) { pc = 0x20b2505; continue; }
  pc = 0x20b24fd; continue;
 }
 0x20b24fd => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  pc = 0x20b2505; continue;
 }
 0x20b2505 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x20b2510); f_020b25a2(cpu); if cpu.returned_to != 0x20b2510 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x16du32); cpu.eax = v; }
  pc = 0x20b2515; continue;
 }
 0x20b2515 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if cpu.cf { pc = 0x20b251e; continue; }
  pc = 0x20b2519; continue;
 }
 0x20b2519 => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  cpu.poll(); pc = 0x20b2515; continue;
 }
 0x20b251e => {
  { let v: u32 = 0x216b6dcu32; cpu.esi = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1cu32.wrapping_add(cpu.ecx), v); }
  cpu.push(0x20b2534); f_020b25a2(cpu); if cpu.returned_to != 0x20b2534 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b253d; continue; }
  pc = 0x20b2538; continue;
 }
 0x20b2538 => {
  { let v: u32 = 0x216b6f6u32; cpu.esi = v; }
  pc = 0x20b253d; continue;
 }
 0x20b253d => {
  { let v: u32 = 0x1fu32; cpu.ebp = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v = cpu.ebp; cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.edx.wrapping_mul(2))); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if cpu.cf { pc = 0x20b2555; continue; }
  pc = 0x20b2554; continue;
 }
 0x20b2554 => {
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x20b2555; continue;
 }
 0x20b2555 => {
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
pub fn f_020b2577(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2577); }
 let mut pc: u32 = 0x20b2577;
 loop { match pc {
 0x20b2577 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x20u32.wrapping_add(cpu.edx), v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x63dfu32; cpu.eax = v; }
  cpu.push(0x20b2592); f_020b2462(cpu); if cpu.returned_to != 0x20b2592 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b25a2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b25a2); }
 let mut pc: u32 = 0x20b25a2;
 loop { match pc {
 0x20b25a2 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.and8((cpu.ebx as u8), 0x3u8); }
  if !cpu.zf { pc = 0x20b25d7; continue; }
  pc = 0x20b25ac; continue;
 }
 0x20b25ac => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x64u32; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b25c5; continue; }
  pc = 0x20b25bc; continue;
 }
 0x20b25bc => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b25c5 => {
  { let v: u32 = 0x190u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ecx; cpu.idiv32(v); }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { cpu.poll(); pc = 0x20b25bc; continue; }
  pc = 0x20b25d7; continue;
 }
 0x20b25d7 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b25dd(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b25dd); }
 let mut pc: u32 = 0x20b25dd;
 loop { match pc {
 0x20b25dd => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x24u32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if !cpu.zf { pc = 0x20b26aa; continue; }
  pc = 0x20b25f2; continue;
 }
 0x20b25f2 => {
  { let v: u32 = 0x76cu32.wrapping_add(cpu.edx); cpu.eax = v; }
  cpu.push(0x20b25fd); f_020b25a2(cpu); if cpu.returned_to != 0x20b25fd { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b2614; continue; }
  pc = 0x20b2601; continue;
 }
 0x20b2601 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x216b6f6u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x216b6f4u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x20b2625; continue;
 }
 0x20b2614 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x216b6dcu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x216b6dau32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.eax = v; }
  pc = 0x20b2625; continue;
 }
 0x20b2625 => {
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
  cpu.push(0x20b2659); f_020b1f3e(cpu); if cpu.returned_to != 0x20b2659 { return; }
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
  if !cpu.zf { pc = 0x20b2698; continue; }
  pc = 0x20b2679; continue;
 }
 0x20b2679 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.edi, 3u32); cpu.edi = v; }
  { let v: u32 = 0x1u32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b2694; continue; }
  pc = 0x20b268c; continue;
 }
 0x20b268c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x2u32); cpu.eax = v; }
  pc = 0x20b2699; continue;
 }
 0x20b2694 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  pc = 0x20b2699; continue;
 }
 0x20b2698 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  pc = 0x20b2699; continue;
 }
 0x20b2699 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  pc = 0x20b26b9; continue;
 }
 0x20b26aa => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x20b26b6; continue; }
  pc = 0x20b26af; continue;
 }
 0x20b26af => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  pc = 0x20b26b9; continue;
 }
 0x20b26b6 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x20b26b9; continue;
 }
 0x20b26b9 => {
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
pub fn f_020b26c2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b26c2); }
 let mut pc: u32 = 0x20b26c2;
 loop { match pc {
 0x20b26c2 => {
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.edx))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b26d6; continue; }
  pc = 0x20b26ce; continue;
 }
 0x20b26ce => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b26d6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x10u32.wrapping_add(cpu.ecx))); }
  if (cpu.sf != cpu.of) { pc = 0x20b26f1; continue; }
  pc = 0x20b26db; continue;
 }
 0x20b26db => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  cpu.push(0x20b26e2); f_020b25dd(cpu); if cpu.returned_to != 0x20b26e2 { return; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20b26ed); f_020b25dd(cpu); if cpu.returned_to != 0x20b26ed { return; }
  { cpu.sub32(cpu.esi, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b26ce; continue; }
  pc = 0x20b26f1; continue;
 }
 0x20b26f1 => {
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b26f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b26f6); }
 let mut pc: u32 = 0x20b26f6;
 loop { match pc {
 0x20b26f6 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp.wrapping_sub(0x1cu32); cpu.esp = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x217b9e6u32); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.and8((cpu.edx as u8), (cpu.edx as u8)); }
  if cpu.zf { pc = 0x20b29aa; continue; }
  pc = 0x20b2712; continue;
 }
 0x20b2712 => {
  { let v: u32 = 0x217b974u32; cpu.edx = v; }
  { let v: u32 = 0x217b950u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.ebx = v; }
  cpu.push(0x20b2724); f_020b26c2(cpu); if cpu.returned_to != 0x20b2724 { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b2738; continue; }
  pc = 0x20b272c; continue;
 }
 0x20b272c => {
  { let v: u32 = 0x217b974u32; cpu.esi = v; }
  { let v: u32 = 0x217b950u32; cpu.ebx = v; }
  pc = 0x20b2742; continue;
 }
 0x20b2738 => {
  { let v: u32 = 0x217b950u32; cpu.esi = v; }
  { let v: u32 = 0x217b974u32; cpu.ebx = v; }
  pc = 0x20b2742; continue;
 }
 0x20b2742 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ecx)); cpu.ebp = v; }
  cpu.push(0x20b2752); f_020b25a2(cpu); if cpu.returned_to != 0x20b2752 { return; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebp).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.eax = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b276a; continue; }
  pc = 0x20b275c; continue;
 }
 0x20b275c => {
  { let v: u32 = cpu.r32(0x216b6f6u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x216b6f4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x20b2776; continue;
 }
 0x20b276a => {
  { let v: u32 = cpu.r32(0x216b6dcu32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x216b6dau32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x20b2776; continue;
 }
 0x20b2776 => {
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xcu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b2846; continue; }
  pc = 0x20b2793; continue;
 }
 0x20b2793 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b27a4; continue; }
  pc = 0x20b279a; continue;
 }
 0x20b279a => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x20b287f; continue;
 }
 0x20b27a4 => {
  if !cpu.zf { pc = 0x20b287f; continue; }
  pc = 0x20b27aa; continue;
 }
 0x20b27aa => {
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
  if !cpu.zf { pc = 0x20b281f; continue; }
  pc = 0x20b27fa; continue;
 }
 0x20b27fa => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x20b287f; continue; }
  pc = 0x20b280c; continue;
 }
 0x20b280c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x20b287f; continue; }
  pc = 0x20b281d; continue;
 }
 0x20b281d => {
  pc = 0x20b287b; continue;
 }
 0x20b281f => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b287f; continue; }
  pc = 0x20b2837; continue;
 }
 0x20b2837 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = 0x1u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b287f; continue; }
  pc = 0x20b2844; continue;
 }
 0x20b2844 => {
  pc = 0x20b287b; continue;
 }
 0x20b2846 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20b286d; continue; }
  pc = 0x20b284e; continue;
 }
 0x20b284e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x20b285b); f_020b25a2(cpu); if cpu.returned_to != 0x20b285b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b286c; continue; }
  pc = 0x20b285f; continue;
 }
 0x20b285f => {
  { let v: u32 = cpu.r32(0x216b6deu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b286c; continue; }
  pc = 0x20b286b; continue;
 }
 0x20b286b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20b286c; continue;
 }
 0x20b286c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x20b286d; continue;
 }
 0x20b286d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b287f; continue; }
  pc = 0x20b2874; continue;
 }
 0x20b2874 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  if !cpu.zf { pc = 0x20b287f; continue; }
  pc = 0x20b287b; continue;
 }
 0x20b287b => {
  { let v: u32 = cpu.edi; cpu.w32(0x10u32.wrapping_add(cpu.esp), v); }
  pc = 0x20b287f; continue;
 }
 0x20b287f => {
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esp)), 0x0u32); }
  if cpu.zf { pc = 0x20b289c; continue; }
  pc = 0x20b2886; continue;
 }
 0x20b2886 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20b288f); f_020b29b9(cpu); if cpu.returned_to != 0x20b288f { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  { let v: u8 = (cpu.zf) as u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  { let v: u32 = cpu.edi & 0xffu32; cpu.edi = v; }
  pc = 0x20b289c; continue;
 }
 0x20b289c => {
  { cpu.and32(cpu.edi, cpu.edi); }
  if !cpu.zf { pc = 0x20b28b3; continue; }
  pc = 0x20b28a0; continue;
 }
 0x20b28a0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if cpu.zf { pc = 0x20b29aa; continue; }
  pc = 0x20b28ac; continue;
 }
 0x20b28ac => {
  { let v: u32 = cpu.edx; cpu.edi = v; }
  pc = 0x20b29aa; continue;
 }
 0x20b28b3 => {
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { pc = 0x20b2956; continue; }
  pc = 0x20b28c0; continue;
 }
 0x20b28c0 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.sub32(cpu.ebp, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b28ce; continue; }
  pc = 0x20b28c7; continue;
 }
 0x20b28c7 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  pc = 0x20b298f; continue;
 }
 0x20b28ce => {
  if !cpu.zf { pc = 0x20b298f; continue; }
  pc = 0x20b28d4; continue;
 }
 0x20b28d4 => {
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
  if !cpu.zf { pc = 0x20b2937; continue; }
  pc = 0x20b2918; continue;
 }
 0x20b2918 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(0x7u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x20b2931; continue; }
  pc = 0x20b2927; continue;
 }
 0x20b2927 => {
  { let v: u32 = 0x1u32; cpu.edi = v; }
  pc = 0x20b298f; continue;
 }
 0x20b2931 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b298f; continue; }
  pc = 0x20b2935; continue;
 }
 0x20b2935 => {
  pc = 0x20b298a; continue;
 }
 0x20b2937 => {
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x20b2927; continue; }
  pc = 0x20b2950; continue;
 }
 0x20b2950 => {
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x20b298f; continue; }
  pc = 0x20b2954; continue;
 }
 0x20b2954 => {
  pc = 0x20b298a; continue;
 }
 0x20b2956 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if !cpu.zf { pc = 0x20b297d; continue; }
  pc = 0x20b295e; continue;
 }
 0x20b295e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x76cu32); cpu.eax = v; }
  cpu.push(0x20b296b); f_020b25a2(cpu); if cpu.returned_to != 0x20b296b { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b297c; continue; }
  pc = 0x20b296f; continue;
 }
 0x20b296f => {
  { let v: u32 = cpu.r32(0x216b6deu32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x20b297c; continue; }
  pc = 0x20b297b; continue;
 }
 0x20b297b => {
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x20b297c; continue;
 }
 0x20b297c => {
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  pc = 0x20b297d; continue;
 }
 0x20b297d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ecx)); cpu.eax = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b298f; continue; }
  pc = 0x20b2984; continue;
 }
 0x20b2984 => {
  { let v: u32 = cpu.edi ^ cpu.edi; cpu.edi = v; }
  { cpu.sub32(cpu.edx, cpu.eax); }
  if !cpu.zf { pc = 0x20b298f; continue; }
  pc = 0x20b298a; continue;
 }
 0x20b298a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x20b298f; continue;
 }
 0x20b298f => {
  { cpu.and32(cpu.esi, cpu.esi); }
  if cpu.zf { pc = 0x20b299e; continue; }
  pc = 0x20b2993; continue;
 }
 0x20b2993 => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  cpu.push(0x20b299c); f_020b29b9(cpu); if cpu.returned_to != 0x20b299c { return; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x20b299e; continue;
 }
 0x20b299e => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x20b29aa; continue; }
  pc = 0x20b29a6; continue;
 }
 0x20b29a6 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edi = v; }
  pc = 0x20b29aa; continue;
 }
 0x20b29aa => {
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
pub fn f_020b29b9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b29b9); }
 let mut pc: u32 = 0x20b29b9;
 loop { match pc {
 0x20b29b9 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.esi = v; }
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf != cpu.of) { pc = 0x20b29dd; continue; }
  pc = 0x20b29c9; continue;
 }
 0x20b29c9 => {
  if !cpu.zf { pc = 0x20b29e2; continue; }
  pc = 0x20b29cb; continue;
 }
 0x20b29cb => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { cpu.sub32(cpu.ecx, cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x20b29dd; continue; }
  pc = 0x20b29d5; continue;
 }
 0x20b29d5 => {
  if !cpu.zf { pc = 0x20b29e2; continue; }
  pc = 0x20b29d7; continue;
 }
 0x20b29d7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x0u32.wrapping_add(cpu.edx))); }
  if (cpu.sf == cpu.of) { pc = 0x20b29e2; continue; }
  pc = 0x20b29dd; continue;
 }
 0x20b29dd => {
  { let v: u32 = 0x1u32; cpu.ebx = v; }
  pc = 0x20b29e2; continue;
 }
 0x20b29e2 => {
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
pub fn f_020b29ec(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b29ec); }
 let mut pc: u32 = 0x20b29ec;
 loop { match pc {
 0x20b29ec => {
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
pub fn f_020b2a14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2a14); }
 let mut pc: u32 = 0x20b2a14;
 loop { match pc {
 0x20b2a14 => {
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
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x20b2a57; continue; }
  pc = 0x20b2a28; continue;
 }
 0x20b2a28 => {
  { cpu.and8((cpu.eax as u8), (cpu.eax as u8)); }
  if !cpu.zf { pc = 0x20b2a50; continue; }
  pc = 0x20b2a2c; continue;
 }
 0x20b2a2c => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x20b2a40; continue; }
  pc = 0x20b2a36; continue;
 }
 0x20b2a36 => {
  { let v: u32 = cpu.xor32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b2a8f; continue;
 }
 0x20b2a40 => {
  if (cpu.cf || cpu.zf) { pc = 0x20b2a49; continue; }
  pc = 0x20b2a42; continue;
 }
 0x20b2a42 => {
  { let v: u32 = 0x8047u32; cpu.eax = v; }
  pc = 0x20b2a81; continue;
 }
 0x20b2a49 => {
  { let v: u32 = 0x4047u32; cpu.eax = v; }
  pc = 0x20b2a81; continue;
 }
 0x20b2a50 => {
  { let v: u32 = 0x2047u32; cpu.eax = v; }
  pc = 0x20b2a81; continue;
 }
 0x20b2a57 => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x10u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if cpu.cf { pc = 0x20b2a6b; continue; }
  pc = 0x20b2a61; continue;
 }
 0x20b2a61 => {
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffe8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xffffffecu32.wrapping_add(cpu.ebp), v); }
  pc = 0x20b2a8f; continue;
 }
 0x20b2a6b => {
  { cpu.fpush(0.0); }
  { let a = cpu.st(0); let b = cpu.rf64(0x8u32.wrapping_add(cpu.ebp)); cpu.fcom(a, b); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if !cpu.cf { pc = 0x20b2a7c; continue; }
  pc = 0x20b2a75; continue;
 }
 0x20b2a75 => {
  { let v: u32 = 0x8107u32; cpu.eax = v; }
  pc = 0x20b2a81; continue;
 }
 0x20b2a7c => {
  { let v: u32 = 0x1107u32; cpu.eax = v; }
  pc = 0x20b2a81; continue;
 }
 0x20b2a81 => {
  { let v: u32 = 0x10u32.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = 0x8u32.wrapping_add(cpu.ebp); cpu.edx = v; }
  cpu.push(0x20b2a8c); f_020c23ed(cpu); if cpu.returned_to != 0x20b2a8c { return; }
  { let v = cpu.st(0); cpu.wf64(0xffffffe8u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  pc = 0x20b2a8f; continue;
 }
 0x20b2a8f => {
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
pub fn f_020b2aa8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2aa8); }
 let mut pc: u32 = 0x20b2aa8;
 loop { match pc {
 0x20b2aa8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.esp, 0x10u32); cpu.esp = v; }
  { let a = cpu.st(0); let b = cpu.rf64(0x217ba14u32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (cpu.cf || cpu.zf) { pc = 0x20b2ad7; continue; }
  pc = 0x20b2abf; continue;
 }
 0x20b2abf => {
  { cpu.sub8((cpu.eax as u8), 0x7u8); }
  if cpu.zf { pc = 0x20b2ad1; continue; }
  pc = 0x20b2ac3; continue;
 }
 0x20b2ac3 => {
  { let v = cpu.st(0); cpu.wf64(0xfffffff0u32.wrapping_add(cpu.ebp), v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.ebp), v); }
  cpu.push(0x20b2ad1); f_020c24c8(cpu); if cpu.returned_to != 0x20b2ad1 { return; }
  pc = 0x20b2ad1; continue;
 }
 0x20b2ad1 => {
  { let v: u8 = 0x1u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2ad7 => {
  { let a = cpu.st(0); let b = cpu.rf64(0x217ba1cu32); cpu.fcom(a, b);  }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0xfffffff0u32.wrapping_add(cpu.ebp), v); }
  {  }
  { let v: u8 = cpu.r8(0xfffffff1u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x20b2aec; continue; }
  pc = 0x20b2ae8; continue;
 }
 0x20b2ae8 => {
  { cpu.fpush(0.0); }
  pc = 0x20b2b00; continue;
 }
 0x20b2aec => {
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
  pc = 0x20b2b00; continue;
 }
 0x20b2b00 => {
  { let v = cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u8 = 0x0u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp; cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b2b14(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x20b2b14); }
 let mut pc: u32 = 0x20b2b14;
 loop { match pc {
 0x20b2b14 => {
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v = cpu.rf80(0x4u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  pc = 0x20b2b1c; continue;
 }
 0x20b2b1c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x20b2bae; continue; }
  pc = 0x20b2b28; continue;
 }
 0x20b2b28 => {
  { let v: u32 = cpu.eax ^ 0xe000000u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, 0xe000000u32); }
  if cpu.zf { pc = 0x20b2b37; continue; }
  pc = 0x20b2b34; continue;
 }
 0x20b2b34 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2b37 => {
  { let v: u32 = cpu.shr32(cpu.eax, 28u32); cpu.eax = v; }
  { cpu.sub8(cpu.r8(0x217cf1cu32.wrapping_add(cpu.eax)), 0x0u8); }
  if !cpu.zf { pc = 0x20b2b46; continue; }
  pc = 0x20b2b43; continue;
 }
 0x20b2b43 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2b46 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b2bb8; continue; }
  pc = 0x20b2b51; continue;
 }
 0x20b2b51 => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x20b2bb8; continue; }
  pc = 0x20b2b58; continue;
 }
 0x20b2b58 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x20b2b97; continue; }
  pc = 0x20b2b80; continue;
 }
 0x20b2b80 => {
  { let v = cpu.st(0) * cpu.rf32(0x217cf2cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x217cf2cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2b97 => {
  { let v = cpu.st(0) * cpu.rf32(0x217cf30u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0) * cpu.rf32(0x217cf30u32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2bae => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esp))); cpu.eax = v; }
  if !cpu.zf { pc = 0x20b2bbb; continue; }
  pc = 0x20b2bb8; continue;
 }
 0x20b2bb8 => {
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2bbb => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x20b2bb8; continue; }
  pc = 0x20b2bc6; continue;
 }
 0x20b2bc6 => {
  { let v = cpu.fpu_control; cpu.w16(0x1cu32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.eax | 0x33fu32; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xf3ffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.esp), v); }
  { cpu.fpu_control = cpu.r16(0x20u32.wrapping_add(cpu.esp)); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7fffu32); cpu.eax = v; }
  if cpu.zf { pc = 0x20b2bfc; continue; }
  pc = 0x20b2beb; continue;
 }
 0x20b2beb => {
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if cpu.zf { pc = 0x20b2c24; continue; }
  pc = 0x20b2bf2; continue;
 }
 0x20b2bf2 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.cf { pc = 0x20b2c24; continue; }
  pc = 0x20b2bfa; continue;
 }
 0x20b2bfa => {
  pc = 0x20b2c04; continue;
 }
 0x20b2bfc => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.cf { pc = 0x20b2c24; continue; }
  pc = 0x20b2c04; continue;
 }
 0x20b2c04 => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x217cf34u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf80(0x4u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.rf80(0x10u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  {  }
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  cpu.poll(); pc = 0x20b2b1c; continue;
 }
 0x20b2c24 => {
  { cpu.fpu_control = cpu.r16(0x1cu32.wrapping_add(cpu.esp)); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_020b2c2b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c2b); } r_020b2c2b(cpu, 0x20b2c2b); }
pub fn f_020b2c35(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c35); } r_020b2c2b(cpu, 0x20b2c35); }
pub fn f_020b2c3b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c3b); } r_020b2c2b(cpu, 0x20b2c3b); }
pub fn f_020b2c40(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c40); } r_020b2c2b(cpu, 0x20b2c40); }
pub fn f_020b2c46(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c46); } r_020b2c2b(cpu, 0x20b2c46); }
pub fn f_020b2c4b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c4b); } r_020b2c2b(cpu, 0x20b2c4b); }
pub fn f_020b2c51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c51); } r_020b2c2b(cpu, 0x20b2c51); }
pub fn f_020b2c57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c57); } r_020b2c2b(cpu, 0x20b2c57); }
pub fn f_020b2c5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c5d); } r_020b2c2b(cpu, 0x20b2c5d); }
pub fn f_020b2c63(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c63); } r_020b2c2b(cpu, 0x20b2c63); }
pub fn f_020b2c7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c7f); } r_020b2c2b(cpu, 0x20b2c7f); }
pub fn f_020b2c84(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c84); } r_020b2c2b(cpu, 0x20b2c84); }
pub fn f_020b2c9a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c9a); } r_020b2c2b(cpu, 0x20b2c9a); }
pub fn f_020b2c9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2c9f); } r_020b2c2b(cpu, 0x20b2c9f); }
pub fn f_020b2cbb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2cbb); } r_020b2c2b(cpu, 0x20b2cbb); }
pub fn f_020b2ccb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2ccb); } r_020b2c2b(cpu, 0x20b2ccb); }
pub fn f_020b2cdf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2cdf); } r_020b2c2b(cpu, 0x20b2cdf); }
pub fn f_020b2cef(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2cef); } r_020b2c2b(cpu, 0x20b2cef); }
pub fn f_020b2d0f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d0f); } r_020b2c2b(cpu, 0x20b2d0f); }
pub fn f_020b2d14(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d14); } r_020b2c2b(cpu, 0x20b2d14); }
pub fn f_020b2d2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d2e); } r_020b2c2b(cpu, 0x20b2d2e); }
pub fn f_020b2d33(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d33); } r_020b2c2b(cpu, 0x20b2d33); }
pub fn f_020b2d53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d53); } r_020b2c2b(cpu, 0x20b2d53); }
pub fn f_020b2d67(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d67); } r_020b2c2b(cpu, 0x20b2d67); }
pub fn f_020b2d7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d7f); } r_020b2c2b(cpu, 0x20b2d7f); }
pub fn f_020b2d93(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2d93); } r_020b2c2b(cpu, 0x20b2d93); }
pub fn f_020b2db3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2db3); } r_020b2c2b(cpu, 0x20b2db3); }
pub fn f_020b2db8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2db8); } r_020b2c2b(cpu, 0x20b2db8); }
pub fn f_020b2dd2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2dd2); } r_020b2c2b(cpu, 0x20b2dd2); }
pub fn f_020b2dd7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2dd7); } r_020b2c2b(cpu, 0x20b2dd7); }
pub fn f_020b2df7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2df7); } r_020b2c2b(cpu, 0x20b2df7); }
pub fn f_020b2e0b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e0b); } r_020b2c2b(cpu, 0x20b2e0b); }
pub fn f_020b2e23(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e23); } r_020b2c2b(cpu, 0x20b2e23); }
pub fn f_020b2e37(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e37); } r_020b2c2b(cpu, 0x20b2e37); }
pub fn f_020b2e57(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e57); } r_020b2c2b(cpu, 0x20b2e57); }
pub fn f_020b2e5c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e5c); } r_020b2c2b(cpu, 0x20b2e5c); }
pub fn f_020b2e76(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e76); } r_020b2c2b(cpu, 0x20b2e76); }
pub fn f_020b2e7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e7b); } r_020b2c2b(cpu, 0x20b2e7b); }
pub fn f_020b2e9b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2e9b); } r_020b2c2b(cpu, 0x20b2e9b); }
pub fn f_020b2eaf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2eaf); } r_020b2c2b(cpu, 0x20b2eaf); }
pub fn f_020b2ec7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2ec7); } r_020b2c2b(cpu, 0x20b2ec7); }
pub fn f_020b2edb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2edb); } r_020b2c2b(cpu, 0x20b2edb); }
pub fn f_020b2efb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2efb); } r_020b2c2b(cpu, 0x20b2efb); }
pub fn f_020b2f00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f00); } r_020b2c2b(cpu, 0x20b2f00); }
pub fn f_020b2f1a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f1a); } r_020b2c2b(cpu, 0x20b2f1a); }
pub fn f_020b2f1f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f1f); } r_020b2c2b(cpu, 0x20b2f1f); }
pub fn f_020b2f3f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f3f); } r_020b2c2b(cpu, 0x20b2f3f); }
pub fn f_020b2f53(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f53); } r_020b2c2b(cpu, 0x20b2f53); }
pub fn f_020b2f6b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f6b); } r_020b2c2b(cpu, 0x20b2f6b); }
pub fn f_020b2f7f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f7f); } r_020b2c2b(cpu, 0x20b2f7f); }
pub fn f_020b2f9f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2f9f); } r_020b2c2b(cpu, 0x20b2f9f); }
pub fn f_020b2fa4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2fa4); } r_020b2c2b(cpu, 0x20b2fa4); }
pub fn f_020b2fbe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2fbe); } r_020b2c2b(cpu, 0x20b2fbe); }
pub fn f_020b2fc3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2fc3); } r_020b2c2b(cpu, 0x20b2fc3); }
pub fn f_020b2fe3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2fe3); } r_020b2c2b(cpu, 0x20b2fe3); }
pub fn f_020b2ff7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b2ff7); } r_020b2c2b(cpu, 0x20b2ff7); }
pub fn f_020b300f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b300f); } r_020b2c2b(cpu, 0x20b300f); }
pub fn f_020b3023(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3023); } r_020b2c2b(cpu, 0x20b3023); }
pub fn f_020b3043(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3043); } r_020b2c2b(cpu, 0x20b3043); }
pub fn f_020b3048(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3048); } r_020b2c2b(cpu, 0x20b3048); }
pub fn f_020b3062(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3062); } r_020b2c2b(cpu, 0x20b3062); }
pub fn f_020b3067(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3067); } r_020b2c2b(cpu, 0x20b3067); }
pub fn f_020b3087(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b3087); } r_020b2c2b(cpu, 0x20b3087); }
pub fn f_020b309b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b309b); } r_020b2c2b(cpu, 0x20b309b); }
pub fn f_020b30b3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x20b30b3); } r_020b2c2b(cpu, 0x20b30b3); }
fn r_020b2c2b(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x20b2c2b => {
  { let v: u32 = cpu.sub32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.poll(); pc = cpu.r32(0x217cf38u32.wrapping_add(cpu.eax.wrapping_mul(4))); continue;
 }
 0x20b2c35 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c3b => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2c40; continue;
 }
 0x20b2c40 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c46 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2c4b; continue;
 }
 0x20b2c4b => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c51 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c57 => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c5d => {
  { let v = cpu.st(0) / cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c63 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2c75); f_020b2b14(cpu); if cpu.returned_to != 0x20b2c75 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c7f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2c84; continue;
 }
 0x20b2c84 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2c90); f_020b2b14(cpu); if cpu.returned_to != 0x20b2c90 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2c9a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2c9f; continue;
 }
 0x20b2c9f => {
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2cb3); f_020b2b14(cpu); if cpu.returned_to != 0x20b2cb3 { return; }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2cbb => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2cc7); f_020b2b14(cpu); if cpu.returned_to != 0x20b2cc7 { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2ccb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2cd7); f_020b2b14(cpu); if cpu.returned_to != 0x20b2cd7 { return; }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2cdf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2ceb); f_020b2b14(cpu); if cpu.returned_to != 0x20b2ceb { return; }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2cef => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2d03); f_020b2b14(cpu); if cpu.returned_to != 0x20b2d03 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2d0f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2d14; continue;
 }
 0x20b2d14 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2d22); f_020b2b14(cpu); if cpu.returned_to != 0x20b2d22 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2d2e => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2d33; continue;
 }
 0x20b2d33 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2d49); f_020b2b14(cpu); if cpu.returned_to != 0x20b2d49 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2d53 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2d61); f_020b2b14(cpu); if cpu.returned_to != 0x20b2d61 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2d67 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2d75); f_020b2b14(cpu); if cpu.returned_to != 0x20b2d75 { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2d7f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2d8d); f_020b2b14(cpu); if cpu.returned_to != 0x20b2d8d { return; }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.set_st(0, b); cpu.set_st(1, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2d93 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2da7); f_020b2b14(cpu); if cpu.returned_to != 0x20b2da7 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2db3 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2db8; continue;
 }
 0x20b2db8 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2dc6); f_020b2b14(cpu); if cpu.returned_to != 0x20b2dc6 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2dd2 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2dd7; continue;
 }
 0x20b2dd7 => {
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2ded); f_020b2b14(cpu); if cpu.returned_to != 0x20b2ded { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2df7 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2e05); f_020b2b14(cpu); if cpu.returned_to != 0x20b2e05 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2e0b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2e19); f_020b2b14(cpu); if cpu.returned_to != 0x20b2e19 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2e23 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2e31); f_020b2b14(cpu); if cpu.returned_to != 0x20b2e31 { return; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2e37 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2e4b); f_020b2b14(cpu); if cpu.returned_to != 0x20b2e4b { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2e57 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2e5c; continue;
 }
 0x20b2e5c => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2e6a); f_020b2b14(cpu); if cpu.returned_to != 0x20b2e6a { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2e76 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2e7b; continue;
 }
 0x20b2e7b => {
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2e91); f_020b2b14(cpu); if cpu.returned_to != 0x20b2e91 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2e9b => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2ea9); f_020b2b14(cpu); if cpu.returned_to != 0x20b2ea9 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2eaf => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2ebd); f_020b2b14(cpu); if cpu.returned_to != 0x20b2ebd { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2ec7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2ed5); f_020b2b14(cpu); if cpu.returned_to != 0x20b2ed5 { return; }
  { let a = cpu.st(0); let b = cpu.st(3); cpu.set_st(0, b); cpu.set_st(3, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2edb => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2eef); f_020b2b14(cpu); if cpu.returned_to != 0x20b2eef { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2efb => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2f00; continue;
 }
 0x20b2f00 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2f0e); f_020b2b14(cpu); if cpu.returned_to != 0x20b2f0e { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2f1a => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2f1f; continue;
 }
 0x20b2f1f => {
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2f35); f_020b2b14(cpu); if cpu.returned_to != 0x20b2f35 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2f3f => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2f4d); f_020b2b14(cpu); if cpu.returned_to != 0x20b2f4d { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2f53 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2f61); f_020b2b14(cpu); if cpu.returned_to != 0x20b2f61 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2f6b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2f79); f_020b2b14(cpu); if cpu.returned_to != 0x20b2f79 { return; }
  { let a = cpu.st(0); let b = cpu.st(4); cpu.set_st(0, b); cpu.set_st(4, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2f7f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2f93); f_020b2b14(cpu); if cpu.returned_to != 0x20b2f93 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2f9f => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2fa4; continue;
 }
 0x20b2fa4 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2fb2); f_020b2b14(cpu); if cpu.returned_to != 0x20b2fb2 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2fbe => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b2fc3; continue;
 }
 0x20b2fc3 => {
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2fd9); f_020b2b14(cpu); if cpu.returned_to != 0x20b2fd9 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2fe3 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b2ff1); f_020b2b14(cpu); if cpu.returned_to != 0x20b2ff1 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b2ff7 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b3005); f_020b2b14(cpu); if cpu.returned_to != 0x20b3005 { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b300f => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b301d); f_020b2b14(cpu); if cpu.returned_to != 0x20b301d { return; }
  { let a = cpu.st(0); let b = cpu.st(5); cpu.set_st(0, b); cpu.set_st(5, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b3023 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b3037); f_020b2b14(cpu); if cpu.returned_to != 0x20b3037 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b3043 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b3048; continue;
 }
 0x20b3048 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b3056); f_020b2b14(cpu); if cpu.returned_to != 0x20b3056 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b3062 => {
  { let v: u32 = cpu.esp.wrapping_add(0x2cu32); cpu.esp = v; }
  { cpu.interrupt(0x6); }
  pc = 0x20b3067; continue;
 }
 0x20b3067 => {
  { let a = cpu.st(0); let b = cpu.st(7); cpu.set_st(0, b); cpu.set_st(7, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf80(0x20u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b307d); f_020b2b14(cpu); if cpu.returned_to != 0x20b307d { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0x20u32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b3087 => {
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b3095); f_020b2b14(cpu); if cpu.returned_to != 0x20b3095 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b309b => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b30a9); f_020b2b14(cpu); if cpu.returned_to != 0x20b30a9 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.rf80(0xcu32.wrapping_add(cpu.esp)); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x20b30b3 => {
  { let v = cpu.st(0); cpu.wf80(0xcu32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v = cpu.st(0); cpu.wf80(0x0u32.wrapping_add(cpu.esp), v); cpu.fpop(); }
  cpu.push(0x20b30c1); f_020b2b14(cpu); if cpu.returned_to != 0x20b30c1 { return; }
  { let a = cpu.st(0); let b = cpu.st(6); cpu.set_st(0, b); cpu.set_st(6, a); }
  { let v: u32 = cpu.add32(cpu.esp, 0x2cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
