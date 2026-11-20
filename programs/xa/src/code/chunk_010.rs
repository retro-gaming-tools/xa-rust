#![allow(warnings)]
use recomp_runtime::Cpu;
use super::*;

pub fn f_03120c24(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3120c24); }
 let mut pc: u32 = 0x3120c24;
 loop { match pc {
 0x3120c24 => {
  { let v: u32 = cpu.ecx & cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.edx.wrapping_mul(1))); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.esi))); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) | 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.r32(0x0u32.wrapping_add(cpu.ecx))); cpu.ebx = v; }
  { let v: u8 = (cpu.eax as u8) | 0x12u8; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.edx, cpu.r32(0x0u32.wrapping_add(cpu.esp).wrapping_add(cpu.ecx.wrapping_mul(1)))); cpu.edx = v; }
  { let v: u8 = cpu.adc8((cpu.eax as u8), cpu.r8(0x0u32.wrapping_add(cpu.ebx))); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  cpu.call(0x3120c34); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0312158c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x312158c); }
 let mut pc: u32 = 0x312158c;
 loop { match pc {
 0x312158c => {
  { let v: u16 = (cpu.eax as u16); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.shl32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u16 = (cpu.ebx as u16); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if cpu.zf { pc = 0x31215a0; continue; }
  pc = 0x312159d; continue;
 }
 0x312159d => {
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  pc = 0x31215a0; continue;
 }
 0x31215a0 => {
  { let v: u32 = cpu.shr32(cpu.ecx, 1u32); cpu.ecx = v; }
  { cpu.stos(4, true); }
  if !cpu.cf { pc = 0x31215a8; continue; }
  pc = 0x31215a6; continue;
 }
 0x31215a6 => {
  { cpu.stos(2, false); }
  pc = 0x31215a8; continue;
 }
 0x31215a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_031215a9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x31215a9); }
 let mut pc: u32 = 0x31215a9;
 loop { match pc {
 0x31215a9 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.edx; cpu.ebp = v; }
  pc = 0x31215b0; continue;
 }
 0x31215b0 => {
  { let v: u32 = cpu.ecx; cpu.edx = v; }
  pc = 0x31215b2; continue;
 }
 0x31215b2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.edi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.esi.wrapping_mul(1))); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { cpu.stos(2, false); }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31215b2; continue; }
  pc = 0x31215be; continue;
 }
 0x31215be => {
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31215b0; continue; }
  pc = 0x31215c3; continue;
 }
 0x31215c3 => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_031217df(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x31217df); }
 let mut pc: u32 = 0x31217df;
 loop { match pc {
 0x31217df => {
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.eax.wrapping_add(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.ebp); cpu.ebp = v; }
  { cpu.and32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x3121817; continue; }
  pc = 0x31217f5; continue;
 }
 0x31217f5 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x3121807; continue; }
  pc = 0x31217f9; continue;
 }
 0x31217f9 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x31217f9; continue; }
  pc = 0x3121804; continue;
 }
 0x3121804 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x3121807 => {
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3121807; continue; }
  pc = 0x3121814; continue;
 }
 0x3121814 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x3121817 => {
  { let v: u32 = cpu.shr32(cpu.ebx, 1u32); cpu.ebx = v; }
  if cpu.cf { pc = 0x312182e; continue; }
  pc = 0x312181b; continue;
 }
 0x312181b => {
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x312181c; continue;
 }
 0x312181c => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { cpu.movs(2, false); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x312181c; continue; }
  pc = 0x312182b; continue;
 }
 0x312182b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x312182e => {
  { cpu.movs(2, false); }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { cpu.movs(4, true); }
  { let v: u32 = cpu.esi.wrapping_add(cpu.eax); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.edx); cpu.edx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x312182e; continue; }
  pc = 0x312183b; continue;
 }
 0x312183b => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0312183e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x312183e); }
 let mut pc: u32 = 0x312183e;
 loop { match pc {
 0x312183e => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.esp; cpu.ebp = v; }
  { let v = cpu.ebp; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u32 = cpu.eax; cpu.w32(0x3300050u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x330004cu32, v); }
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = v; }
  { let v: u32 = cpu.shr32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u16 = cpu.r16(0x8u32.wrapping_add(cpu.ebp)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ecx; cpu.ebp = v; }
  { let v: u16 = (cpu.segments[3] as u16); cpu.ecx = (cpu.ecx & 0xffff0000) | v as u32; }
  { let v = (cpu.ecx as u16); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.edx.wrapping_add(cpu.edx); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  pc = 0x3121a55; continue;
 }
 0x3121869 => {
  { let v: u32 = cpu.r32(0x330004cu32); cpu.ecx = v; }
  pc = 0x3121938; continue;
 }
 0x3121874 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x312187e; continue; }
  pc = 0x312187b; continue;
 }
 0x312187b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x312187e; continue;
 }
 0x312187e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x312188a; continue; }
  pc = 0x3121886; continue;
 }
 0x3121886 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x312188a; continue;
 }
 0x312188a => {
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121896; continue; }
  pc = 0x3121892; continue;
 }
 0x3121892 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x4u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121896; continue;
 }
 0x3121896 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218a2; continue; }
  pc = 0x312189e; continue;
 }
 0x312189e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x6u32.wrapping_add(cpu.edi), v); }
  pc = 0x31218a2; continue;
 }
 0x31218a2 => {
  { let v: u32 = cpu.r32(0x8u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218ae; continue; }
  pc = 0x31218aa; continue;
 }
 0x31218aa => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x8u32.wrapping_add(cpu.edi), v); }
  pc = 0x31218ae; continue;
 }
 0x31218ae => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218ba; continue; }
  pc = 0x31218b6; continue;
 }
 0x31218b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xau32.wrapping_add(cpu.edi), v); }
  pc = 0x31218ba; continue;
 }
 0x31218ba => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218c6; continue; }
  pc = 0x31218c2; continue;
 }
 0x31218c2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xcu32.wrapping_add(cpu.edi), v); }
  pc = 0x31218c6; continue;
 }
 0x31218c6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218d2; continue; }
  pc = 0x31218ce; continue;
 }
 0x31218ce => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0xeu32.wrapping_add(cpu.edi), v); }
  pc = 0x31218d2; continue;
 }
 0x31218d2 => {
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218de; continue; }
  pc = 0x31218da; continue;
 }
 0x31218da => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x10u32.wrapping_add(cpu.edi), v); }
  pc = 0x31218de; continue;
 }
 0x31218de => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218ea; continue; }
  pc = 0x31218e6; continue;
 }
 0x31218e6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x12u32.wrapping_add(cpu.edi), v); }
  pc = 0x31218ea; continue;
 }
 0x31218ea => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31218f6; continue; }
  pc = 0x31218f2; continue;
 }
 0x31218f2 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x14u32.wrapping_add(cpu.edi), v); }
  pc = 0x31218f6; continue;
 }
 0x31218f6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121902; continue; }
  pc = 0x31218fe; continue;
 }
 0x31218fe => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x16u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121902; continue;
 }
 0x3121902 => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x312190e; continue; }
  pc = 0x312190a; continue;
 }
 0x312190a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x18u32.wrapping_add(cpu.edi), v); }
  pc = 0x312190e; continue;
 }
 0x312190e => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x312191a; continue; }
  pc = 0x3121916; continue;
 }
 0x3121916 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1au32.wrapping_add(cpu.edi), v); }
  pc = 0x312191a; continue;
 }
 0x312191a => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121926; continue; }
  pc = 0x3121922; continue;
 }
 0x3121922 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1cu32.wrapping_add(cpu.edi), v); }
  pc = 0x3121926; continue;
 }
 0x3121926 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121932; continue; }
  pc = 0x312192e; continue;
 }
 0x312192e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x1eu32.wrapping_add(cpu.edi), v); }
  pc = 0x3121932; continue;
 }
 0x3121932 => {
  { let v: u32 = cpu.esi.wrapping_add(0x20u32); cpu.esi = v; }
  { let v: u32 = cpu.edi.wrapping_add(0x20u32); cpu.edi = v; }
  pc = 0x3121938; continue;
 }
 0x3121938 => {
  { let v: u32 = cpu.sub32(cpu.ecx, 0x10u32); cpu.ecx = v; }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3121874; continue; }
  pc = 0x3121941; continue;
 }
 0x3121941 => {
  { let v: u32 = cpu.ecx.wrapping_add(0x10u32); cpu.ecx = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x312194d; continue;
 }
 0x312194d => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121956; continue; }
  pc = 0x3121953; continue;
 }
 0x3121953 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121956; continue;
 }
 0x3121956 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121962; continue; }
  pc = 0x312195e; continue;
 }
 0x312195e => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121962; continue;
 }
 0x3121962 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x312196e; continue;
 }
 0x312196e => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121977; continue; }
  pc = 0x3121974; continue;
 }
 0x3121974 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121977; continue;
 }
 0x3121977 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121983; continue; }
  pc = 0x312197f; continue;
 }
 0x312197f => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121983; continue;
 }
 0x3121983 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x312198f; continue;
 }
 0x312198f => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121998; continue; }
  pc = 0x3121995; continue;
 }
 0x3121995 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121998; continue;
 }
 0x3121998 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219a4; continue; }
  pc = 0x31219a0; continue;
 }
 0x31219a0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219a4; continue;
 }
 0x31219a4 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x31219b0; continue;
 }
 0x31219b0 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219b9; continue; }
  pc = 0x31219b6; continue;
 }
 0x31219b6 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219b9; continue;
 }
 0x31219b9 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219c5; continue; }
  pc = 0x31219c1; continue;
 }
 0x31219c1 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219c5; continue;
 }
 0x31219c5 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x31219cd; continue;
 }
 0x31219cd => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219d6; continue; }
  pc = 0x31219d3; continue;
 }
 0x31219d3 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219d6; continue;
 }
 0x31219d6 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219e2; continue; }
  pc = 0x31219de; continue;
 }
 0x31219de => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219e2; continue;
 }
 0x31219e2 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x31219ea; continue;
 }
 0x31219ea => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219f3; continue; }
  pc = 0x31219f0; continue;
 }
 0x31219f0 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219f3; continue;
 }
 0x31219f3 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x31219ff; continue; }
  pc = 0x31219fb; continue;
 }
 0x31219fb => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x31219ff; continue;
 }
 0x31219ff => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x3121a07; continue;
 }
 0x3121a07 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121a10; continue; }
  pc = 0x3121a0d; continue;
 }
 0x3121a0d => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121a10; continue;
 }
 0x3121a10 => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121a1c; continue; }
  pc = 0x3121a18; continue;
 }
 0x3121a18 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121a1c; continue;
 }
 0x3121a1c => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.sub32(cpu.ecx, 0x2u32); cpu.ecx = v; }
  if (cpu.sf != cpu.of) { pc = 0x3121a3c; continue; }
  pc = 0x3121a24; continue;
 }
 0x3121a24 => {
  { cpu.lods(4, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121a2d; continue; }
  pc = 0x3121a2a; continue;
 }
 0x3121a2a => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121a2d; continue;
 }
 0x3121a2d => {
  { let v: u32 = cpu.shr32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121a39; continue; }
  pc = 0x3121a35; continue;
 }
 0x3121a35 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121a39; continue;
 }
 0x3121a39 => {
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  pc = 0x3121a3c; continue;
 }
 0x3121a3c => {
  { cpu.and32(cpu.ecx, 0x1u32); }
  if cpu.zf { pc = 0x3121a51; continue; }
  pc = 0x3121a44; continue;
 }
 0x3121a44 => {
  { cpu.lods(2, false); }
  { cpu.sub16((cpu.eax as u16), (cpu.ebx as u16)); }
  if cpu.zf { pc = 0x3121a4e; continue; }
  pc = 0x3121a4b; continue;
 }
 0x3121a4b => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  pc = 0x3121a4e; continue;
 }
 0x3121a4e => {
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  pc = 0x3121a51; continue;
 }
 0x3121a51 => {
  { let v: u32 = cpu.esi.wrapping_add(cpu.edx); cpu.esi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebp); cpu.edi = v; }
  pc = 0x3121a55; continue;
 }
 0x3121a55 => {
  { let v: u32 = cpu.dec32(cpu.r32(0x3300050u32)); cpu.w32(0x3300050u32, v); }
  if !cpu.sf { cpu.poll(); pc = 0x3121869; continue; }
  pc = 0x3121a61; continue;
 }
 0x3121a61 => {
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { cpu.esp = cpu.ebp; cpu.ebp = cpu.pop(); }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03121a65(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3121a65); }
 let mut pc: u32 = 0x3121a65;
 loop { match pc {
 0x3121a65 => {
  { let v = cpu.ebp; cpu.push(v); }
  { let v: u32 = cpu.edi; cpu.w32(0x3300058u32, v); }
  { let v: u32 = cpu.ecx.wrapping_add(cpu.ecx); cpu.ecx = v; }
  { let v: u32 = cpu.ecx; cpu.w32(0x3300054u32, v); }
  { let v: u32 = cpu.eax ^ cpu.eax; cpu.eax = v; }
  { let v: u32 = cpu.ebx; cpu.ecx = v; }
  { let v: u32 = cpu.imul2_32(cpu.ecx, cpu.r32(0x3300054u32)); cpu.ecx = v; }
  { let v: u32 = cpu.esi ^ cpu.esi; cpu.esi = v; }
  { let v: u32 = cpu.ebx; cpu.ebp = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  pc = 0x3121a84; continue;
 }
 0x3121a84 => {
  { cpu.and32(cpu.ebx, cpu.ebx); }
  if !cpu.sf { pc = 0x3121a93; continue; }
  pc = 0x3121a88; continue;
 }
 0x3121a88 => {
  { let v: u32 = cpu.dec32(cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.ecx.wrapping_sub(cpu.r32(0x3300054u32)); cpu.ecx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_add(cpu.ebp); cpu.ebx = v; }
  pc = 0x3121a93; continue;
 }
 0x3121a93 => {
  { let v: u32 = cpu.r32(0x3300058u32); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.ecx); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.neg32(cpu.esi); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.esi.wrapping_mul(2)), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.ecx); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)), v); }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)), v); }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.edi.wrapping_sub(cpu.eax); cpu.edi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)), v); }
  { let v: u32 = cpu.neg32(cpu.ebp); cpu.ebp = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.ebp.wrapping_mul(2)), v); }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.ebx.wrapping_sub(cpu.esi); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_add(cpu.r32(0x3300054u32)); cpu.eax = v; }
  { cpu.sub32(cpu.esi, cpu.ebp); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x3121a84; continue; }
  pc = 0x3121adf; continue;
 }
 0x3121adf => {
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03121ae1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3121ae1); }
 let mut pc: u32 = 0x3121ae1;
 loop { match pc {
 0x3121ae1 => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(cpu.eax); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.edi, cpu.ebx); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if (!cpu.zf && cpu.sf == cpu.of) { cpu.poll(); pc = 0x3121ae1; continue; }
  pc = 0x3121aeb; continue;
 }
 0x3121aeb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03130000(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3130000); } r_03130000(cpu, 0x3130000); }
pub fn f_03130008(cpu: &mut Cpu) { if cpu.profiling { cpu.hit(0x3130008); } r_03130000(cpu, 0x3130008); }
fn r_03130000(cpu: &mut Cpu, start: u32) {
 let mut pc: u32 = start;
 loop { match pc {
 0x3130000 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3])).wrapping_add((cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.bases[3]), v); }
  pc = 0x3130008; continue;
 }
 0x3130008 => {
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v = cpu.esi; cpu.set_segment(3, v as u16); }
  { cpu.sub32(cpu.edi, 0x0u32); }
  if !cpu.zf { pc = 0x3130014; continue; }
  pc = 0x3130011; continue;
 }
 0x3130011 => {
  cpu.flat_ds(0x3130011); cpu.push(0x3130016); cpu.call(0x2960051); if cpu.returned_to != 0x3130016 { return; }
  { let v: u32 = cpu.r32(0x3u32.wrapping_add(cpu.ebp)).wrapping_add(cpu.esi); cpu.w32(0x3u32.wrapping_add(cpu.ebp), v); }
  pc = 0x3130019; continue;
 }
 0x3130014 => {
  { cpu.sub32(cpu.edi, 0x1u32); }
  if !cpu.zf { pc = 0x313001c; continue; }
  pc = 0x3130019; continue;
 }
 0x3130019 => {
  cpu.flat_ds(0x3130019); cpu.push(0x313001e); cpu.call(0x2960065); if cpu.returned_to != 0x313001e { return; }
  { let v: u8 = ((cpu.edx >> 8) as u8).wrapping_add(cpu.r8(0x3u32.wrapping_add(cpu.ebp))); cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); }
  pc = 0x3130021; continue;
 }
 0x313001c => {
  { cpu.sub32(cpu.edi, 0x2u32); }
  if !cpu.zf { pc = 0x3130024; continue; }
  pc = 0x3130021; continue;
 }
 0x3130021 => {
  cpu.flat_ds(0x3130021); cpu.push(0x3130026); cpu.call(0x2960073); if cpu.returned_to != 0x3130026 { return; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x3u32.wrapping_add(cpu.ebp))); cpu.esi = v; }
  pc = 0x3130029; continue;
 }
 0x3130024 => {
  { cpu.sub32(cpu.edi, 0x3u32); }
  if !cpu.zf { pc = 0x313002c; continue; }
  pc = 0x3130029; continue;
 }
 0x3130029 => {
  cpu.flat_ds(0x3130029); cpu.push(0x313002e); cpu.call(0x2960079); if cpu.returned_to != 0x313002e { return; }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x75u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp.wrapping_add(cpu.eax); cpu.ebp = v; }
  { let v = cpu.edi; cpu.push(v); }
  { let v: u8 = cpu.add8(cpu.r8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37505ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x3130039; continue;
 }
 0x313002c => {
  { cpu.sub32(cpu.edi, 0x4u32); }
  if !cpu.zf { pc = 0x3130034; continue; }
  pc = 0x3130031; continue;
 }
 0x3130031 => {
  cpu.flat_ds(0x3130031); cpu.push(0x3130036); cpu.call(0x296008d); if cpu.returned_to != 0x3130036 { return; }
  { let v: u32 = cpu.eax.wrapping_add(0x55e80375u32); cpu.eax = v; }
  { let v: u8 = cpu.add8(cpu.r8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x37506ffu32.wrapping_add(cpu.ebx).wrapping_add(cpu.bases[3]), v); }
  pc = 0x3130041; continue;
 }
 0x3130034 => {
  { cpu.sub32(cpu.edi, 0x5u32); }
  if !cpu.zf { pc = 0x313003c; continue; }
  pc = 0x3130039; continue;
 }
 0x3130039 => {
  cpu.flat_ds(0x3130039); cpu.push(0x313003e); cpu.call(0x2960093); if cpu.returned_to != 0x313003e { return; }
  { cpu.push(cpu.segments[0] as u32); }
  if !cpu.zf { pc = 0x3130044; continue; }
  pc = 0x3130041; continue;
 }
 0x313003c => {
  { cpu.sub32(cpu.edi, 0x6u32); }
  if !cpu.zf { pc = 0x3130044; continue; }
  pc = 0x3130041; continue;
 }
 0x3130041 => {
  cpu.flat_ds(0x3130041); cpu.push(0x3130046); cpu.call(0x29600a3); if cpu.returned_to != 0x3130046 { return; }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  if !cpu.zf { pc = 0x313004c; continue; }
  pc = 0x3130049; continue;
 }
 0x3130044 => {
  { cpu.sub32(cpu.edi, 0x7u32); }
  if !cpu.zf { pc = 0x313004c; continue; }
  pc = 0x3130049; continue;
 }
 0x3130049 => {
  cpu.flat_ds(0x3130049); cpu.push(0x313004e); cpu.call(0xa3200b1); if cpu.returned_to != 0x313004e { return; }
  cpu.flat_ds(0x313004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 0x313004c => {
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v = cpu.pop(); cpu.set_segment(0, v as u16); }
  cpu.flat_ds(0x313004e); cpu.returned_to = cpu.pop(); cpu.pop(); let f = cpu.pop(); cpu.set_flags(f); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0313004f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x313004f); }
 let mut pc: u32 = 0x313004f;
 loop { match pc {
 0x313004f => {
  { let v = cpu.edi; cpu.push(v); }
  { cpu.push(cpu.segments[0] as u32); }
  { cpu.push(cpu.segments[3] as u32); }
  { let v: u32 = 0x2fcd7a00u32; cpu.eax = v; }
  { let v = cpu.pop(); cpu.set_segment(3, v as u16); }
  { let v: u32 = cpu.edi; cpu.w32(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { let v: u8 = (cpu.eax as u8).wrapping_add(0x0u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.segments[0] as u16); cpu.w16(0x0u32.wrapping_add(cpu.esi).wrapping_add(cpu.bases[3]), v); }
  { cpu.push(cpu.segments[0] as u32); }
  { let v: u8 = cpu.add8(cpu.r8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3])), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.edi).wrapping_add(cpu.bases[3]), v); }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  cpu.poll(); pc = cpu.pop(); continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03000000(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3000000); }
 let mut pc: u32 = 0x3000000;
 loop { match pc {
 0x3000000 => {
  cpu.unimplemented(0x3000000, "Int3");
  cpu.poll(); pc = 0x3000000; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_03003d10(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x3003d10); }
 let mut pc: u32 = 0x3003d10;
 loop { match pc {
 0x3003b39 => {
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x3003b42); f_03093220(cpu); if cpu.returned_to != 0x3003b42 { return; }
  pc = 0x3003b42; continue;
 }
 0x3003b42 => {
  { let v: u32 = cpu.esp; cpu.eax = v; }
  cpu.push(0x3003b49); f_03093170(cpu); if cpu.returned_to != 0x3003b49 { return; }
  { cpu.and32(cpu.eax, cpu.eax); }
  if cpu.zf { pc = 0x3003e8f; continue; }
  pc = 0x3003b51; continue;
 }
 0x3003b51 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x10u32); }
  if cpu.cf { pc = 0x3003b75; continue; }
  pc = 0x3003b59; continue;
 }
 0x3003b59 => {
  if (cpu.cf || cpu.zf) { pc = 0x3003cda; continue; }
  pc = 0x3003b5f; continue;
 }
 0x3003b5f => {
  { cpu.sub32(cpu.eax, 0x100u32); }
  if cpu.cf { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003b66; continue;
 }
 0x3003b66 => {
  if (cpu.cf || cpu.zf) { pc = 0x3003b80; continue; }
  pc = 0x3003b68; continue;
 }
 0x3003b68 => {
  { cpu.sub32(cpu.eax, 0x400u32); }
  if cpu.zf { pc = 0x3003cda; continue; }
  pc = 0x3003b73; continue;
 }
 0x3003b73 => {
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003b75 => {
  { cpu.sub32(cpu.eax, 0x1u32); }
  if cpu.zf { pc = 0x3003d07; continue; }
  pc = 0x3003b7e; continue;
 }
 0x3003b7e => {
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003b80 => {
  { let v: u32 = cpu.ebp; cpu.w32(0x315d5e8u32, v); }
  pc = 0x3003b86; continue;
 }
 0x3003b86 => {
  cpu.push(0x3003b8b); f_0308eab4(cpu); if cpu.returned_to != 0x3003b8b { return; }
  { cpu.and8((cpu.eax as u8), 0x1u8); }
  if cpu.zf { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003b8f; continue;
 }
 0x3003b8f => {
  cpu.push(0x3003b94); f_03091f8c(cpu); if cpu.returned_to != 0x3003b94 { return; }
  { let v: u32 = 0x18u32.wrapping_add(cpu.esp); cpu.eax = v; }
  cpu.push(0x3003b9d); f_03093220(cpu); if cpu.returned_to != 0x3003b9d { return; }
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5dcu32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x3003bd4; continue; }
  pc = 0x3003bae; continue;
 }
 0x3003bae => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5e8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d5e8u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003c03; continue; }
  pc = 0x3003bc4; continue;
 }
 0x3003bc4 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315d5e8u32, v); }
  cpu.push(0x3003bd2); f_030033c8(cpu); if cpu.returned_to != 0x3003bd2 { return; }
  pc = 0x3003c03; continue;
 }
 0x3003bd4 => {
  { let v: u32 = cpu.r32(0x315d608u32); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003c03; continue; }
  pc = 0x3003be1; continue;
 }
 0x3003be1 => {
  { let v: u32 = cpu.r32(0x315d5e8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d5e8u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003c03; continue; }
  pc = 0x3003bf5; continue;
 }
 0x3003bf5 => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315d5e8u32, v); }
  cpu.push(0x3003c03); f_03003418(cpu); if cpu.returned_to != 0x3003c03 { return; }
  pc = 0x3003c03; continue;
 }
 0x3003c03 => {
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5d8u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { cpu.sub32(cpu.eax, cpu.edx); }
  if (cpu.sf == cpu.of) { pc = 0x3003c7e; continue; }
  pc = 0x3003c14; continue;
 }
 0x3003c14 => {
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5e8u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d5e8u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003cad; continue; }
  pc = 0x3003c2e; continue;
 }
 0x3003c2e => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315d614u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5e0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315d5e8u32, v); }
  { let v: u32 = cpu.edx; cpu.w32(0x315d5e0u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003c54; continue; }
  pc = 0x3003c4e; continue;
 }
 0x3003c4e => {
  { let v: u32 = cpu.edi; cpu.w32(0x315d5e0u32, v); }
  pc = 0x3003c54; continue;
 }
 0x3003c54 => {
  { let v: u32 = cpu.r32(0x315d5e0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d604u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { pc = 0x3003cad; continue; }
  pc = 0x3003c63; continue;
 }
 0x3003c63 => {
  { let v: u32 = cpu.r32(0x315d614u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d604u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003cad; continue; }
  pc = 0x3003c76; continue;
 }
 0x3003c76 => {
  { let v: u32 = cpu.edi; cpu.w32(0x315d604u32, v); }
  pc = 0x3003cad; continue;
 }
 0x3003c7e => {
  { let v: u32 = cpu.r32(0x315d5e4u32); cpu.esi = v; }
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.esi.wrapping_mul(1)); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.ebx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003cad; continue; }
  pc = 0x3003c8b; continue;
 }
 0x3003c8b => {
  { let v: u32 = cpu.r32(0x315d5e8u32); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d5e8u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003cad; continue; }
  pc = 0x3003c9f; continue;
 }
 0x3003c9f => {
  { let v: u32 = 0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebp.wrapping_mul(1)); cpu.esi = v; }
  { let v: u32 = cpu.esi; cpu.w32(0x315d5e8u32, v); }
  cpu.push(0x3003cad); f_030034dc(cpu); if cpu.returned_to != 0x3003cad { return; }
  pc = 0x3003cad; continue;
 }
 0x3003cad => {
  { let v: u32 = cpu.r32(0x18u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x16u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5dcu32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x315d5d8u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.eax, 16u32); cpu.eax = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.esi); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ebx); cpu.eax = v; }
  cpu.push(0x3003cd0); f_03003334(cpu); if cpu.returned_to != 0x3003cd0 { return; }
  cpu.push(0x3003cd5); f_03003160(cpu); if cpu.returned_to != 0x3003cd5 { return; }
  cpu.poll(); pc = 0x3003b86; continue;
 }
 0x3003cda => {
  { let v: u32 = cpu.r32(0x14u32.wrapping_add(cpu.esp)); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x10u32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5dcu32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315d5d8u32); cpu.ecx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.ebx); cpu.edx = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.ecx); cpu.eax = v; }
  cpu.push(0x3003cf7); f_03003334(cpu); if cpu.returned_to != 0x3003cf7 { return; }
  { let v: u32 = cpu.r32(0x315d5e0u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003d07 => {
  { let v: u32 = cpu.r32(0xcu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { cpu.sub32(cpu.eax, 0x100u32); }
  pc = 0x3003d10; continue;
 }
 0x3003d10 => {
  if !cpu.zf { pc = 0x3003d22; continue; }
  pc = 0x3003d12; continue;
 }
 0x3003d12 => {
  { let v: u32 = 0xffffffffu32; cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003d22 => {
  { cpu.sub32(cpu.eax, 0x119u32); }
  if !cpu.zf { pc = 0x3003d39; continue; }
  pc = 0x3003d29; continue;
 }
 0x3003d29 => {
  { let v: u32 = cpu.r32(0x315d5e0u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.ecx = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003d39 => {
  { cpu.sub32(cpu.eax, 0x116u32); }
  if !cpu.zf { pc = 0x3003d4a; continue; }
  pc = 0x3003d40; continue;
 }
 0x3003d40 => {
  cpu.push(0x3003d45); f_030033c8(cpu); if cpu.returned_to != 0x3003d45 { return; }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003d4a => {
  { cpu.sub32(cpu.eax, 0x118u32); }
  if !cpu.zf { pc = 0x3003d5b; continue; }
  pc = 0x3003d51; continue;
 }
 0x3003d51 => {
  cpu.push(0x3003d56); f_03003418(cpu); if cpu.returned_to != 0x3003d56 { return; }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003d5b => {
  { cpu.sub32(cpu.eax, 0x113u32); }
  if !cpu.zf { pc = 0x3003d75; continue; }
  pc = 0x3003d62; continue;
 }
 0x3003d62 => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x3003d64; continue;
 }
 0x3003d64 => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x3003d6a); f_030033c8(cpu); if cpu.returned_to != 0x3003d6a { return; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003d73; continue;
 }
 0x3003d73 => {
  cpu.poll(); pc = 0x3003d64; continue;
 }
 0x3003d75 => {
  { cpu.sub32(cpu.eax, 0x114u32); }
  if !cpu.zf { pc = 0x3003d8f; continue; }
  pc = 0x3003d7c; continue;
 }
 0x3003d7c => {
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  pc = 0x3003d7e; continue;
 }
 0x3003d7e => {
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  cpu.push(0x3003d84); f_03003418(cpu); if cpu.returned_to != 0x3003d84 { return; }
  { cpu.sub32(cpu.ebx, 0xau32); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003d8d; continue;
 }
 0x3003d8d => {
  cpu.poll(); pc = 0x3003d7e; continue;
 }
 0x3003d8f => {
  { cpu.sub32(cpu.eax, 0x111u32); }
  if !cpu.zf { pc = 0x3003da7; continue; }
  pc = 0x3003d96; continue;
 }
 0x3003d96 => {
  { let v: u32 = cpu.edi; cpu.w32(0x315d5e0u32, v); }
  { let v: u32 = cpu.edi; cpu.w32(0x315d604u32, v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003da7 => {
  { cpu.sub32(cpu.eax, 0x112u32); }
  if !cpu.zf { pc = 0x3003dd0; continue; }
  pc = 0x3003dae; continue;
 }
 0x3003dae => {
  { let v: u32 = cpu.r32(0x315d60cu32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5f8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax.wrapping_sub(cpu.esi); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d604u32, v); }
  { let v: u32 = cpu.r32(0x315d60cu32); cpu.eax = v; }
  { let v: u32 = cpu.dec32(cpu.eax); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x315d5e0u32, v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003dd0 => {
  { cpu.sub32(cpu.eax, 0x115u32); }
  if !cpu.zf { pc = 0x3003e29; continue; }
  pc = 0x3003dd7; continue;
 }
 0x3003dd7 => {
  { let v: u32 = cpu.r32(0x315d614u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5e0u32); cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d5e0u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { pc = 0x3003df4; continue; }
  pc = 0x3003dee; continue;
 }
 0x3003dee => {
  { let v: u32 = cpu.edi; cpu.w32(0x315d5e0u32, v); }
  pc = 0x3003df4; continue;
 }
 0x3003df4 => {
  { let v: u32 = cpu.r32(0x315d5e0u32); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d604u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003e07; continue;
 }
 0x3003e07 => {
  { let v: u32 = cpu.r32(0x315d614u32); cpu.eax = v; }
  { let v: u32 = cpu.esi; cpu.edx = v; }
  { let v: u32 = cpu.edx.wrapping_sub(cpu.eax); cpu.edx = v; }
  { let v: u32 = cpu.edx; cpu.w32(0x315d604u32, v); }
  { cpu.sub32(cpu.edi, cpu.edx); }
  if (cpu.zf || cpu.sf != cpu.of) { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003e1e; continue;
 }
 0x3003e1e => {
  { let v: u32 = cpu.edi; cpu.w32(0x315d604u32, v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003e29 => {
  { cpu.sub32(cpu.eax, 0x117u32); }
  if !cpu.zf { pc = 0x3003e3a; continue; }
  pc = 0x3003e30; continue;
 }
 0x3003e30 => {
  cpu.push(0x3003e35); f_030034dc(cpu); if cpu.returned_to != 0x3003e35 { return; }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003e3a => {
  { cpu.and8(cpu.r8(0x8u32.wrapping_add(cpu.esp)), 0x4u8); }
  if cpu.zf { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003e45; continue;
 }
 0x3003e45 => {
  { let v: u32 = cpu.r32(0x315d60cu32); cpu.esi = v; }
  { let v: u32 = cpu.edi; cpu.ebx = v; }
  { cpu.sub32(cpu.edi, cpu.esi); }
  if (cpu.sf == cpu.of) { cpu.poll(); pc = 0x3003b42; continue; }
  pc = 0x3003e55; continue;
 }
 0x3003e55 => {
  { let v: u32 = cpu.edi; cpu.eax = v; }
  { let v: u32 = cpu.shl32(cpu.esi, 2u32); cpu.esi = v; }
  pc = 0x3003e5a; continue;
 }
 0x3003e5a => {
  { let v: u32 = cpu.r32(0x315d61cu32); cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(1))); cpu.edx = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.edx)); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.edx & 0xffu32; cpu.edx = v; }
  { cpu.sub32(cpu.edx, cpu.r32(0xcu32.wrapping_add(cpu.esp))); }
  if !cpu.zf { pc = 0x3003e82; continue; }
  pc = 0x3003e71; continue;
 }
 0x3003e71 => {
  { let v: u32 = cpu.ebx; cpu.w32(0x315d604u32, v); }
  { let v: u32 = cpu.ebx; cpu.w32(0x315d5e0u32, v); }
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003e82 => {
  { let v: u32 = cpu.eax.wrapping_add(0x4u32); cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { cpu.sub32(cpu.eax, cpu.esi); }
  if (cpu.sf != cpu.of) { cpu.poll(); pc = 0x3003e5a; continue; }
  pc = 0x3003e8a; continue;
 }
 0x3003e8a => {
  cpu.poll(); pc = 0x3003b42; continue;
 }
 0x3003e8f => {
  cpu.push(0x3003e94); f_03003160(cpu); if cpu.returned_to != 0x3003e94 { return; }
  { cpu.and16((cpu.ecx as u16), (cpu.ecx as u16)); }
  if !cpu.zf { cpu.poll(); pc = 0x3003b39; continue; }
  pc = 0x3003e9d; continue;
 }
 0x3003e9d => {
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.edi = v; }
  { cpu.sub32(cpu.edi, 0xffffffffu32); }
  if cpu.zf { pc = 0x3003eb4; continue; }
  pc = 0x3003ea6; continue;
 }
 0x3003ea6 => {
  { let v: u32 = cpu.r32(0x315d5d0u32); cpu.eax = v; }
  { let v: u32 = cpu.edi; cpu.edx = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.eax).wrapping_add(cpu.edx.wrapping_mul(4))); cpu.eax = v; }
  { let v: u32 = cpu.eax; cpu.w32(0x1cu32.wrapping_add(cpu.esp), v); }
  pc = 0x3003eb4; continue;
 }
 0x3003eb4 => {
  { let v: u32 = 0x1u32; cpu.ecx = v; }
  { let v: u32 = 0x315d480u32; cpu.eax = v; }
  { let v: u32 = cpu.r32(0x315d5dau32); cpu.ebx = v; }
  { let v: u32 = cpu.r32(0x315d5d6u32); cpu.edx = v; }
  { let v: u32 = cpu.sar32(cpu.ebx, 16u32); cpu.ebx = v; }
  { let v: u32 = cpu.sar32(cpu.edx, 16u32); cpu.edx = v; }
  cpu.push(0x3003ed5); f_030924e4(cpu); if cpu.returned_to != 0x3003ed5 { return; }
  { let v: u32 = 0x315d480u32; cpu.eax = v; }
  cpu.push(0x3003edf); f_0308fcc8(cpu); if cpu.returned_to != 0x3003edf { return; }
  { let v: u32 = cpu.r32(0x315d5d4u32); cpu.eax = v; }
  cpu.push(0x3003ee9); f_0308c640(cpu); if cpu.returned_to != 0x3003ee9 { return; }
  { let v: u32 = cpu.r32(0x315d600u32); cpu.eax = v; }
  cpu.push(0x3003ef3); f_0308c640(cpu); if cpu.returned_to != 0x3003ef3 { return; }
  { let v: u32 = cpu.r32(0x315d5d0u32); cpu.eax = v; }
  cpu.push(0x3003efd); f_0308c640(cpu); if cpu.returned_to != 0x3003efd { return; }
  { let v: u32 = cpu.r32(0x315d61cu32); cpu.eax = v; }
  cpu.push(0x3003f07); f_0308c640(cpu); if cpu.returned_to != 0x3003f07 { return; }
  { let v: u32 = cpu.r32(0x1cu32.wrapping_add(cpu.esp)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esp, 0x3cu32); cpu.esp = v; }
  { let v: u32 = cpu.pop(); cpu.ebp = v; }
  { let v: u32 = cpu.pop(); cpu.edi = v; }
  { let v: u32 = cpu.pop(); cpu.esi = v; }
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(16);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_0308eb40(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x308eb40); }
 let mut pc: u32 = 0x308eb40;
 loop { match pc {
 0x308eb40 => {
  { let v: u32 = cpu.dec32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.port_in((cpu.edx as u16), 4) as u32; cpu.eax = v; }
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.ebx)) | (cpu.eax as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u8 = 0xedu8; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.adc32(cpu.eax, 0x1b0308eeu32); cpu.eax = v; }
  { cpu.port_out((cpu.edx as u16), 4, cpu.eax as u32); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  if cpu.pf() { cpu.poll(); pc = 0x308eb40; continue; }
  pc = 0x308eb52; continue;
 }
 0x308eb52 => {
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  cpu.unimplemented(0x308eb54, "Into");
  { cpu.port_out((cpu.edx as u16), 1, (cpu.eax as u8) as u32); }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sbb32(cpu.ebp, cpu.edi); cpu.ebp = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.sbb32(cpu.ebp, cpu.edi); cpu.ebp = v; }
  { let v: u8 = cpu.or8(cpu.r8(0x0u32.wrapping_add(cpu.ebx)), (cpu.eax as u8)); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  f_0308eb60(cpu); return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2bb8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2bb8); }
 let mut pc: u32 = 0x30b2bb8;
 loop { match pc {
 0x30b2bb8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2bb8; continue; }
  pc = 0x30b2bc8; continue;
 }
 0x30b2bc8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2bc9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2bc9); }
 let mut pc: u32 = 0x30b2bc9;
 loop { match pc {
 0x30b2bc9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x4u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2bc9; continue; }
  pc = 0x30b2bd4; continue;
 }
 0x30b2bd4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2bd5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2bd5); }
 let mut pc: u32 = 0x30b2bd5;
 loop { match pc {
 0x30b2bd5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2bd5; continue; }
  pc = 0x30b2bf3; continue;
 }
 0x30b2bf3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2bf4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2bf4); }
 let mut pc: u32 = 0x30b2bf4;
 loop { match pc {
 0x30b2bf4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2bf4; continue; }
  pc = 0x30b2c07; continue;
 }
 0x30b2c07 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c08(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c08); }
 let mut pc: u32 = 0x30b2c08;
 loop { match pc {
 0x30b2c08 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c08; continue; }
  pc = 0x30b2c26; continue;
 }
 0x30b2c26 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c27(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c27); }
 let mut pc: u32 = 0x30b2c27;
 loop { match pc {
 0x30b2c27 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c27; continue; }
  pc = 0x30b2c3a; continue;
 }
 0x30b2c3a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c3b); }
 let mut pc: u32 = 0x30b2c3b;
 loop { match pc {
 0x30b2c3b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c3b; continue; }
  pc = 0x30b2c57; continue;
 }
 0x30b2c57 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c58(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c58); }
 let mut pc: u32 = 0x30b2c58;
 loop { match pc {
 0x30b2c58 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c58; continue; }
  pc = 0x30b2c69; continue;
 }
 0x30b2c69 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c6a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c6a); }
 let mut pc: u32 = 0x30b2c6a;
 loop { match pc {
 0x30b2c6a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c6a; continue; }
  pc = 0x30b2c86; continue;
 }
 0x30b2c86 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c87(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c87); }
 let mut pc: u32 = 0x30b2c87;
 loop { match pc {
 0x30b2c87 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.add32(cpu.esi, 0x8u32); cpu.esi = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c87; continue; }
  pc = 0x30b2c98; continue;
 }
 0x30b2c98 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2c99(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2c99); }
 let mut pc: u32 = 0x30b2c99;
 loop { match pc {
 0x30b2c99 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2c99; continue; }
  pc = 0x30b2cac; continue;
 }
 0x30b2cac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2cad(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2cad); }
 let mut pc: u32 = 0x30b2cad;
 loop { match pc {
 0x30b2cad => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2cad; continue; }
  pc = 0x30b2cbb; continue;
 }
 0x30b2cbb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2cbc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2cbc); }
 let mut pc: u32 = 0x30b2cbc;
 loop { match pc {
 0x30b2cbc => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2cbc; continue; }
  pc = 0x30b2cd1; continue;
 }
 0x30b2cd1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2cd2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2cd2); }
 let mut pc: u32 = 0x30b2cd2;
 loop { match pc {
 0x30b2cd2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2cd2; continue; }
  pc = 0x30b2ce2; continue;
 }
 0x30b2ce2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2ce3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2ce3); }
 let mut pc: u32 = 0x30b2ce3;
 loop { match pc {
 0x30b2ce3 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2ce3; continue; }
  pc = 0x30b2d03; continue;
 }
 0x30b2d03 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2d04(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2d04); }
 let mut pc: u32 = 0x30b2d04;
 loop { match pc {
 0x30b2d04 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2d04; continue; }
  pc = 0x30b2d19; continue;
 }
 0x30b2d19 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2d1a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2d1a); }
 let mut pc: u32 = 0x30b2d1a;
 loop { match pc {
 0x30b2d1a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2d1a; continue; }
  pc = 0x30b2d3a; continue;
 }
 0x30b2d3a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2d3b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2d3b); }
 let mut pc: u32 = 0x30b2d3b;
 loop { match pc {
 0x30b2d3b => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2d3b; continue; }
  pc = 0x30b2d50; continue;
 }
 0x30b2d50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2d51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2d51); }
 let mut pc: u32 = 0x30b2d51;
 loop { match pc {
 0x30b2d51 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2d51; continue; }
  pc = 0x30b2d75; continue;
 }
 0x30b2d75 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2d76(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2d76); }
 let mut pc: u32 = 0x30b2d76;
 loop { match pc {
 0x30b2d76 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2d76; continue; }
  pc = 0x30b2d8f; continue;
 }
 0x30b2d8f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2d90(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2d90); }
 let mut pc: u32 = 0x30b2d90;
 loop { match pc {
 0x30b2d90 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2d90; continue; }
  pc = 0x30b2db4; continue;
 }
 0x30b2db4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2db5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2db5); }
 let mut pc: u32 = 0x30b2db5;
 loop { match pc {
 0x30b2db5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2db5; continue; }
  pc = 0x30b2dce; continue;
 }
 0x30b2dce => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2dcf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2dcf); }
 let mut pc: u32 = 0x30b2dcf;
 loop { match pc {
 0x30b2dcf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2dcf; continue; }
  pc = 0x30b2df1; continue;
 }
 0x30b2df1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2df2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2df2); }
 let mut pc: u32 = 0x30b2df2;
 loop { match pc {
 0x30b2df2 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2df2; continue; }
  pc = 0x30b2e09; continue;
 }
 0x30b2e09 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2e0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2e0a); }
 let mut pc: u32 = 0x30b2e0a;
 loop { match pc {
 0x30b2e0a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2e0a; continue; }
  pc = 0x30b2e2c; continue;
 }
 0x30b2e2c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2e2d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2e2d); }
 let mut pc: u32 = 0x30b2e2d;
 loop { match pc {
 0x30b2e2d => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2e2d; continue; }
  pc = 0x30b2e44; continue;
 }
 0x30b2e44 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2e45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2e45); }
 let mut pc: u32 = 0x30b2e45;
 loop { match pc {
 0x30b2e45 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2e45; continue; }
  pc = 0x30b2e6b; continue;
 }
 0x30b2e6b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2e6c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2e6c); }
 let mut pc: u32 = 0x30b2e6c;
 loop { match pc {
 0x30b2e6c => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2e6c; continue; }
  pc = 0x30b2e87; continue;
 }
 0x30b2e87 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2e88(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2e88); }
 let mut pc: u32 = 0x30b2e88;
 loop { match pc {
 0x30b2e88 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2e88; continue; }
  pc = 0x30b2eae; continue;
 }
 0x30b2eae => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2eaf(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2eaf); }
 let mut pc: u32 = 0x30b2eaf;
 loop { match pc {
 0x30b2eaf => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2eaf; continue; }
  pc = 0x30b2eca; continue;
 }
 0x30b2eca => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2ecb(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2ecb); }
 let mut pc: u32 = 0x30b2ecb;
 loop { match pc {
 0x30b2ecb => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b2eea; continue; }
  pc = 0x30b2ed7; continue;
 }
 0x30b2ed7 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b2ef1; continue; }
  pc = 0x30b2ede; continue;
 }
 0x30b2ede => {
  { let v: u32 = cpu.xor32(cpu.eax, 0x8000u32); cpu.eax = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2ecb; continue; }
  pc = 0x30b2ee9; continue;
 }
 0x30b2ee9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b2eea => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2ede; continue;
 }
 0x30b2ef1 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2ede; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2ef8(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2ef8); }
 let mut pc: u32 = 0x30b2ef8;
 loop { match pc {
 0x30b2ef8 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b2f12; continue; }
  pc = 0x30b2f04; continue;
 }
 0x30b2f04 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b2f19; continue; }
  pc = 0x30b2f0b; continue;
 }
 0x30b2f0b => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2ef8; continue; }
  pc = 0x30b2f11; continue;
 }
 0x30b2f11 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b2f12 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2f0b; continue;
 }
 0x30b2f19 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2f0b; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2f20(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2f20); }
 let mut pc: u32 = 0x30b2f20;
 loop { match pc {
 0x30b2f20 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b2f5d; continue; }
  pc = 0x30b2f2f; continue;
 }
 0x30b2f2f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b2f64; continue; }
  pc = 0x30b2f36; continue;
 }
 0x30b2f36 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b2f6b; continue; }
  pc = 0x30b2f3e; continue;
 }
 0x30b2f3e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b2f72; continue; }
  pc = 0x30b2f46; continue;
 }
 0x30b2f46 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2f20; continue; }
  pc = 0x30b2f5c; continue;
 }
 0x30b2f5c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b2f5d => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2f36; continue;
 }
 0x30b2f64 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2f36; continue;
 }
 0x30b2f6b => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b2f46; continue;
 }
 0x30b2f72 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b2f46; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2f79(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2f79); }
 let mut pc: u32 = 0x30b2f79;
 loop { match pc {
 0x30b2f79 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b2fab; continue; }
  pc = 0x30b2f88; continue;
 }
 0x30b2f88 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b2fb2; continue; }
  pc = 0x30b2f8f; continue;
 }
 0x30b2f8f => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b2fb9; continue; }
  pc = 0x30b2f97; continue;
 }
 0x30b2f97 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b2fc0; continue; }
  pc = 0x30b2f9f; continue;
 }
 0x30b2f9f => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.edx = (cpu.edx & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2f79; continue; }
  pc = 0x30b2faa; continue;
 }
 0x30b2faa => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b2fab => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2f8f; continue;
 }
 0x30b2fb2 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2f8f; continue;
 }
 0x30b2fb9 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b2f9f; continue;
 }
 0x30b2fc0 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b2f9f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b2fc7(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b2fc7); }
 let mut pc: u32 = 0x30b2fc7;
 loop { match pc {
 0x30b2fc7 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3004; continue; }
  pc = 0x30b2fd6; continue;
 }
 0x30b2fd6 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b300b; continue; }
  pc = 0x30b2fdd; continue;
 }
 0x30b2fdd => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3012; continue; }
  pc = 0x30b2fe5; continue;
 }
 0x30b2fe5 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3019; continue; }
  pc = 0x30b2fed; continue;
 }
 0x30b2fed => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b2fc7; continue; }
  pc = 0x30b3003; continue;
 }
 0x30b3003 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3004 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2fdd; continue;
 }
 0x30b300b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b2fdd; continue;
 }
 0x30b3012 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b2fed; continue;
 }
 0x30b3019 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b2fed; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3020(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3020); }
 let mut pc: u32 = 0x30b3020;
 loop { match pc {
 0x30b3020 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3052; continue; }
  pc = 0x30b302f; continue;
 }
 0x30b302f => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3059; continue; }
  pc = 0x30b3036; continue;
 }
 0x30b3036 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3060; continue; }
  pc = 0x30b303e; continue;
 }
 0x30b303e => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3067; continue; }
  pc = 0x30b3046; continue;
 }
 0x30b3046 => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3020; continue; }
  pc = 0x30b3051; continue;
 }
 0x30b3051 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3052 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3036; continue;
 }
 0x30b3059 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3036; continue;
 }
 0x30b3060 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3046; continue;
 }
 0x30b3067 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3046; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b306e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b306e); }
 let mut pc: u32 = 0x30b306e;
 loop { match pc {
 0x30b306e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b30a9; continue; }
  pc = 0x30b307d; continue;
 }
 0x30b307d => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b30b0; continue; }
  pc = 0x30b3084; continue;
 }
 0x30b3084 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b30b7; continue; }
  pc = 0x30b308c; continue;
 }
 0x30b308c => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b30be; continue; }
  pc = 0x30b3094; continue;
 }
 0x30b3094 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b306e; continue; }
  pc = 0x30b30a8; continue;
 }
 0x30b30a8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b30a9 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3084; continue;
 }
 0x30b30b0 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3084; continue;
 }
 0x30b30b7 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3094; continue;
 }
 0x30b30be => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3094; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b30c5(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b30c5); }
 let mut pc: u32 = 0x30b30c5;
 loop { match pc {
 0x30b30c5 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b30f5; continue; }
  pc = 0x30b30d4; continue;
 }
 0x30b30d4 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b30fc; continue; }
  pc = 0x30b30db; continue;
 }
 0x30b30db => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3103; continue; }
  pc = 0x30b30e3; continue;
 }
 0x30b30e3 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b310a; continue; }
  pc = 0x30b30eb; continue;
 }
 0x30b30eb => {
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b30c5; continue; }
  pc = 0x30b30f4; continue;
 }
 0x30b30f4 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b30f5 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b30db; continue;
 }
 0x30b30fc => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b30db; continue;
 }
 0x30b3103 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b30eb; continue;
 }
 0x30b310a => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b30eb; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3111(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3111); }
 let mut pc: u32 = 0x30b3111;
 loop { match pc {
 0x30b3111 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b314c; continue; }
  pc = 0x30b3120; continue;
 }
 0x30b3120 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3153; continue; }
  pc = 0x30b3127; continue;
 }
 0x30b3127 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b315a; continue; }
  pc = 0x30b312f; continue;
 }
 0x30b312f => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3161; continue; }
  pc = 0x30b3137; continue;
 }
 0x30b3137 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.xor32(cpu.edx, 0x8000u32); cpu.edx = v; }
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3111; continue; }
  pc = 0x30b314b; continue;
 }
 0x30b314b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b314c => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3127; continue;
 }
 0x30b3153 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3127; continue;
 }
 0x30b315a => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3137; continue;
 }
 0x30b3161 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3137; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3168(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3168); }
 let mut pc: u32 = 0x30b3168;
 loop { match pc {
 0x30b3168 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3198; continue; }
  pc = 0x30b3177; continue;
 }
 0x30b3177 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b319f; continue; }
  pc = 0x30b317e; continue;
 }
 0x30b317e => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b31a6; continue; }
  pc = 0x30b3186; continue;
 }
 0x30b3186 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b31ad; continue; }
  pc = 0x30b318e; continue;
 }
 0x30b318e => {
  { let v: u8 = ((cpu.edx >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u8 = ((cpu.eax >> 8) as u8); cpu.w8(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.inc32(cpu.edi); cpu.edi = v; }
  { let v: u32 = cpu.inc32(cpu.ebx); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3168; continue; }
  pc = 0x30b3197; continue;
 }
 0x30b3197 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3198 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b317e; continue;
 }
 0x30b319f => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b317e; continue;
 }
 0x30b31a6 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b318e; continue;
 }
 0x30b31ad => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b318e; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b31b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b31b4); }
 let mut pc: u32 = 0x30b31b4;
 loop { match pc {
 0x30b31b4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b31d6; continue; }
  pc = 0x30b31c0; continue;
 }
 0x30b31c0 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b31dd; continue; }
  pc = 0x30b31c7; continue;
 }
 0x30b31c7 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b31b4; continue; }
  pc = 0x30b31d5; continue;
 }
 0x30b31d5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b31d6 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b31c7; continue;
 }
 0x30b31dd => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b31c7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b31e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b31e4); }
 let mut pc: u32 = 0x30b31e4;
 loop { match pc {
 0x30b31e4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3201; continue; }
  pc = 0x30b31f0; continue;
 }
 0x30b31f0 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3208; continue; }
  pc = 0x30b31f7; continue;
 }
 0x30b31f7 => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b31e4; continue; }
  pc = 0x30b3200; continue;
 }
 0x30b3200 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3201 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b31f7; continue;
 }
 0x30b3208 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b31f7; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b320f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b320f); }
 let mut pc: u32 = 0x30b320f;
 loop { match pc {
 0x30b320f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3233; continue; }
  pc = 0x30b321b; continue;
 }
 0x30b321b => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b323a; continue; }
  pc = 0x30b3222; continue;
 }
 0x30b3222 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b320f; continue; }
  pc = 0x30b3232; continue;
 }
 0x30b3232 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3233 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3222; continue;
 }
 0x30b323a => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3222; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3241(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3241); }
 let mut pc: u32 = 0x30b3241;
 loop { match pc {
 0x30b3241 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3260; continue; }
  pc = 0x30b324d; continue;
 }
 0x30b324d => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3267; continue; }
  pc = 0x30b3254; continue;
 }
 0x30b3254 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3241; continue; }
  pc = 0x30b325f; continue;
 }
 0x30b325f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3260 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3254; continue;
 }
 0x30b3267 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3254; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b326e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b326e); }
 let mut pc: u32 = 0x30b326e;
 loop { match pc {
 0x30b326e => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b32ad; continue; }
  pc = 0x30b327d; continue;
 }
 0x30b327d => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b32b4; continue; }
  pc = 0x30b3284; continue;
 }
 0x30b3284 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b32bb; continue; }
  pc = 0x30b328c; continue;
 }
 0x30b328c => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b32c2; continue; }
  pc = 0x30b3294; continue;
 }
 0x30b3294 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b326e; continue; }
  pc = 0x30b32ac; continue;
 }
 0x30b32ac => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b32ad => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3284; continue;
 }
 0x30b32b4 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3284; continue;
 }
 0x30b32bb => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3294; continue;
 }
 0x30b32c2 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3294; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b32c9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b32c9); }
 let mut pc: u32 = 0x30b32c9;
 loop { match pc {
 0x30b32c9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b32fd; continue; }
  pc = 0x30b32d8; continue;
 }
 0x30b32d8 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3304; continue; }
  pc = 0x30b32df; continue;
 }
 0x30b32df => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b330b; continue; }
  pc = 0x30b32e7; continue;
 }
 0x30b32e7 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3312; continue; }
  pc = 0x30b32ef; continue;
 }
 0x30b32ef => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b32c9; continue; }
  pc = 0x30b32fc; continue;
 }
 0x30b32fc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b32fd => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b32df; continue;
 }
 0x30b3304 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b32df; continue;
 }
 0x30b330b => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b32ef; continue;
 }
 0x30b3312 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b32ef; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3319(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3319); }
 let mut pc: u32 = 0x30b3319;
 loop { match pc {
 0x30b3319 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3358; continue; }
  pc = 0x30b3328; continue;
 }
 0x30b3328 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b335f; continue; }
  pc = 0x30b332f; continue;
 }
 0x30b332f => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3366; continue; }
  pc = 0x30b3337; continue;
 }
 0x30b3337 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b336d; continue; }
  pc = 0x30b333f; continue;
 }
 0x30b333f => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3319; continue; }
  pc = 0x30b3357; continue;
 }
 0x30b3357 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3358 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b332f; continue;
 }
 0x30b335f => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b332f; continue;
 }
 0x30b3366 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b333f; continue;
 }
 0x30b336d => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b333f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3374(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3374); }
 let mut pc: u32 = 0x30b3374;
 loop { match pc {
 0x30b3374 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b33a8; continue; }
  pc = 0x30b3383; continue;
 }
 0x30b3383 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b33af; continue; }
  pc = 0x30b338a; continue;
 }
 0x30b338a => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b33b6; continue; }
  pc = 0x30b3392; continue;
 }
 0x30b3392 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b33bd; continue; }
  pc = 0x30b339a; continue;
 }
 0x30b339a => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3374; continue; }
  pc = 0x30b33a7; continue;
 }
 0x30b33a7 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b33a8 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b338a; continue;
 }
 0x30b33af => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b338a; continue;
 }
 0x30b33b6 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b339a; continue;
 }
 0x30b33bd => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b339a; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b33c4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b33c4); }
 let mut pc: u32 = 0x30b33c4;
 loop { match pc {
 0x30b33c4 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3407; continue; }
  pc = 0x30b33d3; continue;
 }
 0x30b33d3 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b340e; continue; }
  pc = 0x30b33da; continue;
 }
 0x30b33da => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3415; continue; }
  pc = 0x30b33e2; continue;
 }
 0x30b33e2 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b341c; continue; }
  pc = 0x30b33ea; continue;
 }
 0x30b33ea => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b33c4; continue; }
  pc = 0x30b3406; continue;
 }
 0x30b3406 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3407 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b33da; continue;
 }
 0x30b340e => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b33da; continue;
 }
 0x30b3415 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b33ea; continue;
 }
 0x30b341c => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b33ea; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3423(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3423); }
 let mut pc: u32 = 0x30b3423;
 loop { match pc {
 0x30b3423 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b345b; continue; }
  pc = 0x30b3432; continue;
 }
 0x30b3432 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3462; continue; }
  pc = 0x30b3439; continue;
 }
 0x30b3439 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3469; continue; }
  pc = 0x30b3441; continue;
 }
 0x30b3441 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3470; continue; }
  pc = 0x30b3449; continue;
 }
 0x30b3449 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3423; continue; }
  pc = 0x30b345a; continue;
 }
 0x30b345a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b345b => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3439; continue;
 }
 0x30b3462 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3439; continue;
 }
 0x30b3469 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3449; continue;
 }
 0x30b3470 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3449; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3477(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3477); }
 let mut pc: u32 = 0x30b3477;
 loop { match pc {
 0x30b3477 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b34ba; continue; }
  pc = 0x30b3486; continue;
 }
 0x30b3486 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b34c1; continue; }
  pc = 0x30b348d; continue;
 }
 0x30b348d => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b34c8; continue; }
  pc = 0x30b3495; continue;
 }
 0x30b3495 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b34cf; continue; }
  pc = 0x30b349d; continue;
 }
 0x30b349d => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3477; continue; }
  pc = 0x30b34b9; continue;
 }
 0x30b34b9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b34ba => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b348d; continue;
 }
 0x30b34c1 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b348d; continue;
 }
 0x30b34c8 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b349d; continue;
 }
 0x30b34cf => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b349d; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b34d6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b34d6); }
 let mut pc: u32 = 0x30b34d6;
 loop { match pc {
 0x30b34d6 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b350e; continue; }
  pc = 0x30b34e5; continue;
 }
 0x30b34e5 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3515; continue; }
  pc = 0x30b34ec; continue;
 }
 0x30b34ec => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b351c; continue; }
  pc = 0x30b34f4; continue;
 }
 0x30b34f4 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3523; continue; }
  pc = 0x30b34fc; continue;
 }
 0x30b34fc => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x2u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.add32(cpu.edi, 0x4u32); cpu.edi = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b34d6; continue; }
  pc = 0x30b350d; continue;
 }
 0x30b350d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b350e => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b34ec; continue;
 }
 0x30b3515 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b34ec; continue;
 }
 0x30b351c => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b34fc; continue;
 }
 0x30b3523 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b34fc; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b352a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b352a); }
 let mut pc: u32 = 0x30b352a;
 loop { match pc {
 0x30b352a => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b356b; continue; }
  pc = 0x30b3539; continue;
 }
 0x30b3539 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3572; continue; }
  pc = 0x30b3540; continue;
 }
 0x30b3540 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3579; continue; }
  pc = 0x30b3548; continue;
 }
 0x30b3548 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3580; continue; }
  pc = 0x30b3550; continue;
 }
 0x30b3550 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b352a; continue; }
  pc = 0x30b356a; continue;
 }
 0x30b356a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b356b => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3540; continue;
 }
 0x30b3572 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3540; continue;
 }
 0x30b3579 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3550; continue;
 }
 0x30b3580 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3550; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3587(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3587); }
 let mut pc: u32 = 0x30b3587;
 loop { match pc {
 0x30b3587 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b35bd; continue; }
  pc = 0x30b3596; continue;
 }
 0x30b3596 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b35c4; continue; }
  pc = 0x30b359d; continue;
 }
 0x30b359d => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b35cb; continue; }
  pc = 0x30b35a5; continue;
 }
 0x30b35a5 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b35d2; continue; }
  pc = 0x30b35ad; continue;
 }
 0x30b35ad => {
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3587; continue; }
  pc = 0x30b35bc; continue;
 }
 0x30b35bc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b35bd => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b359d; continue;
 }
 0x30b35c4 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b359d; continue;
 }
 0x30b35cb => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b35ad; continue;
 }
 0x30b35d2 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b35ad; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b35d9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b35d9); }
 let mut pc: u32 = 0x30b35d9;
 loop { match pc {
 0x30b35d9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b361a; continue; }
  pc = 0x30b35e8; continue;
 }
 0x30b35e8 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3621; continue; }
  pc = 0x30b35ef; continue;
 }
 0x30b35ef => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3628; continue; }
  pc = 0x30b35f7; continue;
 }
 0x30b35f7 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b362f; continue; }
  pc = 0x30b35ff; continue;
 }
 0x30b35ff => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b35d9; continue; }
  pc = 0x30b3619; continue;
 }
 0x30b3619 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b361a => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b35ef; continue;
 }
 0x30b3621 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b35ef; continue;
 }
 0x30b3628 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b35ff; continue;
 }
 0x30b362f => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b35ff; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3636(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3636); }
 let mut pc: u32 = 0x30b3636;
 loop { match pc {
 0x30b3636 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b366c; continue; }
  pc = 0x30b3645; continue;
 }
 0x30b3645 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3673; continue; }
  pc = 0x30b364c; continue;
 }
 0x30b364c => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b367a; continue; }
  pc = 0x30b3654; continue;
 }
 0x30b3654 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3681; continue; }
  pc = 0x30b365c; continue;
 }
 0x30b365c => {
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3636; continue; }
  pc = 0x30b366b; continue;
 }
 0x30b366b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b366c => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b364c; continue;
 }
 0x30b3673 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b364c; continue;
 }
 0x30b367a => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b365c; continue;
 }
 0x30b3681 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b365c; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3688(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3688); }
 let mut pc: u32 = 0x30b3688;
 loop { match pc {
 0x30b3688 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b36cd; continue; }
  pc = 0x30b3697; continue;
 }
 0x30b3697 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b36d4; continue; }
  pc = 0x30b369e; continue;
 }
 0x30b369e => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b36db; continue; }
  pc = 0x30b36a6; continue;
 }
 0x30b36a6 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b36e2; continue; }
  pc = 0x30b36ae; continue;
 }
 0x30b36ae => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b3688; continue; }
  pc = 0x30b36cc; continue;
 }
 0x30b36cc => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b36cd => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b369e; continue;
 }
 0x30b36d4 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b369e; continue;
 }
 0x30b36db => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b36ae; continue;
 }
 0x30b36e2 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b36ae; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b36e9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b36e9); }
 let mut pc: u32 = 0x30b36e9;
 loop { match pc {
 0x30b36e9 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3723; continue; }
  pc = 0x30b36f8; continue;
 }
 0x30b36f8 => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b372a; continue; }
  pc = 0x30b36ff; continue;
 }
 0x30b36ff => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3731; continue; }
  pc = 0x30b3707; continue;
 }
 0x30b3707 => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3738; continue; }
  pc = 0x30b370f; continue;
 }
 0x30b370f => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b36e9; continue; }
  pc = 0x30b3722; continue;
 }
 0x30b3722 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3723 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b36ff; continue;
 }
 0x30b372a => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b36ff; continue;
 }
 0x30b3731 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b370f; continue;
 }
 0x30b3738 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b370f; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b373f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b373f); }
 let mut pc: u32 = 0x30b373f;
 loop { match pc {
 0x30b373f => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3784; continue; }
  pc = 0x30b374e; continue;
 }
 0x30b374e => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b378b; continue; }
  pc = 0x30b3755; continue;
 }
 0x30b3755 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b3792; continue; }
  pc = 0x30b375d; continue;
 }
 0x30b375d => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b3799; continue; }
  pc = 0x30b3765; continue;
 }
 0x30b3765 => {
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.edx ^ 0x8000u32; cpu.edx = v; }
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b373f; continue; }
  pc = 0x30b3783; continue;
 }
 0x30b3783 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b3784 => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3755; continue;
 }
 0x30b378b => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b3755; continue;
 }
 0x30b3792 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3765; continue;
 }
 0x30b3799 => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b3765; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b37a0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b37a0); }
 let mut pc: u32 = 0x30b37a0;
 loop { match pc {
 0x30b37a0 => {
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.esi)); cpu.eax = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.esi)); cpu.edx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x8u32); cpu.esi = v; }
  { cpu.sub32(cpu.eax, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b37da; continue; }
  pc = 0x30b37af; continue;
 }
 0x30b37af => {
  { cpu.sub32(cpu.eax, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b37e1; continue; }
  pc = 0x30b37b6; continue;
 }
 0x30b37b6 => {
  { cpu.sub32(cpu.edx, 0x7fffu32); }
  if (!cpu.zf && cpu.sf == cpu.of) { pc = 0x30b37e8; continue; }
  pc = 0x30b37be; continue;
 }
 0x30b37be => {
  { cpu.sub32(cpu.edx, 0xffff8000u32); }
  if (cpu.sf != cpu.of) { pc = 0x30b37ef; continue; }
  pc = 0x30b37c6; continue;
 }
 0x30b37c6 => {
  { let a = ((cpu.eax >> 8) as u8); let b = (cpu.eax as u8); { let v: u8 = b; cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.eax = (cpu.eax & 0xffffff00) | v as u32; } }
  { let a = ((cpu.edx >> 8) as u8); let b = (cpu.edx as u8); { let v: u8 = b; cpu.edx = (cpu.edx & 0xffff00ff) | ((v as u32) << 8); } { let v: u8 = a; cpu.edx = (cpu.edx & 0xffffff00) | v as u32; } }
  { let v: u16 = (cpu.edx as u16); cpu.w16(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u16 = (cpu.eax as u16); cpu.w16(0x0u32.wrapping_add(cpu.ebx), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x2u32); cpu.edi = v; }
  { let v: u32 = cpu.add32(cpu.ebx, 0x2u32); cpu.ebx = v; }
  { let v: u32 = cpu.dec32(cpu.ecx); cpu.ecx = v; }
  if !cpu.zf { cpu.poll(); pc = 0x30b37a0; continue; }
  pc = 0x30b37d9; continue;
 }
 0x30b37d9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 0x30b37da => {
  { let v: u32 = 0x7fffu32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b37b6; continue;
 }
 0x30b37e1 => {
  { let v: u32 = 0xffff8000u32; cpu.eax = v; }
  cpu.poll(); pc = 0x30b37b6; continue;
 }
 0x30b37e8 => {
  { let v: u32 = 0x7fffu32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b37c6; continue;
 }
 0x30b37ef => {
  { let v: u32 = 0xffff8000u32; cpu.edx = v; }
  cpu.poll(); pc = 0x30b37c6; continue;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b37f6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b37f6); }
 let mut pc: u32 = 0x30b37f6;
 loop { match pc {
 0x30b37f6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3816; continue; }
  pc = 0x30b37fe; continue;
 }
 0x30b37fe => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b37f6; continue; }
  pc = 0x30b3816; continue;
 }
 0x30b3816 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3817(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3817); }
 let mut pc: u32 = 0x30b3817;
 loop { match pc {
 0x30b3817 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b383a; continue; }
  pc = 0x30b381f; continue;
 }
 0x30b381f => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3817; continue; }
  pc = 0x30b383a; continue;
 }
 0x30b383a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b383b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b383b); }
 let mut pc: u32 = 0x30b383b;
 loop { match pc {
 0x30b383b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b386c; continue; }
  pc = 0x30b3843; continue;
 }
 0x30b3843 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b383b; continue; }
  pc = 0x30b386c; continue;
 }
 0x30b386c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b386d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b386d); }
 let mut pc: u32 = 0x30b386d;
 loop { match pc {
 0x30b386d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b389e; continue; }
  pc = 0x30b3875; continue;
 }
 0x30b3875 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b386d; continue; }
  pc = 0x30b389e; continue;
 }
 0x30b389e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b389f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b389f); }
 let mut pc: u32 = 0x30b389f;
 loop { match pc {
 0x30b389f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b38c2; continue; }
  pc = 0x30b38a7; continue;
 }
 0x30b38a7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b389f; continue; }
  pc = 0x30b38c2; continue;
 }
 0x30b38c2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b38c3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b38c3); }
 let mut pc: u32 = 0x30b38c3;
 loop { match pc {
 0x30b38c3 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b38e9; continue; }
  pc = 0x30b38cb; continue;
 }
 0x30b38cb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b38c3; continue; }
  pc = 0x30b38e9; continue;
 }
 0x30b38e9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b38ea(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b38ea); }
 let mut pc: u32 = 0x30b38ea;
 loop { match pc {
 0x30b38ea => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b391d; continue; }
  pc = 0x30b38f2; continue;
 }
 0x30b38f2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b38ea; continue; }
  pc = 0x30b391d; continue;
 }
 0x30b391d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b391e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b391e); }
 let mut pc: u32 = 0x30b391e;
 loop { match pc {
 0x30b391e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3951; continue; }
  pc = 0x30b3926; continue;
 }
 0x30b3926 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b391e; continue; }
  pc = 0x30b3951; continue;
 }
 0x30b3951 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3952(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3952); }
 let mut pc: u32 = 0x30b3952;
 loop { match pc {
 0x30b3952 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b396d; continue; }
  pc = 0x30b395a; continue;
 }
 0x30b395a => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3952; continue; }
  pc = 0x30b396d; continue;
 }
 0x30b396d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b396e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b396e); }
 let mut pc: u32 = 0x30b396e;
 loop { match pc {
 0x30b396e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b398c; continue; }
  pc = 0x30b3976; continue;
 }
 0x30b3976 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b396e; continue; }
  pc = 0x30b398c; continue;
 }
 0x30b398c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b398d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b398d); }
 let mut pc: u32 = 0x30b398d;
 loop { match pc {
 0x30b398d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b39b3; continue; }
  pc = 0x30b3995; continue;
 }
 0x30b3995 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b398d; continue; }
  pc = 0x30b39b3; continue;
 }
 0x30b39b3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b39b4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b39b4); }
 let mut pc: u32 = 0x30b39b4;
 loop { match pc {
 0x30b39b4 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b39da; continue; }
  pc = 0x30b39bc; continue;
 }
 0x30b39bc => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b39b4; continue; }
  pc = 0x30b39da; continue;
 }
 0x30b39da => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b39db(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b39db); }
 let mut pc: u32 = 0x30b39db;
 loop { match pc {
 0x30b39db => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b39f9; continue; }
  pc = 0x30b39e3; continue;
 }
 0x30b39e3 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b39db; continue; }
  pc = 0x30b39f9; continue;
 }
 0x30b39f9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b39fa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b39fa); }
 let mut pc: u32 = 0x30b39fa;
 loop { match pc {
 0x30b39fa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3a1b; continue; }
  pc = 0x30b3a02; continue;
 }
 0x30b3a02 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b39fa; continue; }
  pc = 0x30b3a1b; continue;
 }
 0x30b3a1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3a1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3a1c); }
 let mut pc: u32 = 0x30b3a1c;
 loop { match pc {
 0x30b3a1c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3a44; continue; }
  pc = 0x30b3a24; continue;
 }
 0x30b3a24 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3a1c; continue; }
  pc = 0x30b3a44; continue;
 }
 0x30b3a44 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3a45(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3a45); }
 let mut pc: u32 = 0x30b3a45;
 loop { match pc {
 0x30b3a45 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3a6d; continue; }
  pc = 0x30b3a4d; continue;
 }
 0x30b3a4d => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3a45; continue; }
  pc = 0x30b3a6d; continue;
 }
 0x30b3a6d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3a6e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3a6e); }
 let mut pc: u32 = 0x30b3a6e;
 loop { match pc {
 0x30b3a6e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3a99; continue; }
  pc = 0x30b3a76; continue;
 }
 0x30b3a76 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x315cff8u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3a6e; continue; }
  pc = 0x30b3a99; continue;
 }
 0x30b3a99 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3a9a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3a9a); }
 let mut pc: u32 = 0x30b3a9a;
 loop { match pc {
 0x30b3a9a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3ac8; continue; }
  pc = 0x30b3aa2; continue;
 }
 0x30b3aa2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x315cff8u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3a9a; continue; }
  pc = 0x30b3ac8; continue;
 }
 0x30b3ac8 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3ac9(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3ac9); }
 let mut pc: u32 = 0x30b3ac9;
 loop { match pc {
 0x30b3ac9 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3b08; continue; }
  pc = 0x30b3ad1; continue;
 }
 0x30b3ad1 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3ac9; continue; }
  pc = 0x30b3b08; continue;
 }
 0x30b3b08 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3b09(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3b09); }
 let mut pc: u32 = 0x30b3b09;
 loop { match pc {
 0x30b3b09 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3b48; continue; }
  pc = 0x30b3b11; continue;
 }
 0x30b3b11 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3b09; continue; }
  pc = 0x30b3b48; continue;
 }
 0x30b3b48 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3b49(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3b49); }
 let mut pc: u32 = 0x30b3b49;
 loop { match pc {
 0x30b3b49 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3b7a; continue; }
  pc = 0x30b3b51; continue;
 }
 0x30b3b51 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3b49; continue; }
  pc = 0x30b3b7a; continue;
 }
 0x30b3b7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3b7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3b7b); }
 let mut pc: u32 = 0x30b3b7b;
 loop { match pc {
 0x30b3b7b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3baf; continue; }
  pc = 0x30b3b83; continue;
 }
 0x30b3b83 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3b7b; continue; }
  pc = 0x30b3baf; continue;
 }
 0x30b3baf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3bb0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3bb0); }
 let mut pc: u32 = 0x30b3bb0;
 loop { match pc {
 0x30b3bb0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3bf1; continue; }
  pc = 0x30b3bb8; continue;
 }
 0x30b3bb8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3bb0; continue; }
  pc = 0x30b3bf1; continue;
 }
 0x30b3bf1 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3bf2(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3bf2); }
 let mut pc: u32 = 0x30b3bf2;
 loop { match pc {
 0x30b3bf2 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3c33; continue; }
  pc = 0x30b3bfa; continue;
 }
 0x30b3bfa => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3bf2; continue; }
  pc = 0x30b3c33; continue;
 }
 0x30b3c33 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3c34(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3c34); }
 let mut pc: u32 = 0x30b3c34;
 loop { match pc {
 0x30b3c34 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3c5a; continue; }
  pc = 0x30b3c3c; continue;
 }
 0x30b3c3c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x315cff8u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3c34; continue; }
  pc = 0x30b3c5a; continue;
 }
 0x30b3c5a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3c5b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3c5b); }
 let mut pc: u32 = 0x30b3c5b;
 loop { match pc {
 0x30b3c5b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3c84; continue; }
  pc = 0x30b3c63; continue;
 }
 0x30b3c63 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x315cff8u32)); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3c5b; continue; }
  pc = 0x30b3c84; continue;
 }
 0x30b3c84 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3c85(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3c85); }
 let mut pc: u32 = 0x30b3c85;
 loop { match pc {
 0x30b3c85 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3cb9; continue; }
  pc = 0x30b3c8d; continue;
 }
 0x30b3c8d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3c85; continue; }
  pc = 0x30b3cb9; continue;
 }
 0x30b3cb9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3cba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3cba); }
 let mut pc: u32 = 0x30b3cba;
 loop { match pc {
 0x30b3cba => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3cee; continue; }
  pc = 0x30b3cc2; continue;
 }
 0x30b3cc2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff00ff) | ((v as u32) << 8); }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3cba; continue; }
  pc = 0x30b3cee; continue;
 }
 0x30b3cee => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3cef(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3cef); }
 let mut pc: u32 = 0x30b3cef;
 loop { match pc {
 0x30b3cef => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3d1b; continue; }
  pc = 0x30b3cf7; continue;
 }
 0x30b3cf7 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3cef; continue; }
  pc = 0x30b3d1b; continue;
 }
 0x30b3d1b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3d1c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3d1c); }
 let mut pc: u32 = 0x30b3d1c;
 loop { match pc {
 0x30b3d1c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3d4b; continue; }
  pc = 0x30b3d24; continue;
 }
 0x30b3d24 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3d1c; continue; }
  pc = 0x30b3d4b; continue;
 }
 0x30b3d4b => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3d4c(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3d4c); }
 let mut pc: u32 = 0x30b3d4c;
 loop { match pc {
 0x30b3d4c => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3d82; continue; }
  pc = 0x30b3d54; continue;
 }
 0x30b3d54 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3d4c; continue; }
  pc = 0x30b3d82; continue;
 }
 0x30b3d82 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3d83(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3d83); }
 let mut pc: u32 = 0x30b3d83;
 loop { match pc {
 0x30b3d83 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3db9; continue; }
  pc = 0x30b3d8b; continue;
 }
 0x30b3d8b => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3d83; continue; }
  pc = 0x30b3db9; continue;
 }
 0x30b3db9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3dba(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3dba); }
 let mut pc: u32 = 0x30b3dba;
 loop { match pc {
 0x30b3dba => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3dd5; continue; }
  pc = 0x30b3dc2; continue;
 }
 0x30b3dc2 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3dba; continue; }
  pc = 0x30b3dd5; continue;
 }
 0x30b3dd5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3dd6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3dd6); }
 let mut pc: u32 = 0x30b3dd6;
 loop { match pc {
 0x30b3dd6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3dfb; continue; }
  pc = 0x30b3dde; continue;
 }
 0x30b3dde => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.inc32(cpu.esi); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3dd6; continue; }
  pc = 0x30b3dfb; continue;
 }
 0x30b3dfb => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3dfc(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3dfc); }
 let mut pc: u32 = 0x30b3dfc;
 loop { match pc {
 0x30b3dfc => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3e25; continue; }
  pc = 0x30b3e04; continue;
 }
 0x30b3e04 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3dfc; continue; }
  pc = 0x30b3e25; continue;
 }
 0x30b3e25 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3e26(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3e26); }
 let mut pc: u32 = 0x30b3e26;
 loop { match pc {
 0x30b3e26 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3e50; continue; }
  pc = 0x30b3e2e; continue;
 }
 0x30b3e2e => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3e26; continue; }
  pc = 0x30b3e50; continue;
 }
 0x30b3e50 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3e51(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3e51); }
 let mut pc: u32 = 0x30b3e51;
 loop { match pc {
 0x30b3e51 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3e7a; continue; }
  pc = 0x30b3e59; continue;
 }
 0x30b3e59 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3e51; continue; }
  pc = 0x30b3e7a; continue;
 }
 0x30b3e7a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3e7b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3e7b); }
 let mut pc: u32 = 0x30b3e7b;
 loop { match pc {
 0x30b3e7b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3ea5; continue; }
  pc = 0x30b3e83; continue;
 }
 0x30b3e83 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3e7b; continue; }
  pc = 0x30b3ea5; continue;
 }
 0x30b3ea5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3ea6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3ea6); }
 let mut pc: u32 = 0x30b3ea6;
 loop { match pc {
 0x30b3ea6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3ecf; continue; }
  pc = 0x30b3eae; continue;
 }
 0x30b3eae => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3ea6; continue; }
  pc = 0x30b3ecf; continue;
 }
 0x30b3ecf => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3ed0(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3ed0); }
 let mut pc: u32 = 0x30b3ed0;
 loop { match pc {
 0x30b3ed0 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3f09; continue; }
  pc = 0x30b3ed8; continue;
 }
 0x30b3ed8 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3ed0; continue; }
  pc = 0x30b3f09; continue;
 }
 0x30b3f09 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3f0a(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3f0a); }
 let mut pc: u32 = 0x30b3f0a;
 loop { match pc {
 0x30b3f0a => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3f4c; continue; }
  pc = 0x30b3f12; continue;
 }
 0x30b3f12 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3f0a; continue; }
  pc = 0x30b3f4c; continue;
 }
 0x30b3f4c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3f4d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3f4d); }
 let mut pc: u32 = 0x30b3f4d;
 loop { match pc {
 0x30b3f4d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3f90; continue; }
  pc = 0x30b3f55; continue;
 }
 0x30b3f55 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3f4d; continue; }
  pc = 0x30b3f90; continue;
 }
 0x30b3f90 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3f91(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3f91); }
 let mut pc: u32 = 0x30b3f91;
 loop { match pc {
 0x30b3f91 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b3fd3; continue; }
  pc = 0x30b3f99; continue;
 }
 0x30b3f99 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3f91; continue; }
  pc = 0x30b3fd3; continue;
 }
 0x30b3fd3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b3fd4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b3fd4); }
 let mut pc: u32 = 0x30b3fd4;
 loop { match pc {
 0x30b3fd4 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4017; continue; }
  pc = 0x30b3fdc; continue;
 }
 0x30b3fdc => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b3fd4; continue; }
  pc = 0x30b4017; continue;
 }
 0x30b4017 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4018(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4018); }
 let mut pc: u32 = 0x30b4018;
 loop { match pc {
 0x30b4018 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b403c; continue; }
  pc = 0x30b4020; continue;
 }
 0x30b4020 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4018; continue; }
  pc = 0x30b403c; continue;
 }
 0x30b403c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b403d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b403d); }
 let mut pc: u32 = 0x30b403d;
 loop { match pc {
 0x30b403d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4071; continue; }
  pc = 0x30b4045; continue;
 }
 0x30b4045 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x2u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b403d; continue; }
  pc = 0x30b4071; continue;
 }
 0x30b4071 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4072(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4072); }
 let mut pc: u32 = 0x30b4072;
 loop { match pc {
 0x30b4072 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b40a9; continue; }
  pc = 0x30b407a; continue;
 }
 0x30b407a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4072; continue; }
  pc = 0x30b40a9; continue;
 }
 0x30b40a9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b40aa(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b40aa); }
 let mut pc: u32 = 0x30b40aa;
 loop { match pc {
 0x30b40aa => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b40e2; continue; }
  pc = 0x30b40b2; continue;
 }
 0x30b40b2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b40aa; continue; }
  pc = 0x30b40e2; continue;
 }
 0x30b40e2 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b40e3(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b40e3); }
 let mut pc: u32 = 0x30b40e3;
 loop { match pc {
 0x30b40e3 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b411a; continue; }
  pc = 0x30b40eb; continue;
 }
 0x30b40eb => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b40e3; continue; }
  pc = 0x30b411a; continue;
 }
 0x30b411a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b411b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b411b); }
 let mut pc: u32 = 0x30b411b;
 loop { match pc {
 0x30b411b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4153; continue; }
  pc = 0x30b4123; continue;
 }
 0x30b4123 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.esi.wrapping_add(0x4u32); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b411b; continue; }
  pc = 0x30b4153; continue;
 }
 0x30b4153 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4154(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4154); }
 let mut pc: u32 = 0x30b4154;
 loop { match pc {
 0x30b4154 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b417a; continue; }
  pc = 0x30b415c; continue;
 }
 0x30b415c => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x315cff8u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4154; continue; }
  pc = 0x30b417a; continue;
 }
 0x30b417a => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b417b(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b417b); }
 let mut pc: u32 = 0x30b417b;
 loop { match pc {
 0x30b417b => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b41ab; continue; }
  pc = 0x30b4183; continue;
 }
 0x30b4183 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.esi, cpu.r32(0x315cff8u32)); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b417b; continue; }
  pc = 0x30b41ab; continue;
 }
 0x30b41ab => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b41ac(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b41ac); }
 let mut pc: u32 = 0x30b41ac;
 loop { match pc {
 0x30b41ac => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b41e3; continue; }
  pc = 0x30b41b4; continue;
 }
 0x30b41b4 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b41ac; continue; }
  pc = 0x30b41e3; continue;
 }
 0x30b41e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b41e4(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b41e4); }
 let mut pc: u32 = 0x30b41e4;
 loop { match pc {
 0x30b41e4 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b421c; continue; }
  pc = 0x30b41ec; continue;
 }
 0x30b41ec => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b41e4; continue; }
  pc = 0x30b421c; continue;
 }
 0x30b421c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b421d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b421d); }
 let mut pc: u32 = 0x30b421d;
 loop { match pc {
 0x30b421d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4254; continue; }
  pc = 0x30b4225; continue;
 }
 0x30b4225 => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b421d; continue; }
  pc = 0x30b4254; continue;
 }
 0x30b4254 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4255(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4255); }
 let mut pc: u32 = 0x30b4255;
 loop { match pc {
 0x30b4255 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b428d; continue; }
  pc = 0x30b425d; continue;
 }
 0x30b425d => {
  { let v: u8 = cpu.r8(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffffff00) | v as u32; }
  { let v: u8 = cpu.r8(0x1u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffffff00) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edx).wrapping_add(cpu.eax.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.r32(0x400u32.wrapping_add(cpu.edx).wrapping_add(cpu.ebx.wrapping_mul(4))); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4255; continue; }
  pc = 0x30b428d; continue;
 }
 0x30b428d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b428e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b428e); }
 let mut pc: u32 = 0x30b428e;
 loop { match pc {
 0x30b428e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b42c5; continue; }
  pc = 0x30b4296; continue;
 }
 0x30b4296 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b428e; continue; }
  pc = 0x30b42c5; continue;
 }
 0x30b42c5 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b42c6(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b42c6); }
 let mut pc: u32 = 0x30b42c6;
 loop { match pc {
 0x30b42c6 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b430d; continue; }
  pc = 0x30b42ce; continue;
 }
 0x30b42ce => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b42c6; continue; }
  pc = 0x30b430d; continue;
 }
 0x30b430d => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b430e(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b430e); }
 let mut pc: u32 = 0x30b430e;
 loop { match pc {
 0x30b430e => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b435e; continue; }
  pc = 0x30b4316; continue;
 }
 0x30b4316 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b430e; continue; }
  pc = 0x30b435e; continue;
 }
 0x30b435e => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b435f(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b435f); }
 let mut pc: u32 = 0x30b435f;
 loop { match pc {
 0x30b435f => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b43b0; continue; }
  pc = 0x30b4367; continue;
 }
 0x30b4367 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b435f; continue; }
  pc = 0x30b43b0; continue;
 }
 0x30b43b0 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b43b1(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b43b1); }
 let mut pc: u32 = 0x30b43b1;
 loop { match pc {
 0x30b43b1 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4401; continue; }
  pc = 0x30b43b9; continue;
 }
 0x30b43b9 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b43b1; continue; }
  pc = 0x30b4401; continue;
 }
 0x30b4401 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4402(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4402); }
 let mut pc: u32 = 0x30b4402;
 loop { match pc {
 0x30b4402 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4453; continue; }
  pc = 0x30b440a; continue;
 }
 0x30b440a => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.eax ^ 0x8000u32; cpu.eax = v; }
  { let v: u32 = cpu.ebx ^ 0x8000u32; cpu.ebx = v; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4402; continue; }
  pc = 0x30b4453; continue;
 }
 0x30b4453 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4454(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4454); }
 let mut pc: u32 = 0x30b4454;
 loop { match pc {
 0x30b4454 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4486; continue; }
  pc = 0x30b445c; continue;
 }
 0x30b445c => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4454; continue; }
  pc = 0x30b4486; continue;
 }
 0x30b4486 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4487(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4487); }
 let mut pc: u32 = 0x30b4487;
 loop { match pc {
 0x30b4487 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b44c9; continue; }
  pc = 0x30b448f; continue;
 }
 0x30b448f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4487; continue; }
  pc = 0x30b44c9; continue;
 }
 0x30b44c9 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b44ca(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b44ca); }
 let mut pc: u32 = 0x30b44ca;
 loop { match pc {
 0x30b44ca => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b450f; continue; }
  pc = 0x30b44d2; continue;
 }
 0x30b44d2 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b44ca; continue; }
  pc = 0x30b450f; continue;
 }
 0x30b450f => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4510(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4510); }
 let mut pc: u32 = 0x30b4510;
 loop { match pc {
 0x30b4510 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b4556; continue; }
  pc = 0x30b4518; continue;
 }
 0x30b4518 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4510; continue; }
  pc = 0x30b4556; continue;
 }
 0x30b4556 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b4557(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b4557); }
 let mut pc: u32 = 0x30b4557;
 loop { match pc {
 0x30b4557 => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b459c; continue; }
  pc = 0x30b455f; continue;
 }
 0x30b455f => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x4u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b4557; continue; }
  pc = 0x30b459c; continue;
 }
 0x30b459c => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
pub fn f_030b459d(cpu: &mut Cpu) {
 if cpu.profiling { cpu.hit(0x30b459d); }
 let mut pc: u32 = 0x30b459d;
 loop { match pc {
 0x30b459d => {
  { cpu.sub32(cpu.esi, cpu.r32(0x315cfe8u32)); }
  if !cpu.cf { pc = 0x30b45e3; continue; }
  pc = 0x30b45a5; continue;
 }
 0x30b45a5 => {
  { let v: u16 = cpu.r16(0x0u32.wrapping_add(cpu.esi)); cpu.ebx = (cpu.ebx & 0xffff0000) | v as u32; }
  { let v: u16 = cpu.r16(0x2u32.wrapping_add(cpu.esi)); cpu.eax = (cpu.eax & 0xffff0000) | v as u32; }
  { let v: u32 = cpu.ebp ^ cpu.ebp; cpu.ebp = v; }
  { let v: u32 = cpu.add32(cpu.ecx, cpu.r32(0x315cff4u32)); cpu.ecx = v; }
  { let v: u32 = cpu.adc32(cpu.ebp, cpu.ebp); cpu.ebp = v; }
  { let v: u32 = cpu.esi.wrapping_add(cpu.r32(0x315cff8u32.wrapping_add(cpu.ebp.wrapping_mul(4)))); cpu.esi = v; }
  { let v: u32 = (cpu.eax as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x0u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x0u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x0u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = (cpu.ebx as u16) as i16 as i32 as u32; cpu.ebp = v; }
  { let v: u32 = cpu.imul2_32(cpu.ebp, cpu.r32(0x400u32.wrapping_add(cpu.edx))); cpu.ebp = v; }
  { let v: u32 = cpu.sar32(cpu.ebp, 7u32); cpu.ebp = v; }
  { let v: u32 = cpu.r32(0x4u32.wrapping_add(cpu.edi)).wrapping_add(cpu.ebp); cpu.w32(0x4u32.wrapping_add(cpu.edi), v); }
  { let v: u32 = cpu.edi.wrapping_add(0x8u32); cpu.edi = v; }
  { cpu.sub32(cpu.edi, cpu.r32(0x315cff0u32)); }
  if cpu.cf { cpu.poll(); pc = 0x30b459d; continue; }
  pc = 0x30b45e3; continue;
 }
 0x30b45e3 => {
  cpu.returned_to = cpu.r32(cpu.esp); cpu.esp = cpu.esp.wrapping_add(4);  return;
 }
 _ => { cpu.call(pc); return; }
 } }
}
