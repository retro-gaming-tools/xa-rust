#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_050aeee9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aeee9); }
 let mut pc: u32 = 0x50aeee9;
 loop { match pc {
 0x50aeee9 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { f_050a5f0a(cpu); return; }
  pc = 0x50aeef1; continue;
 }
 0x50aeef1 => {
  cpu.push(0x50aeef6); f_050969f1(cpu); if cpu.returned_to != 0x50aeef6 { return; }
  { let v: u32 = cpu.xor32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aeeff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aeeff); }
 let mut pc: u32 = 0x50aeeff;
 loop { match pc {
 0x50aeeff => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = 0xffu32; cpu.edx = v; }
  { let v: u32 = 0x513b250u32; cpu.eax = v; }
  cpu.push(0x50aef0f); f_050a65b2(cpu); if cpu.returned_to != 0x50aef0f { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aef11(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aef11); }
 let mut pc: u32 = 0x50aef11;
 loop { match pc {
 0x50aef11 => {
  pc = 0x50aef16; continue;
 }
 0x50aef16 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x514beacu32), 0x50aefa5u32); }
  if cpu.zf { pc = 0x50aef3f; continue; }
  pc = 0x50aef24; continue;
 }
 0x50aef24 => {
  { let v: u32 = 0x50aec75u32; cpu.ebx = v; }
  { let v: u32 = 0x50aefa5u32; cpu.ecx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x514ad21u32, v); }
  cpu.push(0x50aef39); f_050aef48(cpu); if cpu.returned_to != 0x50aef39 { return; }
  { let v: u32 = cpu.ecx; cpu.w32(0x514beacu32, v); }
  pc = 0x50aef3f; continue;
 }
 0x50aef3f => {
  cpu.push(0x50aef44); f_050af002(cpu); if cpu.returned_to != 0x50aef44 { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aef48(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aef48); }
 let mut pc: u32 = 0x50aef48;
 loop { match pc {
 0x50aef48 => {
  { cpu.sub16(cpu.r16(0x514bf20u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x50aef5d; continue; }
  pc = 0x50aef52; continue;
 }
 0x50aef52 => {
  { cpu.sub8(cpu.r8(0x51439b8u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if cpu.zf { pc = 0x50aef5d; continue; }
  pc = 0x50aef5b; continue;
 }
 0x50aef5b => {
  pc = 0x50aef5e; continue;
 }
 0x50aef5d => {
  cpu.flat_ds(0x50aef5d); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aef5e => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x50aef74; continue; }
  pc = 0x50aef6d; continue;
 }
 0x50aef6d => {
  cpu.flat_ds(0x50aef6d); cpu.push(0x50aef72); f_050af398(cpu); if cpu.returned_to != 0x50aef72 { return; }
  pc = 0x50aef99; continue;
 }
 0x50aef74 => {
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x50aef84; continue; }
  pc = 0x50aef7d; continue;
 }
 0x50aef7d => {
  cpu.flat_ds(0x50aef7d); cpu.push(0x50aef82); f_050af352(cpu); if cpu.returned_to != 0x50aef82 { return; }
  pc = 0x50aef99; continue;
 }
 0x50aef84 => {
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x50aef94; continue; }
  pc = 0x50aef8d; continue;
 }
 0x50aef8d => {
  cpu.flat_ds(0x50aef8d); cpu.push(0x50aef92); f_050af3f0(cpu); if cpu.returned_to != 0x50aef92 { return; }
  pc = 0x50aef99; continue;
 }
 0x50aef94 => {
  cpu.flat_ds(0x50aef94); cpu.push(0x50aef99); f_050af262(cpu); if cpu.returned_to != 0x50aef99 { return; }
  pc = 0x50aef99; continue;
 }
 0x50aef99 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50aef9a); cpu.push(0x50aef9f); f_050af002(cpu); if cpu.returned_to != 0x50aef9f { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x50aefa4); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050aefa5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50aefa5); }
 let mut pc: u32 = 0x50aefa5;
 loop { match pc {
 0x50aefa5 => {
  { cpu.sub16(cpu.r16(0x514bf20u32.wrapping_add(cpu.bases[3])), 0x0u16); }
  if !cpu.zf { pc = 0x50aefb0; continue; }
  pc = 0x50aefaf; continue;
 }
 0x50aefaf => {
  cpu.flat_ds(0x50aefaf); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50aefb0 => {
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
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x0u8); }
  if !cpu.zf { pc = 0x50aefd7; continue; }
  pc = 0x50aefd0; continue;
 }
 0x50aefd0 => {
  cpu.flat_ds(0x50aefd0); cpu.push(0x50aefd5); f_050af3d6(cpu); if cpu.returned_to != 0x50aefd5 { return; }
  pc = 0x50aeffc; continue;
 }
 0x50aefd7 => {
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x1u8); }
  if !cpu.zf { pc = 0x50aefe7; continue; }
  pc = 0x50aefe0; continue;
 }
 0x50aefe0 => {
  cpu.flat_ds(0x50aefe0); cpu.push(0x50aefe5); f_050af37f(cpu); if cpu.returned_to != 0x50aefe5 { return; }
  pc = 0x50aeffc; continue;
 }
 0x50aefe7 => {
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x9u8); }
  if !cpu.zf { pc = 0x50aeff7; continue; }
  pc = 0x50aeff0; continue;
 }
 0x50aeff0 => {
  cpu.flat_ds(0x50aeff0); cpu.push(0x50aeff5); f_050af448(cpu); if cpu.returned_to != 0x50aeff5 { return; }
  pc = 0x50aeffc; continue;
 }
 0x50aeff7 => {
  cpu.flat_ds(0x50aeff7); cpu.push(0x50aeffc); f_050af325(cpu); if cpu.returned_to != 0x50aeffc { return; }
  pc = 0x50aeffc; continue;
 }
 0x50aeffc => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.flat_ds(0x50af000); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af002(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af002); }
 let mut pc: u32 = 0x50af002;
 loop { match pc {
 0x50af002 => {
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
pub fn f_050af017(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50af017); } r_050af017(cpu, 0x50af017); }
pub fn f_050af024(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50af024); } r_050af017(cpu, 0x50af024); }
fn r_050af017(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50af017 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u8 = 0x20u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.port_out(0x20u16, 1, (cpu.eax as u8) as u32); }
  { cpu.port_out(0xa0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u16 = (cpu.eax as u16) ^ (cpu.eax as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.port_out(0xf0u16, 1, (cpu.eax as u8) as u32); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x50af024; continue;
 }
 0x50af024 => {
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
  cpu.unimplemented(0x50af032, "Fnstenv");
  {  }
  {  }
  {  }
  {  }
  cpu.unimplemented(0x50af03a, "Fndisi");
  {  }
  cpu.flat_ds(0x50af03d); cpu.push(0x50af042); f_05096e08(cpu); if cpu.returned_to != 0x50af042 { return; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = !cpu.edx; cpu.edx = v; }
  { let v: u8 = 0xffu8; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edx & cpu.r32(0x4u32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  pc = 0x50af052; continue;
 }
 0x50af052 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0])); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { cpu.sub8((cpu.ebx as u8), 0xd8u8); }
  if cpu.cf { cpu.poll(); pc = 0x50af052; continue; }
  pc = 0x50af05c; continue;
 }
 0x50af05c => {
  { cpu.sub8((cpu.ebx as u8), 0xdfu8); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x50af052; continue; }
  pc = 0x50af061; continue;
 }
 0x50af061 => {
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebp)); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let a = (cpu.ebx as u8); let b = ((cpu.ebx >> 8) as u8); { let v: u8 = b; cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; } { let v: u8 = a; cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); } }
  { let v: u8 = 0x0u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.and8((cpu.edx as u8), 0x20u8); }
  if cpu.zf { pc = 0x50af074; continue; }
  pc = 0x50af070; continue;
 }
 0x50af070 => {
  { let v: u8 = 0x86u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af0c0; continue;
 }
 0x50af074 => {
  { cpu.and8((cpu.edx as u8), 0x40u8); }
  if cpu.zf { pc = 0x50af088; continue; }
  pc = 0x50af079; continue;
 }
 0x50af079 => {
  { cpu.and16((cpu.edx as u16), 0x200u16); }
  if !cpu.zf { pc = 0x50af084; continue; }
  pc = 0x50af080; continue;
 }
 0x50af080 => {
  { let v: u8 = 0x8bu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af086; continue;
 }
 0x50af084 => {
  { let v: u8 = 0x8au8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af086; continue;
 }
 0x50af086 => {
  pc = 0x50af0c0; continue;
 }
 0x50af088 => {
  { cpu.and8((cpu.edx as u8), 0x1u8); }
  if cpu.zf { pc = 0x50af094; continue; }
  pc = 0x50af08d; continue;
 }
 0x50af08d => {
  cpu.flat_ds(0x50af08d); cpu.push(0x50af092); f_050af12a(cpu); if cpu.returned_to != 0x50af092 { return; }
  pc = 0x50af0c0; continue;
 }
 0x50af094 => {
  { cpu.and8((cpu.edx as u8), 0x2u8); }
  if cpu.zf { pc = 0x50af09d; continue; }
  pc = 0x50af099; continue;
 }
 0x50af099 => {
  { let v: u8 = 0x82u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af0c0; continue;
 }
 0x50af09d => {
  { cpu.and8((cpu.edx as u8), 0x8u8); }
  if cpu.zf { pc = 0x50af0ab; continue; }
  pc = 0x50af0a2; continue;
 }
 0x50af0a2 => {
  cpu.flat_ds(0x50af0a2); cpu.push(0x50af0a7); f_050af1b7(cpu); if cpu.returned_to != 0x50af0a7 { return; }
  { let v: u8 = 0x84u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af0c0; continue;
 }
 0x50af0ab => {
  { cpu.and8((cpu.edx as u8), 0x10u8); }
  if cpu.zf { pc = 0x50af0b4; continue; }
  pc = 0x50af0b0; continue;
 }
 0x50af0b0 => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af0c0; continue;
 }
 0x50af0b4 => {
  { cpu.and8((cpu.edx as u8), 0x4u8); }
  if cpu.zf { pc = 0x50af0c0; continue; }
  pc = 0x50af0b9; continue;
 }
 0x50af0b9 => {
  cpu.flat_ds(0x50af0b9); cpu.push(0x50af0be); f_050af209(cpu); if cpu.returned_to != 0x50af0be { return; }
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af0c0; continue;
 }
 0x50af0c0 => {
  { cpu.sub8((cpu.ecx as u8), 0x0u8); }
  if cpu.zf { pc = 0x50af110; continue; }
  pc = 0x50af0c5; continue;
 }
 0x50af0c5 => {
  { let v = cpu.r32(0x514acf8u32.wrapping_add(cpu.bases[3])); cpu.push(v); }
  { let v: u16 = (cpu.segments[2] as u16); cpu.w16(0x514c142u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.esp; cpu.w32(0x514c144u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.pop(); cpu.set_segment(2, v as u16); }
  { let v: u32 = 0x514c142u32; cpu.esp = v; }
  { let v: u32 = 0x514bf42u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514acf8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = (cpu.ecx as u8) as u32; cpu.eax = v; }
  let t = cpu.r32(0x514ad21u32.wrapping_add(cpu.bases[3])); cpu.flat_ds(0x50af0ee); cpu.push(0x50af0f4); cpu.call(t); if cpu.returned_to != 0x50af0f4 { return; }
  { let v = cpu.r16(0x514c142u32.wrapping_add(cpu.bases[3])); cpu.set_segment(2, v as u16); }
  { let v: u32 = cpu.r32(0x514c144u32.wrapping_add(cpu.bases[3])); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.w32(0x514acf8u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x0u16; cpu.w16(0x514c142u32.wrapping_add(cpu.bases[3]), v); }
  pc = 0x50af110; continue;
 }
 0x50af110 => {
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
  cpu.flat_ds(0x50af129); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af12a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af12a); }
 let mut pc: u32 = 0x50af12a;
 loop { match pc {
 0x50af12a => {
  { let v: u8 = 0x81u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { cpu.sub16((cpu.ebx as u16), 0xd9fau16); }
  if !cpu.zf { pc = 0x50af136; continue; }
  pc = 0x50af133; continue;
 }
 0x50af133 => {
  { let v: u8 = 0x88u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50af136 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f1u16); }
  if !cpu.zf { pc = 0x50af140; continue; }
  pc = 0x50af13d; continue;
 }
 0x50af13d => {
  { let v: u8 = 0x8eu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50af140 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f8u16); }
  if cpu.zf { pc = 0x50af14c; continue; }
  pc = 0x50af147; continue;
 }
 0x50af147 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9f5u16); }
  pc = 0x50af14c; continue;
 }
 0x50af14c => {
  if !cpu.zf { pc = 0x50af152; continue; }
  pc = 0x50af14e; continue;
 }
 0x50af14e => {
  { let v: u8 = 0x8fu8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af1b6; continue;
 }
 0x50af152 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x310u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x310u16); }
  if !cpu.zf { pc = 0x50af165; continue; }
  pc = 0x50af161; continue;
 }
 0x50af161 => {
  { let v: u8 = 0x8du8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af1b6; continue;
 }
 0x50af165 => {
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x50af18f; continue; }
  pc = 0x50af171; continue;
 }
 0x50af171 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.fpush(0.0); }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x50af184; continue; }
  pc = 0x50af17f; continue;
 }
 0x50af17f => {
  cpu.push(0x50af184); f_050af254(cpu); if cpu.returned_to != 0x50af184 { return; }
  pc = 0x50af184; continue;
 }
 0x50af184 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x50af18b; continue; }
  pc = 0x50af189; continue;
 }
 0x50af189 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x50af18b; continue;
 }
 0x50af18b => {
  { let v: u8 = 0x85u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af1b6; continue;
 }
 0x50af18f => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x130u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x30u16); }
  if !cpu.zf { pc = 0x50af1b6; continue; }
  pc = 0x50af19d; continue;
 }
 0x50af19d => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8) & 0x38u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.shr8((cpu.ecx as u8), 2u32); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = cpu.ror16((cpu.edx as u16), (cpu.ecx & 0xff)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0x5u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0x5u8); }
  if !cpu.zf { pc = 0x50af1b6; continue; }
  pc = 0x50af1b4; continue;
 }
 0x50af1b4 => {
  { let v: u8 = 0x83u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  pc = 0x50af1b6; continue;
 }
 0x50af1b6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af1b7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af1b7); }
 let mut pc: u32 = 0x50af1b7;
 loop { match pc {
 0x50af1b7 => {
  { cpu.sub16((cpu.ebx as u16), 0xd9fdu16); }
  if !cpu.zf { pc = 0x50af1c0; continue; }
  pc = 0x50af1be; continue;
 }
 0x50af1be => {
  pc = 0x50af208; continue;
 }
 0x50af1c0 => {
  { let v: u16 = (cpu.ebx as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.edx as u16) & 0x110u16; cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), 0x110u16); }
  if !cpu.zf { pc = 0x50af1ec; continue; }
  pc = 0x50af1cf; continue;
 }
 0x50af1cf => {
  cpu.push(0x50af1d4); f_050af209(cpu); if cpu.returned_to != 0x50af1d4 { return; }
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if cpu.zf { pc = 0x50af1e3; continue; }
  pc = 0x50af1de; continue;
 }
 0x50af1de => {
  cpu.push(0x50af1e3); f_050af254(cpu); if cpu.returned_to != 0x50af1e3 { return; }
  pc = 0x50af1e3; continue;
 }
 0x50af1e3 => {
  { cpu.and8((cpu.ebx as u8), 0x8u8); }
  if cpu.zf { pc = 0x50af1ea; continue; }
  pc = 0x50af1e8; continue;
 }
 0x50af1e8 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x50af1ea; continue;
 }
 0x50af1ea => {
  pc = 0x50af208; continue;
 }
 0x50af1ec => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.edx as u8) & 0xc0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { cpu.sub8((cpu.edx as u8), 0xc0u8); }
  if !cpu.zf { pc = 0x50af203; continue; }
  pc = 0x50af1f6; continue;
 }
 0x50af1f6 => {
  { let v: u32 = 0x514bf2cu32; cpu.esi = v; }
  cpu.push(0x50af201); f_050af22f(cpu); if cpu.returned_to != 0x50af201 { return; }
  pc = 0x50af208; continue;
 }
 0x50af203 => {
  cpu.push(0x50af208); f_050af209(cpu); if cpu.returned_to != 0x50af208 { return; }
  pc = 0x50af208; continue;
 }
 0x50af208 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af209(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af209); }
 let mut pc: u32 = 0x50af209;
 loop { match pc {
 0x50af209 => {
  { let a = cpu.st(0); cpu.fcom(a, 0.0); }
  {  }
  { let v = cpu.fstatus(); cpu.w16(0x14u32.wrapping_add(cpu.ebp), v); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if cpu.zf { pc = 0x50af21e; continue; }
  pc = 0x50af216; continue;
 }
 0x50af216 => {
  { let v = cpu.rf64(0x514bf36u32); cpu.fpush(v); }
  pc = 0x50af224; continue;
 }
 0x50af21e => {
  { let v = cpu.rf32(0x514bf3eu32); cpu.fpush(v); }
  pc = 0x50af224; continue;
 }
 0x50af224 => {
  { cpu.and16(cpu.r16(0x14u32.wrapping_add(cpu.ebp)), 0x100u16); }
  if cpu.zf { pc = 0x50af22e; continue; }
  pc = 0x50af22c; continue;
 }
 0x50af22c => {
  { let v = -cpu.st(0); cpu.set_st(0, v); }
  pc = 0x50af22e; continue;
 }
 0x50af22e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af22f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af22f); }
 let mut pc: u32 = 0x50af22f;
 loop { match pc {
 0x50af22f => {
  { cpu.and8(((cpu.ebx >> 8) as u8), 0x4u8); }
  if !cpu.zf { pc = 0x50af238; continue; }
  pc = 0x50af234; continue;
 }
 0x50af234 => {
  { let v: u8 = 0x0u8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50af23d; continue;
 }
 0x50af238 => {
  { let v: u8 = (cpu.ebx as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.and8((cpu.edx as u8), 0x7u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  pc = 0x50af23d; continue;
 }
 0x50af23d => {
  { let v = cpu.edx; cpu.push(v); }
  pc = 0x50af23e; continue;
 }
 0x50af23e => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x50af246; continue; }
  pc = 0x50af242; continue;
 }
 0x50af242 => {
  cpu.unimplemented(0x50af242, "Fincstp");
  cpu.poll(); pc = 0x50af23e; continue;
 }
 0x50af246 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf80(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x50af24b; continue;
 }
 0x50af24b => {
  { let v: u8 = cpu.dec8((cpu.edx as u8)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  if (cpu.sf != cpu.of) { pc = 0x50af253; continue; }
  pc = 0x50af24f; continue;
 }
 0x50af24f => {
  cpu.unimplemented(0x50af24f, "Fdecstp");
  cpu.poll(); pc = 0x50af24b; continue;
 }
 0x50af253 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af254(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af254); }
 let mut pc: u32 = 0x50af254;
 loop { match pc {
 0x50af254 => {
  { cpu.and8((cpu.ebx as u8), 0x4u8); }
  if cpu.zf { pc = 0x50af25e; continue; }
  pc = 0x50af259; continue;
 }
 0x50af259 => {
  { let v = cpu.st(0); cpu.wf64(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x50af261; continue;
 }
 0x50af25e => {
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[0]), v); }
  pc = 0x50af261; continue;
 }
 0x50af261 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af262(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af262); }
 let mut pc: u32 = 0x50af262;
 loop { match pc {
 0x50af262 => {
  { cpu.push(cpu.segments[3] as u32); }
  { cpu.sub8(cpu.r8(0x514c14bu32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x50af274; continue; }
  pc = 0x50af26c; continue;
 }
 0x50af26c => {
  { let v: u32 = 0x50af017u32; cpu.edx = v; }
  pc = 0x50af27a; continue;
 }
 0x50af274 => {
  { let v: u32 = 0x50af024u32; cpu.edx = v; }
  pc = 0x50af27a; continue;
 }
 0x50af27a => {
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x50af2a1; continue; }
  pc = 0x50af283; continue;
 }
 0x50af283 => {
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
  pc = 0x50af2a1; continue;
 }
 0x50af2a1 => {
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x514c149u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_sub(0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u16 = 0x2502u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bf22u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x514bf20u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = 0x2503u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bf26u32.wrapping_add(cpu.bases[3]), v); }
  { cpu.sub8(cpu.r8(0x514ad16u32.wrapping_add(cpu.bases[3])), 0x3u8); }
  if !cpu.cf { pc = 0x50af318; continue; }
  pc = 0x50af2da; continue;
 }
 0x50af2da => {
  { let v: u32 = cpu.esp.wrapping_sub(0x8u32); cpu.esp = v; }
  cpu.unimplemented(0x50af2dd, "Sidt");
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
  pc = 0x50af323; continue;
 }
 0x50af318 => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.eax; cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2506u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  pc = 0x50af323; continue;
 }
 0x50af323 => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50af324); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af325(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af325); }
 let mut pc: u32 = 0x50af325;
 loop { match pc {
 0x50af325 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u16 = 0x250cu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.ecx as u8).wrapping_add(cpu.r8(0x514c149u32.wrapping_add(cpu.bases[3]))); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.sub8((cpu.ecx as u8), 0x8u8); cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x514bf26u32.wrapping_add(cpu.bases[3])); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x514bf22u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x514bf20u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { let v: u16 = 0x2507u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50af351); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af352(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af352); }
 let mut pc: u32 = 0x50af352;
 loop { match pc {
 0x50af352 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x514c148u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bf22u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x514bf20u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x514c148u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.segments[1] as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = cpu.edx; cpu.set_segment(3, v as u16); }
  { let v: u32 = 0x50af024u32; cpu.edx = v; }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50af37e); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af37f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af37f); }
 let mut pc: u32 = 0x50af37f;
 loop { match pc {
 0x50af37f => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x514c148u32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x514bf22u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x514bf20u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50af397); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af398); }
 let mut pc: u32 = 0x50af398;
 loop { match pc {
 0x50af398 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x35u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x514c14au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x21); }
  { let v: u32 = cpu.ebx; cpu.w32(0x514bf22u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x514bf20u32.wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x514c14au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { cpu.sub8(cpu.r8(0x514c14bu32.wrapping_add(cpu.bases[3])), 0x20u8); }
  if !cpu.zf { pc = 0x50af3c7; continue; }
  pc = 0x50af3bf; continue;
 }
 0x50af3bf => {
  { let v: u32 = 0x50af017u32; cpu.edx = v; }
  pc = 0x50af3cd; continue;
 }
 0x50af3c7 => {
  { let v: u32 = 0x50af024u32; cpu.edx = v; }
  pc = 0x50af3cd; continue;
 }
 0x50af3cd => {
  { let v: u16 = (cpu.segments[1] as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.ebx; cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50af3d5); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af3d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af3d6); }
 let mut pc: u32 = 0x50af3d6;
 loop { match pc {
 0x50af3d6 => {
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u8 = 0x25u8; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x514c14au32.wrapping_add(cpu.bases[3])); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x514bf22u32.wrapping_add(cpu.bases[3])); cpu.edx = v; }
  { let v = cpu.r16(0x514bf20u32.wrapping_add(cpu.bases[3])); cpu.set_segment(3, v as u16); }
  { cpu.interrupt(0x21); }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  cpu.flat_ds(0x50af3ee); cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af3f0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af3f0); }
 let mut pc: u32 = 0x50af3f0;
 loop { match pc {
 0x50af3f0 => {
  { let v: u16 = 0x204u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x514c148u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.edx; cpu.w32(0x514bf22u32, v); }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x514bf20u32, v); }
  { let v: u16 = 0x200u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x514c148u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0x514bf26u32, v); }
  { let v: u16 = (cpu.segments[1] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub8(cpu.r8(0x514c14bu32), 0x20u8); }
  if !cpu.zf { pc = 0x50af435; continue; }
  pc = 0x50af42d; continue;
 }
 0x50af42d => {
  { let v: u32 = 0x50af017u32; cpu.edx = v; }
  pc = 0x50af43b; continue;
 }
 0x50af435 => {
  { let v: u32 = 0x50af024u32; cpu.edx = v; }
  pc = 0x50af43b; continue;
 }
 0x50af43b => {
  { let v: u16 = 0x205u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.r8(0x514c148u32); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { cpu.interrupt(0x31); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af448(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af448); }
 let mut pc: u32 = 0x50af448;
 loop { match pc {
 0x50af448 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af450(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af450); }
 let mut pc: u32 = 0x50af450;
 loop { match pc {
 0x50af450 => {
  { cpu.pushad(); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  pc = 0x50af455; continue;
 }
 0x50af455 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x50af463; continue; }
  pc = 0x50af45f; continue;
 }
 0x50af45f => {
  { let v: u16 = (cpu.eax as u16) | 0x1fu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50af463; continue;
 }
 0x50af463 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if cpu.zf { pc = 0x50af46f; continue; }
  pc = 0x50af46b; continue;
 }
 0x50af46b => {
  { let v: u16 = (cpu.eax as u16) | 0x7e0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50af46f; continue;
 }
 0x50af46f => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if cpu.zf { pc = 0x50af47b; continue; }
  pc = 0x50af477; continue;
 }
 0x50af477 => {
  { let v: u16 = (cpu.eax as u16) | 0xf800u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  pc = 0x50af47b; continue;
 }
 0x50af47b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5212ca0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { cpu.and32(cpu.ecx, 0x1u32); }
  if !cpu.zf { pc = 0x50af492; continue; }
  pc = 0x50af48f; continue;
 }
 0x50af48f => {
  { let v: u32 = cpu.eax & 0xffffffe0u32; cpu.eax = v; }
  pc = 0x50af492; continue;
 }
 0x50af492 => {
  { cpu.and32(cpu.ecx, 0x40u32); }
  if !cpu.zf { pc = 0x50af49f; continue; }
  pc = 0x50af49a; continue;
 }
 0x50af49a => {
  { let v: u32 = cpu.eax & 0xfffff81fu32; cpu.eax = v; }
  pc = 0x50af49f; continue;
 }
 0x50af49f => {
  { cpu.and32(cpu.ecx, 0x800u32); }
  if !cpu.zf { pc = 0x50af4ac; continue; }
  pc = 0x50af4a7; continue;
 }
 0x50af4a7 => {
  { let v: u32 = cpu.eax & 0xffff07ffu32; cpu.eax = v; }
  pc = 0x50af4ac; continue;
 }
 0x50af4ac => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5214ca0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1000u32); }
  if !cpu.zf { cpu.poll(); pc = 0x50af455; continue; }
  pc = 0x50af4bf; continue;
 }
 0x50af4bf => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af4c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af4c1); }
 let mut pc: u32 = 0x50af4c1;
 loop { match pc {
 0x50af4c1 => {
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50af4e1; continue; }
  pc = 0x50af4c8; continue;
 }
 0x50af4c8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.or16((cpu.eax as u16), cpu.r16(0x5212ca0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af4e1; continue;
 }
 0x50af4e1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af4e2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af4e2); }
 let mut pc: u32 = 0x50af4e2;
 loop { match pc {
 0x50af4e2 => {
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16) ^ (cpu.eax as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x50af50b; continue; }
  pc = 0x50af4f2; continue;
 }
 0x50af4f2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = cpu.and16((cpu.eax as u16), cpu.r16(0x5214ca0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1)))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af50b; continue;
 }
 0x50af50b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af50c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af50c); }
 let mut pc: u32 = 0x50af50c;
 loop { match pc {
 0x50af50c => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if cpu.zf { pc = 0x50af527; continue; }
  pc = 0x50af510; continue;
 }
 0x50af510 => {
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x7bef7befu32; cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af527; continue;
 }
 0x50af527 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af528(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af528); }
 let mut pc: u32 = 0x50af528;
 loop { match pc {
 0x50af528 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.edi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.ecx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.edi, cpu.edx); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v = cpu.esi; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.edi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050af54d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50af54d); }
 let mut pc: u32 = 0x50af54d;
 loop { match pc {
 0x50af54d => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x5143044u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5143040u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514302cu32, v); }
  pc = 0x50afa25; continue;
 }
 0x50af57e => {
  { let v: u32 = cpu.r32(0x5143040u32); cpu.ecx = v; }
  pc = 0x50af79c; continue;
 }
 0x50af589 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af5a7; continue; }
  pc = 0x50af590; continue;
 }
 0x50af590 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af5a7; continue;
 }
 0x50af5a7 => {
  { let v: u32 = cpu.r32(0x2u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af5c8; continue; }
  pc = 0x50af5af; continue;
 }
 0x50af5af => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af5c8; continue;
 }
 0x50af5c8 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af5e9; continue; }
  pc = 0x50af5d0; continue;
 }
 0x50af5d0 => {
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af5e9; continue;
 }
 0x50af5e9 => {
  { let v: u32 = cpu.r32(0x6u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af60a; continue; }
  pc = 0x50af5f1; continue;
 }
 0x50af5f1 => {
  { let v: u16 = cpu.r16(0x6u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af60a; continue;
 }
 0x50af60a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af62b; continue; }
  pc = 0x50af612; continue;
 }
 0x50af612 => {
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af62b; continue;
 }
 0x50af62b => {
  { let v: u32 = cpu.r32(0xau32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af64c; continue; }
  pc = 0x50af633; continue;
 }
 0x50af633 => {
  { let v: u16 = cpu.r16(0xau32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x50af64c; continue;
 }
 0x50af64c => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af66d; continue; }
  pc = 0x50af654; continue;
 }
 0x50af654 => {
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x50af66d; continue;
 }
 0x50af66d => {
  { let v: u32 = cpu.r32(0xeu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af68e; continue; }
  pc = 0x50af675; continue;
 }
 0x50af675 => {
  { let v: u16 = cpu.r16(0xeu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x50af68e; continue;
 }
 0x50af68e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af6af; continue; }
  pc = 0x50af696; continue;
 }
 0x50af696 => {
  { let v: u16 = cpu.r16(0x10u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af6af; continue;
 }
 0x50af6af => {
  { let v: u32 = cpu.r32(0x12u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af6d0; continue; }
  pc = 0x50af6b7; continue;
 }
 0x50af6b7 => {
  { let v: u16 = cpu.r16(0x12u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af6d0; continue;
 }
 0x50af6d0 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af6f1; continue; }
  pc = 0x50af6d8; continue;
 }
 0x50af6d8 => {
  { let v: u16 = cpu.r16(0x14u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af6f1; continue;
 }
 0x50af6f1 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af712; continue; }
  pc = 0x50af6f9; continue;
 }
 0x50af6f9 => {
  { let v: u16 = cpu.r16(0x16u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af712; continue;
 }
 0x50af712 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af733; continue; }
  pc = 0x50af71a; continue;
 }
 0x50af71a => {
  { let v: u16 = cpu.r16(0x18u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af733; continue;
 }
 0x50af733 => {
  { let v: u32 = cpu.r32(0x1au32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af754; continue; }
  pc = 0x50af73b; continue;
 }
 0x50af73b => {
  { let v: u16 = cpu.r16(0x1au32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x50af754; continue;
 }
 0x50af754 => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af775; continue; }
  pc = 0x50af75c; continue;
 }
 0x50af75c => {
  { let v: u16 = cpu.r16(0x1cu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x50af775; continue;
 }
 0x50af775 => {
  { let v: u32 = cpu.r32(0x1eu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af796; continue; }
  pc = 0x50af77d; continue;
 }
 0x50af77d => {
  { let v: u16 = cpu.r16(0x1eu32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x50af796; continue;
 }
 0x50af796 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x50af79c; continue;
 }
 0x50af79c => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x50af589; continue; }
  pc = 0x50af7a5; continue;
 }
 0x50af7a5 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af7b1; continue;
 }
 0x50af7b1 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af7cf; continue; }
  pc = 0x50af7b8; continue;
 }
 0x50af7b8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af7cf; continue;
 }
 0x50af7cf => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af7ef; continue; }
  pc = 0x50af7d6; continue;
 }
 0x50af7d6 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af7ef; continue;
 }
 0x50af7ef => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af7fb; continue;
 }
 0x50af7fb => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af819; continue; }
  pc = 0x50af802; continue;
 }
 0x50af802 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af819; continue;
 }
 0x50af819 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af839; continue; }
  pc = 0x50af820; continue;
 }
 0x50af820 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af839; continue;
 }
 0x50af839 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af845; continue;
 }
 0x50af845 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af863; continue; }
  pc = 0x50af84c; continue;
 }
 0x50af84c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af863; continue;
 }
 0x50af863 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af883; continue; }
  pc = 0x50af86a; continue;
 }
 0x50af86a => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af883; continue;
 }
 0x50af883 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af88f; continue;
 }
 0x50af88f => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af8ad; continue; }
  pc = 0x50af896; continue;
 }
 0x50af896 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af8ad; continue;
 }
 0x50af8ad => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af8cd; continue; }
  pc = 0x50af8b4; continue;
 }
 0x50af8b4 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af8cd; continue;
 }
 0x50af8cd => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af8d9; continue;
 }
 0x50af8d9 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af8f7; continue; }
  pc = 0x50af8e0; continue;
 }
 0x50af8e0 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af8f7; continue;
 }
 0x50af8f7 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af917; continue; }
  pc = 0x50af8fe; continue;
 }
 0x50af8fe => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af917; continue;
 }
 0x50af917 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af923; continue;
 }
 0x50af923 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af941; continue; }
  pc = 0x50af92a; continue;
 }
 0x50af92a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af941; continue;
 }
 0x50af941 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af961; continue; }
  pc = 0x50af948; continue;
 }
 0x50af948 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af961; continue;
 }
 0x50af961 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af96d; continue;
 }
 0x50af96d => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af98b; continue; }
  pc = 0x50af974; continue;
 }
 0x50af974 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af98b; continue;
 }
 0x50af98b => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af9ab; continue; }
  pc = 0x50af992; continue;
 }
 0x50af992 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af9ab; continue;
 }
 0x50af9ab => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50af9f4; continue; }
  pc = 0x50af9b3; continue;
 }
 0x50af9b3 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af9d1; continue; }
  pc = 0x50af9ba; continue;
 }
 0x50af9ba => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af9d1; continue;
 }
 0x50af9d1 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50af9f1; continue; }
  pc = 0x50af9d8; continue;
 }
 0x50af9d8 => {
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50af9f1; continue;
 }
 0x50af9f1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50af9f4; continue;
 }
 0x50af9f4 => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x50afa1d; continue; }
  pc = 0x50af9fc; continue;
 }
 0x50af9fc => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50afa1a; continue; }
  pc = 0x50afa03; continue;
 }
 0x50afa03 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afa1a; continue;
 }
 0x50afa1a => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50afa1d; continue;
 }
 0x50afa1d => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x514302cu32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x50afa25; continue;
 }
 0x50afa25 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5143044u32)); cpu.w32(0x5143044u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x50af57e; continue; }
  pc = 0x50afa31; continue;
 }
 0x50afa31 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050afa35(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50afa35); }
 let mut pc: u32 = 0x50afa35;
 loop { match pc {
 0x50afa35 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x5143044u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5143040u32, v); }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5142fd0u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x50afa87; continue;
 }
 0x50afa62 => {
  { let v: u32 = cpu.r32(0x5143040u32); cpu.ecx = v; }
  pc = 0x50afa68; continue;
 }
 0x50afa68 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x50afa7a; continue; }
  pc = 0x50afa70; continue;
 }
 0x50afa70 => {
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x5142fd0u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afa7a; continue;
 }
 0x50afa7a => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50afa68; continue; }
  pc = 0x50afa83; continue;
 }
 0x50afa83 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x50afa87; continue;
 }
 0x50afa87 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5143044u32)); cpu.w32(0x5143044u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x50afa62; continue; }
  pc = 0x50afa8f; continue;
 }
 0x50afa8f => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050afa93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50afa93); }
 let mut pc: u32 = 0x50afa93;
 loop { match pc {
 0x50afa93 => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x5143044u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5143040u32, v); }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ebx as u16); cpu.w16(0x5142fd2u32, v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = cpu.ecx; cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514302cu32, v); }
  pc = 0x50afb02; continue;
 }
 0x50afac2 => {
  { let v: u32 = cpu.r32(0x5143040u32); cpu.ecx = v; }
  pc = 0x50afac8; continue;
 }
 0x50afac8 => {
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { cpu.sub16((cpu.edx as u16), cpu.r16(0x5142fd2u32)); }
  if cpu.zf { pc = 0x50afaf1; continue; }
  pc = 0x50afad6; continue;
 }
 0x50afad6 => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) | cpu.r16(0x5212ca0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afaf1; continue;
 }
 0x50afaf1 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50afac8; continue; }
  pc = 0x50afafa; continue;
 }
 0x50afafa => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x514302cu32)); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x50afb02; continue;
 }
 0x50afb02 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5143044u32)); cpu.w32(0x5143044u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x50afac2; continue; }
  pc = 0x50afb0a; continue;
 }
 0x50afb0a => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050afb0e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50afb0e); } r_050afb0e(cpu, 0x50afb0e); }
pub fn f_050afeee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50afeee); } r_050afb0e(cpu, 0x50afeee); }
pub fn f_050aff47(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50aff47); } r_050afb0e(cpu, 0x50aff47); }
pub fn f_050affa0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50affa0); } r_050afb0e(cpu, 0x50affa0); }
pub fn f_050afff9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50afff9); } r_050afb0e(cpu, 0x50afff9); }
pub fn f_050b0041(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b0041); } r_050afb0e(cpu, 0x50b0041); }
pub fn f_050b009a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b009a); } r_050afb0e(cpu, 0x50b009a); }
pub fn f_050b00f3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b00f3); } r_050afb0e(cpu, 0x50b00f3); }
pub fn f_050b014c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b014c); } r_050afb0e(cpu, 0x50b014c); }
fn r_050afb0e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50afb0e => {
  { cpu.push(cpu.ebp); cpu.ebp = cpu.esp; cpu.esp = cpu.esp.wrapping_sub(0); }
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fdcu32, v); }
  { let v: u32 = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fe0u32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x1cu32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.eax); cpu.w32(0x10u32.wrapping_add(cpu.ebp), v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fe8u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5142ff0u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fecu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5142ff4u32, v); }
  { let v: u32 = cpu.r32(0x5142ff0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142ff8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x5142ffcu32, v); }
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5143014u32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x50afba5; continue; }
  pc = 0x50afb6e; continue;
 }
 0x50afb6e => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5142fdcu32).wrapping_sub(cpu.eax); cpu.w32(0x5142fdcu32, v); }
  { cpu.and32(cpu.r32(0x5142fdcu32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0167; continue; }
  pc = 0x50afb86; continue;
 }
 0x50afb86 => {
  { let v = cpu.r32(0x5142ff0u32); cpu.imul32(v); }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x5142fe8u32).wrapping_sub(cpu.eax); cpu.w32(0x5142fe8u32, v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142ff8u32, v); }
  { let v: u32 = cpu.r32(0x5143014u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x28u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50afba5; continue;
 }
 0x50afba5 => {
  { let v: u32 = cpu.r32(0x28u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5142fdcu32)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x514301cu32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50afbdc; continue; }
  pc = 0x50afbb7; continue;
 }
 0x50afbb7 => {
  { let v: u32 = cpu.r32(0x5142fdcu32).wrapping_sub(cpu.eax); cpu.w32(0x5142fdcu32, v); }
  { cpu.and32(cpu.r32(0x5142fdcu32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0167; continue; }
  pc = 0x50afbcd; continue;
 }
 0x50afbcd => {
  { let v = cpu.r32(0x5142ff0u32); cpu.imul32(v); }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x5142fe8u32).wrapping_sub(cpu.eax); cpu.w32(0x5142fe8u32, v); }
  pc = 0x50afbdc; continue;
 }
 0x50afbdc => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5143018u32)); cpu.eax = v; }
  if !cpu.sf { pc = 0x50afc1e; continue; }
  pc = 0x50afbe7; continue;
 }
 0x50afbe7 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5142fe0u32).wrapping_sub(cpu.eax); cpu.w32(0x5142fe0u32, v); }
  { cpu.and32(cpu.r32(0x5142fe0u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0167; continue; }
  pc = 0x50afbff; continue;
 }
 0x50afbff => {
  { let v = cpu.r32(0x5142ff4u32); cpu.imul32(v); }
  { let v = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x5142fecu32).wrapping_sub(cpu.eax); cpu.w32(0x5142fecu32, v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142ffcu32, v); }
  { let v: u32 = cpu.r32(0x5143018u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2cu32.wrapping_add(cpu.ebp), v); }
  pc = 0x50afc1e; continue;
 }
 0x50afc1e => {
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5142fe0u32)); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5143020u32)); cpu.eax = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50afc55; continue; }
  pc = 0x50afc30; continue;
 }
 0x50afc30 => {
  { let v: u32 = cpu.r32(0x5142fe0u32).wrapping_sub(cpu.eax); cpu.w32(0x5142fe0u32, v); }
  { cpu.and32(cpu.r32(0x5142fe0u32), 0xffffffffu32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0167; continue; }
  pc = 0x50afc46; continue;
 }
 0x50afc46 => {
  { let v = cpu.r32(0x5142ff4u32); cpu.imul32(v); }
  { let v = cpu.r32(0x24u32.wrapping_add(cpu.ebp)); cpu.idiv32(v); }
  { let v: u32 = cpu.r32(0x5142fecu32).wrapping_sub(cpu.eax); cpu.w32(0x5142fecu32, v); }
  pc = 0x50afc55; continue;
 }
 0x50afc55 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fe4u32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x2cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.imul2_32(cpu.eax, cpu.r32(0x1cu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x28u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5142fe8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5142fdcu32); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143000u32, v); }
  { let v: u32 = cpu.r32(0x5142fecu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5142fe0u32); cpu.ebx = v; }
  { cpu.edx = ((cpu.eax as i32) >> 31) as u32; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143004u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x514300cu32, v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5142fdcu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5142fdcu32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143010u32, v); }
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ebp)), 0x1u32); }
  if cpu.zf { pc = 0x50b016a; continue; }
  pc = 0x50afcc1; continue;
 }
 0x50afcc1 => {
  { cpu.sub32(cpu.r32(0x30u32.wrapping_add(cpu.ebp)), 0x2u32); }
  if cpu.zf { pc = 0x50b0207; continue; }
  pc = 0x50afccb; continue;
 }
 0x50afccb => {
  { let v: u32 = cpu.r32(0x5142ffcu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5142fe0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143024u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x5143024u32)); cpu.w32(0x5143024u32, v); }
  pc = 0x50b015b; continue;
 }
 0x50afce6 => {
  { let v: u32 = cpu.r32(0x5142fe4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x514300cu32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5142ff8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5142fdcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5143028u32, v); }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50afd5b; continue; }
  pc = 0x50afd18; continue;
 }
 0x50afd18 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50afd58; continue; }
  pc = 0x50afd2d; continue;
 }
 0x50afd2d => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x10820u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afd58; continue;
 }
 0x50afd58 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50afd5b; continue;
 }
 0x50afd5b => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x5143028u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50afedd; continue; }
  pc = 0x50afd6a; continue;
 }
 0x50afd6a => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50afdc3; continue; }
  pc = 0x50afd97; continue;
 }
 0x50afd97 => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afdc3; continue;
 }
 0x50afdc3 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50afe1c; continue; }
  pc = 0x50afdf0; continue;
 }
 0x50afdf0 => {
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afe1c; continue;
 }
 0x50afe1c => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50afe75; continue; }
  pc = 0x50afe49; continue;
 }
 0x50afe49 => {
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afe75; continue;
 }
 0x50afe75 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50afece; continue; }
  pc = 0x50afea2; continue;
 }
 0x50afea2 => {
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x50afece; continue;
 }
 0x50afece => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x5143028u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50afd6a; continue; }
  pc = 0x50afedd; continue;
 }
 0x50afedd => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143028u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5142f7cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50afeee => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50aff44; continue; }
  pc = 0x50aff1b; continue;
 }
 0x50aff1b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50aff44; continue;
 }
 0x50aff44 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50aff47; continue;
 }
 0x50aff47 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50aff9d; continue; }
  pc = 0x50aff74; continue;
 }
 0x50aff74 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50aff9d; continue;
 }
 0x50aff9d => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50affa0; continue;
 }
 0x50affa0 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50afff6; continue; }
  pc = 0x50affcd; continue;
 }
 0x50affcd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50afff6; continue;
 }
 0x50afff6 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50afff9; continue;
 }
 0x50afff9 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50b0039; continue; }
  pc = 0x50b000e; continue;
 }
 0x50b000e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x10820u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b0039; continue;
 }
 0x50b0039 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b014c; continue;
 }
 0x50b0041 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50b0097; continue; }
  pc = 0x50b006e; continue;
 }
 0x50b006e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b0097; continue;
 }
 0x50b0097 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b009a; continue;
 }
 0x50b009a => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50b00f0; continue; }
  pc = 0x50b00c7; continue;
 }
 0x50b00c7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b00f0; continue;
 }
 0x50b00f0 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b00f3; continue;
 }
 0x50b00f3 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax & 0xffffu32; cpu.eax = v; }
  { let v: u32 = cpu.ebx & 0xffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.esi).wrapping_add(cpu.ebx.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { pc = 0x50b0149; continue; }
  pc = 0x50b0120; continue;
 }
 0x50b0120 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0x8210821u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 15u32); cpu.ebx = v; }
  { let v: u32 = cpu.ecx | cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b0149; continue;
 }
 0x50b0149 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b014c; continue;
 }
 0x50b014c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x5143004u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5143010u32)); cpu.edi = v; }
  pc = 0x50b015b; continue;
 }
 0x50b015b => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5143024u32)); cpu.w32(0x5143024u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50afce6; continue; }
  pc = 0x50b0167; continue;
 }
 0x50b0167 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b016a => {
  { let v: u32 = cpu.r32(0x5142ffcu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5142fe0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143024u32, v); }
  pc = 0x50b01f8; continue;
 }
 0x50b017c => {
  { let v: u32 = cpu.r32(0x5142fe4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x514300cu32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5142ff8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5142fdcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5143028u32, v); }
  pc = 0x50b01a6; continue;
 }
 0x50b01a6 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = 0xffffu32; cpu.ecx = v; }
  { let v: u16 = (cpu.ecx as u16) ^ cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { cpu.sub32(cpu.ecx, 0xffffu32); }
  if cpu.zf { pc = 0x50b01de; continue; }
  pc = 0x50b01c5; continue;
 }
 0x50b01c5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.ecx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x5214ca0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b01de; continue;
 }
 0x50b01de => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x5143028u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b01a6; continue; }
  pc = 0x50b01e9; continue;
 }
 0x50b01e9 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x5143004u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5143010u32)); cpu.edi = v; }
  pc = 0x50b01f8; continue;
 }
 0x50b01f8 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5143024u32)); cpu.w32(0x5143024u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b017c; continue; }
  pc = 0x50b0204; continue;
 }
 0x50b0204 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b0207 => {
  { let v: u32 = cpu.r32(0x5142ffcu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5142fe0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143024u32, v); }
  pc = 0x50b0276; continue;
 }
 0x50b0219 => {
  { let v: u32 = cpu.r32(0x5142fe4u32); cpu.esi = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.imul2_32(cpu.edx, cpu.r32(0x514300cu32)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5142ff8u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5142fdcu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.edi); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x5143028u32, v); }
  pc = 0x50b0243; continue;
 }
 0x50b0243 => {
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x5143000u32)); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.edx, 0x0u32); cpu.edx = v; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = (cpu.edx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx ^ cpu.ecx; cpu.ecx = v; }
  { let v: u16 = cpu.or16((cpu.ecx as u16), cpu.r16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(2)))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  if cpu.zf { pc = 0x50b025c; continue; }
  pc = 0x50b0259; continue;
 }
 0x50b0259 => {
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b025c; continue;
 }
 0x50b025c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x5143028u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b0243; continue; }
  pc = 0x50b0267; continue;
 }
 0x50b0267 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x5143004u32)); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, 0x0u32); cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5143010u32)); cpu.edi = v; }
  pc = 0x50b0276; continue;
 }
 0x50b0276 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x5143024u32)); cpu.w32(0x5143024u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b0219; continue; }
  pc = 0x50b027e; continue;
 }
 0x50b027e => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0281(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0281); }
 let mut pc: u32 = 0x50b0281;
 loop { match pc {
 0x50b0281 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50b0297; continue; }
  pc = 0x50b028b; continue;
 }
 0x50b028b => {
  if cpu.zf { pc = 0x50b029d; continue; }
  pc = 0x50b028d; continue;
 }
 0x50b028d => {
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.movs(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x50b0281; continue; }
  pc = 0x50b0293; continue;
 }
 0x50b0293 => {
  { cpu.movs(2, false); }
  cpu.poll(); pc = 0x50b0281; continue;
 }
 0x50b0297 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x50b0281; continue;
 }
 0x50b029d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b029e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b029e); }
 let mut pc: u32 = 0x50b029e;
 loop { match pc {
 0x50b029e => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50b02d4; continue; }
  pc = 0x50b02a8; continue;
 }
 0x50b02a8 => {
  if cpu.zf { pc = 0x50b02da; continue; }
  pc = 0x50b02aa; continue;
 }
 0x50b02aa => {
  { let v: u32 = 0xffffu32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16) ^ cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx ^ cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 5u32); cpu.ebx = v; }
  { let v: u16 = (cpu.eax as u16) & cpu.r16(0x5214ca0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b02aa; continue; }
  pc = 0x50b02d2; continue;
 }
 0x50b02d2 => {
  cpu.poll(); pc = 0x50b029e; continue;
 }
 0x50b02d4 => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x50b029e; continue;
 }
 0x50b02da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b02db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b02db); }
 let mut pc: u32 = 0x50b02db;
 loop { match pc {
 0x50b02db => {
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.edx; cpu.edx = v; }
  pc = 0x50b02df; continue;
 }
 0x50b02df => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50b030c; continue; }
  pc = 0x50b02e9; continue;
 }
 0x50b02e9 => {
  if cpu.zf { pc = 0x50b0312; continue; }
  pc = 0x50b02eb; continue;
 }
 0x50b02eb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edx); cpu.eax = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b02eb; continue; }
  pc = 0x50b030a; continue;
 }
 0x50b030a => {
  cpu.poll(); pc = 0x50b02df; continue;
 }
 0x50b030c => {
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x50b02df; continue;
 }
 0x50b0312 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0313(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0313); }
 let mut pc: u32 = 0x50b0313;
 loop { match pc {
 0x50b0313 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5142fccu32, v); }
  { let v = cpu.rf32(0x5142fbcu32); cpu.fpush(v); }
  { let v = cpu.rf32(0x5291688u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5142fc8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5142fb4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5142facu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fc0u32, v); }
  { let v: u32 = cpu.r32(0x5142fb0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143044u32, v); }
  { let v: u32 = cpu.r32(0x5142fa8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143048u32, v); }
  pc = 0x50b0352; continue;
 }
 0x50b0352 => {
  { let v: u32 = cpu.r32(0x5142fccu32); cpu.esi = v; }
  pc = 0x50b0358; continue;
 }
 0x50b0358 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x10u32.wrapping_add(cpu.esi)), 0x0u32); }
  if cpu.zf { pc = 0x50b053b; continue; }
  pc = 0x50b0365; continue;
 }
 0x50b0365 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); }
  if (cpu.sf != cpu.of) { pc = 0x50b052d; continue; }
  pc = 0x50b036e; continue;
 }
 0x50b036e => {
  { let v = cpu.edx; cpu.push(v); }
  { let v = cpu.rf32(0x5142fc8u32); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5143048u32); cpu.ebp = v; }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0xcu32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x5142fb8u32); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x5142fc4u32, v); }
  { cpu.and32(cpu.ebp, 0x2u32); }
  if !cpu.zf { pc = 0x50b0414; continue; }
  pc = 0x50b03b2; continue;
 }
 0x50b03b2 => {
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5142fbcu32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x5143034u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5143034u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b0411; continue; }
  pc = 0x50b03c8; continue;
 }
 0x50b03c8 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x5142fd8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5143034u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5143034u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x50b03e8; continue; }
  pc = 0x50b03e6; continue;
 }
 0x50b03e6 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x50b03e8; continue;
 }
 0x50b03e8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = cpu.r16(0x5216ca0u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x5236ca0u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50b0411; continue;
 }
 0x50b0411 => {
  { let v: u32 = cpu.ebp.wrapping_sub(0x2u32); cpu.ebp = v; }
  pc = 0x50b0414; continue;
 }
 0x50b0414 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5142fc4u32)); cpu.eax = v; }
  if cpu.zf { pc = 0x50b04c0; continue; }
  pc = 0x50b0422; continue;
 }
 0x50b0422 => {
  if cpu.sf { pc = 0x50b051f; continue; }
  pc = 0x50b0428; continue;
 }
 0x50b0428 => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  pc = 0x50b042a; continue;
 }
 0x50b042a => {
  { let v: u32 = cpu.r32(0x5142fbcu32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x5143034u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5143034u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b04ab; continue; }
  pc = 0x50b043e; continue;
 }
 0x50b043e => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x5142fd8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5143034u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5143034u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x50b045e; continue; }
  pc = 0x50b045c; continue;
 }
 0x50b045c => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x50b045e; continue;
 }
 0x50b045e => {
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebp)); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffe00000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5216c9eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x1f0000u32; cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 11u32); cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.ecx | cpu.r32(0x5236c9eu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5216ca0u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x5236ca0u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.w32(0xfffffffeu32.wrapping_add(cpu.ebp), v); }
  pc = 0x50b04ab; continue;
 }
 0x50b04ab => {
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.ebp.wrapping_sub(0x4u32); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x5142fc4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b042a; continue; }
  pc = 0x50b04bc; continue;
 }
 0x50b04bc => {
  if !cpu.zf { pc = 0x50b051f; continue; }
  pc = 0x50b04be; continue;
 }
 0x50b04be => {
  pc = 0x50b04c2; continue;
 }
 0x50b04c0 => {
  { let v = cpu.st(0) + cpu.st(2); cpu.set_st(0, v); }
  pc = 0x50b04c2; continue;
 }
 0x50b04c2 => {
  { let v: u32 = cpu.r32(0x5142fbcu32); cpu.ebx = v; }
  { let v = cpu.st(0); cpu.wf32(0x5143034u32, v); }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5143034u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b051f; continue; }
  pc = 0x50b04d6; continue;
 }
 0x50b04d6 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x5142fd8u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5143034u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5143034u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ebx, 0x4b400000u32); cpu.ebx = v; }
  if !cpu.sf { pc = 0x50b04f6; continue; }
  pc = 0x50b04f4; continue;
 }
 0x50b04f4 => {
  { let v: u32 = cpu.ebx ^ cpu.ebx; cpu.ebx = v; }
  pc = 0x50b04f6; continue;
 }
 0x50b04f6 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebp)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xffe0u32; cpu.eax = v; }
  { let v: u32 = cpu.eax | cpu.ebx; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5216ca0u32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx & 0x1fu32; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 5u32); cpu.edx = v; }
  { let v: u32 = cpu.edx | cpu.ebx; cpu.edx = v; }
  { let v: u16 = (cpu.ecx as u16) | cpu.r16(0x5236ca0u32.wrapping_add(cpu.edx.wrapping_mul(2))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.ecx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebp), v); }
  pc = 0x50b051f; continue;
 }
 0x50b051f => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.poll(); pc = 0x50b0358; continue;
 }
 0x50b052d => {
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b036e; continue; }
  pc = 0x50b0536; continue;
 }
 0x50b0536 => {
  cpu.poll(); pc = 0x50b0358; continue;
 }
 0x50b053b => {
  { let v: u32 = cpu.r32(0x5142fccu32).wrapping_add(0x1cu32); cpu.w32(0x5142fccu32, v); }
  { let v: u32 = cpu.r32(0x5142fc0u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.r32(0x5143048u32), cpu.eax); cpu.w32(0x5143048u32, v); }
  { let v: u32 = cpu.dec32(cpu.r32(0x5143044u32)); cpu.w32(0x5143044u32, v); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b0352; continue; }
  pc = 0x50b0559; continue;
 }
 0x50b0559 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b055b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b055b); }
 let mut pc: u32 = 0x50b055b;
 loop { match pc {
 0x50b055b => {
  { cpu.pushad(); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52374a0u32, v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  pc = 0x50b056e; continue;
 }
 0x50b056e => {
  { let v: u32 = cpu.or32(cpu.ebx, cpu.ebx); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b05ca; continue; }
  pc = 0x50b0572; continue;
 }
 0x50b0572 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ecx; cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x52374a8u32, v); }
  { let v: u32 = cpu.r32(0x52374a0u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x52374a8u32); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5237804u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.edx.wrapping_mul(1)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5237cb8u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x52374a0u32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x52374a4u32, v); }
  { let v: u32 = cpu.esi.wrapping_sub(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x52374a8u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5237804u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  { let v: u32 = cpu.r32(0x52374a4u32); cpu.edi = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x5237cb8u32.wrapping_add(cpu.esi.wrapping_mul(4)), v); }
  pc = 0x50b05d0; continue;
 }
 0x50b05ca => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.edx); cpu.edx = v; }
  pc = 0x50b05d0; continue;
 }
 0x50b05d0 => {
  { let v: u32 = cpu.or32(cpu.eax, cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b056e; continue; }
  pc = 0x50b05d4; continue;
 }
 0x50b05d4 => {
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b05d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b05d6); }
 let mut pc: u32 = 0x50b05d6;
 loop { match pc {
 0x50b05d6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.fpu_control; cpu.w16(0x52374acu32, v); }
  { let v: u16 = cpu.r16(0x52374acu32); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp | 0xc00u32; cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.w16(0x52374aeu32, v); }
  { cpu.fpu_control = cpu.r16(0x52374aeu32); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx.wrapping_mul(2)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x52377f0u32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x52377f4u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x52377e4u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x52377e8u32, v); }
  { let v: u32 = cpu.shr32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x7bef7befu32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52377ecu32, v); }
  { let v = (cpu.r32(0x52377f4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x52377f8u32, v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x5142fa4u32), cpu.ebx); }
  if cpu.zf { pc = 0x50b0652; continue; }
  pc = 0x50b0638; continue;
 }
 0x50b0638 => {
  { cpu.sub32(cpu.ebx, 0x96u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b08c7; continue; }
  pc = 0x50b0644; continue;
 }
 0x50b0644 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5142fa4u32, v); }
  cpu.push(0x50b0651); f_050b055b(cpu); if cpu.returned_to != 0x50b0651 { return; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  pc = 0x50b0652; continue;
 }
 0x50b0652 => {
  { let v: u32 = 0x52374bcu32; cpu.edi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x52374b0u32, v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x50b0662; continue;
 }
 0x50b0662 => {
  { let v: u32 = 0x8u32.wrapping_add(cpu.esi); cpu.edx = v; }
  pc = 0x50b0665; continue;
 }
 0x50b0665 => {
  { let v: u32 = cpu.esi; cpu.ebx = v; }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x50b0675; continue; }
  pc = 0x50b0673; continue;
 }
 0x50b0673 => {
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x50b0675; continue;
 }
 0x50b0675 => {
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), 0x0u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b0741; continue; }
  pc = 0x50b067f; continue;
 }
 0x50b067f => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.rf32(0x52377f8u32); cpu.fcom(a, b); cpu.fpop();  }
  {  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  if (!cpu.cf && !cpu.zf) { pc = 0x50b0741; continue; }
  pc = 0x50b0692; continue;
 }
 0x50b0692 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.ebx)), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b06ec; continue; }
  pc = 0x50b06a7; continue;
 }
 0x50b06a7 => {
  { let v = cpu.rf32(0x5142f9cu32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x5142f9cu32); cpu.fpush(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x5142f9cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0741; continue; }
  pc = 0x50b06d4; continue;
 }
 0x50b06d4 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5142fa0u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5142fa0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x50b073d; continue;
 }
 0x50b06ec => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5142f9cu32); cpu.set_st(0, v); }
  { let v = cpu.round(cpu.st(0)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x5142f9cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5142f9cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x8u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x5142f9cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0xcu32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.edi))); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0741; continue; }
  pc = 0x50b0727; continue;
 }
 0x50b0727 => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5142fa0u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5142fa0u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x50b073d; continue;
 }
 0x50b073d => {
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  pc = 0x50b0741; continue;
 }
 0x50b0741 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b0662; continue; }
  pc = 0x50b074b; continue;
 }
 0x50b074b => {
  { let v: u32 = cpu.r32(0x52374b0u32); cpu.edx = v; }
  { let v: u32 = cpu.or32(cpu.ecx, cpu.ecx); cpu.ecx = v; }
  if cpu.zf { cpu.poll(); pc = 0x50b0665; continue; }
  pc = 0x50b0759; continue;
 }
 0x50b0759 => {
  { let v: u32 = cpu.or32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  if cpu.zf { pc = 0x50b08c7; continue; }
  pc = 0x50b0761; continue;
 }
 0x50b0761 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x52374b4u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x52374b8u32, v); }
  pc = 0x50b0771; continue;
 }
 0x50b0771 => {
  { let v: u32 = 0x52374bcu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x52374b4u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x52374b8u32); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b0783; continue;
 }
 0x50b0783 => {
  { cpu.sub32(cpu.r32(0x8u32.wrapping_add(cpu.esi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b079c; continue; }
  pc = 0x50b0788; continue;
 }
 0x50b0788 => {
  { cpu.sub32(cpu.r32(0xcu32.wrapping_add(cpu.esi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b079c; continue; }
  pc = 0x50b078d; continue;
 }
 0x50b078d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x523773cu32.wrapping_add(cpu.ecx.wrapping_mul(4)), v); }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)).wrapping_add(cpu.ebx); cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  pc = 0x50b079c; continue;
 }
 0x50b079c => {
  { let v: u32 = cpu.esi.wrapping_add(0x10u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b0783; continue; }
  pc = 0x50b07a2; continue;
 }
 0x50b07a2 => {
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b08a5; continue; }
  pc = 0x50b07ab; continue;
 }
 0x50b07ab => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x523773cu32; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  pc = 0x50b07b7; continue;
 }
 0x50b07b7 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  pc = 0x50b07bb; continue;
 }
 0x50b07bb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x0u32.wrapping_add(cpu.esi)), cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x50b07c9; continue; }
  pc = 0x50b07c4; continue;
 }
 0x50b07c4 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.esi)); let b = cpu.edx; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); } { let v: u32 = a; cpu.edx = v; } }
  { let v: u32 = cpu.edx; cpu.w32(0xfffffffcu32.wrapping_add(cpu.esi), v); }
  pc = 0x50b07c9; continue;
 }
 0x50b07c9 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b07bb; continue; }
  pc = 0x50b07cc; continue;
 }
 0x50b07cc => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b07b7; continue; }
  pc = 0x50b07cf; continue;
 }
 0x50b07cf => {
  { let v: u32 = cpu.r32(0x52374b8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5237804u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5237cb8u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x52377fcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5237800u32, v); }
  { let v: u32 = 0x523773cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x52377ecu32); cpu.ebp = v; }
  pc = 0x50b07f8; continue;
 }
 0x50b07f8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x8000u32); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(0x8000u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5237800u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b089c; continue; }
  pc = 0x50b081d; continue;
 }
 0x50b081d => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x52377fcu32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b089c; continue; }
  pc = 0x50b0825; continue;
 }
 0x50b0825 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x52377fcu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0832; continue; }
  pc = 0x50b082d; continue;
 }
 0x50b082d => {
  { let v: u32 = cpu.r32(0x52377fcu32); cpu.eax = v; }
  pc = 0x50b0832; continue;
 }
 0x50b0832 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5237800u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b0840; continue; }
  pc = 0x50b083a; continue;
 }
 0x50b083a => {
  { let v: u32 = cpu.r32(0x5237800u32); cpu.ebx = v; }
  pc = 0x50b0840; continue;
 }
 0x50b0840 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b089c; continue; }
  pc = 0x50b0845; continue;
 }
 0x50b0845 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52377e4u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b086c; continue; }
  pc = 0x50b0857; continue;
 }
 0x50b0857 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b089c; continue; }
  pc = 0x50b086c; continue;
 }
 0x50b086c => {
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.zf { pc = 0x50b0885; continue; }
  pc = 0x50b0872; continue;
 }
 0x50b0872 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b0872; continue; }
  pc = 0x50b0885; continue;
 }
 0x50b0885 => {
  { cpu.and32(cpu.edx, 0x1u32); }
  if cpu.zf { pc = 0x50b089c; continue; }
  pc = 0x50b088d; continue;
 }
 0x50b088d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebp); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b089c; continue;
 }
 0x50b089c => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b07f8; continue; }
  pc = 0x50b08a5; continue;
 }
 0x50b08a5 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52374b8u32)); cpu.w32(0x52374b8u32, v); }
  { let v: u32 = cpu.r32(0x52377e8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52377e4u32).wrapping_add(cpu.eax); cpu.w32(0x52377e4u32, v); }
  { let v: u32 = cpu.r32(0x52374b8u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x52377f4u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x50b0771; continue; }
  pc = 0x50b08c7; continue;
 }
 0x50b08c7 => {
  { cpu.fpu_control = cpu.r16(0x52374acu32); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b08d0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b08d0); }
 let mut pc: u32 = 0x50b08d0;
 loop { match pc {
 0x50b08d0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x524444cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244450u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244454u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5244458u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x524445cu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x524446cu32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b08f8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b08f8); }
 let mut pc: u32 = 0x50b08f8;
 loop { match pc {
 0x50b08f8 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244458u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244460u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244464u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244468u32, v); }
  { cpu.sub32(cpu.r32(0x5244458u32), 0x0u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0934; continue; }
  pc = 0x50b0928; continue;
 }
 0x50b0928 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x50b092e); f_050b3ca5(cpu); if cpu.returned_to != 0x50b092e { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244458u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  pc = 0x50b0934; continue;
 }
 0x50b0934 => {
  { let v: u32 = cpu.r32(0x5244460u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244464u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5244468u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x524444cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244450u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244454u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x524446cu32, v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b096a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b096a); }
 let mut pc: u32 = 0x50b096a;
 loop { match pc {
 0x50b096a => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x52445e4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x52445c0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x52445b8u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x52445b4u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52445bcu32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0a69; continue; }
  pc = 0x50b0995; continue;
 }
 0x50b0995 => {
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b099a; continue;
 }
 0x50b099a => {
  { let v: u32 = cpu.ecx; cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 1u32); cpu.edi = v; }
  if cpu.zf { pc = 0x50b09c0; continue; }
  pc = 0x50b09a2; continue;
 }
 0x50b09a2 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v = cpu.r32(0x52445b8u32); cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5244450u32)); cpu.eax = v; }
  if (cpu.sf != cpu.of) { pc = 0x50b09bc; continue; }
  pc = 0x50b09b8; continue;
 }
 0x50b09b8 => {
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  cpu.poll(); pc = 0x50b099a; continue;
 }
 0x50b09bc => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  cpu.poll(); pc = 0x50b099a; continue;
 }
 0x50b09c0 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52445c8u32, v); }
  { let v = cpu.r32(0x52445b8u32); cpu.imul32(v); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x52445c4u32, v); }
  pc = 0x50b09da; continue;
 }
 0x50b09da => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5244450u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b09e7; continue; }
  pc = 0x50b09e5; continue;
 }
 0x50b09e5 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b09e7; continue;
 }
 0x50b09e7 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x52445b4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0a11; continue; }
  pc = 0x50b09ef; continue;
 }
 0x50b09ef => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52445bcu32); cpu.ebx = v; }
  cpu.push(0x50b09fc); f_050b0b0c(cpu); if cpu.returned_to != 0x50b09fc { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0a08; continue; }
  pc = 0x50b0a02; continue;
 }
 0x50b0a02 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x52445ccu32, v); }
  pc = 0x50b0a08; continue;
 }
 0x50b0a08 => {
  { let v: u32 = cpu.esi.wrapping_sub(cpu.r32(0x52445b8u32)); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b09da; continue; }
  pc = 0x50b0a11; continue;
 }
 0x50b0a11 => {
  { let v: u32 = cpu.r32(0x52445c4u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x52445c8u32); cpu.ecx = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x52445b8u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x52445c0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b0a69; continue; }
  pc = 0x50b0a2c; continue;
 }
 0x50b0a2c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5244450u32)); cpu.eax = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0a39; continue; }
  pc = 0x50b0a37; continue;
 }
 0x50b0a37 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b0a39; continue;
 }
 0x50b0a39 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x52445b4u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0a69; continue; }
  pc = 0x50b0a41; continue;
 }
 0x50b0a41 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52445bcu32); cpu.ebx = v; }
  cpu.push(0x50b0a4e); f_050b0b0c(cpu); if cpu.returned_to != 0x50b0a4e { return; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0a5a; continue; }
  pc = 0x50b0a54; continue;
 }
 0x50b0a54 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x52445ccu32, v); }
  pc = 0x50b0a5a; continue;
 }
 0x50b0a5a => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x52445b8u32)); cpu.esi = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x52445c0u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b0a2c; continue; }
  pc = 0x50b0a69; continue;
 }
 0x50b0a69 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52445e4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x50b0a7c; continue; }
  pc = 0x50b0a74; continue;
 }
 0x50b0a74 => {
  { let v: u32 = cpu.r32(0x52445ccu32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b0a7c; continue;
 }
 0x50b0a7c => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0a7e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0a7e); }
 let mut pc: u32 = 0x50b0a7e;
 loop { match pc {
 0x50b0a7e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x52445e4u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b0ace; continue; }
  pc = 0x50b0a8e; continue;
 }
 0x50b0a8e => {
  { let v: u32 = cpu.edx; cpu.w32(0x52445d4u32, v); }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52445d0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52445d8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x52445dcu32, v); }
  pc = 0x50b0aa9; continue;
 }
 0x50b0aa9 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52445d8u32); cpu.ebx = v; }
  cpu.push(0x50b0ab5); f_050b0b0c(cpu); if cpu.returned_to != 0x50b0ab5 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0ac0; continue; }
  pc = 0x50b0aba; continue;
 }
 0x50b0aba => {
  { let v: u32 = cpu.esi; cpu.w32(0x52445e0u32, v); }
  pc = 0x50b0ac0; continue;
 }
 0x50b0ac0 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x52445d4u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x52445d0u32)); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b0aa9; continue; }
  pc = 0x50b0ace; continue;
 }
 0x50b0ace => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52445e4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x0u32); }
  if cpu.zf { pc = 0x50b0af3; continue; }
  pc = 0x50b0ad9; continue;
 }
 0x50b0ad9 => {
  { let v: u32 = cpu.r32(0x52445e0u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x52445dcu32)); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v = cpu.r32(0x52445d4u32); cpu.div32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x52445e4u32); cpu.eax = v; }
  pc = 0x50b0af3; continue;
 }
 0x50b0af3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0af5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0af5); }
 let mut pc: u32 = 0x50b0af5;
 loop { match pc {
 0x50b0af5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x0u32; cpu.w32(0x52445e4u32, v); }
  cpu.push(0x50b0b05); f_050b0b0c(cpu); if cpu.returned_to != 0x50b0b05 { return; }
  { let v: u32 = cpu.r32(0x52445e4u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0b0c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0b0c); }
 let mut pc: u32 = 0x50b0b0c;
 loop { match pc {
 0x50b0b0c => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5146e50u32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x50b0ca2; continue; }
  pc = 0x50b0b1e; continue;
 }
 0x50b0b1e => {
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebp)), 0x4u16); }
  if cpu.zf { pc = 0x50b0bb6; continue; }
  pc = 0x50b0b2a; continue;
 }
 0x50b0b2a => {
  { let v: u32 = cpu.r32(0x5244458u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244454u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x14u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0b46; continue;
 }
 0x50b0b46 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244454u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0b5c; continue;
 }
 0x50b0b5c => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244450u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0b72; continue;
 }
 0x50b0b72 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244450u32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x10u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0b88; continue;
 }
 0x50b0b88 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x524444cu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0b9e; continue;
 }
 0x50b0b9e => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x524444cu32); cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x8u32.wrapping_add(cpu.ebp))); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.edi); }
  if (cpu.sf != cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0bb4; continue;
 }
 0x50b0bb4 => {
  pc = 0x50b0c0b; continue;
 }
 0x50b0bb6 => {
  { let v: u32 = cpu.r32(0x5244458u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5244450u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x14u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0bd3; continue;
 }
 0x50b0bd3 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0bdb; continue;
 }
 0x50b0bdb => {
  { let v: u32 = cpu.r32(0x524444cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x10u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0beb; continue;
 }
 0x50b0beb => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0bf3; continue;
 }
 0x50b0bf3 => {
  { let v: u32 = cpu.r32(0x5244454u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0c03; continue;
 }
 0x50b0c03 => {
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x50b0ca2; continue; }
  pc = 0x50b0c0b; continue;
 }
 0x50b0c0b => {
  { let v: u32 = cpu.r32(0x5244458u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.esi))); cpu.ecx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5143aa8u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5143aacu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244440u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244470u32, v); }
  { cpu.sub32(cpu.r32(0x5244440u32), 0x0u32); }
  if cpu.zf { pc = 0x50b0ca8; continue; }
  pc = 0x50b0c35; continue;
 }
 0x50b0c35 => {
  { let v: u32 = cpu.r32(0x5143aacu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244480u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244484u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244488u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a8cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5143a90u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5143a94u32, v); }
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.esi = v; }
  { let v: u32 = 0x2au32.wrapping_add(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.esi = v; }
  pc = 0x50b0c71; continue;
 }
 0x50b0c71 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x50b0c8c; continue; }
  pc = 0x50b0c7b; continue;
 }
 0x50b0c7b => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x5143ab4u32.wrapping_add(cpu.ebx)); cpu.push(0x50b0c86); cpu.call(t); if cpu.returned_to != 0x50b0c86 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b0c71; continue;
 }
 0x50b0c8c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0ca2; continue; }
  pc = 0x50b0c92; continue;
 }
 0x50b0c92 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244444u32, v); }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  cpu.push(0x50b0ca1); f_050b0cb3(cpu); if cpu.returned_to != 0x50b0ca1 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b0ca2 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b0ca8 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52445e4u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0cb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0cb3); }
 let mut pc: u32 = 0x50b0cb3;
 loop { match pc {
 0x50b0cb3 => {
  { let v: u32 = cpu.r32(0x5244444u32); cpu.esi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52445ecu32, v); }
  cpu.push(0x50b0cc8); f_050b0cc9(cpu); if cpu.returned_to != 0x50b0cc8 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0cc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0cc9); }
 let mut pc: u32 = 0x50b0cc9;
 loop { match pc {
 0x50b0cc9 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x50b0cea; continue; }
  pc = 0x50b0cd3; continue;
 }
 0x50b0cd3 => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x50b0ce2); f_050b0cc9(cpu); if cpu.returned_to != 0x50b0ce2 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.push(0x50b0ce8); f_050b0cc9(cpu); if cpu.returned_to != 0x50b0ce8 { return; }
  pc = 0x50b0d0f; continue;
 }
 0x50b0cea => {
  { let v: u32 = cpu.eax; cpu.w32(0x52445e8u32, v); }
  pc = 0x50b0cef; continue;
 }
 0x50b0cef => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x50b0cf5); f_050b0d15(cpu); if cpu.returned_to != 0x50b0cf5 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0cff; continue; }
  pc = 0x50b0cfa; continue;
 }
 0x50b0cfa => {
  { let v: u32 = cpu.eax; cpu.w32(0x52445ecu32, v); }
  pc = 0x50b0cff; continue;
 }
 0x50b0cff => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52445e8u32)); cpu.w32(0x52445e8u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b0cef; continue; }
  pc = 0x50b0d0f; continue;
 }
 0x50b0d0f => {
  { let v: u32 = cpu.r32(0x52445ecu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0d15(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0d15); }
 let mut pc: u32 = 0x50b0d15;
 loop { match pc {
 0x50b0d15 => {
  { let v = cpu.esi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x5244440u32), 0x2u32); }
  if cpu.zf { pc = 0x50b0d70; continue; }
  pc = 0x50b0d1f; continue;
 }
 0x50b0d1f => {
  { let v: u32 = cpu.r32(0x524444cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244450u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5244454u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244474u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244478u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x524447cu32, v); }
  cpu.push(0x50b0d46); f_050b0f3f(cpu); if cpu.returned_to != 0x50b0d46 { return; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5244458u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0df3; continue; }
  pc = 0x50b0d52; continue;
 }
 0x50b0d52 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x524445cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b0df3; continue; }
  pc = 0x50b0d5e; continue;
 }
 0x50b0d5e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x50b0d66); f_050b0fc5(cpu); if cpu.returned_to != 0x50b0d66 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0df3; continue; }
  pc = 0x50b0d6e; continue;
 }
 0x50b0d6e => {
  pc = 0x50b0db6; continue;
 }
 0x50b0d70 => {
  { let v: u32 = cpu.r32(0x524444cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244450u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5244454u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244474u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244478u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x524447cu32, v); }
  cpu.push(0x50b0d97); f_050b0dfa(cpu); if cpu.returned_to != 0x50b0d97 { return; }
  if cpu.zf { pc = 0x50b0df3; continue; }
  pc = 0x50b0d99; continue;
 }
 0x50b0d99 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5244474u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244478u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x524447cu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  cpu.push(0x50b0db2); f_050b0fc5(cpu); if cpu.returned_to != 0x50b0db2 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b0df3; continue; }
  pc = 0x50b0db6; continue;
 }
 0x50b0db6 => {
  { let v: u32 = cpu.r32(0x524446cu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0xau32); }
  if (cpu.sf == cpu.of) { pc = 0x50b0de7; continue; }
  pc = 0x50b0dc0; continue;
 }
 0x50b0dc0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x5244498u32, v); }
  { let v: u32 = cpu.imul2_32(cpu.eax, 0x1cu32); cpu.eax = v; }
  { let v: u32 = 0x5244480u32; cpu.esi = v; }
  { let v: u32 = 0x524449cu32.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x7u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.inc32(cpu.r32(0x524446cu32)); cpu.w32(0x524446cu32, v); }
  pc = 0x50b0de7; continue;
 }
 0x50b0de7 => {
  { let v: u32 = 0x2u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52445e4u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b0df3 => {
  { let v: u32 = 0x0u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0dfa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0dfa); }
 let mut pc: u32 = 0x50b0dfa;
 loop { match pc {
 0x50b0dfa => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0e07; continue; }
  pc = 0x50b0e05; continue;
 }
 0x50b0e05 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b0e07; continue;
 }
 0x50b0e07 => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x526ab28u32; cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x50b0e29; continue; }
  pc = 0x50b0e23; continue;
 }
 0x50b0e23 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b0e29; continue;
 }
 0x50b0e29 => {
  { let v: u32 = cpu.eax; cpu.w32(0x524448cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244490u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244494u32, v); }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5244460u32); cpu.eax = v; }
  { let v = cpu.r32(0x524448cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5244464u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244490u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5244468u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244494u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.fpush(1.0); }
  { let v: u32 = cpu.eax; cpu.w32(0x52445fcu32, v); }
  { let v = (cpu.r32(0x52445fcu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5244474u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244480u32)); cpu.eax = v; }
  { let v = cpu.r32(0x524448cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5244478u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244484u32)); cpu.eax = v; }
  { let v = cpu.r32(0x5244490u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x524447cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244488u32)); cpu.eax = v; }
  { let v = cpu.r32(0x5244494u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52445fcu32, v); }
  { let v = (cpu.r32(0x52445fcu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x52445fcu32, v); }
  { let v: u32 = cpu.r32(0x52445fcu32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7fffffffu32; cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x3f800000u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0f3a; continue; }
  pc = 0x50b0ee7; continue;
 }
 0x50b0ee7 => {
  { let v = (cpu.r32(0x5244460u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5244474u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x52445f0u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5244464u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5244478u32); cpu.ebx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x52445f4u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5244468u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x524447cu32); cpu.ecx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x52445f8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x52445f0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x52445f4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x52445f8u32)); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.or32(cpu.edx, 0xffffffffu32); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b0f3a => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.edx, cpu.edx); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0f3f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0f3f); }
 let mut pc: u32 = 0x50b0f3f;
 loop { match pc {
 0x50b0f3f => {
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b0f4b; continue; }
  pc = 0x50b0f49; continue;
 }
 0x50b0f49 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b0f4b; continue;
 }
 0x50b0f4b => {
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x526ab28u32; cpu.edi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.edi)) as i16 as i32 as u32; cpu.ecx = v; }
  { cpu.and32(cpu.edx, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x50b0f6d; continue; }
  pc = 0x50b0f67; continue;
 }
 0x50b0f67 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b0f6d; continue;
 }
 0x50b0f6d => {
  { let v: u32 = cpu.eax; cpu.w32(0x524448cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244490u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244494u32, v); }
  { let v: u32 = cpu.r32(0x5244474u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244480u32)); cpu.eax = v; }
  { let v = cpu.r32(0x524448cu32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5244478u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244484u32)); cpu.eax = v; }
  { let v = cpu.r32(0x5244490u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x524447cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244488u32)); cpu.eax = v; }
  { let v = cpu.r32(0x5244494u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b0fc5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b0fc5); }
 let mut pc: u32 = 0x50b0fc5;
 loop { match pc {
 0x50b0fc5 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = 0x5244624u32; cpu.edi = v; }
  cpu.push(0x50b0fd0); f_050b1168(cpu); if cpu.returned_to != 0x50b0fd0 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5244474u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5244458u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x524460cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b1049; continue; }
  pc = 0x50b0fe4; continue;
 }
 0x50b0fe4 => {
  { let v: u32 = cpu.r32(0x5244474u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244458u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5244618u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1049; continue; }
  pc = 0x50b0ff7; continue;
 }
 0x50b0ff7 => {
  { let v: u32 = cpu.r32(0x5244478u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5244458u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5244610u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b1049; continue; }
  pc = 0x50b100a; continue;
 }
 0x50b100a => {
  { let v: u32 = cpu.r32(0x5244478u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244458u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x524461cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1049; continue; }
  pc = 0x50b101d; continue;
 }
 0x50b101d => {
  { let v: u32 = cpu.r32(0x524447cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5244458u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5244614u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b1049; continue; }
  pc = 0x50b1030; continue;
 }
 0x50b1030 => {
  { let v: u32 = cpu.r32(0x524447cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244458u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5244620u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1049; continue; }
  pc = 0x50b1043; continue;
 }
 0x50b1043 => {
  cpu.push(0x50b1048); f_050b104c(cpu); if cpu.returned_to != 0x50b1048 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b1049 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b104c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b104c); }
 let mut pc: u32 = 0x50b104c;
 loop { match pc {
 0x50b104c => {
  { let v: u32 = cpu.r32(0x524448cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5244490u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5244494u32); cpu.ecx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1063; continue; }
  pc = 0x50b1061; continue;
 }
 0x50b1061 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b1063; continue;
 }
 0x50b1063 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1069; continue; }
  pc = 0x50b1067; continue;
 }
 0x50b1067 => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x50b1069; continue;
 }
 0x50b1069 => {
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b106f; continue; }
  pc = 0x50b106d; continue;
 }
 0x50b106d => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b106f; continue;
 }
 0x50b106f => {
  { let v: u32 = 0x5244624u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.ecx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b10ce; continue; }
  pc = 0x50b107d; continue;
 }
 0x50b107d => {
  { cpu.sub32(cpu.ecx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x50b10ce; continue; }
  pc = 0x50b1081; continue;
 }
 0x50b1081 => {
  { let v: u32 = cpu.r32(0x5244494u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x50b108a; continue;
 }
 0x50b108a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244474u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x5244478u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244474u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x5244478u32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1161; continue; }
  pc = 0x50b10c3; continue;
 }
 0x50b10c3 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b108a; continue; }
  pc = 0x50b10c9; continue;
 }
 0x50b10c9 => {
  pc = 0x50b1164; continue;
 }
 0x50b10ce => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b111a; continue; }
  pc = 0x50b10d2; continue;
 }
 0x50b10d2 => {
  { let v: u32 = cpu.r32(0x5244490u32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  { let v: u32 = !cpu.edi; cpu.edi = v; }
  pc = 0x50b10dd; continue;
 }
 0x50b10dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244474u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x524447cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244474u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x524447cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1161; continue; }
  pc = 0x50b1112; continue;
 }
 0x50b1112 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b10dd; continue; }
  pc = 0x50b1118; continue;
 }
 0x50b1118 => {
  pc = 0x50b1164; continue;
 }
 0x50b111a => {
  { let v: u32 = cpu.r32(0x524448cu32); cpu.edi = v; }
  { let v: u32 = cpu.sar32(cpu.edi, 31u32); cpu.edi = v; }
  pc = 0x50b1123; continue;
 }
 0x50b1123 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244478u32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x524447cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5244478u32)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.r32(0x524447cu32)); cpu.edx = v; }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.xor32(cpu.edx, cpu.edi); cpu.edx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1161; continue; }
  pc = 0x50b1159; continue;
 }
 0x50b1159 => {
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b1123; continue; }
  pc = 0x50b115f; continue;
 }
 0x50b115f => {
  pc = 0x50b1164; continue;
 }
 0x50b1161 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b1164 => {
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1168(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1168); }
 let mut pc: u32 = 0x50b1168;
 loop { match pc {
 0x50b1168 => {
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0xcu32.wrapping_add(cpu.edx)); cpu.ebp = (cpu.ebp & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xeu32); cpu.edx = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x524460cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5244610u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5244614u32, v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244618u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x524461cu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5244620u32, v); }
  { let v = cpu.edi; cpu.push(v); }
  pc = 0x50b119e; continue;
 }
 0x50b119e => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = 0x5265d00u32.wrapping_add(cpu.eax.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x524460cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b11c5; continue; }
  pc = 0x50b11c0; continue;
 }
 0x50b11c0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x524460cu32, v); }
  pc = 0x50b11c5; continue;
 }
 0x50b11c5 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5244610u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b11d3; continue; }
  pc = 0x50b11cd; continue;
 }
 0x50b11cd => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5244610u32, v); }
  pc = 0x50b11d3; continue;
 }
 0x50b11d3 => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x5244614u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b11e1; continue; }
  pc = 0x50b11db; continue;
 }
 0x50b11db => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5244614u32, v); }
  pc = 0x50b11e1; continue;
 }
 0x50b11e1 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5244618u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b11ee; continue; }
  pc = 0x50b11e9; continue;
 }
 0x50b11e9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5244618u32, v); }
  pc = 0x50b11ee; continue;
 }
 0x50b11ee => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x524461cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b11fc; continue; }
  pc = 0x50b11f6; continue;
 }
 0x50b11f6 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x524461cu32, v); }
  pc = 0x50b11fc; continue;
 }
 0x50b11fc => {
  { cpu.sub32(cpu.ecx, cpu.r32(0x5244620u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b120a; continue; }
  pc = 0x50b1204; continue;
 }
 0x50b1204 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5244620u32, v); }
  pc = 0x50b120a; continue;
 }
 0x50b120a => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0xcu32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b119e; continue; }
  pc = 0x50b1218; continue;
 }
 0x50b1218 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1223(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1223); }
 let mut pc: u32 = 0x50b1223;
 loop { match pc {
 0x50b1223 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = 0x524449cu32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x524446cu32); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if cpu.zf { pc = 0x50b126d; continue; }
  pc = 0x50b1233; continue;
 }
 0x50b1233 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x18u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi.wrapping_add(0x1cu32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b1233; continue; }
  pc = 0x50b126d; continue;
 }
 0x50b126d => {
  { let v: u32 = cpu.r32(0x524446cu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1274(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1274); }
 let mut pc: u32 = 0x50b1274;
 loop { match pc {
 0x50b1274 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0xcu32.wrapping_add(cpu.ebx))); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x8u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b12c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b12c6); }
 let mut pc: u32 = 0x50b12c6;
 loop { match pc {
 0x50b12c6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5244754u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5244758u32, v); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.edi); cpu.edi = v; }
  { let v = cpu.esi; cpu.push(v); }
  pc = 0x50b12d8; continue;
 }
 0x50b12d8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  pc = 0x50b12e3; continue;
 }
 0x50b12e3 => {
  { cpu.sub32(cpu.ebx, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b12ea; continue; }
  pc = 0x50b12e7; continue;
 }
 0x50b12e7 => {
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.edx; let b = cpu.ebx; { let v: u32 = b; cpu.edx = v; } { let v: u32 = a; cpu.ebx = v; } }
  pc = 0x50b12ea; continue;
 }
 0x50b12ea => {
  { let v: u32 = cpu.sub32(cpu.ebx, cpu.r32(0x5244758u32)); cpu.ebx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1322; continue; }
  pc = 0x50b12f2; continue;
 }
 0x50b12f2 => {
  { let v: u32 = cpu.sub32(cpu.edx, cpu.r32(0x5244758u32)); cpu.edx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50b1322; continue; }
  pc = 0x50b12fa; continue;
 }
 0x50b12fa => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.r32(0x5244754u32)); cpu.eax = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b130c; continue; }
  pc = 0x50b1302; continue;
 }
 0x50b1302 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x5244754u32)); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1314; continue; }
  pc = 0x50b130a; continue;
 }
 0x50b130a => {
  pc = 0x50b1322; continue;
 }
 0x50b130c => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.r32(0x5244754u32)); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1321; continue; }
  pc = 0x50b1314; continue;
 }
 0x50b1314 => {
  { let v = cpu.edx; cpu.imul32(v); }
  { let a = cpu.ecx; let b = cpu.eax; { let v: u32 = b; cpu.ecx = v; } { let v: u32 = a; cpu.eax = v; } }
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  { let v = cpu.edx; cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.sbb32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  if (cpu.sf != cpu.of) { pc = 0x50b1322; continue; }
  pc = 0x50b1321; continue;
 }
 0x50b1321 => {
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  pc = 0x50b1322; continue;
 }
 0x50b1322 => {
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b12d8; continue; }
  pc = 0x50b1328; continue;
 }
 0x50b1328 => {
  if (cpu.sf != cpu.of) { pc = 0x50b1337; continue; }
  pc = 0x50b132a; continue;
 }
 0x50b132a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  cpu.poll(); pc = 0x50b12e3; continue;
 }
 0x50b1337 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x1u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1340(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1340); }
 let mut pc: u32 = 0x50b1340;
 loop { match pc {
 0x50b1340 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5144840u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x527ade0u32, v); }
  { cpu.sub32(cpu.r32(0x5144860u32), cpu.eax); }
  if !cpu.zf { pc = 0x50b1372; continue; }
  pc = 0x50b1355; continue;
 }
 0x50b1355 => {
  { cpu.sub32(cpu.r32(0x5144864u32), cpu.ebx); }
  if !cpu.zf { pc = 0x50b1372; continue; }
  pc = 0x50b135d; continue;
 }
 0x50b135d => {
  { cpu.sub32(cpu.r32(0x5144844u32), cpu.ecx); }
  if !cpu.zf { pc = 0x50b1372; continue; }
  pc = 0x50b1365; continue;
 }
 0x50b1365 => {
  { cpu.sub32(cpu.r32(0x5144870u32), cpu.edi); }
  if !cpu.zf { pc = 0x50b1372; continue; }
  pc = 0x50b136d; continue;
 }
 0x50b136d => {
  pc = 0x50b14de; continue;
 }
 0x50b1372 => {
  { cpu.sub32(cpu.edi, 0x40u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b137f; continue; }
  pc = 0x50b1377; continue;
 }
 0x50b1377 => {
  { cpu.sub32(cpu.edi, 0x400u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1384; continue; }
  pc = 0x50b137f; continue;
 }
 0x50b137f => {
  { let v: u32 = 0x40u32; cpu.edi = v; }
  pc = 0x50b1384; continue;
 }
 0x50b1384 => {
  cpu.push(0x50b1389); f_050b155f(cpu); if cpu.returned_to != 0x50b1389 { return; }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x5143a40u32); }
  { let v: u32 = cpu.edi; cpu.w32(0x5144870u32, v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5144874u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x5144874u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5144878u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x5144884u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5144888u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x5144874u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x5144884u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x514487cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x5144860u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5144864u32, v); }
  { let v = (cpu.r32(0x5144860u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5144868u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5144864u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x514486cu32, v); cpu.fpop(); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5144844u32, v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5144848u32, v); }
  { let v: u32 = 0xffffffffu32.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x514484cu32, v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5144850u32, v); }
  { let v: u32 = cpu.ebx; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 1u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5144854u32, v); }
  { let v = (cpu.r32(0x5144850u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5144858u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5144854u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x514485cu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5144850u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x5143a38u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x51448a4u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x51448a4u32)); cpu.w32(0x51448a4u32, v); }
  { let v = (cpu.r32(0x5144854u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0).sqrt(); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x5143a38u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x51448a8u32, v); cpu.fpop(); }
  { let v: u32 = cpu.inc32(cpu.r32(0x51448a8u32)); cpu.w32(0x51448a8u32, v); }
  { let v = (cpu.r32(0x5144850u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x5143a38u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x51448acu32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5144854u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x5144870u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(1) / cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf64(0x5143a38u32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x51448b0u32, v); cpu.fpop(); }
  { let v: u32 = 0x7fffffffu32; cpu.eax = v; }
  cpu.push(0x50b14d5); f_050b1509(cpu); if cpu.returned_to != 0x50b14d5 { return; }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x5143a42u32); }
  pc = 0x50b14de; continue;
 }
 0x50b14de => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b14e0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b14e0); }
 let mut pc: u32 = 0x50b14e0;
 loop { match pc {
 0x50b14e0 => {
  { cpu.pushad(); }
  cpu.push(0x50b14e6); f_050b9f04(cpu); if cpu.returned_to != 0x50b14e6 { return; }
  cpu.push(0x50b14eb); f_050b89ce(cpu); if cpu.returned_to != 0x50b14eb { return; }
  cpu.push(0x50b14f0); f_050b9f8a(cpu); if cpu.returned_to != 0x50b14f0 { return; }
  cpu.push(0x50b14f5); f_050b155f(cpu); if cpu.returned_to != 0x50b14f5 { return; }
  { cpu.popad(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b14f7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b14f7); }
 let mut pc: u32 = 0x50b14f7;
 loop { match pc {
 0x50b14f7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x51448b4u32, v); }
  { let v = (cpu.r32(0x51448b4u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x51448b8u32, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1509(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1509); }
 let mut pc: u32 = 0x50b1509;
 loop { match pc {
 0x50b1509 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5144880u32, v); }
  { let v = (cpu.r32(0x5144880u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5144884u32, v); cpu.fpop(); }
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.rf32(0x5144884u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5144888u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x5144874u32); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x5144884u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x514487cu32, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b153b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b153b); }
 let mut pc: u32 = 0x50b153b;
 loop { match pc {
 0x50b153b => {
  { let v: u32 = cpu.eax; cpu.w32(0x5144898u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x514489cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x51448a0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x514488cu32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5144890u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x5144894u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b155f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b155f); }
 let mut pc: u32 = 0x50b155f;
 loop { match pc {
 0x50b155f => {
  { let v = cpu.eax; cpu.push(v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { let v = cpu.fpu_control; cpu.w16(0x5143a40u32, v); }
  { let v: u16 = cpu.r16(0x5143a40u32); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xfcffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0x0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xf3ffu16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) | 0xc00u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16) & 0xffc0u16; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.or16((cpu.eax as u16), 0x3fu16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x5143a42u32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b158f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b158f); }
 let mut pc: u32 = 0x50b158f;
 loop { match pc {
 0x50b158f => {
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.r32(0x52ae8a8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = cpu.r32(0x52ae8acu32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x52ae8b0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x0u32.wrapping_add(cpu.ecx), v); }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b15aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b15aa); }
 let mut pc: u32 = 0x50b15aa;
 loop { match pc {
 0x50b15aa => {
  { let v: u32 = cpu.r32(0x527addcu32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b15c0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b15c0); }
 let mut pc: u32 = 0x50b15c0;
 loop { match pc {
 0x50b15c0 => {
  cpu.push(0x50b15c5); f_050b8a87(cpu); if cpu.returned_to != 0x50b15c5 { return; }
  { let v: u32 = 0x5293614u32; cpu.w32(0x5293610u32, v); }
  { let v: u32 = 0x52a2f50u32; cpu.w32(0x52a2f4cu32, v); }
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x5143a42u32); }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8a8u32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8acu32, v); }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.inc32(cpu.r32(0x51448ccu32)); cpu.w32(0x51448ccu32, v); }
  { let v: u32 = 0x5143cf8u32; cpu.eax = v; }
  { let v: u32 = 0x5143d1cu32; cpu.ebx = v; }
  cpu.push(0x50b1615); f_050b3f0b(cpu); if cpu.returned_to != 0x50b1615 { return; }
  { let v: u32 = cpu.r32(0x5143cfcu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143d08u32); cpu.ebx = v; }
  cpu.push(0x50b1625); f_050b3cff(cpu); if cpu.returned_to != 0x50b1625 { return; }
  { let v: u32 = 0x5143cf8u32; cpu.ebx = v; }
  { let v: u32 = 0x5143dacu32; cpu.edi = v; }
  cpu.push(0x50b1634); f_050b3b0f(cpu); if cpu.returned_to != 0x50b1634 { return; }
  { let v: u32 = cpu.r32(0x5143d10u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143d18u32); cpu.ebx = v; }
  cpu.push(0x50b1644); f_050b3cff(cpu); if cpu.returned_to != 0x50b1644 { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x5143cf8u32; cpu.ebx = v; }
  { let v: u32 = 0x5143df4u32; cpu.edi = v; }
  cpu.push(0x50b1655); f_050b3a34(cpu); if cpu.returned_to != 0x50b1655 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1656(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1656); }
 let mut pc: u32 = 0x50b1656;
 loop { match pc {
 0x50b1656 => {
  {  }
  { cpu.top = 0; cpu.fpu_control = 0x37f; }
  { cpu.fpu_control = cpu.r16(0x5143a40u32); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1660(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1660); }
 let mut pc: u32 = 0x50b1660;
 loop { match pc {
 0x50b1660 => {
  { let v: u32 = 0x52448b8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a58u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a5cu32, v); }
  { let v: u32 = 0x52467f8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  { let v: u32 = 0x52542b8u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a60u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a64u32, v); }
  { let v: u32 = 0x52561f8u32; cpu.w32(0x0u32.wrapping_add(cpu.eax), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b168b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b168b); }
 let mut pc: u32 = 0x50b168b;
 loop { match pc {
 0x50b168b => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5143a58u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a5cu32); cpu.ebx = v; }
  cpu.push(0x50b169c); f_050b1b3c(cpu); if cpu.returned_to != 0x50b169c { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b169e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b169e); }
 let mut pc: u32 = 0x50b169e;
 loop { match pc {
 0x50b169e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5143a60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.ebx = v; }
  cpu.push(0x50b16af); f_050b1a70(cpu); if cpu.returned_to != 0x50b16af { return; }
  { let v: u32 = cpu.r32(0x5143a60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.ebx = v; }
  cpu.push(0x50b16bf); f_050b1b3c(cpu); if cpu.returned_to != 0x50b16bf { return; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b16c1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b16c1); }
 let mut pc: u32 = 0x50b16c1;
 loop { match pc {
 0x50b16c1 => {
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b16e9; continue; }
  pc = 0x50b16c5; continue;
 }
 0x50b16c5 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5244880u32, v); }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5244884u32, v); }
  pc = 0x50b16d4; continue;
 }
 0x50b16d4 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x50b16da); f_050b16ea(cpu); if cpu.returned_to != 0x50b16da { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x5244880u32)); cpu.esi = v; }
  { cpu.sub32(cpu.esi, cpu.r32(0x5244884u32)); }
  if !cpu.zf { cpu.poll(); pc = 0x50b16d4; continue; }
  pc = 0x50b16e9; continue;
 }
 0x50b16e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b16ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b16ea); }
 let mut pc: u32 = 0x50b16ea;
 loop { match pc {
 0x50b16ea => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5146e50u32.wrapping_add(cpu.edx.wrapping_mul(4))); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5143aa8u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5143ce0u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x5143ce4u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x5143ce8u32)); cpu.ecx = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x5144898u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1727; continue;
 }
 0x50b1727 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x514489cu32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1736; continue;
 }
 0x50b1736 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x51448a0u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1745; continue;
 }
 0x50b1745 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x514488cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1756; continue;
 }
 0x50b1756 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x5144890u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1765; continue;
 }
 0x50b1765 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x5144894u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1774; continue;
 }
 0x50b1774 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5244888u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x524488cu32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5244890u32, v); }
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.edx)), 0x1u16); }
  if cpu.zf { pc = 0x50b1799; continue; }
  pc = 0x50b1793; continue;
 }
 0x50b1793 => {
  { let v: u32 = cpu.r32(0x5143a5cu32); cpu.ebx = v; }
  pc = 0x50b1799; continue;
 }
 0x50b1799 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143cfcu32); cpu.eax = v; }
  { let v = cpu.r32(0x5244888u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d08u32); cpu.eax = v; }
  { let v = cpu.r32(0x524488cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244890u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x5144880u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b17e6; continue;
 }
 0x50b17e6 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x5144870u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b17f7; continue;
 }
 0x50b17f7 => {
  { let v: u8 = 0x0u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5143cf8u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244888u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d04u32); cpu.eax = v; }
  { let v = cpu.r32(0x524488cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d10u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244890u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x51448acu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1845; continue; }
  pc = 0x50b1843; continue;
 }
 0x50b1843 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x50b1845; continue;
 }
 0x50b1845 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x51448a4u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b185b; continue;
 }
 0x50b185b => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x50b189d; continue; }
  pc = 0x50b1863; continue;
 }
 0x50b1863 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5143d00u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244888u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d0cu32); cpu.eax = v; }
  { let v = cpu.r32(0x524488cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d18u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244890u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b190b; continue;
 }
 0x50b189d => {
  { let v: u32 = cpu.r32(0x5143d00u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244888u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d0cu32); cpu.eax = v; }
  { let v = cpu.r32(0x524488cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x5143d18u32); cpu.eax = v; }
  { let v = cpu.r32(0x5244890u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x51448b0u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b18e9; continue; }
  pc = 0x50b18e7; continue;
 }
 0x50b18e7 => {
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x50b18e9; continue;
 }
 0x50b18e9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x51448a8u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b18ff; continue;
 }
 0x50b18ff => {
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x50b190b; continue; }
  pc = 0x50b1907; continue;
 }
 0x50b1907 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x1u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b190b; continue;
 }
 0x50b190b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x5144880u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1925; continue; }
  pc = 0x50b1918; continue;
 }
 0x50b1918 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ebx))); cpu.ebp = v; }
  { cpu.sub32(cpu.ebp, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b1929; continue; }
  pc = 0x50b1925; continue;
 }
 0x50b1925 => {
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.edi)) | 0x2u8; cpu.w8(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b1929; continue;
 }
 0x50b1929 => {
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x4u32.wrapping_add(cpu.edi))); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.esi; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x1cu32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.ebx = v; }
  { cpu.and16(cpu.r16(0x20u32.wrapping_add(cpu.ebx)), 0x1u16); }
  if cpu.zf { pc = 0x50b1980; continue; }
  pc = 0x50b1965; continue;
 }
 0x50b1965 => {
  { let v: u32 = cpu.r32(0x5143a5cu32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x52467f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1976; continue;
 }
 0x50b1976 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5143a5cu32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50b1999; continue;
 }
 0x50b1980 => {
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, 0x52561f8u32); }
  if (cpu.sf == cpu.of) { pc = 0x50b19a0; continue; }
  pc = 0x50b1991; continue;
 }
 0x50b1991 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5143a64u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  pc = 0x50b1999; continue;
 }
 0x50b1999 => {
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b19a0 => {
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b19a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b19a4); } r_050b19a4(cpu, 0x50b19a4); }
pub fn f_050b19ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b19ad); } r_050b19a4(cpu, 0x50b19ad); }
fn r_050b19a4(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b19a4 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1a1e; continue; }
  pc = 0x50b19ad; continue;
 }
 0x50b19ad => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1a1f; continue; }
  pc = 0x50b19b6; continue;
 }
 0x50b19b6 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x50b19c5; continue;
 }
 0x50b19c5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b19dd; continue; }
  pc = 0x50b19cc; continue;
 }
 0x50b19cc => {
  if cpu.zf { pc = 0x50b19d3; continue; }
  pc = 0x50b19ce; continue;
 }
 0x50b19ce => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x50b19c5; continue;
 }
 0x50b19d3 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (cpu.cf || cpu.zf) { pc = 0x50b19dd; continue; }
  pc = 0x50b19d8; continue;
 }
 0x50b19d8 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x50b19c5; continue;
 }
 0x50b19dd => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (cpu.sf != cpu.of) { pc = 0x50b19f5; continue; }
  pc = 0x50b19e4; continue;
 }
 0x50b19e4 => {
  if cpu.zf { pc = 0x50b19eb; continue; }
  pc = 0x50b19e6; continue;
 }
 0x50b19e6 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b19dd; continue;
 }
 0x50b19eb => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if (cpu.cf || cpu.zf) { pc = 0x50b19f5; continue; }
  pc = 0x50b19f0; continue;
 }
 0x50b19f0 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b19dd; continue;
 }
 0x50b19f5 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1a03; continue; }
  pc = 0x50b19f9; continue;
 }
 0x50b19f9 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x50b1a03; continue;
 }
 0x50b1a03 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b19c5; continue; }
  pc = 0x50b1a07; continue;
 }
 0x50b1a07 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x50b1a14; continue; }
  pc = 0x50b1a0b; continue;
 }
 0x50b1a0b => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x50b1a13); f_050b19ad(cpu); if cpu.returned_to != 0x50b1a13 { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x50b1a14; continue;
 }
 0x50b1a14 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x50b1a1e; continue; }
  pc = 0x50b1a19; continue;
 }
 0x50b1a19 => {
  cpu.push(0x50b1a1e); f_050b19ad(cpu); if cpu.returned_to != 0x50b1a1e { return; }
  pc = 0x50b1a1e; continue;
 }
 0x50b1a1e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b1a1f => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x50b1a21; continue;
 }
 0x50b1a21 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1a46; continue; }
  pc = 0x50b1a33; continue;
 }
 0x50b1a33 => {
  if cpu.zf { pc = 0x50b1a3b; continue; }
  pc = 0x50b1a35; continue;
 }
 0x50b1a35 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1a21; continue; }
  pc = 0x50b1a39; continue;
 }
 0x50b1a39 => {
  pc = 0x50b1a6f; continue;
 }
 0x50b1a3b => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { pc = 0x50b1a46; continue; }
  pc = 0x50b1a40; continue;
 }
 0x50b1a40 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1a21; continue; }
  pc = 0x50b1a44; continue;
 }
 0x50b1a44 => {
  pc = 0x50b1a6f; continue;
 }
 0x50b1a46 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x50b1a4f; continue;
 }
 0x50b1a4f => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x50b1a67; continue; }
  pc = 0x50b1a56; continue;
 }
 0x50b1a56 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b1a4f; continue; }
  pc = 0x50b1a60; continue;
 }
 0x50b1a60 => {
  if !cpu.zf { pc = 0x50b1a67; continue; }
  pc = 0x50b1a62; continue;
 }
 0x50b1a62 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { cpu.poll(); pc = 0x50b1a4f; continue; }
  pc = 0x50b1a67; continue;
 }
 0x50b1a67 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1a21; continue; }
  pc = 0x50b1a6f; continue;
 }
 0x50b1a6f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1a70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b1a70); } r_050b1a70(cpu, 0x50b1a70); }
pub fn f_050b1a79(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b1a79); } r_050b1a70(cpu, 0x50b1a79); }
fn r_050b1a70(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b1a70 => {
  { let v: u32 = 0xfffffffcu32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { cpu.sub32(cpu.esi, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1aea; continue; }
  pc = 0x50b1a79; continue;
 }
 0x50b1a79 => {
  { let v: u32 = cpu.esi; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x24u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1aeb; continue; }
  pc = 0x50b1a82; continue;
 }
 0x50b1a82 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x50b1a91; continue;
 }
 0x50b1a91 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1aa9; continue; }
  pc = 0x50b1a98; continue;
 }
 0x50b1a98 => {
  if cpu.zf { pc = 0x50b1a9f; continue; }
  pc = 0x50b1a9a; continue;
 }
 0x50b1a9a => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x50b1a91; continue;
 }
 0x50b1a9f => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if !cpu.cf { pc = 0x50b1aa9; continue; }
  pc = 0x50b1aa4; continue;
 }
 0x50b1aa4 => {
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  cpu.poll(); pc = 0x50b1a91; continue;
 }
 0x50b1aa9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x14u32.wrapping_add(cpu.edi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1ac1; continue; }
  pc = 0x50b1ab0; continue;
 }
 0x50b1ab0 => {
  if cpu.zf { pc = 0x50b1ab7; continue; }
  pc = 0x50b1ab2; continue;
 }
 0x50b1ab2 => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b1aa9; continue;
 }
 0x50b1ab7 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x18u32.wrapping_add(cpu.edi))); }
  if !cpu.cf { pc = 0x50b1ac1; continue; }
  pc = 0x50b1abc; continue;
 }
 0x50b1abc => {
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b1aa9; continue;
 }
 0x50b1ac1 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1acf; continue; }
  pc = 0x50b1ac5; continue;
 }
 0x50b1ac5 => {
  { let a = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); let b = cpu.edi; { let v: u32 = b; cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); } { let v: u32 = a; cpu.edi = v; } }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_sub(0x4u32); cpu.esi = v; }
  pc = 0x50b1acf; continue;
 }
 0x50b1acf => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b1a91; continue; }
  pc = 0x50b1ad3; continue;
 }
 0x50b1ad3 => {
  { cpu.sub32(cpu.ecx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x50b1ae0; continue; }
  pc = 0x50b1ad7; continue;
 }
 0x50b1ad7 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.ecx; cpu.ebx = v; }
  cpu.push(0x50b1adf); f_050b1a79(cpu); if cpu.returned_to != 0x50b1adf { return; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  pc = 0x50b1ae0; continue;
 }
 0x50b1ae0 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x50b1aea; continue; }
  pc = 0x50b1ae5; continue;
 }
 0x50b1ae5 => {
  cpu.push(0x50b1aea); f_050b1a79(cpu); if cpu.returned_to != 0x50b1aea { return; }
  pc = 0x50b1aea; continue;
 }
 0x50b1aea => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b1aeb => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  pc = 0x50b1aed; continue;
 }
 0x50b1aed => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(0x4u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b1b12; continue; }
  pc = 0x50b1aff; continue;
 }
 0x50b1aff => {
  if cpu.zf { pc = 0x50b1b07; continue; }
  pc = 0x50b1b01; continue;
 }
 0x50b1b01 => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1aed; continue; }
  pc = 0x50b1b05; continue;
 }
 0x50b1b05 => {
  pc = 0x50b1b3b; continue;
 }
 0x50b1b07 => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if cpu.cf { pc = 0x50b1b12; continue; }
  pc = 0x50b1b0c; continue;
 }
 0x50b1b0c => {
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1aed; continue; }
  pc = 0x50b1b10; continue;
 }
 0x50b1b10 => {
  pc = 0x50b1b3b; continue;
 }
 0x50b1b12 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  pc = 0x50b1b1b; continue;
 }
 0x50b1b1b => {
  { let v: u32 = cpu.ebx.wrapping_sub(0x4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.ecx); }
  if (cpu.sf != cpu.of) { pc = 0x50b1b33; continue; }
  pc = 0x50b1b22; continue;
 }
 0x50b1b22 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.edi)), cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b1b1b; continue; }
  pc = 0x50b1b2c; continue;
 }
 0x50b1b2c => {
  if !cpu.zf { pc = 0x50b1b33; continue; }
  pc = 0x50b1b2e; continue;
 }
 0x50b1b2e => {
  { cpu.sub32(cpu.r32(0x18u32.wrapping_add(cpu.edi)), cpu.edx); }
  if (!cpu.cf && !cpu.zf) { cpu.poll(); pc = 0x50b1b1b; continue; }
  pc = 0x50b1b33; continue;
 }
 0x50b1b33 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x4u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.esi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1aed; continue; }
  pc = 0x50b1b3b; continue;
 }
 0x50b1b3b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1b3c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1b3c); }
 let mut pc: u32 = 0x50b1b3c;
 loop { match pc {
 0x50b1b3c => {
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if cpu.zf { pc = 0x50b1c06; continue; }
  pc = 0x50b1b44; continue;
 }
 0x50b1b44 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v: u8 = cpu.r8(0x10u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = (cpu.eax as u8); cpu.w8(0x5143a98u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a8cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5143a90u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5143a94u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a9cu32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5143aa0u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x5143aacu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5146e50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b1bf9; continue; }
  pc = 0x50b1b91; continue;
 }
 0x50b1b91 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5143aa8u32, v); }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u8 = cpu.r8(0x5143a98u32); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.r32(0x5143b54u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a54u32, v); }
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.ebx = v; }
  { let v: u32 = 0x52855f8u32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x5dc0u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1bd0; continue; }
  pc = 0x50b1bbe; continue;
 }
 0x50b1bbe => {
  { let v: u32 = 0x52895fcu32; cpu.eax = v; }
  { cpu.sub32(cpu.ebx, 0x13880u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b1bd0; continue; }
  pc = 0x50b1bcb; continue;
 }
 0x50b1bcb => {
  { let v: u32 = 0x528d600u32; cpu.eax = v; }
  pc = 0x50b1bd0; continue;
 }
 0x50b1bd0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x52855f4u32, v); }
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x32u32); cpu.esi = v; }
  pc = 0x50b1bde; continue;
 }
 0x50b1bde => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x50b1bf9; continue; }
  pc = 0x50b1be8; continue;
 }
 0x50b1be8 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x5143ab4u32.wrapping_add(cpu.ebx)); cpu.push(0x50b1bf3); cpu.call(t); if cpu.returned_to != 0x50b1bf3 { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b1bde; continue;
 }
 0x50b1bf9 => {
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1b44; continue; }
  pc = 0x50b1c06; continue;
 }
 0x50b1c06 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1c1b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1c1b); }
 let mut pc: u32 = 0x50b1c1b;
 loop { match pc {
 0x50b1c1b => {
  { let v: u32 = cpu.add32(cpu.r32(0x5143a54u32), 0xb0u32); cpu.w32(0x5143a54u32, v); }
  { let v: u32 = 0x1u32; cpu.w32(0x5143ab0u32, v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1c30(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1c30); }
 let mut pc: u32 = 0x50b1c30;
 loop { match pc {
 0x50b1c30 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x527ade4u32)); cpu.w32(0x527ade4u32, v); }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  f_050b1c38(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1c38(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1c38); }
 let mut pc: u32 = 0x50b1c38;
 loop { match pc {
 0x50b1c38 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x2u32); cpu.edx = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b1c49; continue; }
  pc = 0x50b1c42; continue;
 }
 0x50b1c42 => {
  cpu.push(0x50b1c47); f_050b1c93(cpu); if cpu.returned_to != 0x50b1c47 { return; }
  pc = 0x50b1c4e; continue;
 }
 0x50b1c49 => {
  cpu.push(0x50b1c4e); f_050b1c4f(cpu); if cpu.returned_to != 0x50b1c4e { return; }
  pc = 0x50b1c4e; continue;
 }
 0x50b1c4e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1c4f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1c4f); }
 let mut pc: u32 = 0x50b1c4f;
 loop { match pc {
 0x50b1c4f => {
  { let v: u32 = cpu.r16(0x6u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf == cpu.of) { pc = 0x50b1c6f; continue; }
  pc = 0x50b1c5c; continue;
 }
 0x50b1c5c => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a6cu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5269b80u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x50b1c7c; continue;
 }
 0x50b1c6f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a6cu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5269b80u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x50b1c7c; continue;
 }
 0x50b1c7c => {
  { let v: u32 = cpu.edx.wrapping_add(0x8u32); cpu.edx = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0xfffffffau32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1c86; continue; }
  pc = 0x50b1c84; continue;
 }
 0x50b1c84 => {
  { let a = cpu.ebx; let b = cpu.edx; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.edx = v; } }
  pc = 0x50b1c86; continue;
 }
 0x50b1c86 => {
  { let v = cpu.ebx; cpu.push(v); }
  cpu.push(0x50b1c8c); f_050b1c38(cpu); if cpu.returned_to != 0x50b1c8c { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  cpu.push(0x50b1c92); f_050b1c38(cpu); if cpu.returned_to != 0x50b1c92 { return; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1c93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1c93); }
 let mut pc: u32 = 0x50b1c93;
 loop { match pc {
 0x50b1c93 => {
  { let v: u32 = cpu.r32(0x52ae8a8u32).wrapping_add(cpu.eax); cpu.w32(0x52ae8a8u32, v); }
  pc = 0x50b1c99; continue;
 }
 0x50b1c99 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.edx)) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b1cb7; continue; }
  pc = 0x50b1ca2; continue;
 }
 0x50b1ca2 => {
  if cpu.zf { pc = 0x50b1cc9; continue; }
  pc = 0x50b1ca4; continue;
 }
 0x50b1ca4 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a6cu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5269b80u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  pc = 0x50b1cc4; continue;
 }
 0x50b1cb7 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a6cu32)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5269b80u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  pc = 0x50b1cc4; continue;
 }
 0x50b1cc4 => {
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b1ce1; continue; }
  pc = 0x50b1cc9; continue;
 }
 0x50b1cc9 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8acu32)); cpu.w32(0x52ae8acu32, v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.ebx); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edx)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.edx.wrapping_add(0xcu32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a54u32)); cpu.ebx = v; }
  let t = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.push(0x50b1ce0); cpu.call(t); if cpu.returned_to != 0x50b1ce0 { return; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  pc = 0x50b1ce1; continue;
 }
 0x50b1ce1 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.edx)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b1c99; continue; }
  pc = 0x50b1ced; continue;
 }
 0x50b1ced => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1cee(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1cee); }
 let mut pc: u32 = 0x50b1cee;
 loop { match pc {
 0x50b1cee => {
  { let v: u32 = cpu.esp.wrapping_sub(0x5cu32); cpu.esp = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { let v: u32 = 0x5143a54u32; cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.esp = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.r32(0x5143a5cu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a58u32, v); }
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a60u32, v); }
  { let v: u32 = cpu.r32(0x5143a7cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a68u32).wrapping_add(cpu.eax); cpu.w32(0x5143a68u32, v); }
  { let v: u32 = cpu.r32(0x5143a80u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a6cu32).wrapping_add(cpu.eax); cpu.w32(0x5143a6cu32, v); }
  { let v: u32 = cpu.r32(0x5143a78u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a70u32).wrapping_add(cpu.eax); cpu.w32(0x5143a70u32, v); }
  pc = 0x50b1d34; continue;
 }
 0x50b1d34 => {
  { let v = cpu.esi; cpu.push(v); }
  cpu.push(0x50b1d3a); f_050b16ea(cpu); if cpu.returned_to != 0x50b1d3a { return; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if !cpu.zf { cpu.poll(); pc = 0x50b1d34; continue; }
  pc = 0x50b1d43; continue;
 }
 0x50b1d43 => {
  { let v: u32 = cpu.r32(0x5143a58u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a5cu32); cpu.ebx = v; }
  cpu.push(0x50b1d53); f_050b1b3c(cpu); if cpu.returned_to != 0x50b1d53 { return; }
  { let v: u32 = cpu.r32(0x5143a60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.ebx = v; }
  cpu.push(0x50b1d63); f_050b1a70(cpu); if cpu.returned_to != 0x50b1d63 { return; }
  { let v: u32 = cpu.r32(0x5143a60u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a64u32); cpu.ebx = v; }
  cpu.push(0x50b1d73); f_050b1b3c(cpu); if cpu.returned_to != 0x50b1d73 { return; }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = 0x5143a54u32; cpu.edi = v; }
  { let v: u32 = 0x17u32; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.add32(cpu.esp, 0x5cu32); cpu.esp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1d85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1d85); }
 let mut pc: u32 = 0x50b1d85;
 loop { match pc {
 0x50b1d85 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b1d86(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b1d86); }
 let mut pc: u32 = 0x50b1d86;
 loop { match pc {
 0x50b1d86 => {
  cpu.push(0x50b1d8b); f_050b233f(cpu); if cpu.returned_to != 0x50b1d8b { return; }
  if !cpu.zf { pc = 0x50b2336; continue; }
  pc = 0x50b1d91; continue;
 }
 0x50b1d91 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b2336 => {
  { let v: u32 = 0x5143cc0u32; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b233f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b233f); }
 let mut pc: u32 = 0x50b233f;
 loop { match pc {
 0x50b233f => {
  { let v: u32 = 0x80000001u32; cpu.eax = v; }
  { let v: u32 = 0x7fffffffu32; cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cdcu32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5263ce4u32, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263cf0u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  { let v: u32 = cpu.r32(0x5143aa8u32); cpu.edi = v; }
  { let v: u32 = cpu.esp.wrapping_sub(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b2394); f_050b2c2a(cpu); if cpu.returned_to != 0x50b2394 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b23b4; continue; }
  pc = 0x50b23ae; continue;
 }
 0x50b23ae => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b23b4; continue;
 }
 0x50b23b4 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b23c2; continue; }
  pc = 0x50b23bc; continue;
 }
 0x50b23bc => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b23c2; continue;
 }
 0x50b23c2 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b23d0; continue; }
  pc = 0x50b23ca; continue;
 }
 0x50b23ca => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b23d0; continue;
 }
 0x50b23d0 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b23eb; continue; }
  pc = 0x50b23e6; continue;
 }
 0x50b23e6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b23eb; continue;
 }
 0x50b23eb => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b23f8; continue; }
  pc = 0x50b23f3; continue;
 }
 0x50b23f3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b23f8; continue;
 }
 0x50b23f8 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2417; continue; }
  pc = 0x50b2412; continue;
 }
 0x50b2412 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b2417; continue;
 }
 0x50b2417 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2424; continue; }
  pc = 0x50b241f; continue;
 }
 0x50b241f => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b2424; continue;
 }
 0x50b2424 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b2445); f_050b2c2a(cpu); if cpu.returned_to != 0x50b2445 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2465; continue; }
  pc = 0x50b245f; continue;
 }
 0x50b245f => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b2465; continue;
 }
 0x50b2465 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2473; continue; }
  pc = 0x50b246d; continue;
 }
 0x50b246d => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b2473; continue;
 }
 0x50b2473 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2481; continue; }
  pc = 0x50b247b; continue;
 }
 0x50b247b => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b2481; continue;
 }
 0x50b2481 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b249c; continue; }
  pc = 0x50b2497; continue;
 }
 0x50b2497 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b249c; continue;
 }
 0x50b249c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b24a9; continue; }
  pc = 0x50b24a4; continue;
 }
 0x50b24a4 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b24a9; continue;
 }
 0x50b24a9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b24c8; continue; }
  pc = 0x50b24c3; continue;
 }
 0x50b24c3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b24c8; continue;
 }
 0x50b24c8 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b24d5; continue; }
  pc = 0x50b24d0; continue;
 }
 0x50b24d0 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b24d5; continue;
 }
 0x50b24d5 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b24f6); f_050b2c2a(cpu); if cpu.returned_to != 0x50b24f6 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2516; continue; }
  pc = 0x50b2510; continue;
 }
 0x50b2510 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b2516; continue;
 }
 0x50b2516 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2524; continue; }
  pc = 0x50b251e; continue;
 }
 0x50b251e => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b2524; continue;
 }
 0x50b2524 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2532; continue; }
  pc = 0x50b252c; continue;
 }
 0x50b252c => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b2532; continue;
 }
 0x50b2532 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b254d; continue; }
  pc = 0x50b2548; continue;
 }
 0x50b2548 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b254d; continue;
 }
 0x50b254d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b255a; continue; }
  pc = 0x50b2555; continue;
 }
 0x50b2555 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b255a; continue;
 }
 0x50b255a => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2579; continue; }
  pc = 0x50b2574; continue;
 }
 0x50b2574 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b2579; continue;
 }
 0x50b2579 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2586; continue; }
  pc = 0x50b2581; continue;
 }
 0x50b2581 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b2586; continue;
 }
 0x50b2586 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b25a7); f_050b2c2a(cpu); if cpu.returned_to != 0x50b25a7 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b25c7; continue; }
  pc = 0x50b25c1; continue;
 }
 0x50b25c1 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b25c7; continue;
 }
 0x50b25c7 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b25d5; continue; }
  pc = 0x50b25cf; continue;
 }
 0x50b25cf => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b25d5; continue;
 }
 0x50b25d5 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b25e3; continue; }
  pc = 0x50b25dd; continue;
 }
 0x50b25dd => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b25e3; continue;
 }
 0x50b25e3 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b25fe; continue; }
  pc = 0x50b25f9; continue;
 }
 0x50b25f9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b25fe; continue;
 }
 0x50b25fe => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b260b; continue; }
  pc = 0x50b2606; continue;
 }
 0x50b2606 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b260b; continue;
 }
 0x50b260b => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b262a; continue; }
  pc = 0x50b2625; continue;
 }
 0x50b2625 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b262a; continue;
 }
 0x50b262a => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2637; continue; }
  pc = 0x50b2632; continue;
 }
 0x50b2632 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b2637; continue;
 }
 0x50b2637 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b2658); f_050b2c2a(cpu); if cpu.returned_to != 0x50b2658 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2678; continue; }
  pc = 0x50b2672; continue;
 }
 0x50b2672 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b2678; continue;
 }
 0x50b2678 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2686; continue; }
  pc = 0x50b2680; continue;
 }
 0x50b2680 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b2686; continue;
 }
 0x50b2686 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2694; continue; }
  pc = 0x50b268e; continue;
 }
 0x50b268e => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b2694; continue;
 }
 0x50b2694 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b26af; continue; }
  pc = 0x50b26aa; continue;
 }
 0x50b26aa => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b26af; continue;
 }
 0x50b26af => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b26bc; continue; }
  pc = 0x50b26b7; continue;
 }
 0x50b26b7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b26bc; continue;
 }
 0x50b26bc => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b26db; continue; }
  pc = 0x50b26d6; continue;
 }
 0x50b26d6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b26db; continue;
 }
 0x50b26db => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b26e8; continue; }
  pc = 0x50b26e3; continue;
 }
 0x50b26e3 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b26e8; continue;
 }
 0x50b26e8 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b2709); f_050b2c2a(cpu); if cpu.returned_to != 0x50b2709 { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2729; continue; }
  pc = 0x50b2723; continue;
 }
 0x50b2723 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b2729; continue;
 }
 0x50b2729 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2737; continue; }
  pc = 0x50b2731; continue;
 }
 0x50b2731 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b2737; continue;
 }
 0x50b2737 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2745; continue; }
  pc = 0x50b273f; continue;
 }
 0x50b273f => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b2745; continue;
 }
 0x50b2745 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2760; continue; }
  pc = 0x50b275b; continue;
 }
 0x50b275b => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b2760; continue;
 }
 0x50b2760 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b276d; continue; }
  pc = 0x50b2768; continue;
 }
 0x50b2768 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b276d; continue;
 }
 0x50b276d => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b278c; continue; }
  pc = 0x50b2787; continue;
 }
 0x50b2787 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b278c; continue;
 }
 0x50b278c => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2799; continue; }
  pc = 0x50b2794; continue;
 }
 0x50b2794 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b2799; continue;
 }
 0x50b2799 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b27ba); f_050b2c2a(cpu); if cpu.returned_to != 0x50b27ba { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b27da; continue; }
  pc = 0x50b27d4; continue;
 }
 0x50b27d4 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b27da; continue;
 }
 0x50b27da => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b27e8; continue; }
  pc = 0x50b27e2; continue;
 }
 0x50b27e2 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b27e8; continue;
 }
 0x50b27e8 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b27f6; continue; }
  pc = 0x50b27f0; continue;
 }
 0x50b27f0 => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b27f6; continue;
 }
 0x50b27f6 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2811; continue; }
  pc = 0x50b280c; continue;
 }
 0x50b280c => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b2811; continue;
 }
 0x50b2811 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b281e; continue; }
  pc = 0x50b2819; continue;
 }
 0x50b2819 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b281e; continue;
 }
 0x50b281e => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b283d; continue; }
  pc = 0x50b2838; continue;
 }
 0x50b2838 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b283d; continue;
 }
 0x50b283d => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b284a; continue; }
  pc = 0x50b2845; continue;
 }
 0x50b2845 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b284a; continue;
 }
 0x50b284a => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.edi)); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x4u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.esp), v); }
  { let v: u32 = cpu.esp; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  cpu.push(0x50b286b); f_050b2c2a(cpu); if cpu.returned_to != 0x50b286b { return; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5143a90u32)); cpu.ebx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.r32(0x5143a94u32)); cpu.ecx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cecu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b288b; continue; }
  pc = 0x50b2885; continue;
 }
 0x50b2885 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cecu32, v); }
  pc = 0x50b288b; continue;
 }
 0x50b288b => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5263cf0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2899; continue; }
  pc = 0x50b2893; continue;
 }
 0x50b2893 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263cf0u32, v); }
  pc = 0x50b2899; continue;
 }
 0x50b2899 => {
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b28a7; continue; }
  pc = 0x50b28a1; continue;
 }
 0x50b28a1 => {
  { let v: u32 = cpu.r32(0x5144870u32); cpu.ebx = v; }
  pc = 0x50b28a7; continue;
 }
 0x50b28a7 => {
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263cdcu32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b28c2; continue; }
  pc = 0x50b28bd; continue;
 }
 0x50b28bd => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263cdcu32, v); }
  pc = 0x50b28c2; continue;
 }
 0x50b28c2 => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce0u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b28cf; continue; }
  pc = 0x50b28ca; continue;
 }
 0x50b28ca => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce0u32, v); }
  pc = 0x50b28cf; continue;
 }
 0x50b28cf => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144854u32)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce4u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b28ee; continue; }
  pc = 0x50b28e9; continue;
 }
 0x50b28e9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce4u32, v); }
  pc = 0x50b28ee; continue;
 }
 0x50b28ee => {
  { cpu.sub32(cpu.eax, cpu.r32(0x5263ce8u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b28fb; continue; }
  pc = 0x50b28f6; continue;
 }
 0x50b28f6 => {
  { let v: u32 = cpu.eax; cpu.w32(0x5263ce8u32, v); }
  pc = 0x50b28fb; continue;
 }
 0x50b28fb => {
  { let v: u32 = cpu.r32(0x5263cf0u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5144870u32)); }
  if (cpu.sf != cpu.of) { pc = 0x50b293c; continue; }
  pc = 0x50b2908; continue;
 }
 0x50b2908 => {
  { let v: u32 = cpu.r32(0x5263cdcu32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x5144848u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b293c; continue; }
  pc = 0x50b2915; continue;
 }
 0x50b2915 => {
  { let v: u32 = cpu.r32(0x5263ce4u32); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x514484cu32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b293c; continue; }
  pc = 0x50b2923; continue;
 }
 0x50b2923 => {
  { let v: u32 = cpu.r32(0x5263ce0u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b293c; continue; }
  pc = 0x50b292c; continue;
 }
 0x50b292c => {
  { let v: u32 = cpu.r32(0x5263ce8u32); cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x50b293c; continue; }
  pc = 0x50b2936; continue;
 }
 0x50b2936 => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b293c => {
  { let v: u32 = cpu.esp.wrapping_add(0xcu32); cpu.esp = v; }
  { let v: u32 = cpu.or32(cpu.eax, 0xffffffffu32); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2967(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2967); }
 let mut pc: u32 = 0x50b2967;
 loop { match pc {
 0x50b2967 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a80u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b2a3a; continue; }
  pc = 0x50b297c; continue;
 }
 0x50b297c => {
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5143a6cu32); cpu.ecx = v; }
  { let v: u32 = 0x5269b84u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d70u32, v); }
  pc = 0x50b2999; continue;
 }
 0x50b2999 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x2u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r16(0x4u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { let v = cpu.r32(0x5143a94u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x5143a90u32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.r32(0x5143a8cu32); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { cpu.sub32(cpu.r32(0x5263d70u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b2999; continue; }
  pc = 0x50b2a3a; continue;
 }
 0x50b2a3a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2a3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2a3b); }
 let mut pc: u32 = 0x50b2a3b;
 loop { match pc {
 0x50b2a3b => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a78u32, v); }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x50b2ab9; continue; }
  pc = 0x50b2a4c; continue;
 }
 0x50b2a4c => {
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5143a70u32); cpu.ecx = v; }
  { let v: u32 = 0x526c29cu32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1)); cpu.edi = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b2a66); f_050b309b(cpu); if cpu.returned_to != 0x50b2a66 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  pc = 0x50b2a6c; continue;
 }
 0x50b2a6c => {
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x5143e60u32); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x5143e64u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.edx); cpu.ebx = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x5143e68u32); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if (cpu.sf == cpu.of) { pc = 0x50b2aa4; continue; }
  pc = 0x50b2aa2; continue;
 }
 0x50b2aa2 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  pc = 0x50b2aa4; continue;
 }
 0x50b2aa4 => {
  { let v: u32 = cpu.eax.wrapping_add(0x300u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), 0x3fffu16); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2ab4; continue; }
  pc = 0x50b2aaf; continue;
 }
 0x50b2aaf => {
  { let v: u32 = 0x3fffu32; cpu.eax = v; }
  pc = 0x50b2ab4; continue;
 }
 0x50b2ab4 => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b2a6c; continue; }
  pc = 0x50b2ab9; continue;
 }
 0x50b2ab9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2aba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2aba); }
 let mut pc: u32 = 0x50b2aba;
 loop { match pc {
 0x50b2aba => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a84u32, v); }
  { let v: u32 = cpu.r32(0x5143a74u32); cpu.ecx = v; }
  { let v: u32 = 0x526ca70u32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(4)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d70u32, v); }
  pc = 0x50b2adc; continue;
 }
 0x50b2adc => {
  { let v: u32 = 0x5143d64u32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143cc8u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143cccu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143cd0u32, v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5143cc8u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143cccu32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x50b2b66); f_050b3cff(cpu); if cpu.returned_to != 0x50b2b66 { return; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x5143cd0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143cccu32); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x50b2b86); f_050b3cff(cpu); if cpu.returned_to != 0x50b2b86 { return; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x5263d70u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b2adc; continue; }
  pc = 0x50b2ba9; continue;
 }
 0x50b2ba9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2baa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2baa); }
 let mut pc: u32 = 0x50b2baa;
 loop { match pc {
 0x50b2baa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2bab(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2bab); }
 let mut pc: u32 = 0x50b2bab;
 loop { match pc {
 0x50b2bab => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v = cpu.eax; cpu.push(v); }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { cpu.lods(2, false); }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2c2a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2c2a); }
 let mut pc: u32 = 0x50b2c2a;
 loop { match pc {
 0x50b2c2a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2c93(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2c93); }
 let mut pc: u32 = 0x50b2c93;
 loop { match pc {
 0x50b2c93 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2cfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2cfc); }
 let mut pc: u32 = 0x50b2cfc;
 loop { match pc {
 0x50b2cfc => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2d69(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2d69); }
 let mut pc: u32 = 0x50b2d69;
 loop { match pc {
 0x50b2d69 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263cf4u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x4u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263cf8u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x8u32.wrapping_add(cpu.ecx))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263cfcu32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d00u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d04u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d08u32, v); }
  { let v: u32 = 0x5263cf4u32; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5263d04u32)); cpu.ebx = v; }
  { cpu.sub32(cpu.ebx, cpu.r32(0x5144870u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b2e57; continue; }
  pc = 0x50b2e19; continue;
 }
 0x50b2e19 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x5263d0cu32, v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5263d00u32)); cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5144850u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5263d08u32)); cpu.eax = v; }
  { let v = cpu.r32(0x5144870u32); cpu.imul32(v); }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.r32(0x5144854u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x1u32; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b2e57 => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b2e5e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b2e5e); }
 let mut pc: u32 = 0x50b2e5e;
 loop { match pc {
 0x50b2e5e => {
  { cpu.sub32(cpu.r32(0x5143a88u32), 0x5143cf8u32); }
  if cpu.zf { pc = 0x50b2f74; continue; }
  pc = 0x50b2e6e; continue;
 }
 0x50b2e6e => {
  { let v: u32 = cpu.r32(0x5143a88u32); cpu.eax = v; }
  { let v: u32 = 0x5263d1cu32; cpu.ebx = v; }
  cpu.push(0x50b2e7d); f_050b3f0b(cpu); if cpu.returned_to != 0x50b2e7d { return; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a7cu32, v); }
  { let v: u32 = cpu.r32(0x5143a68u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x5265d00u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x5263dbcu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = 0x5263d1cu32; cpu.ebx = v; }
  { let v = (cpu.r32(0x5143a8cu32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5263d10u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5143a90u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5263d14u32, v); cpu.fpop(); }
  { let v = (cpu.r32(0x5143a94u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5263d18u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5144870u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a7cu32); cpu.ecx = v; }
  pc = 0x50b2ed4; continue;
 }
 0x50b2ed4 => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x5263d10u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x5263d14u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0) + cpu.rf32(0x5263d18u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b2f57; continue; }
  pc = 0x50b2f33; continue;
 }
 0x50b2f33 => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x5144874u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x5144858u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x514485cu32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x50b2f57; continue;
 }
 0x50b2f57 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b2ed4; continue; }
  pc = 0x50b2f73; continue;
 }
 0x50b2f73 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x50b2f74 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a7cu32, v); }
  { let v: u32 = cpu.r32(0x5143a68u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x5265d00u32.wrapping_add(cpu.ecx.wrapping_mul(8)); cpu.edx = v; }
  { let v: u32 = 0x5263dbcu32.wrapping_add(cpu.ecx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143aacu32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5143ce0u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d40u32, v); }
  { let v = (cpu.r32(0x5263d40u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5143ce4u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x5263d40u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d44u32, v); }
  { let v = (cpu.r32(0x5263d44u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebp)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x5143ce8u32)); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x5263d44u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d48u32, v); }
  { let v = (cpu.r32(0x5263d48u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0); cpu.wf32(0x5263d48u32, v); cpu.fpop(); }
  { let v: u32 = 0x5143d1cu32; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5144870u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5143a7cu32); cpu.ecx = v; }
  pc = 0x50b2ffb; continue;
 }
 0x50b2ffb => {
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5263d48u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5263d44u32); cpu.set_st(0, v); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.esi)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5263d40u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0xcu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x10u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edx), v); }
  { let v = cpu.rf32(0x8u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(5); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x20u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(7); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.edx), v); }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.edx)), cpu.eax); }
  if (cpu.sf != cpu.of) { pc = 0x50b307e; continue; }
  pc = 0x50b305a; continue;
 }
 0x50b305a => {
  { cpu.fpush(1.0); }
  { let v = cpu.st(0) / cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.edx), v); }
  { let v = cpu.st(0) * cpu.rf32(0x5144874u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(4); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x5144858u32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x514485cu32) - cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.edi), v); cpu.fpop(); }
  pc = 0x50b307e; continue;
 }
 0x50b307e => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x10u32); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.edi, 0x8u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b2ffb; continue; }
  pc = 0x50b309a; continue;
 }
 0x50b309a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b309b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b309b); }
 let mut pc: u32 = 0x50b309b;
 loop { match pc {
 0x50b309b => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d70u32, v); }
  pc = 0x50b30a5; continue;
 }
 0x50b30a5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x0u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0xcu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x18u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x4u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x10u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x8u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x14u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.ecx as u16), (cpu.eax as u16)); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x4u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.r16(0x20u32.wrapping_add(cpu.ebx)); cpu.imul16(v); }
  { let v: u16 = cpu.add16((cpu.eax as u16), (cpu.ecx as u16)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let a = (cpu.eax as u16) as u32; let b = (cpu.edx as u16) as u32; let n = 14u32; if n != 0 && n <= 16 { let wide: u32 = ((b << 16) | a) >> n; let r = wide as u16; cpu.cf = (a >> (n - 1)) & 1 != 0; cpu.zf = r == 0; cpu.sf = r & 0x8000 != 0; cpu.parity = r as u8; let v: u16 = r; cpu.eax = (cpu.eax & 0xffff0000) | v as u32; } }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x6u32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x5263d70u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b30a5; continue; }
  pc = 0x50b3140; continue;
 }
 0x50b3140 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3141(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3141); }
 let mut pc: u32 = 0x50b3141;
 loop { match pc {
 0x50b3141 => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d70u32, v); }
  pc = 0x50b314b; continue;
 }
 0x50b314b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x5263d70u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b314b; continue; }
  pc = 0x50b31be; continue;
 }
 0x50b31be => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3398(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3398); }
 let mut pc: u32 = 0x50b3398;
 loop { match pc {
 0x50b3398 => {
  { let v: u32 = cpu.shl32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d70u32, v); }
  pc = 0x50b33a2; continue;
 }
 0x50b33a2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a8cu32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a90u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5143a94u32)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0xcu32); cpu.esi = v; }
  { cpu.sub32(cpu.r32(0x5263d70u32), cpu.edi); }
  if !cpu.zf { cpu.poll(); pc = 0x50b33a2; continue; }
  pc = 0x50b342b; continue;
 }
 0x50b342b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3519(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3519); }
 let mut pc: u32 = 0x50b3519;
 loop { match pc {
 0x50b3519 => {
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.ebx = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = 0x3fffu32; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3553(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3553); }
 let mut pc: u32 = 0x50b3553;
 loop { match pc {
 0x50b3553 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d58u32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d64u32, v); }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d5cu32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d68u32, v); }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d60u32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d6cu32, v); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b36b6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b36b6); }
 let mut pc: u32 = 0x50b36b6;
 loop { match pc {
 0x50b36b6 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.neg32(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.ebx & 0xfffu32; cpu.ebx = v; }
  { let v: u32 = cpu.ecx & 0xfffu32; cpu.ecx = v; }
  { let v: u32 = cpu.edx & 0xfffu32; cpu.edx = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d58u32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d64u32, v); }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d5cu32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.ecx).wrapping_add(cpu.ecx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d68u32, v); }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d60u32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d6cu32, v); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d60u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d6cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d5cu32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d68u32); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b381f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b381f); }
 let mut pc: u32 = 0x50b381f;
 loop { match pc {
 0x50b381f => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.edx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x20u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ebp); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3958(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3958); }
 let mut pc: u32 = 0x50b3958;
 loop { match pc {
 0x50b3958 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5263d58u32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d64u32, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d58u32); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d64u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3a34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3a34); }
 let mut pc: u32 = 0x50b3a34;
 loop { match pc {
 0x50b3a34 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5263d5cu32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d68u32, v); }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d5cu32); cpu.eax = v; }
  { let v = cpu.r32(0x18u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d68u32); cpu.eax = v; }
  { let v = cpu.r32(0x20u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3b0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3b0f); }
 let mut pc: u32 = 0x50b3b0f;
 loop { match pc {
 0x50b3b0f => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x5263d60u32, v); }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d6cu32, v); }
  { let v: u32 = cpu.r32(0x5263d60u32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d60u32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d60u32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d60u32); cpu.eax = v; }
  { let v = cpu.r32(0x0u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d6cu32); cpu.eax = v; }
  { let v = cpu.r32(0xcu32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d60u32); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d60u32); cpu.eax = v; }
  { let v = cpu.r32(0x8u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5263d6cu32); cpu.eax = v; }
  { let v = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = 0x18u32.wrapping_add(cpu.ebx); cpu.esi = v; }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  { cpu.movs(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3be4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3be4); }
 let mut pc: u32 = 0x50b3be4;
 loop { match pc {
 0x50b3be4 => {
  { let v = cpu.ebp; cpu.push(v); }
  cpu.push(0x50b3bea); f_050b3c3d(cpu); if cpu.returned_to != 0x50b3bea { return; }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x1cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  cpu.push(0x50b3bfa); f_050b3c3d(cpu); if cpu.returned_to != 0x50b3bfa { return; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x18u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x1cu32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v = cpu.r32(0x10u32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v = cpu.r32(0x4u32.wrapping_add(cpu.edi)); cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x20u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3c3d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3c3d); }
 let mut pc: u32 = 0x50b3c3d;
 loop { match pc {
 0x50b3c3d => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b3c58; continue;
 }
 0x50b3c58 => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x50b3c63; continue; }
  pc = 0x50b3c5c; continue;
 }
 0x50b3c5c => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x50b3c6d; continue; }
  pc = 0x50b3c63; continue;
 }
 0x50b3c63 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x50b3c58; continue;
 }
 0x50b3c6d => {
  cpu.push(0x50b3c72); f_050b3cdc(cpu); if cpu.returned_to != 0x50b3c72 { return; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { cpu.and32(cpu.esi, cpu.esi); }
  if !cpu.zf { pc = 0x50b3c7f; continue; }
  pc = 0x50b3c7a; continue;
 }
 0x50b3c7a => {
  { let v: u32 = 0x1u32; cpu.esi = v; }
  pc = 0x50b3c7f; continue;
 }
 0x50b3c7f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.esi; cpu.idiv32(v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3ca5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3ca5); }
 let mut pc: u32 = 0x50b3ca5;
 loop { match pc {
 0x50b3ca5 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebp); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b3cbe; continue;
 }
 0x50b3cbe => {
  { cpu.and32(cpu.edx, cpu.edx); }
  if !cpu.zf { pc = 0x50b3cc9; continue; }
  pc = 0x50b3cc2; continue;
 }
 0x50b3cc2 => {
  { cpu.sub32(cpu.eax, 0x3fffffffu32); }
  if cpu.cf { pc = 0x50b3cd3; continue; }
  pc = 0x50b3cc9; continue;
 }
 0x50b3cc9 => {
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 2u32); cpu.edx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  cpu.poll(); pc = 0x50b3cbe; continue;
 }
 0x50b3cd3 => {
  cpu.push(0x50b3cd8); f_050b3cdc(cpu); if cpu.returned_to != 0x50b3cd8 { return; }
  { let v: u32 = cpu.shl32(cpu.eax, (cpu.ecx & 0xff)); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3cdc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3cdc); }
 let mut pc: u32 = 0x50b3cdc;
 loop { match pc {
 0x50b3cdc => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = 0x10000000u32; cpu.ebx = v; }
  pc = 0x50b3ce6; continue;
 }
 0x50b3ce6 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_add(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf != cpu.of) { pc = 0x50b3cf4; continue; }
  pc = 0x50b3cf0; continue;
 }
 0x50b3cf0 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edx); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ebx); cpu.ecx = v; }
  pc = 0x50b3cf4; continue;
 }
 0x50b3cf4 => {
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b3ce6; continue; }
  pc = 0x50b3cf9; continue;
 }
 0x50b3cf9 => {
  { let v: u32 = cpu.ecx; cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3cff(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3cff); }
 let mut pc: u32 = 0x50b3cff;
 loop { match pc {
 0x50b3cff => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b3d0a; continue; }
  pc = 0x50b3d08; continue;
 }
 0x50b3d08 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b3d0a; continue;
 }
 0x50b3d0a => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b3d10; continue; }
  pc = 0x50b3d0e; continue;
 }
 0x50b3d0e => {
  { let v: u32 = cpu.neg32(cpu.ebx); cpu.ebx = v; }
  pc = 0x50b3d10; continue;
 }
 0x50b3d10 => {
  { cpu.sub32(cpu.ebx, cpu.eax); }
  { let v: u8 = ((cpu.zf || cpu.sf != cpu.of)) as u8; cpu.ecx = (cpu.ecx & 0xffffff00) | v as u32; }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b3d18; continue; }
  pc = 0x50b3d17; continue;
 }
 0x50b3d17 => {
  { let a = cpu.ebx; let b = cpu.eax; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.eax = v; } }
  pc = 0x50b3d18; continue;
 }
 0x50b3d18 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b3d4b; continue; }
  pc = 0x50b3d1c; continue;
 }
 0x50b3d1c => {
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 18u32); cpu.edx = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.sar32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u16 = cpu.r16(0x5270214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and8((cpu.ecx as u8), (cpu.ecx as u8)); }
  if cpu.zf { pc = 0x50b3d3c; continue; }
  pc = 0x50b3d35; continue;
 }
 0x50b3d35 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(0x400u32); cpu.eax = v; }
  pc = 0x50b3d3c; continue;
 }
 0x50b3d3c => {
  { let v: u32 = cpu.xor32(cpu.esi, cpu.ebp); cpu.esi = v; }
  if (cpu.sf == cpu.of) { pc = 0x50b3d42; continue; }
  pc = 0x50b3d40; continue;
 }
 0x50b3d40 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b3d42; continue;
 }
 0x50b3d42 => {
  { cpu.and32(cpu.ebp, cpu.ebp); }
  if (cpu.sf == cpu.of) { pc = 0x50b3d4b; continue; }
  pc = 0x50b3d46; continue;
 }
 0x50b3d46 => {
  { let v: u32 = cpu.eax.wrapping_add(0x800u32); cpu.eax = v; }
  pc = 0x50b3d4b; continue;
 }
 0x50b3d4b => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3d52(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3d52); }
 let mut pc: u32 = 0x50b3d52;
 loop { match pc {
 0x50b3d52 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d74u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d78u32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d7cu32, v); }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d80u32, v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d84u32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d88u32, v); }
  { let v: u32 = cpu.r32(0x5263d74u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d78u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d7cu32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50b3dae); f_050b3cdc(cpu); if cpu.returned_to != 0x50b3dae { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d74u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d74u32, v); }
  { let v: u32 = cpu.r32(0x5263d78u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d78u32, v); }
  { let v: u32 = cpu.r32(0x5263d7cu32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d7cu32, v); }
  { let v: u32 = cpu.r32(0x5263d80u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d84u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.adc32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d88u32); cpu.eax = v; }
  { let v = cpu.eax; cpu.imul32(v); }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  cpu.push(0x50b3e1c); f_050b3cdc(cpu); if cpu.returned_to != 0x50b3e1c { return; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5263d80u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d80u32, v); }
  { let v: u32 = cpu.r32(0x5263d84u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d84u32, v); }
  { let v: u32 = cpu.r32(0x5263d88u32); cpu.eax = v; }
  { cpu.edx = (cpu.edx & 0xffff0000) | (((cpu.eax as u16 as i16) >> 15) as u16 as u32); }
  { let v: u32 = cpu.shld32(cpu.edx, cpu.eax, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 14u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d88u32, v); }
  { let v: u32 = cpu.r32(0x5263d7cu32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d84u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d78u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d88u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d8cu32, v); }
  { let v: u32 = cpu.r32(0x5263d74u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d88u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d7cu32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d80u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d90u32, v); }
  { let v: u32 = cpu.r32(0x5263d78u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d80u32); cpu.imul32(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5263d74u32); cpu.eax = v; }
  { let v = cpu.r32(0x5263d84u32); cpu.imul32(v); }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.sbb32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.shrd32(cpu.eax, cpu.edx, 14u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5263d94u32, v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5263d74u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d80u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d8cu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d78u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d84u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d90u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d7cu32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d88u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  { let v: u32 = cpu.r32(0x5263d94u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3f0b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3f0b); }
 let mut pc: u32 = 0x50b3f0b;
 loop { match pc {
 0x50b3f0b => {
  { let v = cpu.rf32(0x5143cdcu32); cpu.fpush(v); }
  { let v = (cpu.r32(0x4u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x4u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x10u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x10u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x1cu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x1cu32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x0u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x0u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0xcu32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0xcu32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x18u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x18u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x8u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x8u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x14u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x14u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = (cpu.r32(0x20u32.wrapping_add(cpu.eax)) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x20u32.wrapping_add(cpu.ebx), v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3f5a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3f5a); }
 let mut pc: u32 = 0x50b3f5a;
 loop { match pc {
 0x50b3f5a => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3f68(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3f68); }
 let mut pc: u32 = 0x50b3f68;
 loop { match pc {
 0x50b3f68 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x526da14u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 15u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3fa5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3fa5); }
 let mut pc: u32 = 0x50b3fa5;
 loop { match pc {
 0x50b3fa5 => {
  { let v: u32 = cpu.and32(cpu.eax, 0xfffu32); cpu.eax = v; }
  { let v: u32 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))) as i16 as i32 as u32; cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3fb3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3fb3); }
 let mut pc: u32 = 0x50b3fb3;
 loop { match pc {
 0x50b3fb3 => {
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v = cpu.edx; cpu.push(v); }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.ebx & 0xfu32; cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0xfffu32; cpu.eax = v; }
  { let v: u16 = cpu.r16(0x526e214u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u16 = cpu.sar16((cpu.edx as u16), 15u32); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v = (cpu.ebx as u16); cpu.idiv16(v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 18u32); cpu.eax = v; }
  { let v: u32 = cpu.pop(); cpu.edx = v; }
  { let v: u32 = cpu.pop(); cpu.ecx = v; }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b3ff0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b3ff0); }
 let mut pc: u32 = 0x50b3ff0;
 loop { match pc {
 0x50b3ff0 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a8cu32, v); }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a90u32, v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x5143a94u32, v); }
  { let v: u32 = cpu.esi; cpu.w32(0x5143aacu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5146e50u32.wrapping_add(cpu.esi.wrapping_mul(4))); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2eu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  pc = 0x50b4020; continue;
 }
 0x50b4020 => {
  { let v: u32 = cpu.r16(0x0u32.wrapping_add(cpu.esi)) as i16 as i32 as u32; cpu.ebx = v; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if (cpu.sf != cpu.of) { pc = 0x50b403d; continue; }
  pc = 0x50b4027; continue;
 }
 0x50b4027 => {
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  let t = cpu.r32(0x5143ab4u32.wrapping_add(cpu.ebx)); cpu.push(0x50b4036); cpu.call(t); if cpu.returned_to != 0x50b4036 { return; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  cpu.poll(); pc = 0x50b4020; continue;
 }
 0x50b403d => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b403f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b403f); }
 let mut pc: u32 = 0x50b403f;
 loop { match pc {
 0x50b403f => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5146e50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x22u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b404d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b404d); }
 let mut pc: u32 = 0x50b404d;
 loop { match pc {
 0x50b404d => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5146e50u32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.esi), v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.eax)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.eax)); cpu.edx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edx; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b4080(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b4080); }
 let mut pc: u32 = 0x50b4080;
 loop { match pc {
 0x50b4080 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b4180(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b4180); }
 let mut pc: u32 = 0x50b4180;
 loop { match pc {
 0x50b4180 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x50b4189; continue;
 }
 0x50b4189 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4189; continue; }
  pc = 0x50b4197; continue;
 }
 0x50b4197 => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x52a4ca0u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b41ab; continue;
 }
 0x50b41ab => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x52aa0a8u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b41cc; continue; }
  pc = 0x50b41bf; continue;
 }
 0x50b41bf => {
  if cpu.zf { pc = 0x50b4328; continue; }
  pc = 0x50b41c5; continue;
 }
 0x50b41c5 => {
  { let v: u32 = 0x52a4ca8u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x50b41cc; continue;
 }
 0x50b41cc => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a18u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a1cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5270a18u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x52a4ca4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4204; continue; }
  pc = 0x50b41fe; continue;
 }
 0x50b41fe => {
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b4204; continue;
 }
 0x50b4204 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x52a4ca0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b4212; continue; }
  pc = 0x50b420c; continue;
 }
 0x50b420c => {
  { let v: u32 = cpu.edx; cpu.w32(0x52a4ca0u32, v); }
  pc = 0x50b4212; continue;
 }
 0x50b4212 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4328; continue; }
  pc = 0x50b421a; continue;
 }
 0x50b421a => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5143f24u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x52a4938u32, v); }
  { let v = (cpu.r32(0x5270a18u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x52a4938u32), 0x52aa0a8u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b425e; continue; }
  pc = 0x50b425c; continue;
 }
 0x50b425c => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b425e; continue;
 }
 0x50b425e => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a20u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5270a20u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a24u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ecx = v; }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b430a; continue; }
  pc = 0x50b42a9; continue;
 }
 0x50b42a9 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b42a9; continue; }
  pc = 0x50b430a; continue;
 }
 0x50b430a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x50b430d; continue;
 }
 0x50b430d => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b430d; continue; }
  pc = 0x50b4321; continue;
 }
 0x50b4321 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x50b4328; continue;
 }
 0x50b4328 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b41ab; continue; }
  pc = 0x50b4334; continue;
 }
 0x50b4334 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b4335(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b4335); }
 let mut pc: u32 = 0x50b4335;
 loop { match pc {
 0x50b4335 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = 0x7fffffffu32; cpu.edx = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.ebp); cpu.ebp = v; }
  pc = 0x50b4345; continue;
 }
 0x50b4345 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), cpu.edx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4352; continue; }
  pc = 0x50b434f; continue;
 }
 0x50b434f => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.edx = v; }
  pc = 0x50b4352; continue;
 }
 0x50b4352 => {
  { cpu.sub32(cpu.r32(0x14u32.wrapping_add(cpu.ebx)), cpu.ebp); }
  if (cpu.sf != cpu.of) { pc = 0x50b435a; continue; }
  pc = 0x50b4357; continue;
 }
 0x50b4357 => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.ebx)); cpu.ebp = v; }
  pc = 0x50b435a; continue;
 }
 0x50b435a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4345; continue; }
  pc = 0x50b4371; continue;
 }
 0x50b4371 => {
  { let v: u32 = cpu.edx; cpu.w32(0x5270a6cu32, v); }
  { let v: u32 = cpu.ebp; cpu.w32(0x5270a70u32, v); }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v = cpu.rf32(0x5270a6cu32); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5143f28u32); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5270a70u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x5270a6cu32); cpu.set_st(0, v); }
  { let a = cpu.st(0); let b = cpu.st(1); cpu.fcom(a, b); cpu.fpop(); cpu.fpop();  }
  { let v = cpu.fstatus(); cpu.set_ax(v); }
  { let v = (cpu.eax >> 8) & 0xff; let d = cpu.df; let o = cpu.of; cpu.set_flags(v); cpu.df = d; cpu.of = o; }
  { let v: u8 = ((!cpu.cf && !cpu.zf)) as u8; cpu.w8(0x5143f2cu32, v); }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = 0x7fffu32; cpu.w32(0x52a4ca0u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b43b9; continue;
 }
 0x50b43b9 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x52aa0a8u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b43da; continue; }
  pc = 0x50b43cd; continue;
 }
 0x50b43cd => {
  if cpu.zf { pc = 0x50b471d; continue; }
  pc = 0x50b43d3; continue;
 }
 0x50b43d3 => {
  { let v: u32 = 0x52a4ca8u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x50b43da; continue;
 }
 0x50b43da => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a18u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a1cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5270a18u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x52a4ca4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4412; continue; }
  pc = 0x50b440c; continue;
 }
 0x50b440c => {
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b4412; continue;
 }
 0x50b4412 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x52a4ca0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b4420; continue; }
  pc = 0x50b441a; continue;
 }
 0x50b441a => {
  { let v: u32 = cpu.edx; cpu.w32(0x52a4ca0u32, v); }
  pc = 0x50b4420; continue;
 }
 0x50b4420 => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b471d; continue; }
  pc = 0x50b4428; continue;
 }
 0x50b4428 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5143f24u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x52a4938u32, v); }
  { let v = (cpu.r32(0x5270a18u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x5270a64u32, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a28u32, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x5270a38u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a2cu32, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.wf32(0x5270a3cu32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x52a4938u32), 0x52aa0a8u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b449e; continue; }
  pc = 0x50b4499; continue;
 }
 0x50b4499 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  pc = 0x50b449e; continue;
 }
 0x50b449e => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a20u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5270a20u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a24u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b454e; continue; }
  pc = 0x50b44ed; continue;
 }
 0x50b44ed => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b44ed; continue; }
  pc = 0x50b454e; continue;
 }
 0x50b454e => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x50b4551; continue;
 }
 0x50b4551 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4551; continue; }
  pc = 0x50b4565; continue;
 }
 0x50b4565 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.rf32(0x5270a3cu32); cpu.fpush(v); }
  { let v = cpu.rf32(0x5270a38u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x5143f24u32); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b458f; continue; }
  pc = 0x50b458d; continue;
 }
 0x50b458d => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b458f; continue;
 }
 0x50b458f => {
  { cpu.sub8(cpu.r8(0x5143f2cu32), 0x0u8); }
  if cpu.zf { pc = 0x50b45a1; continue; }
  pc = 0x50b4598; continue;
 }
 0x50b4598 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  pc = 0x50b45ca; continue;
 }
 0x50b45a1 => {
  { let v: u32 = 0x1u32; cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x30000u32); }
  if (cpu.sf == cpu.of) { pc = 0x50b45ca; continue; }
  pc = 0x50b45af; continue;
 }
 0x50b45af => {
  { let v: u32 = 0x2u32; cpu.ebp = v; }
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { cpu.sub32(cpu.eax, 0x10000u32); }
  if (cpu.sf == cpu.of) { pc = 0x50b45ca; continue; }
  pc = 0x50b45c0; continue;
 }
 0x50b45c0 => {
  { let v: u32 = 0x4u32; cpu.ebp = v; }
  { let v: u32 = 0x2u32; cpu.ecx = v; }
  pc = 0x50b45ca; continue;
 }
 0x50b45ca => {
  { cpu.sub32(cpu.ebp, cpu.r32(0x4u32.wrapping_add(cpu.esp))); }
  if (cpu.sf != cpu.of) { pc = 0x50b45d4; continue; }
  pc = 0x50b45d0; continue;
 }
 0x50b45d0 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  pc = 0x50b45d4; continue;
 }
 0x50b45d4 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x5270a5cu32, v); }
  { let v: u32 = cpu.r32(0x529160cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.ebp; cpu.w32(0x5270a58u32, v); }
  { let v: u32 = cpu.ecx; cpu.w32(0x5270a60u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.rf32(0x5143f20u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x5270a64u32); cpu.fpush(v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x5270a2cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a30u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a34u32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5270a30u32)); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x5270a34u32)); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x5270a58u32); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5270a58u32); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a30u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a34u32, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x5270a5cu32)); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b46da; continue; }
  pc = 0x50b4676; continue;
 }
 0x50b4676 => {
  { let v = cpu.st(0) + cpu.rf32(0x5270a58u32); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x5270a58u32); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a60u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5270a30u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5270a34u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x5270a5cu32); cpu.ecx = v; }
  pc = 0x50b46a6; continue;
 }
 0x50b46a6 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b46a6; continue; }
  pc = 0x50b46bc; continue;
 }
 0x50b46bc => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a30u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a34u32, v); cpu.fpop(); }
  { let v: u32 = cpu.sub32(cpu.ebp, cpu.r32(0x5270a5cu32)); cpu.ebp = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b4676; continue; }
  pc = 0x50b46da; continue;
 }
 0x50b46da => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x5270a5cu32)); cpu.ebp = v; }
  if cpu.zf { pc = 0x50b470e; continue; }
  pc = 0x50b46e2; continue;
 }
 0x50b46e2 => {
  { let v: u32 = cpu.r32(0x5270a60u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5270a30u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x5270a34u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.esi, (cpu.ecx & 0xff)); cpu.esi = v; }
  { let v: u32 = cpu.sar32(cpu.edx, (cpu.ecx & 0xff)); cpu.edx = v; }
  pc = 0x50b46f8; continue;
 }
 0x50b46f8 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b46f8; continue; }
  pc = 0x50b470e; continue;
 }
 0x50b470e => {
  { let v: u32 = cpu.esp.wrapping_add(0x8u32); cpu.esp = v; }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x50b471d; continue;
 }
 0x50b471d => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b43b9; continue; }
  pc = 0x50b4729; continue;
 }
 0x50b4729 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b472a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b472a); }
 let mut pc: u32 = 0x50b472a;
 loop { match pc {
 0x50b472a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x50b4733; continue;
 }
 0x50b4733 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.ebx)).wrapping_add(0x4800000u32); cpu.w32(0x1cu32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4733; continue; }
  pc = 0x50b474f; continue;
 }
 0x50b474f => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x52a4ca0u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b4763; continue;
 }
 0x50b4763 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x52aa0a8u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4784; continue; }
  pc = 0x50b4777; continue;
 }
 0x50b4777 => {
  if cpu.zf { pc = 0x50b49c8; continue; }
  pc = 0x50b477d; continue;
 }
 0x50b477d => {
  { let v: u32 = 0x52a4ca8u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x50b4784; continue;
 }
 0x50b4784 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a18u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a1cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5270a18u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x52a4ca4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b47bc; continue; }
  pc = 0x50b47b6; continue;
 }
 0x50b47b6 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b47bc; continue;
 }
 0x50b47bc => {
  { cpu.sub32(cpu.edx, cpu.r32(0x52a4ca0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b47ca; continue; }
  pc = 0x50b47c4; continue;
 }
 0x50b47c4 => {
  { let v: u32 = cpu.edx; cpu.w32(0x52a4ca0u32, v); }
  pc = 0x50b47ca; continue;
 }
 0x50b47ca => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b49c8; continue; }
  pc = 0x50b47d2; continue;
 }
 0x50b47d2 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5143f24u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x52a4938u32, v); }
  { let v = (cpu.r32(0x5270a18u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.ebx; cpu.push(v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a28u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a28u32); cpu.eax = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a30u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x1cu32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a30u32); cpu.ecx = v; }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a2cu32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a2cu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a34u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a34u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b48c0; continue; }
  pc = 0x50b4858; continue;
 }
 0x50b4858 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc04u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1804u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc08u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1808u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc0cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x180cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1810u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xc14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1814u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b4858; continue; }
  pc = 0x50b48c0; continue;
 }
 0x50b48c0 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x50b48c3; continue;
 }
 0x50b48c3 => {
  { let v: u32 = cpu.eax; cpu.w32(0xc00u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x1800u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.edx); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b48c3; continue; }
  pc = 0x50b48d9; continue;
 }
 0x50b48d9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebx = v; }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x52a4938u32), 0x52aa0a8u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b48f9; continue; }
  pc = 0x50b48f7; continue;
 }
 0x50b48f7 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b48f9; continue;
 }
 0x50b48f9 => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a20u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5270a20u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a24u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.edi)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b49a9; continue; }
  pc = 0x50b4948; continue;
 }
 0x50b4948 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b4948; continue; }
  pc = 0x50b49a9; continue;
 }
 0x50b49a9 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x50b49ac; continue;
 }
 0x50b49ac => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b49ac; continue; }
  pc = 0x50b49c0; continue;
 }
 0x50b49c0 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.pop(); cpu.ebx = v; }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x50b49c8; continue;
 }
 0x50b49c8 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4763; continue; }
  pc = 0x50b49d4; continue;
 }
 0x50b49d4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b49d5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b49d5); }
 let mut pc: u32 = 0x50b49d5;
 loop { match pc {
 0x50b49d5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.eax.wrapping_mul(4)), v); }
  { let v: u32 = cpu.esi; cpu.edi = v; }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  pc = 0x50b49de; continue;
 }
 0x50b49de => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.ebx)).wrapping_add(0x8000000u32); cpu.w32(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.ebx)).wrapping_add(0x4000000u32); cpu.w32(0x18u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b49de; continue; }
  pc = 0x50b49f3; continue;
 }
 0x50b49f3 => {
  { let v: u32 = 0x7fffu32; cpu.w32(0x52a4ca0u32, v); }
  { let v: u32 = 0xffff8001u32; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b4a07; continue;
 }
 0x50b4a07 => {
  { let v = cpu.eax; cpu.push(v); }
  { let v = cpu.esi; cpu.push(v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.ebx)); cpu.eax = v; }
  { let v: u32 = 0x52aa0a8u32; cpu.edi = v; }
  { cpu.sub32(cpu.eax, cpu.r32(0x4u32.wrapping_add(cpu.esi))); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4a28; continue; }
  pc = 0x50b4a1b; continue;
 }
 0x50b4a1b => {
  if cpu.zf { pc = 0x50b4c02; continue; }
  pc = 0x50b4a21; continue;
 }
 0x50b4a21 => {
  { let v: u32 = 0x52a4ca8u32; cpu.edi = v; }
  { let a = cpu.ebx; let b = cpu.esi; { let v: u32 = b; cpu.ebx = v; } { let v: u32 = a; cpu.esi = v; } }
  pc = 0x50b4a28; continue;
 }
 0x50b4a28 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a18u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a1cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a1cu32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x5270a18u32); cpu.edx = v; }
  { cpu.sub32(cpu.ecx, cpu.r32(0x52a4ca4u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4a60; continue; }
  pc = 0x50b4a5a; continue;
 }
 0x50b4a5a => {
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4ca4u32, v); }
  pc = 0x50b4a60; continue;
 }
 0x50b4a60 => {
  { cpu.sub32(cpu.edx, cpu.r32(0x52a4ca0u32)); }
  if (cpu.sf == cpu.of) { pc = 0x50b4a6e; continue; }
  pc = 0x50b4a68; continue;
 }
 0x50b4a68 => {
  { let v: u32 = cpu.edx; cpu.w32(0x52a4ca0u32, v); }
  pc = 0x50b4a6e; continue;
 }
 0x50b4a6e => {
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.edx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4c02; continue; }
  pc = 0x50b4a76; continue;
 }
 0x50b4a76 => {
  { let v = cpu.rf32(0x4u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5143f24u32) / cpu.st(0); cpu.set_st(0, v); }
  { let v = cpu.ecx; cpu.push(v); }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.edx.wrapping_mul(4)); cpu.edi = v; }
  { let v: u32 = cpu.edi; cpu.w32(0x52a4938u32, v); }
  { let v = (cpu.r32(0x5270a18u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) + cpu.rf32(0x5143f1cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.rf32(0x4u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v = cpu.st(0) * cpu.st(1); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x18u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a28u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a28u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a30u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5270a30u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4b0a; continue; }
  pc = 0x50b4ad2; continue;
 }
 0x50b4ad2 => {
  { let v: u32 = cpu.eax; cpu.w32(0x2400u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2404u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2408u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x240cu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2410u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x2414u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b4ad2; continue; }
  pc = 0x50b4b0a; continue;
 }
 0x50b4b0a => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x50b4b0d; continue;
 }
 0x50b4b0d => {
  { let v: u32 = cpu.eax; cpu.w32(0x2400u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4b0d; continue; }
  pc = 0x50b4b1b; continue;
 }
 0x50b4b1b => {
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.rf32(0x0u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(1) - cpu.st(0); cpu.set_st(1, v); }
  { let v = cpu.st(2); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = 0x8000u32; cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x52a4938u32), 0x52aa0a8u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b4b38; continue; }
  pc = 0x50b4b36; continue;
 }
 0x50b4b36 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  pc = 0x50b4b38; continue;
 }
 0x50b4b38 => {
  { let v = cpu.st(1) + cpu.st(0); cpu.set_st(1, v); cpu.fpop(); }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a20u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(2); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x5270a20u32)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v = cpu.fi32(cpu.st(0)); cpu.w32(0x5270a24u32, v); cpu.fpop(); }
  { let v = cpu.rf32(0x14u32.wrapping_add(cpu.ebx)); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x5270a24u32); cpu.ecx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(1) * cpu.st(0); cpu.set_st(1, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.ebp = v; }
  { let v = cpu.st(2) * cpu.st(0); cpu.set_st(2, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4938u32); cpu.edi = v; }
  { let v = cpu.st(0) + cpu.rf32(0x14u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4be4; continue; }
  pc = 0x50b4b83; continue;
 }
 0x50b4b83 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c04u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c08u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c0cu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x10u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c10u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0x14u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c14u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x18u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ebp, 0x6u32); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b4b83; continue; }
  pc = 0x50b4be4; continue;
 }
 0x50b4be4 => {
  { let v: u32 = cpu.ebp.wrapping_add(0x6u32); cpu.ebp = v; }
  pc = 0x50b4be7; continue;
 }
 0x50b4be7 => {
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edx, cpu.ecx); cpu.edx = v; }
  { let v: u32 = cpu.adc32(cpu.eax, cpu.ebx); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x3c00u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) + cpu.st(1); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4be7; continue; }
  pc = 0x50b4bfb; continue;
 }
 0x50b4bfb => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v: u32 = cpu.esp.wrapping_add(0x4u32); cpu.esp = v; }
  pc = 0x50b4c02; continue;
 }
 0x50b4c02 => {
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  if !cpu.zf { cpu.poll(); pc = 0x50b4a07; continue; }
  pc = 0x50b4c0e; continue;
 }
 0x50b4c0e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b4c0f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b4c0f); }
 let mut pc: u32 = 0x50b4c0f;
 loop { match pc {
 0x50b4c0f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4cad; continue; }
  pc = 0x50b4c29; continue;
 }
 0x50b4c29 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x52ae8b8u32, v); }
  pc = 0x50b4c58; continue;
 }
 0x50b4c58 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b4c5e); f_050b8b57(cpu); if cpu.returned_to != 0x50b4c5e { return; }
  if cpu.zf { pc = 0x50b4c9b; continue; }
  pc = 0x50b4c60; continue;
 }
 0x50b4c60 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8b8u32); cpu.eax = v; }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  pc = 0x50b4c73; continue;
 }
 0x50b4c73 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x50b4c93; continue; }
  pc = 0x50b4c7c; continue;
 }
 0x50b4c7c => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b4c89; continue; }
  pc = 0x50b4c86; continue;
 }
 0x50b4c86 => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, false); }
  pc = 0x50b4c89; continue;
 }
 0x50b4c89 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x50b4c73; continue; }
  pc = 0x50b4c8f; continue;
 }
 0x50b4c8f => {
  { cpu.stos(2, false); }
  cpu.poll(); pc = 0x50b4c73; continue;
 }
 0x50b4c93 => {
  if cpu.zf { pc = 0x50b4c9b; continue; }
  pc = 0x50b4c95; continue;
 }
 0x50b4c95 => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x50b4c73; continue;
 }
 0x50b4c9b => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b4c58; continue; }
  pc = 0x50b4cad; continue;
 }
 0x50b4cad => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b4cae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4cae); } r_050b4cae(cpu, 0x50b4cae); }
pub fn f_050b4db2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4db2); } r_050b4cae(cpu, 0x50b4db2); }
pub fn f_050b4dc2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4dc2); } r_050b4cae(cpu, 0x50b4dc2); }
pub fn f_050b4dd2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4dd2); } r_050b4cae(cpu, 0x50b4dd2); }
pub fn f_050b4de2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4de2); } r_050b4cae(cpu, 0x50b4de2); }
pub fn f_050b4e05(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4e05); } r_050b4cae(cpu, 0x50b4e05); }
pub fn f_050b4e15(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4e15); } r_050b4cae(cpu, 0x50b4e15); }
pub fn f_050b4e25(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4e25); } r_050b4cae(cpu, 0x50b4e25); }
pub fn f_050b4e35(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4e35); } r_050b4cae(cpu, 0x50b4e35); }
fn r_050b4cae(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b4cae => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b4e6d; continue; }
  pc = 0x50b4cc8; continue;
 }
 0x50b4cc8 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8b8u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.and32(cpu.eax, 0x7bef7befu32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52ae8b8u32, v); }
  pc = 0x50b4cfd; continue;
 }
 0x50b4cfd => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b4d03); f_050b93ca(cpu); if cpu.returned_to != 0x50b4d03 { return; }
  if cpu.zf { pc = 0x50b4e57; continue; }
  pc = 0x50b4d09; continue;
 }
 0x50b4d09 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8b8u32); cpu.ebx = v; }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4e44; continue; }
  pc = 0x50b4d27; continue;
 }
 0x50b4d27 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b4d44; continue; }
  pc = 0x50b4d2f; continue;
 }
 0x50b4d2f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50b4d44; continue;
 }
 0x50b4d44 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4da1; continue; }
  pc = 0x50b4d5a; continue;
 }
 0x50b4d5a => {
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b4d5a; continue; }
  pc = 0x50b4da1; continue;
 }
 0x50b4da1 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143f30u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b4db2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b4dc2; continue;
 }
 0x50b4dc2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b4dd2; continue;
 }
 0x50b4dd2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b4de2; continue;
 }
 0x50b4de2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b4d27; continue; }
  pc = 0x50b4e01; continue;
 }
 0x50b4e01 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4e44; continue; }
  pc = 0x50b4e03; continue;
 }
 0x50b4e03 => {
  if cpu.zf { pc = 0x50b4e57; continue; }
  pc = 0x50b4e05; continue;
 }
 0x50b4e05 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b4e15; continue;
 }
 0x50b4e15 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b4e25; continue;
 }
 0x50b4e25 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.shr32(cpu.eax, 1u32); cpu.eax = v; }
  { let v: u32 = cpu.eax & 0x7bef7befu32; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b4e35; continue;
 }
 0x50b4e35 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b4d27; continue; }
  pc = 0x50b4e42; continue;
 }
 0x50b4e42 => {
  if cpu.zf { pc = 0x50b4e57; continue; }
  pc = 0x50b4e44; continue;
 }
 0x50b4e44 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b4d27; continue; }
  pc = 0x50b4e55; continue;
 }
 0x50b4e55 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b4e44; continue; }
  pc = 0x50b4e57; continue;
 }
 0x50b4e57 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b4cfd; continue; }
  pc = 0x50b4e6d; continue;
 }
 0x50b4e6d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b4e6e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b4e6e); } r_050b4e6e(cpu, 0x50b4e6e); }
pub fn f_050b53cd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b53cd); } r_050b4e6e(cpu, 0x50b53cd); }
pub fn f_050b53ed(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b53ed); } r_050b4e6e(cpu, 0x50b53ed); }
pub fn f_050b540d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b540d); } r_050b4e6e(cpu, 0x50b540d); }
pub fn f_050b542d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b542d); } r_050b4e6e(cpu, 0x50b542d); }
pub fn f_050b544d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b544d); } r_050b4e6e(cpu, 0x50b544d); }
pub fn f_050b546d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b546d); } r_050b4e6e(cpu, 0x50b546d); }
pub fn f_050b548d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b548d); } r_050b4e6e(cpu, 0x50b548d); }
pub fn f_050b54ad(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b54ad); } r_050b4e6e(cpu, 0x50b54ad); }
pub fn f_050b54dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b54dd); } r_050b4e6e(cpu, 0x50b54dd); }
pub fn f_050b54fd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b54fd); } r_050b4e6e(cpu, 0x50b54fd); }
pub fn f_050b551d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b551d); } r_050b4e6e(cpu, 0x50b551d); }
pub fn f_050b553d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b553d); } r_050b4e6e(cpu, 0x50b553d); }
pub fn f_050b555d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b555d); } r_050b4e6e(cpu, 0x50b555d); }
pub fn f_050b557d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b557d); } r_050b4e6e(cpu, 0x50b557d); }
pub fn f_050b559d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b559d); } r_050b4e6e(cpu, 0x50b559d); }
pub fn f_050b55bd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b55bd); } r_050b4e6e(cpu, 0x50b55bd); }
pub fn f_050b55dd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b55dd); } r_050b4e6e(cpu, 0x50b55dd); }
fn r_050b4e6e(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b4e6e => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b5a50; continue; }
  pc = 0x50b4e88; continue;
 }
 0x50b4e88 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b4eae; continue;
 }
 0x50b4eae => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b4eb5); f_050b8b57(cpu); if cpu.returned_to != 0x50b4eb5 { return; }
  if cpu.zf { pc = 0x50b5a39; continue; }
  pc = 0x50b4ebb; continue;
 }
 0x50b4ebb => {
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b4ed1; continue; }
  pc = 0x50b4ec4; continue;
 }
 0x50b4ec4 => {
  cpu.push(0x50b4ec9); f_050b9df7(cpu); if cpu.returned_to != 0x50b4ec9 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b5a39; continue; }
  pc = 0x50b4ed1; continue;
 }
 0x50b4ed1 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52a497cu32, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf32(0x5143f24u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5143f20u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x5143f20u32); cpu.fpush(v); }
  { let v: u32 = 0x527aabcu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b4fd3; continue; }
  pc = 0x50b4f50; continue;
 }
 0x50b4f50 => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b4f5c; continue; }
  pc = 0x50b4f57; continue;
 }
 0x50b4f57 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x50b4f5c; continue;
 }
 0x50b4f5c => {
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.w32(0x5143f50u32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x527aabcu32); cpu.eax = v; }
  pc = 0x50b5046; continue;
 }
 0x50b4fd3 => {
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.w32(0x5143f50u32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x527aabcu32); cpu.eax = v; }
  pc = 0x50b598f; continue;
 }
 0x50b5046 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x50b55fb; continue; }
  pc = 0x50b505c; continue;
 }
 0x50b505c => {
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b51c4; continue; }
  pc = 0x50b506b; continue;
 }
 0x50b506b => {
  { let v = cpu.st(0) + cpu.rf32(0x529164cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529164cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffccu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd0u32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x529264cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdcu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b506b; continue; }
  pc = 0x50b51c4; continue;
 }
 0x50b51c4 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b532c; continue; }
  pc = 0x50b51d3; continue;
 }
 0x50b51d3 => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x50b532c; continue;
 }
 0x50b532c => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b53a1; continue; }
  pc = 0x50b533a; continue;
 }
 0x50b533a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143f54u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b53a1 => {
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143f54u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b53cd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b53ed; continue;
 }
 0x50b53ed => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b540d; continue;
 }
 0x50b540d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b542d; continue;
 }
 0x50b542d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b544d; continue;
 }
 0x50b544d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b546d; continue;
 }
 0x50b546d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b548d; continue;
 }
 0x50b548d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b54ad; continue;
 }
 0x50b54ad => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b5a2d; continue; }
  pc = 0x50b54ce; continue;
 }
 0x50b54ce => {
  { let v: u32 = cpu.r32(0x5143f50u32).wrapping_add(0x4u32); cpu.w32(0x5143f50u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x50b598f; continue;
 }
 0x50b54dd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b54fd; continue;
 }
 0x50b54fd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b551d; continue;
 }
 0x50b551d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b553d; continue;
 }
 0x50b553d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b555d; continue;
 }
 0x50b555d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b557d; continue;
 }
 0x50b557d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b559d; continue;
 }
 0x50b559d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b55bd; continue;
 }
 0x50b55bd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b55dd; continue;
 }
 0x50b55dd => {
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b5a2d; continue; }
  pc = 0x50b55ec; continue;
 }
 0x50b55ec => {
  { let v: u32 = cpu.r32(0x5143f50u32).wrapping_add(0x4u32); cpu.w32(0x5143f50u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x50b598f; continue;
 }
 0x50b55fb => {
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b5765; continue; }
  pc = 0x50b560a; continue;
 }
 0x50b560a => {
  { let v = cpu.st(0) + cpu.rf32(0x529164cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529164cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffc6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffcau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffceu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0) * cpu.rf32(0x529264cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffd6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffdau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b560a; continue; }
  pc = 0x50b5765; continue;
 }
 0x50b5765 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b58cf; continue; }
  pc = 0x50b5774; continue;
 }
 0x50b5774 => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffeau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffeeu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffau32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x50b58cf; continue;
 }
 0x50b58cf => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b5944; continue; }
  pc = 0x50b58dd; continue;
 }
 0x50b58dd => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143f54u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b5944 => {
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  if cpu.zf { cpu.poll(); pc = 0x50b55dd; continue; }
  pc = 0x50b596f; continue;
 }
 0x50b596f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143f54u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b598f => {
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b59a3; continue; }
  pc = 0x50b599e; continue;
 }
 0x50b599e => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x50b59a3; continue;
 }
 0x50b59a3 => {
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x5143f50u32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b5a2d; continue; }
  pc = 0x50b5a1e; continue;
 }
 0x50b5a1e => {
  { let v: u32 = cpu.r32(0x5143f50u32).wrapping_add(0x4u32); cpu.w32(0x5143f50u32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x50b5046; continue;
 }
 0x50b5a2d => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x50b5a39; continue;
 }
 0x50b5a39 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b4eae; continue; }
  pc = 0x50b5a50; continue;
 }
 0x50b5a50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b5a51(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5a51); } r_050b5a51(cpu, 0x50b5a51); }
pub fn f_050b5cfe(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5cfe); } r_050b5a51(cpu, 0x50b5cfe); }
pub fn f_050b5d26(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5d26); } r_050b5a51(cpu, 0x50b5d26); }
pub fn f_050b5d4e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5d4e); } r_050b5a51(cpu, 0x50b5d4e); }
pub fn f_050b5d76(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5d76); } r_050b5a51(cpu, 0x50b5d76); }
pub fn f_050b5d9e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5d9e); } r_050b5a51(cpu, 0x50b5d9e); }
pub fn f_050b5dc6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5dc6); } r_050b5a51(cpu, 0x50b5dc6); }
pub fn f_050b5dee(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5dee); } r_050b5a51(cpu, 0x50b5dee); }
pub fn f_050b5e16(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5e16); } r_050b5a51(cpu, 0x50b5e16); }
pub fn f_050b5e45(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5e45); } r_050b5a51(cpu, 0x50b5e45); }
pub fn f_050b5e6d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5e6d); } r_050b5a51(cpu, 0x50b5e6d); }
pub fn f_050b5e95(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5e95); } r_050b5a51(cpu, 0x50b5e95); }
pub fn f_050b5ebd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5ebd); } r_050b5a51(cpu, 0x50b5ebd); }
pub fn f_050b5ee5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5ee5); } r_050b5a51(cpu, 0x50b5ee5); }
pub fn f_050b5f0d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5f0d); } r_050b5a51(cpu, 0x50b5f0d); }
pub fn f_050b5f35(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5f35); } r_050b5a51(cpu, 0x50b5f35); }
pub fn f_050b5f5d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5f5d); } r_050b5a51(cpu, 0x50b5f5d); }
pub fn f_050b5fb8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5fb8); } r_050b5a51(cpu, 0x50b5fb8); }
pub fn f_050b5fc4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5fc4); } r_050b5a51(cpu, 0x50b5fc4); }
pub fn f_050b5fd0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5fd0); } r_050b5a51(cpu, 0x50b5fd0); }
pub fn f_050b5fdc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5fdc); } r_050b5a51(cpu, 0x50b5fdc); }
pub fn f_050b5fe8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5fe8); } r_050b5a51(cpu, 0x50b5fe8); }
pub fn f_050b5ff4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b5ff4); } r_050b5a51(cpu, 0x50b5ff4); }
pub fn f_050b6000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6000); } r_050b5a51(cpu, 0x50b6000); }
pub fn f_050b600c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b600c); } r_050b5a51(cpu, 0x50b600c); }
pub fn f_050b6018(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6018); } r_050b5a51(cpu, 0x50b6018); }
pub fn f_050b6024(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6024); } r_050b5a51(cpu, 0x50b6024); }
pub fn f_050b6030(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6030); } r_050b5a51(cpu, 0x50b6030); }
pub fn f_050b603c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b603c); } r_050b5a51(cpu, 0x50b603c); }
pub fn f_050b6048(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6048); } r_050b5a51(cpu, 0x50b6048); }
pub fn f_050b6054(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6054); } r_050b5a51(cpu, 0x50b6054); }
pub fn f_050b6060(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6060); } r_050b5a51(cpu, 0x50b6060); }
pub fn f_050b606c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b606c); } r_050b5a51(cpu, 0x50b606c); }
fn r_050b5a51(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b5a51 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b609c; continue; }
  pc = 0x50b5a6b; continue;
 }
 0x50b5a6b => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b5a91; continue;
 }
 0x50b5a91 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b5a98); f_050b8b57(cpu); if cpu.returned_to != 0x50b5a98 { return; }
  if cpu.zf { pc = 0x50b6085; continue; }
  pc = 0x50b5a9e; continue;
 }
 0x50b5a9e => {
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b5ab4; continue; }
  pc = 0x50b5aa7; continue;
 }
 0x50b5aa7 => {
  cpu.push(0x50b5aac); f_050b9df7(cpu); if cpu.returned_to != 0x50b5aac { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b6085; continue; }
  pc = 0x50b5ab4; continue;
 }
 0x50b5ab4 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a497cu32, v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52a4958u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b5f70; continue; }
  pc = 0x50b5b73; continue;
 }
 0x50b5b73 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b5b94; continue; }
  pc = 0x50b5b7b; continue;
 }
 0x50b5b7b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50b5b94; continue;
 }
 0x50b5b94 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b5ced; continue; }
  pc = 0x50b5bae; continue;
 }
 0x50b5bae => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b5bae; continue; }
  pc = 0x50b5ced; continue;
 }
 0x50b5ced => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143f98u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b5cfe => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5d26; continue;
 }
 0x50b5d26 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5d4e; continue;
 }
 0x50b5d4e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5d76; continue;
 }
 0x50b5d76 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5d9e; continue;
 }
 0x50b5d9e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5dc6; continue;
 }
 0x50b5dc6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5dee; continue;
 }
 0x50b5dee => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5e16; continue;
 }
 0x50b5e16 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b5b73; continue; }
  pc = 0x50b5e39; continue;
 }
 0x50b5e39 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b5f70; continue; }
  pc = 0x50b5e3f; continue;
 }
 0x50b5e3f => {
  if cpu.zf { pc = 0x50b6085; continue; }
  pc = 0x50b5e45; continue;
 }
 0x50b5e45 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5e6d; continue;
 }
 0x50b5e6d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5e95; continue;
 }
 0x50b5e95 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5ebd; continue;
 }
 0x50b5ebd => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5ee5; continue;
 }
 0x50b5ee5 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5f0d; continue;
 }
 0x50b5f0d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5f35; continue;
 }
 0x50b5f35 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b5f5d; continue;
 }
 0x50b5f5d => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b5b73; continue; }
  pc = 0x50b5f6a; continue;
 }
 0x50b5f6a => {
  if cpu.zf { pc = 0x50b6085; continue; }
  pc = 0x50b5f70; continue;
 }
 0x50b5f70 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b5fa7; continue; }
  pc = 0x50b5f84; continue;
 }
 0x50b5f84 => {
  { let v: u32 = cpu.r32(0x52a4958u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x50b5f91; continue;
 }
 0x50b5f91 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b5f91; continue; }
  pc = 0x50b5fa4; continue;
 }
 0x50b5fa4 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x50b5fa7; continue;
 }
 0x50b5fa7 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5143fd8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b5fb8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b5fc4; continue;
 }
 0x50b5fc4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b5fd0; continue;
 }
 0x50b5fd0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b5fdc; continue;
 }
 0x50b5fdc => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b5fe8; continue;
 }
 0x50b5fe8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b5ff4; continue;
 }
 0x50b5ff4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6000; continue;
 }
 0x50b6000 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b600c; continue;
 }
 0x50b600c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6018; continue;
 }
 0x50b6018 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6024; continue;
 }
 0x50b6024 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6030; continue;
 }
 0x50b6030 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b603c; continue;
 }
 0x50b603c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6048; continue;
 }
 0x50b6048 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6054; continue;
 }
 0x50b6054 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6060; continue;
 }
 0x50b6060 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b606c; continue;
 }
 0x50b606c => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b5b73; continue; }
  pc = 0x50b607f; continue;
 }
 0x50b607f => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b5f70; continue; }
  pc = 0x50b6085; continue;
 }
 0x50b6085 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b5a91; continue; }
  pc = 0x50b609c; continue;
 }
 0x50b609c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b609d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x50b609d); }
 let mut pc: u32 = 0x50b609d;
 loop { match pc {
 0x50b609d => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b6145; continue; }
  pc = 0x50b60b7; continue;
 }
 0x50b60b7 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8b8u32); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.or32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.ebx; cpu.w32(0x52ae8b8u32, v); }
  pc = 0x50b60e6; continue;
 }
 0x50b60e6 => {
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b60ec); f_050b98d9(cpu); if cpu.returned_to != 0x50b60ec { return; }
  if cpu.zf { pc = 0x50b6133; continue; }
  pc = 0x50b60ee; continue;
 }
 0x50b60ee => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8b8u32); cpu.eax = v; }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  pc = 0x50b6101; continue;
 }
 0x50b6101 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (cpu.sf == cpu.of) { pc = 0x50b612b; continue; }
  pc = 0x50b610a; continue;
 }
 0x50b610a => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x1u32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6116; continue; }
  pc = 0x50b6111; continue;
 }
 0x50b6111 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  cpu.poll(); pc = 0x50b6101; continue;
 }
 0x50b6116 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b6121; continue; }
  pc = 0x50b611e; continue;
 }
 0x50b611e => {
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  { cpu.stos(2, false); }
  pc = 0x50b6121; continue;
 }
 0x50b6121 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { cpu.poll(); pc = 0x50b6101; continue; }
  pc = 0x50b6127; continue;
 }
 0x50b6127 => {
  { cpu.stos(2, false); }
  cpu.poll(); pc = 0x50b6101; continue;
 }
 0x50b612b => {
  if cpu.zf { pc = 0x50b6133; continue; }
  pc = 0x50b612d; continue;
 }
 0x50b612d => {
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  cpu.poll(); pc = 0x50b6101; continue;
 }
 0x50b6133 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b60e6; continue; }
  pc = 0x50b6145; continue;
 }
 0x50b6145 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b6146(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6146); } r_050b6146(cpu, 0x50b6146); }
pub fn f_050b6400(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6400); } r_050b6146(cpu, 0x50b6400); }
pub fn f_050b6428(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6428); } r_050b6146(cpu, 0x50b6428); }
pub fn f_050b6450(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6450); } r_050b6146(cpu, 0x50b6450); }
pub fn f_050b6478(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6478); } r_050b6146(cpu, 0x50b6478); }
pub fn f_050b64a0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b64a0); } r_050b6146(cpu, 0x50b64a0); }
pub fn f_050b64c8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b64c8); } r_050b6146(cpu, 0x50b64c8); }
pub fn f_050b64f0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b64f0); } r_050b6146(cpu, 0x50b64f0); }
pub fn f_050b6518(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6518); } r_050b6146(cpu, 0x50b6518); }
pub fn f_050b6547(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6547); } r_050b6146(cpu, 0x50b6547); }
pub fn f_050b656f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b656f); } r_050b6146(cpu, 0x50b656f); }
pub fn f_050b6597(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6597); } r_050b6146(cpu, 0x50b6597); }
pub fn f_050b65bf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b65bf); } r_050b6146(cpu, 0x50b65bf); }
pub fn f_050b65e7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b65e7); } r_050b6146(cpu, 0x50b65e7); }
pub fn f_050b660f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b660f); } r_050b6146(cpu, 0x50b660f); }
pub fn f_050b6637(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6637); } r_050b6146(cpu, 0x50b6637); }
pub fn f_050b665f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b665f); } r_050b6146(cpu, 0x50b665f); }
pub fn f_050b66ba(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b66ba); } r_050b6146(cpu, 0x50b66ba); }
pub fn f_050b66c6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b66c6); } r_050b6146(cpu, 0x50b66c6); }
pub fn f_050b66d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b66d2); } r_050b6146(cpu, 0x50b66d2); }
pub fn f_050b66de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b66de); } r_050b6146(cpu, 0x50b66de); }
pub fn f_050b66ea(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b66ea); } r_050b6146(cpu, 0x50b66ea); }
pub fn f_050b66f6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b66f6); } r_050b6146(cpu, 0x50b66f6); }
pub fn f_050b6702(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6702); } r_050b6146(cpu, 0x50b6702); }
pub fn f_050b670e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b670e); } r_050b6146(cpu, 0x50b670e); }
pub fn f_050b671a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b671a); } r_050b6146(cpu, 0x50b671a); }
pub fn f_050b6726(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6726); } r_050b6146(cpu, 0x50b6726); }
pub fn f_050b6732(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6732); } r_050b6146(cpu, 0x50b6732); }
pub fn f_050b673e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b673e); } r_050b6146(cpu, 0x50b673e); }
pub fn f_050b674a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b674a); } r_050b6146(cpu, 0x50b674a); }
pub fn f_050b6756(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6756); } r_050b6146(cpu, 0x50b6756); }
pub fn f_050b6762(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6762); } r_050b6146(cpu, 0x50b6762); }
pub fn f_050b676e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b676e); } r_050b6146(cpu, 0x50b676e); }
fn r_050b6146(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b6146 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b679e; continue; }
  pc = 0x50b6160; continue;
 }
 0x50b6160 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b6186; continue;
 }
 0x50b6186 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b618d); f_050b98d9(cpu); if cpu.returned_to != 0x50b618d { return; }
  if cpu.zf { pc = 0x50b6787; continue; }
  pc = 0x50b6193; continue;
 }
 0x50b6193 => {
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b61a9; continue; }
  pc = 0x50b619c; continue;
 }
 0x50b619c => {
  cpu.push(0x50b61a1); f_050b9df7(cpu); if cpu.returned_to != 0x50b61a1 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b6787; continue; }
  pc = 0x50b61a9; continue;
 }
 0x50b61a9 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a497cu32, v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52a4958u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6672; continue; }
  pc = 0x50b6268; continue;
 }
 0x50b6268 => {
  { cpu.sub32(cpu.eax, 0xffffffffu32); }
  if (cpu.sf != cpu.of) { pc = 0x50b6275; continue; }
  pc = 0x50b626d; continue;
 }
 0x50b626d => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b665f; continue;
 }
 0x50b6275 => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b6296; continue; }
  pc = 0x50b627d; continue;
 }
 0x50b627d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50b6296; continue;
 }
 0x50b6296 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b63ef; continue; }
  pc = 0x50b62b0; continue;
 }
 0x50b62b0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b62b0; continue; }
  pc = 0x50b63ef; continue;
 }
 0x50b63ef => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5144018u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b6400 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6428; continue;
 }
 0x50b6428 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6450; continue;
 }
 0x50b6450 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6478; continue;
 }
 0x50b6478 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b64a0; continue;
 }
 0x50b64a0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b64c8; continue;
 }
 0x50b64c8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b64f0; continue;
 }
 0x50b64f0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6518; continue;
 }
 0x50b6518 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6268; continue; }
  pc = 0x50b653b; continue;
 }
 0x50b653b => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6672; continue; }
  pc = 0x50b6541; continue;
 }
 0x50b6541 => {
  if cpu.zf { pc = 0x50b6787; continue; }
  pc = 0x50b6547; continue;
 }
 0x50b6547 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b656f; continue;
 }
 0x50b656f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6597; continue;
 }
 0x50b6597 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b65bf; continue;
 }
 0x50b65bf => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b65e7; continue;
 }
 0x50b65e7 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b660f; continue;
 }
 0x50b660f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6637; continue;
 }
 0x50b6637 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b665f; continue;
 }
 0x50b665f => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6268; continue; }
  pc = 0x50b666c; continue;
 }
 0x50b666c => {
  if cpu.zf { pc = 0x50b6787; continue; }
  pc = 0x50b6672; continue;
 }
 0x50b6672 => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b66a9; continue; }
  pc = 0x50b6686; continue;
 }
 0x50b6686 => {
  { let v: u32 = cpu.r32(0x52a4958u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 4u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x50b6693; continue;
 }
 0x50b6693 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6693; continue; }
  pc = 0x50b66a6; continue;
 }
 0x50b66a6 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 4u32); cpu.ecx = v; }
  pc = 0x50b66a9; continue;
 }
 0x50b66a9 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5144058u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b66ba => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b66c6; continue;
 }
 0x50b66c6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b66d2; continue;
 }
 0x50b66d2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b66de; continue;
 }
 0x50b66de => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b66ea; continue;
 }
 0x50b66ea => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b66f6; continue;
 }
 0x50b66f6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6702; continue;
 }
 0x50b6702 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b670e; continue;
 }
 0x50b670e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b671a; continue;
 }
 0x50b671a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6726; continue;
 }
 0x50b6726 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6732; continue;
 }
 0x50b6732 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b673e; continue;
 }
 0x50b673e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b674a; continue;
 }
 0x50b674a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6756; continue;
 }
 0x50b6756 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b6762; continue;
 }
 0x50b6762 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b676e; continue;
 }
 0x50b676e => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6268; continue; }
  pc = 0x50b6781; continue;
 }
 0x50b6781 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b6672; continue; }
  pc = 0x50b6787; continue;
 }
 0x50b6787 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b6186; continue; }
  pc = 0x50b679e; continue;
 }
 0x50b679e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b679f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b679f); } r_050b679f(cpu, 0x50b679f); }
pub fn f_050b6a48(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6a48); } r_050b679f(cpu, 0x50b6a48); }
pub fn f_050b6a70(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6a70); } r_050b679f(cpu, 0x50b6a70); }
pub fn f_050b6a98(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6a98); } r_050b679f(cpu, 0x50b6a98); }
pub fn f_050b6ac0(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ac0); } r_050b679f(cpu, 0x50b6ac0); }
pub fn f_050b6ae8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ae8); } r_050b679f(cpu, 0x50b6ae8); }
pub fn f_050b6b10(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6b10); } r_050b679f(cpu, 0x50b6b10); }
pub fn f_050b6b38(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6b38); } r_050b679f(cpu, 0x50b6b38); }
pub fn f_050b6b60(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6b60); } r_050b679f(cpu, 0x50b6b60); }
pub fn f_050b6b7b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6b7b); } r_050b679f(cpu, 0x50b6b7b); }
pub fn f_050b6ba3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ba3); } r_050b679f(cpu, 0x50b6ba3); }
pub fn f_050b6bcb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6bcb); } r_050b679f(cpu, 0x50b6bcb); }
pub fn f_050b6bf3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6bf3); } r_050b679f(cpu, 0x50b6bf3); }
pub fn f_050b6c1b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6c1b); } r_050b679f(cpu, 0x50b6c1b); }
pub fn f_050b6c43(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6c43); } r_050b679f(cpu, 0x50b6c43); }
pub fn f_050b6c6b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6c6b); } r_050b679f(cpu, 0x50b6c6b); }
pub fn f_050b6c93(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6c93); } r_050b679f(cpu, 0x50b6c93); }
fn r_050b679f(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b679f => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b6caa; continue; }
  pc = 0x50b67b9; continue;
 }
 0x50b67b9 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b67df; continue;
 }
 0x50b67df => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b67f7; continue; }
  pc = 0x50b67ea; continue;
 }
 0x50b67ea => {
  cpu.push(0x50b67ef); f_050b9df7(cpu); if cpu.returned_to != 0x50b67ef { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b6c93; continue; }
  pc = 0x50b67f7; continue;
 }
 0x50b67f7 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.edi = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v: u32 = cpu.inc32(cpu.ecx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b6c93; continue; }
  pc = 0x50b6812; continue;
 }
 0x50b6812 => {
  { let v: u32 = cpu.ecx; cpu.w32(0x5144098u32, v); }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a497cu32, v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52a4958u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.r32(0x5144098u32); cpu.edx = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b68de; continue; }
  pc = 0x50b68c1; continue;
 }
 0x50b68c1 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b6c93; continue; }
  pc = 0x50b68de; continue;
 }
 0x50b68de => {
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6a37; continue; }
  pc = 0x50b68f8; continue;
 }
 0x50b68f8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xffffffecu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b68f8; continue; }
  pc = 0x50b6a37; continue;
 }
 0x50b6a37 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x514409cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b6a48 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6a70; continue;
 }
 0x50b6a70 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6a98; continue;
 }
 0x50b6a98 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6ac0; continue;
 }
 0x50b6ac0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6ae8; continue;
 }
 0x50b6ae8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6b10; continue;
 }
 0x50b6b10 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6b38; continue;
 }
 0x50b6b38 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6b60; continue;
 }
 0x50b6b60 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b6c93; continue;
 }
 0x50b6b7b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6ba3; continue;
 }
 0x50b6ba3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6bcb; continue;
 }
 0x50b6bcb => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6bf3; continue;
 }
 0x50b6bf3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6c1b; continue;
 }
 0x50b6c1b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6c43; continue;
 }
 0x50b6c43 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6c6b; continue;
 }
 0x50b6c6b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b6c93; continue;
 }
 0x50b6c93 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b67df; continue; }
  pc = 0x50b6caa; continue;
 }
 0x50b6caa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b6cab(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6cab); } r_050b6cab(cpu, 0x50b6cab); }
pub fn f_050b6de9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6de9); } r_050b6cab(cpu, 0x50b6de9); }
pub fn f_050b6dfd(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6dfd); } r_050b6cab(cpu, 0x50b6dfd); }
pub fn f_050b6e11(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6e11); } r_050b6cab(cpu, 0x50b6e11); }
pub fn f_050b6e25(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6e25); } r_050b6cab(cpu, 0x50b6e25); }
pub fn f_050b6e44(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6e44); } r_050b6cab(cpu, 0x50b6e44); }
pub fn f_050b6e58(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6e58); } r_050b6cab(cpu, 0x50b6e58); }
pub fn f_050b6e6c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6e6c); } r_050b6cab(cpu, 0x50b6e6c); }
pub fn f_050b6e80(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6e80); } r_050b6cab(cpu, 0x50b6e80); }
pub fn f_050b6ecf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ecf); } r_050b6cab(cpu, 0x50b6ecf); }
pub fn f_050b6ed3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ed3); } r_050b6cab(cpu, 0x50b6ed3); }
pub fn f_050b6ed7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ed7); } r_050b6cab(cpu, 0x50b6ed7); }
pub fn f_050b6edb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6edb); } r_050b6cab(cpu, 0x50b6edb); }
pub fn f_050b6edf(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6edf); } r_050b6cab(cpu, 0x50b6edf); }
pub fn f_050b6ee3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ee3); } r_050b6cab(cpu, 0x50b6ee3); }
pub fn f_050b6ee7(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6ee7); } r_050b6cab(cpu, 0x50b6ee7); }
pub fn f_050b6eeb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6eeb); } r_050b6cab(cpu, 0x50b6eeb); }
fn r_050b6cab(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b6cab => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b6f17; continue; }
  pc = 0x50b6cc5; continue;
 }
 0x50b6cc5 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  pc = 0x50b6ce1; continue;
 }
 0x50b6ce1 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b6ce8); f_050b8b57(cpu); if cpu.returned_to != 0x50b6ce8 { return; }
  if cpu.zf { pc = 0x50b6f00; continue; }
  pc = 0x50b6cee; continue;
 }
 0x50b6cee => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.eax); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x7800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x2400u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 31u32); cpu.edx = v; }
  { let v = cpu.ebp; cpu.idiv32(v); }
  { let v: u32 = cpu.eax; cpu.ecx = v; }
  { let v: u32 = cpu.shl32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x52ae8c4u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x2400u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.esi = v; }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6e8f; continue; }
  pc = 0x50b6d4a; continue;
 }
 0x50b6d4a => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b6d5f; continue; }
  pc = 0x50b6d52; continue;
 }
 0x50b6d52 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50b6d5f; continue;
 }
 0x50b6d5f => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6dd8; continue; }
  pc = 0x50b6d75; continue;
 }
 0x50b6d75 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6d75; continue; }
  pc = 0x50b6dd8; continue;
 }
 0x50b6dd8 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x51440dcu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b6de9 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x50b6dfd; continue;
 }
 0x50b6dfd => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x50b6e11; continue;
 }
 0x50b6e11 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x50b6e25; continue;
 }
 0x50b6e25 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6d4a; continue; }
  pc = 0x50b6e3c; continue;
 }
 0x50b6e3c => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6e8f; continue; }
  pc = 0x50b6e3e; continue;
 }
 0x50b6e3e => {
  if cpu.zf { pc = 0x50b6f00; continue; }
  pc = 0x50b6e44; continue;
 }
 0x50b6e44 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x50b6e58; continue;
 }
 0x50b6e58 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x50b6e6c; continue;
 }
 0x50b6e6c => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.stos(4, false); }
  pc = 0x50b6e80; continue;
 }
 0x50b6e80 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6d4a; continue; }
  pc = 0x50b6e8d; continue;
 }
 0x50b6e8d => {
  if cpu.zf { pc = 0x50b6f00; continue; }
  pc = 0x50b6e8f; continue;
 }
 0x50b6e8f => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b6ebe; continue; }
  pc = 0x50b6ea3; continue;
 }
 0x50b6ea3 => {
  { let v: u32 = cpu.ebp; cpu.eax = v; }
  { let v: u32 = cpu.shld32(cpu.eax, cpu.ecx, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x50b6eac; continue;
 }
 0x50b6eac => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.eax); cpu.ebx = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6eac; continue; }
  pc = 0x50b6ebb; continue;
 }
 0x50b6ebb => {
  { let v: u32 = cpu.ror32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x50b6ebe; continue;
 }
 0x50b6ebe => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x51440fcu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b6ecf => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6ed3; continue;
 }
 0x50b6ed3 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6ed7; continue;
 }
 0x50b6ed7 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6edb; continue;
 }
 0x50b6edb => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6edf; continue;
 }
 0x50b6edf => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6ee3; continue;
 }
 0x50b6ee3 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6ee7; continue;
 }
 0x50b6ee7 => {
  { let v: u32 = cpu.add32(cpu.esi, cpu.ecx); cpu.esi = v; }
  { let v: u32 = cpu.adc32(cpu.ebx, cpu.ebp); cpu.ebx = v; }
  pc = 0x50b6eeb; continue;
 }
 0x50b6eeb => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b6d4a; continue; }
  pc = 0x50b6efe; continue;
 }
 0x50b6efe => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b6e8f; continue; }
  pc = 0x50b6f00; continue;
 }
 0x50b6f00 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b6ce1; continue; }
  pc = 0x50b6f17; continue;
 }
 0x50b6f17 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b6f18(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b6f18); } r_050b6f18(cpu, 0x50b6f18); }
pub fn f_050b713c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b713c); } r_050b6f18(cpu, 0x50b713c); }
pub fn f_050b7157(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7157); } r_050b6f18(cpu, 0x50b7157); }
pub fn f_050b7172(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7172); } r_050b6f18(cpu, 0x50b7172); }
pub fn f_050b718d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b718d); } r_050b6f18(cpu, 0x50b718d); }
pub fn f_050b71a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b71a8); } r_050b6f18(cpu, 0x50b71a8); }
pub fn f_050b71c3(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b71c3); } r_050b6f18(cpu, 0x50b71c3); }
pub fn f_050b71de(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b71de); } r_050b6f18(cpu, 0x50b71de); }
pub fn f_050b71f9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b71f9); } r_050b6f18(cpu, 0x50b71f9); }
pub fn f_050b7254(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7254); } r_050b6f18(cpu, 0x50b7254); }
pub fn f_050b7260(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7260); } r_050b6f18(cpu, 0x50b7260); }
pub fn f_050b726c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b726c); } r_050b6f18(cpu, 0x50b726c); }
pub fn f_050b7278(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7278); } r_050b6f18(cpu, 0x50b7278); }
pub fn f_050b7284(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7284); } r_050b6f18(cpu, 0x50b7284); }
pub fn f_050b7290(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7290); } r_050b6f18(cpu, 0x50b7290); }
pub fn f_050b729c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b729c); } r_050b6f18(cpu, 0x50b729c); }
pub fn f_050b72a8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b72a8); } r_050b6f18(cpu, 0x50b72a8); }
fn r_050b6f18(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b6f18 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b72d8; continue; }
  pc = 0x50b6f32; continue;
 }
 0x50b6f32 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b6f58; continue;
 }
 0x50b6f58 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b6f5f); f_050b93ca(cpu); if cpu.returned_to != 0x50b6f5f { return; }
  if cpu.zf { pc = 0x50b72c1; continue; }
  pc = 0x50b6f65; continue;
 }
 0x50b6f65 => {
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b6f7b; continue; }
  pc = 0x50b6f6e; continue;
 }
 0x50b6f6e => {
  cpu.push(0x50b6f73); f_050b9df7(cpu); if cpu.returned_to != 0x50b6f73 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b72c1; continue; }
  pc = 0x50b6f7b; continue;
 }
 0x50b6f7b => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a497cu32, v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52a4958u32, v); }
  { let v: u8 = (cpu.edx as u8); cpu.ecx = (cpu.ecx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b720c; continue; }
  pc = 0x50b703a; continue;
 }
 0x50b703a => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b712b; continue; }
  pc = 0x50b7054; continue;
 }
 0x50b7054 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b706d; continue; }
  pc = 0x50b7069; continue;
 }
 0x50b7069 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b706d; continue;
 }
 0x50b706d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7086; continue; }
  pc = 0x50b7082; continue;
 }
 0x50b7082 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7086; continue;
 }
 0x50b7086 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b709f; continue; }
  pc = 0x50b709b; continue;
 }
 0x50b709b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b709f; continue;
 }
 0x50b709f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b70b8; continue; }
  pc = 0x50b70b4; continue;
 }
 0x50b70b4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b70b8; continue;
 }
 0x50b70b8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b70d1; continue; }
  pc = 0x50b70cd; continue;
 }
 0x50b70cd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b70d1; continue;
 }
 0x50b70d1 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b70ea; continue; }
  pc = 0x50b70e6; continue;
 }
 0x50b70e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.edi), v); }
  pc = 0x50b70ea; continue;
 }
 0x50b70ea => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7103; continue; }
  pc = 0x50b70ff; continue;
 }
 0x50b70ff => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7103; continue;
 }
 0x50b7103 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b711c; continue; }
  pc = 0x50b7118; continue;
 }
 0x50b7118 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b711c; continue;
 }
 0x50b711c => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b7054; continue; }
  pc = 0x50b712b; continue;
 }
 0x50b712b => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x514411cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b713c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7154; continue; }
  pc = 0x50b7151; continue;
 }
 0x50b7151 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7154; continue;
 }
 0x50b7154 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7157; continue;
 }
 0x50b7157 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b716f; continue; }
  pc = 0x50b716c; continue;
 }
 0x50b716c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b716f; continue;
 }
 0x50b716f => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7172; continue;
 }
 0x50b7172 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b718a; continue; }
  pc = 0x50b7187; continue;
 }
 0x50b7187 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b718a; continue;
 }
 0x50b718a => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b718d; continue;
 }
 0x50b718d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b71a5; continue; }
  pc = 0x50b71a2; continue;
 }
 0x50b71a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b71a5; continue;
 }
 0x50b71a5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b71a8; continue;
 }
 0x50b71a8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b71c0; continue; }
  pc = 0x50b71bd; continue;
 }
 0x50b71bd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b71c0; continue;
 }
 0x50b71c0 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b71c3; continue;
 }
 0x50b71c3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b71db; continue; }
  pc = 0x50b71d8; continue;
 }
 0x50b71d8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b71db; continue;
 }
 0x50b71db => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b71de; continue;
 }
 0x50b71de => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b71f6; continue; }
  pc = 0x50b71f3; continue;
 }
 0x50b71f3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b71f6; continue;
 }
 0x50b71f6 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b71f9; continue;
 }
 0x50b71f9 => {
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b703a; continue; }
  pc = 0x50b7206; continue;
 }
 0x50b7206 => {
  if cpu.zf { pc = 0x50b72c1; continue; }
  pc = 0x50b720c; continue;
 }
 0x50b720c => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b7243; continue; }
  pc = 0x50b7220; continue;
 }
 0x50b7220 => {
  { let v: u32 = cpu.r32(0x52a4958u32); cpu.eax = v; }
  { let v: u8 = ((cpu.ecx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x50b722d; continue;
 }
 0x50b722d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.eax); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b722d; continue; }
  pc = 0x50b7240; continue;
 }
 0x50b7240 => {
  { let v: u32 = cpu.ror32(cpu.ecx, 3u32); cpu.ecx = v; }
  pc = 0x50b7243; continue;
 }
 0x50b7243 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x514413cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b7254 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b7260; continue;
 }
 0x50b7260 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b726c; continue;
 }
 0x50b726c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b7278; continue;
 }
 0x50b7278 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b7284; continue;
 }
 0x50b7284 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b7290; continue;
 }
 0x50b7290 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b729c; continue;
 }
 0x50b729c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4958u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), ((cpu.ecx >> 8) as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b72a8; continue;
 }
 0x50b72a8 => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.edi = v; }
  { let v: u32 = cpu.edx.wrapping_add(0x4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b703a; continue; }
  pc = 0x50b72bb; continue;
 }
 0x50b72bb => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b720c; continue; }
  pc = 0x50b72c1; continue;
 }
 0x50b72c1 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b6f58; continue; }
  pc = 0x50b72d8; continue;
 }
 0x50b72d8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b72d9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b72d9); } r_050b72d9(cpu, 0x50b72d9); }
pub fn f_050b78ec(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b78ec); } r_050b72d9(cpu, 0x50b78ec); }
pub fn f_050b7903(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7903); } r_050b72d9(cpu, 0x50b7903); }
pub fn f_050b791a(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b791a); } r_050b72d9(cpu, 0x50b791a); }
pub fn f_050b7931(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7931); } r_050b72d9(cpu, 0x50b7931); }
pub fn f_050b7948(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7948); } r_050b72d9(cpu, 0x50b7948); }
pub fn f_050b795f(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b795f); } r_050b72d9(cpu, 0x50b795f); }
pub fn f_050b7976(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7976); } r_050b72d9(cpu, 0x50b7976); }
pub fn f_050b798d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b798d); } r_050b72d9(cpu, 0x50b798d); }
pub fn f_050b79a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b79a4); } r_050b72d9(cpu, 0x50b79a4); }
pub fn f_050b79bb(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b79bb); } r_050b72d9(cpu, 0x50b79bb); }
pub fn f_050b79d2(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b79d2); } r_050b72d9(cpu, 0x50b79d2); }
pub fn f_050b79e9(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b79e9); } r_050b72d9(cpu, 0x50b79e9); }
pub fn f_050b7a00(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7a00); } r_050b72d9(cpu, 0x50b7a00); }
pub fn f_050b7a17(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7a17); } r_050b72d9(cpu, 0x50b7a17); }
pub fn f_050b7a2e(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7a2e); } r_050b72d9(cpu, 0x50b7a2e); }
pub fn f_050b7a45(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7a45); } r_050b72d9(cpu, 0x50b7a45); }
pub fn f_050b7a5c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7a5c); } r_050b72d9(cpu, 0x50b7a5c); }
fn r_050b72d9(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b72d9 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b7b38; continue; }
  pc = 0x50b72f3; continue;
 }
 0x50b72f3 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b7319; continue;
 }
 0x50b7319 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b7320); f_050b93ca(cpu); if cpu.returned_to != 0x50b7320 { return; }
  if cpu.zf { pc = 0x50b7b21; continue; }
  pc = 0x50b7326; continue;
 }
 0x50b7326 => {
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b733c; continue; }
  pc = 0x50b732f; continue;
 }
 0x50b732f => {
  cpu.push(0x50b7334); f_050b9df7(cpu); if cpu.returned_to != 0x50b7334 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b7b21; continue; }
  pc = 0x50b733c; continue;
 }
 0x50b733c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v = cpu.rf32(0x3c00u32.wrapping_add(cpu.esi)); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.rf32(0x9000u32.wrapping_add(cpu.esi)); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.ebx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 2u32); cpu.eax = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 2u32); cpu.ebx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x52a497cu32, v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.ebp.wrapping_sub(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebp = v; }
  { let v: u32 = cpu.inc32(cpu.ebp); cpu.ebp = v; }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.rf32(0x5143f24u32); cpu.fpush(v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.ebp.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.rf32(0x5143f20u32); cpu.fpush(v); }
  { let v = cpu.rf32(0x5143f20u32); cpu.fpush(v); }
  { let v: u32 = 0x527aabcu32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax)); cpu.ecx = v; }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b743e; continue; }
  pc = 0x50b73bb; continue;
 }
 0x50b73bb => {
  { let v: u32 = cpu.neg32(cpu.ecx); cpu.ecx = v; }
  { cpu.sub32(cpu.ecx, 0x10u32); }
  if (cpu.sf != cpu.of) { pc = 0x50b73c7; continue; }
  pc = 0x50b73c2; continue;
 }
 0x50b73c2 => {
  { let v: u32 = 0x10u32; cpu.ecx = v; }
  pc = 0x50b73c7; continue;
 }
 0x50b73c7 => {
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.w32(0x514415cu32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x527aabcu32); cpu.eax = v; }
  pc = 0x50b74b1; continue;
 }
 0x50b743e => {
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = 0x527aabcu32; cpu.w32(0x514415cu32, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x527aabcu32); cpu.eax = v; }
  pc = 0x50b7a77; continue;
 }
 0x50b74b1 => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.eax.wrapping_mul(2)); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x40u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b7683; continue; }
  pc = 0x50b74ca; continue;
 }
 0x50b74ca => {
  { let v = cpu.st(0) + cpu.rf32(0x529164cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529164cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7511; continue; }
  pc = 0x50b750d; continue;
 }
 0x50b750d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7511; continue;
 }
 0x50b7511 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7526; continue; }
  pc = 0x50b7522; continue;
 }
 0x50b7522 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7526; continue;
 }
 0x50b7526 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b753b; continue; }
  pc = 0x50b7537; continue;
 }
 0x50b7537 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b753b; continue;
 }
 0x50b753b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7550; continue; }
  pc = 0x50b754c; continue;
 }
 0x50b754c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc6u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7550; continue;
 }
 0x50b7550 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7565; continue; }
  pc = 0x50b7561; continue;
 }
 0x50b7561 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffc8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7565; continue;
 }
 0x50b7565 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b757a; continue; }
  pc = 0x50b7576; continue;
 }
 0x50b7576 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffcau32.wrapping_add(cpu.edi), v); }
  pc = 0x50b757a; continue;
 }
 0x50b757a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b758f; continue; }
  pc = 0x50b758b; continue;
 }
 0x50b758b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffccu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b758f; continue;
 }
 0x50b758f => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b75a6; continue; }
  pc = 0x50b75a2; continue;
 }
 0x50b75a2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffceu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b75a6; continue;
 }
 0x50b75a6 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b75bd; continue; }
  pc = 0x50b75b9; continue;
 }
 0x50b75b9 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b75bd; continue;
 }
 0x50b75bd => {
  { let v = cpu.st(0) * cpu.rf32(0x529264cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b75d8; continue; }
  pc = 0x50b75d4; continue;
 }
 0x50b75d4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b75d8; continue;
 }
 0x50b75d8 => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b75ef; continue; }
  pc = 0x50b75eb; continue;
 }
 0x50b75eb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b75ef; continue;
 }
 0x50b75ef => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7606; continue; }
  pc = 0x50b7602; continue;
 }
 0x50b7602 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd6u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7606; continue;
 }
 0x50b7606 => {
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7621; continue; }
  pc = 0x50b761d; continue;
 }
 0x50b761d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffd8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7621; continue;
 }
 0x50b7621 => {
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b763c; continue; }
  pc = 0x50b7638; continue;
 }
 0x50b7638 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdau32.wrapping_add(cpu.edi), v); }
  pc = 0x50b763c; continue;
 }
 0x50b763c => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7653; continue; }
  pc = 0x50b764f; continue;
 }
 0x50b764f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdcu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7653; continue;
 }
 0x50b7653 => {
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b766e; continue; }
  pc = 0x50b766a; continue;
 }
 0x50b766a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffdeu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b766e; continue;
 }
 0x50b766e => {
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b74ca; continue; }
  pc = 0x50b7683; continue;
 }
 0x50b7683 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b784b; continue; }
  pc = 0x50b7692; continue;
 }
 0x50b7692 => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.edi); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(2))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b76e3; continue; }
  pc = 0x50b76df; continue;
 }
 0x50b76df => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b76e3; continue;
 }
 0x50b76e3 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b76f8; continue; }
  pc = 0x50b76f4; continue;
 }
 0x50b76f4 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b76f8; continue;
 }
 0x50b76f8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b770d; continue; }
  pc = 0x50b7709; continue;
 }
 0x50b7709 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b770d; continue;
 }
 0x50b770d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7722; continue; }
  pc = 0x50b771e; continue;
 }
 0x50b771e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe6u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7722; continue;
 }
 0x50b7722 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7737; continue; }
  pc = 0x50b7733; continue;
 }
 0x50b7733 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffe8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7737; continue;
 }
 0x50b7737 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b774c; continue; }
  pc = 0x50b7748; continue;
 }
 0x50b7748 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffeau32.wrapping_add(cpu.edi), v); }
  pc = 0x50b774c; continue;
 }
 0x50b774c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7761; continue; }
  pc = 0x50b775d; continue;
 }
 0x50b775d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffecu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7761; continue;
 }
 0x50b7761 => {
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7778; continue; }
  pc = 0x50b7774; continue;
 }
 0x50b7774 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xffffffeeu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7778; continue;
 }
 0x50b7778 => {
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b778f; continue; }
  pc = 0x50b778b; continue;
 }
 0x50b778b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b778f; continue;
 }
 0x50b778f => {
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b77ac; continue; }
  pc = 0x50b77a8; continue;
 }
 0x50b77a8 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff2u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b77ac; continue;
 }
 0x50b77ac => {
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b77c3; continue; }
  pc = 0x50b77bf; continue;
 }
 0x50b77bf => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b77c3; continue;
 }
 0x50b77c3 => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b77da; continue; }
  pc = 0x50b77d6; continue;
 }
 0x50b77d6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff6u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b77da; continue;
 }
 0x50b77da => {
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b77f5; continue; }
  pc = 0x50b77f1; continue;
 }
 0x50b77f1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b77f5; continue;
 }
 0x50b77f5 => {
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7810; continue; }
  pc = 0x50b780c; continue;
 }
 0x50b780c => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffau32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7810; continue;
 }
 0x50b7810 => {
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7827; continue; }
  pc = 0x50b7823; continue;
 }
 0x50b7823 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7827; continue;
 }
 0x50b7827 => {
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7842; continue; }
  pc = 0x50b783e; continue;
 }
 0x50b783e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xfffffffeu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7842; continue;
 }
 0x50b7842 => {
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x50b784b; continue;
 }
 0x50b784b => {
  { let v: u32 = cpu.edi.wrapping_sub(0x20u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x514415cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b78c0; continue; }
  pc = 0x50b7859; continue;
 }
 0x50b7859 => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5144160u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b78c0 => {
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x5144160u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b78ec => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7900; continue; }
  pc = 0x50b78fd; continue;
 }
 0x50b78fd => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7900; continue;
 }
 0x50b7900 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7903; continue;
 }
 0x50b7903 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7917; continue; }
  pc = 0x50b7914; continue;
 }
 0x50b7914 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7917; continue;
 }
 0x50b7917 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b791a; continue;
 }
 0x50b791a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b792e; continue; }
  pc = 0x50b792b; continue;
 }
 0x50b792b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b792e; continue;
 }
 0x50b792e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7931; continue;
 }
 0x50b7931 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7945; continue; }
  pc = 0x50b7942; continue;
 }
 0x50b7942 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7945; continue;
 }
 0x50b7945 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7948; continue;
 }
 0x50b7948 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b795c; continue; }
  pc = 0x50b7959; continue;
 }
 0x50b7959 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b795c; continue;
 }
 0x50b795c => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b795f; continue;
 }
 0x50b795f => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7973; continue; }
  pc = 0x50b7970; continue;
 }
 0x50b7970 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7973; continue;
 }
 0x50b7973 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7976; continue;
 }
 0x50b7976 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b798a; continue; }
  pc = 0x50b7987; continue;
 }
 0x50b7987 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b798a; continue;
 }
 0x50b798a => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b798d; continue;
 }
 0x50b798d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b79a1; continue; }
  pc = 0x50b799e; continue;
 }
 0x50b799e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b79a1; continue;
 }
 0x50b79a1 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b79a4; continue;
 }
 0x50b79a4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b79b8; continue; }
  pc = 0x50b79b5; continue;
 }
 0x50b79b5 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b79b8; continue;
 }
 0x50b79b8 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b79bb; continue;
 }
 0x50b79bb => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b79cf; continue; }
  pc = 0x50b79cc; continue;
 }
 0x50b79cc => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b79cf; continue;
 }
 0x50b79cf => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b79d2; continue;
 }
 0x50b79d2 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b79e6; continue; }
  pc = 0x50b79e3; continue;
 }
 0x50b79e3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b79e6; continue;
 }
 0x50b79e6 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b79e9; continue;
 }
 0x50b79e9 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b79fd; continue; }
  pc = 0x50b79fa; continue;
 }
 0x50b79fa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b79fd; continue;
 }
 0x50b79fd => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7a00; continue;
 }
 0x50b7a00 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7a14; continue; }
  pc = 0x50b7a11; continue;
 }
 0x50b7a11 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7a14; continue;
 }
 0x50b7a14 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7a17; continue;
 }
 0x50b7a17 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7a2b; continue; }
  pc = 0x50b7a28; continue;
 }
 0x50b7a28 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7a2b; continue;
 }
 0x50b7a2b => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7a2e; continue;
 }
 0x50b7a2e => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7a42; continue; }
  pc = 0x50b7a3f; continue;
 }
 0x50b7a3f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7a42; continue;
 }
 0x50b7a42 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7a45; continue;
 }
 0x50b7a45 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and16((cpu.eax as u16), (cpu.eax as u16)); }
  if cpu.zf { pc = 0x50b7a59; continue; }
  pc = 0x50b7a56; continue;
 }
 0x50b7a56 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7a59; continue;
 }
 0x50b7a59 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x50b7a5c; continue;
 }
 0x50b7a5c => {
  { let v: u32 = cpu.r32(0x514415cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b7b15; continue; }
  pc = 0x50b7a6b; continue;
 }
 0x50b7a6b => {
  { let v: u32 = cpu.r32(0x514415cu32).wrapping_add(0x4u32); cpu.w32(0x514415cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  pc = 0x50b7a77; continue;
 }
 0x50b7a77 => {
  { let v: u32 = cpu.r32(0x514415cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b7a8b; continue; }
  pc = 0x50b7a86; continue;
 }
 0x50b7a86 => {
  { let v: u32 = 0x10u32; cpu.eax = v; }
  pc = 0x50b7a8b; continue;
 }
 0x50b7a8b => {
  { let v = cpu.st(0) + cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v = cpu.st(3); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.rf32(0x529160cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.ror32(cpu.ecx, 14u32); cpu.ecx = v; }
  { let v = cpu.st(3) + cpu.st(0); cpu.set_st(3, v); cpu.fpop(); }
  { let v: u32 = cpu.ror32(cpu.edx, 14u32); cpu.edx = v; }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) / cpu.st(3); cpu.set_st(0, v); }
  { let v = cpu.eax; cpu.push(v); }
  { let v: u32 = cpu.r32(0x514415cu32); cpu.eax = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.eax))); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ecx); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.ecx as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.pop(); cpu.eax = v; }
  { let v = cpu.st(0) - cpu.st(2); cpu.set_st(2, v); }
  { let a = cpu.st(0); let b = cpu.st(2); cpu.set_st(0, b); cpu.set_st(2, a); }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.eax.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.fpush(v); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v = cpu.st(0) * cpu.st(6); cpu.set_st(0, v); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x514415cu32); cpu.eax = v; }
  { cpu.sub32(cpu.r32(0x4u32.wrapping_add(cpu.eax)), 0x0u32); }
  if cpu.zf { pc = 0x50b7b15; continue; }
  pc = 0x50b7b06; continue;
 }
 0x50b7b06 => {
  { let v: u32 = cpu.r32(0x514415cu32).wrapping_add(0x4u32); cpu.w32(0x514415cu32, v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.eax)); cpu.eax = v; }
  cpu.poll(); pc = 0x50b74b1; continue;
 }
 0x50b7b15 => {
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  { let v = cpu.st(0); cpu.set_st(0, v); cpu.fpop(); }
  pc = 0x50b7b21; continue;
 }
 0x50b7b21 => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b7319; continue; }
  pc = 0x50b7b38; continue;
 }
 0x50b7b38 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_050b7b39(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7b39); } r_050b7b39(cpu, 0x50b7b39); }
pub fn f_050b7eb5(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7eb5); } r_050b7b39(cpu, 0x50b7eb5); }
pub fn f_050b7f20(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7f20); } r_050b7b39(cpu, 0x50b7f20); }
pub fn f_050b7f8b(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7f8b); } r_050b7b39(cpu, 0x50b7f8b); }
pub fn f_050b7ff6(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b7ff6); } r_050b7b39(cpu, 0x50b7ff6); }
pub fn f_050b806d(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b806d); } r_050b7b39(cpu, 0x50b806d); }
pub fn f_050b80d8(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b80d8); } r_050b7b39(cpu, 0x50b80d8); }
pub fn f_050b8143(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b8143); } r_050b7b39(cpu, 0x50b8143); }
pub fn f_050b81ae(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b81ae); } r_050b7b39(cpu, 0x50b81ae); }
pub fn f_050b8214(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b8214); } r_050b7b39(cpu, 0x50b8214); }
pub fn f_050b822c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b822c); } r_050b7b39(cpu, 0x50b822c); }
pub fn f_050b8244(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b8244); } r_050b7b39(cpu, 0x50b8244); }
pub fn f_050b825c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b825c); } r_050b7b39(cpu, 0x50b825c); }
pub fn f_050b8274(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b8274); } r_050b7b39(cpu, 0x50b8274); }
pub fn f_050b828c(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b828c); } r_050b7b39(cpu, 0x50b828c); }
pub fn f_050b82a4(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b82a4); } r_050b7b39(cpu, 0x50b82a4); }
pub fn f_050b82bc(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x50b82bc); } r_050b7b39(cpu, 0x50b82bc); }
fn r_050b7b39(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x50b7b39 => {
  { let v: u32 = cpu.inc32(cpu.r32(0x52ae8b0u32)); cpu.w32(0x52ae8b0u32, v); }
  { let v: u32 = cpu.r32(0x52a4ca4u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x52a4ca0u32); cpu.ebx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, cpu.ebx); cpu.ecx = v; }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x50b82f6; continue; }
  pc = 0x50b7b53; continue;
 }
 0x50b7b53 => {
  { let v: u32 = 0x52a4ca8u32.wrapping_add(cpu.ebx.wrapping_mul(4)); cpu.esi = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a4950u32, v); }
  { let v: u32 = cpu.r32(0x5144844u32); cpu.eax = v; }
  { let v = cpu.ebx; cpu.imul32(v); }
  { let v: u32 = cpu.r32(0x5144840u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.eax); cpu.edi = v; }
  { let v: u32 = 0x0u32; cpu.w32(0x52ae8c8u32, v); }
  pc = 0x50b7b79; continue;
 }
 0x50b7b79 => {
  { let v = cpu.esi; cpu.push(v); }
  { let v = cpu.edi; cpu.push(v); }
  cpu.push(0x50b7b80); f_050b93ca(cpu); if cpu.returned_to != 0x50b7b80 { return; }
  if cpu.zf { pc = 0x50b82df; continue; }
  pc = 0x50b7b86; continue;
 }
 0x50b7b86 => {
  { cpu.sub32(cpu.r32(0x52ae8c8u32), 0x0u32); }
  if !cpu.zf { pc = 0x50b7b9c; continue; }
  pc = 0x50b7b8f; continue;
 }
 0x50b7b8f => {
  cpu.push(0x50b7b94); f_050b9df7(cpu); if cpu.returned_to != 0x50b7b94 { return; }
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if cpu.zf { pc = 0x50b82df; continue; }
  pc = 0x50b7b9c; continue;
 }
 0x50b7b9c => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esp)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x6000u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0xc00u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4974u32, v); }
  { let v = (cpu.r32(0x52a4974u32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x5400u32.wrapping_add(cpu.esi)); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.eax); cpu.ecx = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52ae8c8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 1u32); cpu.ebx = v; }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0xc00u32.wrapping_add(cpu.esi)); cpu.ebp = v; }
  { let v: u32 = cpu.rol32(cpu.ebp, 16u32); cpu.ebp = v; }
  { let v: u16 = (cpu.ebp as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v = cpu.st(0); cpu.wf32(0x52a4974u32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x6c00u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.r32(0x1800u32.wrapping_add(cpu.esi))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a497cu32, v); }
  { let v = (cpu.r32(0x52a497cu32) as i32 as f64); cpu.fpush(v); }
  { let v: u32 = cpu.r32(0x1800u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 16u32); cpu.eax = v; }
  { let v = cpu.st(0) * cpu.rf32(0x529260cu32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.set_st(0, v); }
  { let v: u32 = cpu.eax; cpu.esi = v; }
  { let v: u8 = (cpu.eax as u8); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v = cpu.st(0) + cpu.rf32(0x514425cu32); cpu.set_st(0, v); }
  { let v: u32 = cpu.r32(0x52a4974u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(0x4b400000u32); cpu.ecx = v; }
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x52a495cu32, v); }
  { let v = cpu.st(0); cpu.wf32(0x52a497cu32, v); cpu.fpop(); }
  { let v: u32 = cpu.r32(0x52a497cu32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(0x4b400000u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x52a4960u32, v); }
  { let v: u32 = 0x527aabcu32; cpu.w32(0x51441a4u32, v); }
  { let v: u32 = cpu.r32(0x51441a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b81cb; continue; }
  pc = 0x50b7c6a; continue;
 }
 0x50b7c6a => {
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x50b7ccf; continue; }
  pc = 0x50b7c72; continue;
 }
 0x50b7c72 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x50b7cc3; continue; }
  pc = 0x50b7c96; continue;
 }
 0x50b7c96 => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7cc3; continue;
 }
 0x50b7cc3 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x51441a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.eax); cpu.eax = v; }
  pc = 0x50b7ccf; continue;
 }
 0x50b7ccf => {
  { let v: u32 = cpu.neg32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b7ea4; continue; }
  pc = 0x50b7ce9; continue;
 }
 0x50b7ce9 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7d54; continue; }
  pc = 0x50b7d25; continue;
 }
 0x50b7d25 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7d54; continue;
 }
 0x50b7d54 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7dbf; continue; }
  pc = 0x50b7d90; continue;
 }
 0x50b7d90 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff4u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff4u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7dbf; continue;
 }
 0x50b7dbf => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7e2a; continue; }
  pc = 0x50b7dfb; continue;
 }
 0x50b7dfb => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffff8u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffff8u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7e2a; continue;
 }
 0x50b7e2a => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7e95; continue; }
  pc = 0x50b7e66; continue;
 }
 0x50b7e66 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0xfffffffcu32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0xfffffffcu32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7e95; continue;
 }
 0x50b7e95 => {
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b7ce9; continue; }
  pc = 0x50b7ea4; continue;
 }
 0x50b7ea4 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x51441a8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b7eb5 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7f1d; continue; }
  pc = 0x50b7ef1; continue;
 }
 0x50b7ef1 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7f1d; continue;
 }
 0x50b7f1d => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b7f20; continue;
 }
 0x50b7f20 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7f88; continue; }
  pc = 0x50b7f5c; continue;
 }
 0x50b7f5c => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7f88; continue;
 }
 0x50b7f88 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b7f8b; continue;
 }
 0x50b7f8b => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b7ff3; continue; }
  pc = 0x50b7fc7; continue;
 }
 0x50b7fc7 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b7ff3; continue;
 }
 0x50b7ff3 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b7ff6; continue;
 }
 0x50b7ff6 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.and32(cpu.ecx, 0xffffu32); cpu.ecx = v; }
  if cpu.zf { pc = 0x50b8047; continue; }
  pc = 0x50b801a; continue;
 }
 0x50b801a => {
  { let v: u32 = cpu.eax.wrapping_sub(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.edx); cpu.edx = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.edx = (cpu.edx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x10820u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x822u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b8047; continue;
 }
 0x50b8047 => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x51441a4u32).wrapping_add(0x4u32); cpu.w32(0x51441a4u32, v); }
  { let v: u32 = cpu.r32(0x51441a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b7c6a; continue; }
  pc = 0x50b8061; continue;
 }
 0x50b8061 => {
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b81cb; continue; }
  pc = 0x50b8067; continue;
 }
 0x50b8067 => {
  if cpu.zf { pc = 0x50b82df; continue; }
  pc = 0x50b806d; continue;
 }
 0x50b806d => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b80d5; continue; }
  pc = 0x50b80a9; continue;
 }
 0x50b80a9 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b80d5; continue;
 }
 0x50b80d5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b80d8; continue;
 }
 0x50b80d8 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b8140; continue; }
  pc = 0x50b8114; continue;
 }
 0x50b8114 => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b8140; continue;
 }
 0x50b8140 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b8143; continue;
 }
 0x50b8143 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0xfffffffeu32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.ebx).wrapping_add(cpu.ebx.wrapping_mul(1))); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { cpu.and32(cpu.ecx, cpu.ecx); }
  if cpu.zf { pc = 0x50b81ab; continue; }
  pc = 0x50b817f; continue;
 }
 0x50b817f => {
  { let v: u32 = cpu.rol32(cpu.ecx, 16u32); cpu.ecx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.eax, cpu.ecx); cpu.eax = v; }
  { let v: u32 = cpu.adc32(cpu.eax, 0x0u32); cpu.eax = v; }
  { let v: u32 = cpu.edx ^ cpu.ecx; cpu.edx = v; }
  { let v: u32 = cpu.edx ^ cpu.eax; cpu.edx = v; }
  { let v: u32 = cpu.edx & 0x8210821u32; cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.shr32(cpu.edx, 15u32); cpu.edx = v; }
  { let v: u32 = cpu.ecx | cpu.edx; cpu.ecx = v; }
  { let v: u32 = cpu.ecx & 0x1863u32; cpu.ecx = v; }
  { let v: u32 = cpu.eax | cpu.r32(0x5270a74u32.wrapping_add(cpu.ecx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x50b81ab; continue;
 }
 0x50b81ab => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x50b81ae; continue;
 }
 0x50b81ae => {
  { let v: u32 = cpu.r32(0x51441a4u32).wrapping_add(0x4u32); cpu.w32(0x51441a4u32, v); }
  { let v: u32 = cpu.r32(0x51441a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b7c6a; continue; }
  pc = 0x50b81c5; continue;
 }
 0x50b81c5 => {
  if cpu.zf { pc = 0x50b82df; continue; }
  pc = 0x50b81cb; continue;
 }
 0x50b81cb => {
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.edi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x52a4944u32, v); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x50b8203; continue; }
  pc = 0x50b81df; continue;
 }
 0x50b81df => {
  { let v: u32 = cpu.r32(0x52a495cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x52a4960u32); cpu.edx = v; }
  { let v: u32 = cpu.rol32(cpu.eax, 3u32); cpu.eax = v; }
  { let v: u32 = cpu.rol32(cpu.edx, 3u32); cpu.edx = v; }
  pc = 0x50b81f0; continue;
 }
 0x50b81f0 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.eax); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), (cpu.eax as u8)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.edx); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), (cpu.edx as u8)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.edi.wrapping_add(0x10u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x52a4944u32)); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b81f0; continue; }
  pc = 0x50b8203; continue;
 }
 0x50b8203 => {
  { let v: u32 = cpu.edi.wrapping_sub(0x10u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.eax = v; }
  { let v: u32 = cpu.sub32(cpu.eax, cpu.edi); cpu.eax = v; }
  cpu.poll(); pc = cpu.r32(0x51441c8u32.wrapping_add(cpu.eax).wrapping_add(cpu.eax.wrapping_mul(1))); continue;
 }
 0x50b8214 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b822c; continue;
 }
 0x50b822c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b8244; continue;
 }
 0x50b8244 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b825c; continue;
 }
 0x50b825c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b8274; continue;
 }
 0x50b8274 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b828c; continue;
 }
 0x50b828c => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b82a4; continue;
 }
 0x50b82a4 => {
  { let v: u32 = cpu.add32(cpu.ebp, cpu.r32(0x52a495cu32)); cpu.ebp = v; }
  { let v: u8 = cpu.adc8((cpu.ebx as u8), cpu.r8(0x52a495cu32)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.esi, cpu.r32(0x52a4960u32)); cpu.esi = v; }
  { let v: u8 = cpu.adc8(((cpu.ebx >> 8) as u8), cpu.r8(0x52a4960u32)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x50b82bc; continue;
 }
 0x50b82bc => {
  { let v: u32 = cpu.r32(0x52a4944u32); cpu.edi = v; }
  { let v: u32 = cpu.r32(0x51441a4u32).wrapping_add(0x4u32); cpu.w32(0x51441a4u32, v); }
  { let v: u32 = cpu.r32(0x51441a4u32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx)); cpu.eax = v; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x50b7c6a; continue; }
  pc = 0x50b82d9; continue;
 }
 0x50b82d9 => {
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x50b81cb; continue; }
  pc = 0x50b82df; continue;
 }
 0x50b82df => {
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.r32(0x5144844u32)); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.r32(0x52a4950u32)); cpu.w32(0x52a4950u32, v); }
  if !cpu.zf { cpu.poll(); pc = 0x50b7b79; continue; }
  pc = 0x50b82f6; continue;
 }
 0x50b82f6 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
